//! datamodel -- generated_watchdog_datamodel_w15d -- 120 stubs (watchdog w15d Instance/DataModel/Workspace)
//! Source: ida/export.json (85545 funcs) global-dedup vs /tmp/global_eas.txt; Filter RBX::Instance
//! Strict RBX::Instance 8021 total, 0 uncovered in global -> fallback to rbx_sorted 120 (strict 0) EA-sorted asc
//! Range: 0xef04..0x2a3c28
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost::shared_ptr.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xef04 -- __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv")]
pub fn stub_ef04() -> ! {
    todo!("0xef04 rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")
}

// 0xefb4 -- __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_efb4() -> ! {
    todo!("0xefb4 rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf098 -- __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_f098() -> ! {
    todo!("0xf098 boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf198 -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev")]
pub fn stub_f198() -> ! {
    todo!("0xf198 boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xf19c -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_f19c() -> ! {
    todo!("0xf19c boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xf1bc -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_f1bc() -> ! {
    todo!("0xf1bc boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xf1d4 -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_f1d4() -> ! {
    todo!("0xf1d4 boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x179f4 -- __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
// type: void __fastcall(int)
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
#[doc(alias = "__ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE")]
pub fn stub_179f4() -> ! {
    todo!("0x179f4 RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")
}

// 0x31678 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
// was: boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_31678() -> ! {
    todo!("0x31678 rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")
}

// 0x31728 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE
// boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE")]
pub fn stub_31728() -> ! {
    todo!("0x31728 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")
}

// 0x319ec -- __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_319ec() -> ! {
    todo!("0x319ec rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x31aec -- __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_31aec() -> ! {
    todo!("0x31aec boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x31bec -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_31bec() -> ! {
    todo!("0x31bec boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x31bf0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_31bf0() -> ! {
    todo!("0x31bf0 boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x31bf4 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_31bf4() -> ! {
    todo!("0x31bf4 boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x31c14 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_31c14() -> ! {
    todo!("0x31c14 boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x31c2c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_31c2c() -> ! {
    todo!("0x31c2c boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x31cd0 -- __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub fn stub_31cd0() -> ! {
    todo!("0x31cd0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")
}

// 0x31d30 -- __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
// type: int __fastcall(int, int)
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_31d30() -> ! {
    todo!("0x31d30 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")
}

// 0x31d48 -- __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
// type: void __fastcall(int *, void (__fastcall **)(int, int, sp_counted_base **), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_31d48() -> ! {
    todo!("0x31d48 void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")
}

// 0x324fc -- __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_324fc() -> ! {
    todo!("0x324fc rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x325fc -- __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_325fc() -> ! {
    todo!("0x325fc boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x326fc -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_326fc() -> ! {
    todo!("0x326fc boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x32700 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_32700() -> ! {
    todo!("0x32700 boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x33454 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_33454() -> ! {
    todo!("0x33454 boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3346c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_3346c() -> ! {
    todo!("0x3346c boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3a798 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)
// was: boost::shared_ptr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_3a798() -> ! {
    todo!("0x3a798 rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")
}

// 0x3aa10 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_3aa10() -> ! {
    todo!("0x3aa10 boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3aa18 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_3aa18() -> ! {
    todo!("0x3aa18 boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3afe0 -- __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_3afe0() -> ! {
    todo!("0x3afe0 rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3b008 -- __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_3b008() -> ! {
    todo!("0x3b008 boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3b108 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_3b108() -> ! {
    todo!("0x3b108 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3b110 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_3b110() -> ! {
    todo!("0x3b110 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3b130 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_3b130() -> ! {
    todo!("0x3b130 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3b148 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_3b148() -> ! {
    todo!("0x3b148 boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3b674 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)
// was: boost::shared_ptr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_3b674() -> ! {
    todo!("0x3b674 rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")
}

// 0x3b724 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
// boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const&)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE")]
pub fn stub_3b724() -> ! {
    todo!("0x3b724 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")
}

// 0x3b9e8 -- __ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int(void)
// was: boost::shared_ptr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_3b9e8() -> ! {
    todo!("0x3b9e8 rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3ba10 -- __ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_3ba10() -> ! {
    todo!("0x3ba10 boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3bb10 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_3bb10() -> ! {
    todo!("0x3bb10 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3bb18 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_3bb18() -> ! {
    todo!("0x3bb18 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3bb38 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_3bb38() -> ! {
    todo!("0x3bb38 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3bb50 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_3bb50() -> ! {
    todo!("0x3bb50 boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3bbf8 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_
// boost::shared_ptr<RBX::Instance>::operator=(boost::shared_ptr<RBX::Instance> const&)
// was: boost::shared_ptr<RBX::Instance>::operator=(boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_")]
pub fn stub_3bbf8() -> ! {
    todo!("0x3bbf8 rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x3e190 -- __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev")]
pub fn stub_3e190() -> ! {
    todo!("0x3e190 boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x258688 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_258688() -> ! {
    todo!("0x258688 rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")
}

// 0x258738 -- __ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_258738() -> ! {
    todo!("0x258738 rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2588e8 -- __ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_2588e8() -> ! {
    todo!("0x2588e8 boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x2589f0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_2589f0() -> ! {
    todo!("0x2589f0 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2589f4 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_2589f4() -> ! {
    todo!("0x2589f4 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x2589f8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_2589f8() -> ! {
    todo!("0x2589f8 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x258a18 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_258a18() -> ! {
    todo!("0x258a18 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x258a30 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_258a30() -> ! {
    todo!("0x258a30 boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x25bc38 -- __ZNK3RBX5Light12askSetParentEPKNS_8InstanceE
// RBX::Light::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Light *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Light::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Light12askSetParentEPKNS_8InstanceE")]
pub fn stub_25bc38() -> ! {
    todo!("0x25bc38 RBX::Light::askSetParent(RBX::Instance const*)const")
}

// 0x25bc60 -- __ZNK3RBX5Light11askAddChildEPKNS_8InstanceE
// RBX::Light::askAddChild(RBX::Instance const*)const
// type: int __fastcall(RBX::Light *this, const Instance *)
#[doc(alias = "RBX::Light::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Light11askAddChildEPKNS_8InstanceE")]
pub fn stub_25bc60() -> ! {
    todo!("0x25bc60 RBX::Light::askAddChild(RBX::Instance const*)const")
}

// 0x25c4d0 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_25c4d0() -> ! {
    todo!("0x25c4d0 rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")
}

// 0x25c580 -- __ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_25c580() -> ! {
    todo!("0x25c580 rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x25c730 -- __ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_25c730() -> ! {
    todo!("0x25c730 boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x25c838 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_25c838() -> ! {
    todo!("0x25c838 boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x25c83c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_25c83c() -> ! {
    todo!("0x25c83c boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x25c840 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_25c840() -> ! {
    todo!("0x25c840 boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x25c860 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_25c860() -> ! {
    todo!("0x25c860 boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x25c878 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9SpotLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_25c878() -> ! {
    todo!("0x25c878 boost::detail::sp_counted_impl_pd<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x25ce80 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_10PointLightEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_10PointLightEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_25ce80() -> ! {
    todo!("0x25ce80 rbx_core::SharedPtr<RBX::PointLight> RBX::Creatable<RBX::Instance>::create<RBX::PointLight>(void)")
}

// 0x25cf30 -- __ZN5boost10shared_ptrIN3RBX10PointLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10PointLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_25cf30() -> ! {
    todo!("0x25cf30 rbx_core::SharedPtr<RBX::PointLight>::shared_ptr<RBX::PointLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x25d0e0 -- __ZN5boost6detail12shared_countC2IPN3RBX10PointLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10PointLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_25d0e0() -> ! {
    todo!("0x25d0e0 boost::detail::shared_count::shared_count<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x25d1e8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_25d1e8() -> ! {
    todo!("0x25d1e8 boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x25d1ec -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_25d1ec() -> ! {
    todo!("0x25d1ec boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x25d1f0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_25d1f0() -> ! {
    todo!("0x25d1f0 boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x25d210 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_25d210() -> ! {
    todo!("0x25d210 boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x25d228 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10PointLightENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_25d228() -> ! {
    todo!("0x25d228 boost::detail::sp_counted_impl_pd<RBX::PointLight *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x26c350 -- __ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSINS1_8InstanceEEERS4_RKNS0_IT_EE
// boost::shared_ptr<RBX::Reflection::DescribedBase>& boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Reflection::DescribedBase>& boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase>& rbx_core::SharedPtr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSINS1_8InstanceEEERS4_RKNS0_IT_EE")]
pub fn stub_26c350() -> ! {
    todo!("0x26c350 rbx_core::SharedPtr<RBX::Reflection::DescribedBase>& rbx_core::SharedPtr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x26c38c -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrINS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrINS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_")]
pub fn stub_26c38c() -> ! {
    todo!("0x26c38c bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")
}

// 0x26c830 -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrINS_10Reflection7VariantEEEbP9lua_StatejRT_
// bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
pub fn stub_26c830() -> ! {
    todo!("0x26c830 bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26dce4 -- __ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE
// RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Instance> const&)
// type: int __fastcall(int *, int)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_26dce4() -> ! {
    todo!("0x26dce4 RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0x26df08 -- __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
// type: int __fastcall(_DWORD *, _DWORD *)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE")]
pub fn stub_26df08() -> ! {
    todo!("0x26df08 RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")
}

// 0x26ee14 -- __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_26ee14() -> ! {
    todo!("0x26ee14 rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26ef04 -- __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State
// int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,lua_State *)
// type: int __fastcall(char ****, char ****, int)
// was: int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,lua_State *)
#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State")]
pub fn stub_26ef04() -> ! {
    todo!("0x26ef04 int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,lua_State *)")
}

// 0x26fa78 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// type: int __fastcall(int, int, _DWORD *)
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
pub fn stub_26fa78() -> ! {
    todo!("0x26fa78 bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26ff94 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS3_INS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS3_INS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_")]
pub fn stub_26ff94() -> ! {
    todo!("0x26ff94 bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")
}

// 0x277af4 -- __ZN3RBX6CellID14fromParametersEbPfN5boost10shared_ptrINS_8InstanceEEE
// RBX::CellID::fromParameters(bool,float *,boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, int, int, const shared_count *)
// was: RBX::CellID::fromParameters(bool,float *,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::CellID::fromParameters(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX6CellID14fromParametersEbPfN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_277af4() -> ! {
    todo!("0x277af4 RBX::CellID::fromParameters(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x27a4f8 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13registerClassEP9lua_StatePFiS8_ESA_
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// type: int __fastcall(int, int, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13registerClassEP9lua_StatePFiS8_ESA_")]
pub fn stub_27a4f8() -> ! {
    todo!("0x27a4f8 RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")
}

// 0x27a5e0 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE5on_gcEP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_gc(lua_State *)
// type: int __fastcall(int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_gc(lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE5on_gcEP9lua_State")]
pub fn stub_27a5e0() -> ! {
    todo!("0x27a5e0 RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_gc(lua_State *)")
}

// 0x27a608 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringEP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(lua_State *)
// type: int __fastcall(int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringEP9lua_State")]
pub fn stub_27a608() -> ! {
    todo!("0x27a608 RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(lua_State *)")
}

// 0x27c258 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexERKS5_PKcP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_index(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
// type: int __fastcall(int, int, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_index(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexERKS5_PKcP9lua_State")]
pub fn stub_27c258() -> ! {
    todo!("0x27c258 RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")
}

// 0x27dbc8 -- __ZL22PropertyNameCorrectionRKN5boost10shared_ptrIN3RBX8InstanceEEEPKcP9lua_State
// PropertyNameCorrection(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
// type: const char *__fastcall(int, const char *, int)
// was: PropertyNameCorrection(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
#[doc(alias = "PropertyNameCorrection(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
#[doc(alias = "__ZL22PropertyNameCorrectionRKN5boost10shared_ptrIN3RBX8InstanceEEEPKcP9lua_State")]
pub fn stub_27dbc8() -> ! {
    todo!("0x27dbc8 PropertyNameCorrection(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")
}

// 0x27ef18 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexERS5_PKcP9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_newindex(boost::shared_ptr<RBX::Instance>&,char const*,lua_State *)
// type: void __fastcall(RBX::Security::Context *, const char *, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_newindex(boost::shared_ptr<RBX::Instance>&,char const*,lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(rbx_core::SharedPtr<RBX::Instance>&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexERS5_PKcP9lua_State")]
pub fn stub_27ef18() -> ! {
    todo!("0x27ef18 RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(rbx_core::SharedPtr<RBX::Instance>&,char const*,lua_State *)")
}

// 0x280b90 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringERKS5_P9lua_State
// RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(boost::shared_ptr<RBX::Instance> const&,lua_State *)
// type: int __fastcall(RBX::Instance **, int)
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(boost::shared_ptr<RBX::Instance> const&,lua_State *)
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(rbx_core::SharedPtr<RBX::Instance> const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringERKS5_P9lua_State")]
pub fn stub_280b90() -> ! {
    todo!("0x280b90 RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(rbx_core::SharedPtr<RBX::Instance> const&,lua_State *)")
}

// 0x280c4c -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrIS2_EEEEbP9lua_StatejRT_
// bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
#[doc(alias = "bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrIN5boost10shared_ptrIS2_EEEEbP9lua_StatejRT_")]
pub fn stub_280c4c() -> ! {
    todo!("0x280c4c bool RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")
}

// 0x281494 -- __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueIS5_EEbP9lua_StatejRT_
// bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
// type: int __fastcall(int, int, int)
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueIS5_EEbP9lua_StatejRT_")]
pub fn stub_281494() -> ! {
    todo!("0x281494 bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")
}

// 0x284650 -- __ZN24YieldFunctionStateObjectC2EPKN3RBX10Reflection23YieldFunctionDescriptorEN5boost10shared_ptrINS0_8InstanceEEEP9lua_State
// YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,boost::shared_ptr<RBX::Instance>,lua_State *)
// type: _DWORD *__fastcall(_DWORD *, int, const shared_count *, int, int, void *, int, int, int, int)
// was: YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,boost::shared_ptr<RBX::Instance>,lua_State *)
#[doc(alias = "YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,rbx_core::SharedPtr<RBX::Instance>,lua_State *)")]
#[doc(alias = "__ZN24YieldFunctionStateObjectC2EPKN3RBX10Reflection23YieldFunctionDescriptorEN5boost10shared_ptrINS0_8InstanceEEEP9lua_State")]
pub fn stub_284650() -> ! {
    todo!("0x284650 YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,rbx_core::SharedPtr<RBX::Instance>,lua_State *)")
}

// 0x2857a4 -- __ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrEP9lua_Statej
// RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int)
// type: int __fastcall(sp_counted_base **, int, const char *)
#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int)")]
#[doc(alias = "__ZN3RBX3Lua15SharedPtrBridgeINS_8InstanceEE6getPtrEP9lua_Statej")]
pub fn stub_2857a4() -> ! {
    todo!("0x2857a4 RBX::Lua::SharedPtrBridge<RBX::Instance>::getPtr(lua_State *,unsigned int)")
}

// 0x28e0c8 -- __ZN3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_PKNS_8InstanceE
// RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(RBX::Instance const*)
#[doc(alias = "RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_20RuntimeScriptServiceEEEPT_PKNS_8InstanceE")]
pub fn stub_28e0c8() -> ! {
    todo!("0x28e0c8 RBX::RuntimeScriptService * RBX::ServiceProvider::create<RBX::RuntimeScriptService>(RBX::Instance const*)")
}

// 0x28e0e0 -- __ZN3RBX15ServiceProvider6createINS_25ScriptInformationProviderEEEPT_PKNS_8InstanceE
// RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(RBX::Instance const*)
#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_25ScriptInformationProviderEEEPT_PKNS_8InstanceE")]
pub fn stub_28e0e0() -> ! {
    todo!("0x28e0e0 RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(RBX::Instance const*)")
}

// 0x28e0f8 -- __ZN3RBX6Script8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE
// RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)
#[doc(alias = "RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX6Script8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE")]
pub fn stub_28e0f8() -> ! {
    todo!("0x28e0f8 RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)")
}

// 0x28e114 -- __ZNK3RBX6Script12askSetParentEPKNS_8InstanceE
// RBX::Script::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Script *__hidden this, const Instance *)
#[doc(alias = "RBX::Script::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX6Script12askSetParentEPKNS_8InstanceE")]
pub fn stub_28e114() -> ! {
    todo!("0x28e114 RBX::Script::askSetParent(RBX::Instance const*)const")
}

// 0x28e630 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_6ScriptEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)
// was: boost::shared_ptr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6ScriptEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_28e630() -> ! {
    todo!("0x28e630 rbx_core::SharedPtr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)")
}

// 0x28e6e0 -- __ZN5boost10shared_ptrIN3RBX6ScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_28e6e0() -> ! {
    todo!("0x28e6e0 rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x28e890 -- __ZN5boost6detail12shared_countC2IPN3RBX6ScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX6ScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_28e890() -> ! {
    todo!("0x28e890 boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x28e998 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_28e998() -> ! {
    todo!("0x28e998 boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x28e99c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_28e99c() -> ! {
    todo!("0x28e99c boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x28e9a0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_28e9a0() -> ! {
    todo!("0x28e9a0 boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x28e9c0 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_28e9c0() -> ! {
    todo!("0x28e9c0 boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x28e9d8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_28e9d8() -> ! {
    todo!("0x28e9d8 boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x28f0b4 -- __ZN3RBX9CreatableINS_8InstanceEE6createINS_20RuntimeScriptServiceEEEN5boost10shared_ptrIT_EEv
// boost::shared_ptr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)
// was: boost::shared_ptr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_20RuntimeScriptServiceEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_28f0b4() -> ! {
    todo!("0x28f0b4 rbx_core::SharedPtr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)")
}

// 0x28f164 -- __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20RuntimeScriptServiceEEERS3_RKNS0_IT_EE
// boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(boost::shared_ptr<RBX::RuntimeScriptService> const&)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(boost::shared_ptr<RBX::RuntimeScriptService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20RuntimeScriptServiceEEERS3_RKNS0_IT_EE")]
pub fn stub_28f164() -> ! {
    todo!("0x28f164 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const&)")
}

// 0x28f62c -- __ZN5boost10shared_ptrIN3RBX20RuntimeScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// boost::shared_ptr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20RuntimeScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_28f62c() -> ! {
    todo!("0x28f62c rbx_core::SharedPtr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x28f7dc -- __ZN5boost6detail12shared_countC2IPN3RBX20RuntimeScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX20RuntimeScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_28f7dc() -> ! {
    todo!("0x28f7dc boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x28f8e4 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_28f8e4() -> ! {
    todo!("0x28f8e4 boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x28f8e8 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_28f8e8() -> ! {
    todo!("0x28f8e8 boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x28f8ec -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_28f8ec() -> ! {
    todo!("0x28f8ec boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x28f90c -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_28f90c() -> ! {
    todo!("0x28f90c boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x28f924 -- __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_28f924() -> ! {
    todo!("0x28f924 boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x295640 -- __ZN3RBX13ScriptContext26registerDevelopmentLibraryESsN5boost10shared_ptrINS_8InstanceEEE
// RBX::ScriptContext::registerDevelopmentLibrary(std::string,boost::shared_ptr<RBX::Instance>)
// was: RBX::ScriptContext::registerDevelopmentLibrary(std::string,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::ScriptContext::registerDevelopmentLibrary(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13ScriptContext26registerDevelopmentLibraryESsN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_295640() -> ! {
    todo!("0x295640 RBX::ScriptContext::registerDevelopmentLibrary(std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x295890 -- __ZN3RBX13ScriptContext13addCoreScriptEiN5boost10shared_ptrINS_8InstanceEEESs
// RBX::ScriptContext::addCoreScript(int,boost::shared_ptr<RBX::Instance>,std::string)
// was: RBX::ScriptContext::addCoreScript(int,boost::shared_ptr<RBX::Instance>,std::string)
#[doc(alias = "RBX::ScriptContext::addCoreScript(int,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
#[doc(alias = "__ZN3RBX13ScriptContext13addCoreScriptEiN5boost10shared_ptrINS_8InstanceEEESs")]
pub fn stub_295890() -> ! {
    todo!("0x295890 RBX::ScriptContext::addCoreScript(int,rbx_core::SharedPtr<RBX::Instance>,std::string)")
}

// 0x29b090 -- __ZN3RBX13ScriptContext20onCamelCaseViolationEN5boost10shared_ptrINS_8InstanceEEESsS4_
// RBX::ScriptContext::onCamelCaseViolation(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
// was: RBX::ScriptContext::onCamelCaseViolation(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::ScriptContext::onCamelCaseViolation(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13ScriptContext20onCamelCaseViolationEN5boost10shared_ptrINS_8InstanceEEESsS4_")]
pub fn stub_29b090() -> ! {
    todo!("0x29b090 RBX::ScriptContext::onCamelCaseViolation(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x2a3818 -- __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev
// RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_2a3818() -> ! {
    todo!("0x2a3818 RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")
}

// 0x2a38e0 -- __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev")]
pub fn stub_2a38e0() -> ! {
    todo!("0x2a38e0 RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")
}

// 0x2a3a08 -- __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED1Ev
// RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::~BoundFuncDesc()
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED1Ev")]
pub fn stub_2a3a08() -> ! {
    todo!("0x2a3a08 RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")
}

// 0x2a3c28 -- __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_2a3c28() -> ! {
    todo!("0x2a3c28 RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")
}

