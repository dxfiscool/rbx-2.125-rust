//! core bg10 — 120 core stubs EA-sorted asc not yet in crates/core.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) int-deduped vs crates/core stubs — next 120 uncovered 0x2c3fb0..0x3a5158.
//! Sanitized: single quotes, backticks, double quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::WaitingScriptsJob::WaitingScriptsJob(boost::shared_ptr<RBX::ScriptContext>)")]
#[doc(alias = "__ZN3RBX17WaitingScriptsJobC2EN5boost10shared_ptrINS_13ScriptContextEEE")]
// 0x2c3fb0 — __ZN3RBX17WaitingScriptsJobC2EN5boost10shared_ptrINS_13ScriptContextEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
pub fn stub_0x2c3fb0() {
    // IDA 0x2c3fb0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::shared_ptr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext>(boost::weak_ptr<RBX::ScriptContext> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// 0x2c44c0 — __ZN5boost10shared_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: 
pub fn stub_0x2c44c0() {
    // IDA 0x2c44c0: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "RBX::GcJob::GcJob(boost::shared_ptr<RBX::ScriptContext>)")]
#[doc(alias = "__ZN3RBX5GcJobC2EN5boost10shared_ptrINS_13ScriptContextEEE")]
// 0x2c453c — __ZN3RBX5GcJobC2EN5boost10shared_ptrINS_13ScriptContextEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, RBX::TaskScheduler::Job *, int, int, int, int)
pub fn stub_0x2c453c() {
    // IDA 0x2c453c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE")]
// 0x2c78a8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: 
pub fn stub_0x2c78a8() {
    // IDA 0x2c78a8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS3_5list1INS3_5valueIPS8_EEEEEEiE6invokeERNS1_15function_bufferE")]
// 0x2c7908 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS3_5list1INS3_5valueIPS8_EEEEEEiE6invokeERNS1_15function_bufferE
// type: 
pub fn stub_0x2c7908() {
    // IDA 0x2c7908: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
// 0x2c790c — __ZN5boost3_bi6bind_tImNS_4_mfi4cmf0ImN3RBX13ScriptContextEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: 
pub fn stub_0x2c790c() {
    // IDA 0x2c790c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::shared_ptr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_")]
// 0x2f79c8 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16BoxSelectCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: 
pub fn stub_0x2f79c8() {
    // IDA 0x2f79c8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::ScriptInformationProvider::HandleHttpResponse(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>)")]
#[doc(alias = "__ZN3RBX25ScriptInformationProvider18HandleHttpResponseEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS1_8functionIFvNS0_13RequestResultEbbfbEEE")]
// 0x36a724 — __ZN3RBX25ScriptInformationProvider18HandleHttpResponseEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultESsNS1_8functionIFvNS0_13RequestResultEbbfbEEE
// type: void __fastcall(int, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x36a724() {
    // IDA 0x36a724: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::ScriptInformationProvider::getScriptInfo(std::string const&,bool,float,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::AsyncHttpQueue::ResultJob)")]
#[doc(alias = "__ZN3RBX25ScriptInformationProvider13getScriptInfoERKSsbfN5boost8functionIFvNS0_13RequestResultEbbfbEEENS_14AsyncHttpQueue9ResultJobE")]
// 0x36a87c — __ZN3RBX25ScriptInformationProvider13getScriptInfoERKSsbfN5boost8functionIFvNS0_13RequestResultEbbfbEEENS_14AsyncHttpQueue9ResultJobE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const std::string *, int, int, int, int)
pub fn stub_0x36a87c() {
    // IDA 0x36a87c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "RBX::ScriptInformationProvider::CachedScriptInfo::CachedScriptInfo(boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBX25ScriptInformationProvider16CachedScriptInfoC2EN5boost10shared_ptrIKSsEES5_")]
// 0x36b030 — __ZN3RBX25ScriptInformationProvider16CachedScriptInfoC2EN5boost10shared_ptrIKSsEES5_
// type: int __fastcall(int, _DWORD **, _DWORD *)
pub fn stub_0x36b030() {
    // IDA 0x36b030: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "void boost::shared_ptr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::reset<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEE5resetIS5_EEvPT_")]
// 0x36b344 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEE5resetIS5_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *, void *, int, int)
pub fn stub_0x36b344() {
    // IDA 0x36b344: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::operator()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)const")]
#[doc(alias = "__ZNK5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEclES3_bbfb")]
// 0x36b568 — __ZNK5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbEclES3_bbfb
// type: void __fastcall(_DWORD *, int, int, int, float, int)
pub fn stub_0x36b568() {
    // IDA 0x36b568: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list_av_5<RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::type> boost::bind<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>(boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)")]
#[doc(alias = "__ZN5boost4bindINS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEES4_bbfbEENS_3_bi6bind_tINS7_11unspecifiedET_NS7_9list_av_5IT0_T1_T2_T3_T4_E4typeEEESA_SC_SD_SE_SF_SG_")]
// 0x36b678 — __ZN5boost4bindINS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEES4_bbfbEENS_3_bi6bind_tINS7_11unspecifiedET_NS7_9list_av_5IT0_T1_T2_T3_T4_E4typeEEESA_SC_SD_SE_SF_SG_
// type: void __fastcall(int, int, int, char, struct _Unwind_Exception *lpuexcpt, float, char, int, int, int, int, int, int, int)
pub fn stub_0x36b678() {
    // IDA 0x36b678: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list_av_4<boost::weak_ptr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>::type> boost::bind<void,boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>(void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::weak_ptr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS3_13RequestResultEbbfbEEES4_NS_3argILi1EEESsSA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_")]
// 0x36b76c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS2_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS3_13RequestResultEbbfbEEES4_NS_3argILi1EEESsSA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_T3_ENSD_9list_av_4IT4_T5_T6_T7_E4typeEEESL_SN_SO_SP_SQ_
// type: void __fastcall(_DWORD *, int, int *, const std::string *, int)
pub fn stub_0x36b76c() {
    // IDA 0x36b76c: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::weak_ptr<RBX::ScriptInformationProvider> RBX::weak_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_25ScriptInformationProviderEEEN5boost8weak_ptrIT_EEPS4_")]
// 0x36bac4 — __ZN3RBX9weak_fromINS_25ScriptInformationProviderEEEN5boost8weak_ptrIT_EEPS4_
// type: void __fastcall(int, int)
pub fn stub_0x36bac4() {
    // IDA 0x36bac4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x36bfd8 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *)
pub fn stub_0x36bfd8() {
    // IDA 0x36bfd8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x36c200 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_0x36c200() {
    // IDA 0x36c200: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>> const&)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ERKSG_")]
// 0x36c42c — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ERKSG_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x36c42c() {
    // IDA 0x36c42c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEEvT_")]
// 0x36c5a4 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
pub fn stub_0x36c5a4() {
    // IDA 0x36c5a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE")]
// 0x36c7e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_0x36c7e0() {
    // IDA 0x36c7e0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SQ_ST_")]
// 0x36c7fc — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SQ_ST_
// type: int __fastcall(int *, int, int, int)
pub fn stub_0x36c7fc() {
    // IDA 0x36c7fc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x36c820 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
pub fn stub_0x36c820() {
    // IDA 0x36c820: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x36ca44 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int)
pub fn stub_0x36ca44() {
    // IDA 0x36ca44: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x36cc64 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_25ScriptInformationProviderEEES5_SsNS_8functionIFvNSF_13RequestResultEbbfbEEEENSC_5list4INSC_5valueISG_EENS_3argILi1EEENSO_ISsEENSO_ISK_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *)
pub fn stub_0x36cc64() {
    // IDA 0x36cc64: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::operator()<void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSE_ENS0_5list3IRSJ_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x36cd24 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSE_ENS0_5list3IRSJ_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, struct _Unwind_Exception **, int **)
pub fn stub_0x36cd24() {
    // IDA 0x36cd24: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x36cf0c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX25ScriptInformationProviderEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS7_13RequestResultEbbfbEEEENS3_5list4INS3_5valueIS8_EENS_3argILi1EEENSI_ISsEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, void *, int, int, int, int)
pub fn stub_0x36cf0c() {
    // IDA 0x36cf0c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::list4(boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_")]
// 0x36d0dc — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *, int)
pub fn stub_0x36d0dc() {
    // IDA 0x36d0dc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_")]
// 0x36d2b8 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS5_13RequestResultEbbfbEEEEEEC2ES7_S9_SA_SF_
// type: int __fastcall(int, int, const std::string *, boost::detail::sp_counted_base *)
pub fn stub_0x36d2b8() {
    // IDA 0x36d2b8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEEEC2ES7_S9_SA_")]
// 0x36d4c0 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEENS2_ISsEEEC2ES7_S9_SA_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0x36d4c0() {
    // IDA 0x36d4c0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEEEC2ES7_S9_")]
// 0x36d600 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX25ScriptInformationProviderEEEEENS_3argILi1EEEEC2ES7_S9_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x36d600() {
    // IDA 0x36d600: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::weak_ptr<RBX::ScriptInformationProvider>::weak_ptr<RBX::ScriptInformationProvider>(boost::shared_ptr<RBX::ScriptInformationProvider> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptInformationProvider,RBX::ScriptInformationProvider>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX25ScriptInformationProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
// 0x36d748 — __ZN5boost8weak_ptrIN3RBX25ScriptInformationProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x36d748() {
    // IDA 0x36d748: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
// 0x36da30 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_0x36da30() {
    // IDA 0x36da30: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x36dce8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
pub fn stub_0x36dce8() {
    // IDA 0x36dce8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to_own(boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool> const&)")]
#[doc(alias = "__ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE13assign_to_ownERKS4_")]
// 0x36de2c — __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
pub fn stub_0x36de2c() {
    // IDA 0x36de2c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
// 0x36ed5c — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int __fastcall(int, char **)
pub fn stub_0x36ed5c() {
    // IDA 0x36ed5c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_")]
// 0x36ed98 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
pub fn stub_0x36ed98() {
    // IDA 0x36ed98: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::shared_ptr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEEC2IS5_EEPT_")]
// 0x36ee08 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEEEC2IS5_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
pub fn stub_0x36ee08() {
    // IDA 0x36ee08: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>> const*,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x36eef0 — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_25ScriptInformationProvider16CachedScriptInfoELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x36eef0() {
    // IDA 0x36eef0: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_25ScriptInformationProvider16CachedScriptInfoELb0EEEEEPT_")]
// 0x36f018 — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_25ScriptInformationProvider16CachedScriptInfoELb0EEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x36f018() {
    // IDA 0x36f018: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED1Ev")]
// 0x36f110 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED1Ev
// type: void()
pub fn stub_0x36f110() {
    // IDA 0x36f110: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED0Ev")]
// 0x36f114 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED0Ev
// type: int __fastcall(int)
pub fn stub_0x36f114() {
    // IDA 0x36f114: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE7disposeEv")]
// 0x36f118 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x36f118() {
    // IDA 0x36f118: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE11get_deleterERKSt9type_info")]
// 0x36f12c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0x36f12c() {
    // IDA 0x36f12c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE19get_untyped_deleterEv")]
// 0x36f130 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x36f130() {
    // IDA 0x36f130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_")]
// 0x36f4a4 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x36f4a4() {
    // IDA 0x36f4a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_")]
// 0x36fb88 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
// type: int __fastcall(int, int *, int)
pub fn stub_0x36fb88() {
    // IDA 0x36fb88: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
// 0x36fbe4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x36fbe4() {
    // IDA 0x36fbe4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
// 0x36fc10 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
pub fn stub_0x36fc10() {
    // IDA 0x36fc10: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_")]
// 0x36fc50 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
// type: void __fastcall(int, _DWORD *, std::string *, int)
pub fn stub_0x36fc50() {
    // IDA 0x36fc50: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_")]
// 0x36fe00 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
pub fn stub_0x36fe00() {
    // IDA 0x36fe00: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
// 0x36fe24 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
pub fn stub_0x36fe24() {
    // IDA 0x36fe24: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::~node_constructor()")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEED2Ev")]
// 0x36fe74 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEED2Ev
// type: int __fastcall(int)
pub fn stub_0x36fe74() {
    // IDA 0x36fe74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
// 0x36fe90 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
pub fn stub_0x36fe90() {
    // IDA 0x36fe90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")]
// 0x36ffb8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x36ffb8() {
    // IDA 0x36ffb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")]
// 0x370048 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x370048() {
    // IDA 0x370048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE")]
// 0x370074 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_0x370074() {
    // IDA 0x370074: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE9constructEv")]
// 0x3700cc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE9constructEv
// type: std::string *__fastcall(int)
pub fn stub_0x3700cc() {
    // IDA 0x3700cc: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
// 0x370360 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
pub fn stub_0x370360() {
    // IDA 0x370360: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv")]
// 0x370398 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
pub fn stub_0x370398() {
    // IDA 0x370398: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE")]
// 0x370530 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
// type: int __fastcall(int result, unsigned int)
pub fn stub_0x370530() {
    // IDA 0x370530: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::clear(void)")]
#[doc(alias = "__ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE5clearEv")]
// 0x371220 — __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x371220() {
    // IDA 0x371220: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv")]
// 0x38aa70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x38aa70() {
    // IDA 0x38aa70: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info")]
// 0x38aa80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0x38aa80() {
    // IDA 0x38aa80: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::BaseThreadPool::PoolData>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv")]
// 0x38aa84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14BaseThreadPool8PoolDataEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x38aa84() {
    // IDA 0x38aa84: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::mutex>> const&)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_")]
// 0x38aa88 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE13assign_to_ownERKS5_
// type: int __fastcall(int result, int *)
pub fn stub_0x38aa88() {
    // IDA 0x38aa88: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::ThreadPool::ThreadPoolData::getNextTask(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> &)")]
#[doc(alias = "__ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE")]
// 0x38afc8 — __ZN3RBX10ThreadPool14ThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// type: int __fastcall(int)
pub fn stub_0x38afc8() {
    // IDA 0x38afc8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::pop_if_present(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&)")]
#[doc(alias = "__ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_")]
// 0x38afd4 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE14pop_if_presentERS8_
// type: int __fastcall(int, int)
pub fn stub_0x38afd4() {
    // IDA 0x38afd4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv")]
// 0x38b0b4 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9pop_frontEv
// type: int __fastcall(int)
pub fn stub_0x38b0b4() {
    // IDA 0x38b0b4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev")]
// 0x38b0ec — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(_DWORD *, int, int, int)
pub fn stub_0x38b0ec() {
    // IDA 0x38b0ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev")]
// 0x38b1d4 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EED2Ev
// type: int __fastcall(int)
pub fn stub_0x38b1d4() {
    // IDA 0x38b1d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>,std::_Deque_iterator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>&,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>*>)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_")]
// 0x38b200 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE19_M_destroy_data_auxESt15_Deque_iteratorIS7_RS7_PS7_ESD_
// type: void __fastcall(int, int *, int *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x38b200() {
    // IDA 0x38b200: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm")]
// 0x38b338 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_initialize_mapEm
// type: void __fastcall(int *, unsigned int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
pub fn stub_0x38b338() {
    // IDA 0x38b338: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_create_nodes(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**,boost::function<void ()(boost::shared_ptr<RBX::mutex>)>**)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_")]
// 0x38b490 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_create_nodesEPPS7_SB_
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
pub fn stub_0x38b490() {
    // IDA 0x38b490: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::deque(std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_")]
// 0x38b584 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EEC2ERKS9_
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x38b584() {
    // IDA 0x38b584: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float> &,boost::_bi::list3<float &,float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x39dc18 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_fffEENS0_5list3IRfSH_SH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
pub fn stub_0x39dc18() {
    // IDA 0x39dc18: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev")]
// 0x39df38 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x39df38() {
    // IDA 0x39df38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev")]
// 0x39df64 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x39df64() {
    // IDA 0x39df64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")]
// 0x39e038 — __ZN3rbx7signals6signalIFvffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x39e038() {
    // IDA 0x39e038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev")]
// 0x39e0ac — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x39e0ac() {
    // IDA 0x39e0ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev")]
// 0x39e0d8 — __ZN3rbx7signals6signalIFvffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x39e0d8() {
    // IDA 0x39e0d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff")]
// 0x39e1ac — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
// type: int __fastcall(int, int, int)
pub fn stub_0x39e1ac() {
    // IDA 0x39e1ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff")]
// 0x39e1d4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEff
// type: int __fastcall(int, int, int)
pub fn stub_0x39e1d4() {
    // IDA 0x39e1d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list2<float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float> &,boost::_bi::list2<float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x39e1fc — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_ffEENS0_5list2IRfSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, int **)
pub fn stub_0x39e1fc() {
    // IDA 0x39e1fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev")]
// 0x39e228 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x39e228() {
    // IDA 0x39e228: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev")]
// 0x39e254 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19AnimationTrackStateEffEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x39e254() {
    // IDA 0x39e254: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_")]
// 0x39e328 — __ZN3rbx7signals6signalIFvffffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x39e328() {
    // IDA 0x39e328: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_")]
// 0x39e5a8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSEPS6_
// type: int *__fastcall(int *, int)
pub fn stub_0x39e5a8() {
    // IDA 0x39e5a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED1Ev")]
// 0x39e5cc — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x39e5cc() {
    // IDA 0x39e5cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED0Ev")]
// 0x39e5f8 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS6_5list5INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEENSH_ILi4EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x39e5f8() {
    // IDA 0x39e5f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff")]
// 0x39e7e8 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
// type: int __fastcall(int, int, int, int, float)
pub fn stub_0x39e7e8() {
    // IDA 0x39e7e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff")]
// 0x39e824 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_E4callEffff
// type: int __fastcall(int, int, int, int, float)
pub fn stub_0x39e824() {
    // IDA 0x39e824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x39e860 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX19AnimationTrackStateEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEENS7_ILi4EEEEclINS_4_mfi3mf4IvS4_ffffEENS0_5list4IRfSI_SI_SI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
pub fn stub_0x39e860() {
    // IDA 0x39e860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED1Ev")]
// 0x39eb94 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x39eb94() {
    // IDA 0x39eb94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED0Ev")]
// 0x39ebc0 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX19AnimationTrackStateEffffEENS7_5list5INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEELi4ES3_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x39ebc0() {
    // IDA 0x39ebc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,float,float,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvfffE5clearEv")]
// 0x3a017c — __ZN5boost9function3IvfffE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x3a017c() {
    // IDA 0x3a017c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::function<void ()(float,float,float)>>(boost::function<void ()(float,float,float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
// 0x3a08d0 — __ZN3rbx7signals6signalIFvfffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x3a08d0() {
    // IDA 0x3a08d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float)>*>(boost::function<void ()(float,float,float)> const&,rbx::signals::signal<void ()(float,float,float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_")]
// 0x3a09c4 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_EC2IPS4_EERKS8_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x3a09c4() {
    // IDA 0x3a09c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
// 0x3a0ac0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3a0ac0() {
    // IDA 0x3a0ac0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
// 0x3a0bd0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3a0bd0() {
    // IDA 0x3a0bd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff")]
// 0x3a0d00 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
// type: int __fastcall(int)
pub fn stub_0x3a0d00() {
    // IDA 0x3a0d00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff")]
// 0x3a0d08 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_E4callEfff
// type: int __fastcall(int)
pub fn stub_0x3a0d08() {
    // IDA 0x3a0d08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,float,float,float>::operator()(float,float,float)const")]
#[doc(alias = "__ZNK5boost9function3IvfffEclEfff")]
// 0x3a0d10 — __ZNK5boost9function3IvfffEclEfff
// type: void __fastcall(_DWORD *, int, int, int)
pub fn stub_0x3a0d10() {
    // IDA 0x3a0d10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev")]
// 0x3a0dec — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3a0dec() {
    // IDA 0x3a0dec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev")]
// 0x3a0efc — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3a0efc() {
    // IDA 0x3a0efc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function3<void,float,float,float>::assign_to_own(boost::function3<void,float,float,float> const&)")]
#[doc(alias = "__ZN5boost9function3IvfffE13assign_to_ownERKS1_")]
// 0x3a102c — __ZN5boost9function3IvfffE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
pub fn stub_0x3a102c() {
    // IDA 0x3a102c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function4<void,float,float,float,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function4IvffffE5clearEv")]
// 0x3a1f18 — __ZN5boost9function4IvffffE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x3a1f18() {
    // IDA 0x3a1f18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::function<void ()(float,float,float,float)>>(boost::function<void ()(float,float,float,float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
// 0x3a2684 — __ZN3rbx7signals6signalIFvffffEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x3a2684() {
    // IDA 0x3a2684: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float,float)>*>(boost::function<void ()(float,float,float,float)> const&,rbx::signals::signal<void ()(float,float,float,float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_")]
// 0x3a2778 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_EC2IPS4_EERKS8_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x3a2778() {
    // IDA 0x3a2778: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
// 0x3a2874 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3a2874() {
    // IDA 0x3a2874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
// 0x3a2984 — __ZN3rbx7signals6signalIFvffffEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3a2984() {
    // IDA 0x3a2984: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff")]
// 0x3a2ab4 — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
pub fn stub_0x3a2ab4() {
    // IDA 0x3a2ab4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff")]
// 0x3a2abc — __ZThn4_N3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_E4callEffff
// type: int __fastcall(int)
pub fn stub_0x3a2abc() {
    // IDA 0x3a2abc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")]
#[doc(alias = "__ZNK5boost9function4IvffffEclEffff")]
// 0x3a2ac4 — __ZNK5boost9function4IvffffEclEffff
// type: void __fastcall(_DWORD *, int, int, int, float)
pub fn stub_0x3a2ac4() {
    // IDA 0x3a2ac4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev")]
// 0x3a2bac — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x3a2bac() {
    // IDA 0x3a2bac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev")]
// 0x3a2cbc — __ZN3rbx8callableINS_7signals6signalIFvffffEE4slotEN5boost8functionIS3_EELi4ES3_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3a2cbc() {
    // IDA 0x3a2cbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")]
#[doc(alias = "__ZN5boost9function4IvffffE13assign_to_ownERKS1_")]
// 0x3a2dec — __ZN5boost9function4IvffffE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
pub fn stub_0x3a2dec() {
    // IDA 0x3a2dec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Animator::reloadAnimation(boost::shared_ptr<RBX::AnimationTrackState>)")]
#[doc(alias = "__ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE")]
// 0x3a4364 — __ZN3RBX8Animator15reloadAnimationEN5boost10shared_ptrINS_19AnimationTrackStateEEE
// type: int *__fastcall(int, int *, int, int)
pub fn stub_0x3a4364() {
    // IDA 0x3a4364: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Animator::onTrackStepped(boost::shared_ptr<RBX::AnimationTrackState>,double,RBX::KeyframeSequence::Priority,std::vector<RBX::PoseAccumulator,std::allocator<RBX::PoseAccumulator>> *)")]
#[doc(alias = "__ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE")]
// 0x3a4598 — __ZN3RBX8Animator14onTrackSteppedEN5boost10shared_ptrINS_19AnimationTrackStateEEEdNS_16KeyframeSequence8PriorityEPSt6vectorINS_15PoseAccumulatorESaIS8_EE
// type: void __fastcall(int, int *, double, int, _DWORD *)
pub fn stub_0x3a4598() {
    // IDA 0x3a4598: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::shared_ptr<RBX::Animator> RBX::shared_from<RBX::Animator>(RBX::Animator*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_")]
// 0x3a4fe8 — __ZN3RBX11shared_fromINS_8AnimatorEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
pub fn stub_0x3a4fe8() {
    // IDA 0x3a4fe8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::AnimatableRootJoint>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev")]
// 0x3a5158 — __ZN5boost10scoped_ptrIN3RBX19AnimatableRootJointEED1Ev
// type: _DWORD **__fastcall(_DWORD **)
pub fn stub_0x3a5158() {
    // IDA 0x3a5158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
