// Auto-generated skeletons for rbx-script — shard 210 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x369f44..0x377628 | script 21252->21402 distinct (filler 0x369f44 asc, not-in-script 64493->64343)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ` and ' stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x369f44 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::disconnectAll(void)")]
pub fn stub_0x369f44() -> ! {
    todo!("0x369f44 rbx::signals::signal<void ()(RBX::RunTransition)>::disconnectAll(void)")
}

// 0x36a0bc — __GLOBAL__I_a_136
#[doc(alias = "global constructor keyed to_a_136")]
pub fn stub_0x36a0bc() -> ! {
    todo!("0x36a0bc global constructor keyed to_a_136")
}

// 0x36a710 — __ZN3RBXL13findLocalFileERKSsPSs
// type: int()
#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *)")]
pub fn stub_0x36a710() -> ! {
    todo!("0x36a710 RBX::findLocalFile(std::string const&,std::string *)")
}

// 0x36b370 — __ZN3RBX17HeartbeatInstanceD2Ev
// type: void __fastcall(RBX::HeartbeatInstance *__hidden this)
#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
pub fn stub_0x36b370() -> ! {
    todo!("0x36b370 RBX::HeartbeatInstance::~HeartbeatInstance()")
}

// 0x36b644 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6insertERKSsS3_m
// type: unsigned int __fastcall(int, int, int, int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
pub fn stub_0x36b644() -> ! {
    todo!("0x36b644 RBX::SizeEnforcedLRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")
}

// 0x36de5c — __ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m
// type: void __fastcall(int, const std::string *, const std::string *, int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
pub fn stub_0x36de5c() -> ! {
    todo!("0x36de5c RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")
}

// 0x36e3e4 — __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv
// type: int __fastcall(int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")]
pub fn stub_0x36e3e4() -> ! {
    todo!("0x36e3e4 RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")
}

// 0x36e43c — __ZN3RBX8LRUCacheISsSsE6removeERKSs
// type: int __fastcall(int, int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::remove(std::string const&)")]
pub fn stub_0x36e43c() -> ! {
    todo!("0x36e43c RBX::LRUCache<std::string,std::string>::remove(std::string const&)")
}

// 0x36e490 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISA_EESK_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *)")]
pub fn stub_0x36e490() -> ! {
    todo!("0x36e490 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *)")
}

// 0x36e4ec — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36e4ec() -> ! {
    todo!("0x36e4ec boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x36e518 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36e518() -> ! {
    todo!("0x36e518 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x36e558 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_
// type: void __fastcall(int, int)
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")]
pub fn stub_0x36e558() -> ! {
    todo!("0x36e558 __gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")
}

// 0x36e610 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// type: int __fastcall(int, char **)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
pub fn stub_0x36e610() -> ! {
    todo!("0x36e610 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")
}

// 0x36e650 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSF_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEmRKT_RKT0_
// type: int __fastcall(int, unsigned int, std::string *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x36e650() -> ! {
    todo!("0x36e650 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")
}

// 0x36e6bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
pub fn stub_0x36e6bc() -> ! {
    todo!("0x36e6bc std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")
}

// 0x36e874 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE20construct_with_valueINS1_13emplace_args1ISA_EEEEvRKT_
// type: int __fastcall(int, const std::string **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
pub fn stub_0x36e874() -> ! {
    todo!("0x36e874 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")
}

// 0x36e898 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x36e898() -> ! {
    todo!("0x36e898 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x36e8e8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::~node_constructor()")]
pub fn stub_0x36e8e8() -> ! {
    todo!("0x36e8e8 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::~node_constructor()")
}

// 0x36e908 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x36e908() -> ! {
    todo!("0x36e908 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0x36ea30 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x36ea30() -> ! {
    todo!("0x36ea30 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0x36eac0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x36eac0() -> ! {
    todo!("0x36eac0 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0x36eaec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISG_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36eaec() -> ! {
    todo!("0x36eaec boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x36eb44 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEEEEE9constructEv
// type: std::string *__fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct(void)")]
pub fn stub_0x36eb44() -> ! {
    todo!("0x36eb44 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct(void)")
}

// 0x36eb80 — __ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_
// type: _DWORD *__fastcall(_DWORD *, const std::string *, _DWORD *)
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")]
pub fn stub_0x36eb80() -> ! {
    todo!("0x36eb80 std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")
}

// 0x36ec4c — __ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")]
pub fn stub_0x36ec4c() -> ! {
    todo!("0x36ec4c std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")
}

// 0x3705a0 — __ZN3RBX8LRUCacheISsSsED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
pub fn stub_0x3705a0() -> ! {
    todo!("0x3705a0 RBX::LRUCache<std::string,std::string>::~LRUCache()")
}

// 0x3706b4 — __ZN3RBX8LRUCacheISsSsE6resizeEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::resize(unsigned long)")]
pub fn stub_0x3706b4() -> ! {
    todo!("0x3706b4 RBX::LRUCache<std::string,std::string>::resize(unsigned long)")
}

// 0x3706ec — __ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
pub fn stub_0x3706ec() -> ! {
    todo!("0x3706ec std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")
}

// 0x370714 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x370714() -> ! {
    todo!("0x370714 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x37074c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEE5clearEv
// type: void *__fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x37074c() -> ! {
    todo!("0x37074c boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x370780 — __ZN3RBX8LRUCacheISsSsEC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
pub fn stub_0x370780() -> ! {
    todo!("0x370780 RBX::LRUCache<std::string,std::string>::LRUCache(void)")
}

// 0x370860 — __ZN3RBX20SizeEnforcedLRUCacheISsSsE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")]
pub fn stub_0x370860() -> ! {
    todo!("0x370860 RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")
}

// 0x3708e4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImSsEEEEESsS9_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")]
pub fn stub_0x3708e4() -> ! {
    todo!("0x3708e4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")
}

// 0x371250 — __ZN3RBX17HeartbeatInstanceD1Ev
// type: void __fastcall(RBX::HeartbeatInstance *__hidden this)
#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
pub fn stub_0x371250() -> ! {
    todo!("0x371250 RBX::HeartbeatInstance::~HeartbeatInstance()")
}

// 0x371254 — __GLOBAL__I_a_137
#[doc(alias = "global constructor keyed to_a_137")]
pub fn stub_0x371254() -> ! {
    todo!("0x371254 global constructor keyed to_a_137")
}

// 0x37148c — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::EnumDesc(void)")]
pub fn stub_0x37148c() -> ! {
    todo!("0x37148c RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::EnumDesc(void)")
}

// 0x371844 — __Z18checkResultNoThrow11FMOD_RESULT
// type: int __fastcall(unsigned int, int, int, int)
#[doc(alias = "checkResultNoThrow(FMOD_RESULT)")]
pub fn stub_0x371844() -> ! {
    todo!("0x371844 checkResultNoThrow(FMOD_RESULT)")
}

// 0x3719d0 — __Z11checkResult11FMOD_RESULT
// type: void __fastcall(unsigned int)
#[doc(alias = "checkResult(FMOD_RESULT)")]
pub fn stub_0x3719d0() -> ! {
    todo!("0x3719d0 checkResult(FMOD_RESULT)")
}

// 0x371b5c — __ZN3RBX10Soundscape12SoundServiceC1Ev
// type: int __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
pub fn stub_0x371b5c() -> ! {
    todo!("0x371b5c RBX::Soundscape::SoundService::SoundService(void)")
}

// 0x371b60 — __ZN3RBX10Soundscape12SoundServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
pub fn stub_0x371b60() -> ! {
    todo!("0x371b60 RBX::Soundscape::SoundService::SoundService(void)")
}

// 0x371e5c — __ZN3RBX10Soundscape12SoundService8openFmodEv
// type: int __fastcall(RBX::Soundscape::SoundService *this, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::openFmod(void)")]
pub fn stub_0x371e5c() -> ! {
    todo!("0x371e5c RBX::Soundscape::SoundService::openFmod(void)")
}

// 0x3723f4 — __ZN3RBX10Soundscape12SoundService16update3DSettingsEv
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::update3DSettings(void)")]
pub fn stub_0x3723f4() -> ! {
    todo!("0x3723f4 RBX::Soundscape::SoundService::update3DSettings(void)")
}

// 0x372414 — __ZN3RBX10Soundscape12SoundService19updateAmbientReverbEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::updateAmbientReverb(void)")]
pub fn stub_0x372414() -> ! {
    todo!("0x372414 RBX::Soundscape::SoundService::updateAmbientReverb(void)")
}

// 0x372460 — __ZN3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x372460() -> ! {
    todo!("0x372460 RBX::Soundscape::SoundService::~SoundService()")
}

// 0x372500 — __ZN3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x372500() -> ! {
    todo!("0x372500 RBX::Soundscape::SoundService::~SoundService()")
}

// 0x372504 — __ZThn32_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x372504() -> ! {
    todo!("0x372504 non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x37250c — __ZThn36_N3RBX10Soundscape12SoundServiceD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x37250c() -> ! {
    todo!("0x37250c non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x372514 — __ZN3RBX10Soundscape12SoundServiceD2Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x372514() -> ! {
    todo!("0x372514 RBX::Soundscape::SoundService::~SoundService()")
}

// 0x3728b0 — __ZThn32_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x3728b0() -> ! {
    todo!("0x3728b0 non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x3728b8 — __ZThn36_N3RBX10Soundscape12SoundServiceD1Ev
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x3728b8() -> ! {
    todo!("0x3728b8 non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")
}

// 0x3728c0 — __ZN3RBX10Soundscape12SoundService9closeFmodEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundService *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundService::closeFmod(void)")]
pub fn stub_0x3728c0() -> ! {
    todo!("0x3728c0 RBX::Soundscape::SoundService::closeFmod(void)")
}

// 0x3729bc — __ZL12releaseSoundRKSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEE
// type: int __fastcall(int)
#[doc(alias = "releaseSound(std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_0x3729bc() -> ! {
    todo!("0x3729bc releaseSound(std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")
}

// 0x3729c4 — __ZL11initReverbsv
// type: void *__fastcall()
#[doc(alias = "initReverbs(void)")]
pub fn stub_0x3729c4() -> ! {
    todo!("0x3729c4 initReverbs(void)")
}

// 0x372bb0 — __ZN3RBX10Soundscape12SoundService15loadStockSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::loadStockSounds(void)")]
pub fn stub_0x372bb0() -> ! {
    todo!("0x372bb0 RBX::Soundscape::SoundService::loadStockSounds(void)")
}

// 0x373554 — __ZN3RBX10Soundscape12SoundService14loadStockSoundENS_9SoundTypeESs
// type: void __fastcall(RBX::Instance *, int, int, int)
#[doc(alias = "RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType,std::string)")]
pub fn stub_0x373554() -> ! {
    todo!("0x373554 RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType,std::string)")
}

// 0x37384c — __ZN3RBX10Soundscape12SoundChannel10setSoundIdENS0_7SoundIdE
// type: int __fastcall(_DWORD *, std::string *)
#[doc(alias = "RBX::Soundscape::SoundChannel::setSoundId(RBX::Soundscape::SoundId)")]
pub fn stub_0x37384c() -> ! {
    todo!("0x37384c RBX::Soundscape::SoundChannel::setSoundId(RBX::Soundscape::SoundId)")
}

// 0x373894 — __ZN3RBX10Soundscape7SoundIdC1ERKNS_9ContentIdE
// type: RBX::Soundscape::SoundId *__fastcall(RBX::Soundscape::SoundId *this, const RBX::ContentId *)
#[doc(alias = "RBX::Soundscape::SoundId::SoundId(RBX::ContentId const&)")]
pub fn stub_0x373894() -> ! {
    todo!("0x373894 RBX::Soundscape::SoundId::SoundId(RBX::ContentId const&)")
}

// 0x3738a8 — __ZN3RBX10Soundscape12SoundService16setAmbientReverbERKNS0_10ReverbTypeE
// type: int __fastcall(RBX::Instance *, int *)
#[doc(alias = "RBX::Soundscape::SoundService::setAmbientReverb(RBX::Soundscape::ReverbType const&)")]
pub fn stub_0x3738a8() -> ! {
    todo!("0x3738a8 RBX::Soundscape::SoundService::setAmbientReverb(RBX::Soundscape::ReverbType const&)")
}

// 0x3738d8 — __ZN3RBX10Soundscape12SoundService9playSoundENS_9SoundTypeE
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "RBX::Soundscape::SoundService::playSound(RBX::SoundType)")]
pub fn stub_0x3738d8() -> ! {
    todo!("0x3738d8 RBX::Soundscape::SoundService::playSound(RBX::SoundType)")
}

// 0x373918 — __ZN3RBX10Soundscape12SoundChannel4playEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::play(void)")]
pub fn stub_0x373918() -> ! {
    todo!("0x373918 RBX::Soundscape::SoundChannel::play(void)")
}

// 0x373974 — __ZN3RBX10Soundscape12SoundService17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(shared_count *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x373974() -> ! {
    todo!("0x373974 RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x373bf4 — __Z7convertRKN3G3D7Vector3ER11FMOD_VECTOR
// type: int __fastcall(RBX::Math **, _DWORD *)
#[doc(alias = "convert(G3D::Vector3 const&,FMOD_VECTOR &)")]
pub fn stub_0x373bf4() -> ! {
    todo!("0x373bf4 convert(G3D::Vector3 const&,FMOD_VECTOR &)")
}

// 0x373cb8 — __ZN3RBX10Soundscape12SoundService4stepEv
// type: void __fastcall(RBX::Soundscape::SoundService *this, int, int, int (*)(const char *, ...))
#[doc(alias = "RBX::Soundscape::SoundService::step(void)")]
pub fn stub_0x373cb8() -> ! {
    todo!("0x373cb8 RBX::Soundscape::SoundService::step(void)")
}

// 0x373fd0 — __ZN3RBX10Soundscape12SoundService20garbageCollectSoundsEv
// type: void __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::garbageCollectSounds(void)")]
pub fn stub_0x373fd0() -> ! {
    todo!("0x373fd0 RBX::Soundscape::SoundService::garbageCollectSounds(void)")
}

// 0x374028 — __ZN3RBX15StringConverterINS_10Soundscape7SoundIdEE14convertToValueERKSsRS2_
// type: int __fastcall(std::string *, std::string *)
#[doc(alias = "RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&,RBX::Soundscape::SoundId&)")]
pub fn stub_0x374028() -> ! {
    todo!("0x374028 RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&,RBX::Soundscape::SoundId&)")
}

// 0x37414c — __ZN3RBX10Reflection4Type12getSingletonINS_10Soundscape7SoundIdEEERKS1_v
// type: int()
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Soundscape::SoundId>(void)")]
pub fn stub_0x37414c() -> ! {
    todo!("0x37414c RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Soundscape::SoundId>(void)")
}

// 0x374154 — __ZN3RBX10Reflection7Variant7convertINS_10Soundscape7SoundIdEEERT_v
// type: int __fastcall(int)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::convert<RBX::Soundscape::SoundId>(void)")]
pub fn stub_0x374154() -> ! {
    todo!("0x374154 RBX::Soundscape::SoundId & RBX::Reflection::Variant::convert<RBX::Soundscape::SoundId>(void)")
}

// 0x374340 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x374340() -> ! {
    todo!("0x374340 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x374528 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x374528() -> ! {
    todo!("0x374528 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x374758 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE11getDataSizeEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x374758() -> ! {
    todo!("0x374758 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getDataSize(RBX::Reflection::DescribedBase const*)const")
}

// 0x3747b4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::hasStringValue(void)const")]
pub fn stub_0x3747b4() -> ! {
    todo!("0x3747b4 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::hasStringValue(void)const")
}

// 0x3747b8 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14getStringValueEPKNS0_13DescribedBaseE
// type: void __fastcall(std::string *, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3747b8() -> ! {
    todo!("0x3747b8 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x3748d4 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_10Soundscape7SoundIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(RBX::Name *, int, std::string *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x3748d4() -> ! {
    todo!("0x3748d4 RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x374a2c — __ZNK3RBX10Soundscape12SoundChannel10getSoundIdEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::getSoundId(void)const")]
pub fn stub_0x374a2c() -> ! {
    todo!("0x374a2c RBX::Soundscape::SoundChannel::getSoundId(void)const")
}

// 0x374a44 — __ZNK3RBX10Soundscape12SoundChannel9getVolumeEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getVolume(void)const")]
pub fn stub_0x374a44() -> ! {
    todo!("0x374a44 RBX::Soundscape::SoundChannel::getVolume(void)const")
}

// 0x374a48 — __ZN3RBX10Soundscape12SoundChannel9setVolumeEf
// type: int __fastcall(int this, float32_t, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::setVolume(float)")]
pub fn stub_0x374a48() -> ! {
    todo!("0x374a48 RBX::Soundscape::SoundChannel::setVolume(float)")
}

// 0x374aa4 — __ZNK3RBX10Soundscape12SoundChannel8getPitchEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getPitch(void)const")]
pub fn stub_0x374aa4() -> ! {
    todo!("0x374aa4 RBX::Soundscape::SoundChannel::getPitch(void)const")
}

// 0x374aa8 — __ZN3RBX10Soundscape12SoundChannel8setPitchEf
// type: int __fastcall(int this, float, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::setPitch(float)")]
pub fn stub_0x374aa8() -> ! {
    todo!("0x374aa8 RBX::Soundscape::SoundChannel::setPitch(float)")
}

// 0x374af8 — __ZN3RBX10Soundscape12SoundChannel12setPlayCountEi
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::setPlayCount(int)")]
pub fn stub_0x374af8() -> ! {
    todo!("0x374af8 RBX::Soundscape::SoundChannel::setPlayCount(int)")
}

// 0x374b68 — __ZNK3RBX10Soundscape12SoundChannel9getLoopedEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getLooped(void)const")]
pub fn stub_0x374b68() -> ! {
    todo!("0x374b68 RBX::Soundscape::SoundChannel::getLooped(void)const")
}

// 0x374b74 — __ZN3RBX10Soundscape12SoundChannel9setLoopedEb
// type: unsigned int __fastcall(RBX::Soundscape::SoundChannel *this, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::setLooped(bool)")]
pub fn stub_0x374b74() -> ! {
    todo!("0x374b74 RBX::Soundscape::SoundChannel::setLooped(bool)")
}

// 0x374bb4 — __ZNK3RBX10Soundscape12SoundChannel9isPlayingEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::isPlaying(void)const")]
pub fn stub_0x374bb4() -> ! {
    todo!("0x374bb4 RBX::Soundscape::SoundChannel::isPlaying(void)const")
}

// 0x374bec — __ZNK3RBX10Soundscape12SoundChannel8isPausedEv
// type: bool __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::isPaused(void)const")]
pub fn stub_0x374bec() -> ! {
    todo!("0x374bec RBX::Soundscape::SoundChannel::isPaused(void)const")
}

// 0x374c24 — __ZN3RBX10Soundscape12SoundChannel5pauseEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::pause(void)")]
pub fn stub_0x374c24() -> ! {
    todo!("0x374c24 RBX::Soundscape::SoundChannel::pause(void)")
}

// 0x374c68 — __ZN3RBX10Soundscape12SoundChannel4stopEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::stop(void)")]
pub fn stub_0x374c68() -> ! {
    todo!("0x374c68 RBX::Soundscape::SoundChannel::stop(void)")
}

// 0x374cc4 — __ZN3RBX10Soundscape12SoundChannelC2Ev
// type: RBX::Instance *__fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::SoundChannel(void)")]
pub fn stub_0x374cc4() -> ! {
    todo!("0x374cc4 RBX::Soundscape::SoundChannel::SoundChannel(void)")
}

// 0x374ff4 — __ZN3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x374ff4() -> ! {
    todo!("0x374ff4 RBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375094 — __ZN3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x375094() -> ! {
    todo!("0x375094 RBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375098 — __ZThn32_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x375098() -> ! {
    todo!("0x375098 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x3750a0 — __ZThn36_N3RBX10Soundscape12SoundChannelD0Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x3750a0() -> ! {
    todo!("0x3750a0 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x3750a8 — __ZN3RBX10Soundscape12SoundChannelD2Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x3750a8() -> ! {
    todo!("0x3750a8 RBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375330 — __ZThn32_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x375330() -> ! {
    todo!("0x375330 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375338 — __ZThn36_N3RBX10Soundscape12SoundChannelD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x375338() -> ! {
    todo!("0x375338 non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")
}

// 0x375340 — __ZNK3RBX10Soundscape12SoundService11getCpuStatsERNS1_8CpuStatsE
// type: int __fastcall(int, float *)
#[doc(alias = "RBX::Soundscape::SoundService::getCpuStats(RBX::Soundscape::SoundService::CpuStats &)const")]
pub fn stub_0x375340() -> ! {
    todo!("0x375340 RBX::Soundscape::SoundService::getCpuStats(RBX::Soundscape::SoundService::CpuStats &)const")
}

// 0x3753e8 — __ZN3RBX10Soundscape12SoundService13getSoundStatsERKSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEERjSH_
// type: const _Rb_tree_node_base *__fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "RBX::Soundscape::SoundService::getSoundStats(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> const&,unsigned int &,unsigned int &)")]
pub fn stub_0x3753e8() -> ! {
    todo!("0x3753e8 RBX::Soundscape::SoundService::getSoundStats(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> const&,unsigned int &,unsigned int &)")
}

// 0x375418 — __ZNK3RBX10Soundscape12SoundService18getChannelsPlayingERi
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, int *)
#[doc(alias = "RBX::Soundscape::SoundService::getChannelsPlaying(int &)const")]
pub fn stub_0x375418() -> ! {
    todo!("0x375418 RBX::Soundscape::SoundService::getChannelsPlaying(int &)const")
}

// 0x375438 — __ZN3RBX10Soundscape12SoundService8gcSoundsERSt3mapINS0_7SoundIdEN5boost10shared_ptrINS0_5SoundEEESt4lessIS3_ESaISt4pairIKS3_S7_EEE
// type: int __fastcall(int result)
#[doc(alias = "RBX::Soundscape::SoundService::gcSounds(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> &)")]
pub fn stub_0x375438() -> ! {
    todo!("0x375438 RBX::Soundscape::SoundService::gcSounds(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> &)")
}

// 0x3754c4 — __ZN3RBX10Soundscape5Sound7releaseEv
// type: FMOD::Sound *__fastcall(FMOD::Sound **this)
#[doc(alias = "RBX::Soundscape::Sound::release(void)")]
pub fn stub_0x3754c4() -> ! {
    todo!("0x3754c4 RBX::Soundscape::Sound::release(void)")
}

// 0x3754e0 — __ZN3RBX10Soundscape12SoundChannel14releaseChannelEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::releaseChannel(void)")]
pub fn stub_0x3754e0() -> ! {
    todo!("0x3754e0 RBX::Soundscape::SoundChannel::releaseChannel(void)")
}

// 0x37551c — __ZNK3RBX10Soundscape12SoundChannel12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x37551c() -> ! {
    todo!("0x37551c RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")
}

// 0x375520 — __ZN3RBX10Soundscape12SoundChannel17updateListenStateEv
// type: void __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::updateListenState(void)")]
pub fn stub_0x375520() -> ! {
    todo!("0x375520 RBX::Soundscape::SoundChannel::updateListenState(void)")
}

// 0x375660 — __ZN3RBX10Soundscape12SoundChannel11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(FMOD::Channel **, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x375660() -> ! {
    todo!("0x375660 RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&)")
}

// 0x37567c — __ZN3RBX10Soundscape12SoundChannel17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x37567c() -> ! {
    todo!("0x37567c RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x375744 — __ZN3RBX10Soundscape12SoundChannel9playSoundEPKNS_8InstanceE
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")]
pub fn stub_0x375744() -> ! {
    todo!("0x375744 RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")
}

// 0x375b7c — __ZN3RBX10Soundscape12SoundChannel17onServiceProviderEPNS_15ServiceProviderES3_
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x375b7c() -> ! {
    todo!("0x375b7c RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x375be0 — __ZN3RBX10Soundscape12SoundChannel12preloadSoundEv
// type: _DWORD __fastcall(RBX::Soundscape::SoundChannel *__hidden this)
#[doc(alias = "RBX::Soundscape::SoundChannel::preloadSound(void)")]
pub fn stub_0x375be0() -> ! {
    todo!("0x375be0 RBX::Soundscape::SoundChannel::preloadSound(void)")
}

// 0x375c3c — __ZN3RBX10Soundscape12SoundChannel8update3DEPN4FMOD7ChannelE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, FMOD::Channel *)
#[doc(alias = "RBX::Soundscape::SoundChannel::update3D(FMOD::Channel *)")]
pub fn stub_0x375c3c() -> ! {
    todo!("0x375c3c RBX::Soundscape::SoundChannel::update3D(FMOD::Channel *)")
}

// 0x375c8c — __ZN3RBX10Soundscape12SoundChannel12updateLoopedEv
// type: FMOD::Channel *__fastcall(RBX::Soundscape::SoundChannel *this, int, FMOD::ChannelI **)
#[doc(alias = "RBX::Soundscape::SoundChannel::updateLooped(void)")]
pub fn stub_0x375c8c() -> ! {
    todo!("0x375c8c RBX::Soundscape::SoundChannel::updateLooped(void)")
}

// 0x375ce8 — __Z18callbackChannelEndP12FMOD_CHANNEL25FMOD_CHANNEL_CALLBACKTYPEPvS2_
// type: int __fastcall(int, int)
#[doc(alias = "callbackChannelEnd(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *)")]
pub fn stub_0x375ce8() -> ! {
    todo!("0x375ce8 callbackChannelEnd(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *)")
}

// 0x375d0c — __ZNK3RBX10Soundscape12SoundChannel14isHeardLocallyEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::SoundChannel *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")]
pub fn stub_0x375d0c() -> ! {
    todo!("0x375d0c RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")
}

// 0x375dd4 — __ZN3RBX10Soundscape12SoundService9loadSoundENS0_7SoundIdEb
// type: void __fastcall(sp_counted_base **, const shared_count *, const std::string *, int)
#[doc(alias = "RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId,bool)")]
pub fn stub_0x375dd4() -> ! {
    todo!("0x375dd4 RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId,bool)")
}

// 0x376004 — __ZN3RBX10Soundscape5Sound3getEPKNS_8InstanceE
// type: int __fastcall(RBX::Soundscape::Sound *this, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::Sound::get(RBX::Instance const*)")]
pub fn stub_0x376004() -> ! {
    todo!("0x376004 RBX::Soundscape::Sound::get(RBX::Instance const*)")
}

// 0x376198 — __ZN3RBX13registerSoundEv
// type: int __fastcall(RBX *this)
#[doc(alias = "RBX::registerSound(void)")]
pub fn stub_0x376198() -> ! {
    todo!("0x376198 RBX::registerSound(void)")
}

// 0x37619c — __ZN3RBX10Soundscape5SoundD2Ev
// type: void __fastcall(FMOD::Sound **this)
#[doc(alias = "RBX::Soundscape::Sound::~Sound()")]
pub fn stub_0x37619c() -> ! {
    todo!("0x37619c RBX::Soundscape::Sound::~Sound()")
}

// 0x376244 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")]
pub fn stub_0x376244() -> ! {
    todo!("0x376244 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")
}

// 0x3765a4 — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEEPFvRKS1_IS4_S9_EEET0_T_SI_SH_
// type: void (__fastcall *__fastcall(_Rb_tree_node_base *, _Rb_tree_node_base *, void (__fastcall *)(_DWORD *), int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int))(_DWORD *)
#[doc(alias = "void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>))")]
pub fn stub_0x3765a4() -> ! {
    todo!("0x3765a4 void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>))")
}

// 0x37677c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(__guard *)
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
pub fn stub_0x37677c() -> ! {
    todo!("0x37677c rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")
}

// 0x3768dc — __ZNSt3mapIN3RBX9SoundTypeEN5boost10shared_ptrINS0_10Soundscape12SoundChannelEEESt4lessIS1_ESaISt4pairIKS1_S6_EEEixERSA_
// type: int __fastcall(int, int *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::SoundType,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")]
pub fn stub_0x3768dc() -> ! {
    todo!("0x3768dc std::map<RBX::SoundType,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")
}

// 0x376a24 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEaSINS1_10StockSoundEEERS4_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>& rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const&)")]
pub fn stub_0x376a24() -> ! {
    todo!("0x376a24 rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>& rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const&)")
}

// 0x376a58 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEaSERKS5_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::operator=(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const&)")]
pub fn stub_0x376a58() -> ! {
    todo!("0x376a58 rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::operator=(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const&)")
}

// 0x376a90 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")]
pub fn stub_0x376a90() -> ! {
    todo!("0x376a90 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")
}

// 0x376ac4 — __ZN21SoundServiceStatsItem6createEPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(RBX::Stats::Item **this, const RBX::Soundscape::SoundService *)
#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
pub fn stub_0x376ac4() -> ! {
    todo!("0x376ac4 SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")
}

// 0x376c84 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10Soundscape7SoundIdEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), const std::string *))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
pub fn stub_0x376c84() -> ! {
    todo!("0x376c84 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")
}

// 0x376ce4 — __ZN3RBX10Reflection7Variant14genericConvertINS_10Soundscape7SoundIdEEERT_v
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")]
pub fn stub_0x376ce4() -> ! {
    todo!("0x376ce4 RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")
}

// 0x376f90 — __ZN3RBX10Soundscape12SoundService18on3DSettingChangedERKNS_10Reflection18PropertyDescriptorE
// type: FMOD::System *__fastcall(RBX::Soundscape::SoundService *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x376f90() -> ! {
    todo!("0x376f90 RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x376f94 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x376f94() -> ! {
    todo!("0x376f94 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")
}

// 0x376fb8 — __ZNK3RBX10Soundscape12SoundService16getAmbientReverbEv
// type: int __fastcall(RBX::Soundscape::SoundService *this)
#[doc(alias = "RBX::Soundscape::SoundService::getAmbientReverb(void)const")]
pub fn stub_0x376fb8() -> ! {
    todo!("0x376fb8 RBX::Soundscape::SoundService::getAmbientReverb(void)const")
}

// 0x376fc0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_10Soundscape12SoundServiceENS2_10ReverbTypeEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")]
pub fn stub_0x376fc0() -> ! {
    todo!("0x376fc0 RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")
}

// 0x376fe4 — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundServiceEFvNS_9SoundTypeEELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
pub fn stub_0x376fe4() -> ! {
    todo!("0x376fe4 RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")
}

// 0x377024 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelENS2_7SoundIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")]
pub fn stub_0x377024() -> ! {
    todo!("0x377024 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")
}

// 0x377048 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEfED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")]
pub fn stub_0x377048() -> ! {
    todo!("0x377048 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")
}

// 0x37706c — __ZNK3RBX10Soundscape12SoundChannel12getPlayCountEv
// type: int __fastcall(RBX::Soundscape::SoundChannel *this)
#[doc(alias = "RBX::Soundscape::SoundChannel::getPlayCount(void)const")]
pub fn stub_0x37706c() -> ! {
    todo!("0x37706c RBX::Soundscape::SoundChannel::getPlayCount(void)const")
}

// 0x377074 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")]
pub fn stub_0x377074() -> ! {
    todo!("0x377074 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")
}

// 0x377098 — __ZN3RBX10Reflection14PropDescriptorINS_10Soundscape12SoundChannelEbED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")]
pub fn stub_0x377098() -> ! {
    todo!("0x377098 RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")
}

// 0x3770bc — __ZN3RBX10Reflection13BoundFuncDescINS_10Soundscape12SoundChannelEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x3770bc() -> ! {
    todo!("0x3770bc RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")
}

// 0x3770e0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10Soundscape12SoundChannelES5_EENSA_5list2INSA_5valueIPSF_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")]
pub fn stub_0x3770e0() -> ! {
    todo!("0x3770e0 rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")
}

// 0x377154 — __ZN3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_0x377154() -> ! {
    todo!("0x377154 RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}

// 0x37716c — __ZN5boost10shared_ptrIN3RBX10Soundscape5SoundEEaSERKS4_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
pub fn stub_0x37716c() -> ! {
    todo!("0x37716c rbx_core::SharedPtr<RBX::Soundscape::Sound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")
}

// 0x3771a4 — __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x3771a4() -> ! {
    todo!("0x3771a4 __ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x3772c0 — __ZNSt3mapIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, const std::string *)
#[doc(alias = "std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")]
pub fn stub_0x3772c0() -> ! {
    todo!("0x3772c0 std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")
}

// 0x37750c — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
pub fn stub_0x37750c() -> ! {
    todo!("0x37750c __ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")
}

// 0x37751c — __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
pub fn stub_0x37751c() -> ! {
    todo!("0x37751c __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")
}

// 0x37752c — __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
pub fn stub_0x37752c() -> ! {
    todo!("0x37752c __ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")
}

// 0x37753c — __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
pub fn stub_0x37753c() -> ! {
    todo!("0x37753c __ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")
}

// 0x37754c — __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x37754c() -> ! {
    todo!("0x37754c __ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev")
}

// 0x377550 — __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev")]
pub fn stub_0x377550() -> ! {
    todo!("0x377550 __ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev")
}

// 0x377554 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_0x377554() -> ! {
    todo!("0x377554 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")
}

// 0x377558 — __ZN3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_0x377558() -> ! {
    todo!("0x377558 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")
}

// 0x3775f8 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")]
pub fn stub_0x3775f8() -> ! {
    todo!("0x3775f8 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")
}

// 0x377628 — __ZNK3RBX10Reflection8EnumDescINS_10Soundscape10ReverbTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x377628() -> ! {
    todo!("0x377628 RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(RBX::Reflection::Variant const&)const")
}
