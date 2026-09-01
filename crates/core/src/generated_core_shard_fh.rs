//! core shard FH — 100 core stubs EA-sorted, lowest uncovered 0xf2e3e4..0xf2f714 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FG 0xf2dad4).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf2dad4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.


#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::detail::shared_count::shared_count<boost::filesystem::detail::dir_itr_imp>(boost::filesystem::detail::dir_itr_imp *)")]
// 0xf2e3e4 — j___ZN5boost6detail12shared_countC2INS_10filesystem6detail11dir_itr_impEEEPT_
pub fn stub_f2e3e4() -> ! {
    todo!("0xf2e3e4 j___ZN5boost6detail12shared_countC2INS_10filesystem6detail11dir_itr_impEEEPT_")
}

#[doc(alias = "RBX::Http::MutexGuard::MutexGuard(void)")]
// 0xf2e3f4 — j___ZN3RBX4Http10MutexGuardC2Ev
pub fn stub_f2e3f4() -> ! {
    todo!("0xf2e3f4 j___ZN3RBX4Http10MutexGuardC2Ev")
}

#[doc(alias = "RBX::Http::MutexGuard::~MutexGuard()")]
// 0xf2e404 — j___ZN3RBX4Http10MutexGuardD2Ev
pub fn stub_f2e404() -> ! {
    todo!("0xf2e404 j___ZN3RBX4Http10MutexGuardD2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0xf2e414 — j___ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
// was: boost::shared_ptr<std::istream>::shared_ptr<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)
pub fn stub_f2e414() -> ! {
    todo!("0xf2e414 j___ZN5boost10shared_ptrISiEC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_")
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy(void)")]
// 0xf2e424 — j___ZN5boost15circular_bufferIdSaIdEE7destroyEv
pub fn stub_f2e424() -> ! {
    todo!("0xf2e424 j___ZN5boost15circular_bufferIdSaIdEE7destroyEv")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// 0xf2e434 — j___ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
pub fn stub_f2e434() -> ! {
    todo!("0xf2e434 j___ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
// 0xf2e444 — j___ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<boost::shared_ptr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex> &> &,int)
pub fn stub_f2e444() -> ! {
    todo!("0xf2e444 j___ZN5boost3_bi5list3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// 0xf2e454 — j___ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
// was: boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_f2e454() -> ! {
    todo!("0xf2e454 j___ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
// 0xf2e464 — j___ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsS5_bbSD_ENS0_5list1IRNS4_IN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<boost::shared_ptr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex> &> &,int)
pub fn stub_f2e464() -> ! {
    todo!("0xf2e464 j___ZN5boost3_bi5list5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsS5_bbSD_ENS0_5list1IRNS4_IN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// 0xf2e474 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
pub fn stub_f2e474() -> ! {
    todo!("0xf2e474 j___ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex> &> &,int)")]
// 0xf2e484 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::operator()<void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list1<boost::shared_ptr<RBX::mutex> &>>(boost::_bi::type<void>,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex> &> &,int)
pub fn stub_f2e484() -> ! {
    todo!("0xf2e484 j___ZN5boost3_bi5list5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEclIPFvSsSsbbSA_ENS0_5list1IRNS_10shared_ptrIN3RBX5mutexEEEEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>)")]
// 0xf2e494 — j___ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_10shared_ptrISiEEEEEC2ES3_S6_
// was: boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>)
pub fn stub_f2e494() -> ! {
    todo!("0xf2e494 j___ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_10shared_ptrISiEEEEEC2ES3_S6_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
// 0xf2e4a4 — j___ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_
pub fn stub_f2e4a4() -> ! {
    todo!("0xf2e4a4 j___ZN5boost3_bi8storage2INS0_5valueISsEENS2_IbEEEC2ES3_S4_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>)")]
// 0xf2e4b4 — j___ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEEEC2ES3_S6_S7_
// was: boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>)
pub fn stub_f2e4b4() -> ! {
    todo!("0xf2e4b4 j___ZN5boost3_bi8storage3INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEEEC2ES3_S6_S7_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// 0xf2e4c4 — j___ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_
pub fn stub_f2e4c4() -> ! {
    todo!("0xf2e4c4 j___ZN5boost3_bi8storage3INS0_5valueISsEENS2_IbEENS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S4_SB_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
// 0xf2e4d4 — j___ZN5boost3_bi8storage3INS0_5valueISsEES3_NS2_IbEEEC2ES3_S3_S4_
pub fn stub_f2e4d4() -> ! {
    todo!("0xf2e4d4 j___ZN5boost3_bi8storage3INS0_5valueISsEES3_NS2_IbEEEC2ES3_S3_S4_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
// 0xf2e4e4 — j___ZN5boost3_bi8storage4INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_EC2ES3_S6_S7_S7_
// was: boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>)
pub fn stub_f2e4e4() -> ! {
    todo!("0xf2e4e4 j___ZN5boost3_bi8storage4INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_EC2ES3_S6_S7_S7_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>)")]
// 0xf2e4f4 — j___ZN5boost3_bi8storage4INS0_5valueISsEES3_NS2_IbEES4_EC2ES3_S3_S4_S4_
pub fn stub_f2e4f4() -> ! {
    todo!("0xf2e4f4 j___ZN5boost3_bi8storage4INS0_5valueISsEES3_NS2_IbEES4_EC2ES3_S3_S4_S4_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// 0xf2e504 — j___ZN5boost3_bi8storage5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_
// was: boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)
pub fn stub_f2e504() -> ! {
    todo!("0xf2e504 j___ZN5boost3_bi8storage5INS0_5valueISsEENS2_INS_10shared_ptrISiEEEENS2_IbEES7_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S6_S7_S7_SE_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>)")]
// 0xf2e514 — j___ZN5boost3_bi8storage5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_
pub fn stub_f2e514() -> ! {
    todo!("0xf2e514 j___ZN5boost3_bi8storage5INS0_5valueISsEES3_NS2_IbEES4_NS2_INS_8functionIFvPSsPSt9exceptionEEEEEEC2ES3_S3_S4_S4_SB_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
// 0xf2e524 — j___ZN5boost4bindIvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEESsS2_bbS8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// was: boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>)
pub fn stub_f2e524() -> ! {
    todo!("0xf2e524 j___ZN5boost4bindIvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEESsS2_bbS8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_5<std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>)")]
// 0xf2e534 — j___ZN5boost4bindIvSsSsbbNS_8functionIFvPSsPSt9exceptionEEESsSsbbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_T3_T4_ENS7_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESG_SI_SJ_SK_SL_SM_
pub fn stub_f2e534() -> ! {
    todo!("0xf2e534 j___ZN5boost4bindIvSsSsbbNS_8functionIFvPSsPSt9exceptionEEESsSsbbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_T3_T4_ENS7_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESG_SI_SJ_SK_SL_SM_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list_av_3<std::string,bool,boost::function<void ()(std::string *,std::exception *)>>::type> boost::bind<void,std::string,bool,boost::function<void ()(std::string *,std::exception *)>,std::string,bool,boost::function<void ()(std::string *,std::exception *)>>(void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),std::string,bool,boost::function<void ()(std::string *,std::exception *)>)")]
// 0xf2e544 — j___ZN5boost4bindIvSsbNS_8functionIFvPSsPSt9exceptionEEESsbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
pub fn stub_f2e544() -> ! {
    todo!("0xf2e544 j___ZN5boost4bindIvSsbNS_8functionIFvPSsPSt9exceptionEEESsbS6_EENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0xf2e554 — j___ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_
pub fn stub_f2e554() -> ! {
    todo!("0xf2e554 j___ZN5boost6detail12shared_countC2ISt19basic_istringstreamIcSt11char_traitsIcESaIcEEEEPT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf2e564 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f2e564() -> ! {
    todo!("0xf2e564 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_10shared_ptrISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEENSG_IS6_EENSG_IbEESJ_NSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf2e574 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_f2e574() -> ! {
    todo!("0xf2e574 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list5INS3_5valueISsEESF_NSE_IbEESG_NSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf2e584 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_f2e584() -> ! {
    todo!("0xf2e584 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS3_5list3INS3_5valueISsEENSE_IbEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
// 0xf2e5c4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_f2e5c4() -> ! {
    todo!("0xf2e5c4 j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS1_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEENSJ_IS9_EENSJ_IbEESM_NSJ_ISF_EEEEEEEEvT_")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
// 0xf2e5d4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_f2e5d4() -> ! {
    todo!("0xf2e5d4 j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list5INS7_5valueISsEESJ_NSI_IbEESK_NSI_ISE_EEEEEEEEvT_")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)")]
// 0xf2e5e4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEEvT_
// was: void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>)
pub fn stub_f2e5e4() -> ! {
    todo!("0xf2e5e4 j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS7_5list3INS7_5valueISsEENSI_IbEENSI_ISE_EEEEEEEEvT_")
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::assign_to_own(boost::function2<void,std::string *,std::exception *> const&)")]
// 0xf2e624 — j___ZN5boost9function2IvPSsPSt9exceptionE13assign_to_ownERKS4_
pub fn stub_f2e624() -> ! {
    todo!("0xf2e624 j___ZN5boost9function2IvPSsPSt9exceptionE13assign_to_ownERKS4_")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf2e634 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f2e634() -> ! {
    todo!("0xf2e634 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf2e644 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f2e644() -> ! {
    todo!("0xf2e644 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf2e654 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f2e654() -> ! {
    todo!("0xf2e654 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf2e664 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f2e664() -> ! {
    todo!("0xf2e664 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,rbx_core::SharedPtr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::SharedPtr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf2e674 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,boost::shared_ptr<std::istream>,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<boost::shared_ptr<std::istream>>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f2e674() -> ! {
    todo!("0xf2e674 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsNS3_ISiEEbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEENSL_ISB_EENSL_IbEESO_NSL_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf2e684 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f2e684() -> ! {
    todo!("0xf2e684 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf2e694 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,bool,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f2e694() -> ! {
    todo!("0xf2e694 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsSsbbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list5INS9_5valueISsEESL_NSK_IbEESM_NSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const")]
// 0xf2e6a4 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f2e6a4() -> ! {
    todo!("0xf2e6a4 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf2e6b4 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>>(boost::_bi::bind_t<void,void (*)(std::string,bool,boost::function<void ()(std::string *,std::exception *)>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<bool>,boost::_bi::value<boost::function<void ()(std::string *,std::exception *)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f2e6b4() -> ! {
    todo!("0xf2e6b4 j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvSsbNS_8functionIFvPSsPSt9exceptionEEEENS9_5list3INS9_5valueISsEENSK_IbEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::operator()(std::string *,std::exception *)const")]
// 0xf2e6c4 — j___ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_
pub fn stub_f2e6c4() -> ! {
    todo!("0xf2e6c4 j___ZNK5boost9function2IvPSsPSt9exceptionEclES1_S3_")
}

#[doc(alias = "RBX::Face::getAxis(int)const")]
// 0xf2e6e4 — j___ZNK3RBX4Face7getAxisEi
pub fn stub_f2e6e4() -> ! {
    todo!("0xf2e6e4 j___ZNK3RBX4Face7getAxisEi")
}

#[doc(alias = "RBX::IndexedMesh::lowersChanged(void)")]
// 0xf2e714 — j___ZN3RBX11IndexedMesh13lowersChangedEv
pub fn stub_f2e714() -> ! {
    todo!("0xf2e714 j___ZN3RBX11IndexedMesh13lowersChangedEv")
}

#[doc(alias = "RBX::IndexedMesh * RBX::IndexedTree::getTypedChild<RBX::IndexedMesh>(int)")]
// 0xf2e724 — j___ZN3RBX11IndexedTree13getTypedChildINS_11IndexedMeshEEEPT_i
pub fn stub_f2e724() -> ! {
    todo!("0xf2e724 j___ZN3RBX11IndexedTree13getTypedChildINS_11IndexedMeshEEEPT_i")
}

#[doc(alias = "RBX::IndexArray<RBX::IndexedTree,&RBX::IndexedTree::getIndex>::fastRemove(RBX::IndexedTree*)")]
// 0xf2e784 — j___ZN3RBX10IndexArrayINS_11IndexedTreeEXadL_ZNS1_8getIndexEvEEE10fastRemoveEPS1_
pub fn stub_f2e784() -> ! {
    todo!("0xf2e784 j___ZN3RBX10IndexArrayINS_11IndexedTreeEXadL_ZNS1_8getIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator-=(int)")]
// 0xf2e794 — j___ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi
pub fn stub_f2e794() -> ! {
    todo!("0xf2e794 j___ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEmIEi")
}

#[doc(alias = "boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::operator+=(int)")]
// 0xf2e7a4 — j___ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi
pub fn stub_f2e7a4() -> ! {
    todo!("0xf2e7a4 j___ZN5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEEpLEi")
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::set_capacity(unsigned long)")]
// 0xf2e7b4 — j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm
pub fn stub_f2e7b4() -> ! {
    todo!("0xf2e7b4 j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE12set_capacityEm")
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
// 0xf2e7c4 — j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_
pub fn stub_f2e7c4() -> ! {
    todo!("0xf2e7c4 j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorIS5_NS6_15nonconst_traitsIS4_EEEESA_")
}

#[doc(alias = "boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::push_back(RBX::InterpolatedCFrame::FrameInfo const&)")]
// 0xf2e7d4 — j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE9push_backERKS3_
pub fn stub_f2e7d4() -> ! {
    todo!("0xf2e7d4 j___ZN5boost15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_low_capacity(unsigned long)")]
// 0xf2e7e4 — j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm
pub fn stub_f2e7e4() -> ! {
    todo!("0xf2e7e4 j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE18check_low_capacityEm")
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::check_high_capacity(void)")]
// 0xf2e7f4 — j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv
pub fn stub_f2e7f4() -> ! {
    todo!("0xf2e7f4 j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE19check_high_capacityEv")
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::clear(void)")]
// 0xf2e804 — j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv
pub fn stub_f2e804() -> ! {
    todo!("0xf2e804 j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5clearEv")
}

#[doc(alias = "boost::circular_buffer_space_optimized<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::erase(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>,boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>)")]
// 0xf2e814 — j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_
pub fn stub_f2e814() -> ! {
    todo!("0xf2e814 j___ZN5boost31circular_buffer_space_optimizedIN3RBX18InterpolatedCFrame9FrameInfoESaIS3_EE5eraseENS_10cb_details8iteratorINS_15circular_bufferIS3_S4_EENS6_15nonconst_traitsIS4_EEEESC_")
}

#[doc(alias = "boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>::linearize_pointer<boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>>(boost::cb_details::iterator<boost::circular_buffer<RBX::InterpolatedCFrame::FrameInfo,std::allocator<RBX::InterpolatedCFrame::FrameInfo>>,boost::cb_details::nonconst_traits<std::allocator<RBX::InterpolatedCFrame::FrameInfo>>::pointer> const&)const")]
// 0xf2e824 — j___ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE
pub fn stub_f2e824() -> ! {
    todo!("0xf2e824 j___ZNK5boost10cb_details8iteratorINS_15circular_bufferIN3RBX18InterpolatedCFrame9FrameInfoESaIS5_EEENS0_15nonconst_traitsIS6_EEE17linearize_pointerIS9_EENT_7pointerERKNS1_IS7_SC_EE")
}

#[doc(alias = "std::_Vector_base<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_allocate(unsigned long)")]
// 0xf2e844 — j___ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm
pub fn stub_f2e844() -> ! {
    todo!("0xf2e844 j___ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::KeywordFilterType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeywordFilterType *,RBX::KeywordFilterType *>(RBX::KeywordFilterType *,RBX::KeywordFilterType *,RBX::KeywordFilterType *)")]
// 0xf2e854 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_
pub fn stub_f2e854() -> ! {
    todo!("0xf2e854 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::KeywordFilterType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::operator[](RBX::Name const* const&)")]
// 0xf2e864 — j___ZNSt3mapIPKN3RBX4NameENS0_17KeywordFilterTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_f2e864() -> ! {
    todo!("0xf2e864 j___ZNSt3mapIPKN3RBX4NameENS0_17KeywordFilterTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,RBX::KeywordFilterType const&)")]
// 0xf2e874 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f2e874() -> ! {
    todo!("0xf2e874 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,unsigned long,RBX::KeywordFilterType const&)")]
// 0xf2e884 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_f2e884() -> ! {
    todo!("0xf2e884 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::resize(unsigned long,RBX::KeywordFilterType)")]
// 0xf2e894 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE6resizeEmS1_
pub fn stub_f2e894() -> ! {
    todo!("0xf2e894 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::push_back(RBX::KeywordFilterType const&)")]
// 0xf2e8a4 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE9push_backERKS1_
pub fn stub_f2e8a4() -> ! {
    todo!("0xf2e8a4 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
// 0xf2e8b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_f2e8b4() -> ! {
    todo!("0xf2e8b4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
// 0xf2e8c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_f2e8c4() -> ! {
    todo!("0xf2e8c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
// 0xf2e8d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_f2e8d4() -> ! {
    todo!("0xf2e8d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> const&)")]
// 0xf2e8e4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
pub fn stub_f2e8e4() -> ! {
    todo!("0xf2e8e4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::string>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf2e8f4 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_f2e8f4() -> ! {
    todo!("0xf2e8f4 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::string>>,std::string,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf2e904 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_f2e904() -> ! {
    todo!("0xf2e904 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSsEESsSsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "boost::function1<void,bool>::clear(void)")]
// 0xf2ef54 — j___ZN5boost9function1IvbE5clearEv
pub fn stub_f2ef54() -> ! {
    todo!("0xf2ef54 j___ZN5boost9function1IvbE5clearEv")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)")]
// 0xf2ef64 — j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::clear(void)
pub fn stub_f2ef64() -> ! {
    todo!("0xf2ef64 j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::expired(void)const")]
// 0xf2f324 — j___ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv
// was: boost::weak_ptr<RBX::AsyncHttpQueue>::expired(void)const
pub fn stub_f2f324() -> ! {
    todo!("0xf2f324 j___ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::MeshId>(RBX::MeshId const&)")]
// 0xf2f4d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6MeshIdEEERS3_RKT_
pub fn stub_f2f4d4() -> ! {
    todo!("0xf2f4d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6MeshIdEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::singleton(void)")]
// 0xf2f4e4 — j___ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE9singletonEv
pub fn stub_f2f4e4() -> ! {
    todo!("0xf2f4e4 j___ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE9singletonEv")
}

#[doc(alias = "RBX::MeshId * rbx::any_cast<RBX::MeshId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf2f4f4 — j___ZN3rbx8any_castIN3RBX6MeshIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f2f4f4() -> ! {
    todo!("0xf2f4f4 j___ZN3rbx8any_castIN3RBX6MeshIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::MeshId & rbx::any_cast<RBX::MeshId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf2f504 — j___ZN3rbx8any_castIRN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f2f504() -> ! {
    todo!("0xf2f504 j___ZN3rbx8any_castIRN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::queuing_rw_mutex::queuing_rw_mutex(void)")]
// 0xf2f514 — j___ZN3RBX16queuing_rw_mutexC2Ev
pub fn stub_f2f514() -> ! {
    todo!("0xf2f514 j___ZN3RBX16queuing_rw_mutexC2Ev")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0xf2f524 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_f2f524() -> ! {
    todo!("0xf2f524 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf2f534 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE
pub fn stub_f2f534() -> ! {
    todo!("0xf2f534 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0xf2f544 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_f2f544() -> ! {
    todo!("0xf2f544 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)")]
// 0xf2f554 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSG_9null_typeESI_SI_SI_SI_SI_SI_SI_SI_EENSH_ISI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EEEEEEvRKT_
pub fn stub_f2f554() -> ! {
    todo!("0xf2f554 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSG_9null_typeESI_SI_SI_SI_SI_SI_SI_SI_EENSH_ISI_SI_SI_SI_SI_SI_SI_SI_SI_SI_EEEEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct(void)")]
// 0xf2f564 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEE9constructEv
pub fn stub_f2f564() -> ! {
    todo!("0xf2f564 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::~node_constructor()")]
// 0xf2f574 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEED2Ev
pub fn stub_f2f574() -> ! {
    todo!("0xf2f574 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsPN3RBX4NameEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf2f584 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_f2f584() -> ! {
    todo!("0xf2f584 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf2f594 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_f2f594() -> ! {
    todo!("0xf2f594 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf2f5a4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_f2f5a4() -> ! {
    todo!("0xf2f5a4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf2f5b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_f2f5b4() -> ! {
    todo!("0xf2f5b4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> const&)")]
// 0xf2f5c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE
pub fn stub_f2f5c4() -> ! {
    todo!("0xf2f5c4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSC_RKSE_RKSaINS1_8ptr_nodeIS9_EEE")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf2f5d4 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_
pub fn stub_f2f5d4() -> ! {
    todo!("0xf2f5d4 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSE_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS9_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0xf2f5e4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_f2f5e4() -> ! {
    todo!("0xf2f5e4 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf2f5f4 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_f2f5f4() -> ! {
    todo!("0xf2f5f4 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsPN3RBX4NameEEESsS8_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "std::_Vector_base<RBX::Name *,std::allocator<RBX::Name *>>::_M_allocate(unsigned long)")]
// 0xf2f604 — j___ZNSt12_Vector_baseIPN3RBX4NameESaIS2_EE11_M_allocateEm
pub fn stub_f2f604() -> ! {
    todo!("0xf2f604 j___ZNSt12_Vector_baseIPN3RBX4NameESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
// 0xf2f614 — j___ZNSt6vectorIPN3RBX4NameESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f2f614() -> ! {
    todo!("0xf2f614 j___ZNSt6vectorIPN3RBX4NameESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::insert(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
// 0xf2f624 — j___ZNSt6vectorIPN3RBX4NameESaIS2_EE6insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f2f624() -> ! {
    todo!("0xf2f624 j___ZNSt6vectorIPN3RBX4NameESaIS2_EE6insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<int,std::allocator<int>>::_M_allocate(unsigned long)")]
// 0xf2f634 — j___ZNSt12_Vector_baseIiSaIiEE11_M_allocateEm
pub fn stub_f2f634() -> ! {
    todo!("0xf2f634 j___ZNSt12_Vector_baseIiSaIiEE11_M_allocateEm")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ProtectedString>(RBX::ProtectedString const&)")]
// 0xf2f664 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15ProtectedStringEEERS3_RKT_
pub fn stub_f2f664() -> ! {
    todo!("0xf2f664 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15ProtectedStringEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::singleton(void)")]
// 0xf2f674 — j___ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE9singletonEv
pub fn stub_f2f674() -> ! {
    todo!("0xf2f674 j___ZN3rbx14implementation12typed_holderIN3RBX15ProtectedStringEE9singletonEv")
}

#[doc(alias = "RBX::ProtectedString * rbx::any_cast<RBX::ProtectedString,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf2f684 — j___ZN3rbx8any_castIN3RBX15ProtectedStringENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f2f684() -> ! {
    todo!("0xf2f684 j___ZN3rbx8any_castIN3RBX15ProtectedStringENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ProtectedString & rbx::any_cast<RBX::ProtectedString &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf2f694 — j___ZN3rbx8any_castIRN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f2f694() -> ! {
    todo!("0xf2f694 j___ZN3rbx8any_castIRN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService> RBX::shared_from<RBX::RunService>(RBX::RunService*)")]
// 0xf2f704 — j___ZN3RBX11shared_fromINS_10RunServiceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::RunService> RBX::shared_from<RBX::RunService>(RBX::RunService*)
pub fn stub_f2f704() -> ! {
    todo!("0xf2f704 j___ZN3RBX11shared_fromINS_10RunServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::HeartbeatTask::HeartbeatTask(rbx_core::SharedPtr<RBX::RunService>)")]
// 0xf2f714 — j___ZN3RBX13HeartbeatTaskC2EN5boost10shared_ptrINS_10RunServiceEEE
// was: RBX::HeartbeatTask::HeartbeatTask(boost::shared_ptr<RBX::RunService>)
pub fn stub_f2f714() -> ! {
    todo!("0xf2f714 j___ZN3RBX13HeartbeatTaskC2EN5boost10shared_ptrINS_10RunServiceEEE")
}
