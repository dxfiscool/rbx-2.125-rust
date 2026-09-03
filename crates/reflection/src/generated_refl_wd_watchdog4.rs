//! reflection — generated_refl_wd_watchdog4 — 120 stubs EA-sorted asc global gap filler 0x2fe5f0..0x3044c4 not yet in crates/reflection/src
//! Source: ida/export.json (85545 funcs) EA asc not in crates/reflection/src — next 120 uncovered sorted asc (RBX::Reflection 16171 filtered exhausted, fallback global gap)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Shard WD_watchdog4 — watchdog shard 4 (global gap filler EA-sorted asc after 0x2fe524)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2fe5f0 — __ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_
#[doc(alias = "boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4HttpEEaSERKS3_")]
pub fn stub_0x2fe5f0() -> ! {
    todo!("0x2fe5f0 boost::shared_ptr<RBX::Http>::operator=(boost::shared_ptr<RBX::Http> const&)")
}

// 0x2fe628 — __ZN5boost10shared_ptrISsE5resetISsEEvPT_
#[doc(alias = "void boost::shared_ptr<std::string>::reset<std::string>(std::string *)")]
#[doc(alias = "__ZN5boost10shared_ptrISsE5resetISsEEvPT_")]
pub fn stub_0x2fe628() -> ! {
    todo!("0x2fe628 void boost::shared_ptr<std::string>::reset<std::string>(std::string *)")
}

// 0x2fe654 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_")]
pub fn stub_0x2fe654() -> ! {
    todo!("0x2fe654 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue...")
}

// 0x2fe884 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX14AsyncHttpQueue15CallbackWrapperESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_PNS2_8InstanceENS3_13RequestResultENSA_10shared_ptrISsEEENSB_5list4INSA_3argILi1EEENSB_5valueISE_EENSN_ISF_EENSN_ISH_EEEEEEET0_T_SU_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>>)")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX14AsyncHttpQueue15CallbackWrapperESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvPFvS4_PNS2_8InstanceENS3_13RequestResultENSA_10shared_ptrISsEEENSB_5list4INSA_3argILi1EEENSB_5valueISE_EENSN_ISF_EENSN_ISH_EEEEEEET0_T_SU_ST_")]
pub fn stub_0x2fe884() -> ! {
    todo!("0x2fe884 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::...")
}

// 0x2fe8f4 — __ZN5boost4bindIvN3RBX14AsyncHttpQueue15CallbackWrapperEPNS1_8InstanceENS2_13RequestResultENS_10shared_ptrISsEENS_3argILi1EEES5_S6_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX14AsyncHttpQueue15CallbackWrapperEPNS1_8InstanceENS2_13RequestResultENS_10shared_ptrISsEENS_3argILi1EEES5_S6_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_T3_ENSB_9list_av_4IT4_T5_T6_T7_E4typeEEESJ_SL_SM_SN_SO_")]
pub fn stub_0x2fe8f4() -> ! {
    todo!("0x2fe8f4 boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Ins...")
}

// 0x2fea20 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
#[doc(alias = "__ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_")]
pub fn stub_0x2fea20() -> ! {
    todo!("0x2fea20 std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")
}

// 0x2fea58 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_")]
pub fn stub_0x2fea58() -> ! {
    todo!("0x2fea58 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")
}

// 0x2feaa8 — __ZN3RBX14AsyncHttpQueue15registerContentERKSsN5boost10shared_ptrIS1_EES5_
#[doc(alias = "RBX::AsyncHttpQueue::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBX14AsyncHttpQueue15registerContentERKSsN5boost10shared_ptrIS1_EES5_")]
pub fn stub_0x2feaa8() -> ! {
    todo!("0x2feaa8 RBX::AsyncHttpQueue::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")
}

// 0x2feab0 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x2feab0() -> ! {
    todo!("0x2feab0 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<...")
}

// 0x2fee5c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm")]
pub fn stub_0x2fee5c() -> ! {
    todo!("0x2fee5c std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")
}

// 0x2fee80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_
#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)> const&)")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_")]
pub fn stub_0x2fee80() -> ! {
    todo!("0x2fee80 boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *...")
}

// 0x2fef44 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_")]
pub fn stub_0x2fef44() -> ! {
    todo!("0x2fef44 boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::sh...")
}

// 0x2ff020 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>&)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_")]
pub fn stub_0x2ff020() -> ! {
    todo!("0x2ff020 boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,bo...")
}

// 0x2ff128 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_")]
pub fn stub_0x2ff128() -> ! {
    todo!("0x2ff128 RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::Asyn...")
}

// 0x2ff188 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
#[doc(alias = "__ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_")]
pub fn stub_0x2ff188() -> ! {
    todo!("0x2ff188 std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")
}

// 0x2ff2d4 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_")]
pub fn stub_0x2ff2d4() -> ! {
    todo!("0x2ff2d4 std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::C...")
}

// 0x2ff43c — __ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_")]
pub fn stub_0x2ff43c() -> ! {
    todo!("0x2ff43c std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")
}

// 0x2ff470 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEclIPFvNS9_15CallbackWrapperES7_SA_SD_ENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string>) &,boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEclIPFvNS9_15CallbackWrapperES7_SA_SD_ENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x2ff470() -> ! {
    todo!("0x2ff470 void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::operator()<void (*)...")
}

// 0x2ff588 — __ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEC2ES3_S8_SB_SE_
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS_3argILi1EEENS0_5valueIPN3RBX8InstanceEEENS4_INS5_14AsyncHttpQueue13RequestResultEEENS4_INS_10shared_ptrISsEEEEEC2ES3_S8_SB_SE_")]
pub fn stub_0x2ff588() -> ! {
    todo!("0x2ff588 boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string>>>::list4(boost::arg<1>,boos...")
}

// 0x2ff674 — __ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
#[doc(alias = "__ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E")]
pub fn stub_0x2ff674() -> ! {
    todo!("0x2ff674 std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")
}

// 0x2ff758 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_")]
pub fn stub_0x2ff758() -> ! {
    todo!("0x2ff758 RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::...")
}

// 0x2ff8c0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_")]
pub fn stub_0x2ff8c0() -> ! {
    todo!("0x2ff8c0 RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::C...")
}

// 0x2ff91c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_
#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_")]
pub fn stub_0x2ff91c() -> ! {
    todo!("0x2ff91c RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue...")
}

// 0x2ff978 — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev
#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev")]
pub fn stub_0x2ff978() {
    // IDA 0x2ff978: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x2ffa44 — __ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_
#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
#[doc(alias = "__ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_")]
pub fn stub_0x2ffa44() -> ! {
    todo!("0x2ffa44 std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")
}

// 0x2ffb24 — __ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_
#[doc(alias = "boost::shared_ptr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4HttpEEC2IS2_EEPT_")]
pub fn stub_0x2ffb24() -> ! {
    todo!("0x2ffb24 boost::shared_ptr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")
}

// 0x2ffbfc — __ZN5boost6detail12shared_countC2IN3RBX4HttpEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX4HttpEEEPT_")]
pub fn stub_0x2ffbfc() -> ! {
    todo!("0x2ffbfc boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *)")
}

// 0x2ffd34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED1Ev")]
pub fn stub_0x2ffd34() {
    // IDA 0x2ffd34: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x2ffd38 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEED0Ev")]
pub fn stub_0x2ffd38() {
    // IDA 0x2ffd38: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2ffd3c — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE7disposeEv")]
pub fn stub_0x2ffd3c() -> ! {
    todo!("0x2ffd3c boost::detail::sp_counted_impl_p<RBX::Http>::dispose(void)")
}

// 0x2ffe10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE11get_deleterERKSt9type_info")]
pub fn stub_0x2ffe10() -> ! {
    todo!("0x2ffe10 boost::detail::sp_counted_impl_p<RBX::Http>::get_deleter(std::type_info const&)")
}

// 0x2ffe14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX4HttpEE19get_untyped_deleterEv")]
pub fn stub_0x2ffe14() -> ! {
    todo!("0x2ffe14 boost::detail::sp_counted_impl_p<RBX::Http>::get_untyped_deleter(void)")
}

// 0x2ffe1c — __ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "boost::shared_ptr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(boost::weak_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0x2ffe1c() -> ! {
    todo!("0x2ffe1c boost::shared_ptr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(boost::weak_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")
}

// 0x2ffe98 — __ZN5boost10shared_ptrISsEC2ISsEEPT_
#[doc(alias = "boost::shared_ptr<std::string>::shared_ptr<std::string>(std::string *)")]
#[doc(alias = "__ZN5boost10shared_ptrISsEC2ISsEEPT_")]
pub fn stub_0x2ffe98() -> ! {
    todo!("0x2ffe98 boost::shared_ptr<std::string>::shared_ptr<std::string>(std::string *)")
}

// 0x2fff70 — __ZN5boost6detail17sp_counted_impl_pISsED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISsED0Ev")]
pub fn stub_0x2fff70() {
    // IDA 0x2fff70: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x2fff78 — __ZN5boost6detail17sp_counted_impl_pISsE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISsE11get_deleterERKSt9type_info")]
pub fn stub_0x2fff78() -> ! {
    todo!("0x2fff78 boost::detail::sp_counted_impl_p<std::string>::get_deleter(std::type_info const&)")
}

// 0x2fff80 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS7_5list3INS7_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS7_5list3INS7_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2fff80() -> ! {
    todo!("0x2fff80 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS7_5list3INS7_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS...")
}

// 0x3000d8 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3000d8() -> ! {
    todo!("0x3000d8 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEE...")
}

// 0x300230 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEEvT_")]
pub fn stub_0x300230() -> ! {
    todo!("0x300230 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string cons...")
}

// 0x30039c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x30039c() -> ! {
    todo!("0x30039c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::Async...")
}

// 0x3003b8 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESP_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESP_")]
pub fn stub_0x3003b8() -> ! {
    todo!("0x3003b8 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>...")
}

// 0x3003d4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x3003d4() -> ! {
    todo!("0x3003d4 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::share...")
}

// 0x300530 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x300530() -> ! {
    todo!("0x300530 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::share...")
}

// 0x300688 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_SG_ENS8_5list3INS8_5valueISI_EENSM_ISC_EENSM_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x300688() -> ! {
    todo!("0x300688 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::...")
}

// 0x300798 — __ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEclIPFvSC_S6_SA_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEclIPFvSC_S6_SA_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x300798() -> ! {
    todo!("0x300798 void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::Req...")
}

// 0x3008a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x3008a4() -> ! {
    todo!("0x3008a4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,RBX::Async...")
}

// 0x300a60 — __ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")]
pub fn stub_0x300a60() -> ! {
    todo!("0x300a60 boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestR...")
}

// 0x300b6c — __ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")]
pub fn stub_0x300b6c() -> ! {
    todo!("0x300b6c boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::Reque...")
}

// 0x300c6c — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_")]
pub fn stub_0x300c6c() -> ! {
    todo!("0x300c6c boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::Reque...")
}

// 0x300d3c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x300d3c() -> ! {
    todo!("0x300d3c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1...")
}

// 0x300e68 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x300e68() -> ! {
    todo!("0x300e68 __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1E...")
}

// 0x300f98 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_0x300f98() -> ! {
    todo!("0x300f98 void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::s...")
}

// 0x3010d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
pub fn stub_0x3010d8() -> ! {
    todo!("0x3010d8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost...")
}

// 0x3010f4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::mutex>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_")]
pub fn stub_0x3010f4() -> ! {
    todo!("0x3010f4 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mu...")
}

// 0x30110c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0x30110c() -> ! {
    todo!("0x30110c bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQu...")
}

// 0x301238 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0x301238() -> ! {
    todo!("0x301238 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQu...")
}

// 0x301360 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x301360() -> ! {
    todo!("0x301360 void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncH...")
}

// 0x301478 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list1<boost::shared_ptr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0x301478() -> ! {
    todo!("0x301478 void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(boost::weak_p...")
}

// 0x3015d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0x3015d8() -> ! {
    todo!("0x3015d8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost...")
}

// 0x301770 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")]
pub fn stub_0x301770() -> ! {
    todo!("0x301770 boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_pt...")
}

// 0x30188c — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")]
pub fn stub_0x30188c() -> ! {
    todo!("0x30188c boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<boost::w...")
}

// 0x3019a8 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_")]
pub fn stub_0x3019a8() -> ! {
    todo!("0x3019a8 boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::A...")
}

// 0x301afc — __ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "boost::weak_ptr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(boost::shared_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
pub fn stub_0x301afc() -> ! {
    todo!("0x301afc boost::weak_ptr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(boost::shared_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::t...")
}

// 0x301b4c — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv")]
pub fn stub_0x301b4c() -> ! {
    todo!("0x301b4c std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")
}

// 0x301b80 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm")]
pub fn stub_0x301b80() -> ! {
    todo!("0x301b80 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")
}

// 0x301b98 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv")]
pub fn stub_0x301b98() -> ! {
    todo!("0x301b98 std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")
}

// 0x301f74 — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
#[doc(alias = "__ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_")]
pub fn stub_0x301f74() -> ! {
    todo!("0x301f74 std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRe...")
}

// 0x302028 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev")]
pub fn stub_0x302028() {
    // IDA 0x302028: D2 base-object destructor: destroy members in place, no delete (decompiled 0x111270 PluginList map-node loop, 0x35bfec NameMap, 0xdc29cc Ogre::SceneNode; 0x4a15b0 EnumDesc). Rust: Drop glue covers it; no explicit body.
}

// 0x302054 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm")]
pub fn stub_0x302054() -> ! {
    todo!("0x302054 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")
}

// 0x3021d4 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
#[doc(alias = "__ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_")]
pub fn stub_0x3021d4() -> ! {
    todo!("0x3021d4 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")
}

// 0x3022c8 — __ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_
#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")]
#[doc(alias = "__ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_")]
pub fn stub_0x3022c8() -> ! {
    todo!("0x3022c8 boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")
}

// 0x3022f8 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv")]
pub fn stub_0x3022f8() -> ! {
    todo!("0x3022f8 boost::function1<void,boost::shared_ptr<RBX::mutex>>::clear(void)")
}

// 0x302324 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18HttpQueueStatsItemEPNS_14AsyncHttpQueueEPS1_EEN5boost10shared_ptrIT_EET0_T1_
#[doc(alias = "boost::shared_ptr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_18HttpQueueStatsItemEPNS_14AsyncHttpQueueEPS1_EEN5boost10shared_ptrIT_EET0_T1_")]
pub fn stub_0x302324() -> ! {
    todo!("0x302324 boost::shared_ptr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")
}

// 0x3023dc — __ZN3RBX18HttpQueueStatsItem4initEv
#[doc(alias = "RBX::HttpQueueStatsItem::init(void)")]
#[doc(alias = "__ZN3RBX18HttpQueueStatsItem4initEv")]
pub fn stub_0x3023dc() -> ! {
    todo!("0x3023dc RBX::HttpQueueStatsItem::init(void)")
}

// 0x302418 — __ZN3RBX18HttpQueueStatsItemC2EPNS_14AsyncHttpQueueEPNS_8InstanceE
#[doc(alias = "RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")]
#[doc(alias = "__ZN3RBX18HttpQueueStatsItemC2EPNS_14AsyncHttpQueueEPNS_8InstanceE")]
pub fn stub_0x302418() -> ! {
    todo!("0x302418 RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")
}

// 0x30266c — __ZN3RBX18HttpQueueStatsItemD1Ev
#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
#[doc(alias = "__ZN3RBX18HttpQueueStatsItemD1Ev")]
pub fn stub_0x30266c() {
    // IDA 0x30266c: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x3026a8 — __ZN3RBX18HttpQueueStatsItemD0Ev
#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
#[doc(alias = "__ZN3RBX18HttpQueueStatsItemD0Ev")]
pub fn stub_0x3026a8() {
    // IDA 0x3026a8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x30277c — __ZN3RBX18HttpQueueStatsItem6updateEv
#[doc(alias = "RBX::HttpQueueStatsItem::update(void)")]
#[doc(alias = "__ZN3RBX18HttpQueueStatsItem6updateEv")]
pub fn stub_0x30277c() -> ! {
    todo!("0x30277c RBX::HttpQueueStatsItem::update(void)")
}

// 0x3027d0 — __ZThn32_N3RBX18HttpQueueStatsItemD1Ev
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
#[doc(alias = "__ZThn32_N3RBX18HttpQueueStatsItemD1Ev")]
pub fn stub_0x3027d0() {
    // IDA 0x3027d0: __ZThn32 thunk (D1 base dtor): `this -= 32`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x302810 — __ZThn32_N3RBX18HttpQueueStatsItemD0Ev
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
#[doc(alias = "__ZThn32_N3RBX18HttpQueueStatsItemD0Ev")]
pub fn stub_0x302810() {
    // IDA 0x302810: __ZThn32 thunk (D0 deleting dtor): `this -= 32`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3028e8 — __ZThn36_N3RBX18HttpQueueStatsItemD1Ev
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
#[doc(alias = "__ZThn36_N3RBX18HttpQueueStatsItemD1Ev")]
pub fn stub_0x3028e8() {
    // IDA 0x3028e8: __ZThn36 thunk (D1 base dtor): `this -= 36`, run base-object dtor in place (cf. decompiled 0x6d2e50). Rust: Drop glue covers it; no explicit body.
}

// 0x302928 — __ZThn36_N3RBX18HttpQueueStatsItemD0Ev
#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
#[doc(alias = "__ZThn36_N3RBX18HttpQueueStatsItemD0Ev")]
pub fn stub_0x302928() {
    // IDA 0x302928: __ZThn36 thunk (D0 deleting dtor): `this -= 36`, run complete-object dtor, `operator delete` (cf. decompiled 0xfb5c). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x3029fc — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x3029fc() -> ! {
    todo!("0x3029fc boost::shared_ptr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x302bac — __ZN5boost6detail12shared_countC2IPN3RBX18HttpQueueStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX18HttpQueueStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x302bac() -> ! {
    todo!("0x302bac boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x302cb4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x302cb4() {
    // IDA 0x302cb4: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0x302cb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x302cb8() {
    // IDA 0x302cb8: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0x302cbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x302cbc() -> ! {
    todo!("0x302cbc boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x302cdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x302cdc() -> ! {
    todo!("0x302cdc boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x302cf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x302cf4() -> ! {
    todo!("0x302cf4 boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x302cf8 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv")]
pub fn stub_0x302cf8() -> ! {
    todo!("0x302cf8 std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")
}

// 0x302d20 — __GLOBAL__I_a_106
#[doc(alias = "global constructor keyed to_a_106")]
#[doc(alias = "__GLOBAL__I_a_106")]
pub fn stub_0x302d20() -> ! {
    todo!("0x302d20 global constructor keyed to_a_106")
}

// 0x302eb8 — __ZN3RBX4AxesC1Ei
#[doc(alias = "RBX::Axes::Axes(int)")]
#[doc(alias = "__ZN3RBX4AxesC1Ei")]
pub fn stub_0x302eb8() -> ! {
    todo!("0x302eb8 RBX::Axes::Axes(int)")
}

// 0x302ebc — __ZN3RBX4Axes14normalIdToAxisENS_8NormalIdE
#[doc(alias = "RBX::Axes::normalIdToAxis(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX4Axes14normalIdToAxisENS_8NormalIdE")]
pub fn stub_0x302ebc() -> ! {
    todo!("0x302ebc RBX::Axes::normalIdToAxis(RBX::NormalId)")
}

// 0x302ed8 — __ZN3RBX4Axes14axisToNormalIdEN3G3D7Vector34AxisE
#[doc(alias = "RBX::Axes::axisToNormalId(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX4Axes14axisToNormalIdEN3G3D7Vector34AxisE")]
pub fn stub_0x302ed8() -> ! {
    todo!("0x302ed8 RBX::Axes::axisToNormalId(G3D::Vector3::Axis)")
}

// 0x302ee0 — __ZN3RBX4Axes10axisToMaskEN3G3D7Vector34AxisE
#[doc(alias = "RBX::Axes::axisToMask(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX4Axes10axisToMaskEN3G3D7Vector34AxisE")]
pub fn stub_0x302ee0() -> ! {
    todo!("0x302ee0 RBX::Axes::axisToMask(G3D::Vector3::Axis)")
}

// 0x302ef0 — __ZNK3RBX4Axes17getAxisByNormalIdENS_8NormalIdE
#[doc(alias = "RBX::Axes::getAxisByNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX4Axes17getAxisByNormalIdENS_8NormalIdE")]
pub fn stub_0x302ef0() -> ! {
    todo!("0x302ef0 RBX::Axes::getAxisByNormalId(RBX::NormalId)const")
}

// 0x302f1c — __ZNK3RBX4Axes7getAxisEN3G3D7Vector34AxisE
#[doc(alias = "RBX::Axes::getAxis(G3D::Vector3::Axis)const")]
#[doc(alias = "__ZNK3RBX4Axes7getAxisEN3G3D7Vector34AxisE")]
pub fn stub_0x302f1c() -> ! {
    todo!("0x302f1c RBX::Axes::getAxis(G3D::Vector3::Axis)const")
}

// 0x302f30 — __ZN3RBX15StringConverterINS_4AxesEE15convertToStringERKS1_
#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_4AxesEE15convertToStringERKS1_")]
pub fn stub_0x302f30() -> ! {
    todo!("0x302f30 RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")
}

// 0x303304 — __ZN3RBX15StringConverterIN3G3D7Vector34AxisEE14convertToValueERKSsRS3_
#[doc(alias = "RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&,G3D::Vector3::Axis&)")]
#[doc(alias = "__ZN3RBX15StringConverterIN3G3D7Vector34AxisEE14convertToValueERKSsRS3_")]
pub fn stub_0x303304() -> ! {
    todo!("0x303304 RBX::StringConverter<G3D::Vector3::Axis>::convertToValue(std::string const&,G3D::Vector3::Axis&)")
}

// 0x303418 — __ZN3RBX15StringConverterINS_4AxesEE14convertToValueERKSsRS1_
#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_4AxesEE14convertToValueERKSsRS1_")]
pub fn stub_0x303418() -> ! {
    todo!("0x303418 RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")
}

// 0x303bc8 — __ZN3rbx8any_castIN3G3D7Vector34AxisEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3::Axis * rbx::any_cast<G3D::Vector3::Axis,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3G3D7Vector34AxisEN3RBX7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0x303bc8() -> ! {
    todo!("0x303bc8 G3D::Vector3::Axis * rbx::any_cast<G3D::Vector3::Axis,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")
}

// 0x303c20 — __ZN3rbx8any_castIRN3G3D7Vector34AxisEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3::Axis & rbx::any_cast<G3D::Vector3::Axis &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3G3D7Vector34AxisEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x303c20() -> ! {
    todo!("0x303c20 G3D::Vector3::Axis & rbx::any_cast<G3D::Vector3::Axis &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x303d10 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::resize(unsigned long,G3D::Vector3::Axis)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE6resizeEmS2_")]
pub fn stub_0x303d10() -> ! {
    todo!("0x303d10 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::resize(unsigned long,G3D::Vector3::Axis)")
}

// 0x303d44 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::push_back(G3D::Vector3::Axis const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE9push_backERKS2_")]
pub fn stub_0x303d44() -> ! {
    todo!("0x303d44 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::push_back(G3D::Vector3::Axis const&)")
}

// 0x303d6c — __ZNSt3mapIPKN3RBX4NameEN3G3D7Vector34AxisESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
#[doc(alias = "std::map<RBX::Name const*,G3D::Vector3::Axis,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameEN3G3D7Vector34AxisESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")]
pub fn stub_0x303d6c() -> ! {
    todo!("0x303d6c std::map<RBX::Name const*,G3D::Vector3::Axis,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::operator[](RBX::Name const* const&)")
}

// 0x303dc4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_0x303dc4() -> ! {
    todo!("0x303dc4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<...")
}

// 0x303e78 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
pub fn stub_0x303e78() -> ! {
    todo!("0x303e78 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<...")
}

// 0x303ed0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_insert_unique(std::pair<RBX::Name const* const,G3D::Vector3::Axis> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_0x303ed0() -> ! {
    todo!("0x303ed0 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<...")
}

// 0x303f38 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,G3D::Vector3::Axis const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x303f38() -> ! {
    todo!("0x303f38 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>...")
}

// 0x30401c — __ZNSt12_Vector_baseIN3G3D7Vector34AxisESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3G3D7Vector34AxisESaIS2_EE11_M_allocateEm")]
pub fn stub_0x30401c() -> ! {
    todo!("0x30401c std::_Vector_base<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_allocate(unsigned long)")
}

// 0x304034 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector34AxisES6_EET0_T_S8_S7_
#[doc(alias = "G3D::Vector3::Axis * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3::Axis *,G3D::Vector3::Axis *>(G3D::Vector3::Axis *,G3D::Vector3::Axis *,G3D::Vector3::Axis *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector34AxisES6_EET0_T_S8_S7_")]
pub fn stub_0x304034() -> ! {
    todo!("0x304034 G3D::Vector3::Axis * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3::Axis *,G3D::Vector3::Axis *>(G3D::Vector3::Axis *,G3D::Vector3::Axis *,G3D::Vector3::Axis *)")
}

// 0x304070 — __ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>>,unsigned long,G3D::Vector3::Axis const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D7Vector34AxisESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x304070() -> ! {
    todo!("0x304070 std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3::Axis*,std::vector<G3D::Vector3::Axis,std::allocator<G3D::Vector3::Axis>...")
}

// 0x304200 — __GLOBAL__I_a_107
#[doc(alias = "global constructor keyed to_a_107")]
#[doc(alias = "__GLOBAL__I_a_107")]
pub fn stub_0x304200() -> ! {
    todo!("0x304200 global constructor keyed to_a_107")
}

// 0x3042c8 — __ZN3RBX10BrickColor8BrickMap9singletonEv
#[doc(alias = "RBX::BrickColor::BrickMap::singleton(void)")]
#[doc(alias = "__ZN3RBX10BrickColor8BrickMap9singletonEv")]
pub fn stub_0x3042c8() -> ! {
    todo!("0x3042c8 RBX::BrickColor::BrickMap::singleton(void)")
}

// 0x3043c4 — __ZN3RBX10BrickColor12colorPaletteEv
#[doc(alias = "RBX::BrickColor::colorPalette(void)")]
#[doc(alias = "__ZN3RBX10BrickColor12colorPaletteEv")]
pub fn stub_0x3043c4() -> ! {
    todo!("0x3043c4 RBX::BrickColor::colorPalette(void)")
}

// 0x3043dc — __ZNK3RBX10BrickColor22getClosestPaletteIndexEv
#[doc(alias = "RBX::BrickColor::getClosestPaletteIndex(void)const")]
#[doc(alias = "__ZNK3RBX10BrickColor22getClosestPaletteIndexEv")]
pub fn stub_0x3043dc() -> ! {
    todo!("0x3043dc RBX::BrickColor::getClosestPaletteIndex(void)const")
}

// 0x3043fc — __ZN3RBX10BrickColor5parseEPKc
#[doc(alias = "RBX::BrickColor::parse(char const*)")]
#[doc(alias = "__ZN3RBX10BrickColor5parseEPKc")]
pub fn stub_0x3043fc() -> ! {
    todo!("0x3043fc RBX::BrickColor::parse(char const*)")
}

// 0x304468 — __ZN3RBX10BrickColor6randomEv
#[doc(alias = "RBX::BrickColor::random(void)")]
#[doc(alias = "__ZN3RBX10BrickColor6randomEv")]
pub fn stub_0x304468() -> ! {
    todo!("0x304468 RBX::BrickColor::random(void)")
}

// 0x3044a0 — __ZN3RBX10BrickColor7closestEN3G3D6Color3E
#[doc(alias = "RBX::BrickColor::closest(G3D::Color3)")]
#[doc(alias = "__ZN3RBX10BrickColor7closestEN3G3D6Color3E")]
pub fn stub_0x3044a0() -> ! {
    todo!("0x3044a0 RBX::BrickColor::closest(G3D::Color3)")
}

// 0x3044c4 — __ZN3RBX10BrickColor7closestEN3G3D6Color4E
#[doc(alias = "RBX::BrickColor::closest(G3D::Color4)")]
#[doc(alias = "__ZN3RBX10BrickColor7closestEN3G3D6Color4E")]
pub fn stub_0x3044c4() -> ! {
    todo!("0x3044c4 RBX::BrickColor::closest(G3D::Color4)")
}

