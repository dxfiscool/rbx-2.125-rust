//! core shard DP — 100 core stubs EA-sorted, next uncovered after DO 0x7f1a54 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0x7f1a78 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_7f1a78() -> ! {
    todo!("0x7f1a78 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>>::~node_constructor()")]
// 0x7f1ac8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEED2Ev
pub fn stub_7f1ac8() -> ! {
    todo!("0x7f1ac8 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0x7f1ae8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_7f1ae8() -> ! {
    todo!("0x7f1ae8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0x7f1c10 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_7f1c10() -> ! {
    todo!("0x7f1c10 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0x7f1ca0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_7f1ca0() -> ! {
    todo!("0x7f1ca0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x7f1ccc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
pub fn stub_7f1ccc() -> ! {
    todo!("0x7f1ccc __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>>::construct(void)")]
// 0x7f1d24 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEE9constructEv
pub fn stub_7f1d24() -> ! {
    todo!("0x7f1d24 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0x7f1d60 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
pub fn stub_7f1d60() -> ! {
    todo!("0x7f1d60 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>::pair(std::string const&,std::pair<unsigned long,RBX::ContentProvider::CachedContent> const&)")]
// 0x7f1dcc — __ZNSt4pairISsS_ImN3RBX15ContentProvider13CachedContentEEEC2ERKSsRKS3_
pub fn stub_7f1dcc() -> ! {
    todo!("0x7f1dcc __ZNSt4pairISsS_ImN3RBX15ContentProvider13CachedContentEEEC2ERKSsRKS3_")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>> const&)")]
// 0x7f1ed8 — __ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_7f1ed8() -> ! {
    todo!("0x7f1ed8 __ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *)")]
// 0x7f202c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
pub fn stub_7f202c() -> ! {
    todo!("0x7f202c __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x7f2088 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_7f2088() -> ! {
    todo!("0x7f2088 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0x7f20b4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_7f20b4() -> ! {
    todo!("0x7f20b4 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>)")]
// 0x7f20f4 — __ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
pub fn stub_7f20f4() -> ! {
    todo!("0x7f20f4 __ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0x7f21dc — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_7f21dc() -> ! {
    todo!("0x7f21dc __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>> & boost::algorithm::iter_split<std::vector<std::string,std::allocator<std::string>>,std::string,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(std::vector<std::string,std::allocator<std::string>> &,std::string &,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
// 0x7f2220 — __ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_
pub fn stub_7f2220() -> ! {
    todo!("0x7f2220 __ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_")
}

#[doc(alias = "boost::algorithm::detail::is_any_ofF<char>::is_any_ofF(boost::algorithm::detail::is_any_ofF<char> const&)")]
// 0x7f24a4 — __ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_
pub fn stub_7f24a4() -> ! {
    todo!("0x7f24a4 __ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to_own(boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>> const&)")]
// 0x7f24d4 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_
pub fn stub_7f24d4() -> ! {
    todo!("0x7f24d4 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::vector<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::allocator<std::string> const&)")]
// 0x7f2504 — __ZNSt6vectorISsSaISsEEC2IN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEET_SH_RKS0_
pub fn stub_7f2504() -> ! {
    todo!("0x7f2504 __ZNSt6vectorISsSaISsEEC2IN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEET_SH_RKS0_")
}

#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_initialize_dispatch<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::__false_type)")]
// 0x7f265c — __ZNSt6vectorISsSaISsEE22_M_initialize_dispatchIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St12__false_type
pub fn stub_7f265c() -> ! {
    todo!("0x7f265c __ZNSt6vectorISsSaISsEE22_M_initialize_dispatchIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St12__false_type")
}

#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_range_initialize<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::input_iterator_tag)")]
// 0x7f2784 — __ZNSt6vectorISsSaISsEE19_M_range_initializeIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St18input_iterator_tag
pub fn stub_7f2784() -> ! {
    todo!("0x7f2784 __ZNSt6vectorISsSaISsEE19_M_range_initializeIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St18input_iterator_tag")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::push_back(std::string const&)")]
// 0x7f28d0 — __ZNSt6vectorISsSaISsEE9push_backERKSs
pub fn stub_7f28d0() -> ! {
    todo!("0x7f28d0 __ZNSt6vectorISsSaISsEE9push_backERKSs")
}

#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::increment(void)")]
// 0x7f2910 — __ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv
pub fn stub_7f2910() -> ! {
    todo!("0x7f2910 __ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::operator()(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")]
// 0x7f2964 — __ZNK5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EclES5_S5_
pub fn stub_7f2964() -> ! {
    todo!("0x7f2964 __ZNK5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EclES5_S5_")
}

#[doc(alias = "char * std::string::_S_construct<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,std::allocator<char> const&,std::forward_iterator_tag)")]
// 0x7f2a30 — __ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSsEEEES2_T_S4_RKSaIcESt20forward_iterator_tag
pub fn stub_7f2a30() -> ! {
    todo!("0x7f2a30 __ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSsEEEES2_T_S4_RKSaIcESt20forward_iterator_tag")
}

#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::equal(boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>> const&)const")]
// 0x7f2a94 — __ZNK5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE5equalERKS6_
pub fn stub_7f2a94() -> ! {
    todo!("0x7f2a94 __ZNK5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE5equalERKS6_")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::clear(void)")]
// 0x7f2b20 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv
pub fn stub_7f2b20() -> ! {
    todo!("0x7f2b20 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv")
}

#[doc(alias = "boost::algorithm::detail::find_iterator_base<__gnu_cxx::__normal_iterator<char *,std::string>>::find_iterator_base<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,int)")]
// 0x7f2b4c — __ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i
pub fn stub_7f2b4c() -> ! {
    todo!("0x7f2b4c __ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i")
}

#[doc(alias = "void boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
// 0x7f2ce0 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_
pub fn stub_7f2ce0() -> ! {
    todo!("0x7f2ce0 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x7f2db8 — __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE6manageERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeE
pub fn stub_7f2db8() -> ! {
    todo!("0x7f2db8 __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE6manageERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker2<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::invoke(boost::detail::function::function_buffer &,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)")]
// 0x7f2dd4 — __ZN5boost6detail8function21function_obj_invoker2INS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEENS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEESD_SD_E6invokeERNS1_15function_bufferESD_SD_
pub fn stub_7f2dd4() -> ! {
    todo!("0x7f2dd4 __ZN5boost6detail8function21function_obj_invoker2INS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEENS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEESD_SD_E6invokeERNS1_15function_bufferESD_SD_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::detail::function::function_buffer &)const")]
// 0x7f2de0 — __ZNK5boost6detail8function13basic_vtable2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES7_S7_E9assign_toINS_9algorithm6detail13token_finderFINSC_10is_any_ofFIcEEEEEEbT_RNS1_15function_bufferE
pub fn stub_7f2de0() -> ! {
    todo!("0x7f2de0 __ZNK5boost6detail8function13basic_vtable2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES7_S7_E9assign_toINS_9algorithm6detail13token_finderFINSC_10is_any_ofFIcEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>> boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>::operator()<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")]
// 0x7f2eb0 — __ZNK5boost9algorithm6detail13token_finderFINS1_10is_any_ofFIcEEEclIN9__gnu_cxx17__normal_iteratorIPcSsEEEENS_14iterator_rangeIT_EESC_SC_
pub fn stub_7f2eb0() -> ! {
    todo!("0x7f2eb0 __ZNK5boost9algorithm6detail13token_finderFINS1_10is_any_ofFIcEEEclIN9__gnu_cxx17__normal_iteratorIPcSsEEEENS_14iterator_rangeIT_EESC_SC_")
}

#[doc(alias = "bool boost::algorithm::detail::is_any_ofF<char>::operator()<char>(char)const")]
// 0x7f2fec — __ZNK5boost9algorithm6detail10is_any_ofFIcEclIcEEbT_
pub fn stub_7f2fec() -> ! {
    todo!("0x7f2fec __ZNK5boost9algorithm6detail10is_any_ofFIcEclIcEEbT_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char *,std::string> std::__find_if<__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>,std::random_access_iterator_tag)")]
// 0x7f3040 — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPcSsEEN5boost9algorithm6detail10is_any_ofFIcEEET_S9_S9_T0_St26random_access_iterator_tag
pub fn stub_7f3040() -> ! {
    todo!("0x7f3040 __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPcSsEEN5boost9algorithm6detail10is_any_ofFIcEEET_S9_S9_T0_St26random_access_iterator_tag")
}

#[doc(alias = "boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>::iterator_range<std::string>(std::string &,boost::iterator_range_detail::range_tag)")]
// 0x7f30f0 — __ZN5boost14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2ISsEERT_NS_21iterator_range_detail9range_tagE
pub fn stub_7f30f0() -> ! {
    todo!("0x7f30f0 __ZN5boost14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2ISsEERT_NS_21iterator_range_detail9range_tagE")
}

#[doc(alias = "boost::algorithm::detail::is_any_ofF<char>::is_any_ofF<boost::iterator_range<char const*>>(boost::iterator_range<char const*> const&)")]
// 0x7f3118 — __ZN5boost9algorithm6detail10is_any_ofFIcEC2INS_14iterator_rangeIPKcEEEERKT_
pub fn stub_7f3118() -> ! {
    todo!("0x7f3118 __ZN5boost9algorithm6detail10is_any_ofFIcEC2INS_14iterator_rangeIPKcEEEERKT_")
}

#[doc(alias = "void std::__introsort_loop<char *,int>(char *,char *,int)")]
// 0x7f3178 — __ZSt16__introsort_loopIPciEvT_S1_T0_
pub fn stub_7f3178() -> ! {
    todo!("0x7f3178 __ZSt16__introsort_loopIPciEvT_S1_T0_")
}

#[doc(alias = "void std::__final_insertion_sort<char *>(char *,char *)")]
// 0x7f323c — __ZSt22__final_insertion_sortIPcEvT_S1_
pub fn stub_7f323c() -> ! {
    todo!("0x7f323c __ZSt22__final_insertion_sortIPcEvT_S1_")
}

#[doc(alias = "void std::__insertion_sort<char *>(char *,char *)")]
// 0x7f328c — __ZSt16__insertion_sortIPcEvT_S1_
pub fn stub_7f328c() -> ! {
    todo!("0x7f328c __ZSt16__insertion_sortIPcEvT_S1_")
}

#[doc(alias = "void std::sort_heap<char *>(char *,char *)")]
// 0x7f32f4 — __ZSt9sort_heapIPcEvT_S1_
pub fn stub_7f32f4() -> ! {
    todo!("0x7f32f4 __ZSt9sort_heapIPcEvT_S1_")
}

#[doc(alias = "void std::pop_heap<char *>(char *,char *)")]
// 0x7f3318 — __ZSt8pop_heapIPcEvT_S1_
pub fn stub_7f3318() -> ! {
    todo!("0x7f3318 __ZSt8pop_heapIPcEvT_S1_")
}

#[doc(alias = "void std::__adjust_heap<char *,int,char>(char *,int,int,char)")]
// 0x7f3328 — __ZSt13__adjust_heapIPcicEvT_T0_S2_T1_
pub fn stub_7f3328() -> ! {
    todo!("0x7f3328 __ZSt13__adjust_heapIPcicEvT_T0_S2_T1_")
}

#[doc(alias = "void std::make_heap<char *>(char *,char *)")]
// 0x7f33a8 — __ZSt9make_heapIPcEvT_S1_
pub fn stub_7f33a8() -> ! {
    todo!("0x7f33a8 __ZSt9make_heapIPcEvT_S1_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>)")]
// 0x7f33d8 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
pub fn stub_7f33d8() -> ! {
    todo!("0x7f33d8 __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>)")]
// 0x7f34e0 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_
pub fn stub_7f34e0() -> ! {
    todo!("0x7f34e0 __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_")
}

#[doc(alias = "void boost::iostreams::detail::close_all<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &)")]
// 0x7f35d8 — __ZN5boost9iostreams6detail9close_allINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEEEEvRT_
pub fn stub_7f35d8() -> ! {
    todo!("0x7f35d8 __ZN5boost9iostreams6detail9close_allINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEEEEvRT_")
}

#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
// 0x7f3694 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS3_ISt19basic_ostringstreamIcS6_SaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_
pub fn stub_7f3694() -> ! {
    todo!("0x7f3694 __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS3_ISt19basic_ostringstreamIcS6_SaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_")
}

#[doc(alias = "rbx_core::SharedPtr<std::string const>::shared_ptr<std::string>(std::string *)")]
// 0x7f37a4 — __ZN5boost10shared_ptrIKSsEC2ISsEEPT_
// was: boost::shared_ptr<std::string const>::shared_ptr<std::string>(std::string *)
pub fn stub_7f37a4() -> ! {
    todo!("0x7f37a4 __ZN5boost10shared_ptrIKSsEC2ISsEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::string>(std::string *)")]
// 0x7f3878 — __ZN5boost6detail12shared_countC2ISsEEPT_
pub fn stub_7f3878() -> ! {
    todo!("0x7f3878 __ZN5boost6detail12shared_countC2ISsEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::~sp_counted_impl_p()")]
// 0x7f3970 — __ZN5boost6detail17sp_counted_impl_pISsED1Ev
pub fn stub_7f3970() -> ! {
    todo!("0x7f3970 __ZN5boost6detail17sp_counted_impl_pISsED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::dispose(void)")]
// 0x7f3978 — __ZN5boost6detail17sp_counted_impl_pISsE7disposeEv
pub fn stub_7f3978() -> ! {
    todo!("0x7f3978 __ZN5boost6detail17sp_counted_impl_pISsE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::get_untyped_deleter(void)")]
// 0x7f3998 — __ZN5boost6detail17sp_counted_impl_pISsE19get_untyped_deleterEv
pub fn stub_7f3998() -> ! {
    todo!("0x7f3998 __ZN5boost6detail17sp_counted_impl_pISsE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x7f3dac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_7f3dac() -> ! {
    todo!("0x7f3dac __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x7f42a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>,boost::shared_ptr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_7f42a0() -> ! {
    todo!("0x7f42a0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
// 0x7f4458 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_
// was: boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)
pub fn stub_7f4458() -> ! {
    todo!("0x7f4458 __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
// 0x7f4560 — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_
// was: boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)>>,boost::_bi::value<boost::shared_ptr<std::string const>>)
pub fn stub_7f4560() -> ! {
    todo!("0x7f4560 __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>)")]
// 0x7f4664 — __ZN5boost3_bi8storage3INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEEEC2ES5_S7_S8_
pub fn stub_7f4664() -> ! {
    todo!("0x7f4664 __ZN5boost3_bi8storage3INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEEEC2ES5_S7_S8_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::ContentId>,boost::arg<1>>::storage2(boost::_bi::value<RBX::ContentId>,boost::arg<1>)")]
// 0x7f4788 — __ZN5boost3_bi8storage2INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEEEC2ES5_S7_
pub fn stub_7f4788() -> ! {
    todo!("0x7f4788 __ZN5boost3_bi8storage2INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEEEC2ES5_S7_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
// 0x7f5aa4 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_
// was: boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)
pub fn stub_7f5aa4() -> ! {
    todo!("0x7f5aa4 __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const")]
// 0x7f5b8c — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const
pub fn stub_7f5b8c() -> ! {
    todo!("0x7f5b8c __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
// 0x7f5c70 — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_
pub fn stub_7f5c70() -> ! {
    todo!("0x7f5c70 __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")]
// 0x7f5d68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED1Ev
pub fn stub_7f5d68() -> ! {
    todo!("0x7f5d68 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::~sp_counted_impl_p()")]
// 0x7f5d6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED0Ev
pub fn stub_7f5d6c() -> ! {
    todo!("0x7f5d6c __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::dispose(void)")]
// 0x7f5d70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE7disposeEv
pub fn stub_7f5d70() -> ! {
    todo!("0x7f5d70 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_deleter(std::type_info const&)")]
// 0x7f5d80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE11get_deleterERKSt9type_info
pub fn stub_7f5d80() -> ! {
    todo!("0x7f5d80 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::get_untyped_deleter(void)")]
// 0x7f5d84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE19get_untyped_deleterEv
pub fn stub_7f5d84() -> ! {
    todo!("0x7f5d84 __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_15ContentProvider13CachedContentELb0EEEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")]
// 0x7f5ed8 — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED1Ev
pub fn stub_7f5ed8() -> ! {
    todo!("0x7f5ed8 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED1Ev")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::~AsyncHttpCache()")]
// 0x7f5fe0 — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED0Ev
pub fn stub_7f5fe0() -> ! {
    todo!("0x7f5fe0 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EED0Ev")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// 0x7f60f8 — __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
// was: RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_7f60f8() -> ! {
    todo!("0x7f60f8 __ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_")
}

#[doc(alias = "RBX::ContentProvider::CachedContent::CachedContent(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// 0x7f6340 — __ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEES5_
// was: RBX::ContentProvider::CachedContent::CachedContent(boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_7f6340() -> ! {
    todo!("0x7f6340 __ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEES5_")
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::~LRUCache()")]
// 0x7f6420 — __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEED2Ev
pub fn stub_7f6420() -> ! {
    todo!("0x7f6420 __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEED2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::resize(unsigned long)")]
// 0x7f6520 — __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm
pub fn stub_7f6520() -> ! {
    todo!("0x7f6520 __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm")
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_clear(void)")]
// 0x7f6594 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_clearEv
pub fn stub_7f6594() -> ! {
    todo!("0x7f6594 __ZNSt10_List_baseISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_clearEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0x7f668c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_7f668c() -> ! {
    todo!("0x7f668c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// 0x7f66c4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_7f66c4() -> ! {
    todo!("0x7f66c4 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv")
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::LRUCache(void)")]
// 0x7f66f8 — __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEEC2Ev
pub fn stub_7f66f8() -> ! {
    todo!("0x7f66f8 __ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEEC2Ev")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ContentProvider::CachedContent>::resize(unsigned long)")]
// 0x7f67d8 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm
pub fn stub_7f67d8() -> ! {
    todo!("0x7f67d8 __ZN3RBX20SizeEnforcedLRUCacheISsNS_15ContentProvider13CachedContentEE6resizeEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> const&)")]
// 0x7f6850 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
pub fn stub_7f6850() -> ! {
    todo!("0x7f6850 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE")
}

#[doc(alias = "boost::detail::function::function_invoker2<bool (*)(std::string const&,std::string *),bool,std::string const&,std::string *>::invoke(boost::detail::function::function_buffer &,std::string const&,std::string *)")]
// 0x7f68c0 — __ZN5boost6detail8function17function_invoker2IPFbRKSsPSsEbS4_S5_E6invokeERNS1_15function_bufferES4_S5_
pub fn stub_7f68c0() -> ! {
    todo!("0x7f68c0 __ZN5boost6detail8function17function_invoker2IPFbRKSsPSsEbS4_S5_E6invokeERNS1_15function_bufferES4_S5_")
}

#[doc(alias = "rbx::implementation::typed_holder<std::string>::singleton(void)")]
// 0x7f6ca0 — __ZN3rbx14implementation12typed_holderISsE9singletonEv
pub fn stub_7f6ca0() -> ! {
    todo!("0x7f6ca0 __ZN3rbx14implementation12typed_holderISsE9singletonEv")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>::operator=(rbx::placement_any<RBX::Region3> const&)")]
// 0x7f7198 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_
pub fn stub_7f7198() -> ! {
    todo!("0x7f7198 __ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<int>(int const&)")]
// 0x7f7370 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIiEERS3_RKT_
pub fn stub_7f7370() -> ! {
    todo!("0x7f7370 __ZN3rbx13placement_anyIN3RBX7Region3EEaSIiEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<int>::singleton(void)")]
// 0x7f73c0 — __ZN3rbx14implementation12typed_holderIiE9singletonEv
pub fn stub_7f73c0() -> ! {
    todo!("0x7f73c0 __ZN3rbx14implementation12typed_holderIiE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<int>::destruct_func(char *)")]
// 0x7f7430 — __ZN3rbx14implementation12typed_holderIiE13destruct_funcEPc
pub fn stub_7f7430() -> ! {
    todo!("0x7f7430 __ZN3rbx14implementation12typed_holderIiE13destruct_funcEPc")
}

#[doc(alias = "boost::scoped_ptr<RBX::ContentId>::~scoped_ptr()")]
// 0x7f7c58 — __ZN5boost10scoped_ptrIN3RBX9ContentIdEED2Ev
pub fn stub_7f7c58() -> ! {
    todo!("0x7f7c58 __ZN5boost10scoped_ptrIN3RBX9ContentIdEED2Ev")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to_own(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>> const&)")]
// 0x7f8928 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE13assign_to_ownERKS8_
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to_own(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>> const&)
pub fn stub_7f8928() -> ! {
    todo!("0x7f8928 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE13assign_to_ownERKS8_")
}

#[doc(alias = "RBX::ContentId::ContentId(std::string const&,RBX::Name const&)")]
// 0x7f8958 — __ZN3RBX9ContentIdC2ERKSsRKNS_4NameE
pub fn stub_7f8958() -> ! {
    todo!("0x7f8958 __ZN3RBX9ContentIdC2ERKSsRKNS_4NameE")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::string>> *)")]
// 0x7f8a08 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_7f8a08() -> ! {
    todo!("0x7f8a08 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "RBX::ContentProvider::CachedContent::CachedContent(rbx_core::SharedPtr<std::string const>)")]
// 0x7f8a44 — __ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEE
// was: RBX::ContentProvider::CachedContent::CachedContent(boost::shared_ptr<std::string const>)
pub fn stub_7f8a44() -> ! {
    todo!("0x7f8a44 __ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEE")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::operator()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)const")]
// 0x7f8b44 — __ZNK5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEclES3_S4_S7_
// was: boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::operator()(RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)const
pub fn stub_7f8b44() -> ! {
    todo!("0x7f8b44 __ZNK5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEclES3_S4_S7_")
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::clear(void)")]
// 0x7f8f00 — __ZN5boost9function2IbRKSsPSsE5clearEv
pub fn stub_7f8f00() -> ! {
    todo!("0x7f8f00 __ZN5boost9function2IbRKSsPSsE5clearEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::insert(rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot *)")]
// 0x7fd938 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE6insertEPNS8_4slotE
pub fn stub_7fd938() -> ! {
    todo!("0x7fd938 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE6insertEPNS8_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot>::operator=(rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot*)")]
// 0x7fdb44 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES7_EEE4slotEEaSEPSB_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot>::operator=(rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot*)
pub fn stub_7fdb44() -> ! {
    todo!("0x7fdb44 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES7_EEE4slotEEaSEPSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot::disconnect(void)")]
// 0x7fdc68 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot10disconnectEv
pub fn stub_7fdc68() -> ! {
    todo!("0x7fdc68 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot::connected(void)const")]
// 0x7fdd78 — __ZNK3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot9connectedEv
pub fn stub_7fdd78() -> ! {
    todo!("0x7fdd78 __ZNK3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot9connectedEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::remove(rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot *)")]
// 0x7fde00 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE6removeEPNS8_4slotE
pub fn stub_7fde00() -> ! {
    todo!("0x7fde00 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE6removeEPNS8_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot::safe_static_init_mutex(void)")]
// 0x7fdef0 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot22safe_static_init_mutexEv
pub fn stub_7fdef0() -> ! {
    todo!("0x7fdef0 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot::safe_static_do_get_mutex(void)")]
// 0x7fdef4 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot24safe_static_do_get_mutexEv
pub fn stub_7fdef4() -> ! {
    todo!("0x7fdef4 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot::~slot()")]
// 0x7fdfe4 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slotD1Ev
pub fn stub_7fdfe4() -> ! {
    todo!("0x7fdfe4 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slotD1Ev")
}
