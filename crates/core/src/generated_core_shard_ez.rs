//! core shard EZ — 100 core stubs EA-sorted, lowest uncovered 0xcaa994..0xeb2280 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EY 0xc3a2bc).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xc3a2bc.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::map<unsigned long,std::string,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::operator[](unsigned long const&)")]
// 0xcaa994 — __ZNSt3mapImSsSt4lessImESaISt4pairIKmSsEEEixERS3_
pub fn stub_caa994() -> ! {
    todo!("0xcaa994 __ZNSt3mapImSsSt4lessImESaISt4pairIKmSsEEEixERS3_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned long const,std::string>>,std::pair<unsigned long const,std::string> const&)")]
// 0xcaac64 — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_caac64() -> ! {
    todo!("0xcaac64 __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long const,std::string> const&)")]
// 0xcaad18 — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_caad18() -> ! {
    todo!("0xcaad18 __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert_unique(std::pair<unsigned long const,std::string> const&)")]
// 0xcaae44 — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_caae44() -> ! {
    todo!("0xcaae44 __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,std::string>> *)")]
// 0xcabfdc — __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_cabfdc() -> ! {
    todo!("0xcabfdc __ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned short>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xd0d7ac — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_d0d7ac() -> ! {
    todo!("0xd0d7ac __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xd0dee0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_d0dee0() -> ! {
    todo!("0xd0dee0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0xd0dfb8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_d0dfb8() -> ! {
    todo!("0xd0dfb8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned short>>>>::construct_with_value<boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>(boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type> &&)")]
// 0xd0e22c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSstEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_
pub fn stub_d0e22c() -> ! {
    todo!("0xd0e22c __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSstEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xd0e2c8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_d0e2c8() -> ! {
    todo!("0xd0e2c8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
// 0xd0eb70 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEED2Ev
pub fn stub_d0eb70() -> ! {
    todo!("0xd0eb70 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEED2Ev")
}

#[doc(alias = "std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>>(std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>)")]
// 0xd9950c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_
pub fn stub_d9950c() -> ! {
    todo!("0xd9950c __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *> std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>>(std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>)")]
// 0xd995b0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_
pub fn stub_d995b0() -> ! {
    todo!("0xd995b0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned int>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xdded98 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_dded98() -> ! {
    todo!("0xdded98 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0xddf130 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_ddf130() -> ! {
    todo!("0xddf130 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned int>>>>::construct_with_value<boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>(boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type> &&)")]
// 0xddf3a4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsjEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_
pub fn stub_ddf3a4() -> ! {
    todo!("0xddf3a4 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsjEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xddf440 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_ddf440() -> ! {
    todo!("0xddf440 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xddf5e8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_ddf5e8() -> ! {
    todo!("0xddf5e8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
// 0xde159c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEED2Ev
pub fn stub_de159c() -> ! {
    todo!("0xde159c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEED2Ev")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::append(unsigned short const*,unsigned long)")]
// 0xe51b14 — __ZNSbItSt11char_traitsItESaItEE6appendEPKtm
pub fn stub_e51b14() -> ! {
    todo!("0xe51b14 __ZNSbItSt11char_traitsItESaItEE6appendEPKtm")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_M_mutate(unsigned long,unsigned long,unsigned long)")]
// 0xe51c90 — __ZNSbItSt11char_traitsItESaItEE9_M_mutateEmmm
pub fn stub_e51c90() -> ! {
    todo!("0xe51c90 __ZNSbItSt11char_traitsItESaItEE9_M_mutateEmmm")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_Rep::_M_clone(std::allocator<unsigned short> const&,unsigned long)")]
// 0xe51e28 — __ZNSbItSt11char_traitsItESaItEE4_Rep8_M_cloneERKS1_m
pub fn stub_e51e28() -> ! {
    todo!("0xe51e28 __ZNSbItSt11char_traitsItESaItEE4_Rep8_M_cloneERKS1_m")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert_unique(std::pair<std::string const,unsigned short> const&)")]
// 0xe5a058 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_e5a058() -> ! {
    todo!("0xe5a058 __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned short> const&)")]
// 0xe5a13c — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_e5a13c() -> ! {
    todo!("0xe5a13c __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::find(std::string const&)")]
// 0xe5a284 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_e5a284() -> ! {
    todo!("0xe5a284 __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned short>> *)")]
// 0xe5a3f8 — __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_e5a3f8() -> ! {
    todo!("0xe5a3f8 __ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>> *,boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>> *)")]
// 0xe7eea0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
pub fn stub_e7eea0() -> ! {
    todo!("0xe7eea0 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<std::pair<unsigned int const,unsigned int>>(unsigned int const&,std::pair<unsigned int const,unsigned int> &&)")]
// 0xe7ef34 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS6_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERS5_DpOT_
pub fn stub_e7ef34() -> ! {
    todo!("0xe7ef34 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS6_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERS5_DpOT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
// 0xe7f100 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
pub fn stub_e7f100() -> ! {
    todo!("0xe7f100 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
// 0xe7f2a8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
pub fn stub_e7f2a8() -> ! {
    todo!("0xe7f2a8 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm")
}

#[doc(alias = "RBX::ReadFileMesh(std::istream &)")]
// 0xea7378 — __ZN3RBX12ReadFileMeshERSi
pub fn stub_ea7378() -> ! {
    todo!("0xea7378 __ZN3RBX12ReadFileMeshERSi")
}

#[doc(alias = "anonymous namespace::computeAABB(RBX::FileMeshData *)")]
// 0xea7dac — __ZN12_GLOBAL__N_111computeAABBEPN3RBX12FileMeshDataE
// was: `anonymous namespace::computeAABB(RBX::FileMeshData *)
pub fn stub_ea7dac() -> ! {
    todo!("0xea7dac __ZN12_GLOBAL__N_111computeAABBEPN3RBX12FileMeshDataE")
}

#[doc(alias = "anonymous namespace::readMeshFromV1(std::istream &,float)")]
// 0xea7f00 — __ZN12_GLOBAL__N_114readMeshFromV1ERSif
// was: `anonymous namespace::readMeshFromV1(std::istream &,float)
pub fn stub_ea7f00() -> ! {
    todo!("0xea7f00 __ZN12_GLOBAL__N_114readMeshFromV1ERSif")
}

#[doc(alias = "anonymous namespace::optimizeMesh(RBX::FileMeshData &)")]
// 0xea85d8 — __ZN12_GLOBAL__N_112optimizeMeshERN3RBX12FileMeshDataE
// was: `anonymous namespace::optimizeMesh(RBX::FileMeshData &)
pub fn stub_ea85d8() -> ! {
    todo!("0xea85d8 __ZN12_GLOBAL__N_112optimizeMeshERN3RBX12FileMeshDataE")
}

#[doc(alias = "std::_Rb_tree<RBX::FileMeshVertexNormalTexture3d,std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>,std::_Select1st<std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>>,anonymous namespace::MeshVertexComparator,std::allocator<std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>> *)")]
// 0xea8a30 — __ZNSt8_Rb_treeIN3RBX29FileMeshVertexNormalTexture3dESt4pairIKS1_jESt10_Select1stIS4_EN12_GLOBAL__N_120MeshVertexComparatorESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<RBX::FileMeshVertexNormalTexture3d,std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>,std::_Select1st<std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>>,`anonymous namespace::MeshVertexComparator,std::allocator<std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::FileMeshVertexNormalTexture3d const,unsigned int>> *)
pub fn stub_ea8a30() -> ! {
    todo!("0xea8a30 __ZNSt8_Rb_treeIN3RBX29FileMeshVertexNormalTexture3dESt4pairIKS1_jESt10_Select1stIS4_EN12_GLOBAL__N_120MeshVertexComparatorESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FileMeshVertexNormalTexture3d*,std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>>,unsigned long,RBX::FileMeshVertexNormalTexture3d const&)")]
// 0xea8bb8 — __ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_ea8bb8() -> ! {
    todo!("0xea8bb8 __ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,unsigned long,RBX::FileMeshFace const&)")]
// 0xea8f04 — __ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_ea8f04() -> ! {
    todo!("0xea8f04 __ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::reserve(unsigned long)")]
// 0xea91a8 — __ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE7reserveEm
pub fn stub_ea91a8() -> ! {
    todo!("0xea91a8 __ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::reserve(unsigned long)")]
// 0xea9254 — __ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE7reserveEm
pub fn stub_ea9254() -> ! {
    todo!("0xea9254 __ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,RBX::FileMeshFace const&)")]
// 0xea92f0 — __ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_ea92f0() -> ! {
    todo!("0xea92f0 __ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FileMeshVertexNormalTexture3d*,std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>>,RBX::FileMeshVertexNormalTexture3d const&)")]
// 0xea9498 — __ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_ea9498() -> ! {
    todo!("0xea9498 __ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "void std::vector<char,std::allocator<char>>::_M_range_insert<char *>(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,char *,char *,std::forward_iterator_tag)")]
// 0xea96b0 — __ZNSt6vectorIcSaIcEE15_M_range_insertIPcEEvN9__gnu_cxx17__normal_iteratorIS3_S1_EET_S7_St20forward_iterator_tag
pub fn stub_ea96b0() -> ! {
    todo!("0xea96b0 __ZNSt6vectorIcSaIcEE15_M_range_insertIPcEEvN9__gnu_cxx17__normal_iteratorIS3_S1_EET_S7_St20forward_iterator_tag")
}

#[doc(alias = "RBX::AdornBillboarder::isVisibleAndValid(void)const")]
// 0xeaa004 — __ZNK3RBX16AdornBillboarder17isVisibleAndValidEv
pub fn stub_eaa004() -> ! {
    todo!("0xeaa004 __ZNK3RBX16AdornBillboarder17isVisibleAndValidEv")
}

#[doc(alias = "RBX::AdornBillboarder::getViewport(void)const")]
// 0xeaa00c — __ZNK3RBX16AdornBillboarder11getViewportEv
pub fn stub_eaa00c() -> ! {
    todo!("0xeaa00c __ZNK3RBX16AdornBillboarder11getViewportEv")
}

#[doc(alias = "RBX::ViewportBillboarder::ViewportBillboarder(void)")]
// 0xeaa6c0 — __ZN3RBX19ViewportBillboarderC1Ev
pub fn stub_eaa6c0() -> ! {
    todo!("0xeaa6c0 __ZN3RBX19ViewportBillboarderC1Ev")
}

#[doc(alias = "RBX::AdornBillboarder::getCamera(void)const")]
// 0xeab23c — __ZNK3RBX16AdornBillboarder9getCameraEv
pub fn stub_eab23c() -> ! {
    todo!("0xeab23c __ZNK3RBX16AdornBillboarder9getCameraEv")
}

#[doc(alias = "RBX::AdornBillboarder::~AdornBillboarder()")]
// 0xeab240 — __ZN3RBX16AdornBillboarderD1Ev
pub fn stub_eab240() -> ! {
    todo!("0xeab240 __ZN3RBX16AdornBillboarderD1Ev")
}

#[doc(alias = "RBX::AdornBillboarder::~AdornBillboarder()")]
// 0xeab24c — __ZN3RBX16AdornBillboarderD0Ev
pub fn stub_eab24c() -> ! {
    todo!("0xeab24c __ZN3RBX16AdornBillboarderD0Ev")
}

#[doc(alias = "RBX::AdornBillboarder::createTextureProxy(RBX::ContentId const&,bool &,bool)")]
// 0xeab2ec — __ZN3RBX16AdornBillboarder18createTextureProxyERKNS_9ContentIdERbb
pub fn stub_eab2ec() -> ! {
    todo!("0xeab2ec __ZN3RBX16AdornBillboarder18createTextureProxyERKNS_9ContentIdERbb")
}

#[doc(alias = "RBX::AdornBillboarder::setTexture(int,rbx_core::SharedPtr<RBX::TextureProxyBase> const&)")]
// 0xeab30c — __ZN3RBX16AdornBillboarder10setTextureEiRKN5boost10shared_ptrINS_16TextureProxyBaseEEE
// was: RBX::AdornBillboarder::setTexture(int,boost::shared_ptr<RBX::TextureProxyBase> const&)
pub fn stub_eab30c() -> ! {
    todo!("0xeab30c __ZN3RBX16AdornBillboarder10setTextureEiRKN5boost10shared_ptrINS_16TextureProxyBaseEEE")
}

#[doc(alias = "RBX::AdornBillboarder::getTextureSize(rbx_core::SharedPtr<RBX::TextureProxyBase> const&)const")]
// 0xeab31c — __ZNK3RBX16AdornBillboarder14getTextureSizeERKN5boost10shared_ptrINS_16TextureProxyBaseEEE
// was: RBX::AdornBillboarder::getTextureSize(boost::shared_ptr<RBX::TextureProxyBase> const&)const
pub fn stub_eab31c() -> ! {
    todo!("0xeab31c __ZNK3RBX16AdornBillboarder14getTextureSizeERKN5boost10shared_ptrINS_16TextureProxyBaseEEE")
}

#[doc(alias = "RBX::AdornBillboarder::getRenderCaps(void)const")]
// 0xeac1f0 — __ZNK3RBX16AdornBillboarder13getRenderCapsEv
pub fn stub_eac1f0() -> ! {
    todo!("0xeac1f0 __ZNK3RBX16AdornBillboarder13getRenderCapsEv")
}

#[doc(alias = "RBX::FrameRateManager::FrameRateManager(void)")]
// 0xeac5f0 — __ZN3RBX16FrameRateManagerC1Ev
pub fn stub_eac5f0() -> ! {
    todo!("0xeac5f0 __ZN3RBX16FrameRateManagerC1Ev")
}

#[doc(alias = "RBX::FrameRateManager::FrameRateManager(void)")]
// 0xeac5fc — __ZN3RBX16FrameRateManagerC2Ev
pub fn stub_eac5fc() -> ! {
    todo!("0xeac5fc __ZN3RBX16FrameRateManagerC2Ev")
}

#[doc(alias = "RBX::FrameRateManager::getAntialiasingMode(void)")]
// 0xeac948 — __ZN3RBX16FrameRateManager19getAntialiasingModeEv
pub fn stub_eac948() -> ! {
    todo!("0xeac948 __ZN3RBX16FrameRateManager19getAntialiasingModeEv")
}

#[doc(alias = "RBX::FrameRateManager::~FrameRateManager()")]
// 0xeac958 — __ZN3RBX16FrameRateManagerD1Ev
pub fn stub_eac958() -> ! {
    todo!("0xeac958 __ZN3RBX16FrameRateManagerD1Ev")
}

#[doc(alias = "RBX::FrameRateManager::~FrameRateManager()")]
// 0xeac964 — __ZN3RBX16FrameRateManagerD2Ev
pub fn stub_eac964() -> ! {
    todo!("0xeac964 __ZN3RBX16FrameRateManagerD2Ev")
}

#[doc(alias = "RBX::FrameRateManager::TestAndAddBlockQuota(int,int,float,float,RBX::eShadowCullingPriority)")]
// 0xeaca48 — __ZN3RBX16FrameRateManager20TestAndAddBlockQuotaEiiffNS_22eShadowCullingPriorityE
pub fn stub_eaca48() -> ! {
    todo!("0xeaca48 __ZN3RBX16FrameRateManager20TestAndAddBlockQuotaEiiffNS_22eShadowCullingPriorityE")
}

#[doc(alias = "RBX::FrameRateManager::SubmitCurrentFrame(double,double,double)")]
// 0xeacae8 — __ZN3RBX16FrameRateManager18SubmitCurrentFrameEddd
pub fn stub_eacae8() -> ! {
    todo!("0xeacae8 __ZN3RBX16FrameRateManager18SubmitCurrentFrameEddd")
}

#[doc(alias = "RBX::FrameRateManager::UpdateStats(double,double,double)")]
// 0xeacd40 — __ZN3RBX16FrameRateManager11UpdateStatsEddd
pub fn stub_eacd40() -> ! {
    todo!("0xeacd40 __ZN3RBX16FrameRateManager11UpdateStatsEddd")
}

#[doc(alias = "RBX::FrameRateManager::AdjustQuality(double,double,bool)")]
// 0xeace90 — __ZN3RBX16FrameRateManager13AdjustQualityEddb
pub fn stub_eace90() -> ! {
    todo!("0xeace90 __ZN3RBX16FrameRateManager13AdjustQualityEddb")
}

#[doc(alias = "RBX::FrameRateManager::StartCapturingMetrics(void)")]
// 0xead260 — __ZN3RBX16FrameRateManager21StartCapturingMetricsEv
pub fn stub_ead260() -> ! {
    todo!("0xead260 __ZN3RBX16FrameRateManager21StartCapturingMetricsEv")
}

#[doc(alias = "RBX::FrameRateManager::GetTargetFrameTimeForNextLevel(void)const")]
// 0xead2a0 — __ZNK3RBX16FrameRateManager30GetTargetFrameTimeForNextLevelEv
pub fn stub_ead2a0() -> ! {
    todo!("0xead2a0 __ZNK3RBX16FrameRateManager30GetTargetFrameTimeForNextLevelEv")
}

#[doc(alias = "RBX::FrameRateManager::GetTargetRenderTimeForNextLevel(void)const")]
// 0xead2c0 — __ZNK3RBX16FrameRateManager31GetTargetRenderTimeForNextLevelEv
pub fn stub_ead2c0() -> ! {
    todo!("0xead2c0 __ZNK3RBX16FrameRateManager31GetTargetRenderTimeForNextLevelEv")
}

#[doc(alias = "RBX::FrameRateManager::StepQuality(bool,bool)")]
// 0xead318 — __ZN3RBX16FrameRateManager11StepQualityEbb
pub fn stub_ead318() -> ! {
    todo!("0xead318 __ZN3RBX16FrameRateManager11StepQualityEbb")
}

#[doc(alias = "RBX::FrameRateManager::getMetricValue(std::string const&)")]
// 0xead528 — __ZN3RBX16FrameRateManager14getMetricValueERKSs
pub fn stub_ead528() -> ! {
    todo!("0xead528 __ZN3RBX16FrameRateManager14getMetricValueERKSs")
}

#[doc(alias = "RBX::FrameRateManager::GetViewCullDistance(void)")]
// 0xead770 — __ZN3RBX16FrameRateManager19GetViewCullDistanceEv
pub fn stub_ead770() -> ! {
    todo!("0xead770 __ZN3RBX16FrameRateManager19GetViewCullDistanceEv")
}

#[doc(alias = "RBX::FrameRateManager::GetFrameTimeAverage(void)")]
// 0xead784 — __ZN3RBX16FrameRateManager19GetFrameTimeAverageEv
pub fn stub_ead784() -> ! {
    todo!("0xead784 __ZN3RBX16FrameRateManager19GetFrameTimeAverageEv")
}

#[doc(alias = "RBX::FrameRateManager::GetPrepareTimeAverage(void)")]
// 0xead7a4 — __ZN3RBX16FrameRateManager21GetPrepareTimeAverageEv
pub fn stub_ead7a4() -> ! {
    todo!("0xead7a4 __ZN3RBX16FrameRateManager21GetPrepareTimeAverageEv")
}

#[doc(alias = "RBX::FrameRateManager::GetRenderTimeAverage(void)")]
// 0xead7c4 — __ZN3RBX16FrameRateManager20GetRenderTimeAverageEv
pub fn stub_ead7c4() -> ! {
    todo!("0xead7c4 __ZN3RBX16FrameRateManager20GetRenderTimeAverageEv")
}

#[doc(alias = "RBX::FrameRateManager::GetRenderTimeStats(void)")]
// 0xead7e8 — __ZN3RBX16FrameRateManager18GetRenderTimeStatsEv
pub fn stub_ead7e8() -> ! {
    todo!("0xead7e8 __ZN3RBX16FrameRateManager18GetRenderTimeStatsEv")
}

#[doc(alias = "RBX::FrameRateManager::GetViewCullSqDistance(void)")]
// 0xead7ec — __ZN3RBX16FrameRateManager21GetViewCullSqDistanceEv
pub fn stub_ead7ec() -> ! {
    todo!("0xead7ec __ZN3RBX16FrameRateManager21GetViewCullSqDistanceEv")
}

#[doc(alias = "RBX::FrameRateManager::getShadingDistance(void)const")]
// 0xead7f0 — __ZNK3RBX16FrameRateManager18getShadingDistanceEv
pub fn stub_ead7f0() -> ! {
    todo!("0xead7f0 __ZNK3RBX16FrameRateManager18getShadingDistanceEv")
}

#[doc(alias = "RBX::FrameRateManager::getPhysicsThrottling(void)const")]
// 0xead804 — __ZNK3RBX16FrameRateManager20getPhysicsThrottlingEv
pub fn stub_ead804() -> ! {
    todo!("0xead804 __ZNK3RBX16FrameRateManager20getPhysicsThrottlingEv")
}

#[doc(alias = "RBX::FrameRateManager::getShadingSqDistance(void)const")]
// 0xead818 — __ZNK3RBX16FrameRateManager20getShadingSqDistanceEv
pub fn stub_ead818() -> ! {
    todo!("0xead818 __ZNK3RBX16FrameRateManager20getShadingSqDistanceEv")
}

#[doc(alias = "RBX::FrameRateManager::getLightGridRadius(void)const")]
// 0xead834 — __ZNK3RBX16FrameRateManager18getLightGridRadiusEv
pub fn stub_ead834() -> ! {
    todo!("0xead834 __ZNK3RBX16FrameRateManager18getLightGridRadiusEv")
}

#[doc(alias = "RBX::FrameRateManager::getLightingNonFixedEnabled(void)const")]
// 0xead848 — __ZNK3RBX16FrameRateManager26getLightingNonFixedEnabledEv
pub fn stub_ead848() -> ! {
    todo!("0xead848 __ZNK3RBX16FrameRateManager26getLightingNonFixedEnabledEv")
}

#[doc(alias = "RBX::FrameRateManager::getLightingChunkBudget(void)const")]
// 0xead85c — __ZNK3RBX16FrameRateManager22getLightingChunkBudgetEv
pub fn stub_ead85c() -> ! {
    todo!("0xead85c __ZNK3RBX16FrameRateManager22getLightingChunkBudgetEv")
}

#[doc(alias = "RBX::FrameRateManager::GetMaxNextViewCullDistance(void)")]
// 0xead870 — __ZN3RBX16FrameRateManager26GetMaxNextViewCullDistanceEv
pub fn stub_ead870() -> ! {
    todo!("0xead870 __ZN3RBX16FrameRateManager26GetMaxNextViewCullDistanceEv")
}

#[doc(alias = "RBX::FrameRateManager::getSSAOLevel(void)")]
// 0xead898 — __ZN3RBX16FrameRateManager12getSSAOLevelEv
pub fn stub_ead898() -> ! {
    todo!("0xead898 __ZN3RBX16FrameRateManager12getSSAOLevelEv")
}

#[doc(alias = "RBX::FrameRateManager::Configure(RBX::RenderCaps const*,RBX::CRenderSettings *)")]
// 0xead8c0 — __ZN3RBX16FrameRateManager9ConfigureEPKNS_10RenderCapsEPNS_15CRenderSettingsE
pub fn stub_ead8c0() -> ! {
    todo!("0xead8c0 __ZN3RBX16FrameRateManager9ConfigureEPKNS_10RenderCapsEPNS_15CRenderSettingsE")
}

#[doc(alias = "RBX::FrameRateManager::CountParticleQuotas(int)")]
// 0xead9ac — __ZN3RBX16FrameRateManager19CountParticleQuotasEi
pub fn stub_ead9ac() -> ! {
    todo!("0xead9ac __ZN3RBX16FrameRateManager19CountParticleQuotasEi")
}

#[doc(alias = "RBX::FrameRateManager::GetParticleThrottleFactor(void)")]
// 0xeada68 — __ZN3RBX16FrameRateManager25GetParticleThrottleFactorEv
pub fn stub_eada68() -> ! {
    todo!("0xeada68 __ZN3RBX16FrameRateManager25GetParticleThrottleFactorEv")
}

#[doc(alias = "RBX::WindowAverage<double,double>::getStats(unsigned long)const")]
// 0xeadac0 — __ZNK3RBX13WindowAverageIddE8getStatsEm
pub fn stub_eadac0() -> ! {
    todo!("0xeadac0 __ZNK3RBX13WindowAverageIddE8getStatsEm")
}

#[doc(alias = "RBX::GfxBinding::~GfxBinding()")]
// 0xeafa14 — __ZN3RBX10GfxBindingD2Ev
pub fn stub_eafa14() -> ! {
    todo!("0xeafa14 __ZN3RBX10GfxBindingD2Ev")
}

#[doc(alias = "RBX::GfxBinding::isBound(void)")]
// 0xeafb64 — __ZN3RBX10GfxBinding7isBoundEv
pub fn stub_eafb64() -> ! {
    todo!("0xeafb64 __ZN3RBX10GfxBinding7isBoundEv")
}

#[doc(alias = "RBX::GfxBinding::zombify(void)")]
// 0xeb0610 — __ZN3RBX10GfxBinding7zombifyEv
pub fn stub_eb0610() -> ! {
    todo!("0xeb0610 __ZN3RBX10GfxBinding7zombifyEv")
}

#[doc(alias = "RBX::GfxBinding::unbind(void)")]
// 0xeb0628 — __ZN3RBX10GfxBinding6unbindEv
pub fn stub_eb0628() -> ! {
    todo!("0xeb0628 __ZN3RBX10GfxBinding6unbindEv")
}

#[doc(alias = "RBX::GfxBinding::onSpecialShapeChangedEx(void)")]
// 0xeb0670 — __ZN3RBX10GfxBinding23onSpecialShapeChangedExEv
pub fn stub_eb0670() -> ! {
    todo!("0xeb0670 __ZN3RBX10GfxBinding23onSpecialShapeChangedExEv")
}

#[doc(alias = "RBX::GfxAttachment::unbind(void)")]
// 0xeb0b38 — __ZN3RBX13GfxAttachment6unbindEv
pub fn stub_eb0b38() -> ! {
    todo!("0xeb0b38 __ZN3RBX13GfxAttachment6unbindEv")
}

#[doc(alias = "RBX::IAdornable::~IAdornable()")]
// 0xeb1660 — __ZN3RBX10IAdornableD2Ev
pub fn stub_eb1660() -> ! {
    todo!("0xeb1660 __ZN3RBX10IAdornableD2Ev")
}

#[doc(alias = "RBX::IAdornable::shouldRenderSetDirty(void)")]
// 0xeb17e4 — __ZN3RBX10IAdornable20shouldRenderSetDirtyEv
pub fn stub_eb17e4() -> ! {
    todo!("0xeb17e4 __ZN3RBX10IAdornable20shouldRenderSetDirtyEv")
}

#[doc(alias = "RBX::IAdornableCollector::recomputeShouldRender(RBX::IAdornable *)")]
// 0xeb17f8 — __ZN3RBX19IAdornableCollector21recomputeShouldRenderEPNS_10IAdornableE
pub fn stub_eb17f8() -> ! {
    todo!("0xeb17f8 __ZN3RBX19IAdornableCollector21recomputeShouldRenderEPNS_10IAdornableE")
}

#[doc(alias = "RBX::IAdornableCollector::~IAdornableCollector()")]
// 0xeb1b80 — __ZN3RBX19IAdornableCollectorD1Ev
pub fn stub_eb1b80() -> ! {
    todo!("0xeb1b80 __ZN3RBX19IAdornableCollectorD1Ev")
}

#[doc(alias = "RBX::IAdornableCollector::~IAdornableCollector()")]
// 0xeb1b8c — __ZN3RBX19IAdornableCollectorD2Ev
pub fn stub_eb1b8c() -> ! {
    todo!("0xeb1b8c __ZN3RBX19IAdornableCollectorD2Ev")
}

#[doc(alias = "RBX::IAdornableCollector::onRenderableDescendantAdded(RBX::IAdornable *)")]
// 0xeb1ddc — __ZN3RBX19IAdornableCollector27onRenderableDescendantAddedEPNS_10IAdornableE
pub fn stub_eb1ddc() -> ! {
    todo!("0xeb1ddc __ZN3RBX19IAdornableCollector27onRenderableDescendantAddedEPNS_10IAdornableE")
}

#[doc(alias = "RBX::IAdornableCollector::onRenderableDescendantRemoving(RBX::IAdornable *)")]
// 0xeb1f1c — __ZN3RBX19IAdornableCollector30onRenderableDescendantRemovingEPNS_10IAdornableE
pub fn stub_eb1f1c() -> ! {
    todo!("0xeb1f1c __ZN3RBX19IAdornableCollector30onRenderableDescendantRemovingEPNS_10IAdornableE")
}

#[doc(alias = "RBX::IAdornableCollector::render2dItems(RBX::Adorn *)")]
// 0xeb2118 — __ZN3RBX19IAdornableCollector13render2dItemsEPNS_5AdornE
pub fn stub_eb2118() -> ! {
    todo!("0xeb2118 __ZN3RBX19IAdornableCollector13render2dItemsEPNS_5AdornE")
}

#[doc(alias = "RBX::IAdornableCollector::render3dAdornItems(RBX::Adorn *)")]
// 0xeb21cc — __ZN3RBX19IAdornableCollector18render3dAdornItemsEPNS_5AdornE
pub fn stub_eb21cc() -> ! {
    todo!("0xeb21cc __ZN3RBX19IAdornableCollector18render3dAdornItemsEPNS_5AdornE")
}

#[doc(alias = "RBX::IAdornableCollector::append3dSortedAdornItems(std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>> &,RBX::Camera const*)const")]
// 0xeb2280 — __ZNK3RBX19IAdornableCollector24append3dSortedAdornItemsERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE
pub fn stub_eb2280() -> ! {
    todo!("0xeb2280 __ZNK3RBX19IAdornableCollector24append3dSortedAdornItemsERSt6vectorIPNS_10IAdornableESaIS3_EEPKNS_6CameraE")
}
