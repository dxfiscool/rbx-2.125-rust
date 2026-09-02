//! rendering shard rend_wd_watchdog8 — 120 stubs 0x7f1ccc..0x7f7468 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render filtered exhausted -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x7f1ca0
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7f1ccc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE")]
pub fn stub_7f1ccc() -> ! {
    todo!("0x7f1ccc boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x7f1d24 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>>::construct(void)")]
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEE9constructEv")]
pub fn stub_7f1d24() -> ! {
    todo!("0x7f1d24 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>>::construct(void)")
}

// 0x7f1d60 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_")]
pub fn stub_7f1d60() -> ! {
    todo!("0x7f1d60 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")
}

// 0x7f1dcc — __ZNSt4pairISsS_ImN3RBX15ContentProvider13CachedContentEEEC2ERKSsRKS3_
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>::pair(std::string const&,std::pair<unsigned long,RBX::ContentProvider::CachedContent> const&)")]
#[doc(alias = "__ZNSt4pairISsS_ImN3RBX15ContentProvider13CachedContentEEEC2ERKSsRKS3_")]
pub fn stub_7f1dcc() -> ! {
    todo!("0x7f1dcc std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>::pair(std::string const&,std::pair<unsigned long,RBX::ContentProvider::CachedContent> const&)")
}

// 0x7f1ed8 — __ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>> const&)")]
#[doc(alias = "__ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE14_M_create_nodeERKS5_")]
pub fn stub_7f1ed8() -> ! {
    todo!("0x7f1ed8 std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>> const&)")
}

// 0x7f202c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *)")]
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_")]
pub fn stub_7f202c() -> ! {
    todo!("0x7f202c boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *)")
}

// 0x7f2088 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
pub fn stub_7f2088() -> ! {
    todo!("0x7f2088 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x7f20b4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")]
pub fn stub_7f20b4() -> ! {
    todo!("0x7f20b4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x7f20f4 — __ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>)")]
#[doc(alias = "__ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E")]
pub fn stub_7f20f4() -> ! {
    todo!("0x7f20f4 std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>)")
}

// 0x7f21dc — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
pub fn stub_7f21dc() -> ! {
    todo!("0x7f21dc boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")
}

// 0x7f2220 — __ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, void *, int, int, int, char, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "std::vector<std::string,std::allocator<std::string>> & boost::algorithm::iter_split<std::vector<std::string,std::allocator<std::string>>,std::string,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(std::vector<std::string,std::allocator<std::string>> &,std::string &,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
#[doc(alias = "__ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_")]
pub fn stub_7f2220() -> ! {
    todo!("0x7f2220 std::vector<std::string,std::allocator<std::string>> & boost::algorithm::iter_split<std::vector<std::string,std::allocator<std::string>>,std::string,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(std::vector<std::string,std::allocator<std::string>> &,std::string &,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")
}

// 0x7f24a4 — __ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "boost::algorithm::detail::is_any_ofF<char>::is_any_ofF(boost::algorithm::detail::is_any_ofF<char> const&)")]
#[doc(alias = "__ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_")]
pub fn stub_7f24a4() -> ! {
    todo!("0x7f24a4 boost::algorithm::detail::is_any_ofF<char>::is_any_ofF(boost::algorithm::detail::is_any_ofF<char> const&)")
}

// 0x7f24d4 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_
#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to_own(boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>> const&)")]
#[doc(alias = "__ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_")]
pub fn stub_7f24d4() -> ! {
    todo!("0x7f24d4 boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to_own(boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>> const&)")
}

// 0x7f2504 — __ZNSt6vectorISsSaISsEEC2IN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEET_SH_RKS0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::vector<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::allocator<std::string> const&)")]
#[doc(alias = "__ZNSt6vectorISsSaISsEEC2IN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEET_SH_RKS0_")]
pub fn stub_7f2504() -> ! {
    todo!("0x7f2504 std::vector<std::string,std::allocator<std::string>>::vector<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::allocator<std::string> const&)")
}

// 0x7f265c — __ZNSt6vectorISsSaISsEE22_M_initialize_dispatchIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St12__false_type
// type: int __fastcall(int, int, char, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_initialize_dispatch<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::__false_type)")]
#[doc(alias = "__ZNSt6vectorISsSaISsEE22_M_initialize_dispatchIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St12__false_type")]
pub fn stub_7f265c() -> ! {
    todo!("0x7f265c void std::vector<std::string,std::allocator<std::string>>::_M_initialize_dispatch<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::__false_type)")
}

// 0x7f2784 — __ZNSt6vectorISsSaISsEE19_M_range_initializeIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St18input_iterator_tag
// type: void __fastcall(int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_range_initialize<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::input_iterator_tag)")]
#[doc(alias = "__ZNSt6vectorISsSaISsEE19_M_range_initializeIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St18input_iterator_tag")]
pub fn stub_7f2784() -> ! {
    todo!("0x7f2784 void std::vector<std::string,std::allocator<std::string>>::_M_range_initialize<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::input_iterator_tag)")
}

// 0x7f28d0 — __ZNSt6vectorISsSaISsEE9push_backERKSs
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::push_back(std::string const&)")]
#[doc(alias = "__ZNSt6vectorISsSaISsEE9push_backERKSs")]
pub fn stub_7f28d0() -> ! {
    todo!("0x7f28d0 std::vector<std::string,std::allocator<std::string>>::push_back(std::string const&)")
}

// 0x7f2910 — __ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv
#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::increment(void)")]
#[doc(alias = "__ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv")]
pub fn stub_7f2910() -> ! {
    todo!("0x7f2910 boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::increment(void)")
}

// 0x7f2964 — __ZNK5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EclES5_S5_
#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::operator()(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")]
#[doc(alias = "__ZNK5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EclES5_S5_")]
pub fn stub_7f2964() -> ! {
    todo!("0x7f2964 boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::operator()(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")
}

// 0x7f2a30 — __ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSsEEEES2_T_S4_RKSaIcESt20forward_iterator_tag
#[doc(alias = "char * std::string::_S_construct<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,std::allocator<char> const&,std::forward_iterator_tag)")]
#[doc(alias = "__ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSsEEEES2_T_S4_RKSaIcESt20forward_iterator_tag")]
pub fn stub_7f2a30() -> ! {
    todo!("0x7f2a30 char * std::string::_S_construct<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,std::allocator<char> const&,std::forward_iterator_tag)")
}

// 0x7f2a94 — __ZNK5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE5equalERKS6_
#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::equal(boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>> const&)const")]
#[doc(alias = "__ZNK5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE5equalERKS6_")]
pub fn stub_7f2a94() -> ! {
    todo!("0x7f2a94 boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::equal(boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>> const&)const")
}

// 0x7f2b20 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv
#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::clear(void)")]
#[doc(alias = "__ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv")]
pub fn stub_7f2b20() -> ! {
    todo!("0x7f2b20 boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::clear(void)")
}

// 0x7f2b4c — __ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::algorithm::detail::find_iterator_base<__gnu_cxx::__normal_iterator<char *,std::string>>::find_iterator_base<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,int)")]
#[doc(alias = "__ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i")]
pub fn stub_7f2b4c() -> ! {
    todo!("0x7f2b4c boost::algorithm::detail::find_iterator_base<__gnu_cxx::__normal_iterator<char *,std::string>>::find_iterator_base<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,int)")
}

// 0x7f2c14 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EC2INS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "__ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EC2INS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EC2INS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
pub fn stub_7f2c14() -> ! {
    todo!("0x7f2c14 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EC2INS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")
}

// 0x7f2ce0 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "void boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
#[doc(alias = "__ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_")]
pub fn stub_7f2ce0() -> ! {
    todo!("0x7f2ce0 void boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")
}

// 0x7f2db8 — __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE6manageERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE6manageERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeE")]
pub fn stub_7f2db8() -> ! {
    todo!("0x7f2db8 boost::detail::function::functor_manager<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x7f2dd4 — __ZN5boost6detail8function21function_obj_invoker2INS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEENS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEESD_SD_E6invokeERNS1_15function_bufferESD_SD_
#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::invoke(boost::detail::function::function_buffer &,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker2INS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEENS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEESD_SD_E6invokeERNS1_15function_bufferESD_SD_")]
pub fn stub_7f2dd4() -> ! {
    todo!("0x7f2dd4 boost::detail::function::function_obj_invoker2<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::invoke(boost::detail::function::function_buffer &,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)")
}

// 0x7f2de0 — __ZNK5boost6detail8function13basic_vtable2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES7_S7_E9assign_toINS_9algorithm6detail13token_finderFINSC_10is_any_ofFIcEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "bool boost::detail::function::basic_vtable2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES7_S7_E9assign_toINS_9algorithm6detail13token_finderFINSC_10is_any_ofFIcEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_7f2de0() -> ! {
    todo!("0x7f2de0 bool boost::detail::function::basic_vtable2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::detail::function::function_buffer &)const")
}

// 0x7f2eb0 — __ZNK5boost9algorithm6detail13token_finderFINS1_10is_any_ofFIcEEEclIN9__gnu_cxx17__normal_iteratorIPcSsEEEENS_14iterator_rangeIT_EESC_SC_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>> boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>::operator()<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")]
#[doc(alias = "__ZNK5boost9algorithm6detail13token_finderFINS1_10is_any_ofFIcEEEclIN9__gnu_cxx17__normal_iteratorIPcSsEEEENS_14iterator_rangeIT_EESC_SC_")]
pub fn stub_7f2eb0() -> ! {
    todo!("0x7f2eb0 boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>> boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>::operator()<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")
}

// 0x7f2fec — __ZNK5boost9algorithm6detail10is_any_ofFIcEclIcEEbT_
#[doc(alias = "bool boost::algorithm::detail::is_any_ofF<char>::operator()<char>(char)const")]
#[doc(alias = "__ZNK5boost9algorithm6detail10is_any_ofFIcEclIcEEbT_")]
pub fn stub_7f2fec() -> ! {
    todo!("0x7f2fec bool boost::algorithm::detail::is_any_ofF<char>::operator()<char>(char)const")
}

// 0x7f3040 — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPcSsEEN5boost9algorithm6detail10is_any_ofFIcEEET_S9_S9_T0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<char *,std::string> std::__find_if<__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPcSsEEN5boost9algorithm6detail10is_any_ofFIcEEET_S9_S9_T0_St26random_access_iterator_tag")]
pub fn stub_7f3040() -> ! {
    todo!("0x7f3040 __gnu_cxx::__normal_iterator<char *,std::string> std::__find_if<__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>,std::random_access_iterator_tag)")
}

// 0x7f30f0 — __ZN5boost14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2ISsEERT_NS_21iterator_range_detail9range_tagE
// type: int __fastcall(int, std::string *this)
#[doc(alias = "boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>::iterator_range<std::string>(std::string &,boost::iterator_range_detail::range_tag)")]
#[doc(alias = "__ZN5boost14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2ISsEERT_NS_21iterator_range_detail9range_tagE")]
pub fn stub_7f30f0() -> ! {
    todo!("0x7f30f0 boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>::iterator_range<std::string>(std::string &,boost::iterator_range_detail::range_tag)")
}

// 0x7f3118 — __ZN5boost9algorithm6detail10is_any_ofFIcEC2INS_14iterator_rangeIPKcEEEERKT_
// type: int __fastcall(void *__dst)
#[doc(alias = "boost::algorithm::detail::is_any_ofF<char>::is_any_ofF<boost::iterator_range<char const*>>(boost::iterator_range<char const*> const&)")]
#[doc(alias = "__ZN5boost9algorithm6detail10is_any_ofFIcEC2INS_14iterator_rangeIPKcEEEERKT_")]
pub fn stub_7f3118() -> ! {
    todo!("0x7f3118 boost::algorithm::detail::is_any_ofF<char>::is_any_ofF<boost::iterator_range<char const*>>(boost::iterator_range<char const*> const&)")
}

// 0x7f3178 — __ZSt16__introsort_loopIPciEvT_S1_T0_
#[doc(alias = "void std::__introsort_loop<char *,int>(char *,char *,int)")]
#[doc(alias = "__ZSt16__introsort_loopIPciEvT_S1_T0_")]
pub fn stub_7f3178() -> ! {
    todo!("0x7f3178 void std::__introsort_loop<char *,int>(char *,char *,int)")
}

// 0x7f323c — __ZSt22__final_insertion_sortIPcEvT_S1_
#[doc(alias = "void std::__final_insertion_sort<char *>(char *,char *)")]
#[doc(alias = "__ZSt22__final_insertion_sortIPcEvT_S1_")]
pub fn stub_7f323c() -> ! {
    todo!("0x7f323c void std::__final_insertion_sort<char *>(char *,char *)")
}

// 0x7f328c — __ZSt16__insertion_sortIPcEvT_S1_
// type: char *__fastcall(char *__src, char *)
#[doc(alias = "void std::__insertion_sort<char *>(char *,char *)")]
#[doc(alias = "__ZSt16__insertion_sortIPcEvT_S1_")]
pub fn stub_7f328c() -> ! {
    todo!("0x7f328c void std::__insertion_sort<char *>(char *,char *)")
}

// 0x7f32f4 — __ZSt9sort_heapIPcEvT_S1_
#[doc(alias = "void std::sort_heap<char *>(char *,char *)")]
#[doc(alias = "__ZSt9sort_heapIPcEvT_S1_")]
pub fn stub_7f32f4() -> ! {
    todo!("0x7f32f4 void std::sort_heap<char *>(char *,char *)")
}

// 0x7f3318 — __ZSt8pop_heapIPcEvT_S1_
#[doc(alias = "void std::pop_heap<char *>(char *,char *)")]
#[doc(alias = "__ZSt8pop_heapIPcEvT_S1_")]
pub fn stub_7f3318() -> ! {
    todo!("0x7f3318 void std::pop_heap<char *>(char *,char *)")
}

// 0x7f3328 — __ZSt13__adjust_heapIPcicEvT_T0_S2_T1_
#[doc(alias = "void std::__adjust_heap<char *,int,char>(char *,int,int,char)")]
#[doc(alias = "__ZSt13__adjust_heapIPcicEvT_T0_S2_T1_")]
pub fn stub_7f3328() -> ! {
    todo!("0x7f3328 void std::__adjust_heap<char *,int,char>(char *,int,int,char)")
}

// 0x7f33a8 — __ZSt9make_heapIPcEvT_S1_
#[doc(alias = "void std::make_heap<char *>(char *,char *)")]
#[doc(alias = "__ZSt9make_heapIPcEvT_S1_")]
pub fn stub_7f33a8() -> ! {
    todo!("0x7f33a8 void std::make_heap<char *>(char *,char *)")
}

// 0x7f33d8 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")]
pub fn stub_7f33d8() -> ! {
    todo!("0x7f33d8 boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>)")
}

// 0x7f34e0 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_")]
pub fn stub_7f34e0() -> ! {
    todo!("0x7f34e0 boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>)")
}

// 0x7f35d8 — __ZN5boost9iostreams6detail9close_allINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEEEEvRT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "void boost::iostreams::detail::close_all<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &)")]
#[doc(alias = "__ZN5boost9iostreams6detail9close_allINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEEEEvRT_")]
pub fn stub_7f35d8() -> ! {
    todo!("0x7f35d8 void boost::iostreams::detail::close_all<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &)")
}

// 0x7f3694 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS3_ISt19basic_ostringstreamIcS6_SaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_
// type: int __fastcall(int, int, unsigned int, int, int, void *, int, int, int, int)
#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS3_ISt19basic_ostringstreamIcS6_SaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_")]
pub fn stub_7f3694() -> ! {
    todo!("0x7f3694 int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")
}

// 0x7f37a4 — __ZN5boost10shared_ptrIKSsEC2ISsEEPT_
#[doc(alias = "boost::shared_ptr<std::string const>::shared_ptr<std::string>(std::string *)")]
#[doc(alias = "__ZN5boost10shared_ptrIKSsEC2ISsEEPT_")]
// was: boost::shared_ptr<std::string const>::shared_ptr<std::string>(std::string *)
pub fn stub_7f37a4() -> ! {
    todo!("0x7f37a4 rbx_core::SharedPtr<std::string const>::shared_ptr<std::string>(std::string *)")
}

// 0x7f3878 — __ZN5boost6detail12shared_countC2ISsEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::string>(std::string *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2ISsEEPT_")]
pub fn stub_7f3878() -> ! {
    todo!("0x7f3878 boost::detail::shared_count::shared_count<std::string>(std::string *)")
}

// 0x7f3970 — __ZN5boost6detail17sp_counted_impl_pISsED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISsED1Ev")]
pub fn stub_7f3970() -> ! {
    todo!("0x7f3970 boost::detail::sp_counted_impl_p<std::string>::~sp_counted_impl_p()")
}

// 0x7f3978 — __ZN5boost6detail17sp_counted_impl_pISsE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISsE7disposeEv")]
pub fn stub_7f3978() -> ! {
    todo!("0x7f3978 boost::detail::sp_counted_impl_p<std::string>::dispose(void)")
}

// 0x7f3998 — __ZN5boost6detail17sp_counted_impl_pISsE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pISsE19get_untyped_deleterEv")]
pub fn stub_7f3998() -> ! {
    todo!("0x7f3998 boost::detail::sp_counted_impl_p<std::string>::get_untyped_deleter(void)")
}

// 0x7f399c — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS7_5list2INS7_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS7_5list2INS7_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS7_5list2INS7_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_7f399c() -> ! {
    todo!("0x7f399c __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS7_5list2INS7_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x7f3af0 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS6_5list2INS6_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS6_5list2INS6_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS6_5list2INS6_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_7f3af0() -> ! {
    todo!("0x7f3af0 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS6_5list2INS6_5valueISG_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x7f3c44 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS6_5list2INS6_5valueISG_EENSK_ISE_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESE_ENS6_5list2INS6_5valueISG_EENSK_ISE_EEEEEEEEvT_")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>)
pub fn stub_7f3c44() -> ! {
    todo!("0x7f3c44 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>)")
}

// 0x7f3dac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_7f3dac() -> ! {
    todo!("0x7f3dac boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x7f3dc8 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESO_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESO_")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_7f3dc8() -> ! {
    todo!("0x7f3dc8 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x7f3de4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESG_ENS8_5list2INS8_5valueISI_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESG_ENS8_5list2INS8_5valueISI_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &)const
pub fn stub_7f3de4() -> ! {
    todo!("0x7f3de4 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &)const")
}

// 0x7f3f3c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESG_ENS8_5list2INS8_5valueISI_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESG_ENS8_5list2INS8_5valueISI_EENSM_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_7f3f3c() -> ! {
    todo!("0x7f3f3c bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x7f4090 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESG_ENS8_5list2INS8_5valueISI_EENSM_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESG_ENS8_5list2INS8_5valueISI_EENSM_ISG_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_7f4090() -> ! {
    todo!("0x7f4090 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x7f4198 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEclIPFvSC_SA_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEclIPFvSC_SA_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_7f4198() -> ! {
    todo!("0x7f4198 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0x7f42a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_7f42a0() -> ! {
    todo!("0x7f42a0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x7f4458 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")]
// was: boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)
pub fn stub_7f4458() -> ! {
    todo!("0x7f4458 boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")
}

// 0x7f4560 — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")]
// was: boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)
pub fn stub_7f4560() -> ! {
    todo!("0x7f4560 boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")
}

// 0x7f4664 — __ZN5boost3_bi8storage3INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEEEC2ES5_S7_S8_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEEEC2ES5_S7_S8_")]
pub fn stub_7f4664() -> ! {
    todo!("0x7f4664 boost::_bi::storage3<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>)")
}

// 0x7f4788 — __ZN5boost3_bi8storage2INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEEEC2ES5_S7_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::ContentId>,boost::arg<1>>::storage2(boost::_bi::value<RBX::ContentId>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEEEC2ES5_S7_")]
pub fn stub_7f4788() -> ! {
    todo!("0x7f4788 boost::_bi::storage2<boost::_bi::value<RBX::ContentId>,boost::arg<1>>::storage2(boost::_bi::value<RBX::ContentId>,boost::arg<1>)")
}

// 0x7f48bc — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS0_IFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSB_5list4INSB_5valueISD_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS0_IFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSB_5list4INSB_5valueISD_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS0_IFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSB_5list4INSB_5valueISD_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_7f48bc() -> ! {
    todo!("0x7f48bc __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS0_IFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSB_5list4INSB_5valueISD_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")
}

// 0x7f4ae0 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS_8functionIFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSA_5list4INSA_5valueISC_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS_8functionIFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSA_5list4INSA_5valueISC_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS_8functionIFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSA_5list4INSA_5valueISC_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_7f4ae0() -> ! {
    todo!("0x7f4ae0 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS_8functionIFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSA_5list4INSA_5valueISC_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")
}

// 0x7f4d08 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS_8functionIFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSA_5list4INSA_5valueISC_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvRKNS1_9ContentIdES3_S4_NS_8functionIFvS3_NS5_ISt6vectorINS5_INS1_8InstanceEEESaISI_EEEEEEEENSA_5list4INSA_5valueISC_EENS_3argILi1EEENST_ILi2EEENSR_ISN_EEEEEEEEvT_")]
// was: void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>)
pub fn stub_7f4d08() -> ! {
    todo!("0x7f4d08 void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>)")
}

// 0x7f4f40 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKN3RBX9ContentIdENS5_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvSA_NS_10shared_ptrISt6vectorINSD_INS5_8InstanceEEESaISG_EEEEEEEENS3_5list4INS3_5valueIS6_EENS_3argILi1EEENSR_ILi2EEENSP_ISL_EEEEEEE6manageERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKN3RBX9ContentIdENS5_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvSA_NS_10shared_ptrISt6vectorINSD_INS5_8InstanceEEESaISG_EEEEEEEENS3_5list4INS3_5valueIS6_EENS_3argILi1EEENSR_ILi2EEENSP_ISL_EEEEEEE6manageERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_7f4f40() -> ! {
    todo!("0x7f4f40 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x7f4f5c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvRKN3RBX9ContentIdENS5_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvSA_NS_10shared_ptrISt6vectorINSD_INS5_8InstanceEEESaISG_EEEEEEEENS3_5list4INS3_5valueIS6_EENS_3argILi1EEENSR_ILi2EEENSP_ISL_EEEEEEvSA_SB_NSD_IKSsEEE6invokeERNS1_15function_bufferESA_SB_SY_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvRKN3RBX9ContentIdENS5_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvSA_NS_10shared_ptrISt6vectorINSD_INS5_8InstanceEEESaISG_EEEEEEEENS3_5list4INS3_5valueIS6_EENS_3argILi1EEENSR_ILi2EEENSP_ISL_EEEEEEvSA_SB_NSD_IKSsEEE6invokeERNS1_15function_bufferESA_SB_SY_")]
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
pub fn stub_7f4f5c() -> ! {
    todo!("0x7f4f5c boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")
}

// 0x7f4f80 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvRKNS3_9ContentIdES5_S6_NS_8functionIFvS5_NS7_ISt6vectorINS7_INS3_8InstanceEEESaISK_EEEEEEEENSC_5list4INSC_5valueISE_EENS_3argILi1EEENSV_ILi2EEENST_ISP_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvRKNS3_9ContentIdES5_S6_NS_8functionIFvS5_NS7_ISt6vectorINS7_INS3_8InstanceEEESaISK_EEEEEEEENSC_5list4INSC_5valueISE_EENS_3argILi1EEENSV_ILi2EEENST_ISP_EEEEEEEEbT_RNS1_15function_bufferE")]
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_7f4f80() -> ! {
    todo!("0x7f4f80 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x7f51a8 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvRKNS3_9ContentIdES5_S6_NS_8functionIFvS5_NS7_ISt6vectorINS7_INS3_8InstanceEEESaISK_EEEEEEEENSC_5list4INSC_5valueISE_EENS_3argILi1EEENSV_ILi2EEENST_ISP_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, std::string *)
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvRKNS3_9ContentIdES5_S6_NS_8functionIFvS5_NS7_ISt6vectorINS7_INS3_8InstanceEEESaISK_EEEEEEEENSC_5list4INSC_5valueISE_EENS_3argILi1EEENSV_ILi2EEENST_ISP_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_7f51a8() -> ! {
    todo!("0x7f51a8 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x7f53cc — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvRKNS3_9ContentIdES5_S6_NS_8functionIFvS5_NS7_ISt6vectorINS7_INS3_8InstanceEEESaISK_EEEEEEEENSC_5list4INSC_5valueISE_EENS_3argILi1EEENSV_ILi2EEENST_ISP_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvRKNS3_9ContentIdES5_S6_NS_8functionIFvS5_NS7_ISt6vectorINS7_INS3_8InstanceEEESaISK_EEEEEEEENSC_5list4INSC_5valueISE_EENS_3argILi1EEENSV_ILi2EEENST_ISP_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// was: void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_7f53cc() -> ! {
    todo!("0x7f53cc void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x7f54bc — __ZN5boost3_bi5list4INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEENS2_INS_8functionIFvNS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS3_8InstanceEEESaISF_EEEEEEEEEEclIPFvRKS4_SB_PSiSK_ENS0_5list3IRSB_RSQ_RNSC_IKSsEEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>::operator()<void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &>>(boost::_bi::type<void>,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEENS2_INS_8functionIFvNS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS3_8InstanceEEESaISF_EEEEEEEEEEclIPFvRKS4_SB_PSiSK_ENS0_5list3IRSB_RSQ_RNSC_IKSsEEEEEEvNS0_4typeIvEERT_RT0_i")]
// was: void boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>::operator()<void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &>>(boost::_bi::type<void>,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &> &,int)
pub fn stub_7f54bc() -> ! {
    todo!("0x7f54bc void boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>::operator()<void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")
}

// 0x7f5590 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKN3RBX9ContentIdENS5_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvSA_NS_10shared_ptrISt6vectorINSD_INS5_8InstanceEEESaISG_EEEEEEEENS3_5list4INS3_5valueIS6_EENS_3argILi1EEENSR_ILi2EEENSP_ISL_EEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKN3RBX9ContentIdENS5_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvSA_NS_10shared_ptrISt6vectorINSD_INS5_8InstanceEEESaISG_EEEEEEEENS3_5list4INS3_5valueIS6_EENS_3argILi1EEENSR_ILi2EEENSP_ISL_EEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_7f5590() -> ! {
    todo!("0x7f5590 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x7f5714 — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE13assign_to_ownERKSB_
#[doc(alias = "boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to_own(boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>> const&)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEE13assign_to_ownERKSB_")]
// was: boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::assign_to_own(boost::function2<void,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>> const&)
pub fn stub_7f5714() -> ! {
    todo!("0x7f5714 boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::assign_to_own(boost::function2<void,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>> const&)")
}

// 0x7f5744 — __ZN5boost3_bi5list4INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEENS2_INS_8functionIFvNS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS3_8InstanceEEESaISF_EEEEEEEEEEC2ES5_S7_S8_SL_
#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>::list4(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEENS2_INS_8functionIFvNS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS3_8InstanceEEESaISF_EEEEEEEEEEC2ES5_S7_S8_SL_")]
// was: boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>::list4(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>)
pub fn stub_7f5744() -> ! {
    todo!("0x7f5744 boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>::list4(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>)")
}

// 0x7f58a4 — __ZN5boost3_bi8storage4INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEENS2_INS_8functionIFvNS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS3_8InstanceEEESaISF_EEEEEEEEEEC2ES5_S7_S8_SL_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>::storage4(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEENS2_INS_8functionIFvNS3_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINSC_INS3_8InstanceEEESaISF_EEEEEEEEEEC2ES5_S7_S8_SL_")]
// was: boost::_bi::storage4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>::storage4(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>)
pub fn stub_7f58a4() -> ! {
    todo!("0x7f58a4 boost::_bi::storage4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>::storage4(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>)")
}

// 0x7f59e8 — __ZN5boost3_bi6bind_tIvPFvRKN3RBX9ContentIdENS2_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvS7_NS_10shared_ptrISt6vectorINSA_INS2_8InstanceEEESaISD_EEEEEEEENS0_5list4INS0_5valueIS3_EENS_3argILi1EEENSO_ILi2EEENSM_ISI_EEEEEC2ESK_RKSS_
// type: int __fastcall(int, int, std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>::bind_t(void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>> const&)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvPFvRKN3RBX9ContentIdENS2_14AsyncHttpQueue13RequestResultEPSiNS_8functionIFvS7_NS_10shared_ptrISt6vectorINSA_INS2_8InstanceEEESaISD_EEEEEEEENS0_5list4INS0_5valueIS3_EENS_3argILi1EEENSO_ILi2EEENSM_ISI_EEEEEC2ESK_RKSS_")]
// was: boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>>>::bind_t(void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>)>>> const&)
pub fn stub_7f59e8() -> ! {
    todo!("0x7f59e8 boost::_bi::bind_t<void,void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>>>::bind_t(void (*)(RBX::ContentId const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>),boost::_bi::list4<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>)>>> const&)")
}

// 0x7f5aa4 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_")]
// was: boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)
pub fn stub_7f5aa4() -> ! {
    todo!("0x7f5aa4 rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")
}

// 0x7f5b8c — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const
pub fn stub_7f5b8c() -> ! {
    todo!("0x7f5b8c void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const")
}

// 0x7f5c70 — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_")]
pub fn stub_7f5c70() -> ! {
    todo!("0x7f5c70 boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")
}

// 0x7f5d68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED1Ev")]
pub fn stub_7f5d68() -> ! {
    todo!("0x7f5d68 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")
}

// 0x7f5d6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED0Ev")]
pub fn stub_7f5d6c() -> ! {
    todo!("0x7f5d6c boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")
}

// 0x7f5d70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE7disposeEv")]
pub fn stub_7f5d70() -> ! {
    todo!("0x7f5d70 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::dispose(void)")
}

// 0x7f5d80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE11get_deleterERKSt9type_info")]
pub fn stub_7f5d80() -> ! {
    todo!("0x7f5d80 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_deleter(std::type_info const&)")
}

// 0x7f5d84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE19get_untyped_deleterEv")]
pub fn stub_7f5d84() -> ! {
    todo!("0x7f5d84 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_untyped_deleter(void)")
}

// 0x7f5d88 — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
// type: int __fastcall(int, int, int, int, RBX::AsyncHttpQueue *, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii")]
pub fn stub_7f5d88() -> ! {
    todo!("0x7f5d88 RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")
}

// 0x7f5ed8 — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED1Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED1Ev")]
pub fn stub_7f5ed8() -> ! {
    todo!("0x7f5ed8 RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")
}

// 0x7f5fe0 — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED0Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED0Ev")]
pub fn stub_7f5fe0() -> ! {
    todo!("0x7f5fe0 RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")
}

// 0x7f60f8 — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_")]
// was: RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_7f60f8() -> ! {
    todo!("0x7f60f8 RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x7f6340 — __ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEES5_
#[doc(alias = "RBX::ContentProvider::CachedContent::CachedContent(boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)")]
#[doc(alias = "__ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEES5_")]
// was: RBX::ContentProvider::CachedContent::CachedContent(boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_7f6340() -> ! {
    todo!("0x7f6340 RBX::ContentProvider::CachedContent::CachedContent(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x7f6420 — __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEED2Ev
// type: int __fastcall(std::string *, int, int, int, int, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::~LRUCache()")]
#[doc(alias = "__ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEED2Ev")]
pub fn stub_7f6420() -> ! {
    todo!("0x7f6420 RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::~LRUCache()")
}

// 0x7f6520 — __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm
#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::resize(unsigned long)")]
#[doc(alias = "__ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm")]
pub fn stub_7f6520() -> ! {
    todo!("0x7f6520 RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::resize(unsigned long)")
}

// 0x7f6594 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_clearEv
// type: int __fastcall(int, int, int, int, int, std::string *, int, int, int, int)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_clearEv")]
pub fn stub_7f6594() -> ! {
    todo!("0x7f6594 std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_clear(void)")
}

// 0x7f668c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
pub fn stub_7f668c() -> ! {
    todo!("0x7f668c boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x7f66c4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv")]
pub fn stub_7f66c4() -> ! {
    todo!("0x7f66c4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x7f66f8 — __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::LRUCache(void)")]
#[doc(alias = "__ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEEC2Ev")]
pub fn stub_7f66f8() -> ! {
    todo!("0x7f66f8 RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::LRUCache(void)")
}

// 0x7f67d8 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ContentProvider::CachedContent>::resize(unsigned long)")]
#[doc(alias = "__ZN3RBX20SizeEnforcedLRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm")]
pub fn stub_7f67d8() -> ! {
    todo!("0x7f67d8 RBX::SizeEnforcedLRUCache<std::string,RBX::ContentProvider::CachedContent>::resize(unsigned long)")
}

// 0x7f6850 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> const&)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE")]
pub fn stub_7f6850() -> ! {
    todo!("0x7f6850 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> const&)")
}

// 0x7f68c0 — __ZN5boost6detail8function17function_invoker2IPFbRKSsPSsEbS4_S5_E6invokeERNS1_15function_bufferES4_S5_
#[doc(alias = "boost::detail::function::function_invoker2<bool (*)(std::string const&,std::string *),bool,std::string const&,std::string *>::invoke(boost::detail::function::function_buffer &,std::string const&,std::string *)")]
#[doc(alias = "__ZN5boost6detail8function17function_invoker2IPFbRKSsPSsEbS4_S5_E6invokeERNS1_15function_bufferES4_S5_")]
pub fn stub_7f68c0() -> ! {
    todo!("0x7f68c0 boost::detail::function::function_invoker2<bool (*)(std::string const&,std::string *),bool,std::string const&,std::string *>::invoke(boost::detail::function::function_buffer &,std::string const&,std::string *)")
}

// 0x7f68d4 — __ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_7f68d4() -> ! {
    todo!("0x7f68d4 __ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0x7f69f0 — __ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_7f69f0() -> ! {
    todo!("0x7f69f0 __ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x7f69f4 — __ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_7f69f4() -> ! {
    todo!("0x7f69f4 __ZN3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x7f6a94 — __ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_7f6a94() -> ! {
    todo!("0x7f6a94 __ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x7f6a9c — __ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_7f6a9c() -> ! {
    todo!("0x7f6a9c __ZThn32_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x7f6b40 — __ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_7f6b40() -> ! {
    todo!("0x7f6b40 __ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x7f6b48 — __ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_7f6b48() -> ! {
    todo!("0x7f6b48 __ZThn36_N3RBX10Reflection9DescribedINS_15ContentProviderELZNS_16sContentProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sContentProviderEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x7f6bf0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKcPN3RBX10Reflection18CallbackDescriptorEEES6_SB_NS9_19StringHashPredicateENS9_20StringEqualPredicateEEEE14delete_bucketsEv")]
pub fn stub_7f6bf0() -> ! {
    todo!("0x7f6bf0 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")
}

// 0x7f6c40 — __ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::push_back(RBX::Reflection::Variant const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EE9push_backERKS2_")]
pub fn stub_7f6c40() -> ! {
    todo!("0x7f6c40 std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::push_back(RBX::Reflection::Variant const&)")
}

// 0x7f6ca0 — __ZN3rbx14implementation12typed_holderISsE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<std::string>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderISsE9singletonEv")]
pub fn stub_7f6ca0() -> ! {
    todo!("0x7f6ca0 rbx::implementation::typed_holder<std::string>::singleton(void)")
}

// 0x7f6d10 — __ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,RBX::Reflection::Variant const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_7f6d10() -> ! {
    todo!("0x7f6d10 std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,RBX::Reflection::Variant const&)")
}

// 0x7f7198 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_
#[doc(alias = "rbx::placement_any<RBX::Region3>::operator=(rbx::placement_any<RBX::Region3> const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_")]
pub fn stub_7f7198() -> ! {
    todo!("0x7f7198 rbx::placement_any<RBX::Region3>::operator=(rbx::placement_any<RBX::Region3> const&)")
}

// 0x7f71c8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Reflection7VariantES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Reflection::Variant * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Reflection::Variant *,RBX::Reflection::Variant *>(RBX::Reflection::Variant *,RBX::Reflection::Variant *,RBX::Reflection::Variant *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Reflection7VariantES6_EET0_T_S8_S7_")]
pub fn stub_7f71c8() -> ! {
    todo!("0x7f71c8 RBX::Reflection::Variant * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Reflection::Variant *,RBX::Reflection::Variant *>(RBX::Reflection::Variant *,RBX::Reflection::Variant *,RBX::Reflection::Variant *)")
}

// 0x7f7234 — __ZN3RBX10Reflection14PropDescriptorINS_15ContentProviderEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::PropDescriptor<int (RBX::ContentProvider::*)(void)const,int>(char const*,char const*,int (RBX::ContentProvider::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_15ContentProviderEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_7f7234() -> ! {
    todo!("0x7f7234 RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::PropDescriptor<int (RBX::ContentProvider::*)(void)const,int>(char const*,char const*,int (RBX::ContentProvider::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x7f7344 — __ZN3RBX10Reflection14PropDescriptorINS_15ContentProviderEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_15ContentProviderEiED0Ev")]
pub fn stub_7f7344() -> ! {
    todo!("0x7f7344 RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::~PropDescriptor()")
}

// 0x7f7370 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIiEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<int>(int const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIiEERS3_RKT_")]
pub fn stub_7f7370() -> ! {
    todo!("0x7f7370 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<int>(int const&)")
}

// 0x7f73c0 — __ZN3rbx14implementation12typed_holderIiE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<int>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIiE9singletonEv")]
pub fn stub_7f73c0() -> ! {
    todo!("0x7f73c0 rbx::implementation::typed_holder<int>::singleton(void)")
}

// 0x7f7430 — __ZN3rbx14implementation12typed_holderIiE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<int>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIiE13destruct_funcEPc")]
pub fn stub_7f7430() -> ! {
    todo!("0x7f7430 rbx::implementation::typed_holder<int>::destruct_func(char *)")
}

// 0x7f7438 — __ZN3RBX10Reflection23TypedPropertyDescriptorIiED0Ev
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<int>::~TypedPropertyDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection23TypedPropertyDescriptorIiED0Ev")]
pub fn stub_7f7438() -> ! {
    todo!("0x7f7438 RBX::Reflection::TypedPropertyDescriptor<int>::~TypedPropertyDescriptor()")
}

// 0x7f7464 — __ZNK3RBX10Reflection14PropDescriptorINS_15ContentProviderEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::GetImpl<int (RBX::ContentProvider::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_15ContentProviderEiE7GetImplIMS2_KFivEE10isReadOnlyEv")]
pub fn stub_7f7464() -> ! {
    todo!("0x7f7464 RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::GetImpl<int (RBX::ContentProvider::*)(void)const>::isReadOnly(void)const")
}

// 0x7f7468 — __ZNK3RBX10Reflection14PropDescriptorINS_15ContentProviderEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::GetImpl<int (RBX::ContentProvider::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_15ContentProviderEiE7GetImplIMS2_KFivEE11isWriteOnlyEv")]
pub fn stub_7f7468() -> ! {
    todo!("0x7f7468 RBX::Reflection::PropDescriptor<RBX::ContentProvider,int>::GetImpl<int (RBX::ContentProvider::*)(void)const>::isWriteOnly(void)const")
}
