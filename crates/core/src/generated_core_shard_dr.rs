//! core shard DR — 100 core stubs EA-sorted, next uncovered after DQ 0x81a8d0 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string)")]
// 0x81ab04 — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsS5_SsEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string>::type> boost::bind<void,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string>(void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string)
pub fn stub_81ab04() -> ! {
    todo!("0x81ab04 __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsS5_SsEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_")
}

#[doc(alias = "RBX::LibraryService::LibraryStateObject::resumeThreadWithException(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string)")]
// 0x81ad50 — __ZN3RBX14LibraryService18LibraryStateObject25resumeThreadWithExceptionEN5boost10shared_ptrIS1_EESs
// was: RBX::LibraryService::LibraryStateObject::resumeThreadWithException(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string)
pub fn stub_81ad50() -> ! {
    todo!("0x81ad50 __ZN3RBX14LibraryService18LibraryStateObject25resumeThreadWithExceptionEN5boost10shared_ptrIS1_EESs")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
// 0x81b018 — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEES5_EENS_3_bi6bind_tIT_PFS8_T0_ENS6_9list_av_1IT1_E4typeEEESB_SD_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list_av_1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>::type> boost::bind<void,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>(void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)
pub fn stub_81b018() -> ! {
    todo!("0x81b018 __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEES5_EENS_3_bi6bind_tIT_PFS8_T0_ENS6_9list_av_1IT1_E4typeEEESB_SD_")
}

#[doc(alias = "RBX::LibraryService::LibraryStateObject::resumeThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
// 0x81b130 — __ZN3RBX14LibraryService18LibraryStateObject12resumeThreadEN5boost10shared_ptrIS1_EE
// was: RBX::LibraryService::LibraryStateObject::resumeThread(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)
pub fn stub_81b130() -> ! {
    todo!("0x81b130 __ZN3RBX14LibraryService18LibraryStateObject12resumeThreadEN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>)")]
// 0x81b3e8 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf2IvS4_S6_RKSsEENS8_5list3INS8_5valueIPS4_EENS1_3argILi1EEENSG_ISsEEEEEEET0_T_SP_SO_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>>(std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>)
pub fn stub_81b3e8() -> ! {
    todo!("0x81b3e8 __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf2IvS4_S6_RKSsEENS8_5list3INS8_5valueIPS4_EENS1_3argILi1EEENSG_ISsEEEEEEET0_T_SP_SO_")
}

#[doc(alias = "std::map<std::string,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::operator[](std::string const&)")]
// 0x81b444 — __ZNSt3mapISsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS6_EESt4lessISsESaISt4pairIKSsS8_EEEixERSC_
// was: std::map<std::string,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::operator[](std::string const&)
pub fn stub_81b444() -> ! {
    todo!("0x81b444 __ZNSt3mapISsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS6_EESt4lessISsESaISt4pairIKSsS8_EEEixERSC_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list_av_3<RBX::LibraryService*,boost::arg<1>,std::string>::type> boost::bind<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&,RBX::LibraryService*,boost::arg<1>,std::string>(void (RBX::LibraryService::*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&),RBX::LibraryService*,boost::arg<1>,std::string)")]
// 0x81b66c — __ZN5boost4bindIvN3RBX14LibraryServiceENS_10shared_ptrINS2_18LibraryStateObjectEEERKSsPS2_NS_3argILi1EEESsEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list_av_3<RBX::LibraryService*,boost::arg<1>,std::string>::type> boost::bind<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&,RBX::LibraryService*,boost::arg<1>,std::string>(void (RBX::LibraryService::*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&),RBX::LibraryService*,boost::arg<1>,std::string)
pub fn stub_81b66c() -> ! {
    todo!("0x81b66c __ZN5boost4bindIvN3RBX14LibraryServiceENS_10shared_ptrINS2_18LibraryStateObjectEEERKSsPS2_NS_3argILi1EEESsEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>)")]
// 0x81b828 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf1IvS4_S6_EENS8_5list2INS8_5valueIPS4_EENS1_3argILi1EEEEEEEET0_T_SM_SL_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>> std::for_each<std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>>(std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>)
pub fn stub_81b828() -> ! {
    todo!("0x81b828 __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf1IvS4_S6_EENS8_5list2INS8_5valueIPS4_EENS1_3argILi1EEEEEEEET0_T_SM_SL_")
}

#[doc(alias = "void (*)(boost::function<void ()(void)>) std::for_each<std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>)>(std::_List_iterator<boost::function<void ()(void)>>,std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>))")]
// 0x81b87c — __ZSt8for_eachISt14_List_iteratorIN5boost8functionIFvvEEEEPFvS4_EET0_T_S9_S8_
pub fn stub_81b87c() -> ! {
    todo!("0x81b87c __ZSt8for_eachISt14_List_iteratorIN5boost8functionIFvvEEEEPFvS4_EET0_T_S9_S8_")
}

#[doc(alias = "std::map<std::string,RBX::LibraryService::LibraryDefinition,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::operator[](std::string const&)")]
// 0x81bbb4 — __ZNSt3mapISsN3RBX14LibraryService17LibraryDefinitionESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
pub fn stub_81bbb4() -> ! {
    todo!("0x81bbb4 __ZNSt3mapISsN3RBX14LibraryService17LibraryDefinitionESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int)")]
// 0x81beac — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiS5_iEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list_av_2<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int>::type> boost::bind<void,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int>(void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int)
pub fn stub_81beac() -> ! {
    todo!("0x81beac __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiS5_iEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_")
}

#[doc(alias = "RBX::LibraryService::LibraryStateObject::justResume(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int)")]
// 0x81bfcc — __ZN3RBX14LibraryService18LibraryStateObject10justResumeEN5boost10shared_ptrIS1_EEi
// was: RBX::LibraryService::LibraryStateObject::justResume(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int)
pub fn stub_81bfcc() -> ! {
    todo!("0x81bfcc __ZN3RBX14LibraryService18LibraryStateObject10justResumeEN5boost10shared_ptrIS1_EEi")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>::shared_ptr<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")]
// 0x81c750 — __ZN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::LibraryService::LibraryStateObject>::shared_ptr<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)
pub fn stub_81c750() -> ! {
    todo!("0x81c750 __ZN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEC2IS3_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")]
// 0x81c824 — __ZN5boost6detail12shared_countC2IN3RBX14LibraryService18LibraryStateObjectEEEPT_
pub fn stub_81c824() -> ! {
    todo!("0x81c824 __ZN5boost6detail12shared_countC2IN3RBX14LibraryService18LibraryStateObjectEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::~sp_counted_impl_p()")]
// 0x81c9bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED1Ev
pub fn stub_81c9bc() -> ! {
    todo!("0x81c9bc __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::~sp_counted_impl_p()")]
// 0x81c9c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED0Ev
pub fn stub_81c9c0() -> ! {
    todo!("0x81c9c0 __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::dispose(void)")]
// 0x81c9c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE7disposeEv
pub fn stub_81c9c4() -> ! {
    todo!("0x81c9c4 __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::get_deleter(std::type_info const&)")]
// 0x81ca78 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE11get_deleterERKSt9type_info
pub fn stub_81ca78() -> ! {
    todo!("0x81ca78 __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::get_untyped_deleter(void)")]
// 0x81ca7c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE19get_untyped_deleterEv
pub fn stub_81ca7c() -> ! {
    todo!("0x81ca7c __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE19get_untyped_deleterEv")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject> const&)")]
// 0x81e5a8 — __ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE14_M_create_nodeERKS5_
// was: std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::_M_create_node(boost::shared_ptr<RBX::LibraryService::LibraryStateObject> const&)
pub fn stub_81e5a8() -> ! {
    todo!("0x81e5a8 __ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>)")]
// 0x81e850 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>)
pub fn stub_81e850() -> ! {
    todo!("0x81e850 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x81e944 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_81e944() -> ! {
    todo!("0x81e944 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x81e960 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_81e960() -> ! {
    todo!("0x81e960 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")]
// 0x81e974 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const
pub fn stub_81e974() -> ! {
    todo!("0x81e974 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x81ea58 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_81ea58() -> ! {
    todo!("0x81ea58 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x81eb38 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_81eb38() -> ! {
    todo!("0x81eb38 __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::operator()<void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int) &,boost::_bi::list0 &,int)")]
// 0x81ec0c — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEclIPFvS7_iENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::operator()<void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int) &,boost::_bi::list0 &,int)
pub fn stub_81ec0c() -> ! {
    todo!("0x81ec0c __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEclIPFvS7_iENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x81ecd8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_81ecd8() -> ! {
    todo!("0x81ecd8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)")]
// 0x81ee30 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEC2ES8_S9_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::list2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)
pub fn stub_81ee30() -> ! {
    todo!("0x81ee30 __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEC2ES8_S9_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)")]
// 0x81ef08 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEC2ES8_S9_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>)
pub fn stub_81ef08() -> ! {
    todo!("0x81ef08 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_IiEEEC2ES8_S9_")
}

#[doc(alias = "std::pair<std::string const,RBX::LibraryService::LibraryDefinition>::pair(std::string const&,RBX::LibraryService::LibraryDefinition const&)")]
// 0x81f124 — __ZNSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEEC2ERS0_RKS3_
pub fn stub_81f124() -> ! {
    todo!("0x81f124 __ZNSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEEC2ERS0_RKS3_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0x81f1fc — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_81f1fc() -> ! {
    todo!("0x81f1fc __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0x81f2e8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_81f2e8() -> ! {
    todo!("0x81f2e8 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_insert_unique(std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0x81f338 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_81f338() -> ! {
    todo!("0x81f338 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_create_node(std::pair<std::string const,RBX::LibraryService::LibraryDefinition> const&)")]
// 0x81f3bc — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_81f3bc() -> ! {
    todo!("0x81f3bc __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::lower_bound(std::string const&)")]
// 0x81f4e0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
pub fn stub_81f4e0() -> ! {
    todo!("0x81f4e0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")
}

#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::insert_value(RBX::ProtectedString const&)")]
// 0x81f510 — __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE12insert_valueERKS5_
pub fn stub_81f510() -> ! {
    todo!("0x81f510 __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE12insert_valueERKS5_")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase(boost::multi_index::detail::hashed_index_iterator<boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>,boost::multi_index::detail::bucket_array<std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>)")]
// 0x81f7a0 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE5eraseENS1_21hashed_index_iteratorINS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEENS1_12bucket_arrayISO_EEEE
pub fn stub_81f7a0() -> ! {
    todo!("0x81f7a0 __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE5eraseENS1_21hashed_index_iteratorINS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEENS1_12bucket_arrayISO_EEEE")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *)")]
// 0x81f808 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE6erase_EPNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
pub fn stub_81f808() -> ! {
    todo!("0x81f808 __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE6erase_EPNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE")
}

#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&)")]
// 0x81f848 — __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EE7insert_ERKSA_
pub fn stub_81f848() -> ! {
    todo!("0x81f848 __ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EE7insert_ERKSA_")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *)")]
// 0x81f948 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7insert_ERKSC_PNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
pub fn stub_81f948() -> ! {
    todo!("0x81f948 __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7insert_ERKSC_PNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::reserve(unsigned long)")]
// 0x81f9c8 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7reserveEm
pub fn stub_81f9c8() -> ! {
    todo!("0x81f9c8 __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7reserveEm")
}

#[doc(alias = "boost::hash<RBX::ProtectedString>::operator()(RBX::ProtectedString const&)const")]
// 0x81fa10 — __ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_
pub fn stub_81fa10() -> ! {
    todo!("0x81fa10 __ZNK5boost4hashIN3RBX15ProtectedStringEEclERKS2_")
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)")]
// 0x81fa48 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm
pub fn stub_81fa48() -> ! {
    todo!("0x81fa48 __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm")
}

#[doc(alias = "boost::filesystem::path& boost::filesystem::path::append<std::string>(std::string const&,std::codecvt<wchar_t,char,__mbstate_t> const&)")]
// 0x81fc40 — __ZN5boost10filesystem4path6appendISsEERS1_RKT_RKSt7codecvtIwc11__mbstate_tE
pub fn stub_81fc40() -> ! {
    todo!("0x81fc40 __ZN5boost10filesystem4path6appendISsEERS1_RKT_RKSt7codecvtIwc11__mbstate_tE")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::find(std::string const&)")]
// 0x820088 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_820088() -> ! {
    todo!("0x820088 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")
}

#[doc(alias = "std::_List_base<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_clear(void)")]
// 0x8200d8 — __ZNSt10_List_baseIN5boost8functionIFvvEEESaIS3_EE8_M_clearEv
pub fn stub_8200d8() -> ! {
    todo!("0x8200d8 __ZNSt10_List_baseIN5boost8functionIFvvEEESaIS3_EE8_M_clearEv")
}

#[doc(alias = "std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::list(std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>> const&)")]
// 0x820100 — __ZNSt4listIN5boost8functionIFvvEEESaIS3_EEC2ERKS5_
pub fn stub_820100() -> ! {
    todo!("0x820100 __ZNSt4listIN5boost8functionIFvvEEESaIS3_EEC2ERKS5_")
}

#[doc(alias = "void std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::function<void ()(void)>>>(std::_List_const_iterator<boost::function<void ()(void)>>,std::_List_const_iterator<boost::function<void ()(void)>>,std::__false_type)")]
// 0x8201c8 — __ZNSt4listIN5boost8functionIFvvEEESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type
pub fn stub_8201c8() -> ! {
    todo!("0x8201c8 __ZNSt4listIN5boost8functionIFvvEEESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&> &,int)")]
// 0x8201ec — __ZN5boost3_bi5list2INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS4_18LibraryStateObjectEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>> &,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&> &,int)
pub fn stub_8201ec() -> ! {
    todo!("0x8201ec __ZN5boost3_bi5list2INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS4_18LibraryStateObjectEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>::operator()(RBX::LibraryService*,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)const")]
// 0x8202c4 — __ZNK5boost4_mfi3mf1IvN3RBX14LibraryServiceENS_10shared_ptrINS3_18LibraryStateObjectEEEEclEPS3_S6_
// was: boost::_mfi::mf1<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>::operator()(RBX::LibraryService*,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)const
pub fn stub_8202c4() -> ! {
    todo!("0x8202c4 __ZNK5boost4_mfi3mf1IvN3RBX14LibraryServiceENS_10shared_ptrINS3_18LibraryStateObjectEEEEclEPS3_S6_")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::string>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0x8203ac — __ZNK5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEmRKT_RKT0_
pub fn stub_8203ac() -> ! {
    todo!("0x8203ac __ZNK5boost9unordered6detail10table_implINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISsEEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0x820418 — __ZNK5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERKSs
pub fn stub_820418() -> ! {
    todo!("0x820418 __ZNK5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEE9find_nodeERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::string const&)")]
// 0x82045c — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::string const&)
pub fn stub_82045c() -> ! {
    todo!("0x82045c __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>)")]
// 0x820484 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESJ_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>)
pub fn stub_820484() -> ! {
    todo!("0x820484 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESJ_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>)")]
// 0x8204d8 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseESt17_Rb_tree_iteratorISB_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>)
pub fn stub_8204d8() -> ! {
    todo!("0x8204d8 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE5eraseESt17_Rb_tree_iteratorISB_E")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>::destroy(std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>*)")]
// 0x820500 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS9_EEEE7destroyEPSC_
// was: __gnu_cxx::new_allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>::destroy(std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>*)
pub fn stub_820500() -> ! {
    todo!("0x820500 __ZN9__gnu_cxx13new_allocatorISt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS9_EEEE7destroyEPSC_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>> *)")]
// 0x8205a0 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>> *)
pub fn stub_8205a0() -> ! {
    todo!("0x8205a0 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::lower_bound(std::string const&)")]
// 0x8205d0 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE11lower_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::lower_bound(std::string const&)
pub fn stub_8205d0() -> ! {
    todo!("0x8205d0 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::upper_bound(std::string const&)")]
// 0x820600 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE11upper_boundERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::upper_bound(std::string const&)
pub fn stub_820600() -> ! {
    todo!("0x820600 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE11upper_boundERS1_")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>>::operator()<boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>&> &,int)")]
// 0x820634 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEENS2_ISsEEEclINS_4_mfi3mf2IvS4_NS_10shared_ptrINS4_18LibraryStateObjectEEERKSsEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>>::operator()<boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&> &,boost::_bi::list1<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>&> &,int)
pub fn stub_820634() -> ! {
    todo!("0x820634 __ZN5boost3_bi5list3INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEENS2_ISsEEEclINS_4_mfi3mf2IvS4_NS_10shared_ptrINS4_18LibraryStateObjectEEERKSsEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>::operator()(RBX::LibraryService*,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&)const")]
// 0x820710 — __ZNK5boost4_mfi3mf2IvN3RBX14LibraryServiceENS_10shared_ptrINS3_18LibraryStateObjectEEERKSsEclEPS3_S6_S8_
// was: boost::_mfi::mf2<void,RBX::LibraryService,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&>::operator()(RBX::LibraryService*,boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&)const
pub fn stub_820710() -> ! {
    todo!("0x820710 __ZNK5boost4_mfi3mf2IvN3RBX14LibraryServiceENS_10shared_ptrINS3_18LibraryStateObjectEEERKSsEclEPS3_S6_S8_")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>>::list3(boost::_bi::value<RBX::LibraryService *>,boost::arg<1>,boost::_bi::value<std::string>)")]
// 0x8207fc — __ZN5boost3_bi5list3INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEENS2_ISsEEEC2ES6_S8_S9_
pub fn stub_8207fc() -> ! {
    todo!("0x8207fc __ZN5boost3_bi5list3INS0_5valueIPN3RBX14LibraryServiceEEENS_3argILi1EEENS2_ISsEEEC2ES6_S8_S9_")
}

#[doc(alias = "std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>::pair(std::string const&,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>> const&)")]
// 0x820920 — __ZNSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS7_EEEC2ERS0_RKS9_
// was: std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>::pair(std::string const&,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>> const&)
pub fn stub_820920() -> ! {
    todo!("0x820920 __ZNSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS7_EEEC2ERS0_RKS9_")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::list(std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>> const&)")]
// 0x8209c8 — __ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EEC2ERKS7_
// was: std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::list(std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>> const&)
pub fn stub_8209c8() -> ! {
    todo!("0x8209c8 __ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EEC2ERKS7_")
}

#[doc(alias = "void std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_initialize_dispatch<std::_List_const_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>(std::_List_const_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_const_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::__false_type)")]
// 0x820a90 — __ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE22_M_initialize_dispatchISt20_List_const_iteratorIS5_EEEvT_SB_St12__false_type
// was: void std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::_M_initialize_dispatch<std::_List_const_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>(std::_List_const_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::_List_const_iterator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,std::__false_type)
pub fn stub_820a90() -> ! {
    todo!("0x820a90 __ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE22_M_initialize_dispatchISt20_List_const_iteratorIS5_EEEvT_SB_St12__false_type")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0x820ab4 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_820ab4() -> ! {
    todo!("0x820ab4 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0x820ba0 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_820ba0() -> ! {
    todo!("0x820ba0 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSB_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0x820bf0 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE16_M_insert_uniqueERKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_insert_unique(std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_820bf0() -> ! {
    todo!("0x820bf0 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE16_M_insert_uniqueERKSB_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_create_node(std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>> const&)")]
// 0x820c74 — __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE14_M_create_nodeERKSB_
// was: std::_Rb_tree<std::string,std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,std::_Select1st<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>>::_M_create_node(std::pair<std::string const,std::list<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::allocator<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>> const&)
pub fn stub_820c74() -> ! {
    todo!("0x820c74 __ZNSt8_Rb_treeISsSt4pairIKSsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS8_EEESt10_Select1stISB_ESt4lessISsESaISB_EE14_M_create_nodeERKSB_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>)")]
// 0x820f28 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>)
pub fn stub_820f28() -> ! {
    todo!("0x820f28 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x821018 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_821018() -> ! {
    todo!("0x821018 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x821034 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_821034() -> ! {
    todo!("0x821034 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &)const")]
// 0x82103c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS5_5list1INS5_5valueISB_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &)const
pub fn stub_82103c() -> ! {
    todo!("0x82103c __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS5_5list1INS5_5valueISB_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x82111c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS5_5list1INS5_5valueISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_82111c() -> ! {
    todo!("0x82111c __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS5_5list1INS5_5valueISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>) &,boost::_bi::list0 &,int)")]
// 0x821214 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEclIPFvS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::operator()<void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>) &,boost::_bi::list0 &,int)
pub fn stub_821214() -> ! {
    todo!("0x821214 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEclIPFvS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x8212e0 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE12manage_smallERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_8212e0() -> ! {
    todo!("0x8212e0 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS3_5list1INS3_5valueIS9_EEEEEEE12manage_smallERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>)")]
// 0x821364 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEC2ES8_
// was: boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>)
pub fn stub_821364() -> ! {
    todo!("0x821364 __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEEEC2ES8_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>)")]
// 0x821758 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>)
pub fn stub_821758() -> ! {
    todo!("0x821758 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x8218f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_8218f0() -> ! {
    todo!("0x8218f0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x82190c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_82190c() -> ! {
    todo!("0x82190c __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0x821920 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_821920() -> ! {
    todo!("0x821920 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x821aa8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_821aa8() -> ! {
    todo!("0x821aa8 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x821c2c — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_821c2c() -> ! {
    todo!("0x821c2c __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS5_5list2INS5_5valueISB_EENSF_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string) &,boost::_bi::list0 &,int)")]
// 0x821d34 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEclIPFvS7_SsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string) &,boost::_bi::list0 &,int)
pub fn stub_821d34() -> ! {
    todo!("0x821d34 __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEclIPFvS7_SsENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x821ea0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_821ea0() -> ! {
    todo!("0x821ea0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)")]
// 0x82203c — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEC2ES8_S9_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)
pub fn stub_82203c() -> ! {
    todo!("0x82203c __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEC2ES8_S9_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)")]
// 0x8221a8 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEC2ES8_S9_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<std::string>)
pub fn stub_8221a8() -> ! {
    todo!("0x8221a8 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEEENS2_ISsEEEC2ES8_S9_")
}

#[doc(alias = "std::list<boost::function<void ()(void)>,std::allocator<boost::function<void ()(void)>>>::_M_create_node(boost::function<void ()(void)> const&)")]
// 0x8222b0 — __ZNSt4listIN5boost8functionIFvvEEESaIS3_EE14_M_create_nodeERKS3_
pub fn stub_8222b0() -> ! {
    todo!("0x8222b0 __ZNSt4listIN5boost8functionIFvvEEESaIS3_EE14_M_create_nodeERKS3_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<std::string>,std::string,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::string>> const&)")]
// 0x822388 — __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeISsEEE
pub fn stub_822388() -> ! {
    todo!("0x822388 __ZN5boost9unordered6detail5tableINS1_3setISaISsESsNS_4hashISsEESt8equal_toISsEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeISsEEE")
}

#[doc(alias = "RBX::LibraryService::LibraryStateObject::LibraryStateObject(lua_State *,std::string const&,bool)")]
// 0x8223f8 — __ZN3RBX14LibraryService18LibraryStateObjectC2EP9lua_StateRKSsb
pub fn stub_8223f8() -> ! {
    todo!("0x8223f8 __ZN3RBX14LibraryService18LibraryStateObjectC2EP9lua_StateRKSsb")
}

#[doc(alias = "void boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::erase<bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&)>(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&,bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&))")]
// 0x822500 — __ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE5eraseIPFbRKNS1_17refcounted_handleIPKNS1_16refcounted_valueINS6_8rep_typeES5_EESE_EEEEEvSO_T_
pub fn stub_822500() -> ! {
    todo!("0x822500 __ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE5eraseIPFbRKNS1_17refcounted_handleIPKNS1_16refcounted_valueINS6_8rep_typeES5_EESE_EEEEEvSO_T_")
}

#[doc(alias = "std::_List_base<std::string,std::allocator<std::string>>::_M_clear(void)")]
// 0x8225f4 — __ZNSt10_List_baseISsSaISsEE8_M_clearEv
pub fn stub_8225f4() -> ! {
    todo!("0x8225f4 __ZNSt10_List_baseISsSaISsEE8_M_clearEv")
}

#[doc(alias = "std::list<std::string,std::allocator<std::string>>::_M_create_node(std::string const&)")]
// 0x82261c — __ZNSt4listISsSaISsEE14_M_create_nodeERKSs
pub fn stub_82261c() -> ! {
    todo!("0x82261c __ZNSt4listISsSaISsEE14_M_create_nodeERKSs")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::insert(RobloxExtraSpace&)")]
// 0x823b10 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E6insertERS2_
pub fn stub_823b10() -> ! {
    todo!("0x823b10 __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E6insertERS2_")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Hook::remove(void)")]
// 0x823d98 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E4Hook6removeEv
pub fn stub_823d98() -> ! {
    todo!("0x823d98 __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E4Hook6removeEv")
}

#[doc(alias = "luaL_loadprotected(lua_State *,RBX::ProtectedString const&,char const*)")]
// 0x824b14 — __Z18luaL_loadprotectedP9lua_StateRKN3RBX15ProtectedStringEPKc
pub fn stub_824b14() -> ! {
    todo!("0x824b14 __Z18luaL_loadprotectedP9lua_StateRKN3RBX15ProtectedStringEPKc")
}

#[doc(alias = "rbx_core::SharedPtr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
// 0x82e624 — __ZN5boost10shared_ptrIN16RobloxExtraSpace6SharedEEC2IS2_EEPT_
// was: boost::shared_ptr<RobloxExtraSpace::Shared>::shared_ptr<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)
pub fn stub_82e624() -> ! {
    todo!("0x82e624 __ZN5boost10shared_ptrIN16RobloxExtraSpace6SharedEEC2IS2_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RobloxExtraSpace::Shared>(RobloxExtraSpace::Shared *)")]
// 0x82e6f8 — __ZN5boost6detail12shared_countC2IN16RobloxExtraSpace6SharedEEEPT_
pub fn stub_82e6f8() -> ! {
    todo!("0x82e6f8 __ZN5boost6detail12shared_countC2IN16RobloxExtraSpace6SharedEEEPT_")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::erase(RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator)")]
// 0x82e808 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E5eraseENS3_8IteratorE
pub fn stub_82e808() -> ! {
    todo!("0x82e808 __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E5eraseENS3_8IteratorE")
}
