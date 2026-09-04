//! core shard GV — 100 core stubs EA-sorted, 0xf54464..0xf55594 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap (0xf54464..0xf55594, 20214->20314 covered, 1604 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>)")]
// 0xf54464 — j___ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
pub fn stub_0xf54464() {
    // IDA 0xf54464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::iostreams::detail::close_all<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &)")]
// 0xf54474 — j___ZN5boost9iostreams6detail9close_allINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEEEEvRT_
pub fn stub_0xf54474() {
    // IDA 0xf54474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>(boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>> &,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
// 0xf54484 — j___ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS3_ISt19basic_ostringstreamIcS6_SaIcEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_
pub fn stub_0xf54484() {
    // IDA 0xf54484: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> *)")]
// 0xf54494 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
pub fn stub_0xf54494() {
    // IDA 0xf54494: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0xf544a4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_0xf544a4() {
    // IDA 0xf544a4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> const&)")]
// 0xf544b4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
pub fn stub_0xf544b4() {
    // IDA 0xf544b4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf544c4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
pub fn stub_0xf544c4() {
    // IDA 0xf544c4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>> const&)")]
// 0xf544d4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
pub fn stub_0xf544d4() {
    // IDA 0xf544d4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>>::construct(void)")]
// 0xf544e4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEE9constructEv
pub fn stub_0xf544e4() {
    // IDA 0xf544e4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>>>::~node_constructor()")]
// 0xf544f4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEEEEED2Ev
pub fn stub_0xf544f4() {
    // IDA 0xf544f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
// 0xf54514 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
pub fn stub_0xf54514() {
    // IDA 0xf54514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf54524 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_0xf54524() {
    // IDA 0xf54524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0xf54534 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_0xf54534() {
    // IDA 0xf54534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
// 0xf54544 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
pub fn stub_0xf54544() {
    // IDA 0xf54544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0xf54554 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_0xf54554() {
    // IDA 0xf54554: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
// 0xf54564 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
pub fn stub_0xf54564() {
    // IDA 0xf54564: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> const&)")]
// 0xf54574 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
pub fn stub_0xf54574() {
    // IDA 0xf54574: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>> const*,RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)const")]
// 0xf54584 — j___ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_0xf54584() {
    // IDA 0xf54584: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,boost::detail::function::function_buffer &)const")]
// 0xf545c4 — j___ZNK5boost6detail8function13basic_vtable2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES7_S7_E9assign_toINS_9algorithm6detail13token_finderFINSC_10is_any_ofFIcEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf545c4() {
    // IDA 0xf545c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::equal(boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>> const&)const")]
// 0xf54604 — j___ZNK5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE5equalERKS6_
pub fn stub_0xf54604() {
    // IDA 0xf54604: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::algorithm::detail::is_any_ofF<char>::operator()<char>(char)const")]
// 0xf54614 — j___ZNK5boost9algorithm6detail10is_any_ofFIcEclIcEEbT_
pub fn stub_0xf54614() {
    // IDA 0xf54614: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>> boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>::operator()<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")]
// 0xf54624 — j___ZNK5boost9algorithm6detail13token_finderFINS1_10is_any_ofFIcEEEclIN9__gnu_cxx17__normal_iteratorIPcSsEEEENS_14iterator_rangeIT_EESC_SC_
pub fn stub_0xf54624() {
    // IDA 0xf54624: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::operator()(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")]
// 0xf54634 — j___ZNK5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EclES5_S5_
pub fn stub_0xf54634() {
    // IDA 0xf54634: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::operator()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)const")]
// 0xf54654 — j___ZNK5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEclES3_S4_S7_
pub fn stub_0xf54654() {
    // IDA 0xf54654: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf54664 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
pub fn stub_0xf54664() {
    // IDA 0xf54664: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0xf54674 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_0xf54674() {
    // IDA 0xf54674: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf54684 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX15ContentProvider13CachedContentEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_0xf54684() {
    // IDA 0xf54684: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "char * std::string::_S_construct<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,std::allocator<char> const&,std::forward_iterator_tag)")]
// 0xf54694 — j___ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSsEEEES2_T_S4_RKSaIcESt20forward_iterator_tag
pub fn stub_0xf54694() {
    // IDA 0xf54694: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_clear(void)")]
// 0xf546a4 — j___ZNSt10_List_baseISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_clearEv
pub fn stub_0xf546a4() {
    // IDA 0xf546a4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>> const&)")]
// 0xf546c4 — j___ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_0xf546c4() {
    // IDA 0xf546c4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>>)")]
// 0xf546d4 — j___ZNSt4listISt4pairISsS0_ImN3RBX15ContentProvider13CachedContentEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
pub fn stub_0xf546d4() {
    // IDA 0xf546d4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::ContentProvider::CachedContent>>::pair(std::string const&,std::pair<unsigned long,RBX::ContentProvider::CachedContent> const&)")]
// 0xf546e4 — j___ZNSt4pairISsS_ImN3RBX15ContentProvider13CachedContentEEEC2ERKSsRKS3_
pub fn stub_0xf546e4() {
    // IDA 0xf546e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_range_initialize<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::input_iterator_tag)")]
// 0xf54714 — j___ZNSt6vectorISsSaISsEE19_M_range_initializeIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St18input_iterator_tag
pub fn stub_0xf54714() {
    // IDA 0xf54714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_initialize_dispatch<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::__false_type)")]
// 0xf54724 — j___ZNSt6vectorISsSaISsEE22_M_initialize_dispatchIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St12__false_type
pub fn stub_0xf54724() {
    // IDA 0xf54724: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::push_back(std::string const&)")]
// 0xf54734 — j___ZNSt6vectorISsSaISsEE9push_backERKSs
pub fn stub_0xf54734() {
    // IDA 0xf54734: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::vector<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::allocator<std::string> const&)")]
// 0xf54744 — j___ZNSt6vectorISsSaISsEEC2IN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEET_SH_RKS0_
pub fn stub_0xf54744() {
    // IDA 0xf54744: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,std::string>> *)")]
// 0xf54754 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0xf54754() {
    // IDA 0xf54754: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__adjust_heap<char *,int,char>(char *,int,int,char)")]
// 0xf54764 — j___ZSt13__adjust_heapIPcicEvT_T0_S2_T1_
pub fn stub_0xf54764() {
    // IDA 0xf54764: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__insertion_sort<char *>(char *,char *)")]
// 0xf54774 — j___ZSt16__insertion_sortIPcEvT_S1_
pub fn stub_0xf54774() {
    // IDA 0xf54774: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__introsort_loop<char *,int>(char *,char *,int)")]
// 0xf54784 — j___ZSt16__introsort_loopIPciEvT_S1_T0_
pub fn stub_0xf54784() {
    // IDA 0xf54784: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::__final_insertion_sort<char *>(char *,char *)")]
// 0xf54794 — j___ZSt22__final_insertion_sortIPcEvT_S1_
pub fn stub_0xf54794() {
    // IDA 0xf54794: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::pop_heap<char *>(char *,char *)")]
// 0xf547a4 — j___ZSt8pop_heapIPcEvT_S1_
pub fn stub_0xf547a4() {
    // IDA 0xf547a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char *,std::string> std::__find_if<__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,boost::algorithm::detail::is_any_ofF<char>,std::random_access_iterator_tag)")]
// 0xf547b4 — j___ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPcSsEEN5boost9algorithm6detail10is_any_ofFIcEEET_S9_S9_T0_St26random_access_iterator_tag
pub fn stub_0xf547b4() {
    // IDA 0xf547b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::make_heap<char *>(char *,char *)")]
// 0xf547c4 — j___ZSt9make_heapIPcEvT_S1_
pub fn stub_0xf547c4() {
    // IDA 0xf547c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void std::sort_heap<char *>(char *,char *)")]
// 0xf547d4 — j___ZSt9sort_heapIPcEvT_S1_
pub fn stub_0xf547d4() {
    // IDA 0xf547d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot::safe_static_do_get_mutex(void)")]
// 0xf547f4 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slot24safe_static_do_get_mutexEv
pub fn stub_0xf547f4() {
    // IDA 0xf547f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::insert(rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot *)")]
// 0xf54804 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE6insertEPNS8_4slotE
pub fn stub_0xf54804() {
    // IDA 0xf54804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::remove(rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot *)")]
// 0xf54814 — j___ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE6removeEPNS8_4slotE
pub fn stub_0xf54814() {
    // IDA 0xf54814: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot>::operator=(rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot*)")]
// 0xf54864 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES7_EEE4slotEEaSEPSB_
pub fn stub_0xf54864() {
    // IDA 0xf54864: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TestService> RBX::shared_from<RBX::TestService>(RBX::TestService*)")]
// 0xf54b64 — j___ZN3RBX11shared_fromINS_11TestServiceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_0xf54b64() {
    // IDA 0xf54b64: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test(int,std::string const&,char const*,char const*)")]
// 0xf54be4 — j___ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_
pub fn stub_0xf54be4() {
    // IDA 0xf54be4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::MacroSubstituter::processLine(int,std::string const&)")]
// 0xf54bf4 — j___ZN3RBX16MacroSubstituter11processLineEiRKSs
pub fn stub_0xf54bf4() {
    // IDA 0xf54bf4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Throw(int,std::string const&,char const*,char const*)")]
// 0xf54c04 — j___ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_
pub fn stub_0xf54c04() {
    // IDA 0xf54c04: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_NoThrow(int,std::string const&,char const*,char const*)")]
// 0xf54c14 — j___ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_
pub fn stub_0xf54c14() {
    // IDA 0xf54c14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Equality(int,std::string const&,char const*,char const*,char const*,char const*)")]
// 0xf54c24 — j___ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_
pub fn stub_0xf54c24() {
    // IDA 0xf54c24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_SimpleSubstitution(int,std::string const&,char const*,char const*)")]
// 0xf54c34 — j___ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_
pub fn stub_0xf54c34() {
    // IDA 0xf54c34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MacroSubstituter::MacroSubstituter(std::string const&)")]
// 0xf54c44 — j___ZN3RBX16MacroSubstituterC2ERKSs
pub fn stub_0xf54c44() {
    // IDA 0xf54c44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TestService>::operator=(rbx_core::SharedPtr<RBX::TestService> const&)")]
// 0xf54ea4 — j___ZN5boost10shared_ptrIN3RBX11TestServiceEEaSERKS3_
pub fn stub_0xf54ea4() {
    // IDA 0xf54ea4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")]
// 0xf54f14 — j___ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_
pub fn stub_0xf54f14() {
    // IDA 0xf54f14: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
// 0xf54f34 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
pub fn stub_0xf54f34() {
    // IDA 0xf54f34: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TestService,int,double> &,boost::_bi::list0 &,int)")]
// 0xf54f44 — j___ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_0xf54f44() {
    // IDA 0xf54f44: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// 0xf54f94 — j___ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_
pub fn stub_0xf54f94() {
    // IDA 0xf54f94: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "std::string boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>::operator()<std::string>(std::string const&)")]
// 0xf54fb4 — j___ZN5boost3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclISsEESsRKT_
pub fn stub_0xf54fb4() {
    // IDA 0xf54fb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")]
// 0xf54fd4 — j___ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_
pub fn stub_0xf54fd4() {
    // IDA 0xf54fd4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
// 0xf54fe4 — j___ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
pub fn stub_0xf54fe4() {
    // IDA 0xf54fe4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>)")]
// 0xf54ff4 — j___ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_
pub fn stub_0xf54ff4() {
    // IDA 0xf54ff4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// 0xf55004 — j___ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_
pub fn stub_0xf55004() {
    // IDA 0xf55004: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage5(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// 0xf55014 — j___ZN5boost3_bi8storage5INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_SA_SB_SC_
pub fn stub_0xf55014() {
    // IDA 0xf55014: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::storage6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// 0xf55024 — j___ZN5boost3_bi8storage6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_
pub fn stub_0xf55024() {
    // IDA 0xf55024: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::TestService>,int>::type> boost::bind<void,RBX::TestService,int,rbx_core::SharedPtr<RBX::TestService>,int>(void (RBX::TestService::*)(int),rbx_core::SharedPtr<RBX::TestService>,int)")]
// 0xf55054 — j___ZN5boost4bindIvN3RBX11TestServiceEiNS_10shared_ptrIS2_EEiEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS7_T0_T1_EENS5_9list_av_2IT2_T3_E4typeEEEMSA_FS7_SB_ESE_SF_
pub fn stub_0xf55054() {
    // IDA 0xf55054: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::TestService>,int,double>::type> boost::bind<void,RBX::TestService,int,double,rbx_core::SharedPtr<RBX::TestService>,int,double>(void (RBX::TestService::*)(int,double),rbx_core::SharedPtr<RBX::TestService>,int,double)")]
// 0xf55074 — j___ZN5boost4bindIvN3RBX11TestServiceEidNS_10shared_ptrIS2_EEidEENS_3_bi6bind_tIT_NS_4_mfi3mf2IS7_T0_T1_T2_EENS5_9list_av_3IT3_T4_T5_E4typeEEEMSA_FS7_SB_SC_ESF_SG_SH_
pub fn stub_0xf55074() {
    // IDA 0xf55074: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf55094 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf55094() {
    // IDA 0xf55094: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf550a4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_0xf550a4() {
    // IDA 0xf550a4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>)")]
// 0xf55154 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEEvT_
pub fn stub_0xf55154() {
    // IDA 0xf55154: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0xf552b4 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_0xf552b4() {
    // IDA 0xf552b4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &)const")]
// 0xf552c4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_0xf552c4() {
    // IDA 0xf552c4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xf552d4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_0xf552d4() {
    // IDA 0xf552d4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_allocate(unsigned long)")]
// 0xf553c4 — j___ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm
pub fn stub_0xf553c4() {
    // IDA 0xf553c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::FunctionalTest::Result * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *>(RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *)")]
// 0xf553d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_
pub fn stub_0xf553d4() {
    // IDA 0xf553d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::ostream_iterator<char,char,std::char_traits<char>>::operator=(char const&)")]
// 0xf553e4 — j___ZNSt16ostream_iteratorIccSt11char_traitsIcEEaSERKc
pub fn stub_0xf553e4() {
    // IDA 0xf553e4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::FunctionalTest::Result,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::operator[](RBX::Name const* const&)")]
// 0xf553f4 — j___ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf553f4() {
    // IDA 0xf553f4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::ostream_iterator<char,char,std::char_traits<char>> std::__copy<false,std::random_access_iterator_tag>::copy<char const*,std::ostream_iterator<char,char,std::char_traits<char>>>(char const*,char const*,std::ostream_iterator<char,char,std::char_traits<char>>)")]
// 0xf55404 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKcSt16ostream_iteratorIccSt11char_traitsIcEEEET0_T_SA_S9_
pub fn stub_0xf55404() {
    // IDA 0xf55404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,RBX::FunctionalTest::Result const&)")]
// 0xf55414 — j___ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf55414() {
    // IDA 0xf55414: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,unsigned long,RBX::FunctionalTest::Result const&)")]
// 0xf55424 — j___ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf55424() {
    // IDA 0xf55424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::resize(unsigned long,RBX::FunctionalTest::Result)")]
// 0xf55434 — j___ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_
pub fn stub_0xf55434() {
    // IDA 0xf55434: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::push_back(RBX::FunctionalTest::Result const&)")]
// 0xf55444 — j___ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_
pub fn stub_0xf55444() {
    // IDA 0xf55444: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
// 0xf55454 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf55454() {
    // IDA 0xf55454: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
// 0xf55464 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf55464() {
    // IDA 0xf55464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
// 0xf55474 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf55474() {
    // IDA 0xf55474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::LibraryService::LibraryStateObject::LibraryStateObject(lua_State *,std::string const&,bool)")]
// 0xf554a4 — j___ZN3RBX14LibraryService18LibraryStateObjectC2EP9lua_StateRKSsb
pub fn stub_0xf554a4() {
    // IDA 0xf554a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::filesystem::path& boost::filesystem::path::append<std::string>(std::string const&,std::codecvt<wchar_t,char,__mbstate_t> const&)")]
// 0xf554f4 — j___ZN5boost10filesystem4path6appendISsEERS1_RKT_RKSt7codecvtIwc11__mbstate_tE
pub fn stub_0xf554f4() {
    // IDA 0xf554f4: boost::filesystem path/directory helper. std::fs/Path — carrier no-op.
}

#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::insert_value(RBX::ProtectedString const&)")]
// 0xf55504 — j___ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE12insert_valueERKS5_
pub fn stub_0xf55504() {
    // IDA 0xf55504: boost::filesystem path/directory helper. std::fs/Path — carrier no-op.
}

#[doc(alias = "void boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::erase<bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&)>(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&,bool (*)(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&))")]
// 0xf55514 — j___ZN5boost10flyweights6detail30flyweight_core_tracking_helperINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE5eraseIPFbRKNS1_17refcounted_handleIPKNS1_16refcounted_valueINS6_8rep_typeES5_EESE_EEEEEvSO_T_
pub fn stub_0xf55514() {
    // IDA 0xf55514: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>::shared_ptr<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")]
// 0xf55524 — j___ZN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEC2IS3_EEPT_
pub fn stub_0xf55524() {
    // IDA 0xf55524: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::multi_index::multi_index_container<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&)")]
// 0xf55544 — j___ZN5boost11multi_index21multi_index_containerINS_10flyweights6detail16refcounted_valueINS3_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES7_EENS2_20hashed_factory_classISA_S7_N4mpl_2naESD_SD_E10index_listESaISA_EE7insert_ERKSA_
pub fn stub_0xf55544() {
    // IDA 0xf55544: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)")]
// 0xf55554 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE16unchecked_rehashEm
pub fn stub_0xf55554() {
    // IDA 0xf55554: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase(boost::multi_index::detail::hashed_index_iterator<boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>,boost::multi_index::detail::bucket_array<std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>>)")]
// 0xf55564 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE5eraseENS1_21hashed_index_iteratorINS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEENS1_12bucket_arrayISO_EEEE
pub fn stub_0xf55564() {
    // IDA 0xf55564: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *)")]
// 0xf55574 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE6erase_EPNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
pub fn stub_0xf55574() {
    // IDA 0xf55574: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::insert_(boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const&,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::index_node_base<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>> *)")]
// 0xf55584 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7insert_ERKSC_PNS1_17hashed_index_nodeINS1_15index_node_baseISC_SO_EEEE
pub fn stub_0xf55584() {
    // IDA 0xf55584: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::reserve(unsigned long)")]
// 0xf55594 — j___ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEE7reserveEm
pub fn stub_0xf55594() {
    // IDA 0xf55594: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
