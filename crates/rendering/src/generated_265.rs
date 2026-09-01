//! rendering shard 265 — 150 stubs EA-sorted asc global gap filler after 0x36f018 not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 28777->28927 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 28777 before -> 28927 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x36f110 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED1Ev
pub fn stub_36f110() -> ! {
    todo!("0x36f110 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")
}

// 0x36f114 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED0Ev
pub fn stub_36f114() -> ! {
    todo!("0x36f114 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")
}

// 0x36f118 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE7disposeEv
pub fn stub_36f118() -> ! {
    todo!("0x36f118 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::dispose(void)")
}

// 0x36f12c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE11get_deleterERKSt9type_info
pub fn stub_36f12c() -> ! {
    todo!("0x36f12c boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_deleter(std::type_info const&)")
}

// 0x36f130 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE19get_untyped_deleterEv
pub fn stub_36f130() -> ! {
    todo!("0x36f130 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_untyped_deleter(void)")
}

// 0x36f134 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
// type: int __fastcall(int, int, int, int, RBX::AsyncHttpQueue *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
pub fn stub_36f134() -> ! {
    todo!("0x36f134 RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")
}

// 0x36f284 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED1Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED1Ev
pub fn stub_36f284() -> ! {
    todo!("0x36f284 RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache()")
}

// 0x36f38c — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED0Ev
pub fn stub_36f38c() -> ! {
    todo!("0x36f38c RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache()")
}

// 0x36f4a4 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
pub fn stub_36f4a4() -> ! {
    todo!("0x36f4a4 RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")
}

// 0x36f694 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
// type: unsigned int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
pub fn stub_36f694() -> ! {
    todo!("0x36f694 RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")
}

// 0x36f6c8 — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
// type: void __fastcall(int, const std::string *, int *, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
pub fn stub_36f6c8() -> ! {
    todo!("0x36f6c8 RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")
}

// 0x36fadc — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::removeLeastRecentlyUsed(void)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv
pub fn stub_36fadc() -> ! {
    todo!("0x36fadc RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::removeLeastRecentlyUsed(void)")
}

// 0x36fb34 — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6removeERKSs
// type: int __fastcall(int, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::remove(std::string const&)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6removeERKSs
pub fn stub_36fb34() -> ! {
    todo!("0x36fb34 RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::remove(std::string const&)")
}

// 0x36fb88 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
pub fn stub_36fb88() -> ! {
    todo!("0x36fb88 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *)")
}

// 0x36fbe4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_36fbe4() -> ! {
    todo!("0x36fbe4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x36fc10 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_36fc10() -> ! {
    todo!("0x36fc10 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x36fc50 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
// type: void __fastcall(int, _DWORD *, std::string *, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
pub fn stub_36fc50() -> ! {
    todo!("0x36fc50 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")
}

// 0x36fe00 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
pub fn stub_36fe00() -> ! {
    todo!("0x36fe00 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")
}

// 0x36fe24 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_36fe24() -> ! {
    todo!("0x36fe24 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x36fe74 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEED2Ev
pub fn stub_36fe74() -> ! {
    todo!("0x36fe74 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::~node_constructor()")
}

// 0x36fe90 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_36fe90() -> ! {
    todo!("0x36fe90 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0x36ffb8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_36ffb8() -> ! {
    todo!("0x36ffb8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0x370048 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_370048() -> ! {
    todo!("0x370048 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0x370074 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
pub fn stub_370074() -> ! {
    todo!("0x370074 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x3700cc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE9constructEv
// type: std::string *__fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE9constructEv
pub fn stub_3700cc() -> ! {
    todo!("0x3700cc boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct(void)")
}

// 0x370108 — __ZNSt4listISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>> const&)")]
// was: __ZNSt4listISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_370108() -> ! {
    todo!("0x370108 std::list<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>> const&)")
}

// 0x3701ec — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::~LRUCache()")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEED2Ev
pub fn stub_3701ec() -> ! {
    todo!("0x3701ec RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::~LRUCache()")
}

// 0x370300 — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
pub fn stub_370300() -> ! {
    todo!("0x370300 RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")
}

// 0x370338 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE8_M_clearEv
pub fn stub_370338() -> ! {
    todo!("0x370338 std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_clear(void)")
}

// 0x370360 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_370360() -> ! {
    todo!("0x370360 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x370398 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_370398() -> ! {
    todo!("0x370398 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x3703cc — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEEC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::LRUCache(void)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEEC2Ev
pub fn stub_3703cc() -> ! {
    todo!("0x3703cc RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::LRUCache(void)")
}

// 0x3704ac — __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
pub fn stub_3704ac() -> ! {
    todo!("0x3704ac RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")
}

// 0x370530 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
pub fn stub_370530() -> ! {
    todo!("0x370530 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> const&)")
}

// 0x3705a0 — __ZN3RBX8LRUCacheISsSsED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
// was: __ZN3RBX8LRUCacheISsSsED2Ev
pub fn stub_3705a0() -> ! {
    todo!("0x3705a0 RBX::LRUCache<std::string,std::string>::~LRUCache()")
}

// 0x3706b4 — __ZN3RBX8LRUCacheISsSsE6resizeEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::resize(unsigned long)")]
// was: __ZN3RBX8LRUCacheISsSsE6resizeEm
pub fn stub_3706b4() -> ! {
    todo!("0x3706b4 RBX::LRUCache<std::string,std::string>::resize(unsigned long)")
}

// 0x3706ec — __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
pub fn stub_3706ec() -> ! {
    todo!("0x3706ec std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")
}

// 0x370714 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_370714() -> ! {
    todo!("0x370714 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x37074c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_37074c() -> ! {
    todo!("0x37074c boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x370780 — __ZN3RBX8LRUCacheISsSsEC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
// was: __ZN3RBX8LRUCacheISsSsEC2Ev
pub fn stub_370780() -> ! {
    todo!("0x370780 RBX::LRUCache<std::string,std::string>::LRUCache(void)")
}

// 0x370860 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm
pub fn stub_370860() -> ! {
    todo!("0x370860 RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")
}

// 0x3708e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
pub fn stub_3708e4() -> ! {
    todo!("0x3708e4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")
}

// 0x370950 — __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_370950() -> ! {
    todo!("0x370950 __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x370954 — __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_370954() -> ! {
    todo!("0x370954 __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3709f4 — __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3709f4() -> ! {
    todo!("0x3709f4 __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3709fc — __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3709fc() -> ! {
    todo!("0x3709fc __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x370aa0 — __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_370aa0() -> ! {
    todo!("0x370aa0 __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x370aa8 — __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_370aa8() -> ! {
    todo!("0x370aa8 __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x370b4c — __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::BoundFuncDesc(void (RBX::ScriptInformationProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_370b4c() -> ! {
    todo!("0x370b4c RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::BoundFuncDesc(void (RBX::ScriptInformationProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x370cc8 — __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_370cc8() -> ! {
    todo!("0x370cc8 RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x370cf8 — __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EED0Ev
pub fn stub_370cf8() -> ! {
    todo!("0x370cf8 RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::~BoundFuncDesc()")
}

// 0x370e00 — __ZNK3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_370e00() -> ! {
    todo!("0x370e00 RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x370f3c — __ZN3RBX10Reflection11Call1HelperINS_25ScriptInformationProviderEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ScriptInformationProvider,void (RBX::ScriptInformationProvider::*)(std::string),std::string,void>::call(RBX::ScriptInformationProvider*,void (RBX::ScriptInformationProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_25ScriptInformationProviderEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
pub fn stub_370f3c() -> ! {
    todo!("0x370f3c RBX::Reflection::Call1Helper<RBX::ScriptInformationProvider,void (RBX::ScriptInformationProvider::*)(std::string),std::string,void>::call(RBX::ScriptInformationProvider*,void (RBX::ScriptInformationProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&)")
}

// 0x371070 — __ZN3RBX25ScriptInformationProviderD2Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZN3RBX25ScriptInformationProviderD2Ev
pub fn stub_371070() -> ! {
    todo!("0x371070 RBX::ScriptInformationProvider::~ScriptInformationProvider()")
}

// 0x371220 — __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::clear(void)")]
// was: __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE5clearEv
pub fn stub_371220() -> ! {
    todo!("0x371220 boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::clear(void)")
}

// 0x371250 — __ZN3RBX17HeartbeatInstanceD1Ev
// type: void __fastcall(RBX::HeartbeatInstance *__hidden this)
#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
// was: __ZN3RBX17HeartbeatInstanceD1Ev
pub fn stub_371250() -> ! {
    todo!("0x371250 RBX::HeartbeatInstance::~HeartbeatInstance()")
}

// 0x371254 — __GLOBAL__I_a_137
#[doc(alias = "global constructor keyed to_a_137")]
// was: __GLOBAL__I_a_137
pub fn stub_371254() -> ! {
    todo!("0x371254 global constructor keyed to_a_137")
}

// 0x37148c — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEEC2Ev
pub fn stub_37148c() -> ! {
    todo!("0x37148c RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::EnumDesc(void)")
}

// 0x371844 — __Z18checkResultNoThrow11FMOD_RESULT
// type: int __fastcall(unsigned int, int, int, int)
#[doc(alias = "checkResultNoThrow(FMOD_RESULT)")]
// was: __Z18checkResultNoThrow11FMOD_RESULT
pub fn stub_371844() -> ! {
    todo!("0x371844 checkResultNoThrow(FMOD_RESULT)")
}

// 0x3719d0 — __Z11checkResult11FMOD_RESULT
// type: void __fastcall(unsigned int)
#[doc(alias = "checkResult(FMOD_RESULT)")]
// was: __Z11checkResult11FMOD_RESULT
pub fn stub_3719d0() -> ! {
    todo!("0x3719d0 checkResult(FMOD_RESULT)")
}

// 0x371b5c — __ZN3RBX10Soundscape12SoundServiceC1Ev
// type: int __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
// was: __ZN3RBX10Soundscape12SoundServiceC1Ev
pub fn stub_371b5c() -> ! {
    todo!("0x371b5c RBX::Soundscape::SoundService::SoundService(void)")
}

// 0x371b60 — __ZN3RBX10Soundscape12SoundServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
// was: __ZN3RBX10Soundscape12SoundServiceC2Ev
pub fn stub_371b60() -> ! {
    todo!("0x371b60 RBX::Soundscape::SoundService::SoundService(void)")
}

// 0x371e5c — __ZN3RBX10Soundscape12SoundService8openFmodEv
// type: int __fastcall(RBX::Soundscape::SoundService *this, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::openFmod(void)")]
// was: __ZN3RBX10Soundscape12SoundService8openFmodEv
pub fn stub_371e5c() -> ! {
    todo!("0x371e5c RBX::Soundscape::SoundService::openFmod(void)")
}

// 0x3723f4 — __ZN3RBX10Soundscape12SoundService16update3DSettingsEv
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::update3DSettings(void)")]
// was: __ZN3RBX10Soundscape12SoundService16update3DSettingsEv
pub fn stub_3723f4() -> ! {
    todo!("0x3723f4 RBX::Soundscape::SoundService::update3DSettings(void)")
}

// 0x372414 — __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::updateAmbientReverb(void)")]
// was: __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv
pub fn stub_372414() -> ! {
    todo!("0x372414 RBX::Soundscape::SoundService::updateAmbientReverb(void)")
}

// 0x372460 — __ZN3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// was: __ZN3RBX10Soundscape12SoundServiceD0Ev
pub fn stub_372460() -> ! {
    todo!("0x372460 RBX::Soundscape::SoundService::~SoundService()")
}

// 0x372500 — __ZN3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// was: __ZN3RBX10Soundscape12SoundServiceD1Ev
pub fn stub_372500() -> ! {
    todo!("0x372500 RBX::Soundscape::SoundService::~SoundService()")
}

// 0x372504 — __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev
pub fn stub_372504() -> ! {
    todo!("0x372504 non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x37250c — __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev
pub fn stub_37250c() -> ! {
    todo!("0x37250c non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x372514 — __ZN3RBX10Soundscape12SoundServiceD2Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// was: __ZN3RBX10Soundscape12SoundServiceD2Ev
pub fn stub_372514() -> ! {
    todo!("0x372514 RBX::Soundscape::SoundService::~SoundService()")
}

// 0x3728b0 — __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev
pub fn stub_3728b0() -> ! {
    todo!("0x3728b0 non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x3728b8 — __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev
pub fn stub_3728b8() -> ! {
    todo!("0x3728b8 non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x3728c0 — __ZN3RBX10Soundscape12SoundService9closeFmodEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::closeFmod(void)")]
// was: __ZN3RBX10Soundscape12SoundService9closeFmodEv
pub fn stub_3728c0() -> ! {
    todo!("0x3728c0 RBX::Soundscape::SoundService::closeFmod(void)")
}

// 0x3729bc — __ZL12releaseSoundRKSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEE
// type: int __fastcall(int)
#[doc(alias = "releaseSound(std::pair<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")]
// was: __ZL12releaseSoundRKSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEE
pub fn stub_3729bc() -> ! {
    todo!("0x3729bc releaseSound(std::pair<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>> const&)")
}

// 0x3729c4 — __ZL11initReverbsv
// type: void *__fastcall()
#[doc(alias = "initReverbs(void)")]
// was: __ZL11initReverbsv
pub fn stub_3729c4() -> ! {
    todo!("0x3729c4 initReverbs(void)")
}

// 0x372bb0 — __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::loadStockSounds(void)")]
// was: __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv
pub fn stub_372bb0() -> ! {
    todo!("0x372bb0 RBX::Soundscape::SoundService::loadStockSounds(void)")
}

// 0x373554 — __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs
// type: void __fastcall(RBX::Instance *, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType,std::string)")]
// was: __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs
pub fn stub_373554() -> ! {
    todo!("0x373554 RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType,std::string)")
}

// 0x37384c — __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE
// type: int __fastcall(_DWORD *, std::string *)
#[doc(alias = "RBX::Soundscape::SoundChannel::setSoundId(RBX::Soundscape::SoundId)")]
// was: __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE
pub fn stub_37384c() -> ! {
    todo!("0x37384c RBX::Soundscape::SoundChannel::setSoundId(RBX::Soundscape::SoundId)")
}

// 0x373894 — __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE
// type: RBX::Soundscape::SoundId *__fastcall(RBX::Soundscape::SoundId *this, const RBX::ContentId *)
#[doc(alias = "RBX::Soundscape::SoundId::SoundId(RBX::ContentId const&)")]
// was: __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE
pub fn stub_373894() -> ! {
    todo!("0x373894 RBX::Soundscape::SoundId::SoundId(RBX::ContentId const&)")
}

// 0x3738a8 — __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE
// type: int __fastcall(RBX::Instance *, int *)
#[doc(alias = "RBX::Soundscape::SoundService::setAmbientReverb(RBX::Soundscape::ReverbType const&)")]
// was: __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE
pub fn stub_3738a8() -> ! {
    todo!("0x3738a8 RBX::Soundscape::SoundService::setAmbientReverb(RBX::Soundscape::ReverbType const&)")
}

// 0x3738d8 — __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::Soundscape::SoundService::playSound(RBX::SoundType)")]
// was: __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE
pub fn stub_3738d8() -> ! {
    todo!("0x3738d8 RBX::Soundscape::SoundService::playSound(RBX::SoundType)")
}

// 0x373918 — __ZN3RBX10Soundscape12SoundChannel4playEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::play(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel4playEv
pub fn stub_373918() -> ! {
    todo!("0x373918 RBX::Soundscape::SoundChannel::play(void)")
}

// 0x373974 — __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(shared_count *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_
pub fn stub_373974() -> ! {
    todo!("0x373974 RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x373cb8 — __ZN3RBX10Soundscape12SoundService4stepEv
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int, int (*)(const char *, ...))
#[doc(alias = "RBX::Soundscape::SoundService::step(void)")]
// was: __ZN3RBX10Soundscape12SoundService4stepEv
pub fn stub_373cb8() -> ! {
    todo!("0x373cb8 RBX::Soundscape::SoundService::step(void)")
}

// 0x373fd0 — __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::garbageCollectSounds(void)")]
// was: __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv
pub fn stub_373fd0() -> ! {
    todo!("0x373fd0 RBX::Soundscape::SoundService::garbageCollectSounds(void)")
}

// 0x374028 — __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_
// type: int __fastcall(std::string *, std::string *)
#[doc(alias = "RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&,RBX::Soundscape::SoundId&)")]
// was: __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_
pub fn stub_374028() -> ! {
    todo!("0x374028 RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&,RBX::Soundscape::SoundId&)")
}

// 0x37414c — __ZN3RBX10Reflection4Type12getSingletonINS_10Soundscape7SoundIdEEERKS1_v
// type: int()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Soundscape::SoundId>(void)")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_10Soundscape7SoundIdEEERKS1_v
pub fn stub_37414c() -> ! {
    todo!("0x37414c RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Soundscape::SoundId>(void)")
}

// 0x374154 — __ZN3RBX10Reflection7Variant7convertINS_10Soundscape7SoundIdEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::convert<RBX::Soundscape::SoundId>(void)")]
// was: __ZN3RBX10Reflection7Variant7convertINS_10Soundscape7SoundIdEEERT_v
pub fn stub_374154() -> ! {
    todo!("0x374154 RBX::Soundscape::SoundId & RBX::Reflection::Variant::convert<RBX::Soundscape::SoundId>(void)")
}

// 0x374340 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
pub fn stub_374340() -> ! {
    todo!("0x374340 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x374528 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
pub fn stub_374528() -> ! {
    todo!("0x374528 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x374758 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11getDataSizeEPKNS0_13DescribedBaseE
pub fn stub_374758() -> ! {
    todo!("0x374758 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getDataSize(RBX::Reflection::DescribedBase const*)const")
}

// 0x3747b4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14hasStringValueEv
pub fn stub_3747b4() -> ! {
    todo!("0x3747b4 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::hasStringValue(void)const")
}

// 0x3747b8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14getStringValueEPKNS0_13DescribedBaseE
// type: void __fastcall(std::string *, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14getStringValueEPKNS0_13DescribedBaseE
pub fn stub_3747b8() -> ! {
    todo!("0x3747b8 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3748d4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(RBX::Name *, int, std::string *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14setStringValueEPNS0_13DescribedBaseERKSs
pub fn stub_3748d4() -> ! {
    todo!("0x3748d4 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x374a2c — __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::getSoundId(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv
pub fn stub_374a2c() -> ! {
    todo!("0x374a2c RBX::Soundscape::SoundChannel::getSoundId(void)const")
}

// 0x374a44 — __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getVolume(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv
pub fn stub_374a44() -> ! {
    todo!("0x374a44 RBX::Soundscape::SoundChannel::getVolume(void)const")
}

// 0x374a48 — __ZN3RBX10Soundscape12SoundChannel9setVolumeEf
// type: int __fastcall(int this, float32_t, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::setVolume(float)")]
// was: __ZN3RBX10Soundscape12SoundChannel9setVolumeEf
pub fn stub_374a48() -> ! {
    todo!("0x374a48 RBX::Soundscape::SoundChannel::setVolume(float)")
}

// 0x374aa4 — __ZNK3RBX10Soundscape12SoundChannel8getPitchEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getPitch(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel8getPitchEv
pub fn stub_374aa4() -> ! {
    todo!("0x374aa4 RBX::Soundscape::SoundChannel::getPitch(void)const")
}

// 0x374aa8 — __ZN3RBX10Soundscape12SoundChannel8setPitchEf
// type: int __fastcall(int this, float, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::setPitch(float)")]
// was: __ZN3RBX10Soundscape12SoundChannel8setPitchEf
pub fn stub_374aa8() -> ! {
    todo!("0x374aa8 RBX::Soundscape::SoundChannel::setPitch(float)")
}

// 0x374af8 — __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::setPlayCount(int)")]
// was: __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi
pub fn stub_374af8() -> ! {
    todo!("0x374af8 RBX::Soundscape::SoundChannel::setPlayCount(int)")
}

// 0x374b68 — __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getLooped(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv
pub fn stub_374b68() -> ! {
    todo!("0x374b68 RBX::Soundscape::SoundChannel::getLooped(void)const")
}

// 0x374b74 — __ZN3RBX10Soundscape12SoundChannel9setLoopedEb
// type: unsigned int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::setLooped(bool)")]
// was: __ZN3RBX10Soundscape12SoundChannel9setLoopedEb
pub fn stub_374b74() -> ! {
    todo!("0x374b74 RBX::Soundscape::SoundChannel::setLooped(bool)")
}

// 0x374bb4 — __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::isPlaying(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv
pub fn stub_374bb4() -> ! {
    todo!("0x374bb4 RBX::Soundscape::SoundChannel::isPlaying(void)const")
}

// 0x374bec — __ZNK3RBX10Soundscape12SoundChannel8isPausedEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::isPaused(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel8isPausedEv
pub fn stub_374bec() -> ! {
    todo!("0x374bec RBX::Soundscape::SoundChannel::isPaused(void)const")
}

// 0x374c24 — __ZN3RBX10Soundscape12SoundChannel5pauseEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::pause(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel5pauseEv
pub fn stub_374c24() -> ! {
    todo!("0x374c24 RBX::Soundscape::SoundChannel::pause(void)")
}

// 0x374c68 — __ZN3RBX10Soundscape12SoundChannel4stopEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::stop(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel4stopEv
pub fn stub_374c68() -> ! {
    todo!("0x374c68 RBX::Soundscape::SoundChannel::stop(void)")
}

// 0x374cc4 — __ZN3RBX10Soundscape12SoundChannelC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::SoundChannel(void)")]
// was: __ZN3RBX10Soundscape12SoundChannelC2Ev
pub fn stub_374cc4() -> ! {
    todo!("0x374cc4 RBX::Soundscape::SoundChannel::SoundChannel(void)")
}

// 0x374ff4 — __ZN3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZN3RBX10Soundscape12SoundChannelD0Ev
pub fn stub_374ff4() -> ! {
    todo!("0x374ff4 RBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375094 — __ZN3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZN3RBX10Soundscape12SoundChannelD1Ev
pub fn stub_375094() -> ! {
    todo!("0x375094 RBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375098 — __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev
pub fn stub_375098() -> ! {
    todo!("0x375098 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x3750a0 — __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev
pub fn stub_3750a0() -> ! {
    todo!("0x3750a0 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x3750a8 — __ZN3RBX10Soundscape12SoundChannelD2Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZN3RBX10Soundscape12SoundChannelD2Ev
pub fn stub_3750a8() -> ! {
    todo!("0x3750a8 RBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375330 — __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev
pub fn stub_375330() -> ! {
    todo!("0x375330 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375338 — __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev
pub fn stub_375338() -> ! {
    todo!("0x375338 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375340 — __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE
// type: int __fastcall(int, float *)
#[doc(alias = "RBX::Soundscape::SoundService::getCpuStats(RBX::Soundscape::SoundService::CpuStats &)const")]
// was: __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE
pub fn stub_375340() -> ! {
    todo!("0x375340 RBX::Soundscape::SoundService::getCpuStats(RBX::Soundscape::SoundService::CpuStats &)const")
}

// 0x3753e8 — __ZN3RBX10Soundscape12SoundService13getSoundStatsERKSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEERjSH_
// type: const _Rb_tree_node_base *__fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "RBX::Soundscape::SoundService::getSoundStats(std::map<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>> const&,unsigned int &,unsigned int &)")]
// was: __ZN3RBX10Soundscape12SoundService13getSoundStatsERKSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEERjSH_
pub fn stub_3753e8() -> ! {
    todo!("0x3753e8 RBX::Soundscape::SoundService::getSoundStats(std::map<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>> const&,unsigned int &,unsigned int &)")
}

// 0x375418 — __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, int *)
#[doc(alias = "RBX::Soundscape::SoundService::getChannelsPlaying(int &)const")]
// was: __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi
pub fn stub_375418() -> ! {
    todo!("0x375418 RBX::Soundscape::SoundService::getChannelsPlaying(int &)const")
}

// 0x375438 — __ZN3RBX10Soundscape12SoundService8gcSoundsERSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEE
// type: int __fastcall(int result)
#[doc(alias = "RBX::Soundscape::SoundService::gcSounds(std::map<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>> &)")]
// was: __ZN3RBX10Soundscape12SoundService8gcSoundsERSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEE
pub fn stub_375438() -> ! {
    todo!("0x375438 RBX::Soundscape::SoundService::gcSounds(std::map<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>> &)")
}

// 0x3754c4 — __ZN3RBX10Soundscape5Sound7releaseEv
// type: FMOD::Sound *__fastcall(FMOD::Sound **this)
#[doc(alias = "RBX::Soundscape::Sound::release(void)")]
// was: __ZN3RBX10Soundscape5Sound7releaseEv
pub fn stub_3754c4() -> ! {
    todo!("0x3754c4 RBX::Soundscape::Sound::release(void)")
}

// 0x3754e0 — __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::releaseChannel(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv
pub fn stub_3754e0() -> ! {
    todo!("0x3754e0 RBX::Soundscape::SoundChannel::releaseChannel(void)")
}

// 0x37551c — __ZNK3RBX10Soundscape12SoundChannel12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel12askSetParentEPKNS_8InstanceE
pub fn stub_37551c() -> ! {
    todo!("0x37551c RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")
}

// 0x375520 — __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv
// type: void __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::updateListenState(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv
pub fn stub_375520() -> ! {
    todo!("0x375520 RBX::Soundscape::SoundChannel::updateListenState(void)")
}

// 0x375660 — __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(FMOD::Channel **, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&)")]
// was: __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE
pub fn stub_375660() -> ! {
    todo!("0x375660 RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x37567c — __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_37567c() -> ! {
    todo!("0x37567c RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x375744 — __ZN3RBX10Soundscape12SoundChannel9playSoundEPKNS_8InstanceE
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")]
// was: __ZN3RBX10Soundscape12SoundChannel9playSoundEPKNS_8InstanceE
pub fn stub_375744() -> ! {
    todo!("0x375744 RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")
}

// 0x375b7c — __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_
pub fn stub_375b7c() -> ! {
    todo!("0x375b7c RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x375be0 — __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::preloadSound(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv
pub fn stub_375be0() -> ! {
    todo!("0x375be0 RBX::Soundscape::SoundChannel::preloadSound(void)")
}

// 0x375c3c — __ZN3RBX10Soundscape12SoundChannel8update3DEPN4FMOD7ChannelE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, FMOD::Channel *)
#[doc(alias = "RBX::Soundscape::SoundChannel::update3D(FMOD::Channel *)")]
// was: __ZN3RBX10Soundscape12SoundChannel8update3DEPN4FMOD7ChannelE
pub fn stub_375c3c() -> ! {
    todo!("0x375c3c RBX::Soundscape::SoundChannel::update3D(FMOD::Channel *)")
}

// 0x375c8c — __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv
// type: FMOD::Channel *__fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::updateLooped(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv
pub fn stub_375c8c() -> ! {
    todo!("0x375c8c RBX::Soundscape::SoundChannel::updateLooped(void)")
}

// 0x375ce8 — __Z18callbackChannelEndP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS2_
// type: int __fastcall(int, int)
#[doc(alias = "callbackChannelEnd(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *)")]
// was: __Z18callbackChannelEndP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS2_
pub fn stub_375ce8() -> ! {
    todo!("0x375ce8 callbackChannelEnd(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *)")
}

// 0x375d0c — __ZNK3RBX10Soundscape12SoundChannel14isHeardLocallyEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel14isHeardLocallyEPKNS_8InstanceE
pub fn stub_375d0c() -> ! {
    todo!("0x375d0c RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")
}

// 0x375dd4 — __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb
// type: void __fastcall(sp_counted_base **, const shared_count *, const std::string *, int)
#[doc(alias = "RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId,bool)")]
// was: __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb
pub fn stub_375dd4() -> ! {
    todo!("0x375dd4 RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId,bool)")
}

// 0x376004 — __ZN3RBX10Soundscape5Sound3getEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::Sound *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::Sound::get(RBX::Instance const*)")]
// was: __ZN3RBX10Soundscape5Sound3getEPKNS_8InstanceE
pub fn stub_376004() -> ! {
    todo!("0x376004 RBX::Soundscape::Sound::get(RBX::Instance const*)")
}

// 0x376198 — __ZN3RBX13registerSoundEv
// type: int __fastcall(RBX *this)
#[doc(alias = "RBX::registerSound(void)")]
// was: __ZN3RBX13registerSoundEv
pub fn stub_376198() -> ! {
    todo!("0x376198 RBX::registerSound(void)")
}

// 0x37619c — __ZN3RBX10Soundscape5SoundD2Ev
// type: void __fastcall(FMOD::Sound **this)
#[doc(alias = "RBX::Soundscape::Sound::~Sound()")]
// was: __ZN3RBX10Soundscape5SoundD2Ev
pub fn stub_37619c() -> ! {
    todo!("0x37619c RBX::Soundscape::Sound::~Sound()")
}

// 0x376244 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc
pub fn stub_376244() -> ! {
    todo!("0x376244 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")
}

// 0x3765a4 — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
// type: void (__fastcall *__fastcall(_Rb_tree_node_base *, _Rb_tree_node_base *, void (__fastcall *)(_DWORD *), int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int))(_DWORD *)
#[doc(alias = "void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>))")]
// was: __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
pub fn stub_3765a4() -> ! {
    todo!("0x3765a4 void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,boost::shared_ptr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,boost::shared_ptr<RBX::Soundscape::Sound>>))")
}

// 0x37677c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(__guard *)
#[doc(alias = "boost::shared_ptr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
pub fn stub_37677c() -> ! {
    todo!("0x37677c boost::shared_ptr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")
}

// 0x3768dc — __ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
// type: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::SoundType,boost::shared_ptr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")]
// was: __ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
pub fn stub_3768dc() -> ! {
    todo!("0x3768dc std::map<RBX::SoundType,boost::shared_ptr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,boost::shared_ptr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")
}

// 0x376a24 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundChannel>& boost::shared_ptr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(boost::shared_ptr<RBX::StockSound> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
pub fn stub_376a24() -> ! {
    todo!("0x376a24 boost::shared_ptr<RBX::Soundscape::SoundChannel>& boost::shared_ptr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(boost::shared_ptr<RBX::StockSound> const&)")
}

// 0x376a58 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::operator=(boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
pub fn stub_376a58() -> ! {
    todo!("0x376a58 boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob>::operator=(boost::shared_ptr<RBX::Soundscape::SoundService::SoundJob> const&)")
}

// 0x376a90 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
pub fn stub_376a90() -> ! {
    todo!("0x376a90 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const&)")
}

// 0x376ac4 — __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(RBX::Stats::Item **this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
// was: __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
pub fn stub_376ac4() -> ! {
    todo!("0x376ac4 SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")
}

// 0x376c84 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), const std::string *))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
pub fn stub_376c84() -> ! {
    todo!("0x376c84 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")
}

// 0x376ce4 — __ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
pub fn stub_376ce4() -> ! {
    todo!("0x376ce4 RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")
}

// 0x376f90 — __ZN3RBX10Soundscape12SoundService18on3DSettingChangedERKNS_10Reflection18PropertyDescriptorE
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX10Soundscape12SoundService18on3DSettingChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_376f90() -> ! {
    todo!("0x376f90 RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x376f94 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED1Ev
pub fn stub_376f94() -> ! {
    todo!("0x376f94 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")
}

// 0x376fb8 — __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
// type: int __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::getAmbientReverb(void)const")]
// was: __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
pub fn stub_376fb8() -> ! {
    todo!("0x376fb8 RBX::Soundscape::SoundService::getAmbientReverb(void)const")
}

// 0x376fc0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED1Ev
pub fn stub_376fc0() -> ! {
    todo!("0x376fc0 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")
}

// 0x376fe4 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED1Ev
pub fn stub_376fe4() -> ! {
    todo!("0x376fe4 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")
}