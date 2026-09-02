//! rendering shard wd_rendA — 120 stubs 0x8995c8..0x8abb98 EA-sorted asc gap filler not yet in crates/rendering/src (RBX::Gfx/Render filtered exhausted -> global gap filler distinct per crate, skip global)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x857138
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x8995c8 — __ZN3RBX24CacheableContentProvider25LoadContentCallbackHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEESs
// type: void __fastcall(int, int, int, const shared_count *, std::string *)
#[doc(alias = "RBX::CacheableContentProvider::LoadContentCallbackHelper(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider25LoadContentCallbackHelperEN5boost8weak_ptrIS0_EENS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEESs")]
pub fn stub_8995c8() -> ! {
    todo!("0x8995c8 RBX::CacheableContentProvider::LoadContentCallbackHelper(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)")
}

// 0x899774 — __ZN3RBX24CacheableContentProvider19LoadContentCallbackENS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIKSsEESs
// type: void __fastcall(int, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::CacheableContentProvider::LoadContentCallback(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider19LoadContentCallbackENS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIKSsEESs")]
pub fn stub_899774() -> ! {
    todo!("0x899774 RBX::CacheableContentProvider::LoadContentCallback(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string)")
}

// 0x899858 — __ZN3RBX24CacheableContentProvider9ErrorTaskERKSs
// type: int __fastcall(RBX::CacheableContentProvider *this, const std::string *)
#[doc(alias = "RBX::CacheableContentProvider::ErrorTask(std::string const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider9ErrorTaskERKSs")]
pub fn stub_899858() -> ! {
    todo!("0x899858 RBX::CacheableContentProvider::ErrorTask(std::string const&)")
}

// 0x89985c — __ZN3RBX24CacheableContentProvider17markContentFailedERKSs
// type: void __fastcall(RBX::CacheableContentProvider *this, const std::string *)
#[doc(alias = "RBX::CacheableContentProvider::markContentFailed(std::string const&)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider17markContentFailedERKSs")]
pub fn stub_89985c() -> ! {
    todo!("0x89985c RBX::CacheableContentProvider::markContentFailed(std::string const&)")
}

// 0x8999a0 — __ZN3RBX24CacheableContentProvider13updateContentERKSsN5boost10shared_ptrINS0_10CachedItemEEE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::CacheableContentProvider::updateContent(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>)")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider13updateContentERKSsN5boost10shared_ptrINS0_10CachedItemEEE")]
pub fn stub_8999a0() -> ! {
    todo!("0x8999a0 RBX::CacheableContentProvider::updateContent(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>)")
}

// 0x899a7c — __ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::ContentProviderJob>::operator=(boost::shared_ptr<RBX::ContentProviderJob> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEaSERKS3_")]
pub fn stub_899a7c() -> ! {
    todo!("0x899a7c boost::shared_ptr<RBX::ContentProviderJob>::operator=(boost::shared_ptr<RBX::ContentProviderJob> const&)")
}

// 0x899ab4 — __ZN5boost4bindIN3RBX13TaskScheduler10StepResultENS_8weak_ptrINS1_24CacheableContentProviderEEERKSsNS_10shared_ptrIS7_EES6_NS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_PFSG_T0_T1_T2_ENSE_9list_av_3IT3_T4_T5_E4typeEEESL_SN_SO_SP_
// type: void __fastcall(_DWORD *, int, int *)
#[doc(alias = "boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::TaskScheduler::StepResult,boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>(RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost4bindIN3RBX13TaskScheduler10StepResultENS_8weak_ptrINS1_24CacheableContentProviderEEERKSsNS_10shared_ptrIS7_EES6_NS_3argILi1EEENSB_ILi2EEEEENS_3_bi6bind_tIT_PFSG_T0_T1_T2_ENSE_9list_av_3IT3_T4_T5_E4typeEEESL_SN_SO_SP_")]
pub fn stub_899ab4() -> ! {
    todo!("0x899ab4 boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list_av_3<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>::type> boost::bind<RBX::TaskScheduler::StepResult,boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>>(RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>)")
}

// 0x899c54 — __ZN3RBX9weak_fromINS_24CacheableContentProviderEEEN5boost8weak_ptrIT_EEPS4_
// type: void __fastcall(int, int)
#[doc(alias = "boost::weak_ptr<RBX::CacheableContentProvider> RBX::weak_from<RBX::CacheableContentProvider>(RBX::CacheableContentProvider*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_24CacheableContentProviderEEEN5boost8weak_ptrIT_EEPS4_")]
pub fn stub_899c54() -> ! {
    todo!("0x899c54 boost::weak_ptr<RBX::CacheableContentProvider> RBX::weak_from<RBX::CacheableContentProvider>(RBX::CacheableContentProvider*)")
}

// 0x899e4c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsS4_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// type: void __fastcall(_DWORD *, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list_av_2<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsS4_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")]
pub fn stub_899e4c() -> ! {
    todo!("0x899e4c boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list_av_2<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>)")
}

// 0x899fec — __ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEE5resetIS3_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *, int)
#[doc(alias = "void boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>::reset<RBX::CacheableContentProvider::CachedItem>(RBX::CacheableContentProvider::CachedItem *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEE5resetIS3_EEvPT_")]
pub fn stub_899fec() -> ! {
    todo!("0x899fec void boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>::reset<RBX::CacheableContentProvider::CachedItem>(RBX::CacheableContentProvider::CachedItem *)")
}

// 0x89a018 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsS4_NS_3argILi1EEENSB_ILi2EEENSB_ILi3EEESsEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_ENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESO_SQ_SR_SS_ST_SU_
// type: void __fastcall(_DWORD *, int, int *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list_av_5<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>::type> boost::bind<void,boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>(void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsS4_NS_3argILi1EEENSB_ILi2EEENSB_ILi3EEESsEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_ENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESO_SQ_SR_SS_ST_SU_")]
pub fn stub_89a018() -> ! {
    todo!("0x89a018 boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list_av_5<boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>::type> boost::bind<void,boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string,boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string>(void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::weak_ptr<RBX::CacheableContentProvider>,boost::arg<1>,boost::arg<2>,boost::arg<3>,std::string)")
}

// 0x89a424 — __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6removeERKSs
// type: int __fastcall(int, int)
#[doc(alias = "RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::remove(std::string const&)")]
#[doc(alias = "__ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6removeERKSs")]
pub fn stub_89a424() -> ! {
    todo!("0x89a424 RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::remove(std::string const&)")
}

// 0x89a4b8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISF_EESP_")]
pub fn stub_89a4b8() -> ! {
    todo!("0x89a4b8 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> *)")
}

// 0x89a518 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
pub fn stub_89a518() -> ! {
    todo!("0x89a518 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x89a558 — __ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_eraseESt14_List_iteratorIS8_E
// type: void __fastcall(int, boost::detail::sp_counted_base **this, int, int, int, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>)")]
#[doc(alias = "__ZNSt4listISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_eraseESt14_List_iteratorIS8_E")]
pub fn stub_89a558() -> ! {
    todo!("0x89a558 std::list<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>)")
}

// 0x89a630 — __ZN5boost10shared_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::CacheableContentProvider>::shared_ptr<RBX::CacheableContentProvider>(boost::weak_ptr<RBX::CacheableContentProvider> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_89a630() -> ! {
    todo!("0x89a630 boost::shared_ptr<RBX::CacheableContentProvider>::shared_ptr<RBX::CacheableContentProvider>(boost::weak_ptr<RBX::CacheableContentProvider> const&,boost::detail::sp_nothrow_tag)")
}

// 0x89aa34 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_24CacheableContentProviderEEES3_S4_S7_SsENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEEEvT_")]
pub fn stub_89aa34() -> ! {
    todo!("0x89aa34 void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>)")
}

// 0x89ac08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
pub fn stub_89ac08() -> ! {
    todo!("0x89ac08 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x89ac24 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_")]
pub fn stub_89ac24() -> ! {
    todo!("0x89ac24 boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")
}

// 0x89ac48 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, void *)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_89ac48() -> ! {
    todo!("0x89ac48 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")
}

// 0x89ae0c — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, void *)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_89ae0c() -> ! {
    todo!("0x89ae0c bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x89afcc — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_24CacheableContentProviderEEES5_S6_S9_SsENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEENSK_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_89afcc() -> ! {
    todo!("0x89afcc void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x89b110 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, struct _Unwind_Exception **, int **)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::operator()<void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_89b110() -> ! {
    todo!("0x89b110 void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::operator()<void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const>&> &,int)")
}

// 0x89b310 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEENS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEESsENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSK_ILi2EEENSK_ILi3EEENSI_ISsEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_89b310() -> ! {
    todo!("0x89b310 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>,std::string),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x89b500 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, const std::string *)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::list5(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_")]
pub fn stub_89b500() -> ! {
    todo!("0x89b500 boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::list5(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>)")
}

// 0x89b6ac — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_
// type: int __fastcall(int, int *, const std::string *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEENS2_ISsEEEC2ES7_S9_SA_SB_SC_")]
pub fn stub_89b6ac() -> ! {
    todo!("0x89b6ac boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::_bi::value<std::string>)")
}

// 0x89b7ec — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEEEC2ES7_S9_SA_SB_
// type: int __fastcall(int, int *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEENS8_ILi3EEEEC2ES7_S9_SA_SB_")]
pub fn stub_89b7ec() -> ! {
    todo!("0x89b7ec boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x89b904 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
// type: struct _Unwind_Exception *__fastcall(struct _Unwind_Exception *, boost::detail::sp_counted_base **, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_")]
pub fn stub_89b904() -> ! {
    todo!("0x89b904 boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>)")
}

// 0x89ba1c — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_")]
pub fn stub_89ba1c() -> ! {
    todo!("0x89ba1c boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>)")
}

// 0x89bb68 — __ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEC2IS3_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>::shared_ptr<RBX::CacheableContentProvider::CachedItem>(RBX::CacheableContentProvider::CachedItem *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEC2IS3_EEPT_")]
pub fn stub_89bb68() -> ! {
    todo!("0x89bb68 boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>::shared_ptr<RBX::CacheableContentProvider::CachedItem>(RBX::CacheableContentProvider::CachedItem *)")
}

// 0x89bc40 — __ZN3RBX24CacheableContentProvider10CachedItemD2Ev
// type: void __fastcall(RBX::CacheableContentProvider::CachedItem *__hidden this)
#[doc(alias = "RBX::CacheableContentProvider::CachedItem::~CachedItem()")]
#[doc(alias = "__ZN3RBX24CacheableContentProvider10CachedItemD2Ev")]
pub fn stub_89bc40() -> ! {
    todo!("0x89bc40 RBX::CacheableContentProvider::CachedItem::~CachedItem()")
}

// 0x89bd10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX24CacheableContentProvider10CachedItemEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CacheableContentProvider::CachedItem>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX24CacheableContentProvider10CachedItemEED0Ev")]
pub fn stub_89bd10() -> ! {
    todo!("0x89bd10 boost::detail::sp_counted_impl_p<RBX::CacheableContentProvider::CachedItem>::~sp_counted_impl_p()")
}

// 0x89bd18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX24CacheableContentProvider10CachedItemEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::CacheableContentProvider::CachedItem>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX24CacheableContentProvider10CachedItemEE19get_untyped_deleterEv")]
pub fn stub_89bd18() -> ! {
    todo!("0x89bd18 boost::detail::sp_counted_impl_p<RBX::CacheableContentProvider::CachedItem>::get_untyped_deleter(void)")
}

// 0x89bd1c — __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE21internalMakeEvictableERKSsRKS5_m
// type: int __fastcall(int, int)
#[doc(alias = "RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::internalMakeEvictable(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
#[doc(alias = "__ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE21internalMakeEvictableERKSsRKS5_m")]
pub fn stub_89bd1c() -> ! {
    todo!("0x89bd1c RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::internalMakeEvictable(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")
}

// 0x89bdfc — __ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>::operator=(boost::shared_ptr<RBX::CacheableContentProvider::CachedItem> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEaSERKS4_")]
pub fn stub_89bdfc() -> ! {
    todo!("0x89bdfc boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>::operator=(boost::shared_ptr<RBX::CacheableContentProvider::CachedItem> const&)")
}

// 0x89be38 — __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE5fetchERKSsPS5_b
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::fetch(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>*,bool)")]
#[doc(alias = "__ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE5fetchERKSsPS5_b")]
pub fn stub_89be38() -> ! {
    todo!("0x89be38 RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::fetch(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>*,bool)")
}

// 0x89bed0 — __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)")]
#[doc(alias = "__ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm")]
pub fn stub_89bed0() -> ! {
    todo!("0x89bed0 RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)")
}

// 0x89c06c — __ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEC2IS2_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::ContentProviderJob>::shared_ptr<RBX::ContentProviderJob>(RBX::ContentProviderJob *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18ContentProviderJobEEC2IS2_EEPT_")]
pub fn stub_89c06c() -> ! {
    todo!("0x89c06c boost::shared_ptr<RBX::ContentProviderJob>::shared_ptr<RBX::ContentProviderJob>(RBX::ContentProviderJob *)")
}

// 0x89c154 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_18ContentProviderJobES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::ContentProviderJob,RBX::ContentProviderJob>(boost::shared_ptr<RBX::ContentProviderJob> const*,RBX::ContentProviderJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_18ContentProviderJobES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_89c154() -> ! {
    todo!("0x89c154 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::ContentProviderJob,RBX::ContentProviderJob>(boost::shared_ptr<RBX::ContentProviderJob> const*,RBX::ContentProviderJob *)const")
}

// 0x89c238 — __ZN5boost6detail12shared_countC2IN3RBX18ContentProviderJobEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentProviderJob>(RBX::ContentProviderJob *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX18ContentProviderJobEEEPT_")]
pub fn stub_89c238() -> ! {
    todo!("0x89c238 boost::detail::shared_count::shared_count<RBX::ContentProviderJob>(RBX::ContentProviderJob *)")
}

// 0x89c330 — __ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEED1Ev")]
pub fn stub_89c330() -> ! {
    todo!("0x89c330 boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::~sp_counted_impl_p()")
}

// 0x89c334 — __ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEED0Ev")]
pub fn stub_89c334() -> ! {
    todo!("0x89c334 boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::~sp_counted_impl_p()")
}

// 0x89c338 — __ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEE7disposeEv")]
pub fn stub_89c338() -> ! {
    todo!("0x89c338 boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::dispose(void)")
}

// 0x89c348 — __ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEE11get_deleterERKSt9type_info")]
pub fn stub_89c348() -> ! {
    todo!("0x89c348 boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::get_deleter(std::type_info const&)")
}

// 0x89c34c — __ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX18ContentProviderJobEE19get_untyped_deleterEv")]
pub fn stub_89c34c() -> ! {
    todo!("0x89c34c boost::detail::sp_counted_impl_p<RBX::ContentProviderJob>::get_untyped_deleter(void)")
}

// 0x89c5a0 — __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_89c5a0() -> ! {
    todo!("0x89c5a0 void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>)")
}

// 0x89c6d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
pub fn stub_89c6d8() -> ! {
    todo!("0x89c6d8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x89c6f4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs")]
pub fn stub_89c6f4() -> ! {
    todo!("0x89c6f4 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")
}

// 0x89c70c — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_89c70c() -> ! {
    todo!("0x89c70c bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x89c82c — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS5_5list2INS5_5valueISA_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_89c82c() -> ! {
    todo!("0x89c82c bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x89c9a8 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEclIPFvS6_RKSsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, _DWORD), _DWORD *)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&) &,boost::_bi::list1<std::string &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEclIPFvS6_RKSsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_89c9a8() -> ! {
    todo!("0x89c9a8 void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&) &,boost::_bi::list1<std::string &> &,int)")
}

// 0x89cabc — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX24CacheableContentProviderEEERKSsENS3_5list2INS3_5valueIS8_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")]
pub fn stub_89cabc() -> ! {
    todo!("0x89cabc boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x89cde4 — __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "__ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS3_PFS3_NS_8weak_ptrINS1_24CacheableContentProviderEEERS5_S6_ENS9_5list3INS9_5valueISD_EENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_")]
pub fn stub_89cde4() -> ! {
    todo!("0x89cde4 void boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x89cf1c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
pub fn stub_89cf1c() -> ! {
    todo!("0x89cf1c boost::detail::function::functor_manager<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x89cf38 — __ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEES7_SsSE_E6invokeERNS1_15function_bufferESsSE_
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,std::string,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEES7_SsSE_E6invokeERNS1_15function_bufferESsSE_")]
pub fn stub_89cf38() -> ! {
    todo!("0x89cf38 boost::detail::function::function_obj_invoker2<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,std::string,boost::shared_ptr<std::string const>)")
}

// 0x89cf54 — __ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_89cf54() -> ! {
    todo!("0x89cf54 bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")
}

// 0x89d074 — __ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS_8weak_ptrINS3_24CacheableContentProviderEEERS7_S8_ENSB_5list3INSB_5valueISF_EENS_3argILi1EEENSM_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_89d074() -> ! {
    todo!("0x89d074 bool boost::detail::function::basic_vtable2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x89d1f0 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEclINS4_13TaskScheduler10StepResultEPFSE_S6_RKSsNS_10shared_ptrISF_EEENS0_5list2IRSsRSI_EEEET_NS0_4typeISP_EERT0_RT1_l
// type: int __fastcall(int *, int (__fastcall **)(int *, _DWORD, int *), __int64 *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TaskScheduler::StepResult boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&>>(boost::_bi::type<RBX::TaskScheduler::StepResult>,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>) &,boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEclINS4_13TaskScheduler10StepResultEPFSE_S6_RKSsNS_10shared_ptrISF_EEENS0_5list2IRSsRSI_EEEET_NS0_4typeISP_EERT0_RT1_l")]
pub fn stub_89d1f0() -> ! {
    todo!("0x89d1f0 RBX::TaskScheduler::StepResult boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::operator()<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&>>(boost::_bi::type<RBX::TaskScheduler::StepResult>,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>) &,boost::_bi::list2<std::string &,boost::shared_ptr<std::string const>&> &,long)")
}

// 0x89d350 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIN3RBX13TaskScheduler10StepResultEPFS7_NS_8weak_ptrINS5_24CacheableContentProviderEEERKSsNS_10shared_ptrISB_EEENS3_5list3INS3_5valueISA_EENS_3argILi1EEENSK_ILi2EEEEEEEE12manage_smallERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
pub fn stub_89d350() -> ! {
    todo!("0x89d350 boost::detail::function::functor_manager_common<boost::_bi::bind_t<RBX::TaskScheduler::StepResult,RBX::TaskScheduler::StepResult (*)(boost::weak_ptr<RBX::CacheableContentProvider>,std::string const&,boost::shared_ptr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x89d428 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_
// type: struct _Unwind_Exception *__fastcall(struct _Unwind_Exception *, boost::detail::sp_counted_base **, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>::list2(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEEEC2ES7_S9_")]
pub fn stub_89d428() -> ! {
    todo!("0x89d428 boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>>::list2(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>)")
}

// 0x89d540 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
// type: int __fastcall(int, int *)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX24CacheableContentProviderEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_")]
pub fn stub_89d540() -> ! {
    todo!("0x89d540 boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>>::list3(boost::_bi::value<boost::weak_ptr<RBX::CacheableContentProvider>>,boost::arg<1>,boost::arg<2>)")
}

// 0x89d658 — __ZN5boost8weak_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::weak_ptr<RBX::CacheableContentProvider>::weak_ptr<RBX::CacheableContentProvider>(boost::shared_ptr<RBX::CacheableContentProvider> const&,boost::detail::sp_enable_if_convertible<RBX::CacheableContentProvider,RBX::CacheableContentProvider>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX24CacheableContentProviderEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
pub fn stub_89d658() -> ! {
    todo!("0x89d658 boost::weak_ptr<RBX::CacheableContentProvider>::weak_ptr<RBX::CacheableContentProvider>(boost::shared_ptr<RBX::CacheableContentProvider> const&,boost::detail::sp_enable_if_convertible<RBX::CacheableContentProvider,RBX::CacheableContentProvider>::type)")
}

// 0x89d6a8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE5clearEv")]
pub fn stub_89d6a8() -> ! {
    todo!("0x89d6a8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x89d6dc — __ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_clearEv
// type: void __fastcall(_DWORD **, int, int, int, struct _Unwind_Exception *lpuexcpt, std::string *, int, int, int, int)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseISt4pairISsS0_ImN5boost10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEESaIS8_EE8_M_clearEv")]
pub fn stub_89d6dc() -> ! {
    todo!("0x89d6dc std::_List_base<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>,std::allocator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>::_M_clear(void)")
}

// 0x89d7c0 — __ZN3RBX28ConcurrentControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmmNS_22CacheSizeEnforceMethodE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::ConcurrentControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::ConcurrentControlledLRUCache(unsigned long,unsigned long,RBX::CacheSizeEnforceMethod)")]
#[doc(alias = "__ZN3RBX28ConcurrentControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEEC2EmmNS_22CacheSizeEnforceMethodE")]
pub fn stub_89d7c0() -> ! {
    todo!("0x89d7c0 RBX::ConcurrentControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::ConcurrentControlledLRUCache(unsigned long,unsigned long,RBX::CacheSizeEnforceMethod)")
}

// 0x89d880 — __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::~ControlledLRUCache()")]
#[doc(alias = "__ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev")]
pub fn stub_89d880() -> ! {
    todo!("0x89d880 RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::~ControlledLRUCache()")
}

// 0x89d940 — __ZN5boost10scoped_ptrIN3RBX8LRUCacheISsNS_10shared_ptrINS1_24CacheableContentProvider10CachedItemEEEEEED2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(std::string **, int, int, int, int)
#[doc(alias = "boost::scoped_ptr<RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>::~scoped_ptr()")]
#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX8LRUCacheISsNS_10shared_ptrINS1_24CacheableContentProvider10CachedItemEEEEEED2Ev")]
pub fn stub_89d940() -> ! {
    todo!("0x89d940 boost::scoped_ptr<RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>::~scoped_ptr()")
}

// 0x89d9e8 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev
// type: std::string *__fastcall(std::string *, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::~LRUCache()")]
#[doc(alias = "__ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEED2Ev")]
pub fn stub_89d9e8() -> ! {
    todo!("0x89d9e8 RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::~LRUCache()")
}

// 0x89dafc — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)")]
#[doc(alias = "__ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6resizeEm")]
pub fn stub_89dafc() -> ! {
    todo!("0x89dafc RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::resize(unsigned long)")
}

// 0x89db70 — __ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m
// type: void __fastcall(int, const std::string *, const shared_count *, int)
#[doc(alias = "RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
#[doc(alias = "__ZN3RBX8LRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m")]
pub fn stub_89db70() -> ! {
    todo!("0x89db70 RBX::LRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")
}

// 0x89e08c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS5_RKT_")]
pub fn stub_89e08c() -> ! {
    todo!("0x89e08c std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> const&)")
}

// 0x89e244 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_")]
pub fn stub_89e244() -> ! {
    todo!("0x89e244 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>> const&)")
}

// 0x89e268 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
pub fn stub_89e268() -> ! {
    todo!("0x89e268 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x89e2b8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>>::~node_constructor()")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEEEEED2Ev")]
pub fn stub_89e2b8() -> ! {
    todo!("0x89e2b8 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>>>::~node_constructor()")
}

// 0x8a68d8 — __ZNK3RBX17BuoyancyConnector22getConnectorKernelTypeEv
// type: int __fastcall(RBX::BuoyancyConnector *this)
#[doc(alias = "RBX::BuoyancyConnector::getConnectorKernelType(void)const")]
#[doc(alias = "__ZNK3RBX17BuoyancyConnector22getConnectorKernelTypeEv")]
pub fn stub_8a68d8() -> ! {
    todo!("0x8a68d8 RBX::BuoyancyConnector::getConnectorKernelType(void)const")
}

// 0x8a68dc — __ZN3RBX17BuoyancyConnectorD1Ev
// type: void __fastcall(RBX::BuoyancyConnector *__hidden this)
#[doc(alias = "RBX::BuoyancyConnector::~BuoyancyConnector()")]
#[doc(alias = "__ZN3RBX17BuoyancyConnectorD1Ev")]
pub fn stub_8a68dc() -> ! {
    todo!("0x8a68dc RBX::BuoyancyConnector::~BuoyancyConnector()")
}

// 0x8a68e0 — __ZN3RBX17BuoyancyConnectorD0Ev
// type: void __fastcall(RBX::BuoyancyConnector *__hidden this)
#[doc(alias = "RBX::BuoyancyConnector::~BuoyancyConnector()")]
#[doc(alias = "__ZN3RBX17BuoyancyConnectorD0Ev")]
pub fn stub_8a68e0() -> ! {
    todo!("0x8a68e0 RBX::BuoyancyConnector::~BuoyancyConnector()")
}

// 0x8a6ab4 — __ZN3RBX5HUMAN8SwimmingC1EPNS_8HumanoidENS0_9StateTypeE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::HUMAN::Swimming::Swimming(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN8SwimmingC1EPNS_8HumanoidENS0_9StateTypeE")]
pub fn stub_8a6ab4() -> ! {
    todo!("0x8a6ab4 RBX::HUMAN::Swimming::Swimming(RBX::Humanoid *,RBX::HUMAN::StateType)")
}

// 0x8a6ab8 — __ZN3RBX5HUMAN8SwimmingC2EPNS_8HumanoidENS0_9StateTypeE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::HUMAN::Swimming::Swimming(RBX::Humanoid *,RBX::HUMAN::StateType)")]
#[doc(alias = "__ZN3RBX5HUMAN8SwimmingC2EPNS_8HumanoidENS0_9StateTypeE")]
pub fn stub_8a6ab8() -> ! {
    todo!("0x8a6ab8 RBX::HUMAN::Swimming::Swimming(RBX::Humanoid *,RBX::HUMAN::StateType)")
}

// 0x8a6bd0 — __ZN3RBX5HUMAN8Swimming18onComputeForceImplEv
// type: RBX::Body *__fastcall(RBX::HUMAN::Swimming *this)
#[doc(alias = "RBX::HUMAN::Swimming::onComputeForceImpl(void)")]
#[doc(alias = "__ZN3RBX5HUMAN8Swimming18onComputeForceImplEv")]
pub fn stub_8a6bd0() -> ! {
    todo!("0x8a6bd0 RBX::HUMAN::Swimming::onComputeForceImpl(void)")
}

// 0x8a7118 — __ZN3RBX5HUMAN8Swimming19onSimulatorStepImplEf
// type: RBX::Velocity *__fastcall(RBX::HUMAN::Swimming *this, float32_t)
#[doc(alias = "RBX::HUMAN::Swimming::onSimulatorStepImpl(float)")]
#[doc(alias = "__ZN3RBX5HUMAN8Swimming19onSimulatorStepImplEf")]
pub fn stub_8a7118() -> ! {
    todo!("0x8a7118 RBX::HUMAN::Swimming::onSimulatorStepImpl(float)")
}

// 0x8a7238 — __ZN3RBX5HUMAN8Swimming10fireEventsEv
// type: int __fastcall(RBX::HUMAN::Swimming *this)
#[doc(alias = "RBX::HUMAN::Swimming::fireEvents(void)")]
#[doc(alias = "__ZN3RBX5HUMAN8Swimming10fireEventsEv")]
pub fn stub_8a7238() -> ! {
    todo!("0x8a7238 RBX::HUMAN::Swimming::fireEvents(void)")
}

// 0x8a728c — __ZN3RBX5HUMAN13HumanoidState20minSwimmingMoveForceEv
// type: int __fastcall(RBX::HUMAN::HumanoidState *this)
#[doc(alias = "RBX::HUMAN::HumanoidState::minSwimmingMoveForce(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState20minSwimmingMoveForceEv")]
pub fn stub_8a728c() -> ! {
    todo!("0x8a728c RBX::HUMAN::HumanoidState::minSwimmingMoveForce(void)")
}

// 0x8a72e8 — __ZN3RBX5HUMAN13HumanoidState20maxSwimmingMoveForceEv
// type: int __fastcall(RBX::HUMAN::HumanoidState *this)
#[doc(alias = "RBX::HUMAN::HumanoidState::maxSwimmingMoveForce(void)")]
#[doc(alias = "__ZN3RBX5HUMAN13HumanoidState20maxSwimmingMoveForceEv")]
pub fn stub_8a72e8() -> ! {
    todo!("0x8a72e8 RBX::HUMAN::HumanoidState::maxSwimmingMoveForce(void)")
}

// 0x8a7374 — __ZN3RBX5HUMAN8SwimmingD1Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
#[doc(alias = "RBX::HUMAN::Swimming::~Swimming()")]
#[doc(alias = "__ZN3RBX5HUMAN8SwimmingD1Ev")]
pub fn stub_8a7374() -> ! {
    todo!("0x8a7374 RBX::HUMAN::Swimming::~Swimming()")
}

// 0x8a7378 — __ZN3RBX5HUMAN8SwimmingD0Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
#[doc(alias = "RBX::HUMAN::Swimming::~Swimming()")]
#[doc(alias = "__ZN3RBX5HUMAN8SwimmingD0Ev")]
pub fn stub_8a7378() -> ! {
    todo!("0x8a7378 RBX::HUMAN::Swimming::~Swimming()")
}

// 0x8a7418 — __ZNK3RBX5HUMAN8Swimming12getStateTypeEv
// type: int __fastcall(RBX::HUMAN::Swimming *this)
#[doc(alias = "RBX::HUMAN::Swimming::getStateType(void)const")]
#[doc(alias = "__ZNK3RBX5HUMAN8Swimming12getStateTypeEv")]
pub fn stub_8a7418() -> ! {
    todo!("0x8a7418 RBX::HUMAN::Swimming::getStateType(void)const")
}

// 0x8a741c — __ZThn4_N3RBX5HUMAN8SwimmingD1Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::HUMAN::Swimming::~Swimming()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN8SwimmingD1Ev")]
pub fn stub_8a741c() -> ! {
    todo!("0x8a741c `non-virtual thunk to'RBX::HUMAN::Swimming::~Swimming()")
}

// 0x8a7424 — __ZThn4_N3RBX5HUMAN8SwimmingD0Ev
// type: void __fastcall(RBX::HUMAN::Swimming *__hidden this)
#[doc(alias = "`non-virtual thunk toRBX::HUMAN::Swimming::~Swimming()")]
#[doc(alias = "__ZThn4_N3RBX5HUMAN8SwimmingD0Ev")]
pub fn stub_8a7424() -> ! {
    todo!("0x8a7424 `non-virtual thunk to'RBX::HUMAN::Swimming::~Swimming()")
}

// 0x8a781c — __ZN3RBX7UintSetC1Ev
// type: int __fastcall(int this)
#[doc(alias = "RBX::UintSet::UintSet(void)")]
#[doc(alias = "__ZN3RBX7UintSetC1Ev")]
pub fn stub_8a781c() -> ! {
    todo!("0x8a781c RBX::UintSet::UintSet(void)")
}

// 0x8a7840 — __ZNK3RBX7UintSet4sizeEv
// type: int __fastcall(RBX::UintSet *this)
#[doc(alias = "RBX::UintSet::size(void)const")]
#[doc(alias = "__ZNK3RBX7UintSet4sizeEv")]
pub fn stub_8a7840() -> ! {
    todo!("0x8a7840 RBX::UintSet::size(void)const")
}

// 0x8a7844 — __ZN3RBX7UintSet6insertEj
// type: _DWORD __fastcall(RBX::UintSet *__hidden this, unsigned int)
#[doc(alias = "RBX::UintSet::insert(unsigned int)")]
#[doc(alias = "__ZN3RBX7UintSet6insertEj")]
pub fn stub_8a7844() -> ! {
    todo!("0x8a7844 RBX::UintSet::insert(unsigned int)")
}

// 0x8a7948 — __ZN3RBX7UintSet8containsEj
// type: bool __fastcall(RBX::UintSet *this, unsigned int)
#[doc(alias = "RBX::UintSet::contains(unsigned int)")]
#[doc(alias = "__ZN3RBX7UintSet8containsEj")]
pub fn stub_8a7948() -> ! {
    todo!("0x8a7948 RBX::UintSet::contains(unsigned int)")
}

// 0x8a798c — __ZN3RBX7UintSet12pop_smallestEPj
// type: unsigned int __fastcall(RBX::UintSet *this, unsigned int *, int)
#[doc(alias = "RBX::UintSet::pop_smallest(unsigned int *)")]
#[doc(alias = "__ZN3RBX7UintSet12pop_smallestEPj")]
pub fn stub_8a798c() -> ! {
    todo!("0x8a798c RBX::UintSet::pop_smallest(unsigned int *)")
}

// 0x8a7b90 — __ZN3RBX17DoubleEndedVectorIjE9pop_frontEPj
// type: int __fastcall(int *, _DWORD *, int)
#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::pop_front(unsigned int *)")]
#[doc(alias = "__ZN3RBX17DoubleEndedVectorIjE9pop_frontEPj")]
pub fn stub_8a7b90() -> ! {
    todo!("0x8a7b90 RBX::DoubleEndedVector<unsigned int>::pop_front(unsigned int *)")
}

// 0x8a7c00 — __ZN3RBX17DoubleEndedVectorIjE4growEv
// type: void __fastcall(int *, int, int, int, int, int, int, void *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::DoubleEndedVector<unsigned int>::grow(void)")]
#[doc(alias = "__ZN3RBX17DoubleEndedVectorIjE4growEv")]
pub fn stub_8a7c00() -> ! {
    todo!("0x8a7c00 RBX::DoubleEndedVector<unsigned int>::grow(void)")
}

// 0x8a7e34 — __ZN3RBX17ManualJointHelperD1Ev
// type: void __fastcall(RBX::ManualJointHelper *__hidden this)
#[doc(alias = "RBX::ManualJointHelper::~ManualJointHelper()")]
#[doc(alias = "__ZN3RBX17ManualJointHelperD1Ev")]
pub fn stub_8a7e34() -> ! {
    todo!("0x8a7e34 RBX::ManualJointHelper::~ManualJointHelper()")
}

// 0x8a7e38 — __ZN3RBX17ManualJointHelperD2Ev
// type: void __fastcall(RBX::ManualJointHelper *__hidden this)
#[doc(alias = "RBX::ManualJointHelper::~ManualJointHelper()")]
#[doc(alias = "__ZN3RBX17ManualJointHelperD2Ev")]
pub fn stub_8a7e38() -> ! {
    todo!("0x8a7e38 RBX::ManualJointHelper::~ManualJointHelper()")
}

// 0x8a80e4 — __ZN3RBX17ManualJointHelperC1Ev
// type: int __fastcall(int this)
#[doc(alias = "RBX::ManualJointHelper::ManualJointHelper(void)")]
#[doc(alias = "__ZN3RBX17ManualJointHelperC1Ev")]
pub fn stub_8a80e4() -> ! {
    todo!("0x8a80e4 RBX::ManualJointHelper::ManualJointHelper(void)")
}

// 0x8a8134 — __ZN3RBX17ManualJointHelper31clearAndDeleteJointSurfacePairsEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::ManualJointHelper::clearAndDeleteJointSurfacePairs(void)")]
#[doc(alias = "__ZN3RBX17ManualJointHelper31clearAndDeleteJointSurfacePairsEv")]
pub fn stub_8a8134() -> ! {
    todo!("0x8a8134 RBX::ManualJointHelper::clearAndDeleteJointSurfacePairs(void)")
}

// 0x8a816c — __ZN3RBX17ManualJointHelper32findPermissibleJointSurfacePairsEv
// type: void __fastcall(RBX::ManualJointHelper *this)
#[doc(alias = "RBX::ManualJointHelper::findPermissibleJointSurfacePairs(void)")]
#[doc(alias = "__ZN3RBX17ManualJointHelper32findPermissibleJointSurfacePairsEv")]
pub fn stub_8a816c() -> ! {
    todo!("0x8a816c RBX::ManualJointHelper::findPermissibleJointSurfacePairs(void)")
}

// 0x8a8478 — __ZN3RBX17ManualJointHelper22createJointSurfacePairERNS_9PrimitiveERmS2_S3_
// type: void __fastcall(struct _Unwind_Exception *this, RBX::Primitive *, const RBX::Primitive *, RBX::Primitive *, const RBX::Primitive *)
#[doc(alias = "RBX::ManualJointHelper::createJointSurfacePair(RBX::Primitive &,unsigned long &,RBX::Primitive &,unsigned long &)")]
#[doc(alias = "__ZN3RBX17ManualJointHelper22createJointSurfacePairERNS_9PrimitiveERmS2_S3_")]
pub fn stub_8a8478() -> ! {
    todo!("0x8a8478 RBX::ManualJointHelper::createJointSurfacePair(RBX::Primitive &,unsigned long &,RBX::Primitive &,unsigned long &)")
}

// 0x8a9070 — __ZN3RBX17ManualJointHelper12createJointsEv
// type: int __fastcall(RBX::ManualJointHelper *this)
#[doc(alias = "RBX::ManualJointHelper::createJoints(void)")]
#[doc(alias = "__ZN3RBX17ManualJointHelper12createJointsEv")]
pub fn stub_8a9070() -> ! {
    todo!("0x8a9070 RBX::ManualJointHelper::createJoints(void)")
}

// 0x8a909c — __ZN3RBX17ManualJointHelper28createJointsIfEnabledFromGuiEv
// type: int __fastcall(RBX::ManualJointHelper *this)
#[doc(alias = "RBX::ManualJointHelper::createJointsIfEnabledFromGui(void)")]
#[doc(alias = "__ZN3RBX17ManualJointHelper28createJointsIfEnabledFromGuiEv")]
pub fn stub_8a909c() -> ! {
    todo!("0x8a909c RBX::ManualJointHelper::createJointsIfEnabledFromGui(void)")
}

// 0x8aa3bc — __ZN3RBX22ManualJointSurfacePair11createJointEv
// type: void __fastcall(RBX::ManualJointSurfacePair *this, const RBX::Primitive *)
#[doc(alias = "RBX::ManualJointSurfacePair::createJoint(void)")]
#[doc(alias = "__ZN3RBX22ManualJointSurfacePair11createJointEv")]
pub fn stub_8aa3bc() -> ! {
    todo!("0x8aa3bc RBX::ManualJointSurfacePair::createJoint(void)")
}

// 0x8ab238 — __ZN3RBX29TerrainManualJointSurfacePair11createJointEv
// type: void __fastcall(RBX::TerrainManualJointSurfacePair *this)
#[doc(alias = "RBX::TerrainManualJointSurfacePair::createJoint(void)")]
#[doc(alias = "__ZN3RBX29TerrainManualJointSurfacePair11createJointEv")]
pub fn stub_8ab238() -> ! {
    todo!("0x8ab238 RBX::TerrainManualJointSurfacePair::createJoint(void)")
}

// 0x8ab7bc — __ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::push_back(RBX::ConstraintSurfacePair * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX21ConstraintSurfacePairESaIS2_EE9push_backERKS2_")]
pub fn stub_8ab7bc() -> ! {
    todo!("0x8ab7bc std::vector<RBX::ConstraintSurfacePair *,std::allocator<RBX::ConstraintSurfacePair *>>::push_back(RBX::ConstraintSurfacePair * const&)")
}

// 0x8ab85c — __ZN3RBX24StudAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::StudAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX24StudAutoJointSurfacePairD1Ev")]
pub fn stub_8ab85c() -> ! {
    todo!("0x8ab85c RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")
}

// 0x8ab860 — __ZN3RBX24StudAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::StudAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX24StudAutoJointSurfacePairD0Ev")]
pub fn stub_8ab860() -> ! {
    todo!("0x8ab860 RBX::StudAutoJointSurfacePair::~StudAutoJointSurfacePair()")
}

// 0x8ab900 — __ZN3RBX21ConstraintSurfacePair11createJointEv
// type: void __fastcall(RBX::ConstraintSurfacePair *this)
#[doc(alias = "RBX::ConstraintSurfacePair::createJoint(void)")]
#[doc(alias = "__ZN3RBX21ConstraintSurfacePair11createJointEv")]
pub fn stub_8ab900() -> ! {
    todo!("0x8ab900 RBX::ConstraintSurfacePair::createJoint(void)")
}

// 0x8ab904 — __ZN3RBX24WeldAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::WeldAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX24WeldAutoJointSurfacePairD1Ev")]
pub fn stub_8ab904() -> ! {
    todo!("0x8ab904 RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")
}

// 0x8ab908 — __ZN3RBX24WeldAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::WeldAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX24WeldAutoJointSurfacePairD0Ev")]
pub fn stub_8ab908() -> ! {
    todo!("0x8ab908 RBX::WeldAutoJointSurfacePair::~WeldAutoJointSurfacePair()")
}

// 0x8ab9a8 — __ZN3RBX24GlueAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::GlueAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX24GlueAutoJointSurfacePairD1Ev")]
pub fn stub_8ab9a8() -> ! {
    todo!("0x8ab9a8 RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")
}

// 0x8ab9ac — __ZN3RBX24GlueAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::GlueAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX24GlueAutoJointSurfacePairD0Ev")]
pub fn stub_8ab9ac() -> ! {
    todo!("0x8ab9ac RBX::GlueAutoJointSurfacePair::~GlueAutoJointSurfacePair()")
}

// 0x8aba4c — __ZN3RBX25HingeAutoJointSurfacePairD1Ev
// type: void __fastcall(RBX::HingeAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX25HingeAutoJointSurfacePairD1Ev")]
pub fn stub_8aba4c() -> ! {
    todo!("0x8aba4c RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")
}

// 0x8aba50 — __ZN3RBX25HingeAutoJointSurfacePairD0Ev
// type: void __fastcall(RBX::HingeAutoJointSurfacePair *__hidden this)
#[doc(alias = "RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")]
#[doc(alias = "__ZN3RBX25HingeAutoJointSurfacePairD0Ev")]
pub fn stub_8aba50() -> ! {
    todo!("0x8aba50 RBX::HingeAutoJointSurfacePair::~HingeAutoJointSurfacePair()")
}

// 0x8abaf0 — __ZN3RBX26DisallowedJointSurfacePairD1Ev
// type: void __fastcall(RBX::DisallowedJointSurfacePair *__hidden this)
#[doc(alias = "RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")]
#[doc(alias = "__ZN3RBX26DisallowedJointSurfacePairD1Ev")]
pub fn stub_8abaf0() -> ! {
    todo!("0x8abaf0 RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")
}

// 0x8abaf4 — __ZN3RBX26DisallowedJointSurfacePairD0Ev
// type: void __fastcall(RBX::DisallowedJointSurfacePair *__hidden this)
#[doc(alias = "RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")]
#[doc(alias = "__ZN3RBX26DisallowedJointSurfacePairD0Ev")]
pub fn stub_8abaf4() -> ! {
    todo!("0x8abaf4 RBX::DisallowedJointSurfacePair::~DisallowedJointSurfacePair()")
}

// 0x8abb94 — __ZN3RBX22ManualJointSurfacePairD1Ev
// type: void __fastcall(RBX::ManualJointSurfacePair *__hidden this)
#[doc(alias = "RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")]
#[doc(alias = "__ZN3RBX22ManualJointSurfacePairD1Ev")]
pub fn stub_8abb94() -> ! {
    todo!("0x8abb94 RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")
}

// 0x8abb98 — __ZN3RBX22ManualJointSurfacePairD0Ev
// type: void __fastcall(RBX::ManualJointSurfacePair *__hidden this)
#[doc(alias = "RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")]
#[doc(alias = "__ZN3RBX22ManualJointSurfacePairD0Ev")]
pub fn stub_8abb98() -> ! {
    todo!("0x8abb98 RBX::ManualJointSurfacePair::~ManualJointSurfacePair()")
}
