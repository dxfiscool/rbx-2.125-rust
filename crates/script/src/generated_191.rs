// Auto-generated skeletons for rbx-script — Lua/Script/lua filtered
// Filter: Lua|Script|lua (5041 filtered, 1777 remaining not yet in any crate) -> next 120 EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x58c788..0x76fa0c | script 13851->13971 distinct (filtered)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  " and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x58c788 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LuaWebServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService> RBX::Creatable<RBX::Instance>::create<RBX::LuaWebService>(void)")]
pub fn stub_0x58c788() -> ! {
    todo!("0x58c788 boost::shared_ptr<RBX::LuaWebService> RBX::Creatable<RBX::Instance>::create<RBX::LuaWebService>(void)")
}

// 0x58c838 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LuaWebServiceEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&)")]
pub fn stub_0x58c838() -> ! {
    todo!("0x58c838 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LuaWebService>(boost::shared_ptr<RBX::LuaWebService> const&)")
}

// 0x58c86c — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13LuaWebServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LuaWebService>(void)")]
pub fn stub_0x58c86c() -> ! {
    todo!("0x58c86c void RBX::ServiceProvider::callDoGetClassIndex<RBX::LuaWebService>(void)")
}

// 0x58c870 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x58c870() -> ! {
    todo!("0x58c870 boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x594f3c — __ZN5boost4bindIvN3RBX25ScriptInformationProvider13RequestResultEfNS_8weak_ptrINS1_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS1_8InstanceEEESt6vectorISB_SaISB_EEEEfNS9_ISF_EENS_8functionIFvSB_EEENS4_IS2_EESB_NS_3argILi1EEENSM_ILi4EEES6_SG_fSH_SK_SL_SB_EENS_3_bi6bind_tIT_PFSR_T0_T1_T2_T3_T4_T5_T6_T7_T8_ENSP_9list_av_9IT9_T10_T11_T12_T13_T14_T15_T16_T17_E4typeEEES12_S14_S15_S16_S17_S18_S19_S1A_S1B_S1C_
// type: int __fastcall(int, char, int, boost::detail::sp_counted_base *, char, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_9<boost::arg<1>,boost::arg<4>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>::type> boost::bind<void,RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>,boost::arg<1>,boost::arg<4>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>(void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::arg<1>,boost::arg<4>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x594f3c() -> ! {
    todo!("0x594f3c boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_9<boost::arg<1>,boost::arg<4>,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>>::type> boost::bind<void,RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>,boost::arg<1>,boost::arg<4>,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>>(void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::arg<1>,boost::arg<4>,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>)")
}

// 0x596684 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS2_8InstanceEEESt6vectorIS9_SaIS9_EEEEfNS7_ISD_EENS_8functionIFvS9_EEENS1_INS2_25ScriptInformationProviderEEES9_S4_SE_fSF_SI_SK_S9_EENS_3_bi6bind_tIT_PFSN_T0_T1_T2_T3_T4_T5_T6_ENSL_9list_av_7IT7_T8_T9_T10_T11_T12_T13_E4typeEEESW_SY_SZ_S10_S11_S12_S13_S14_
// type: int __fastcall(int, char, int, boost::detail::sp_counted_base *, char, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_7<rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>(void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x596684() -> ! {
    todo!("0x596684 boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_7<boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>>::type> boost::bind<void,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>>(void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>)")
}

// 0x596f44 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS4_8InstanceEEESt6vectorISC_SaISC_EEEEEENS2_IfEENS2_INSA_ISG_EEEENS2_INS_8functionIFvSC_EEEEENS2_INS3_INS4_25ScriptInformationProviderEEEEENS2_ISC_EEEC2ES7_SI_SJ_SL_SP_SS_ST_
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, char, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::list7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x596f44() -> ! {
    todo!("0x596f44 boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>::list7(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>)")
}

// 0x597118 — __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS4_8InstanceEEESt6vectorISC_SaISC_EEEEEENS2_IfEENS2_INSA_ISG_EEEENS2_INS_8functionIFvSC_EEEEENS2_INS3_INS4_25ScriptInformationProviderEEEEENS2_ISC_EEEC2ES7_SI_SJ_SL_SP_SS_ST_
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::storage7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x597118() -> ! {
    todo!("0x597118 boost::_bi::storage7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>::storage7(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>)")
}

// 0x597310 — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS4_8InstanceEEESt6vectorISC_SaISC_EEEEEENS2_IfEENS2_INSA_ISG_EEEENS2_INS_8functionIFvSC_EEEEENS2_INS3_INS4_25ScriptInformationProviderEEEEEEC2ES7_SI_SJ_SL_SP_SS_
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>)")]
pub fn stub_0x597310() -> ! {
    todo!("0x597310 boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>>::storage6(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>)")
}

// 0x5989ec — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS1_8InstanceEEESt6vectorISE_SaISE_EEEEfNSC_ISI_EENS_8functionIFvSE_EEENS8_INS1_25ScriptInformationProviderEEESE_ENS6_5list7INS6_5valueIS9_EENST_ISJ_EENST_IfEENST_ISK_EENST_ISN_EENST_ISP_EENST_ISE_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>)")]
pub fn stub_0x5989ec() -> ! {
    todo!("0x5989ec void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>)")
}

// 0x5991d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS6_8InstanceEEESt6vectorISD_SaISD_EEEEfNSB_ISH_EENS_8functionIFvSD_EEENS5_INS6_25ScriptInformationProviderEEESD_ENS3_5list7INS3_5valueIS8_EENSS_ISI_EENSS_IfEENSS_ISJ_EENSS_ISM_EENSS_ISO_EENSS_ISD_EEEEEEE6manageERKNS1_15function_bufferERS13_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x5991d8() -> ! {
    todo!("0x5991d8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x5991f4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS6_8InstanceEEESt6vectorISD_SaISD_EEEEfNSB_ISH_EENS_8functionIFvSD_EEENS5_INS6_25ScriptInformationProviderEEESD_ENS3_5list7INS3_5valueIS8_EENSS_ISI_EENSS_IfEENSS_ISJ_EENSS_ISM_EENSS_ISO_EENSS_ISD_EEEEEEvPS7_E6invokeERNS1_15function_bufferES12_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
pub fn stub_0x5991f4() -> ! {
    todo!("0x5991f4 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")
}

// 0x599210 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISG_SaISG_EEEEfNSE_ISK_EENS_8functionIFvSG_EEENSA_INS3_25ScriptInformationProviderEEESG_ENS8_5list7INS8_5valueISB_EENSV_ISL_EENSV_IfEENSV_ISM_EENSV_ISP_EENSV_ISR_EENSV_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x599210() -> ! {
    todo!("0x599210 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")
}

// 0x5999e8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISG_SaISG_EEEEfNSE_ISK_EENS_8functionIFvSG_EEENSA_INS3_25ScriptInformationProviderEEESG_ENS8_5list7INS8_5valueISB_EENSV_ISL_EENSV_IfEENSV_ISM_EENSV_ISP_EENSV_ISR_EENSV_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x5999e8() -> ! {
    todo!("0x5999e8 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x59a1bc — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISG_SaISG_EEEEfNSE_ISK_EENS_8functionIFvSG_EEENSA_INS3_25ScriptInformationProviderEEESG_ENS8_5list7INS8_5valueISB_EENSV_ISL_EENSV_IfEENSV_ISM_EENSV_ISP_EENSV_ISR_EENSV_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x59a1bc() -> ! {
    todo!("0x59a1bc void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x59a324 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS4_8InstanceEEESt6vectorISC_SaISC_EEEEEENS2_IfEENS2_INSA_ISG_EEEENS2_INS_8functionIFvSC_EEEEENS2_INS3_INS4_25ScriptInformationProviderEEEEENS2_ISC_EEEclIPFvS6_SH_fSK_SO_SR_SC_ENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
pub fn stub_0x59a324() -> ! {
    todo!("0x59a324 void boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>::operator()<void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")
}

// 0x59a4f8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS6_8InstanceEEESt6vectorISD_SaISD_EEEEfNSB_ISH_EENS_8functionIFvSD_EEENS5_INS6_25ScriptInformationProviderEEESD_ENS3_5list7INS3_5valueIS8_EENSS_ISI_EENSS_IfEENSS_ISJ_EENSS_ISM_EENSS_ISO_EENSS_ISD_EEEEEEE7managerERKNS1_15function_bufferERS13_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x59a4f8() -> ! {
    todo!("0x59a4f8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x59a754 — __ZN5boost3_bi5list9INS_3argILi1EEENS2_ILi4EEENS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS5_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS7_8InstanceEEESt6vectorISF_SaISF_EEEEEENS5_IfEENS5_INSD_ISJ_EEEENS5_INS_8functionIFvSF_EEEEENS5_INS6_INS7_25ScriptInformationProviderEEEEENS5_ISF_EEEC2ES3_S4_SA_SL_SM_SO_SS_SV_SW_
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, char, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::list9(boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x59a754() -> ! {
    todo!("0x59a754 boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>::list9(boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>)")
}

// 0x59a928 — __ZN5boost3_bi8storage9INS_3argILi1EEENS2_ILi4EEENS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS5_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS7_8InstanceEEESt6vectorISF_SaISF_EEEEEENS5_IfEENS5_INSD_ISJ_EEEENS5_INS_8functionIFvSF_EEEEENS5_INS6_INS7_25ScriptInformationProviderEEEEENS5_ISF_EEEC2ES3_S4_SA_SL_SM_SO_SS_SV_SW_
// type: uintptr_t __fastcall(uintptr_t, void (__cdecl **)(_Unwind_Reason_Code, _Unwind_Exception *), boost::detail::sp_counted_base *, int, const shared_count *, int, struct _Unwind_Exception *lpuexcpt, const shared_count *, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::storage9(boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x59a928() -> ! {
    todo!("0x59a928 boost::_bi::storage9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>::storage9(boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>)")
}

// 0x59ab20 — __ZN5boost3_bi8storage8INS_3argILi1EEENS2_ILi4EEENS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS5_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS7_8InstanceEEESt6vectorISF_SaISF_EEEEEENS5_IfEENS5_INSD_ISJ_EEEENS5_INS_8functionIFvSF_EEEEENS5_INS6_INS7_25ScriptInformationProviderEEEEEEC2ES3_S4_SA_SL_SM_SO_SS_SV_
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage8<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>>::storage8(boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>)")]
pub fn stub_0x59ab20() -> ! {
    todo!("0x59ab20 boost::_bi::storage8<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>>::storage8(boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>)")
}

// 0x59c1fc — __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE9assign_toINS_3_bi6bind_tIvPFvS3_fNS_8weak_ptrINS1_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS1_8InstanceEEESt6vectorISF_SaISF_EEEEfNSD_ISJ_EENS_8functionIFvSF_EEENS8_IS2_EESF_ENS6_5list9INS_3argILi1EEENST_ILi4EEENS6_5valueISA_EENSW_ISK_EENSW_IfEENSW_ISL_EENSW_ISO_EENSW_ISP_EENSW_ISF_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>)")]
pub fn stub_0x59c1fc() -> ! {
    todo!("0x59c1fc void boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>)")
}

// 0x59c9e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX25ScriptInformationProvider13RequestResultEfNS_8weak_ptrINS5_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS5_8InstanceEEESt6vectorISF_SaISF_EEEEfNSD_ISJ_EENS_8functionIFvSF_EEENS8_IS6_EESF_ENS3_5list9INS_3argILi1EEENST_ILi4EEENS3_5valueISA_EENSW_ISK_EENSW_IfEENSW_ISL_EENSW_ISO_EENSW_ISP_EENSW_ISF_EEEEEEE6manageERKNS1_15function_bufferERS17_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x59c9e8() -> ! {
    todo!("0x59c9e8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x59ca04 — __ZN5boost6detail8function26void_function_obj_invoker5INS_3_bi6bind_tIvPFvN3RBX25ScriptInformationProvider13RequestResultEfNS_8weak_ptrINS5_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS5_8InstanceEEESt6vectorISF_SaISF_EEEEfNSD_ISJ_EENS_8functionIFvSF_EEENS8_IS6_EESF_ENS3_5list9INS_3argILi1EEENST_ILi4EEENS3_5valueISA_EENSW_ISK_EENSW_IfEENSW_ISL_EENSW_ISO_EENSW_ISP_EENSW_ISF_EEEEEEvS7_bbfbE6invokeERNS1_15function_bufferES7_bbfb
// type: int __fastcall(int, int, int, int, float, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker5<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::invoke(boost::detail::function::function_buffer &,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)")]
pub fn stub_0x59ca04() -> ! {
    todo!("0x59ca04 boost::detail::function::void_function_obj_invoker5<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::invoke(boost::detail::function::function_buffer &,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)")
}

// 0x59ca4c — __ZNK5boost6detail8function13basic_vtable5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE9assign_toINS_3_bi6bind_tIvPFvS5_fNS_8weak_ptrINS3_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISH_SaISH_EEEEfNSF_ISL_EENS_8functionIFvSH_EEENSA_IS4_EESH_ENS8_5list9INS_3argILi1EEENSV_ILi4EEENS8_5valueISC_EENSY_ISM_EENSY_IfEENSY_ISN_EENSY_ISQ_EENSY_ISR_EENSY_ISH_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x59ca4c() -> ! {
    todo!("0x59ca4c bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")
}

// 0x59d224 — __ZNK5boost6detail8function13basic_vtable5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE9assign_toINS_3_bi6bind_tIvPFvS5_fNS_8weak_ptrINS3_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISH_SaISH_EEEEfNSF_ISL_EENS_8functionIFvSH_EEENSA_IS4_EESH_ENS8_5list9INS_3argILi1EEENSV_ILi4EEENS8_5valueISC_EENSY_ISM_EENSY_IfEENSY_ISN_EENSY_ISQ_EENSY_ISR_EENSY_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x59d224() -> ! {
    todo!("0x59d224 bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x59d9f8 — __ZNK5boost6detail8function13basic_vtable5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE14assign_functorINS_3_bi6bind_tIvPFvS5_fNS_8weak_ptrINS3_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISH_SaISH_EEEEfNSF_ISL_EENS_8functionIFvSH_EEENSA_IS4_EESH_ENS8_5list9INS_3argILi1EEENSV_ILi4EEENS8_5valueISC_EENSY_ISM_EENSY_IfEENSY_ISN_EENSY_ISQ_EENSY_ISR_EENSY_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x59d9f8() -> ! {
    todo!("0x59d9f8 void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x59db60 — __ZN5boost3_bi5list9INS_3argILi1EEENS2_ILi4EEENS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS5_IN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS7_8InstanceEEESt6vectorISF_SaISF_EEEEEENS5_IfEENS5_INSD_ISJ_EEEENS5_INS_8functionIFvSF_EEEEENS5_INS6_INS7_25ScriptInformationProviderEEEEENS5_ISF_EEEclIPFvNST_13RequestResultEfS9_SK_fSN_SR_SU_SF_ENS0_5list5IRSZ_RbS14_RfS14_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::operator()<void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &>>(boost::_bi::type<void>,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &> &,int)")]
pub fn stub_0x59db60() -> ! {
    todo!("0x59db60 void boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>::operator()<void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &>>(boost::_bi::type<void>,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &> &,int)")
}

// 0x59dd4c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX25ScriptInformationProvider13RequestResultEfNS_8weak_ptrINS5_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS5_8InstanceEEESt6vectorISF_SaISF_EEEEfNSD_ISJ_EENS_8functionIFvSF_EEENS8_IS6_EESF_ENS3_5list9INS_3argILi1EEENST_ILi4EEENS3_5valueISA_EENSW_ISK_EENSW_IfEENSW_ISL_EENSW_ISO_EENSW_ISP_EENSW_ISF_EEEEEEE7managerERKNS1_15function_bufferERS17_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x59dd4c() -> ! {
    todo!("0x59dd4c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x59dfa8 — __ZN5boost10shared_ptrIN3RBX25ScriptInformationProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider>::shared_ptr<RBX::ScriptInformationProvider>(rbx_core::WeakPtr<RBX::ScriptInformationProvider> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x59dfa8() -> ! {
    todo!("0x59dfa8 boost::shared_ptr<RBX::ScriptInformationProvider>::shared_ptr<RBX::ScriptInformationProvider>(boost::weak_ptr<RBX::ScriptInformationProvider> const&,boost::detail::sp_nothrow_tag)")
}

// 0x59e024 — __ZN5boost3_bi5list3INS0_5valueIPSt6vectorINS_10shared_ptrIN3RBX8InstanceEEESaIS7_EEEENS2_IPNS5_25ScriptInformationProviderEEENS_3argILi1EEEEclIPFvSA_SD_S7_ENS0_5list1IRKS7_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *>,boost::_bi::value<RBX::ScriptInformationProvider *>,boost::arg<1>>::operator()<void (*)(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_0x59e024() -> ! {
    todo!("0x59e024 void boost::_bi::list3<boost::_bi::value<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *>,boost::_bi::value<RBX::ScriptInformationProvider *>,boost::arg<1>>::operator()<void (*)(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,boost::shared_ptr<RBX::Instance>) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")
}

// 0x5ba2b4 — __ZN3RBX24KeyframeSequenceProvider22getKeyframeSequenceLuaENS_9ContentIdE
#[doc(alias = "RBX::KeyframeSequenceProvider::getKeyframeSequenceLua(RBX::ContentId)")]
pub fn stub_0x5ba2b4() -> ! {
    todo!("0x5ba2b4 RBX::KeyframeSequenceProvider::getKeyframeSequenceLua(RBX::ContentId)")
}

// 0x5fb8e0 — __ZN3RBX13BasePlayerGui15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fb8e0() -> ! {
    todo!("0x5fb8e0 RBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")
}

// 0x5fba7c — __ZThn92_N3RBX13BasePlayerGui15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::BasePlayerGui *__hidden this, RBX::BaseScript *)
#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fba7c() -> ! {
    todo!("0x5fba7c non-virtual thunk toRBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")
}

// 0x5fd8b4 — __ZN3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fd8b4() -> ! {
    todo!("0x5fd8b4 RBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")
}

// 0x5fda38 — __ZThn92_N3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fda38() -> ! {
    todo!("0x5fda38 non-virtual thunk toRBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")
}

// 0x5fdcb4 — __ZN3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fdcb4() -> ! {
    todo!("0x5fdcb4 RBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")
}

// 0x5fe170 — __ZThn92_N3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "non-virtual thunk toRBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fe170() -> ! {
    todo!("0x5fe170 non-virtual thunk toRBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")
}

// 0x614a00 — __ZN3RBX18ScriptMouseCommandC1EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")]
pub fn stub_0x614a00() -> ! {
    todo!("0x614a00 RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")
}

// 0x614a04 — __ZN3RBX18ScriptMouseCommandC2EPNS_9WorkspaceE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, RBX::Workspace *)
#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *) [0x614a04]")]
pub fn stub_0x614a04() -> ! {
    todo!("0x614a04 RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")
}

// 0x614b58 — __ZN3RBX18ScriptMouseCommandD0Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand()")]
pub fn stub_0x614b58() -> ! {
    todo!("0x614b58 RBX::ScriptMouseCommand::~ScriptMouseCommand()")
}

// 0x614bf8 — __ZN3RBX18ScriptMouseCommandD1Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand() [0x614bf8]")]
pub fn stub_0x614bf8() -> ! {
    todo!("0x614bf8 RBX::ScriptMouseCommand::~ScriptMouseCommand()")
}

// 0x614bfc — __ZThn36_N3RBX18ScriptMouseCommandD0Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand()")]
pub fn stub_0x614bfc() -> ! {
    todo!("0x614bfc non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand()")
}

// 0x614c04 — __ZN3RBX18ScriptMouseCommandD2Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand() [0x614c04]")]
pub fn stub_0x614c04() -> ! {
    todo!("0x614c04 RBX::ScriptMouseCommand::~ScriptMouseCommand()")
}

// 0x614d30 — __ZThn36_N3RBX18ScriptMouseCommandD1Ev
// type: void __fastcall(RBX::ScriptMouseCommand *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand() [0x614d30]")]
pub fn stub_0x614d30() -> ! {
    todo!("0x614d30 non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand()")
}

// 0x614d38 — __ZNK3RBX18ScriptMouseCommand11getCursorIdEv
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this)
#[doc(alias = "RBX::ScriptMouseCommand::getCursorId(void)const")]
pub fn stub_0x614d38() -> ! {
    todo!("0x614d38 RBX::ScriptMouseCommand::getCursorId(void)const")
}

// 0x614d48 — __ZN3RBX18ScriptMouseCommand11onMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x614d48() -> ! {
    todo!("0x614d48 RBX::ScriptMouseCommand::onMouseDown(RBX::UIEvent const&)")
}

// 0x614e20 — __ZN3RBX18ScriptMouseCommand12onMouseHoverERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onMouseHover(RBX::UIEvent const&)")]
pub fn stub_0x614e20() -> ! {
    todo!("0x614e20 RBX::ScriptMouseCommand::onMouseHover(RBX::UIEvent const&)")
}

// 0x614e2c — __ZN3RBX18ScriptMouseCommand11onMouseIdleERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_0x614e2c() -> ! {
    todo!("0x614e2c RBX::ScriptMouseCommand::onMouseIdle(RBX::UIEvent const&)")
}

// 0x614e38 — __ZN3RBX18ScriptMouseCommand19onMouseWheelForwardERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onMouseWheelForward(RBX::UIEvent const&)")]
pub fn stub_0x614e38() -> ! {
    todo!("0x614e38 RBX::ScriptMouseCommand::onMouseWheelForward(RBX::UIEvent const&)")
}

// 0x614f10 — __ZN3RBX18ScriptMouseCommand20onMouseWheelBackwardERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onMouseWheelBackward(RBX::UIEvent const&)")]
pub fn stub_0x614f10() -> ! {
    todo!("0x614f10 RBX::ScriptMouseCommand::onMouseWheelBackward(RBX::UIEvent const&)")
}

// 0x614fe8 — __ZN3RBX18ScriptMouseCommand16onRightMouseDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onRightMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x614fe8() -> ! {
    todo!("0x614fe8 RBX::ScriptMouseCommand::onRightMouseDown(RBX::UIEvent const&)")
}

// 0x6150c0 — __ZN3RBX18ScriptMouseCommand14onRightMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onRightMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x6150c0() -> ! {
    todo!("0x6150c0 RBX::ScriptMouseCommand::onRightMouseUp(RBX::UIEvent const&)")
}

// 0x615198 — __ZN3RBX18ScriptMouseCommand9onMouseUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x615198() -> ! {
    todo!("0x615198 RBX::ScriptMouseCommand::onMouseUp(RBX::UIEvent const&)")
}

// 0x615270 — __ZN3RBX18ScriptMouseCommand13onPeekKeyDownERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onPeekKeyDown(RBX::UIEvent const&)")]
pub fn stub_0x615270() -> ! {
    todo!("0x615270 RBX::ScriptMouseCommand::onPeekKeyDown(RBX::UIEvent const&)")
}

// 0x615348 — __ZN3RBX18ScriptMouseCommand11onPeekKeyUpERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::ScriptMouseCommand::onPeekKeyUp(RBX::UIEvent const&)")]
pub fn stub_0x615348() -> ! {
    todo!("0x615348 RBX::ScriptMouseCommand::onPeekKeyUp(RBX::UIEvent const&)")
}

// 0x615420 — __ZNK3RBX18ScriptMouseCommand7getNameEv
// type: _DWORD __fastcall(RBX::ScriptMouseCommand *__hidden this)
#[doc(alias = "RBX::ScriptMouseCommand::getName(void)const")]
pub fn stub_0x615420() -> ! {
    todo!("0x615420 RBX::ScriptMouseCommand::getName(void)const")
}

// 0x615424 — __ZN3RBX11shared_fromINS_18ScriptMouseCommandEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptMouseCommand> RBX::shared_from<RBX::ScriptMouseCommand>(RBX::ScriptMouseCommand*)")]
pub fn stub_0x615424() -> ! {
    todo!("0x615424 boost::shared_ptr<RBX::ScriptMouseCommand> RBX::shared_from<RBX::ScriptMouseCommand>(RBX::ScriptMouseCommand*)")
}

// 0x619080 — __ZN3RBX9Selection26propagateChangeSignalToLuaERKNS_16SelectionChangedE
#[doc(alias = "RBX::Selection::propagateChangeSignalToLua(RBX::SelectionChanged const&)")]
pub fn stub_0x619080() -> ! {
    todo!("0x619080 RBX::Selection::propagateChangeSignalToLua(RBX::SelectionChanged const&)")
}

// 0x647bd4 — __ZN3RBX5Stats12StatsService16tryToStartScriptEv
// type: _DWORD __fastcall(RBX::Stats::StatsService *__hidden this)
#[doc(alias = "RBX::Stats::StatsService::tryToStartScript(void)")]
pub fn stub_0x647bd4() -> ! {
    todo!("0x647bd4 RBX::Stats::StatsService::tryToStartScript(void)")
}

// 0x652a70 — __ZN3RBX4Name13callDoDeclareILZNS_14sScriptContextEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sScriptContextEEEEvv")]
pub fn stub_0x652a70() -> ! {
    todo!("0x652a70 __ZN3RBX4Name13callDoDeclareILZNS_14sScriptContextEEEEvv")
}

// 0x689260 — __ZNK3RBX5NamedINS_18ScriptMouseCommandELZNS_17sToolMouseCommandEEE7getNameEv
#[doc(alias = "__ZNK3RBX5NamedINS_18ScriptMouseCommandELZNS_17sToolMouseCommandEEE7getNameEv")]
pub fn stub_0x689260() -> ! {
    todo!("0x689260 __ZNK3RBX5NamedINS_18ScriptMouseCommandELZNS_17sToolMouseCommandEEE7getNameEv")
}

// 0x6d0138 — __ZN3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::Workspace *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x6d0138() -> ! {
    todo!("0x6d0138 RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")
}

// 0x6d02e4 — __ZThn388_N3RBX9Workspace15scriptShouldRunEPNS_10BaseScriptE
// type: int __fastcall(RBX::Workspace *this, RBX::BaseScript *)
#[doc(alias = "non-virtual thunk toRBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x6d02e4() -> ! {
    todo!("0x6d02e4 non-virtual thunk toRBX::Workspace::scriptShouldRun(RBX::BaseScript *)")
}

// 0x6d3ca0 — __ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv")]
pub fn stub_0x6d3ca0() -> ! {
    todo!("0x6d3ca0 __ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv")
}

// 0x6f98e4 — __ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: void __fastcall(RBX::Lua::WeakFunctionRef *, _DWORD **)
#[doc(alias = "RBX::Lua::WeakFunctionRef rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x6f98e4() -> ! {
    todo!("0x6f98e4 RBX::Lua::WeakFunctionRef rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x701470 — __ZN3RBX8Instance8luaCloneEv
// type: _DWORD __fastcall(RBX::Instance *__hidden this)
#[doc(alias = "RBX::Instance::luaClone(void)")]
pub fn stub_0x701470() -> ! {
    todo!("0x701470 RBX::Instance::luaClone(void)")
}

// 0x705bb0 — __ZNK3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(void)const")]
pub fn stub_0x705bb0() -> ! {
    todo!("0x705bb0 RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(void)const")
}

// 0x705d28 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE15isNullClassNameEv")]
pub fn stub_0x705d28() -> ! {
    todo!("0x705d28 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE15isNullClassNameEv")
}

// 0x705dc8 — __ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v")]
pub fn stub_0x705dc8() -> ! {
    todo!("0x705dc8 __ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v")
}

// 0x705e10 — __ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v")]
pub fn stub_0x705e10() -> ! {
    todo!("0x705e10 __ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v")
}

// 0x705ef8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptService>(void)")]
pub fn stub_0x705ef8() -> ! {
    todo!("0x705ef8 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptService>(void)")
}

// 0x7105b8 — __ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x7105b8() -> ! {
    todo!("0x7105b8 __ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x762f10 — __ZN3RBX5World27onAssemblyInSimluationStageEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::World *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::World::onAssemblyInSimluationStage(RBX::Assembly *)")]
pub fn stub_0x762f10() -> ! {
    todo!("0x762f10 RBX::World::onAssemblyInSimluationStage(RBX::Assembly *)")
}

// 0x767b10 — __ZN3RBX9Scripting15DebuggerManager15enableDebuggingEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::enableDebugging(void)")]
pub fn stub_0x767b10() -> ! {
    todo!("0x767b10 RBX::Scripting::DebuggerManager::enableDebugging(void)")
}

// 0x76829c — __ZN3RBX9Scripting14ScriptDebugger6resumeEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::resume(void)")]
pub fn stub_0x76829c() -> ! {
    todo!("0x76829c RBX::Scripting::ScriptDebugger::resume(void)")
}

// 0x7685c4 — __ZN3RBX9Scripting14ScriptDebugger8stepOverEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOver(void)")]
pub fn stub_0x7685c4() -> ! {
    todo!("0x7685c4 RBX::Scripting::ScriptDebugger::stepOver(void)")
}

// 0x768750 — __ZN3RBX9Scripting14ScriptDebugger8stepIntoEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::stepInto(void)")]
pub fn stub_0x768750() -> ! {
    todo!("0x768750 RBX::Scripting::ScriptDebugger::stepInto(void)")
}

// 0x7688d8 — __ZN3RBX9Scripting14ScriptDebugger7stepOutEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOut(void)")]
pub fn stub_0x7688d8() -> ! {
    todo!("0x7688d8 RBX::Scripting::ScriptDebugger::stepOut(void)")
}

// 0x769338 — __ZN3RBX9Scripting14ScriptDebugger9getLocalsEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getLocals(int)")]
pub fn stub_0x769338() -> ! {
    todo!("0x769338 RBX::Scripting::ScriptDebugger::getLocals(int)")
}

// 0x769414 — __ZN3RBX9Scripting14ScriptDebugger11getUpvaluesEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getUpvalues(int)")]
pub fn stub_0x769414() -> ! {
    todo!("0x769414 RBX::Scripting::ScriptDebugger::getUpvalues(int)")
}

// 0x7694f0 — __ZN3RBX9Scripting14ScriptDebugger10getGlobalsEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getGlobals(void)")]
pub fn stub_0x7694f0() -> ! {
    todo!("0x7694f0 RBX::Scripting::ScriptDebugger::getGlobals(void)")
}

// 0x769db0 — __ZNK3RBX9Scripting14ScriptDebugger13getScriptPathEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getScriptPath(void)const")]
pub fn stub_0x769db0() -> ! {
    todo!("0x769db0 RBX::Scripting::ScriptDebugger::getScriptPath(void)const")
}

// 0x769f7c — __ZN3RBX9Scripting14ScriptDebugger13setScriptPathESs
#[doc(alias = "RBX::Scripting::ScriptDebugger::setScriptPath(std::string)")]
pub fn stub_0x769f7c() -> ! {
    todo!("0x769f7c RBX::Scripting::ScriptDebugger::setScriptPath(std::string)")
}

// 0x76a5c0 — __ZN3RBX9Scripting13DebuggerWatch21checkExpressionSyntaxEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerWatch::checkExpressionSyntax(void)")]
pub fn stub_0x76a5c0() -> ! {
    todo!("0x76a5c0 RBX::Scripting::DebuggerWatch::checkExpressionSyntax(void)")
}

// 0x76a92c — __ZN3RBX9Scripting15DebuggerManager9singletonEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::singleton(void)")]
pub fn stub_0x76a92c() -> ! {
    todo!("0x76a92c RBX::Scripting::DebuggerManager::singleton(void)")
}

// 0x76ab8c — __ZN3RBX9Scripting15DebuggerManagerC2Ev
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::DebuggerManager(void)")]
pub fn stub_0x76ab8c() -> ! {
    todo!("0x76ab8c RBX::Scripting::DebuggerManager::DebuggerManager(void)")
}

// 0x76aec4 — __ZN3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
pub fn stub_0x76aec4() -> ! {
    todo!("0x76aec4 RBX::Scripting::DebuggerManager::~DebuggerManager()")
}

// 0x76af64 — __ZN3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager() [0x76af64]")]
pub fn stub_0x76af64() -> ! {
    todo!("0x76af64 RBX::Scripting::DebuggerManager::~DebuggerManager()")
}

// 0x76af68 — __ZThn32_N3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")]
pub fn stub_0x76af68() -> ! {
    todo!("0x76af68 non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")
}

// 0x76af70 — __ZThn36_N3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager() [0x76af70]")]
pub fn stub_0x76af70() -> ! {
    todo!("0x76af70 non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")
}

// 0x76af78 — __ZN3RBX9Scripting15DebuggerManagerD2Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager() [0x76af78]")]
pub fn stub_0x76af78() -> ! {
    todo!("0x76af78 RBX::Scripting::DebuggerManager::~DebuggerManager()")
}

// 0x76b128 — __ZThn32_N3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager() [0x76b128]")]
pub fn stub_0x76b128() -> ! {
    todo!("0x76b128 non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")
}

// 0x76b130 — __ZThn36_N3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager() [0x76b130]")]
pub fn stub_0x76b130() -> ! {
    todo!("0x76b130 non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")
}

// 0x76b13c — __ZN3RBX9Scripting15DebuggerManager12findDebuggerEP9lua_State
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(lua_State *)")]
pub fn stub_0x76b13c() -> ! {
    todo!("0x76b13c RBX::Scripting::DebuggerManager::findDebugger(lua_State *)")
}

// 0x76b2b0 — __ZN3RBX9Scripting15DebuggerManager12findDebuggerEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(RBX::Script *)")]
pub fn stub_0x76b2b0() -> ! {
    todo!("0x76b2b0 RBX::Scripting::DebuggerManager::findDebugger(RBX::Script *)")
}

// 0x76b470 — __ZN3RBX9Scripting15DebuggerManager11addDebuggerEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::DebuggerManager::addDebugger(RBX::Script *)")]
pub fn stub_0x76b470() -> ! {
    todo!("0x76b470 RBX::Scripting::DebuggerManager::addDebugger(RBX::Script *)")
}

// 0x76b99c — __ZN3RBX9Scripting14ScriptDebuggerC2ERNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::ScriptDebugger(RBX::Script &)")]
pub fn stub_0x76b99c() -> ! {
    todo!("0x76b99c RBX::Scripting::ScriptDebugger::ScriptDebugger(RBX::Script &)")
}

// 0x76c054 — __ZN3RBX9Scripting14ScriptDebugger9setScriptEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Script *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::setScript(RBX::Script *)")]
pub fn stub_0x76c054() -> ! {
    todo!("0x76c054 RBX::Scripting::ScriptDebugger::setScript(RBX::Script *)")
}

// 0x76c3a4 — __ZN3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
pub fn stub_0x76c3a4() -> ! {
    todo!("0x76c3a4 RBX::Scripting::ScriptDebugger::~ScriptDebugger()")
}

// 0x76c444 — __ZN3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76c444]")]
pub fn stub_0x76c444() -> ! {
    todo!("0x76c444 RBX::Scripting::ScriptDebugger::~ScriptDebugger()")
}

// 0x76c448 — __ZThn32_N3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
pub fn stub_0x76c448() -> ! {
    todo!("0x76c448 non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")
}

// 0x76c450 — __ZThn36_N3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76c450]")]
pub fn stub_0x76c450() -> ! {
    todo!("0x76c450 non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")
}

// 0x76c458 — __ZN3RBX9Scripting14ScriptDebuggerD2Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76c458]")]
pub fn stub_0x76c458() -> ! {
    todo!("0x76c458 RBX::Scripting::ScriptDebugger::~ScriptDebugger()")
}

// 0x76ca0c — __ZThn32_N3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76ca0c]")]
pub fn stub_0x76ca0c() -> ! {
    todo!("0x76ca0c non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")
}

// 0x76ca14 — __ZThn36_N3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76ca14]")]
pub fn stub_0x76ca14() -> ! {
    todo!("0x76ca14 non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")
}

// 0x76ca1c — __ZN3RBX9Scripting14ScriptDebugger8addWatchESs
#[doc(alias = "RBX::Scripting::ScriptDebugger::addWatch(std::string)")]
pub fn stub_0x76ca1c() -> ! {
    todo!("0x76ca1c RBX::Scripting::ScriptDebugger::addWatch(std::string)")
}

// 0x76cb6c — __ZN3RBX9Scripting14ScriptDebugger13getWatchValueEPNS0_13DebuggerWatchE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Scripting::DebuggerWatch *)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatchValue(RBX::Scripting::DebuggerWatch *)")]
pub fn stub_0x76cb6c() -> ! {
    todo!("0x76cb6c RBX::Scripting::ScriptDebugger::getWatchValue(RBX::Scripting::DebuggerWatch *)")
}

// 0x76cd58 — __ZL14readWatchValueSsP9lua_State
#[doc(alias = "readWatchValue(std::string,lua_State *)")]
pub fn stub_0x76cd58() -> ! {
    todo!("0x76cd58 readWatchValue(std::string,lua_State *)")
}

// 0x76d500 — __ZN3RBX9Scripting14ScriptDebugger4hookEP9lua_StateP9lua_Debug
#[doc(alias = "RBX::Scripting::ScriptDebugger::hook(lua_State *,lua_Debug *)")]
pub fn stub_0x76d500() -> ! {
    todo!("0x76d500 RBX::Scripting::ScriptDebugger::hook(lua_State *,lua_Debug *)")
}

// 0x76d5e0 — __ZN3RBX9Scripting14ScriptDebugger13debuggerBreakEP9lua_StateP9lua_Debug
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::debuggerBreak(lua_State *,lua_Debug *)")]
pub fn stub_0x76d5e0() -> ! {
    todo!("0x76d5e0 RBX::Scripting::ScriptDebugger::debuggerBreak(lua_State *,lua_Debug *)")
}

// 0x76d95c — __ZN3RBX9Scripting14ScriptDebugger10readLocalsEiP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readLocals(int,lua_State *)")]
pub fn stub_0x76d95c() -> ! {
    todo!("0x76d95c RBX::Scripting::ScriptDebugger::readLocals(int,lua_State *)")
}

// 0x76dc5c — __ZN3RBX9Scripting14ScriptDebugger11readGlobalsEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readGlobals(lua_State *)")]
pub fn stub_0x76dc5c() -> ! {
    todo!("0x76dc5c RBX::Scripting::ScriptDebugger::readGlobals(lua_State *)")
}

// 0x76dfcc — __ZN3RBX9Scripting14ScriptDebugger12readUpvaluesEiP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readUpvalues(int,lua_State *)")]
pub fn stub_0x76dfcc() -> ! {
    todo!("0x76dfcc RBX::Scripting::ScriptDebugger::readUpvalues(int,lua_State *)")
}

// 0x76e434 — __ZN3RBX9Scripting14ScriptDebugger9readStackEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::readStack(lua_State *)")]
pub fn stub_0x76e434() -> ! {
    todo!("0x76e434 RBX::Scripting::ScriptDebugger::readStack(lua_State *)")
}

// 0x76e860 — __ZN3RBX9Scripting14ScriptDebugger20getScriptForLuaStateEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::getScriptForLuaState(lua_State *)")]
pub fn stub_0x76e860() -> ! {
    todo!("0x76e860 RBX::Scripting::ScriptDebugger::getScriptForLuaState(lua_State *)")
}

// 0x76ea28 — __ZN3RBX9Scripting14ScriptDebugger10onLineHookEP9lua_StateP9lua_Debug
// type: int __fastcall(char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::onLineHook(lua_State *,lua_Debug *)")]
pub fn stub_0x76ea28() -> ! {
    todo!("0x76ea28 RBX::Scripting::ScriptDebugger::onLineHook(lua_State *,lua_Debug *)")
}

// 0x76ecb0 — __ZN3RBX9Scripting14ScriptDebugger14findBreakpointEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::findBreakpoint(int)")]
pub fn stub_0x76ecb0() -> ! {
    todo!("0x76ecb0 RBX::Scripting::ScriptDebugger::findBreakpoint(int)")
}

// 0x76ece8 — __ZN3RBX9Scripting14ScriptDebugger11shouldBreakEPNS0_18DebuggerBreakpointEP9lua_State
#[doc(alias = "RBX::Scripting::ScriptDebugger::shouldBreak(RBX::Scripting::DebuggerBreakpoint *,lua_State *)")]
pub fn stub_0x76ece8() -> ! {
    todo!("0x76ece8 RBX::Scripting::ScriptDebugger::shouldBreak(RBX::Scripting::DebuggerBreakpoint *,lua_State *)")
}

// 0x76f488 — __ZN3RBX9Scripting14ScriptDebugger13setBreakpointEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::setBreakpoint(int)")]
pub fn stub_0x76f488() -> ! {
    todo!("0x76f488 RBX::Scripting::ScriptDebugger::setBreakpoint(int)")
}

// 0x76fa0c — __ZN3RBX9Scripting14ScriptDebugger16onScriptStartingEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int)
#[doc(alias = "RBX::Scripting::ScriptDebugger::onScriptStarting(lua_State *)")]
pub fn stub_0x76fa0c() -> ! {
    todo!("0x76fa0c RBX::Scripting::ScriptDebugger::onScriptStarting(lua_State *)")
}
