//! generated_watchdog_core_w6 - 120 core stubs (watchdog w6-core, global-deduped).
//! Source: ida/export.json filtered core namespace EA-ascending, excludes /tmp/global_eas.txt
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Vector_base<bool (*)(void),std::allocator<bool (*)(void)>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPFbvESaIS1_EE11_M_allocateEm")]
// 0xf29824 - j___ZNSt12_Vector_baseIPFbvESaIS1_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf29824() {
    // IDA 0xf29824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::_M_insert_aux(__gnu_cxx::__normal_iterator<bool (**)(void),std::vector<bool (*)(void),std::allocator<bool (*)(void)>>>,bool (* const&)(void))")]
#[doc(alias = "j___ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf29854 - j___ZNSt6vectorIPFbvESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf29854() {
    // IDA 0xf29854: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<boost::thread>::~scoped_ptr()")]
#[doc(alias = "j___ZN5boost10scoped_ptrINS_6threadEED2Ev")]
// 0xf2b8a4 - j___ZN5boost10scoped_ptrINS_6threadEED2Ev
pub fn stub_0xf2b8a4() {
    // IDA 0xf2b8a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::string>::~scoped_ptr()")]
#[doc(alias = "j___ZN5boost10scoped_ptrISsED2Ev")]
// 0xf2b8b4 - j___ZN5boost10scoped_ptrISsED2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf2b8b4() {
    // IDA 0xf2b8b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::condition_variable::~condition_variable()")]
#[doc(alias = "j___ZN5boost18condition_variableD2Ev")]
// 0xf2bb84 - j___ZN5boost18condition_variableD2Ev
// type: void __fastcall(pthread_mutex_t *this)
pub fn stub_0xf2bb84() {
    // IDA 0xf2bb84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::list3(boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_")]
// 0xf2bc94 - j___ZN5boost3_bi5list3INS_3argILi1EEENS2_ILi2EEENS0_5valueISsEEEC2ES3_S4_S6_
pub fn stub_0xf2bc94() {
    // IDA 0xf2bc94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_")]
// 0xf2bdd4 - j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_9function0IvEEEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2bdd4() {
    // IDA 0xf2bdd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread::join(void)")]
#[doc(alias = "j___ZN5boost6thread4joinEv")]
// 0xf2bf04 - j___ZN5boost6thread4joinEv
// type: _DWORD __fastcall(boost::thread *__hidden this)
pub fn stub_0xf2bf04() {
    // IDA 0xf2bf04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread::~thread()")]
#[doc(alias = "j___ZN5boost6threadD2Ev")]
// 0xf2bf24 - j___ZN5boost6threadD2Ev
// type: void __fastcall(boost::thread *__hidden this)
pub fn stub_0xf2bf24() {
    // IDA 0xf2bf24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function<std::string ()(std::string const&)>::operator=(boost::function<std::string ()(std::string const&)> const&)")]
#[doc(alias = "j___ZN5boost8functionIFSsRKSsEEaSERKS4_")]
// 0xf2bf34 - j___ZN5boost8functionIFSsRKSsEEaSERKS4_
pub fn stub_0xf2bf34() {
    // IDA 0xf2bf34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::move_assign(boost::function1<std::string,std::string const&>&)")]
#[doc(alias = "j___ZN5boost9function1ISsRKSsE11move_assignERS3_")]
// 0xf2c004 - j___ZN5boost9function1ISsRKSsE11move_assignERS3_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2c004() {
    // IDA 0xf2c004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::assign_to_own(boost::function1<std::string,std::string const&> const&)")]
#[doc(alias = "j___ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_")]
// 0xf2c014 - j___ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2c014() {
    // IDA 0xf2c014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::swap(boost::function1<std::string,std::string const&>&)")]
#[doc(alias = "j___ZN5boost9function1ISsRKSsE4swapERS3_")]
// 0xf2c024 - j___ZN5boost9function1ISsRKSsE4swapERS3_
pub fn stub_0xf2c024() {
    // IDA 0xf2c024: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::clear(void)")]
#[doc(alias = "j___ZN5boost9function1ISsRKSsE5clearEv")]
// 0xf2c034 - j___ZN5boost9function1ISsRKSsE5clearEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf2c034() {
    // IDA 0xf2c034: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::rehash_impl(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm")]
// 0xf2c254 - j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm
pub fn stub_0xf2c254() {
    // IDA 0xf2c254: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<unsigned int>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<boost::unordered::detail::emplace_args1<unsigned int>>(unsigned int const&,boost::unordered::detail::emplace_args1<unsigned int> const&)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE12emplace_implINS1_13emplace_args1IjEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEbERKjRKT_")]
// 0xf2c264 - j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE12emplace_implINS1_13emplace_args1IjEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEbERKjRKT_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf2c264() {
    // IDA 0xf2c264: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>> &,boost::unordered::detail::ptr_bucket *)")]
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE15place_in_bucketERNS1_5tableIS9_EEPNS1_10ptr_bucketE")]
// 0xf2c274 - j___ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE15place_in_bucketERNS1_5tableIS9_EEPNS1_10ptr_bucketE
pub fn stub_0xf2c274() {
    // IDA 0xf2c274: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<unsigned int>>>::construct(void)")]
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIjEEEE9constructEv")]
// 0xf2c284 - j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIjEEEE9constructEv
pub fn stub_0xf2c284() {
    // IDA 0xf2c284: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm")]
// 0xf2c2c4 - j___ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
pub fn stub_0xf2c2c4() {
    // IDA 0xf2c2c4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::function0<void>::operator()(void)const")]
#[doc(alias = "j___ZNK5boost9function0IvEclEv")]
// 0xf2c5c4 - j___ZNK5boost9function0IvEclEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf2c5c4() {
    // IDA 0xf2c5c4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function1<std::string,std::string const&>::operator()(std::string const&)const")]
#[doc(alias = "j___ZNK5boost9function1ISsRKSsEclES2_")]
// 0xf2c5d4 - j___ZNK5boost9function1ISsRKSsEclES2_
pub fn stub_0xf2c5d4() {
    // IDA 0xf2c5d4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function1<void,bool>::operator()(bool)const")]
#[doc(alias = "j___ZNK5boost9function1IvbEclEb")]
// 0xf2c614 - j___ZNK5boost9function1IvbEclEb
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2c614() {
    // IDA 0xf2c614: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<unsigned int>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::find_node_impl<unsigned int,std::equal_to<unsigned int>>(unsigned long,unsigned int const&,std::equal_to<unsigned int> const&)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14find_node_implIjS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEmRKT_RKT0_")]
// 0xf2c664 - j___ZNK5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14find_node_implIjS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEmRKT_RKT0_
pub fn stub_0xf2c664() {
    // IDA 0xf2c664: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::min_buckets_for_size(unsigned long)const")]
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE20min_buckets_for_sizeEm")]
// 0xf2c674 - j___ZNK5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE20min_buckets_for_sizeEm
pub fn stub_0xf2c674() {
    // IDA 0xf2c674: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::~_Deque_base()")]
#[doc(alias = "j___ZNSt11_Deque_baseISsSaISsEED2Ev")]
// 0xf2c6d4 - j___ZNSt11_Deque_baseISsSaISsEED2Ev
pub fn stub_0xf2c6d4() {
    // IDA 0xf2c6d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<char const*,std::allocator<char const*>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm")]
// 0xf2c704 - j___ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm
pub fn stub_0xf2c704() {
    // IDA 0xf2c704: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::pair<std::string const,std::string>::pair(std::string const&,std::string const&)")]
#[doc(alias = "j___ZNSt4pairIKSsSsEC2ERS0_S2_")]
// 0xf2c764 - j___ZNSt4pairIKSsSsEC2ERS0_S2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf2c764() {
    // IDA 0xf2c764: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::~deque()")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEED2Ev")]
// 0xf2c804 - j___ZNSt5dequeISsSaISsEED2Ev
pub fn stub_0xf2c804() {
    // IDA 0xf2c804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<char const**,std::vector<char const*,std::allocator<char const*>>>,char const* const&)")]
#[doc(alias = "j___ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf2c8c4 - j___ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf2c8c4() {
    // IDA 0xf2c8c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::push_back(char const* const&)")]
#[doc(alias = "j___ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_")]
// 0xf2c8d4 - j___ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_
pub fn stub_0xf2c8d4() {
    // IDA 0xf2c8d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::lower_bound(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_")]
// 0xf2caa4 - j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
pub fn stub_0xf2caa4() {
    // IDA 0xf2caa4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xf2cab4 - j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf2cab4() {
    // IDA 0xf2cab4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>,std::pair<std::string const,std::string> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")]
// 0xf2cac4 - j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf2cac4() {
    // IDA 0xf2cac4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::string> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xf2cad4 - j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf2cad4() {
    // IDA 0xf2cad4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::basic_string<char,std::char_traits<char>,std::allocator<char>> std::operator+<char,std::char_traits<char>,std::allocator<char>>(std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&,std::basic_string<char,std::char_traits<char>,std::allocator<char>> const&)")]
#[doc(alias = "j___ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_ERKS6_S8_")]
// 0xf2cb44 - j___ZStplIcSt11char_traitsIcESaIcEESbIT_T0_T1_ERKS6_S8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf2cb44() {
    // IDA 0xf2cb44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "j___ZNSt11_Deque_baseISsSaISsEE15_M_allocate_mapEm")]
// 0xf2ccf4 - j___ZNSt11_Deque_baseISsSaISsEE15_M_allocate_mapEm
pub fn stub_0xf2ccf4() {
    // IDA 0xf2ccf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_create_nodes(std::string **,std::string **)")]
#[doc(alias = "j___ZNSt11_Deque_baseISsSaISsEE15_M_create_nodesEPPSsS3_")]
// 0xf2cd04 - j___ZNSt11_Deque_baseISsSaISsEE15_M_create_nodesEPPSsS3_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2cd04() {
    // IDA 0xf2cd04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "j___ZNSt11_Deque_baseISsSaISsEE17_M_initialize_mapEm")]
// 0xf2cd14 - j___ZNSt11_Deque_baseISsSaISsEE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
pub fn stub_0xf2cd14() {
    // IDA 0xf2cd14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_push_back_aux(std::string const&)")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs")]
// 0xf2cd34 - j___ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs
pub fn stub_0xf2cd34() {
    // IDA 0xf2cd34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb")]
// 0xf2cd44 - j___ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb
pub fn stub_0xf2cd44() {
    // IDA 0xf2cd44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::string,std::string &,std::string *>,std::_Deque_iterator<std::string,std::string &,std::string *>)")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEE19_M_destroy_data_auxESt15_Deque_iteratorISsRSsPSsES5_")]
// 0xf2cd54 - j___ZNSt5dequeISsSaISsEE19_M_destroy_data_auxESt15_Deque_iteratorISsRSsPSsES5_
pub fn stub_0xf2cd54() {
    // IDA 0xf2cd54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEE22_M_reserve_map_at_backEm")]
// 0xf2cd64 - j___ZNSt5dequeISsSaISsEE22_M_reserve_map_at_backEm
pub fn stub_0xf2cd64() {
    // IDA 0xf2cd64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::pop_back(void)")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEE8pop_backEv")]
// 0xf2cd74 - j___ZNSt5dequeISsSaISsEE8pop_backEv
pub fn stub_0xf2cd74() {
    // IDA 0xf2cd74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::push_back(std::string const&)")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEE9push_backERKSs")]
// 0xf2cd84 - j___ZNSt5dequeISsSaISsEE9push_backERKSs
pub fn stub_0xf2cd84() {
    // IDA 0xf2cd84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::deque(std::deque<std::string,std::allocator<std::string>> const&)")]
#[doc(alias = "j___ZNSt5dequeISsSaISsEEC2ERKS1_")]
// 0xf2cd94 - j___ZNSt5dequeISsSaISsEEC2ERKS1_
pub fn stub_0xf2cd94() {
    // IDA 0xf2cd94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_iterator<std::string,std::string &,std::string *> std::__uninitialized_copy_aux<std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>>(std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>,std::__false_type)")]
#[doc(alias = "j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorISsRKSsPS1_ES0_ISsRSsPSsEET0_T_S9_S8_St12__false_type")]
// 0xf2ce04 - j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorISsRKSsPS1_ES0_ISsRSsPSsEET0_T_S9_S8_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
pub fn stub_0xf2ce04() {
    // IDA 0xf2ce04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf2d7e4 - j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf2d7e4() {
    // IDA 0xf2d7e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf2d7f4 - j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf2d7f4() {
    // IDA 0xf2d7f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE6resizeEmS2_")]
// 0xf2d804 - j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf2d804() {
    // IDA 0xf2d804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE9push_backERKS2_")]
// 0xf2d814 - j___ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d814() {
    // IDA 0xf2d814: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf2d824 - j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf2d824() {
    // IDA 0xf2d824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf2d834 - j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf2d834() {
    // IDA 0xf2d834: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf2d844 - j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf2d844() {
    // IDA 0xf2d844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11AnimationIdEEERS3_RKT_")]
// 0xf2d864 - j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11AnimationIdEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2d864() {
    // IDA 0xf2d864: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE9singletonEv")]
// 0xf2d874 - j___ZN3rbx14implementation12typed_holderIN3RBX11AnimationIdEE9singletonEv
pub fn stub_0xf2d874() {
    // IDA 0xf2d874: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX11AnimationIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf2d884 - j___ZN3rbx8any_castIN3RBX11AnimationIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf2d884() {
    // IDA 0xf2d884: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf2d894 - j___ZN3rbx8any_castIRN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf2d894() {
    // IDA 0xf2d894: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
#[doc(alias = "j___ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_")]
// 0xf2d8e4 - j___ZN3RBX9weak_fromINS_14AsyncHttpQueueEEEN5boost8weak_ptrIT_EEPS4_
pub fn stub_0xf2d8e4() {
    // IDA 0xf2d8e4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(rbx_core::WeakPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// 0xf2d8f4 - j___ZN5boost10shared_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_0xf2d8f4() {
    // IDA 0xf2d8f4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "void rbx_core::SharedPtr<std::string>::reset<std::string>(std::string *)")]
#[doc(alias = "j___ZN5boost10shared_ptrISsE5resetISsEEvPT_")]
// 0xf2d944 - j___ZN5boost10shared_ptrISsE5resetISsEEvPT_
pub fn stub_0xf2d944() {
    // IDA 0xf2d944: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::string>::shared_ptr<std::string>(std::string *)")]
#[doc(alias = "j___ZN5boost10shared_ptrISsEC2ISsEEPT_")]
// 0xf2d954 - j___ZN5boost10shared_ptrISsEC2ISsEEPT_
pub fn stub_0xf2d954() {
    // IDA 0xf2d954: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")]
// 0xf2d974 - j___ZN5boost3_bi5list3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
pub fn stub_0xf2d974() {
    // IDA 0xf2d974: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")]
// 0xf2d994 - j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
pub fn stub_0xf2d994() {
    // IDA 0xf2d994: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0xf2d9a4 - j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf2d9a4() {
    // IDA 0xf2d9a4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)")]
#[doc(alias = "j___ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_")]
// 0xf2d9d4 - j___ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EEEC2ESD_SE_
pub fn stub_0xf2d9d4() {
    // IDA 0xf2d9d4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_")]
// 0xf2d9e4 - j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf2d9e4() {
    // IDA 0xf2d9e4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_")]
// 0xf2d9f4 - j___ZN5boost3_bi8storage3INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_IS6_EENS2_ISA_EEEC2ESD_SE_SF_
pub fn stub_0xf2d9f4() {
    // IDA 0xf2d9f4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
#[doc(alias = "j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_")]
// 0xf2da04 - j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
pub fn stub_0xf2da04() {
    // IDA 0xf2da04: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
#[doc(alias = "j___ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_")]
// 0xf2da24 - j___ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES4_S8_SA_S4_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
// type: int __fastcall(int, int, int, char, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf2da24() {
    // IDA 0xf2da24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)")]
#[doc(alias = "j___ZN5boost4bindIvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS3_7RequestEENS_10shared_ptrINS2_5mutexEEES4_S7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_")]
// 0xf2da34 - j___ZN5boost4bindIvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS3_7RequestEENS_10shared_ptrINS2_5mutexEEES4_S7_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
pub fn stub_0xf2da34() {
    // IDA 0xf2da34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0xf2da74 - j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SC_ENS3_5list3INS3_5valueISE_EENSI_IS8_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xf2da74() {
    // IDA 0xf2da74: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0xf2da84 - j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf2da84() {
    // IDA 0xf2da84: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> const&)")]
#[doc(alias = "j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_")]
// 0xf2da94 - j___ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEaSERKS9_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2da94() {
    // IDA 0xf2da94: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(rbx_core::SharedPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")]
#[doc(alias = "j___ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
// 0xf2dac4 - j___ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
pub fn stub_0xf2dac4() {
    // IDA 0xf2dac4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::clear(void)")]
#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv")]
// 0xf2dad4 - j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf2dad4() {
    // IDA 0xf2dad4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")]
#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_")]
// 0xf2dae4 - j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf2dae4() {
    // IDA 0xf2dae4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")]
#[doc(alias = "j___ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_")]
// 0xf2db24 - j___ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_
// type: int()
pub fn stub_0xf2db24() {
    // IDA 0xf2db24: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_")]
// 0xf2db34 - j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE11move_assignERS8_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2db34() {
    // IDA 0xf2db34: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_")]
// 0xf2db44 - j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE4swapERS8_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf2db44() {
    // IDA 0xf2db44: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0xf2db64 - j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
pub fn stub_0xf2db64() {
    // IDA 0xf2db64: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// 0xf2db74 - j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf2db74() {
    // IDA 0xf2db74: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0xf2db84 - j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf2db84() {
    // IDA 0xf2db84: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const")]
#[doc(alias = "j___ZNK5boost9function2IbRKSsPSsEclES2_S3_")]
// 0xf2dbc4 - j___ZNK5boost9function2IbRKSsPSsEclES2_S3_
// type: int()
pub fn stub_0xf2dbc4() {
    // IDA 0xf2dbc4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
#[doc(alias = "j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv")]
// 0xf2dbd4 - j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
// type: int __fastcall(int, int, int, int, int, std::string *, int, int, int, int)
pub fn stub_0xf2dbd4() {
    // IDA 0xf2dbd4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
#[doc(alias = "j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv")]
// 0xf2dbe4 - j___ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
// type: int()
pub fn stub_0xf2dbe4() {
    // IDA 0xf2dbe4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm")]
// 0xf2dbf4 - j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
// type: int()
pub fn stub_0xf2dbf4() {
    // IDA 0xf2dbf4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_")]
// 0xf2dc04 - j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
pub fn stub_0xf2dc04() {
    // IDA 0xf2dc04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm")]
// 0xf2dc14 - j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
pub fn stub_0xf2dc14() {
    // IDA 0xf2dc14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
#[doc(alias = "j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev")]
// 0xf2dc24 - j___ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
// type: int __fastcall(int, int)
pub fn stub_0xf2dc24() {
    // IDA 0xf2dc24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm")]
// 0xf2dc34 - j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE11_M_allocateEm
pub fn stub_0xf2dc34() {
    // IDA 0xf2dc34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_")]
// 0xf2dc44 - j___ZNSt12_Vector_baseIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2EmRKS3_
// type: int()
pub fn stub_0xf2dc44() {
    // IDA 0xf2dc44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_")]
// 0xf2dc54 - j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
pub fn stub_0xf2dc54() {
    // IDA 0xf2dc54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
#[doc(alias = "j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_")]
// 0xf2dc64 - j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE14_M_create_nodeERKS2_
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
pub fn stub_0xf2dc64() {
    // IDA 0xf2dc64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
#[doc(alias = "j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E")]
// 0xf2dc74 - j___ZNSt4listIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf2dc74() {
    // IDA 0xf2dc74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
#[doc(alias = "j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_")]
// 0xf2dc84 - j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE14_M_create_nodeERKS2_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2dc84() {
    // IDA 0xf2dc84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
#[doc(alias = "j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_")]
// 0xf2dc94 - j___ZNSt4listIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE5eraseESt14_List_iteratorIS2_ES6_
// type: int __fastcall(int, std::_List_node_base *this)
pub fn stub_0xf2dc94() {
    // IDA 0xf2dc94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
#[doc(alias = "j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv")]
// 0xf2dca4 - j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
// type: int()
pub fn stub_0xf2dca4() {
    // IDA 0xf2dca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
#[doc(alias = "j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_")]
// 0xf2dcb4 - j___ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
// type: int __fastcall(int)
pub fn stub_0xf2dcb4() {
    // IDA 0xf2dcb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_")]
// 0xf2dcc4 - j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3RBX14AsyncHttpQueue15CallbackWrapperEPS5_EET0_T_SA_S9_
// type: int()
pub fn stub_0xf2dcc4() {
    // IDA 0xf2dcc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_")]
// 0xf2dcd4 - j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX14AsyncHttpQueue15CallbackWrapperES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf2dcd4() {
    // IDA 0xf2dcd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf2dce4 - j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf2dce4() {
    // IDA 0xf2dce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_")]
// 0xf2dcf4 - j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE20_M_allocate_and_copyIN9__gnu_cxx17__normal_iteratorIPKS2_S4_EEEEPS2_mT_SC_
pub fn stub_0xf2dcf4() {
    // IDA 0xf2dcf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_")]
// 0xf2dd04 - j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf2dd04() {
    // IDA 0xf2dd04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_")]
// 0xf2dd14 - j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEC2ERKS4_
// type: int()
pub fn stub_0xf2dd14() {
    // IDA 0xf2dd14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev")]
// 0xf2dd24 - j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EED2Ev
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf2dd24() {
    // IDA 0xf2dd24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_")]
// 0xf2dd34 - j___ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EEaSERKS4_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xf2dd34() {
    // IDA 0xf2dd34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")]
#[doc(alias = "j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_")]
// 0xf2de34 - j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_
// type: int()
pub fn stub_0xf2de34() {
    // IDA 0xf2de34: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(void)")]
#[doc(alias = "j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv")]
// 0xf2de44 - j___ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv
// type: int __fastcall(RBX::BrickColor::BrickMap *this)
pub fn stub_0xf2de44() {
    // IDA 0xf2de44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BrickColor::BrickMap::setRenderingSupportedPaletteSize(unsigned long)")]
#[doc(alias = "j___ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm")]
// 0xf2de54 - j___ZN3RBX10BrickColor8BrickMap32setRenderingSupportedPaletteSizeEm
// type: int __fastcall(RBX::BrickColor::BrickMap *this, unsigned int)
pub fn stub_0xf2de54() {
    // IDA 0xf2de54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")]
#[doc(alias = "j___ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs")]
// 0xf2de64 - j___ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf2de64() {
    // IDA 0xf2de64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")]
#[doc(alias = "j___ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_")]
// 0xf2de74 - j___ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_
// type: int()
pub fn stub_0xf2de74() {
    // IDA 0xf2de74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::BrickMap(void)")]
#[doc(alias = "j___ZN3RBX10BrickColor8BrickMapC2Ev")]
// 0xf2de84 - j___ZN3RBX10BrickColor8BrickMapC2Ev
// type: int __fastcall(RBX::BrickColor::BrickMap *this)
pub fn stub_0xf2de84() {
    // IDA 0xf2de84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
#[doc(alias = "j___ZN3RBX10BrickColor8BrickMapD2Ev")]
// 0xf2de94 - j___ZN3RBX10BrickColor8BrickMapD2Ev
// type: void __fastcall(RBX::BrickColor::BrickMap *__hidden this)
pub fn stub_0xf2de94() {
    // IDA 0xf2de94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm")]
// 0xf2dea4 - j___ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm
// type: int()
pub fn stub_0xf2dea4() {
    // IDA 0xf2dea4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm")]
// 0xf2deb4 - j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm
// type: int()
pub fn stub_0xf2deb4() {
    // IDA 0xf2deb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_")]
// 0xf2dec4 - j___ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_
// type: int()
pub fn stub_0xf2dec4() {
    // IDA 0xf2dec4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_")]
// 0xf2ded4 - j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_
// type: int()
pub fn stub_0xf2ded4() {
    // IDA 0xf2ded4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_")]
// 0xf2dee4 - j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_
// type: int()
pub fn stub_0xf2dee4() {
    // IDA 0xf2dee4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")]
#[doc(alias = "j___ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_")]
// 0xf2def4 - j___ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_
// type: int()
pub fn stub_0xf2def4() {
    // IDA 0xf2def4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
// 0xf2df04 - j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: int()
pub fn stub_0xf2df04() {
    // IDA 0xf2df04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
