//! platform — generated_watchdog_platB_1788386334 — 120 stubs EA-sorted asc Http/Platform/TaskScheduler/RunService
//! Source: ida/export.json (85545 funcs) filter Http|Platform|TaskScheduler|RunService skip /tmp/global_eas.txt EA-sorted asc global dedup 66066/85545
//! Batch: 120 stubs | range 0x7e4644..0xf2da24 | rbx_core::SharedPtr not boost | iOS App already fully deduped (515/515 Platform/iOS in platform)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x7e4644 — RBX::ContentProviderJob::addTask(std::string const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::ContentProviderJob::addTask(std::string const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBX18ContentProviderJob7addTaskERKSsNS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIS1_EE")]
pub fn stub_7e4644() -> ! {
    todo!("0x7e4644 __ZN3RBX18ContentProviderJob7addTaskERKSsNS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIS1_EE")
}

// 0x7e4ce4 — RBX::ContentProviderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: _DWORD __fastcall(RBX::ContentProviderJob *__hidden this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::ContentProviderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX18ContentProviderJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_7e4ce4() -> ! {
    todo!("0x7e4ce4 __ZN3RBX18ContentProviderJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0x7e4d00 — RBX::ContentProviderJob::error(RBX::TaskScheduler::Job::Stats const&)
#[doc(alias = "RBX::ContentProviderJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX18ContentProviderJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_7e4d00() -> ! {
    todo!("0x7e4d00 __ZN3RBX18ContentProviderJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0x7e4ff8 — boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::operator()(std::string,boost::shared_ptr<std::string const>)const
#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::operator()(std::string,boost::shared_ptr<std::string const>)const")]
#[doc(alias = "__ZNK5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEclESsS6_")]
pub fn stub_7e4ff8() -> ! {
    todo!("0x7e4ff8 __ZNK5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEclESsS6_")
}

// 0x7e5abc — boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to_own(boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>> const&)
#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to_own(boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>> const&)")]
#[doc(alias = "__ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE13assign_to_ownERKS7_")]
pub fn stub_7e5abc() -> ! {
    todo!("0x7e5abc __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE13assign_to_ownERKS7_")
}

// 0x7ecdb0 — RBX::ContentProvider::privateLoadContent(RBX::ContentId &,RBX::ContentProvider::RequestType,float,RBX::ContentProvider::CachedContent *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob)
// type: int __fastcall(int, int, int, int, int, int, int)
#[doc(alias = "RBX::ContentProvider::privateLoadContent(RBX::ContentId &,RBX::ContentProvider::RequestType,float,RBX::ContentProvider::CachedContent *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob)")]
#[doc(alias = "__ZN3RBX15ContentProvider18privateLoadContentERNS_9ContentIdENS0_11RequestTypeEfPNS0_13CachedContentEPN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS6_10shared_ptrIKSsEEEEENS8_9ResultJobE")]
pub fn stub_7ecdb0() -> ! {
    todo!("0x7ecdb0 __ZN3RBX15ContentProvider18privateLoadContentERNS_9ContentIdENS0_11RequestTypeEfPNS0_13CachedContentEPN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS6_10shared_ptrIKSsEEEEENS8_9ResultJobE")
}

// 0x7ed940 — RBX::ContentProvider::getContent(RBX::ContentId const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::ResultJob)
// type: int __fastcall(char, int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "RBX::ContentProvider::getContent(RBX::ContentId const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::ResultJob)")]
#[doc(alias = "__ZN3RBX15ContentProvider10getContentERKNS_9ContentIdEfN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS4_10shared_ptrIKSsEEEEENS6_9ResultJobE")]
pub fn stub_7ed940() -> ! {
    todo!("0x7ed940 __ZN3RBX15ContentProvider10getContentERKNS_9ContentIdEfN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS4_10shared_ptrIKSsEEEEENS6_9ResultJobE")
}

// 0x7ee158 — RBX::InvokeFileCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>)
#[doc(alias = "RBX::InvokeFileCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBXL18InvokeFileCallbackEN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS0_10shared_ptrIKSsEEEEES7_")]
pub fn stub_7ee158() -> ! {
    todo!("0x7ee158 __ZN3RBXL18InvokeFileCallbackEN5boost8functionIFvNS_14AsyncHttpQueue13RequestResultEPSiNS0_10shared_ptrIKSsEEEEES7_")
}

// 0x7ee300 — RBX::ContentProvider::requestContentFile(RBX::ContentId const&,float,RBX::AsyncHttpQueue::RequestResult &,std::string &)
// type: int __fastcall(int, int, int, int, std::string *)
#[doc(alias = "RBX::ContentProvider::requestContentFile(RBX::ContentId const&,float,RBX::AsyncHttpQueue::RequestResult &,std::string &)")]
#[doc(alias = "__ZN3RBX15ContentProvider18requestContentFileERKNS_9ContentIdEfRNS_14AsyncHttpQueue13RequestResultERSs")]
pub fn stub_7ee300() -> ! {
    todo!("0x7ee300 __ZN3RBX15ContentProvider18requestContentFileERKNS_9ContentIdEfRNS_14AsyncHttpQueue13RequestResultERSs")
}

// 0x7f04e0 — RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::setCacheSize(int)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::setCacheSize(int)")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE12setCacheSizeEi")]
pub fn stub_7f04e0() -> ! {
    todo!("0x7f04e0 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE12setCacheSizeEi")
}

// 0x7f0628 — void boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::reset<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)
#[doc(alias = "void boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::reset<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEE5resetIS5_EEvPT_")]
pub fn stub_7f0628() -> ! {
    todo!("0x7f0628 __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEE5resetIS5_EEvPT_")
}

// 0x7f0684 — RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::insertCacheItem(std::string const&,RBX::ContentProvider::CachedContent const&)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::insertCacheItem(std::string const&,RBX::ContentProvider::CachedContent const&)")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15insertCacheItemERKSsRKS2_")]
pub fn stub_7f0684() -> ! {
    todo!("0x7f0684 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15insertCacheItemERKSsRKS2_")
}

// 0x7f0a54 — boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list_av_2<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list_av_2<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SA_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_")]
pub fn stub_7f0a54() -> ! {
    todo!("0x7f0a54 __ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SA_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_")
}

// 0x7f0db4 — RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::findCacheItem(std::string const&,RBX::ContentProvider::CachedContent*)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::findCacheItem(std::string const&,RBX::ContentProvider::CachedContent*)")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE13findCacheItemERKSsPS2_")]
pub fn stub_7f0db4() -> ! {
    todo!("0x7f0db4 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE13findCacheItemERKSsPS2_")
}

// 0x7f3dac — boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_7f3dac() -> ! {
    todo!("0x7f3dac __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

// 0x7f42a0 — boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_7f42a0() -> ! {
    todo!("0x7f42a0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0x7f4458 — boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")]
pub fn stub_7f4458() -> ! {
    todo!("0x7f4458 __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")
}

// 0x7f4560 — boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")]
pub fn stub_7f4560() -> ! {
    todo!("0x7f4560 __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")
}

// 0x7f5aa4 — boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_")]
pub fn stub_7f5aa4() -> ! {
    todo!("0x7f5aa4 __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_")
}

// 0x7f5b8c — void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_7f5b8c() -> ! {
    todo!("0x7f5b8c __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x7f5c70 — boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_")]
pub fn stub_7f5c70() -> ! {
    todo!("0x7f5c70 __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_")
}

// 0x7f5d68 — boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED1Ev")]
pub fn stub_7f5d68() -> ! {
    todo!("0x7f5d68 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED1Ev")
}

// 0x7f5d6c — boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED0Ev")]
pub fn stub_7f5d6c() -> ! {
    todo!("0x7f5d6c __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED0Ev")
}

// 0x7f5d70 — boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE7disposeEv")]
pub fn stub_7f5d70() -> ! {
    todo!("0x7f5d70 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE7disposeEv")
}

// 0x7f5d80 — boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE11get_deleterERKSt9type_info")]
pub fn stub_7f5d80() -> ! {
    todo!("0x7f5d80 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE11get_deleterERKSt9type_info")
}

// 0x7f5d84 — boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE19get_untyped_deleterEv")]
pub fn stub_7f5d84() -> ! {
    todo!("0x7f5d84 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE19get_untyped_deleterEv")
}

// 0x7f5ed8 — RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED1Ev")]
pub fn stub_7f5ed8() -> ! {
    todo!("0x7f5ed8 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED1Ev")
}

// 0x7f5fe0 — RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED0Ev")]
pub fn stub_7f5fe0() -> ! {
    todo!("0x7f5fe0 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED0Ev")
}

// 0x7f60f8 — RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_")]
pub fn stub_7f60f8() -> ! {
    todo!("0x7f60f8 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_")
}

// 0x7f8928 — boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to_own(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>> const&)
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to_own(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>> const&)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE13assign_to_ownERKS8_")]
pub fn stub_7f8928() -> ! {
    todo!("0x7f8928 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE13assign_to_ownERKS8_")
}

// 0x7f8b44 — boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::operator()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)const
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::operator()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)const")]
#[doc(alias = "__ZNK5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEclES3_S4_S7_")]
pub fn stub_7f8b44() -> ! {
    todo!("0x7f8b44 __ZNK5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEclES3_S4_S7_")
}

// 0x818804 — RBX::LibraryService::contentReady(std::string const&,std::string const&,RBX::AsyncHttpQueue::RequestResult,std::string const*)
#[doc(alias = "RBX::LibraryService::contentReady(std::string const&,std::string const&,RBX::AsyncHttpQueue::RequestResult,std::string const*)")]
#[doc(alias = "__ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_")]
pub fn stub_818804() -> ! {
    todo!("0x818804 __ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_")
}

// 0x899020 — RBX::CacheableContentProvider::requestContent(RBX::ContentId const&,float,bool,RBX::AsyncHttpQueue::RequestResult &)
// type: void __fastcall(_QWORD *, int32_t *, const std::string *, int, int, _DWORD *)
#[doc(alias = "RBX::CacheableContentProvider::requestContent(RBX::ContentId const&,float,bool,RBX::AsyncHttpQueue::RequestResult &)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider14requestContentERKNS_9ContentIdEfbRNS_14AsyncHttpQueue13RequestResultE")]
pub fn stub_899020() -> ! {
    todo!("0x899020 __ZN3RBX24CacheableContentProvider14requestContentERKNS_9ContentIdEfbRNS_14AsyncHttpQueue13RequestResultE")
}

// 0x8995c8 — RBX::CacheableContentProvider::LoadContentCallbackHelper(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)
// type: void __fastcall(int, int, int, const shared_count *, std::string *)
#[doc(alias = "RBX::CacheableContentProvider::LoadContentCallbackHelper(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider25LoadContentCallbackHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEESs")]
pub fn stub_8995c8() -> ! {
    todo!("0x8995c8 __ZN3RBX24CacheableContentProvider25LoadContentCallbackHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEESs")
}

// 0x899774 — RBX::CacheableContentProvider::LoadContentCallback(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)
// type: void __fastcall(int, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CacheableContentProvider::LoadContentCallback(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider19LoadContentCallbackENS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIKSsEESs")]
pub fn stub_899774() -> ! {
    todo!("0x899774 __ZN3RBX24CacheableContentProvider19LoadContentCallbackENS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIKSsEESs")
}

// 0x899ab4 — boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::TaskScheduler::StepResult,boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>(RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>)
// type: void __fastcall(_DWORD *, int, int *)
#[doc(alias = "boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::TaskScheduler::StepResult,boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>(RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIN3RBX13TaskScheduler10StepResultENS_8weak_ptrINS1_24CacheableContentProviderEEERKSsNS_10shared_ptrIS7_EES6_NS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_PFSG_T0_T1_T2_ENSE_9list_av_3IT3_T4_T5_E4typeEEESL_SN_SO_SP_")]
pub fn stub_899ab4() -> ! {
    todo!("0x899ab4 __ZN5boost4bindIN3RBX13TaskScheduler10StepResultENS_8weak_ptrINS1_24CacheableContentProviderEEERKSsNS_10shared_ptrIS7_EES6_NS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_PFSG_T0_T1_T2_ENSE_9list_av_3IT3_T4_T5_E4typeEEESL_SN_SO_SP_")
}

// 0x89a018 — boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list_av_5<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>::type> boost::bind<void,boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>(void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string)
// type: void __fastcall(_DWORD *, int, int *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list_av_5<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>::type> boost::bind<void,boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>(void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsS4_NS_3argILi1EEENSB_ILi2EEENSB_ILi3EEESsEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_ENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESO_SQ_SR_SS_ST_SU_")]
pub fn stub_89a018() -> ! {
    todo!("0x89a018 __ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsS4_NS_3argILi1EEENSB_ILi2EEENSB_ILi3EEESsEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_ENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESO_SQ_SR_SS_ST_SU_")
}

// 0x89a6b0 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSJ_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSJ_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_89a6b0() -> ! {
    todo!("0x89a6b0 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSL_ILi2EEENSL_ILi3EEENSJ_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

// 0x89a870 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_89a870() -> ! {
    todo!("0x89a870 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0x89aa34 — void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>)
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEEvT_")]
pub fn stub_89aa34() -> ! {
    todo!("0x89aa34 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEEvT_")
}

// 0x89ac08 — boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
pub fn stub_89ac08() -> ! {
    todo!("0x89ac08 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")
}

// 0x89ac24 — boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_")]
pub fn stub_89ac24() -> ! {
    todo!("0x89ac24 __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_")
}

// 0x89ac48 — bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int *, void *)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_89ac48() -> ! {
    todo!("0x89ac48 __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE")
}

// 0x89ae0c — bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, void *)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_89ae0c() -> ! {
    todo!("0x89ae0c __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x89afcc — void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: void __fastcall(int, _DWORD *, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_89afcc() -> ! {
    todo!("0x89afcc __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

// 0x89b110 — void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::operator()<void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&> &,int)
// type: void __fastcall(int *, struct _Unwind_Exception **, int **)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::operator()<void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_89b110() -> ! {
    todo!("0x89b110 __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x89b310 — boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: void __fastcall(_DWORD **, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_89b310() -> ! {
    todo!("0x89b310 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0x89c154 — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::ContentProviderJob,RBX::ContentProviderJob>(boost::shared_ptr<RBX::ContentProviderJob> const*,RBX::ContentProviderJob *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::ContentProviderJob,RBX::ContentProviderJob>(boost::shared_ptr<RBX::ContentProviderJob> const*,RBX::ContentProviderJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_18ContentProviderJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_89c154() -> ! {
    todo!("0x89c154 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_18ContentProviderJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0x89cb94 — __ZN5boost8functionIFN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENSA_5list3INSA_5valueISE_EENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENSA_5list3INSA_5valueISE_EENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_89cb94() -> ! {
    todo!("0x89cb94 __ZN5boost8functionIFN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENSA_5list3INSA_5valueISE_EENS_3argILi1EEENSL_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x89ccb8 — __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_89ccb8() -> ! {
    todo!("0x89ccb8 __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x89cde4 — void boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>)
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "__ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_")]
pub fn stub_89cde4() -> ! {
    todo!("0x89cde4 __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_")
}

// 0x89cf1c — boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
pub fn stub_89cf1c() -> ! {
    todo!("0x89cf1c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")
}

// 0x89cf38 — boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,std::string,boost::shared_ptr<std::string const>)
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,std::string,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEES7_SsSE_E6invokeERNS1_15function_bufferESsSE_")]
pub fn stub_89cf38() -> ! {
    todo!("0x89cf38 __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEES7_SsSE_E6invokeERNS1_15function_bufferESsSE_")
}

// 0x89cf54 — bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_89cf54() -> ! {
    todo!("0x89cf54 __ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

// 0x89d074 — bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_89d074() -> ! {
    todo!("0x89d074 __ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x89d1f0 — RBX::TaskScheduler::StepResult boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&>>(boost::_bi::type<RBX::TaskScheduler::StepResult>,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>) &,boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&> &,long)
// type: int __fastcall(int *, int (__fastcall **)(int *, _DWORD, int *), __int64 *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::StepResult boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&>>(boost::_bi::type<RBX::TaskScheduler::StepResult>,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>) &,boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEclINS4_13TaskScheduler10StepResultEPFSE_S6_RKSsNS_10shared_ptrISF_EEENS0_5list2IRSsRSI_EEEET_NS0_4typeISP_EERT0_RT1_l")]
pub fn stub_89d1f0() -> ! {
    todo!("0x89d1f0 __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEclINS4_13TaskScheduler10StepResultEPFSE_S6_RKSsNS_10shared_ptrISF_EEENS0_5list2IRSsRSI_EEEET_NS0_4typeISP_EERT0_RT1_l")
}

// 0x89d350 — boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
pub fn stub_89d350() -> ! {
    todo!("0x89d350 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")
}

// 0x89eee4 — boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::clear(void)
// type: int __fastcall(int *)
#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE5clearEv")]
pub fn stub_89eee4() -> ! {
    todo!("0x89eee4 __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE5clearEv")
}

// 0x8eca94 — RBX::OnScreenProfiler::UpdateJobStart(RBX::TaskScheduler::Job *)
// type: int __fastcall(RBX::OnScreenProfiler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::OnScreenProfiler::UpdateJobStart(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler14UpdateJobStartEPNS_13TaskScheduler3JobE")]
pub fn stub_8eca94() -> ! {
    todo!("0x8eca94 __ZN3RBX16OnScreenProfiler14UpdateJobStartEPNS_13TaskScheduler3JobE")
}

// 0x8ecb00 — RBX::OnScreenProfiler::AllocateNewJobInfo(RBX::TaskScheduler::Job *)
// type: char *__fastcall(RBX::OnScreenProfiler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::OnScreenProfiler::AllocateNewJobInfo(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler18AllocateNewJobInfoEPNS_13TaskScheduler3JobE")]
pub fn stub_8ecb00() -> ! {
    todo!("0x8ecb00 __ZN3RBX16OnScreenProfiler18AllocateNewJobInfoEPNS_13TaskScheduler3JobE")
}

// 0x8ece38 — RBX::OnScreenProfiler::UpdateJobEnd(RBX::TaskScheduler::Job *)
// type: int __fastcall(RBX::OnScreenProfiler *this, RBX::TaskScheduler::Job *)
#[doc(alias = "RBX::OnScreenProfiler::UpdateJobEnd(RBX::TaskScheduler::Job *)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler12UpdateJobEndEPNS_13TaskScheduler3JobE")]
pub fn stub_8ece38() -> ! {
    todo!("0x8ece38 __ZN3RBX16OnScreenProfiler12UpdateJobEndEPNS_13TaskScheduler3JobE")
}

// 0x90d4e0 — RBX::Http::Http(std::string const&)
// type: _DWORD __fastcall(RBX::Http *__hidden this, const std::string *)
#[doc(alias = "RBX::Http::Http(std::string const&)")]
#[doc(alias = "__ZN3RBX4HttpC2ERKSs")]
pub fn stub_90d4e0() -> ! {
    todo!("0x90d4e0 __ZN3RBX4HttpC2ERKSs")
}

// 0x9c58dc — RBX::Network::PhysicsSender::TouchJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender8TouchJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c58dc() -> ! {
    todo!("0x9c58dc __ZN3RBX7Network13PhysicsSender8TouchJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0x9c58fc — RBX::Network::PhysicsSender::TouchJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::PhysicsSender::TouchJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::PhysicsSender::TouchJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender8TouchJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c58fc() -> ! {
    todo!("0x9c58fc __ZN3RBX7Network13PhysicsSender8TouchJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0x9c6214 — RBX::Network::PhysicsSender::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PhysicsSender::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c6214() -> ! {
    todo!("0x9c6214 __ZN3RBX7Network13PhysicsSender3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0x9c6234 — RBX::Network::PhysicsSender::Job::error(RBX::TaskScheduler::Job::Stats const&)
// type: int __fastcall(RBX::Network::PhysicsSender::Job *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::PhysicsSender::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network13PhysicsSender3Job5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_9c6234() -> ! {
    todo!("0x9c6234 __ZN3RBX7Network13PhysicsSender3Job5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0x9cb36c — boost::shared_ptr<RBX::TaskScheduler::Job>::reset(void)
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job>::reset(void)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEE5resetEv")]
pub fn stub_9cb36c() -> ! {
    todo!("0x9cb36c __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEE5resetEv")
}

// 0xa51fe4 — RBX::Http::~Http()
// type: void __fastcall(RBX::Http *__hidden this)
#[doc(alias = "RBX::Http::~Http()")]
#[doc(alias = "__ZN3RBX4HttpD2Ev")]
pub fn stub_a51fe4() -> ! {
    todo!("0xa51fe4 __ZN3RBX4HttpD2Ev")
}

// 0xaa346c — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS4_INS1_13ModelInstanceEEES3_SA_ENSE_5list3INSE_5valueISH_EENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS4_INS1_13ModelInstanceEEES3_SA_ENSE_5list3INSE_5valueISH_EENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aa346c() -> ! {
    todo!("0xaa346c __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS4_INS1_13ModelInstanceEEES3_SA_ENSE_5list3INSE_5valueISH_EENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

// 0xacba9c — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSE_5list6INSE_5valueISJ_EENS_3argILi1EEENSP_ILi2EEENSN_ISsEENSN_IbEENSN_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSE_5list6INSE_5valueISJ_EENS_3argILi1EEENSP_ILi2EEENSN_ISsEENSN_IbEENSN_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")]
pub fn stub_acba9c() -> ! {
    todo!("0xacba9c __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_7Network6PlayerEEES3_SA_SsbdENSE_5list6INSE_5valueISJ_EENS_3argILi1EEENSP_ILi2EEENSN_ISsEENSN_IbEENSN_IdEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")
}

// 0xad622c — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PacketReceiveJob,RBX::Network::PacketReceiveJob>(boost::shared_ptr<RBX::Network::PacketReceiveJob> const*,RBX::Network::PacketReceiveJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PacketReceiveJob,RBX::Network::PacketReceiveJob>(boost::shared_ptr<RBX::Network::PacketReceiveJob> const*,RBX::Network::PacketReceiveJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16PacketReceiveJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_ad622c() -> ! {
    todo!("0xad622c __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16PacketReceiveJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xad74f8 — RBX::Network::PacketReceiveJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::PacketReceiveJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::PacketReceiveJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network16PacketReceiveJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_ad74f8() -> ! {
    todo!("0xad74f8 __ZN3RBX7Network16PacketReceiveJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0xad7514 — RBX::Network::PacketReceiveJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::PacketReceiveJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network16PacketReceiveJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_ad7514() -> ! {
    todo!("0xad7514 __ZN3RBX7Network16PacketReceiveJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0xadcc60 — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver::Job> const*,RBX::Network::InterpolatingPhysicsReceiver::Job *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver::Job> const*,RBX::Network::InterpolatingPhysicsReceiver::Job *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_adcc60() -> ! {
    todo!("0xadcc60 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xade234 — RBX::Network::InterpolatingPhysicsReceiver::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_ade234() -> ! {
    todo!("0xade234 __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0xade250 — RBX::Network::InterpolatingPhysicsReceiver::Job::error(RBX::TaskScheduler::Job::Stats const&)
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_ade250() -> ! {
    todo!("0xade250 __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0xae5f4c — RBX::Network::Replicator::SendDataJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: int __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::Replicator::SendDataJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_ae5f4c() -> ! {
    todo!("0xae5f4c __ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0xae603c — RBX::Network::Replicator::SendClusterJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: int __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_ae603c() -> ! {
    todo!("0xae603c __ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb06f18 — RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>)
// type: void __fastcall(int, int *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::remove(boost::shared_ptr<RBX::TaskScheduler::Job>)")]
#[doc(alias = "__ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE")]
pub fn stub_b06f18() -> ! {
    todo!("0xb06f18 __ZN3RBX13TaskScheduler6removeEN5boost10shared_ptrINS0_3JobEEE")
}

// 0xb0d0b4 — RBX::Network::Replicator::SendDataJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::SendDataJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator11SendDataJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b0d0b4() -> ! {
    todo!("0xb0d0b4 __ZN3RBX7Network10Replicator11SendDataJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb0dcbc — RBX::Network::Replicator::SendClusterJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator14SendClusterJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b0dcbc() -> ! {
    todo!("0xb0dcbc __ZN3RBX7Network10Replicator14SendClusterJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb23e88 — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(boost::shared_ptr<RBX::Network::Replicator::PingJob> const*,RBX::Network::Replicator::PingJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(boost::shared_ptr<RBX::Network::Replicator::PingJob> const*,RBX::Network::Replicator::PingJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_b23e88() -> ! {
    todo!("0xb23e88 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xb24310 — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(boost::shared_ptr<RBX::Network::Replicator::ProcessPacketsJob> const*,RBX::Network::Replicator::ProcessPacketsJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(boost::shared_ptr<RBX::Network::Replicator::ProcessPacketsJob> const*,RBX::Network::Replicator::ProcessPacketsJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_b24310() -> ! {
    todo!("0xb24310 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xb24798 — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(boost::shared_ptr<RBX::Network::Replicator::SendClusterJob> const*,RBX::Network::Replicator::SendClusterJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(boost::shared_ptr<RBX::Network::Replicator::SendClusterJob> const*,RBX::Network::Replicator::SendClusterJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_b24798() -> ! {
    todo!("0xb24798 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xb24c20 — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(boost::shared_ptr<RBX::Network::Replicator::SendDataJob> const*,RBX::Network::Replicator::SendDataJob *)const
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(boost::shared_ptr<RBX::Network::Replicator::SendDataJob> const*,RBX::Network::Replicator::SendDataJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_b24c20() -> ! {
    todo!("0xb24c20 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xb32a10 — RBX::Network::Replicator::PingJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::Replicator::PingJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::PingJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b32a10() -> ! {
    todo!("0xb32a10 __ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb32a2c — RBX::Network::Replicator::PingJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::Replicator::PingJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b32a2c() -> ! {
    todo!("0xb32a2c __ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb33080 — RBX::Network::Replicator::ProcessPacketsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b33080() -> ! {
    todo!("0xb33080 __ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb33128 — RBX::Network::Replicator::ProcessPacketsJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b33128() -> ! {
    todo!("0xb33128 __ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb5bf90 — RBX::Network::Replicator::StreamJob::error(RBX::TaskScheduler::Job::Stats const&)
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b5bf90() -> ! {
    todo!("0xb5bf90 __ZN3RBX7Network10Replicator9StreamJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

// 0xb5f4dc — RBX::Network::Replicator::StreamJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::StreamJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_b5f4dc() -> ! {
    todo!("0xb5f4dc __ZN3RBX7Network10Replicator9StreamJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

// 0xf1f444 — __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD2Ev$shim
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD2Ev$shim")]
pub fn stub_f1f444() -> ! {
    todo!("0xf1f444 __ZN3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7CreatorD2Ev$shim")
}

// 0xf1f450 — __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv$shim")]
pub fn stub_f1f450() -> ! {
    todo!("0xf1f450 __ZNK3RBX14FactoryProductINS_11HttpServiceENS_8InstanceELZNS_12sHttpServiceEES2_E7Creator12getClassNameEv$shim")
}

// 0xf1fbb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fbb8() -> ! {
    todo!("0xf1fbb8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf1fbc4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f1fbc4() -> ! {
    todo!("0xf1fbc4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf212d4 — __ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD2Ev$shim
#[doc(alias = "__ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD2Ev$shim")]
pub fn stub_f212d4() -> ! {
    todo!("0xf212d4 __ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD2Ev$shim")
}

// 0xf21334 — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv$shim
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv$shim")]
pub fn stub_f21334() -> ! {
    todo!("0xf21334 __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv$shim")
}

// 0xf213e8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_$shim")]
pub fn stub_f213e8() -> ! {
    todo!("0xf213e8 __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_$shim")
}

// 0xf213f4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv$shim")]
pub fn stub_f213f4() -> ! {
    todo!("0xf213f4 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv$shim")
}

// 0xf21820 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21820() -> ! {
    todo!("0xf21820 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE14doGetSingletonEv$shim")
}

// 0xf2182c — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED2Ev$shim")]
pub fn stub_f2182c() -> ! {
    todo!("0xf2182c __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED2Ev$shim")
}

// 0xf2269c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f2269c() -> ! {
    todo!("0xf2269c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13InsertServiceEEENS6_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSB_INS6_8InstanceEEESaISE_EEEENS_8functionIFvSE_EEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf2b904 — boost::shared_ptr<RBX::RunService>::operator=(boost::shared_ptr<RBX::RunService> const&)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::shared_ptr<RBX::RunService>::operator=(boost::shared_ptr<RBX::RunService> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_")]
pub fn stub_f2b904() -> ! {
    todo!("0xf2b904 j___ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_")
}

// 0xf2b9c4 — boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::GcJob>(RBX::GcJob *)
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job>::shared_ptr<RBX::GcJob>(RBX::GcJob *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_")]
pub fn stub_f2b9c4() -> ! {
    todo!("0xf2b9c4 j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_")
}

// 0xf2b9d4 — boost::shared_ptr<RBX::TaskScheduler::Job>::operator=(boost::shared_ptr<RBX::TaskScheduler::Job> const&)
#[doc(alias = "boost::shared_ptr<RBX::TaskScheduler::Job>::operator=(boost::shared_ptr<RBX::TaskScheduler::Job> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_")]
pub fn stub_f2b9d4() -> ! {
    todo!("0xf2b9d4 j___ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_")
}

// 0xf2c434 — void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::GcJob>(boost::shared_ptr<RBX::TaskScheduler::Job> const*,RBX::GcJob *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::GcJob>(boost::shared_ptr<RBX::TaskScheduler::Job> const*,RBX::GcJob *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f2c434() -> ! {
    todo!("0xf2c434 j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_")
}

// 0xf2d8a4 — RBX::HttpQueueStatsItem::init(void)
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::init(void)")]
#[doc(alias = "j___ZN3RBX18HttpQueueStatsItem4initEv")]
pub fn stub_f2d8a4() -> ! {
    todo!("0xf2d8a4 j___ZN3RBX18HttpQueueStatsItem4initEv")
}

// 0xf2d8e4 — boost::weak_ptr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)
#[doc(alias = "boost::weak_ptr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
#[doc(alias = "j___ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_")]
pub fn stub_f2d8e4() -> ! {
    todo!("0xf2d8e4 j___ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_")
}

// 0xf2d8f4 — boost::shared_ptr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(boost::weak_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)
#[doc(alias = "boost::shared_ptr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(boost::weak_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_f2d8f4() -> ! {
    todo!("0xf2d8f4 j___ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

// 0xf2d914 — boost::shared_ptr<RBX::HttpQueueStatsItem>::operator=(boost::shared_ptr<RBX::HttpQueueStatsItem> const&)
#[doc(alias = "boost::shared_ptr<RBX::HttpQueueStatsItem>::operator=(boost::shared_ptr<RBX::HttpQueueStatsItem> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEaSERKS3_")]
pub fn stub_f2d914() -> ! {
    todo!("0xf2d914 j___ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEaSERKS3_")
}

// 0xf2d924 — boost::shared_ptr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)
#[doc(alias = "boost::shared_ptr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_")]
pub fn stub_f2d924() -> ! {
    todo!("0xf2d924 j___ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_")
}

// 0xf2d934 — boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)
#[doc(alias = "boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_")]
pub fn stub_f2d934() -> ! {
    todo!("0xf2d934 j___ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_")
}

// 0xf2d974 — boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")]
pub fn stub_f2d974() -> ! {
    todo!("0xf2d974 j___ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")
}

// 0xf2d994 — boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")]
pub fn stub_f2d994() -> ! {
    todo!("0xf2d994 j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")
}

// 0xf2d9a4 — void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list1<boost::shared_ptr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex>&> &,int)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list1<boost::shared_ptr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex>&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_f2d9a4() -> ! {
    todo!("0xf2d9a4 j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0xf2d9d4 — boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)")]
#[doc(alias = "j___ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_")]
pub fn stub_f2d9d4() -> ! {
    todo!("0xf2d9d4 j___ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_")
}

// 0xf2d9e4 — boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_")]
pub fn stub_f2d9e4() -> ! {
    todo!("0xf2d9e4 j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_")
}

// 0xf2d9f4 — boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")]
pub fn stub_f2d9f4() -> ! {
    todo!("0xf2d9f4 j___ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")
}

// 0xf2da04 — boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
#[doc(alias = "j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")]
pub fn stub_f2da04() -> ! {
    todo!("0xf2da04 j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")
}

// 0xf2da24 — boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)
// type: int __fastcall(int, int, int, char, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>)")]
#[doc(alias = "j___ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_")]
pub fn stub_f2da24() -> ! {
    todo!("0xf2da24 j___ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_")
}

