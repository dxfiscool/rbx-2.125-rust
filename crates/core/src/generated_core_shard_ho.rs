//! core shard HO — 100 core stubs EA-sorted, 0xf67514..0xf6b3e4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HN 0xf67504 (21814->21914 covered, 4 remaining).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered (0xf67514..0xf6b3e4, 21814->21914 covered, 4 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned long const,std::string>> *)")]
// 0xf67514 — j___ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0xf67514() -> ! {
    todo!("0xf67514 j___ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,std::string>,std::_Select1st<std::pair<unsigned long const,std::string>>,std::less<unsigned long>,std::allocator<std::pair<unsigned long const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned long const,std::string> const&)")]
// 0xf67524 — j___ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_0xf67524() -> ! {
    todo!("0xf67524 j___ZNSt8_Rb_treeImSt4pairIKmSsESt10_Select1stIS2_ESt4lessImESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0xf67e84 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_0xf67e84() -> ! {
    todo!("0xf67e84 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned short>>>>::construct_with_value<boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>(boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type> &&)")]
// 0xf67e94 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSstEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_
pub fn stub_0xf67e94() -> ! {
    todo!("0xf67e94 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSstEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf67ea4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_0xf67ea4() -> ! {
    todo!("0xf67ea4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf67eb4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_0xf67eb4() -> ! {
    todo!("0xf67eb4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
// 0xf67ec4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEED2Ev
pub fn stub_0xf67ec4() -> ! {
    todo!("0xf67ec4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEED2Ev")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned short>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned short>>,std::string,unsigned short,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf67ed4 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_0xf67ed4() -> ! {
    todo!("0xf67ed4 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSstEESstNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")
}

#[doc(alias = "std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *> std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>>(std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>)")]
// 0xf69184 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_
pub fn stub_0xf69184() -> ! {
    todo!("0xf69184 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>>(std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>,std::_Deque_iterator<unsigned long,unsigned long &,unsigned long *>)")]
// 0xf691d4 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_
pub fn stub_0xf691d4() -> ! {
    todo!("0xf691d4 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorImRmPmES6_EET0_T_S8_S7_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0xf69cb4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_0xf69cb4() -> ! {
    todo!("0xf69cb4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned int>>>>::construct_with_value<boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>(boost::unordered::piecewise_construct_t const&,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type> &&)")]
// 0xf69cc4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsjEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_
pub fn stub_0xf69cc4() -> ! {
    todo!("0xf69cc4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsjEEEEE20construct_with_valueIJRKNS0_21piecewise_construct_tENS_6tuples5tupleISsNSE_9null_typeESG_SG_SG_SG_SG_SG_SG_SG_EENSF_ISG_SG_SG_SG_SG_SG_SG_SG_SG_SG_EEEEEvDpOT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf69cd4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_0xf69cd4() -> ! {
    todo!("0xf69cd4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf69ce4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_0xf69ce4() -> ! {
    todo!("0xf69ce4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::~table()")]
// 0xf69cf4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEED2Ev
pub fn stub_0xf69cf4() -> ! {
    todo!("0xf69cf4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEED2Ev")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,unsigned int>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,unsigned int>>,std::string,unsigned int,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf69d04 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_0xf69d04() -> ! {
    todo!("0xf69d04 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsjEESsjNS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_Rep::_M_clone(std::allocator<unsigned short> const&,unsigned long)")]
// 0xf6a4f4 — j___ZNSbItSt11char_traitsItESaItEE4_Rep8_M_cloneERKS1_m
pub fn stub_0xf6a4f4() -> ! {
    todo!("0xf6a4f4 j___ZNSbItSt11char_traitsItESaItEE4_Rep8_M_cloneERKS1_m")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::append(unsigned short const*,unsigned long)")]
// 0xf6a504 — j___ZNSbItSt11char_traitsItESaItEE6appendEPKtm
pub fn stub_0xf6a504() -> ! {
    todo!("0xf6a504 j___ZNSbItSt11char_traitsItESaItEE6appendEPKtm")
}

#[doc(alias = "std::basic_string<unsigned short,std::char_traits<unsigned short>,std::allocator<unsigned short>>::_M_mutate(unsigned long,unsigned long,unsigned long)")]
// 0xf6a514 — j___ZNSbItSt11char_traitsItESaItEE9_M_mutateEmmm
pub fn stub_0xf6a514() -> ! {
    todo!("0xf6a514 j___ZNSbItSt11char_traitsItESaItEE9_M_mutateEmmm")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert_unique(std::pair<std::string const,unsigned short> const&)")]
// 0xf6a604 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_0xf6a604() -> ! {
    todo!("0xf6a604 j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::find(std::string const&)")]
// 0xf6a614 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_0xf6a614() -> ! {
    todo!("0xf6a614 j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,unsigned short>> *)")]
// 0xf6a624 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0xf6a624() -> ! {
    todo!("0xf6a624 j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned short>,std::_Select1st<std::pair<std::string const,unsigned short>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned short>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,unsigned short> const&)")]
// 0xf6a634 — j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_0xf6a634() -> ! {
    todo!("0xf6a634 j___ZNSt8_Rb_treeISsSt4pairIKSstESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>> *,boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>> *)")]
// 0xf6a9d4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
pub fn stub_0xf6a9d4() -> ! {
    todo!("0xf6a9d4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<unsigned int const,unsigned int>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<std::pair<unsigned int const,unsigned int>>(unsigned int const&,std::pair<unsigned int const,unsigned int> &&)")]
// 0xf6a9e4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS6_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERS5_DpOT_
pub fn stub_0xf6a9e4() -> ! {
    todo!("0xf6a9e4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE12emplace_implIJS6_EEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERS5_DpOT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
// 0xf6a9f4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
pub fn stub_0xf6a9f4() -> ! {
    todo!("0xf6a9f4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<unsigned int const,unsigned int>>,unsigned int,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
// 0xf6aa04 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
pub fn stub_0xf6aa04() -> ! {
    todo!("0xf6aa04 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKjjEEjjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm")
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,RBX::FileMeshFace const&)")]
// 0xf6ac84 — j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0xf6ac84() -> ! {
    todo!("0xf6ac84 j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FileMeshFace*,std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>>,unsigned long,RBX::FileMeshFace const&)")]
// 0xf6ac94 — j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0xf6ac94() -> ! {
    todo!("0xf6ac94 j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::FileMeshFace,std::allocator<RBX::FileMeshFace>>::reserve(unsigned long)")]
// 0xf6aca4 — j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE7reserveEm
pub fn stub_0xf6aca4() -> ! {
    todo!("0xf6aca4 j___ZNSt6vectorIN3RBX12FileMeshFaceESaIS1_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FileMeshVertexNormalTexture3d*,std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>>,RBX::FileMeshVertexNormalTexture3d const&)")]
// 0xf6acb4 — j___ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_0xf6acb4() -> ! {
    todo!("0xf6acb4 j___ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FileMeshVertexNormalTexture3d*,std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>>,unsigned long,RBX::FileMeshVertexNormalTexture3d const&)")]
// 0xf6acc4 — j___ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_0xf6acc4() -> ! {
    todo!("0xf6acc4 j___ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::FileMeshVertexNormalTexture3d,std::allocator<RBX::FileMeshVertexNormalTexture3d>>::reserve(unsigned long)")]
// 0xf6acd4 — j___ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE7reserveEm
pub fn stub_0xf6acd4() -> ! {
    todo!("0xf6acd4 j___ZNSt6vectorIN3RBX29FileMeshVertexNormalTexture3dESaIS1_EE7reserveEm")
}

#[doc(alias = "void std::vector<char,std::allocator<char>>::_M_range_insert<char *>(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,char *,char *,std::forward_iterator_tag)")]
// 0xf6ace4 — j___ZNSt6vectorIcSaIcEE15_M_range_insertIPcEEvN9__gnu_cxx17__normal_iteratorIS3_S1_EET_S7_St20forward_iterator_tag
pub fn stub_0xf6ace4() -> ! {
    todo!("0xf6ace4 j___ZNSt6vectorIcSaIcEE15_M_range_insertIPcEEvN9__gnu_cxx17__normal_iteratorIS3_S1_EET_S7_St20forward_iterator_tag")
}

#[doc(alias = "RBX::WindowAverage<double,double>::getStats(unsigned long)const")]
// 0xf6acf4 — j___ZNK3RBX13WindowAverageIddE8getStatsEm
pub fn stub_0xf6acf4() -> ! {
    todo!("0xf6acf4 j___ZNK3RBX13WindowAverageIddE8getStatsEm")
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc2d>::fastRemove(RBX::IAdornable*)")]
// 0xf6ad54 — j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc2dEvEEE10fastRemoveEPS1_
pub fn stub_0xf6ad54() -> ! {
    todo!("0xf6ad54 j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc2dEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3d>::fastRemove(RBX::IAdornable*)")]
// 0xf6ad64 — j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc3dEvEEE10fastRemoveEPS1_
pub fn stub_0xf6ad64() -> ! {
    todo!("0xf6ad64 j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_11indexFunc3dEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "RBX::IndexArray<RBX::IAdornable,&RBX::IAdornable::indexFunc3dSorted>::fastRemove(RBX::IAdornable*)")]
// 0xf6ad74 — j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_17indexFunc3dSortedEvEEE10fastRemoveEPS1_
pub fn stub_0xf6ad74() -> ! {
    todo!("0xf6ad74 j___ZN3RBX10IndexArrayINS_10IAdornableEXadL_ZNS1_17indexFunc3dSortedEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable * const&)")]
// 0xf6ad84 — j___ZNSt6vectorIPN3RBX10IAdornableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf6ad84() -> ! {
    todo!("0xf6ad84 j___ZNSt6vectorIPN3RBX10IAdornableESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::string::find_last_of(char const*,unsigned long,unsigned long)const")]
// 0xf6b024 — __ZNKSs12find_last_ofEPKcmm
pub fn stub_0xf6b024() -> ! {
    todo!("0xf6b024 __ZNKSs12find_last_ofEPKcmm")
}

#[doc(alias = "std::string::find_first_of(char const*,unsigned long,unsigned long)const")]
// 0xf6b034 — __ZNKSs13find_first_ofEPKcmm
pub fn stub_0xf6b034() -> ! {
    todo!("0xf6b034 __ZNKSs13find_first_ofEPKcmm")
}

#[doc(alias = "std::string::find_last_not_of(char const*,unsigned long,unsigned long)const")]
// 0xf6b044 — __ZNKSs16find_last_not_ofEPKcmm
pub fn stub_0xf6b044() -> ! {
    todo!("0xf6b044 __ZNKSs16find_last_not_ofEPKcmm")
}

#[doc(alias = "std::string::find_last_not_of(char,unsigned long)const")]
// 0xf6b054 — __ZNKSs16find_last_not_ofEcm
pub fn stub_0xf6b054() -> ! {
    todo!("0xf6b054 __ZNKSs16find_last_not_ofEcm")
}

#[doc(alias = "std::string::find_first_not_of(char const*,unsigned long,unsigned long)const")]
// 0xf6b064 — __ZNKSs17find_first_not_ofEPKcmm
pub fn stub_0xf6b064() -> ! {
    todo!("0xf6b064 __ZNKSs17find_first_not_ofEPKcmm")
}

#[doc(alias = "std::string::find(char const*,unsigned long,unsigned long)const")]
// 0xf6b074 — __ZNKSs4findEPKcmm
pub fn stub_0xf6b074() -> ! {
    todo!("0xf6b074 __ZNKSs4findEPKcmm")
}

#[doc(alias = "std::string::find(char,unsigned long)const")]
// 0xf6b084 — __ZNKSs4findEcm
pub fn stub_0xf6b084() -> ! {
    todo!("0xf6b084 __ZNKSs4findEcm")
}

#[doc(alias = "std::string::rfind(char const*,unsigned long,unsigned long)const")]
// 0xf6b094 — __ZNKSs5rfindEPKcmm
pub fn stub_0xf6b094() -> ! {
    todo!("0xf6b094 __ZNKSs5rfindEPKcmm")
}

#[doc(alias = "std::string::rfind(char,unsigned long)const")]
// 0xf6b0a4 — __ZNKSs5rfindEcm
pub fn stub_0xf6b0a4() -> ! {
    todo!("0xf6b0a4 __ZNKSs5rfindEcm")
}

#[doc(alias = "std::string::substr(unsigned long,unsigned long)const")]
// 0xf6b0b4 — __ZNKSs6substrEmm
pub fn stub_0xf6b0b4() -> ! {
    todo!("0xf6b0b4 __ZNKSs6substrEmm")
}

#[doc(alias = "std::string::compare(char const*)const")]
// 0xf6b0c4 — __ZNKSs7compareEPKc
pub fn stub_0xf6b0c4() -> ! {
    todo!("0xf6b0c4 __ZNKSs7compareEPKc")
}

#[doc(alias = "std::string::compare(std::string const&)const")]
// 0xf6b0d4 — __ZNKSs7compareERKSs
pub fn stub_0xf6b0d4() -> ! {
    todo!("0xf6b0d4 __ZNKSs7compareERKSs")
}

#[doc(alias = "std::string::compare(unsigned long,unsigned long,char const*)const")]
// 0xf6b0e4 — __ZNKSs7compareEmmPKc
pub fn stub_0xf6b0e4() -> ! {
    todo!("0xf6b0e4 __ZNKSs7compareEmmPKc")
}

#[doc(alias = "std::string::compare(unsigned long,unsigned long,std::string const&)const")]
// 0xf6b0f4 — __ZNKSs7compareEmmRKSs
pub fn stub_0xf6b0f4() -> ! {
    todo!("0xf6b0f4 __ZNKSs7compareEmmRKSs")
}

#[doc(alias = "std::__basic_file<char>::is_open(void)const")]
// 0xf6b104 — __ZNKSt12__basic_fileIcE7is_openEv
pub fn stub_0xf6b104() -> ! {
    todo!("0xf6b104 __ZNKSt12__basic_fileIcE7is_openEv")
}

#[doc(alias = "std::runtime_error::what(void)const")]
// 0xf6b114 — __ZNKSt13runtime_error4whatEv
pub fn stub_0xf6b114() -> ! {
    todo!("0xf6b114 __ZNKSt13runtime_error4whatEv")
}

#[doc(alias = "std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::str(void)const")]
// 0xf6b124 — __ZNKSt15basic_stringbufIcSt11char_traitsIcESaIcEE3strEv
pub fn stub_0xf6b124() -> ! {
    todo!("0xf6b124 __ZNKSt15basic_stringbufIcSt11char_traitsIcESaIcEE3strEv")
}

#[doc(alias = "std::locale::id::_M_id(void)const")]
// 0xf6b134 — __ZNKSt6locale2id5_M_idEv
pub fn stub_0xf6b134() -> ! {
    todo!("0xf6b134 __ZNKSt6locale2id5_M_idEv")
}

#[doc(alias = "std::locale::operator==(std::locale const&)const")]
// 0xf6b144 — __ZNKSt6localeeqERKS_
pub fn stub_0xf6b144() -> ! {
    todo!("0xf6b144 __ZNKSt6localeeqERKS_")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::fill(void)const")]
// 0xf6b154 — __ZNKSt9basic_iosIcSt11char_traitsIcEE4fillEv
pub fn stub_0xf6b154() -> ! {
    todo!("0xf6b154 __ZNKSt9basic_iosIcSt11char_traitsIcEE4fillEv")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::widen(char)const")]
// 0xf6b164 — __ZNKSt9basic_iosIcSt11char_traitsIcEE5widenEc
pub fn stub_0xf6b164() -> ! {
    todo!("0xf6b164 __ZNKSt9basic_iosIcSt11char_traitsIcEE5widenEc")
}

#[doc(alias = "std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>::_Rep::_M_destroy(std::allocator<wchar_t> const&)")]
// 0xf6b174 — __ZNSbIwSt11char_traitsIwESaIwEE4_Rep10_M_destroyERKS1_
pub fn stub_0xf6b174() -> ! {
    todo!("0xf6b174 __ZNSbIwSt11char_traitsIwESaIwEE4_Rep10_M_destroyERKS1_")
}

#[doc(alias = "std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>::reserve(unsigned long)")]
// 0xf6b184 — __ZNSbIwSt11char_traitsIwESaIwEE7reserveEm
pub fn stub_0xf6b184() -> ! {
    todo!("0xf6b184 __ZNSbIwSt11char_traitsIwESaIwEE7reserveEm")
}

#[doc(alias = "std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>::~basic_string()")]
// 0xf6b194 — __ZNSbIwSt11char_traitsIwESaIwEED2Ev
pub fn stub_0xf6b194() -> ! {
    todo!("0xf6b194 __ZNSbIwSt11char_traitsIwESaIwEED2Ev")
}

#[doc(alias = "std::istream & std::istream::_M_extract<bool>(bool &)")]
// 0xf6b1a4 — __ZNSi10_M_extractIbEERSiRT_
pub fn stub_0xf6b1a4() -> ! {
    todo!("0xf6b1a4 __ZNSi10_M_extractIbEERSiRT_")
}

#[doc(alias = "std::istream & std::istream::_M_extract<float>(float &)")]
// 0xf6b1b4 — __ZNSi10_M_extractIfEERSiRT_
pub fn stub_0xf6b1b4() -> ! {
    todo!("0xf6b1b4 __ZNSi10_M_extractIfEERSiRT_")
}

#[doc(alias = "std::istream & std::istream::_M_extract<unsigned int>(unsigned int &)")]
// 0xf6b1c4 — __ZNSi10_M_extractIjEERSiRT_
pub fn stub_0xf6b1c4() -> ! {
    todo!("0xf6b1c4 __ZNSi10_M_extractIjEERSiRT_")
}

#[doc(alias = "std::istream & std::istream::_M_extract<long>(long &)")]
// 0xf6b1d4 — __ZNSi10_M_extractIlEERSiRT_
pub fn stub_0xf6b1d4() -> ! {
    todo!("0xf6b1d4 __ZNSi10_M_extractIlEERSiRT_")
}

#[doc(alias = "std::istream & std::istream::_M_extract<unsigned long>(unsigned long &)")]
// 0xf6b1e4 — __ZNSi10_M_extractImEERSiRT_
pub fn stub_0xf6b1e4() -> ! {
    todo!("0xf6b1e4 __ZNSi10_M_extractImEERSiRT_")
}

#[doc(alias = "std::istream::get(char &)")]
// 0xf6b1f4 — __ZNSi3getERc
pub fn stub_0xf6b1f4() -> ! {
    todo!("0xf6b1f4 __ZNSi3getERc")
}

#[doc(alias = "std::istream::get(void)")]
// 0xf6b204 — __ZNSi3getEv
pub fn stub_0xf6b204() -> ! {
    todo!("0xf6b204 __ZNSi3getEv")
}

#[doc(alias = "std::istream::peek(void)")]
// 0xf6b214 — __ZNSi4peekEv
pub fn stub_0xf6b214() -> ! {
    todo!("0xf6b214 __ZNSi4peekEv")
}

#[doc(alias = "std::istream::read(char *,int)")]
// 0xf6b224 — __ZNSi4readEPci
pub fn stub_0xf6b224() -> ! {
    todo!("0xf6b224 __ZNSi4readEPci")
}

#[doc(alias = "std::istream::seekg(std::fpos<__mbstate_t>)")]
// 0xf6b234 — __ZNSi5seekgESt4fposI11__mbstate_tE
pub fn stub_0xf6b234() -> ! {
    todo!("0xf6b234 __ZNSi5seekgESt4fposI11__mbstate_tE")
}

#[doc(alias = "std::istream::seekg(long long,std::_Ios_Seekdir)")]
// 0xf6b244 — __ZNSi5seekgExSt12_Ios_Seekdir
pub fn stub_0xf6b244() -> ! {
    todo!("0xf6b244 __ZNSi5seekgExSt12_Ios_Seekdir")
}

#[doc(alias = "std::istream::tellg(void)")]
// 0xf6b254 — __ZNSi5tellgEv
pub fn stub_0xf6b254() -> ! {
    todo!("0xf6b254 __ZNSi5tellgEv")
}

#[doc(alias = "std::istream::getline(char *,int,char)")]
// 0xf6b264 — __ZNSi7getlineEPcic
pub fn stub_0xf6b264() -> ! {
    todo!("0xf6b264 __ZNSi7getlineEPcic")
}

#[doc(alias = "std::istream::operator>>(int &)")]
// 0xf6b274 — __ZNSirsERi
pub fn stub_0xf6b274() -> ! {
    todo!("0xf6b274 __ZNSirsERi")
}

#[doc(alias = "std::ostream::put(char)")]
// 0xf6b284 — __ZNSo3putEc
pub fn stub_0xf6b284() -> ! {
    todo!("0xf6b284 __ZNSo3putEc")
}

#[doc(alias = "std::ostream::flush(void)")]
// 0xf6b294 — __ZNSo5flushEv
pub fn stub_0xf6b294() -> ! {
    todo!("0xf6b294 __ZNSo5flushEv")
}

#[doc(alias = "std::ostream::tellp(void)")]
// 0xf6b2a4 — __ZNSo5tellpEv
pub fn stub_0xf6b2a4() -> ! {
    todo!("0xf6b2a4 __ZNSo5tellpEv")
}

#[doc(alias = "std::ostream::write(char const*,int)")]
// 0xf6b2b4 — __ZNSo5writeEPKci
pub fn stub_0xf6b2b4() -> ! {
    todo!("0xf6b2b4 __ZNSo5writeEPKci")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<void const*>(void const*)")]
// 0xf6b2c4 — __ZNSo9_M_insertIPKvEERSoT_
pub fn stub_0xf6b2c4() -> ! {
    todo!("0xf6b2c4 __ZNSo9_M_insertIPKvEERSoT_")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<bool>(bool)")]
// 0xf6b2d4 — __ZNSo9_M_insertIbEERSoT_
pub fn stub_0xf6b2d4() -> ! {
    todo!("0xf6b2d4 __ZNSo9_M_insertIbEERSoT_")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<double>(double)")]
// 0xf6b2e4 — __ZNSo9_M_insertIdEERSoT_
pub fn stub_0xf6b2e4() -> ! {
    todo!("0xf6b2e4 __ZNSo9_M_insertIdEERSoT_")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<long>(long)")]
// 0xf6b2f4 — __ZNSo9_M_insertIlEERSoT_
pub fn stub_0xf6b2f4() -> ! {
    todo!("0xf6b2f4 __ZNSo9_M_insertIlEERSoT_")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<unsigned long>(unsigned long)")]
// 0xf6b304 — __ZNSo9_M_insertImEERSoT_
pub fn stub_0xf6b304() -> ! {
    todo!("0xf6b304 __ZNSo9_M_insertImEERSoT_")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<long long>(long long)")]
// 0xf6b314 — __ZNSo9_M_insertIxEERSoT_
pub fn stub_0xf6b314() -> ! {
    todo!("0xf6b314 __ZNSo9_M_insertIxEERSoT_")
}

#[doc(alias = "std::ostream & std::ostream::_M_insert<unsigned long long>(unsigned long long)")]
// 0xf6b324 — __ZNSo9_M_insertIyEERSoT_
pub fn stub_0xf6b324() -> ! {
    todo!("0xf6b324 __ZNSo9_M_insertIyEERSoT_")
}

#[doc(alias = "std::ostream::operator<<(std::basic_streambuf<char,std::char_traits<char>> *)")]
// 0xf6b334 — __ZNSolsEPSt15basic_streambufIcSt11char_traitsIcEE
pub fn stub_0xf6b334() -> ! {
    todo!("0xf6b334 __ZNSolsEPSt15basic_streambufIcSt11char_traitsIcEE")
}

#[doc(alias = "std::ostream::operator<<(int)")]
// 0xf6b344 — __ZNSolsEi
pub fn stub_0xf6b344() -> ! {
    todo!("0xf6b344 __ZNSolsEi")
}

#[doc(alias = "std::string::_M_leak_hard(void)")]
// 0xf6b354 — __ZNSs12_M_leak_hardEv
pub fn stub_0xf6b354() -> ! {
    todo!("0xf6b354 __ZNSs12_M_leak_hardEv")
}

#[doc(alias = "std::string::_M_replace_aux(unsigned long,unsigned long,unsigned long,char)")]
// 0xf6b364 — __ZNSs14_M_replace_auxEmmmc
pub fn stub_0xf6b364() -> ! {
    todo!("0xf6b364 __ZNSs14_M_replace_auxEmmmc")
}

#[doc(alias = "std::string::at(unsigned long)")]
// 0xf6b374 — __ZNSs2atEm
pub fn stub_0xf6b374() -> ! {
    todo!("0xf6b374 __ZNSs2atEm")
}

#[doc(alias = "std::string::end(void)")]
// 0xf6b384 — __ZNSs3endEv
pub fn stub_0xf6b384() -> ! {
    todo!("0xf6b384 __ZNSs3endEv")
}

#[doc(alias = "std::string::_Rep::_M_destroy(std::allocator<char> const&)")]
// 0xf6b394 — __ZNSs4_Rep10_M_destroyERKSaIcE
pub fn stub_0xf6b394() -> ! {
    todo!("0xf6b394 __ZNSs4_Rep10_M_destroyERKSaIcE")
}

#[doc(alias = "std::string::_Rep::_S_create(unsigned long,unsigned long,std::allocator<char> const&)")]
// 0xf6b3a4 — __ZNSs4_Rep9_S_createEmmRKSaIcE
pub fn stub_0xf6b3a4() -> ! {
    todo!("0xf6b3a4 __ZNSs4_Rep9_S_createEmmRKSaIcE")
}

#[doc(alias = "std::string::swap(std::string &)")]
// 0xf6b3b4 — __ZNSs4swapERSs
pub fn stub_0xf6b3b4() -> ! {
    todo!("0xf6b3b4 __ZNSs4swapERSs")
}

#[doc(alias = "std::string::erase(unsigned long,unsigned long)")]
// 0xf6b3c4 — __ZNSs5eraseEmm
pub fn stub_0xf6b3c4() -> ! {
    todo!("0xf6b3c4 __ZNSs5eraseEmm")
}

#[doc(alias = "std::string::append(char const*,unsigned long)")]
// 0xf6b3d4 — __ZNSs6appendEPKcm
pub fn stub_0xf6b3d4() -> ! {
    todo!("0xf6b3d4 __ZNSs6appendEPKcm")
}

#[doc(alias = "std::string::append(std::string const&)")]
// 0xf6b3e4 — __ZNSs6appendERKSs
pub fn stub_0xf6b3e4() -> ! {
    todo!("0xf6b3e4 __ZNSs6appendERKSs")
}

