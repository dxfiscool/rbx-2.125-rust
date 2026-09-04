// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace (broad, includes PartInstance/MegaClusterInstance etc), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xef04..0x25c730 | total filtered 13121, remaining 9881->9781 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 120 EA-sorted ascending next uncovered gap from 0xef04

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0xef04 — __ZN3RBX9CreatableINS_8InstanceEE6createI19CRenderSettingsItemEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem> RBX::Creatable<RBX::Instance>::create<CRenderSettingsItem>(void)")]
pub use crate::instance::stub_0xef04 as stub_ef04;
// 0xefb4 — __ZN5boost10shared_ptrI19CRenderSettingsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<CRenderSettingsItem>::shared_ptr<CRenderSettingsItem,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0xefb4 as stub_efb4;
// 0xf098 — __ZN5boost6detail12shared_countC2IP19CRenderSettingsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>(CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0xf098 as stub_f098;
// 0xf198 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0xf198 as stub_f198;
// 0xf19c — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0xf19c as stub_f19c;
// 0xf1bc — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0xf1bc as stub_f1bc;
// 0xf1d4 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0xf1d4 as stub_f1d4;
// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
#[doc(alias = "RBX::DataModel::serverSave(void)")]
pub use crate::instance::stub_0x179e8 as stub_179e8;
// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
pub use crate::instance::stub_0x179ec as stub_179ec;
// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
pub use crate::instance::stub_0x179f0 as stub_179f0;
// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
pub use crate::instance::stub_0x179f4 as stub_179f4;
// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// was: executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub use crate::instance::stub_0x2ba54 as stub_2ba54;
// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// was: executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub use crate::instance::stub_0x2bdb0 as stub_2bdb0;
// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
// was: executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub use crate::instance::stub_0x2bf74 as stub_2bf74;
// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
pub use crate::instance::stub_0x2d544 as stub_2d544;
// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub use crate::instance::stub_0x2d660 as stub_2d660;
// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
pub use crate::instance::stub_0x2d67c as stub_2d67c;
// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub use crate::instance::stub_0x2d768 as stub_2d768;
// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
pub use crate::instance::stub_0x2d884 as stub_2d884;
// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub use crate::instance::stub_0x31348 as stub_31348;
// 0x31678 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12LoginServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService> RBX::Creatable<RBX::Instance>::create<RBX::LoginService>(void)")]
pub use crate::instance::stub_0x31678 as stub_31678;
// 0x31728 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12LoginServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const&)")]
pub use crate::instance::stub_0x31728 as stub_31728;
// 0x319ec — __ZN5boost10shared_ptrIN3RBX12LoginServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::LoginService>::shared_ptr<RBX::LoginService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x319ec as stub_319ec;
// 0x31aec — __ZN5boost6detail12shared_countC2IPN3RBX12LoginServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x31aec as stub_31aec;
// 0x31bec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x31bec as stub_31bec;
// 0x31bf0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x31bf0 as stub_31bf0;
// 0x31bf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x31bf4 as stub_31bf4;
// 0x31c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x31c14 as stub_31c14;
// 0x31c2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12LoginServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LoginService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x31c2c as stub_31c2c;
// 0x31cd0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub use crate::instance::stub_0x31cd0 as stub_31cd0;
// 0x31d30 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list3INS3_5valueIS6_EENSF_IS7_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,boost::shared_ptr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::Instance>::invoke(boost::detail::function::function_buffer &,RBX::Instance)")]
pub use crate::instance::stub_0x31d30 as stub_31d30;
// 0x31d48 — __ZN5boost3_bi5list3INS0_5valueIP11objc_objectEENS2_IP13objc_selectorEENS_3argILi1EEEEclIPFvS4_S6_NS_10shared_ptrIN3RBX8InstanceEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::value<objc_selector *>,boost::arg<1>>::operator()<void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::Instance&>>(boost::_bi::type<void>,void (*)(objc_object *,objc_selector,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::Instance&> &,int)")]
pub use crate::instance::stub_0x31d48 as stub_31d48;
// 0x324fc — __ZN5boost10shared_ptrIN3RBX21TaskSchedulerSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskSchedulerSettings>::shared_ptr<RBX::TaskSchedulerSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x324fc as stub_324fc;
// 0x325fc — __ZN5boost6detail12shared_countC2IPN3RBX21TaskSchedulerSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x325fc as stub_325fc;
// 0x326fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x326fc as stub_326fc;
// 0x32700 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX21TaskSchedulerSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TaskSchedulerSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x32700 as stub_32700;
// 0x33454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x33454 as stub_33454;
// 0x3346c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x3346c as stub_3346c;
// 0x380a4 — __ZN10RobloxView13bindWorkspaceEN5boost10shared_ptrIN3RBX8ViewBaseEEENS1_INS2_9DataModelEEENS1_INS2_16OverlayDataModelEEE
// was: RobloxView::bindWorkspace(boost::shared_ptr<RBX::ViewBase>,boost::shared_ptr<RBX::DataModel>,boost::shared_ptr<RBX::OverlayDataModel>)
#[doc(alias = "RobloxView::bindWorkspace(rbx_core::SharedPtr<RBX::ViewBase>,rbx_core::SharedPtr<RBX::DataModel>,rbx_core::SharedPtr<RBX::OverlayDataModel>)")]
pub use crate::instance::stub_0x380a4 as stub_380a4;
// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
// was: boost::shared_ptr<RBX::DataModel>& boost::shared_ptr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &&)
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
pub use crate::instance::stub_0x3a2ec as stub_3a2ec;
// 0x3a798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6CameraEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera> RBX::Creatable<RBX::Instance>::create<RBX::Camera>(void)")]
pub use crate::instance::stub_0x3a798 as stub_3a798;
// 0x3aa10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3aa10 as stub_3aa10;
// 0x3aa18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x3aa18 as stub_3aa18;
// 0x3afe0 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3afe0 as stub_3afe0;
// 0x3b008 — __ZN5boost6detail12shared_countC2IPN3RBX10RunServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3b008 as stub_3b008;
// 0x3b108 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3b108 as stub_3b108;
// 0x3b110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x3b110 as stub_3b110;
// 0x3b130 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x3b130 as stub_3b130;
// 0x3b148 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10RunServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RunService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x3b148 as stub_3b148;
// 0x3b674 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_17ControllerServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService> RBX::Creatable<RBX::Instance>::create<RBX::ControllerService>(void)")]
pub use crate::instance::stub_0x3b674 as stub_3b674;
// 0x3b724 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_17ControllerServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ControllerService>(rbx_core::SharedPtr<RBX::ControllerService> const&)")]
pub use crate::instance::stub_0x3b724 as stub_3b724;
// 0x3b9e8 — __ZN5boost10shared_ptrIN3RBX17ControllerServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::ControllerService>::shared_ptr<RBX::ControllerService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3b9e8 as stub_3b9e8;
// 0x3ba10 — __ZN5boost6detail12shared_countC2IPN3RBX17ControllerServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x3ba10 as stub_3ba10;
// 0x3bb10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3bb10 as stub_3bb10;
// 0x3bb18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x3bb18 as stub_3bb18;
// 0x3bb38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x3bb38 as stub_3bb38;
// 0x3bb50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ControllerServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ControllerService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x3bb50 as stub_3bb50;
// 0x3bbf8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSERKS3_
// was: boost::shared_ptr<RBX::Instance>::operator=(boost::shared_ptr<RBX::Instance> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>::operator=(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub use crate::instance::stub_0x3bbf8 as stub_3bbf8;
// 0x3e190 — __ZN5boost6detail18sp_counted_impl_pdIP19CRenderSettingsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<CRenderSettingsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x3e190 as stub_3e190;
// 0x3e1e8 — __ZN5boost14singleton_poolIN3RBX16OnDemandInstanceELj20ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
#[doc(alias = "boost::singleton_pool<RBX::OnDemandInstance,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub use crate::instance::stub_0x3e1e8 as stub_3e1e8;
// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
// was: RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,boost::shared_ptr<RBX::DataModel>)
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
pub use crate::instance::stub_0x3ecf0 as stub_3ecf0;
// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub use crate::instance::stub_0x3f094 as stub_3f094;
// 0x40318 — __ZN5boost8weak_ptrIN3RBX9DataModelEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::DataModel>::weak_ptr<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)
#[doc(alias = "Weak<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")]
pub use crate::instance::stub_0x40318 as stub_40318;
// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
pub use crate::instance::stub_0x49e7c as stub_49e7c;
// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub use crate::instance::stub_0x4b164 as stub_4b164;
// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
pub use crate::instance::stub_0x4b374 as stub_4b374;
// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
pub use crate::instance::stub_0x4b418 as stub_4b418;
// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
pub use crate::instance::stub_0x4b4bc as stub_4b4bc;
// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
pub use crate::instance::stub_0x4b4c0 as stub_4b4c0;
// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
pub use crate::instance::stub_0x4b5b8 as stub_4b5b8;
// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub use crate::instance::stub_0x4b6b4 as stub_4b6b4;
// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
pub use crate::instance::stub_0x4b788 as stub_4b788;
// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
pub use crate::instance::stub_0x4b860 as stub_4b860;
// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
pub use crate::instance::stub_0x4b970 as stub_4b970;
// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub use crate::instance::stub_0x4b97c as stub_4b97c;
// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
pub use crate::instance::stub_0x4b984 as stub_4b984;
// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
pub use crate::instance::stub_0x4b98c as stub_4b98c;
// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
pub use crate::instance::stub_0x4ba50 as stub_4ba50;
// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
pub use crate::instance::stub_0x4bb40 as stub_4bb40;
// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
pub use crate::instance::stub_0x4bb44 as stub_4bb44;
// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub use crate::instance::stub_0x4bc34 as stub_4bc34;
// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
pub use crate::instance::stub_0x4bd08 as stub_4bd08;
// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub use crate::instance::stub_0x4bde0 as stub_4bde0;
// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
pub use crate::instance::stub_0x4be8c as stub_4be8c;
// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
pub use crate::instance::stub_0x4bf3c as stub_4bf3c;
// 0x4bf6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
pub use crate::instance::stub_0x4bf6c as stub_4bf6c;
// 0x4bfcc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")]
pub use crate::instance::stub_0x4bfcc as stub_4bfcc;
// 0x258688 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11HttpServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService> RBX::Creatable<RBX::Instance>::create<RBX::HttpService>(void)")]
pub use crate::instance::stub_0x258688 as stub_258688;
// 0x258738 — __ZN5boost10shared_ptrIN3RBX11HttpServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpService>::shared_ptr<RBX::HttpService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x258738 as stub_258738;
// 0x2588e8 — __ZN5boost6detail12shared_countC2IPN3RBX11HttpServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x2588e8 as stub_2588e8;
// 0x2589f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x2589f0 as stub_2589f0;
// 0x2589f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x2589f4 as stub_2589f4;
// 0x2589f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x2589f8 as stub_2589f8;
// 0x258a18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x258a18 as stub_258a18;
// 0x258a30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11HttpServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x258a30 as stub_258a30;
// 0x25bc38 — __ZNK3RBX5Light12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Light::askSetParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x25bc38 as stub_25bc38;
// 0x25bc60 — __ZNK3RBX5Light11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Light::askAddChild(RBX::Instance const*)const")]
pub use crate::instance::stub_0x25bc60 as stub_25bc60;
// 0x25c4d0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9SpotLightEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight> RBX::Creatable<RBX::Instance>::create<RBX::SpotLight>(void)")]
pub use crate::instance::stub_0x25c4d0 as stub_25c4d0;
// 0x25c580 — __ZN5boost10shared_ptrIN3RBX9SpotLightEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::SpotLight>::shared_ptr<RBX::SpotLight,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x25c580 as stub_25c580;
// 0x25c730 — __ZN5boost6detail12shared_countC2IPN3RBX9SpotLightENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SpotLight *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x25c730 as stub_25c730;
