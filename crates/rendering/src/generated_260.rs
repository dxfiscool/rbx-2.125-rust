//! rendering shard 260 — 100 stubs EA-sorted asc global gap filler after 0x352ce4 not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 28220->28320 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x352dc8 — __ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv
#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::expired(void)const")]
// was: __ZNK5boost8weak_ptrIN3RBX14AsyncHttpQueueEE7expiredEv
pub fn stub_352dc8() -> ! {
    todo!("0x352dc8 rbx_core::WeakPtr<RBX::AsyncHttpQueue>::expired(void)const")
}

// 0x352e1c — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEEPT_
pub fn stub_352e1c() -> ! {
    todo!("0x352e1c boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")
}

// 0x352f14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED1Ev
pub fn stub_352f14() -> ! {
    todo!("0x352f14 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")
}

// 0x352f18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED0Ev
pub fn stub_352f18() -> ! {
    todo!("0x352f18 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")
}

// 0x352f1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE7disposeEv
pub fn stub_352f1c() -> ! {
    todo!("0x352f1c boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::dispose(void)")
}

// 0x352f30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
pub fn stub_352f30() -> ! {
    todo!("0x352f30 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")
}

// 0x352f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
pub fn stub_352f34() -> ! {
    todo!("0x352f34 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_untyped_deleter(void)")
}

// 0x352f38 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
// type: int __fastcall(int, int, int, int, RBX::AsyncHttpQueue *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
pub fn stub_352f38() -> ! {
    todo!("0x352f38 RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")
}

// 0x353088 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED1Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED1Ev
pub fn stub_353088() -> ! {
    todo!("0x353088 RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")
}

// 0x353190 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED0Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED0Ev
pub fn stub_353190() -> ! {
    todo!("0x353190 RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")
}

// 0x3532a8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
pub fn stub_3532a8() -> ! {
    todo!("0x3532a8 RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x353554 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
// type: unsigned int __fastcall(int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_353554() -> ! {
    todo!("0x353554 RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")
}

// 0x353588 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_353588() -> ! {
    todo!("0x353588 RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")
}

// 0x353b10 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed(void)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv
pub fn stub_353b10() -> ! {
    todo!("0x353b10 RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed(void)")
}

// 0x353b68 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6removeERKSs
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::remove(std::string const&)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6removeERKSs
pub fn stub_353b68() -> ! {
    todo!("0x353b68 RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::remove(std::string const&)")
}

// 0x353bbc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
pub fn stub_353bbc() -> ! {
    todo!("0x353bbc boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *)")
}

// 0x353c18 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_353c18() -> ! {
    todo!("0x353c18 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x353c44 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_353c44() -> ! {
    todo!("0x353c44 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x353c84 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEE7destroyEPS6_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>::destroy(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>*)")]
// was: __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEE7destroyEPS6_
pub fn stub_353c84() -> ! {
    todo!("0x353c84 __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>::destroy(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>*)")
}

// 0x353d3c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
pub fn stub_353d3c() -> ! {
    todo!("0x353d3c std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")
}

// 0x353eec — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
pub fn stub_353eec() -> ! {
    todo!("0x353eec void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")
}

// 0x353f10 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_353f10() -> ! {
    todo!("0x353f10 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x353f60 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEED2Ev
pub fn stub_353f60() -> ! {
    todo!("0x353f60 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::~node_constructor()")
}

// 0x353f80 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_353f80() -> ! {
    todo!("0x353f80 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0x3540a8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_3540a8() -> ! {
    todo!("0x3540a8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0x354138 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_354138() -> ! {
    todo!("0x354138 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0x354164 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
pub fn stub_354164() -> ! {
    todo!("0x354164 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x3541bc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE9constructEv
pub fn stub_3541bc() -> ! {
    todo!("0x3541bc boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct(void)")
}

// 0x3541f8 — __ZNSt4pairISsS_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEC2ERKSsRKS3_
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo> const&)")]
// was: __ZNSt4pairISsS_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEC2ERKSsRKS3_
pub fn stub_3541f8() -> ! {
    todo!("0x3541f8 std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo> const&)")
}

// 0x3542c4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>> const&)")]
// was: __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_3542c4() -> ! {
    todo!("0x3542c4 std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>> const&)")
}

// 0x3543d4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEED2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::~LRUCache()")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEED2Ev
pub fn stub_3543d4() -> ! {
    todo!("0x3543d4 RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::~LRUCache()")
}

// 0x3544e8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
pub fn stub_3544e8() -> ! {
    todo!("0x3544e8 RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")
}

// 0x354520 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
pub fn stub_354520() -> ! {
    todo!("0x354520 std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_clear(void)")
}

// 0x354548 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_354548() -> ! {
    todo!("0x354548 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x354580 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_354580() -> ! {
    todo!("0x354580 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x3545b4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::LRUCache(void)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEEC2Ev
pub fn stub_3545b4() -> ! {
    todo!("0x3545b4 RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::LRUCache(void)")
}

// 0x354694 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
pub fn stub_354694() -> ! {
    todo!("0x354694 RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")
}

// 0x354718 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
pub fn stub_354718() -> ! {
    todo!("0x354718 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> const&)")
}

// 0x354784 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
// was: __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
pub fn stub_354784() -> ! {
    todo!("0x354784 rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")
}

// 0x35486c — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_35486c() -> ! {
    todo!("0x35486c void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const")
}

// 0x354950 — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEEPT_
pub fn stub_354950() -> ! {
    todo!("0x354950 boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")
}

// 0x354a48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED1Ev
pub fn stub_354a48() -> ! {
    todo!("0x354a48 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")
}

// 0x354a4c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED0Ev
pub fn stub_354a4c() -> ! {
    todo!("0x354a4c boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")
}

// 0x354a50 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE7disposeEv
pub fn stub_354a50() -> ! {
    todo!("0x354a50 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::dispose(void)")
}

// 0x354a60 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
pub fn stub_354a60() -> ! {
    todo!("0x354a60 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")
}

// 0x354a64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
pub fn stub_354a64() -> ! {
    todo!("0x354a64 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_untyped_deleter(void)")
}

// 0x354a68 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
// type: int __fastcall(int, int, int, int, RBX::AsyncHttpQueue *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
pub fn stub_354a68() -> ! {
    todo!("0x354a68 RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")
}

// 0x354bb8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED1Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED1Ev
pub fn stub_354bb8() -> ! {
    todo!("0x354bb8 RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")
}

// 0x354cc0 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED0Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED0Ev
pub fn stub_354cc0() -> ! {
    todo!("0x354cc0 RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")
}

// 0x354dd8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
pub fn stub_354dd8() -> ! {
    todo!("0x354dd8 RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x355034 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_355034() -> ! {
    todo!("0x355034 RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")
}

// 0x3550a8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
pub fn stub_3550a8() -> ! {
    todo!("0x3550a8 RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")
}

// 0x35561c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
pub fn stub_35561c() -> ! {
    todo!("0x35561c boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *)")
}

// 0x355678 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_355678() -> ! {
    todo!("0x355678 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x3556a4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_3556a4() -> ! {
    todo!("0x3556a4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x3556e4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
// type: int __fastcall(int, std::_List_node_base *this, int, int, int, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>)")]
// was: __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
pub fn stub_3556e4() -> ! {
    todo!("0x3556e4 std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>)")
}

// 0x3557bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
pub fn stub_3557bc() -> ! {
    todo!("0x3557bc std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")
}

// 0x355974 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
pub fn stub_355974() -> ! {
    todo!("0x355974 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")
}

// 0x355998 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_355998() -> ! {
    todo!("0x355998 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x3559e8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEED2Ev
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEED2Ev
pub fn stub_3559e8() -> ! {
    todo!("0x3559e8 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::~node_constructor()")
}

// 0x355a08 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_355a08() -> ! {
    todo!("0x355a08 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0x355b30 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_355b30() -> ! {
    todo!("0x355b30 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0x355bc0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_355bc0() -> ! {
    todo!("0x355bc0 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0x355bec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
pub fn stub_355bec() -> ! {
    todo!("0x355bec boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x355c44 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE9constructEv
pub fn stub_355c44() -> ! {
    todo!("0x355c44 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct(void)")
}

// 0x355c80 — __ZNSt4pairISsS_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEC2ERKSsRKS3_
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo> const&)")]
// was: __ZNSt4pairISsS_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEC2ERKSsRKS3_
pub fn stub_355c80() -> ! {
    todo!("0x355c80 std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo> const&)")
}

// 0x355d60 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>> const&)")]
// was: __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_355d60() -> ! {
    todo!("0x355d60 std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>> const&)")
}

// 0x355e88 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEED2Ev
// type: int __fastcall(std::string *, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::~LRUCache()")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEED2Ev
pub fn stub_355e88() -> ! {
    todo!("0x355e88 RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::~LRUCache()")
}

// 0x355f9c — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
pub fn stub_355f9c() -> ! {
    todo!("0x355f9c RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")
}

// 0x356010 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, std::string *, int, int, int, int)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
pub fn stub_356010() -> ! {
    todo!("0x356010 std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_clear(void)")
}

// 0x3560f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_3560f8() -> ! {
    todo!("0x3560f8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x356130 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_356130() -> ! {
    todo!("0x356130 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x356164 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::LRUCache(void)")]
// was: __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEEC2Ev
pub fn stub_356164() -> ! {
    todo!("0x356164 RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::LRUCache(void)")
}

// 0x356244 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
pub fn stub_356244() -> ! {
    todo!("0x356244 RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")
}

// 0x3562bc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
pub fn stub_3562bc() -> ! {
    todo!("0x3562bc boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> const&)")
}

// 0x356328 — __ZN5boost6detail8function15functor_managerIPFbRKSsPSsEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<bool (*)(std::string const&,std::string *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerIPFbRKSsPSsEE6manageERKNS1_15function_bufferERS9_NS1_30functor_manager_operation_typeE
pub fn stub_356328() -> ! {
    todo!("0x356328 boost::detail::function::functor_manager<bool (*)(std::string const&,std::string *)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x356388 — __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_356388() -> ! {
    todo!("0x356388 __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x35638c — __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_35638c() -> ! {
    todo!("0x35638c __ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x35642c — __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_35642c() -> ! {
    todo!("0x35642c __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x356434 — __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_356434() -> ! {
    todo!("0x356434 __ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3564d8 — __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3564d8() -> ! {
    todo!("0x3564d8 __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3564e0 — __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3564e0() -> ! {
    todo!("0x3564e0 __ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x356588 — __ZN5boost9function1IvbE5clearEv
#[doc(alias = "boost::function1<void,bool>::clear(void)")]
// was: __ZN5boost9function1IvbE5clearEv
pub fn stub_356588() -> ! {
    todo!("0x356588 boost::function1<void,bool>::clear(void)")
}

// 0x3565b8 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv
#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE5clearEv
pub fn stub_3565b8() -> ! {
    todo!("0x3565b8 boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::clear(void)")
}

// 0x3565e4 — __GLOBAL__I_a_126
#[doc(alias = "global constructor keyed to_a_126")]
// was: __GLOBAL__I_a_126
pub fn stub_3565e4() -> ! {
    todo!("0x3565e4 global constructor keyed to_a_126")
}

// 0x3568e0 — __ZN3RBX4Math18deltaRotationCloseEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, float, float)
#[doc(alias = "RBX::Math::deltaRotationClose(float,float)")]
// was: __ZN3RBX4Math18deltaRotationCloseEff
pub fn stub_3568e0() -> ! {
    todo!("0x3568e0 RBX::Math::deltaRotationClose(float,float)")
}

// 0x3569d8 — __ZN3RBX4Math20averageRotationCloseEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, float, float)
#[doc(alias = "RBX::Math::averageRotationClose(float,float)")]
// was: __ZN3RBX4Math20averageRotationCloseEff
pub fn stub_3569d8() -> ! {
    todo!("0x3569d8 RBX::Math::averageRotationClose(float,float)")
}

// 0x356c80 — __ZN3RBX4Math10isDenormalEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::isDenormal(float)")]
// was: __ZN3RBX4Math10isDenormalEf
pub fn stub_356c80() -> ! {
    todo!("0x356c80 RBX::Math::isDenormal(float)")
}

// 0x356c94 — __ZN3RBX4Math8isNanInfEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::isNanInf(float)")]
// was: __ZN3RBX4Math8isNanInfEf
pub fn stub_356c94() -> ! {
    todo!("0x356c94 RBX::Math::isNanInf(float)")
}

// 0x356e34 — __ZN3RBX14segSizeRadiansEv
// type: _DWORD __fastcall(RBX *__hidden this)
#[doc(alias = "RBX::segSizeRadians(void)")]
// was: __ZN3RBX14segSizeRadiansEv
pub fn stub_356e34() -> ! {
    todo!("0x356e34 RBX::segSizeRadians(void)")
}

// 0x356e6c — __ZN3RBX18rotationToByteBaseEf
// type: _DWORD __fastcall(RBX *__hidden this, float)
#[doc(alias = "RBX::rotationToByteBase(float)")]
// was: __ZN3RBX18rotationToByteBaseEf
pub fn stub_356e6c() -> ! {
    todo!("0x356e6c RBX::rotationToByteBase(float)")
}

// 0x356ff0 — __ZN3RBX4Math14rotationToByteEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::rotationToByte(float)")]
// was: __ZN3RBX4Math14rotationToByteEf
pub fn stub_356ff0() -> ! {
    todo!("0x356ff0 RBX::Math::rotationToByte(float)")
}

// 0x3570e8 — __ZN3RBX4Math16rotationFromByteEh
// type: _DWORD __fastcall(RBX::Math *__hidden this, unsigned __int8)
#[doc(alias = "RBX::Math::rotationFromByte(unsigned char)")]
// was: __ZN3RBX4Math16rotationFromByteEh
pub fn stub_3570e8() -> ! {
    todo!("0x3570e8 RBX::Math::rotationFromByte(unsigned char)")
}

// 0x358460 — __ZN3RBX4Math17closestPointOnRayERKNS_6RbxRayES3_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const RBX::RbxRay *, const RBX::RbxRay *)
#[doc(alias = "RBX::Math::closestPointOnRay(RBX::RbxRay const&,RBX::RbxRay const&)")]
// was: __ZN3RBX4Math17closestPointOnRayERKNS_6RbxRayES3_
pub fn stub_358460() -> ! {
    todo!("0x358460 RBX::Math::closestPointOnRay(RBX::RbxRay const&,RBX::RbxRay const&)")
}

// 0x35856c — __ZN3RBX4Math13matrixRotateXEv
// type: _DWORD __fastcall(RBX::Math *__hidden this)
#[doc(alias = "RBX::Math::matrixRotateX(void)")]
// was: __ZN3RBX4Math13matrixRotateXEv
pub fn stub_35856c() -> ! {
    todo!("0x35856c RBX::Math::matrixRotateX(void)")
}

// 0x358658 — __ZN3RBX4Math13matrixRotateYEv
// type: _DWORD __fastcall(RBX::Math *__hidden this)
#[doc(alias = "RBX::Math::matrixRotateY(void)")]
// was: __ZN3RBX4Math13matrixRotateYEv
pub fn stub_358658() -> ! {
    todo!("0x358658 RBX::Math::matrixRotateY(void)")
}

// 0x358744 — __ZN3RBX4Math11matrixTiltZEv
// type: _DWORD __fastcall(RBX::Math *__hidden this)
#[doc(alias = "RBX::Math::matrixTiltZ(void)")]
// was: __ZN3RBX4Math11matrixTiltZEv
pub fn stub_358744() -> ! {
    todo!("0x358744 RBX::Math::matrixTiltZ(void)")
}

// 0x358830 — __ZN3RBX4Math19matrixTiltNegativeZEv
// type: _DWORD __fastcall(RBX::Math *__hidden this)
#[doc(alias = "RBX::Math::matrixTiltNegativeZ(void)")]
// was: __ZN3RBX4Math19matrixTiltNegativeZEv
pub fn stub_358830() -> ! {
    todo!("0x358830 RBX::Math::matrixTiltNegativeZ(void)")
}

// 0x358918 — __ZN3RBX4Math18matrixTiltQuadrantEi
// type: _DWORD __fastcall(RBX::Math *__hidden this, int)
#[doc(alias = "RBX::Math::matrixTiltQuadrant(int)")]
// was: __ZN3RBX4Math18matrixTiltQuadrantEi
pub fn stub_358918() -> ! {
    todo!("0x358918 RBX::Math::matrixTiltQuadrant(int)")
}

// 0x3589e8 — __ZN3RBX4Math17radiansToQuadrantEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::radiansToQuadrant(float)")]
// was: __ZN3RBX4Math17radiansToQuadrantEf
pub fn stub_3589e8() -> ! {
    todo!("0x3589e8 RBX::Math::radiansToQuadrant(float)")
}
