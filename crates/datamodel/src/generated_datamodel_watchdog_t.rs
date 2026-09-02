// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x317378..0x322b38 | datamodel distinct 33819->33939 global uncovered 52527->52407, lowest gap EA-sorted asc next 120 after watchdog_s (0x30d740..0x316f2c)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ` and ' stripped from alias where needed
// Shard: watchdog_t EA-sorted ascending next uncovered gap after watchdog_s (distinct check via export.json sorted EA, no overlap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x317378 — __ZN3RBXL6doPostESsSsbbN5boost8functionIFvPSsPSt9exceptionEEE
#[doc(alias = "RBX::doPost(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
pub fn stub_0x317378() -> ! {
    todo!("0x317378 RBX::doPost(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")
}

// 0x317570 — __ZN3RBX4Http4postEN5boost10shared_ptrISiEEbNS1_8functionIFvPSsPSt9exceptionEEEb
#[doc(alias = "RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")]
// was: RBX::Http::post(boost::shared_ptr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)
pub fn stub_0x317570() -> ! {
    todo!("0x317570 RBX::Http::post(rbx_core::SharedPtr<std::istream>,bool,boost::function<void ()(std::string *,std::exception *)>,bool)")
}

// 0x317a08 — __ZN3RBXL12doPostStreamESsN5boost10shared_ptrISiEEbbNS0_8functionIFvPSsPSt9exceptionEEE
#[doc(alias = "RBX::doPostStream(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
// was: RBX::doPostStream(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)
pub fn stub_0x317a08() -> ! {
    todo!("0x317a08 RBX::doPostStream(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")
}

// 0x317de0 — __ZN3RBX4Http3getERSsb
// type: _DWORD __fastcall(RBX::Http *__hidden this, std::string *, bool)
#[doc(alias = "RBX::Http::get(std::string &,bool)")]
pub fn stub_0x317de0() -> ! {
    todo!("0x317de0 RBX::Http::get(std::string &,bool)")
}

// 0x3180dc — __ZN3RBX4Http12isRobloxSiteEPKc
// type: _DWORD __fastcall(RBX::Http *__hidden this, const char *)
#[doc(alias = "RBX::Http::isRobloxSite(char const*)")]
pub fn stub_0x3180dc() -> ! {
    todo!("0x3180dc RBX::Http::isRobloxSite(char const*)")
}

// 0x3180ec — __ZN3RBXL14initTrustCheckEv
// type: void __fastcall(RBX *this)
#[doc(alias = "RBX::initTrustCheck(void)")]
pub fn stub_0x3180ec() -> ! {
    todo!("0x3180ec RBX::initTrustCheck(void)")
}

// 0x3180f0 — __ZN3RBX13WindowAverageIddED1Ev
#[doc(alias = "RBX::WindowAverage<double,double>::~WindowAverage()")]
pub fn stub_0x3180f0() -> ! {
    todo!("0x3180f0 RBX::WindowAverage<double,double>::~WindowAverage()")
}

// 0x318100 — __ZN3RBX4Http10MutexGuardD1Ev
// type: void __fastcall(RBX::Http::MutexGuard *__hidden this)
#[doc(alias = "RBX::Http::MutexGuard::~MutexGuard()")]
pub fn stub_0x318100() -> ! {
    todo!("0x318100 RBX::Http::MutexGuard::~MutexGuard()")
}

// 0x318104 — __ZN5boost10scoped_ptrIN3RBX10ThreadPoolEED1Ev
#[doc(alias = "boost::scoped_ptr<RBX::ThreadPool>::~scoped_ptr()")]
pub fn stub_0x318104() -> ! {
    todo!("0x318104 boost::scoped_ptr<RBX::ThreadPool>::~scoped_ptr()")
}

// 0x318118 — __ZN5boost4bindIvSsbNS_8functionIFvPSsPSt9exceptionEEESsbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
// type: int __fastcall(int, int, std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_3<std::string,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,bool,boost::function<void ()(std::string *,std::exception *)>)")]
pub fn stub_0x318118() -> ! {
    todo!("0x318118 boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_3<std::string,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,bool,boost::function<void ()(std::string *,std::exception *)>)")
}

// 0x3183e0 — __ZN5boost4bindIvSsSsbbNS_8functionIFvPSsPSt9exceptionEEESsSsbbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_T3_T4_ENS7_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESG_SI_SJ_SK_SL_SM_
// type: int __fastcall(int, int, std::string *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
pub fn stub_0x3183e0() -> ! {
    todo!("0x3183e0 boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")
}

// 0x31888c — __ZN5boost4bindIvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEESsS2_bbS8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
// was: boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)
pub fn stub_0x31888c() -> ! {
    todo!("0x31888c boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")
}

// 0x318d7c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEENSJ_ISA_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEENSJ_ISA_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x318d7c() -> ! {
    todo!("0x318d7c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEENSJ_ISA_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x31919c — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x31919c() -> ! {
    todo!("0x31919c __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x3195c0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x3195c0() -> ! {
    todo!("0x3195c0 void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")
}

// 0x3199f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x3199f4() -> ! {
    todo!("0x3199f4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x319a10 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEvNS5_IN3RBX5mutexEEEE6invokeERNS1_15function_bufferESP_
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,boost::shared_ptr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::mutex>)
pub fn stub_0x319a10() -> ! {
    todo!("0x319a10 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")
}

// 0x319a28 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x319a28() -> ! {
    todo!("0x319a28 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x319e4c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x319e4c() -> ! {
    todo!("0x319e4c bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x31a26c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31a26c() -> ! {
    todo!("0x31a26c void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x31a3d8 — __ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsS5_bbSD_ENS0_5list1IRNS4_IN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<boost::shared_ptr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex> &> &,int)
pub fn stub_0x31a3d8() -> ! {
    todo!("0x31a3d8 void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")
}

// 0x31a594 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x31a594() -> ! {
    todo!("0x31a594 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x31a7f0 — __ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// was: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_0x31a7f0() -> ! {
    todo!("0x31a7f0 boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")
}

// 0x31a99c — __ZN5boost3_bi8storage5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// was: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_0x31a99c() -> ! {
    todo!("0x31a99c boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")
}

// 0x31ab68 — __ZN5boost3_bi8storage4INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_EC2ES3_S6_S7_S7_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
// was: boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)
pub fn stub_0x31ab68() -> ! {
    todo!("0x31ab68 boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)")
}

// 0x31ace0 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEEEC2ES3_S6_S7_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>)")]
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>)
pub fn stub_0x31ace0() -> ! {
    todo!("0x31ace0 boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>)")
}

// 0x31ae54 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_10shared_ptrISiEEEEEC2ES3_S6_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>)")]
// was: boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>)
pub fn stub_0x31ae54() -> ! {
    todo!("0x31ae54 boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>)")
}

// 0x31afb8 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x31afb8() -> ! {
    todo!("0x31afb8 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS0_IFvPSsPSt9exceptionEEEENS8_5list5INS8_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x31b33c — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x31b33c() -> ! {
    todo!("0x31b33c __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x31b6c4 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x31b6c4() -> ! {
    todo!("0x31b6c4 void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")
}

// 0x31ba5c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x31ba5c() -> ! {
    todo!("0x31ba5c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x31ba78 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,boost::shared_ptr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::mutex>)
pub fn stub_0x31ba78() -> ! {
    todo!("0x31ba78 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")
}

// 0x31ba80 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x31ba80() -> ! {
    todo!("0x31ba80 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x31be08 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x31be08() -> ! {
    todo!("0x31be08 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x31c18c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31c18c() -> ! {
    todo!("0x31c18c void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x31c2e8 — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<boost::shared_ptr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex> &> &,int)
pub fn stub_0x31c2e8() -> ! {
    todo!("0x31c2e8 void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")
}

// 0x31c4e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x31c4e4() -> ! {
    todo!("0x31c4e4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x31c72c — __ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
pub fn stub_0x31c72c() -> ! {
    todo!("0x31c72c boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")
}

// 0x31c918 — __ZN5boost3_bi8storage5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
pub fn stub_0x31c918() -> ! {
    todo!("0x31c918 boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")
}

// 0x31cb24 — __ZN5boost3_bi8storage4INS0_5valueISsEES3_NS2_IbEES4_EC2ES3_S3_S4_S4_
// type: int __fastcall(int, const std::string *, const std::string *, unsigned __int8, char)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
pub fn stub_0x31cb24() -> ! {
    todo!("0x31cb24 boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>)")
}

// 0x31ccd4 — __ZN5boost3_bi8storage3INS0_5valueISsEES3_NS2_IbEEEC2ES3_S3_S4_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
pub fn stub_0x31ccd4() -> ! {
    todo!("0x31ccd4 boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")
}

// 0x31ce80 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9exceptionEEEENS8_5list3INS8_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9exceptionEEEENS8_5list3INS8_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x31ce80() -> ! {
    todo!("0x31ce80 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvSsbNS0_IFvPSsPSt9exceptionEEEENS8_5list3INS8_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x31d0a8 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x31d0a8() -> ! {
    todo!("0x31d0a8 __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0x31d2d0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_0x31d2d0() -> ! {
    todo!("0x31d2d0 void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")
}

// 0x31d50c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x31d50c() -> ! {
    todo!("0x31d50c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x31d528 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEvNS_10shared_ptrIN3RBX5mutexEEEE6invokeERNS1_15function_bufferESN_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,boost::shared_ptr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::mutex>)
pub fn stub_0x31d528() -> ! {
    todo!("0x31d528 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")
}

// 0x31d530 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x31d530() -> ! {
    todo!("0x31d530 bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x31d75c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, std::string *)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x31d75c() -> ! {
    todo!("0x31d75c bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x31d984 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x31d984() -> ! {
    todo!("0x31d984 void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x31da84 — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<boost::shared_ptr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex> &> &,int)
pub fn stub_0x31da84() -> ! {
    todo!("0x31da84 void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")
}

// 0x31dbf0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x31dbf0() -> ! {
    todo!("0x31dbf0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x31dd9c — __ZN5boost9function2IvPSsPSt9exceptionE13assign_to_ownERKS4_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function2<void,std::string *,std::exception *>::assign_to_own(boost::function2<void,std::string *,std::exception *> const&)")]
pub fn stub_0x31dd9c() -> ! {
    todo!("0x31dd9c boost::function2<void,std::string *,std::exception *>::assign_to_own(boost::function2<void,std::string *,std::exception *> const&)")
}

// 0x31ddcc — __ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
// type: int(void)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
pub fn stub_0x31ddcc() -> ! {
    todo!("0x31ddcc boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")
}

// 0x31df30 — __ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
pub fn stub_0x31df30() -> ! {
    todo!("0x31df30 boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")
}

// 0x31e084 — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
pub fn stub_0x31e084() -> ! {
    todo!("0x31e084 boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<bool>)")
}

// 0x31e1a8 — __ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_
#[doc(alias = "boost::function2<void,std::string *,std::exception *>::operator()(std::string *,std::exception *)const")]
pub fn stub_0x31e1a8() -> ! {
    todo!("0x31e1a8 boost::function2<void,std::string *,std::exception *>::operator()(std::string *,std::exception *)const")
}

// 0x31e270 — __ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
#[doc(alias = "rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// was: boost::shared_ptr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_0x31e270() -> ! {
    todo!("0x31e270 rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")
}

// 0x31e344 — __ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
pub fn stub_0x31e344() -> ! {
    todo!("0x31e344 boost::detail::shared_count::shared_count<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")
}

// 0x31e43c — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
pub fn stub_0x31e43c() -> ! {
    todo!("0x31e43c boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")
}

// 0x31e440 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
pub fn stub_0x31e440() -> ! {
    todo!("0x31e440 boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")
}

// 0x31e444 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)")]
pub fn stub_0x31e444() -> ! {
    todo!("0x31e444 boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::dispose(void)")
}

// 0x31e454 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")]
pub fn stub_0x31e454() -> ! {
    todo!("0x31e454 boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_deleter(std::type_info const&)")
}

// 0x31e458 — __ZN5boost6detail17sp_counted_impl_pISt19basic_istringstreamIcSt11char_traitsIcESaIcEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")]
pub fn stub_0x31e458() -> ! {
    todo!("0x31e458 boost::detail::sp_counted_impl_p<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>::get_untyped_deleter(void)")
}

// 0x31e45c — __ZN3RBX4Http10MutexGuardD2Ev
// type: void __fastcall(RBX::Http::MutexGuard *__hidden this)
#[doc(alias = "RBX::Http::MutexGuard::~MutexGuard()")]
pub fn stub_0x31e45c() -> ! {
    todo!("0x31e45c RBX::Http::MutexGuard::~MutexGuard()")
}

// 0x31e558 — __ZN3RBX4Http10MutexGuardC2Ev
// type: _DWORD __fastcall(RBX::Http::MutexGuard *__hidden this)
#[doc(alias = "RBX::Http::MutexGuard::MutexGuard(void)")]
pub fn stub_0x31e558() -> ! {
    todo!("0x31e558 RBX::Http::MutexGuard::MutexGuard(void)")
}

// 0x31e63c — __ZN5boost15circular_bufferIdSaIdEE7destroyEv
// type: int(void)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy(void)")]
pub fn stub_0x31e63c() -> ! {
    todo!("0x31e63c boost::circular_buffer<double,std::allocator<double>>::destroy(void)")
}

// 0x31e658 — __GLOBAL__I_a_113
#[doc(alias = "global constructor keyed to_a_113")]
pub fn stub_0x31e658() -> ! {
    todo!("0x31e658 global constructor keyed to_a_113")
}

// 0x31e8b0 — __ZNK3RBX7Extents13clampInsideOfERKS0_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const RBX::Extents *)
#[doc(alias = "RBX::Extents::clampInsideOf(RBX::Extents const&)const")]
pub fn stub_0x31e8b0() -> ! {
    todo!("0x31e8b0 RBX::Extents::clampInsideOf(RBX::Extents const&)const")
}

// 0x31e9f4 — __ZN3RBX7Extents11closestFaceERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Extents::closestFace(G3D::Vector3 const&)")]
pub fn stub_0x31e9f4() -> ! {
    todo!("0x31e9f4 RBX::Extents::closestFace(G3D::Vector3 const&)")
}

// 0x31eae4 — __ZNK3RBX7Extents14getCornerIndexEi
// type: _DWORD __fastcall(RBX::Extents *__hidden this, int)
#[doc(alias = "RBX::Extents::getCornerIndex(int)const")]
pub fn stub_0x31eae4() -> ! {
    todo!("0x31eae4 RBX::Extents::getCornerIndex(int)const")
}

// 0x31eba8 — __ZNK3RBX7Extents9getCornerEi
// type: _DWORD __fastcall(RBX::Extents *__hidden this, int)
#[doc(alias = "RBX::Extents::getCorner(int)const")]
pub fn stub_0x31eba8() -> ! {
    todo!("0x31eba8 RBX::Extents::getCorner(int)const")
}

// 0x31ebfc — __ZNK3RBX7Extents14getFaceCornersENS_8NormalIdERN3G3D7Vector3ES4_S4_S4_
#[doc(alias = "RBX::Extents::getFaceCorners(RBX::NormalId,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")]
pub fn stub_0x31ebfc() -> ! {
    todo!("0x31ebfc RBX::Extents::getFaceCorners(RBX::NormalId,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)const")
}

// 0x31ee8c — __ZNK3RBX7Extents7expressERKN3G3D15CoordinateFrameES4_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Extents::express(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&)const")]
pub fn stub_0x31ee8c() -> ! {
    todo!("0x31ee8c RBX::Extents::express(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&)const")
}

// 0x31f464 — __ZNK3RBX7Extents10faceCenterENS_8NormalIdE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Extents::faceCenter(RBX::NormalId)const")]
pub fn stub_0x31f464() -> ! {
    todo!("0x31f464 RBX::Extents::faceCenter(RBX::NormalId)const")
}

// 0x31f4d0 — __ZNK3RBX7Extents31computeClosestSqDistanceToPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Extents::computeClosestSqDistanceToPoint(G3D::Vector3 const&)const")]
pub fn stub_0x31f4d0() -> ! {
    todo!("0x31f4d0 RBX::Extents::computeClosestSqDistanceToPoint(G3D::Vector3 const&)const")
}

// 0x31f5b4 — __ZNK3RBX7Extents19separatedByMoreThanERKS0_f
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const RBX::Extents *, float)
#[doc(alias = "RBX::Extents::separatedByMoreThan(RBX::Extents const&,float)const")]
pub fn stub_0x31f5b4() -> ! {
    todo!("0x31f5b4 RBX::Extents::separatedByMoreThan(RBX::Extents const&,float)const")
}

// 0x31f68c — __ZN3RBX7Extents2vvERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Extents *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Extents::vv(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub fn stub_0x31f68c() -> ! {
    todo!("0x31f68c RBX::Extents::vv(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0x31f738 — __GLOBAL__I_a_114
#[doc(alias = "global constructor keyed to_a_114")]
pub fn stub_0x31f738() -> ! {
    todo!("0x31f738 global constructor keyed to_a_114")
}

// 0x31f90c — __ZNK3RBX4FaceixEi
#[doc(alias = "RBX::Face::operator[](int)const")]
pub fn stub_0x31f90c() -> ! {
    todo!("0x31f90c RBX::Face::operator[](int)const")
}

// 0x31f918 — __ZN3RBX4FaceixEi
#[doc(alias = "RBX::Face::operator[](int)")]
pub fn stub_0x31f918() -> ! {
    todo!("0x31f918 RBX::Face::operator[](int)")
}

// 0x31f924 — __ZN3RBX4Face10snapToGridEf
// type: _DWORD __fastcall(RBX::Face *__hidden this, float)
#[doc(alias = "RBX::Face::snapToGrid(float)")]
pub fn stub_0x31f924() -> ! {
    todo!("0x31f924 RBX::Face::snapToGrid(float)")
}

// 0x31f964 — __ZN3RBX4Face19overlapWithinPlanesERKS0_S2_f
// type: _DWORD __fastcall(Vector3 *this, const RBX::Face *, const RBX::Face *, float)
#[doc(alias = "RBX::Face::overlapWithinPlanes(RBX::Face const&,RBX::Face const&,float)")]
pub fn stub_0x31f964() -> ! {
    todo!("0x31f964 RBX::Face::overlapWithinPlanes(RBX::Face const&,RBX::Face const&,float)")
}

// 0x31fa44 — __ZNK3RBX4Face18projectOverlapOnMeERKS0_
// type: _DWORD __fastcall(RBX::Face *__hidden this, const RBX::Face *)
#[doc(alias = "RBX::Face::projectOverlapOnMe(RBX::Face const&)const")]
pub fn stub_0x31fa44() -> ! {
    todo!("0x31fa44 RBX::Face::projectOverlapOnMe(RBX::Face const&)const")
}

// 0x31fcd4 — __ZNK3RBX4Face24fuzzyContainsInExtrusionERKN3G3D7Vector3Ef
// type: _DWORD __fastcall(RBX::Face *__hidden this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Face::fuzzyContainsInExtrusion(G3D::Vector3 const&,float)const")]
pub fn stub_0x31fcd4() -> ! {
    todo!("0x31fcd4 RBX::Face::fuzzyContainsInExtrusion(G3D::Vector3 const&,float)const")
}

// 0x31fdc4 — __ZNK3RBX4Face6minMaxERKN3G3D7Vector3ES4_RfS5_
// type: int __fastcall(int this, const Vector3 *, const Vector3 *, float *, float *)
#[doc(alias = "RBX::Face::minMax(G3D::Vector3 const&,G3D::Vector3 const&,float &,float &)const")]
pub fn stub_0x31fdc4() -> ! {
    todo!("0x31fdc4 RBX::Face::minMax(G3D::Vector3 const&,G3D::Vector3 const&,float &,float &)const")
}

// 0x31fe6c — __ZN3RBX4Face10hasOverlapERKS0_S2_f
// type: _DWORD __fastcall(RBX::Face *__hidden this, const RBX::Face *, const RBX::Face *, float)
#[doc(alias = "RBX::Face::hasOverlap(RBX::Face const&,RBX::Face const&,float)")]
pub fn stub_0x31fe6c() -> ! {
    todo!("0x31fe6c RBX::Face::hasOverlap(RBX::Face const&,RBX::Face const&,float)")
}

// 0x31fefc — __ZN3RBX4Face14cornersAlignedERKS0_S2_f
// type: _DWORD __fastcall(RBX::Face *__hidden this, const RBX::Face *, const RBX::Face *, float)
#[doc(alias = "RBX::Face::cornersAligned(RBX::Face const&,RBX::Face const&,float)")]
pub fn stub_0x31fefc() -> ! {
    todo!("0x31fefc RBX::Face::cornersAligned(RBX::Face const&,RBX::Face const&,float)")
}

// 0x31ffe4 — __ZN3RBX4Face15fromExtentsSideERKNS_7ExtentsENS_8NormalIdE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Face::fromExtentsSide(RBX::Extents const&,RBX::NormalId)")]
pub fn stub_0x31ffe4() -> ! {
    todo!("0x31ffe4 RBX::Face::fromExtentsSide(RBX::Extents const&,RBX::NormalId)")
}

// 0x32010c — __ZNK3RBX4Face13toObjectSpaceERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Face::toObjectSpace(G3D::CoordinateFrame const&)const")]
pub fn stub_0x32010c() -> ! {
    todo!("0x32010c RBX::Face::toObjectSpace(G3D::CoordinateFrame const&)const")
}

// 0x3201f4 — __ZNK3RBX4Face7getAxisEi
// type: _DWORD __fastcall(RBX::Face *__hidden this, int)
#[doc(alias = "RBX::Face::getAxis(int)const")]
pub fn stub_0x3201f4() -> ! {
    todo!("0x3201f4 RBX::Face::getAxis(int)const")
}

// 0x3202dc — __GLOBAL__I_a_115
// type: int()
#[doc(alias = "global constructor keyed to_a_115")]
pub fn stub_0x3202dc() -> ! {
    todo!("0x3202dc global constructor keyed to_a_115")
}

// 0x320314 — __ZN3RBX5FacesC1Ei
// type: _DWORD __fastcall(RBX::Faces *__hidden this, int)
#[doc(alias = "RBX::Faces::Faces(int)")]
pub fn stub_0x320314() -> ! {
    todo!("0x320314 RBX::Faces::Faces(int)")
}

// 0x320318 — __ZN3RBX5Faces11setNormalIdENS_8NormalIdEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Faces::setNormalId(RBX::NormalId,bool)")]
pub fn stub_0x320318() -> ! {
    todo!("0x320318 RBX::Faces::setNormalId(RBX::NormalId,bool)")
}

// 0x320338 — __ZNK3RBX5Faces11getNormalIdENS_8NormalIdE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Faces::getNormalId(RBX::NormalId)const")]
pub fn stub_0x320338() -> ! {
    todo!("0x320338 RBX::Faces::getNormalId(RBX::NormalId)const")
}

// 0x32034c — __ZN3RBX15StringConverterINS_5FacesEE15convertToStringERKS1_
// type: void __fastcall(std::string *, int *)
#[doc(alias = "RBX::StringConverter<RBX::Faces>::convertToString(RBX::Faces const&)")]
pub fn stub_0x32034c() -> ! {
    todo!("0x32034c RBX::StringConverter<RBX::Faces>::convertToString(RBX::Faces const&)")
}

// 0x32059c — __ZN3RBX15StringConverterINS_5FacesEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::Faces>::convertToValue(std::string const&,RBX::Faces&)")]
pub fn stub_0x32059c() -> ! {
    todo!("0x32059c RBX::StringConverter<RBX::Faces>::convertToValue(std::string const&,RBX::Faces&)")
}

// 0x3207f8 — __GLOBAL__I_a_116
#[doc(alias = "global constructor keyed to_a_116")]
pub fn stub_0x3207f8() -> ! {
    todo!("0x3207f8 global constructor keyed to_a_116")
}

// 0x3208c0 — _gpc_free_polygon
// type: int __fastcall(_DWORD)
#[doc(alias = "_gpc_free_polygon")]
pub fn stub_0x3208c0() -> ! {
    todo!("0x3208c0 _gpc_free_polygon")
}

// 0x320910 — _gpc_polygon_clip
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_gpc_polygon_clip")]
pub fn stub_0x320910() -> ! {
    todo!("0x320910 _gpc_polygon_clip")
}

// 0x321838 — _minimax_test
#[doc(alias = "_minimax_test")]
pub fn stub_0x321838() -> ! {
    todo!("0x321838 _minimax_test")
}

// 0x321a18 — _build_lmt
#[doc(alias = "_build_lmt")]
pub fn stub_0x321a18() -> ! {
    todo!("0x321a18 _build_lmt")
}

// 0x321fd4 — _build_sbt
#[doc(alias = "_build_sbt")]
pub fn stub_0x321fd4() -> ! {
    todo!("0x321fd4 _build_sbt")
}

// 0x322004 — _free_sbtree
#[doc(alias = "_free_sbtree")]
pub fn stub_0x322004() -> ! {
    todo!("0x322004 _free_sbtree")
}

// 0x322030 — _add_edge_to_aet
#[doc(alias = "_add_edge_to_aet")]
pub fn stub_0x322030() -> ! {
    todo!("0x322030 _add_edge_to_aet")
}

// 0x322088 — _add_local_min
#[doc(alias = "_add_local_min")]
pub fn stub_0x322088() -> ! {
    todo!("0x322088 _add_local_min")
}

// 0x322140 — _add_right
#[doc(alias = "_add_right")]
pub fn stub_0x322140() -> ! {
    todo!("0x322140 _add_right")
}

// 0x3221b0 — _add_left
// type: _QWORD *__fastcall(int)
#[doc(alias = "_add_left")]
pub fn stub_0x3221b0() -> ! {
    todo!("0x3221b0 _add_left")
}

// 0x32221c — _merge_right
#[doc(alias = "_merge_right")]
pub fn stub_0x32221c() -> ! {
    todo!("0x32221c _merge_right")
}

// 0x322268 — _merge_left
#[doc(alias = "_merge_left")]
pub fn stub_0x322268() -> ! {
    todo!("0x322268 _merge_left")
}

// 0x3222b8 — _build_intersection_table
#[doc(alias = "_build_intersection_table")]
pub fn stub_0x3222b8() -> ! {
    todo!("0x3222b8 _build_intersection_table")
}

// 0x3224d8 — _insert_bound
// type: int *__fastcall(int *result, int)
#[doc(alias = "_insert_bound")]
pub fn stub_0x3224d8() -> ! {
    todo!("0x3224d8 _insert_bound")
}

// 0x322518 — _bound_list
#[doc(alias = "_bound_list")]
pub fn stub_0x322518() -> ! {
    todo!("0x322518 _bound_list")
}

// 0x3225b8 — _create_contour_bboxes
#[doc(alias = "_create_contour_bboxes")]
pub fn stub_0x3225b8() -> ! {
    todo!("0x3225b8 _create_contour_bboxes")
}

// 0x3226f8 — __ZL14initLocalScopev
// type: _DWORD __fastcall()
#[doc(alias = "initLocalScope(void)")]
pub fn stub_0x3226f8() -> ! {
    todo!("0x3226f8 initLocalScope(void)")
}

// 0x32281c — __ZN3RBX4GuidC1Ev
// type: _DWORD __fastcall(RBX::Guid *__hidden this)
#[doc(alias = "RBX::Guid::Guid(void)")]
pub fn stub_0x32281c() -> ! {
    todo!("0x32281c RBX::Guid::Guid(void)")
}

// 0x322850 — __ZN3RBX4Guid20generateStandardGUIDERSs
// type: _DWORD __fastcall(RBX::Guid *__hidden this, std::string *)
#[doc(alias = "RBX::Guid::generateStandardGUID(std::string &)")]
pub fn stub_0x322850() -> ! {
    todo!("0x322850 RBX::Guid::generateStandardGUID(std::string &)")
}

// 0x32298c — __ZN3RBX4Guid15generateRBXGUIDERSs
// type: _DWORD __fastcall(struct _Unwind_Exception *lpuexcpt, std::string *)
#[doc(alias = "RBX::Guid::generateRBXGUID(std::string &)")]
pub fn stub_0x32298c() -> ! {
    todo!("0x32298c RBX::Guid::generateRBXGUID(std::string &)")
}

// 0x322b04 — __ZN3RBX4Guid6assignENS0_4DataE
// type: Data *__fastcall(Data *this, Data)
#[doc(alias = "RBX::Guid::assign(RBX::Guid::Data)")]
pub fn stub_0x322b04() -> ! {
    todo!("0x322b04 RBX::Guid::assign(RBX::Guid::Data)")
}

// 0x322b10 — __ZNK3RBX4Guid4DataltERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Guid::Data::operator<(RBX::Guid::Data const&)const")]
pub fn stub_0x322b10() -> ! {
    todo!("0x322b10 RBX::Guid::Data::operator<(RBX::Guid::Data const&)const")
}

// 0x322b38 — __ZN3RBX4Guid7compareEPKS0_S2_
// type: _DWORD __fastcall(RBX::Guid *__hidden this, const RBX::Guid *, const RBX::Guid *)
#[doc(alias = "RBX::Guid::compare(RBX::Guid const*,RBX::Guid const*)")]
pub fn stub_0x322b38() -> ! {
    todo!("0x322b38 RBX::Guid::compare(RBX::Guid const*,RBX::Guid const*)")
}
