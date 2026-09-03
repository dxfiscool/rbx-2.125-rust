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
// IDA 0x36f110: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_36f110() {
}

// 0x36f114 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEED0Ev
// IDA 0x36f114: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_36f114() {
}

// 0x36f118 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE7disposeEv
// IDA 0x36f118: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f118() {
}

// 0x36f12c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE11get_deleterERKSt9type_info
// IDA 0x36f12c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f12c() {
}

// 0x36f130 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_25ScriptInformationProvider16CachedScriptInfoELb0EEEE19get_untyped_deleterEv
// IDA 0x36f130: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f130() {
}

// 0x36f134 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
// type: int __fastcall(int, int, int, int, RBX::AsyncHttpQueue *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
// IDA 0x36f134: 123 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f134() {
}

// 0x36f284 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED1Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED1Ev
// IDA 0x36f284: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36f284() {
}

// 0x36f38c — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache()")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EED0Ev
// IDA 0x36f38c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36f38c() {
}

// 0x36f4a4 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
// IDA 0x36f4a4: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f4a4() {
}

// 0x36f694 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
// type: unsigned int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
// IDA 0x36f694: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f694() {
}

// 0x36f6c8 — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
// type: void __fastcall(int, const std::string *, int *, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6insertERKSsRKS2_m
// IDA 0x36f6c8: 373 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36f6c8() {
}

// 0x36fadc — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::removeLeastRecentlyUsed(void)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE23removeLeastRecentlyUsedEv
// IDA 0x36fadc: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fadc() {
}

// 0x36fb34 — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6removeERKSs
// type: int __fastcall(int, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::remove(std::string const&)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6removeERKSs
// IDA 0x36fb34: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fb34() {
}

// 0x36fb88 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
// IDA 0x36fb88: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fb88() {
}

// 0x36fbe4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// IDA 0x36fbe4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fbe4() {
}

// 0x36fc10 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// IDA 0x36fc10: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fc10() {
}

// 0x36fc50 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
// type: void __fastcall(int, _DWORD *, std::string *, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
// IDA 0x36fc50: 156 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fc50() {
}

// 0x36fe00 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
// IDA 0x36fe00: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fe00() {
}

// 0x36fe24 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// IDA 0x36fe24: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fe24() {
}

// 0x36fe74 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEED2Ev
// IDA 0x36fe74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_36fe74() {
}

// 0x36fe90 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// IDA 0x36fe90: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36fe90() {
}

// 0x36ffb8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// IDA 0x36ffb8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_36ffb8() {
}

// 0x370048 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// IDA 0x370048: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370048() {
}

// 0x370074 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
// IDA 0x370074: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370074() {
}

// 0x3700cc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE9constructEv
// type: std::string *__fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEEEEE9constructEv
// IDA 0x3700cc: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3700cc() {
}

// 0x370108 — __ZNSt4listISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>> const&)")]
// was: __ZNSt4listISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE14_M_create_nodeERKS5_
// IDA 0x370108: 80 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370108() {
}

// 0x3701ec — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::~LRUCache()")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEED2Ev
// IDA 0x3701ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3701ec() {
}

// 0x370300 — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
// IDA 0x370300: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370300() {
}

// 0x370338 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseISt4pairISsS0_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEESaIS5_EE8_M_clearEv
// IDA 0x370338: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370338() {
}

// 0x370360 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// IDA 0x370360: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370360() {
}

// 0x370398 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// IDA 0x370398: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370398() {
}

// 0x3703cc — __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEEC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::LRUCache(void)")]
// was: __ZN3RBX8LRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEEC2Ev
// IDA 0x3703cc: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3703cc() {
}

// 0x3704ac — __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsNS_25ScriptInformationProvider16CachedScriptInfoEE6resizeEm
// IDA 0x3704ac: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3704ac() {
}

// 0x370530 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX25ScriptInformationProvider16CachedScriptInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
// IDA 0x370530: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370530() {
}

// 0x3705a0 — __ZN3RBX8LRUCacheISsSsED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
// was: __ZN3RBX8LRUCacheISsSsED2Ev
// IDA 0x3705a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3705a0() {
}

// 0x3706b4 — __ZN3RBX8LRUCacheISsSsE6resizeEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::resize(unsigned long)")]
// was: __ZN3RBX8LRUCacheISsSsE6resizeEm
// IDA 0x3706b4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3706b4() {
}

// 0x3706ec — __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
// IDA 0x3706ec: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3706ec() {
}

// 0x370714 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// IDA 0x370714: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370714() {
}

// 0x37074c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// IDA 0x37074c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37074c() {
}

// 0x370780 — __ZN3RBX8LRUCacheISsSsEC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
// was: __ZN3RBX8LRUCacheISsSsEC2Ev
// IDA 0x370780: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370780() {
}

// 0x370860 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")]
// was: __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm
// IDA 0x370860: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370860() {
}

// 0x3708e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// IDA 0x3708e4: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3708e4() {
}

// 0x370950 — __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x370950: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_370950() {
}

// 0x370954 — __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x370954: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_370954() {
}

// 0x3709f4 — __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3709f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3709f4() {
}

// 0x3709fc — __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3709fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3709fc() {
}

// 0x370aa0 — __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x370aa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_370aa0() {
}

// 0x370aa8 — __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x370aa8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_370aa8() {
}

// 0x370b4c — __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::BoundFuncDesc(void (RBX::ScriptInformationProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x370b4c: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370b4c() {
}

// 0x370cc8 — __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x370cc8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370cc8() {
}

// 0x370cf8 — __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EED0Ev
// IDA 0x370cf8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_370cf8() {
}

// 0x370e00 — __ZNK3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_25ScriptInformationProviderEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x370e00: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370e00() {
}

// 0x370f3c — __ZN3RBX10Reflection11Call1HelperINS_25ScriptInformationProviderEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ScriptInformationProvider,void (RBX::ScriptInformationProvider::*)(std::string),std::string,void>::call(RBX::ScriptInformationProvider*,void (RBX::ScriptInformationProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_25ScriptInformationProviderEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// IDA 0x370f3c: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_370f3c() {
}

// 0x371070 — __ZN3RBX25ScriptInformationProviderD2Ev
// type: void __fastcall(RBX::ScriptInformationProvider *__hidden this)
#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider()")]
// was: __ZN3RBX25ScriptInformationProviderD2Ev
// IDA 0x371070: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_371070() {
}

// 0x371220 — __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::clear(void)")]
// was: __ZN5boost9function5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE5clearEv
// IDA 0x371220: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_371220() {
}

// 0x371250 — __ZN3RBX17HeartbeatInstanceD1Ev
// type: void __fastcall(RBX::HeartbeatInstance *__hidden this)
#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
// was: __ZN3RBX17HeartbeatInstanceD1Ev
// IDA 0x371250: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_371250() {
}

// 0x371254 — __GLOBAL__I_a_137
#[doc(alias = "global constructor keyed to_a_137")]
// was: __GLOBAL__I_a_137
// IDA 0x371254: 181 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_371254() {
}

// 0x37148c — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEEC2Ev
// IDA 0x37148c: 334 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37148c() {
}

// 0x371844 — __Z18checkResultNoThrow11FMOD_RESULT
// type: int __fastcall(unsigned int, int, int, int)
#[doc(alias = "checkResultNoThrow(FMOD_RESULT)")]
// was: __Z18checkResultNoThrow11FMOD_RESULT
// IDA 0x371844: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_371844() {
}

// 0x3719d0 — __Z11checkResult11FMOD_RESULT
// type: void __fastcall(unsigned int)
#[doc(alias = "checkResult(FMOD_RESULT)")]
// was: __Z11checkResult11FMOD_RESULT
// IDA 0x3719d0: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3719d0() {
}

// 0x371b5c — __ZN3RBX10Soundscape12SoundServiceC1Ev
// type: int __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
// was: __ZN3RBX10Soundscape12SoundServiceC1Ev
// IDA 0x371b5c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_371b5c() {
}

// 0x371b60 — __ZN3RBX10Soundscape12SoundServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
// was: __ZN3RBX10Soundscape12SoundServiceC2Ev
// IDA 0x371b60: 264 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_371b60() {
}

// 0x371e5c — __ZN3RBX10Soundscape12SoundService8openFmodEv
// type: int __fastcall(RBX::Soundscape::SoundService *this, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::openFmod(void)")]
// was: __ZN3RBX10Soundscape12SoundService8openFmodEv
// IDA 0x371e5c: 490 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_371e5c() {
}

// 0x3723f4 — __ZN3RBX10Soundscape12SoundService16update3DSettingsEv
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::update3DSettings(void)")]
// was: __ZN3RBX10Soundscape12SoundService16update3DSettingsEv
// IDA 0x3723f4: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3723f4() {
}

// 0x372414 — __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::updateAmbientReverb(void)")]
// was: __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv
// IDA 0x372414: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_372414() {
}

// 0x372460 — __ZN3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// was: __ZN3RBX10Soundscape12SoundServiceD0Ev
// IDA 0x372460: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_372460() {
}

// 0x372500 — __ZN3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// was: __ZN3RBX10Soundscape12SoundServiceD1Ev
// IDA 0x372500: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_372500() {
}

// 0x372504 — __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev
// IDA 0x372504: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_372504() {
}

// 0x37250c — __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev
// IDA 0x37250c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37250c() {
}

// 0x372514 — __ZN3RBX10Soundscape12SoundServiceD2Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
// was: __ZN3RBX10Soundscape12SoundServiceD2Ev
// IDA 0x372514: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_372514() {
}

// 0x3728b0 — __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev
// IDA 0x3728b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3728b0() {
}

// 0x3728b8 — __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
// was: __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev
// IDA 0x3728b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3728b8() {
}

// 0x3728c0 — __ZN3RBX10Soundscape12SoundService9closeFmodEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::closeFmod(void)")]
// was: __ZN3RBX10Soundscape12SoundService9closeFmodEv
// IDA 0x3728c0: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3728c0() {
}

// 0x3729bc — __ZL12releaseSoundRKSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEE
// type: int __fastcall(int)
#[doc(alias = "releaseSound(std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
// was: __ZL12releaseSoundRKSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEE
// IDA 0x3729bc: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3729bc() {
}

// 0x3729c4 — __ZL11initReverbsv
// type: void *__fastcall()
#[doc(alias = "initReverbs(void)")]
// was: __ZL11initReverbsv
// IDA 0x3729c4: 148 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3729c4() {
}

// 0x372bb0 — __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::loadStockSounds(void)")]
// was: __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv
// IDA 0x372bb0: 844 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_372bb0() {
}

// 0x373554 — __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs
// type: void __fastcall(RBX::Instance *, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType,std::string)")]
// was: __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs
// IDA 0x373554: 262 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_373554() {
}

// 0x37384c — __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE
// type: int __fastcall(_DWORD *, std::string *)
#[doc(alias = "RBX::Soundscape::SoundChannel::setSoundId(RBX::Soundscape::SoundId)")]
// was: __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE
// IDA 0x37384c: 25 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37384c() {
}

// 0x373894 — __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE
// type: RBX::Soundscape::SoundId *__fastcall(RBX::Soundscape::SoundId *this, const RBX::ContentId *)
#[doc(alias = "RBX::Soundscape::SoundId::SoundId(RBX::ContentId const&)")]
// was: __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE
// IDA 0x373894: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_373894() {
}

// 0x3738a8 — __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE
// type: int __fastcall(RBX::Instance *, int *)
#[doc(alias = "RBX::Soundscape::SoundService::setAmbientReverb(RBX::Soundscape::ReverbType const&)")]
// was: __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE
// IDA 0x3738a8: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3738a8() {
}

// 0x3738d8 — __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::Soundscape::SoundService::playSound(RBX::SoundType)")]
// was: __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE
// IDA 0x3738d8: 25 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3738d8() {
}

// 0x373918 — __ZN3RBX10Soundscape12SoundChannel4playEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::play(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel4playEv
// IDA 0x373918: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_373918() {
}

// 0x373974 — __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(shared_count *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_
// IDA 0x373974: 238 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_373974() {
}

// 0x373cb8 — __ZN3RBX10Soundscape12SoundService4stepEv
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int, int (*)(const char *, ...))
#[doc(alias = "RBX::Soundscape::SoundService::step(void)")]
// was: __ZN3RBX10Soundscape12SoundService4stepEv
// IDA 0x373cb8: 275 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_373cb8() {
}

// 0x373fd0 — __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::garbageCollectSounds(void)")]
// was: __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv
// IDA 0x373fd0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_373fd0() {
}

// 0x374028 — __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_
// type: int __fastcall(std::string *, std::string *)
#[doc(alias = "RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&,RBX::Soundscape::SoundId&)")]
// was: __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_
// IDA 0x374028: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374028() {
}

// 0x37414c — __ZN3RBX10Reflection4Type12getSingletonINS_10Soundscape7SoundIdEEERKS1_v
// type: int()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Soundscape::SoundId>(void)")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_10Soundscape7SoundIdEEERKS1_v
// IDA 0x37414c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_37414c() {
}

// 0x374154 — __ZN3RBX10Reflection7Variant7convertINS_10Soundscape7SoundIdEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::convert<RBX::Soundscape::SoundId>(void)")]
// was: __ZN3RBX10Reflection7Variant7convertINS_10Soundscape7SoundIdEEERT_v
// IDA 0x374154: 169 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374154() {
}

// 0x374340 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x374340: 178 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374340() {
}

// 0x374528 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x374528: 198 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374528() {
}

// 0x374758 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11getDataSizeEPKNS0_13DescribedBaseE
// IDA 0x374758: 34 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374758() {
}

// 0x3747b4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14hasStringValueEv
// IDA 0x3747b4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3747b4() {
}

// 0x3747b8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14getStringValueEPKNS0_13DescribedBaseE
// type: void __fastcall(std::string *, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x3747b8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3747b8() {
}

// 0x3748d4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(RBX::Name *, int, std::string *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x3748d4: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3748d4() {
}

// 0x374a2c — __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::getSoundId(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv
// IDA 0x374a2c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374a2c() {
}

// 0x374a44 — __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getVolume(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv
// IDA 0x374a44: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374a44() {
}

// 0x374a48 — __ZN3RBX10Soundscape12SoundChannel9setVolumeEf
// type: int __fastcall(int this, float32_t, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::setVolume(float)")]
// was: __ZN3RBX10Soundscape12SoundChannel9setVolumeEf
// IDA 0x374a48: 27 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374a48() {
}

// 0x374aa4 — __ZNK3RBX10Soundscape12SoundChannel8getPitchEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getPitch(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel8getPitchEv
// IDA 0x374aa4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374aa4() {
}

// 0x374aa8 — __ZN3RBX10Soundscape12SoundChannel8setPitchEf
// type: int __fastcall(int this, float, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::setPitch(float)")]
// was: __ZN3RBX10Soundscape12SoundChannel8setPitchEf
// IDA 0x374aa8: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374aa8() {
}

// 0x374af8 — __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::setPlayCount(int)")]
// was: __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi
// IDA 0x374af8: 36 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374af8() {
}

// 0x374b68 — __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getLooped(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv
// IDA 0x374b68: 4 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374b68() {
}

// 0x374b74 — __ZN3RBX10Soundscape12SoundChannel9setLoopedEb
// type: unsigned int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::setLooped(bool)")]
// was: __ZN3RBX10Soundscape12SoundChannel9setLoopedEb
// IDA 0x374b74: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374b74() {
}

// 0x374bb4 — __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::isPlaying(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv
// IDA 0x374bb4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374bb4() {
}

// 0x374bec — __ZNK3RBX10Soundscape12SoundChannel8isPausedEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::isPaused(void)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel8isPausedEv
// IDA 0x374bec: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374bec() {
}

// 0x374c24 — __ZN3RBX10Soundscape12SoundChannel5pauseEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::pause(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel5pauseEv
// IDA 0x374c24: 23 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374c24() {
}

// 0x374c68 — __ZN3RBX10Soundscape12SoundChannel4stopEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::stop(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel4stopEv
// IDA 0x374c68: 29 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374c68() {
}

// 0x374cc4 — __ZN3RBX10Soundscape12SoundChannelC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::SoundChannel(void)")]
// was: __ZN3RBX10Soundscape12SoundChannelC2Ev
// IDA 0x374cc4: 292 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_374cc4() {
}

// 0x374ff4 — __ZN3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZN3RBX10Soundscape12SoundChannelD0Ev
// IDA 0x374ff4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_374ff4() {
}

// 0x375094 — __ZN3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZN3RBX10Soundscape12SoundChannelD1Ev
// IDA 0x375094: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_375094() {
}

// 0x375098 — __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev
// IDA 0x375098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_375098() {
}

// 0x3750a0 — __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev
// IDA 0x3750a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3750a0() {
}

// 0x3750a8 — __ZN3RBX10Soundscape12SoundChannelD2Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZN3RBX10Soundscape12SoundChannelD2Ev
// IDA 0x3750a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3750a8() {
}

// 0x375330 — __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev
// IDA 0x375330: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_375330() {
}

// 0x375338 — __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
// was: __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev
// IDA 0x375338: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_375338() {
}

// 0x375340 — __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE
// type: int __fastcall(int, float *)
#[doc(alias = "RBX::Soundscape::SoundService::getCpuStats(RBX::Soundscape::SoundService::CpuStats &)const")]
// was: __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE
// IDA 0x375340: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375340() {
}

// 0x3753e8 — __ZN3RBX10Soundscape12SoundService13getSoundStatsERKSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEERjSH_
// type: const _Rb_tree_node_base *__fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "RBX::Soundscape::SoundService::getSoundStats(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> const&,unsigned int &,unsigned int &)")]
// was: __ZN3RBX10Soundscape12SoundService13getSoundStatsERKSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEERjSH_
// IDA 0x3753e8: 22 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3753e8() {
}

// 0x375418 — __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, int *)
#[doc(alias = "RBX::Soundscape::SoundService::getChannelsPlaying(int &)const")]
// was: __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi
// IDA 0x375418: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375418() {
}

// 0x375438 — __ZN3RBX10Soundscape12SoundService8gcSoundsERSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEE
// type: int __fastcall(int result)
#[doc(alias = "RBX::Soundscape::SoundService::gcSounds(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> &)")]
// was: __ZN3RBX10Soundscape12SoundService8gcSoundsERSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEE
// IDA 0x375438: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375438() {
}

// 0x3754c4 — __ZN3RBX10Soundscape5Sound7releaseEv
// type: FMOD::Sound *__fastcall(FMOD::Sound **this)
#[doc(alias = "RBX::Soundscape::Sound::release(void)")]
// was: __ZN3RBX10Soundscape5Sound7releaseEv
// IDA 0x3754c4: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3754c4() {
}

// 0x3754e0 — __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::releaseChannel(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv
// IDA 0x3754e0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3754e0() {
}

// 0x37551c — __ZNK3RBX10Soundscape12SoundChannel12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel12askSetParentEPKNS_8InstanceE
// IDA 0x37551c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37551c() {
}

// 0x375520 — __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv
// type: void __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::updateListenState(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv
// IDA 0x375520: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375520() {
}

// 0x375660 — __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(FMOD::Channel **, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&)")]
// was: __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE
// IDA 0x375660: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375660() {
}

// 0x37567c — __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE
// IDA 0x37567c: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37567c() {
}

// 0x375744 — __ZN3RBX10Soundscape12SoundChannel9playSoundEPKNS_8InstanceE
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")]
// was: __ZN3RBX10Soundscape12SoundChannel9playSoundEPKNS_8InstanceE
// IDA 0x375744: 387 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375744() {
}

// 0x375b7c — __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_
// IDA 0x375b7c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375b7c() {
}

// 0x375be0 — __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::preloadSound(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv
// IDA 0x375be0: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375be0() {
}

// 0x375c3c — __ZN3RBX10Soundscape12SoundChannel8update3DEPN4FMOD7ChannelE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, FMOD::Channel *)
#[doc(alias = "RBX::Soundscape::SoundChannel::update3D(FMOD::Channel *)")]
// was: __ZN3RBX10Soundscape12SoundChannel8update3DEPN4FMOD7ChannelE
// IDA 0x375c3c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375c3c() {
}

// 0x375c8c — __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv
// type: FMOD::Channel *__fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::updateLooped(void)")]
// was: __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv
// IDA 0x375c8c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375c8c() {
}

// 0x375ce8 — __Z18callbackChannelEndP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS2_
// type: int __fastcall(int, int)
#[doc(alias = "callbackChannelEnd(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *)")]
// was: __Z18callbackChannelEndP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS2_
// IDA 0x375ce8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375ce8() {
}

// 0x375d0c — __ZNK3RBX10Soundscape12SoundChannel14isHeardLocallyEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")]
// was: __ZNK3RBX10Soundscape12SoundChannel14isHeardLocallyEPKNS_8InstanceE
// IDA 0x375d0c: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375d0c() {
}

// 0x375dd4 — __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb
// type: void __fastcall(sp_counted_base **, const shared_count *, const std::string *, int)
#[doc(alias = "RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId,bool)")]
// was: __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb
// IDA 0x375dd4: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_375dd4() {
}

// 0x376004 — __ZN3RBX10Soundscape5Sound3getEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::Sound *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::Sound::get(RBX::Instance const*)")]
// was: __ZN3RBX10Soundscape5Sound3getEPKNS_8InstanceE
// IDA 0x376004: 139 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376004() {
}

// 0x376198 — __ZN3RBX13registerSoundEv
// type: int __fastcall(RBX *this)
#[doc(alias = "RBX::registerSound(void)")]
// was: __ZN3RBX13registerSoundEv
// IDA 0x376198: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_376198() {
}

// 0x37619c — __ZN3RBX10Soundscape5SoundD2Ev
// type: void __fastcall(FMOD::Sound **this)
#[doc(alias = "RBX::Soundscape::Sound::~Sound()")]
// was: __ZN3RBX10Soundscape5SoundD2Ev
// IDA 0x37619c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_37619c() {
}

// 0x376244 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc
// IDA 0x376244: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376244() {
}

// 0x3765a4 — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
// type: void (__fastcall *__fastcall(_Rb_tree_node_base *, _Rb_tree_node_base *, void (__fastcall *)(_DWORD *), int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int))(_DWORD *)
#[doc(alias = "void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>))")]
// was: __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
// IDA 0x3765a4: 168 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3765a4() {
}

// 0x37677c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(__guard *)
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
// IDA 0x37677c: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_37677c() {
}

// 0x3768dc — __ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
// type: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::SoundType,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")]
// was: __ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
// IDA 0x3768dc: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3768dc() {
}

// 0x376a24 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>& rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// IDA 0x376a24: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376a24() {
}

// 0x376a58 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::operator=(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// IDA 0x376a58: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376a58() {
}

// 0x376a90 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
// IDA 0x376a90: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376a90() {
}

// 0x376ac4 — __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(RBX::Stats::Item **this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
// was: __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// IDA 0x376ac4: 143 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376ac4() {
}

// 0x376c84 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), const std::string *))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// IDA 0x376c84: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376c84() {
}

// 0x376ce4 — __ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
// IDA 0x376ce4: 166 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376ce4() {
}

// 0x376f90 — __ZN3RBX10Soundscape12SoundService18on3DSettingChangedERKNS_10Reflection18PropertyDescriptorE
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX10Soundscape12SoundService18on3DSettingChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x376f90: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_376f90() {
}

// 0x376f94 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED1Ev
// IDA 0x376f94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_376f94() {
}

// 0x376fb8 — __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
// type: int __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::getAmbientReverb(void)const")]
// was: __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
// IDA 0x376fb8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_376fb8() {
}

// 0x376fc0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED1Ev
// IDA 0x376fc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_376fc0() {
}

// 0x376fe4 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED1Ev
// IDA 0x376fe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_376fe4() {
}