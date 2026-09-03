//! core bg17 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct not yet in /tmp/global_eas.txt — next 100 uncovered after 0xf32de4 -> 0xf33624..0xf39e24.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm")]
// 0xf33624 — j___ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf33624() -> ! {
    todo!("0xf33624 j___ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_Vector_base(unsigned long,std::allocator<unsigned int> const&)")]
#[doc(alias = "j___ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_")]
// 0xf33634 — j___ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf33634() -> ! {
    todo!("0xf33634 j___ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_")
}

#[doc(alias = "std::map<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::operator[](unsigned int const&)")]
#[doc(alias = "j___ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_")]
// 0xf33684 — j___ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_
// type: int __fastcall(int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
pub fn stub_0xf33684() -> ! {
    todo!("0xf33684 j___ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_")
}

#[doc(alias = "void std::vector<unsigned int,std::allocator<unsigned int>>::_M_assign_aux<__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>>(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,std::forward_iterator_tag)")]
#[doc(alias = "j___ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag")]
// 0xf33784 — j___ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag
// type: int __fastcall(int, void *__src)
pub fn stub_0xf33784() -> ! {
    todo!("0xf33784 j___ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,unsigned int const&)")]
#[doc(alias = "j___ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj")]
// 0xf33794 — j___ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj
// type: int __fastcall(int, void *__src)
pub fn stub_0xf33794() -> ! {
    todo!("0xf33794 j___ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::push_back(unsigned int const&)")]
#[doc(alias = "j___ZNSt6vectorIjSaIjEE9push_backERKj")]
// 0xf337a4 — j___ZNSt6vectorIjSaIjEE9push_backERKj
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf337a4() -> ! {
    todo!("0xf337a4 j___ZNSt6vectorIjSaIjEE9push_backERKj")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::vector(std::vector<unsigned int,std::allocator<unsigned int>> const&)")]
#[doc(alias = "j___ZNSt6vectorIjSaIjEEC2ERKS1_")]
// 0xf337b4 — j___ZNSt6vectorIjSaIjEEC2ERKS1_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf337b4() -> ! {
    todo!("0xf337b4 j___ZNSt6vectorIjSaIjEEC2ERKS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_create_node(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_")]
// 0xf338c4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf338c4() -> ! {
    todo!("0xf338c4 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
// 0xf338d4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf338d4() -> ! {
    todo!("0xf338d4 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0xf338e4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
pub fn stub_0xf338e4() -> ! {
    todo!("0xf338e4 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf338f4 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
pub fn stub_0xf338f4() -> ! {
    todo!("0xf338f4 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::find(unsigned int const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_")]
// 0xf33904 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_
pub fn stub_0xf33904() -> ! {
    todo!("0xf33904 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> const*,std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>*)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_")]
// 0xf33914 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf33914() -> ! {
    todo!("0xf33914 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf33924 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf33924() -> ! {
    todo!("0xf33924 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// 0xf33934 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf33934() -> ! {
    todo!("0xf33934 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_Rb_tree(std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_")]
// 0xf33944 — j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_
// type: int __fastcall(int)
pub fn stub_0xf33944() -> ! {
    todo!("0xf33944 j___ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_")
}

#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
#[doc(alias = "j___ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm")]
// 0xf38314 — j___ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm
// type: int()
pub fn stub_0xf38314() -> ! {
    todo!("0xf38314 j___ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::rate(void)const")]
#[doc(alias = "j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE4rateEv")]
// 0xf38324 — j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE4rateEv
// type: __int64 __fastcall(int)
pub fn stub_0xf38324() -> ! {
    todo!("0xf38324 j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE4rateEv")
}

#[doc(alias = "RBX::Kernel::numBodies(void)const")]
#[doc(alias = "j___ZNK3RBX6Kernel9numBodiesEv")]
// 0xf38334 — j___ZNK3RBX6Kernel9numBodiesEv
// type: int __fastcall(RBX::Kernel *this)
pub fn stub_0xf38334() -> ! {
    todo!("0xf38334 j___ZNK3RBX6Kernel9numBodiesEv")
}

#[doc(alias = "std::ctype<char>::_M_widen_init(void)const")]
#[doc(alias = "j___ZNKSt5ctypeIcE13_M_widen_initEv")]
// 0xf387f4 — j___ZNKSt5ctypeIcE13_M_widen_initEv
// type: int __fastcall(_BYTE *)
pub fn stub_0xf387f4() -> ! {
    todo!("0xf387f4 j___ZNKSt5ctypeIcE13_M_widen_initEv")
}

#[doc(alias = "std::ctype<char>::widen(char)const")]
#[doc(alias = "j___ZNKSt5ctypeIcE5widenEc")]
// 0xf38804 — j___ZNKSt5ctypeIcE5widenEc
pub fn stub_0xf38804() -> ! {
    todo!("0xf38804 j___ZNKSt5ctypeIcE5widenEc")
}

#[doc(alias = "std::_Bit_iterator std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt13_Bit_iteratorS3_EET0_T_S5_S4_")]
// 0xf38944 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt13_Bit_iteratorS3_EET0_T_S5_S4_
// type: int **__fastcall(int **result, int, int, _DWORD *, int, int *, int)
pub fn stub_0xf38944() -> ! {
    todo!("0xf38944 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt13_Bit_iteratorS3_EET0_T_S5_S4_")
}

#[doc(alias = "std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt13_Bit_iteratorS3_EET0_T_S5_S4_")]
// 0xf38a54 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt13_Bit_iteratorS3_EET0_T_S5_S4_
// type: int **__fastcall(int **result, _DWORD *, int, int, int, int *, int *)
pub fn stub_0xf38a54() -> ! {
    todo!("0xf38a54 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt13_Bit_iteratorS3_EET0_T_S5_S4_")
}

#[doc(alias = "std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_const_iterator,std::_Bit_iterator>(std::_Bit_const_iterator,std::_Bit_const_iterator,std::_Bit_iterator)")]
#[doc(alias = "j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt19_Bit_const_iteratorSt13_Bit_iteratorEET0_T_S6_S5_")]
// 0xf38a64 — j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt19_Bit_const_iteratorSt13_Bit_iteratorEET0_T_S6_S5_
pub fn stub_0xf38a64() -> ! {
    todo!("0xf38a64 j___ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt19_Bit_const_iteratorSt13_Bit_iteratorEET0_T_S6_S5_")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::_M_fill_insert(std::_Bit_iterator,unsigned long,bool)")]
#[doc(alias = "j___ZNSt6vectorIbSaIbEE14_M_fill_insertESt13_Bit_iteratormb")]
// 0xf38c44 — j___ZNSt6vectorIbSaIbEE14_M_fill_insertESt13_Bit_iteratormb
// type: int __fastcall(__int64, int, unsigned int, _BOOL4)
pub fn stub_0xf38c44() -> ! {
    todo!("0xf38c44 j___ZNSt6vectorIbSaIbEE14_M_fill_insertESt13_Bit_iteratormb")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::resize(unsigned long,bool)")]
#[doc(alias = "j___ZNSt6vectorIbSaIbEE6resizeEmb")]
// 0xf38c54 — j___ZNSt6vectorIbSaIbEE6resizeEmb
// type: _DWORD *__fastcall(_DWORD *result, unsigned int, int)
pub fn stub_0xf38c54() -> ! {
    todo!("0xf38c54 j___ZNSt6vectorIbSaIbEE6resizeEmb")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf38da4 — j___ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(int, int)
pub fn stub_0xf38da4() -> ! {
    todo!("0xf38da4 j___ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::auto_ptr<XmlElement>::~auto_ptr()")]
#[doc(alias = "j___ZNSt8auto_ptrI10XmlElementED2Ev")]
// 0xf38db4 — j___ZNSt8auto_ptrI10XmlElementED2Ev
// type: XmlElement **__fastcall(XmlElement **)
pub fn stub_0xf38db4() -> ! {
    todo!("0xf38db4 j___ZNSt8auto_ptrI10XmlElementED2Ev")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,int,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
#[doc(alias = "j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiS4_PFbPKS3_SB_EEvT_T0_SF_T1_T2_")]
// 0xf38dc4 — j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiS4_PFbPKS3_SB_EEvT_T0_SF_T1_T2_
// type: int __fastcall(int, int, int, int, int (__fastcall *)(_DWORD, int))
pub fn stub_0xf38dc4() -> ! {
    todo!("0xf38dc4 j___ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiS4_PFbPKS3_SB_EEvT_T0_SF_T1_T2_")
}

#[doc(alias = "void std::__heap_select<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
#[doc(alias = "j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_SE_T0_")]
// 0xf38dd4 — j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_SE_T0_
// type: int __fastcall(char *, _DWORD *, unsigned int, int (__fastcall *)(_DWORD, _DWORD))
pub fn stub_0xf38dd4() -> ! {
    todo!("0xf38dd4 j___ZSt13__heap_selectIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_SE_T0_")
}

#[doc(alias = "void std::__insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
#[doc(alias = "j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_")]
// 0xf38de4 — j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, int (__fastcall *)(int, _DWORD))
pub fn stub_0xf38de4() -> ! {
    todo!("0xf38de4 j___ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_")
}

#[doc(alias = "void std::__introsort_loop<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,int,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
#[doc(alias = "j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiPFbPKS3_SB_EEvT_SE_T0_T1_")]
// 0xf38df4 — j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiPFbPKS3_SB_EEvT_SE_T0_T1_
// type: int __fastcall(_DWORD *, int, int, int (__fastcall *)(_DWORD, _DWORD))
pub fn stub_0xf38df4() -> ! {
    todo!("0xf38df4 j___ZSt16__introsort_loopIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEiPFbPKS3_SB_EEvT_SE_T0_T1_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>> std::__unguarded_partition<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,RBX::IAdornable *,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
#[doc(alias = "j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEES4_PFbPKS3_SB_EET_SE_SE_T0_T1_")]
// 0xf38e04 — j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEES4_PFbPKS3_SB_EET_SE_SE_T0_T1_
// type: int *__fastcall(int *, int *, int, int (__fastcall *)(int, int))
pub fn stub_0xf38e04() -> ! {
    todo!("0xf38e04 j___ZSt21__unguarded_partitionIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEES4_PFbPKS3_SB_EET_SE_SE_T0_T1_")
}

#[doc(alias = "void std::__final_insertion_sort<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
#[doc(alias = "j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_")]
// 0xf38e14 — j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_
// type: _DWORD *__fastcall(int, _DWORD *, int (__fastcall *)(_DWORD, _DWORD))
pub fn stub_0xf38e14() -> ! {
    todo!("0xf38e14 j___ZSt22__final_insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_")
}

#[doc(alias = "std::fill(std::_Bit_iterator,std::_Bit_iterator,bool const&)")]
#[doc(alias = "j___ZSt4fillSt13_Bit_iteratorS_RKb")]
// 0xf38e44 — j___ZSt4fillSt13_Bit_iteratorS_RKb
// type: unsigned int *__fastcall(_Bit_iterator, _Bit_iterator, const bool *)
pub fn stub_0xf38e44() -> ! {
    todo!("0xf38e44 j___ZSt4fillSt13_Bit_iteratorS_RKb")
}

#[doc(alias = "void std::sort_heap<__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*)>(__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,__gnu_cxx::__normal_iterator<RBX::IAdornable **,std::vector<RBX::IAdornable *,std::allocator<RBX::IAdornable *>>>,bool (*)(RBX::IAdornable const*,RBX::IAdornable const*))")]
#[doc(alias = "j___ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_")]
// 0xf38e74 — j___ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_
pub fn stub_0xf38e74() -> ! {
    todo!("0xf38e74 j___ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_")
}

#[doc(alias = "RBX::ActivityMeter<2>::updateBuckets(void)")]
#[doc(alias = "j___ZN3RBX13ActivityMeterILi2EE13updateBucketsEv")]
// 0xf38ea4 — j___ZN3RBX13ActivityMeterILi2EE13updateBucketsEv
// type: int()
pub fn stub_0xf38ea4() -> ! {
    todo!("0xf38ea4 j___ZN3RBX13ActivityMeterILi2EE13updateBucketsEv")
}

#[doc(alias = "RBX::OnScreenProfiler::Create(void)")]
#[doc(alias = "j___ZN3RBX16OnScreenProfiler6CreateEv")]
// 0xf38eb4 — j___ZN3RBX16OnScreenProfiler6CreateEv
// type: int __fastcall(RBX::OnScreenProfiler *this)
pub fn stub_0xf38eb4() -> ! {
    todo!("0xf38eb4 j___ZN3RBX16OnScreenProfiler6CreateEv")
}

#[doc(alias = "RBX::OnScreenProfiler::GetInst(void)")]
#[doc(alias = "j___ZN3RBX16OnScreenProfiler7GetInstEv")]
// 0xf38ec4 — j___ZN3RBX16OnScreenProfiler7GetInstEv
// type: int __fastcall(RBX::OnScreenProfiler *this)
pub fn stub_0xf38ec4() -> ! {
    todo!("0xf38ec4 j___ZN3RBX16OnScreenProfiler7GetInstEv")
}

#[doc(alias = "std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseImSaImEE11_M_allocateEm")]
// 0xf38f34 — j___ZNSt12_Vector_baseImSaImEE11_M_allocateEm
// type: int()
pub fn stub_0xf38f34() -> ! {
    todo!("0xf38f34 j___ZNSt12_Vector_baseImSaImEE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")]
#[doc(alias = "j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_")]
// 0xf38fd4 — j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_
// type: int __fastcall(int, int, int)
pub fn stub_0xf38fd4() -> ! {
    todo!("0xf38fd4 j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)")]
#[doc(alias = "j___ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs")]
// 0xf38fe4 — j___ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf38fe4() -> ! {
    todo!("0xf38fe4 j___ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DebugSettings::ErrorReporting>(RBX::DebugSettings::ErrorReporting const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_")]
// 0xf397a4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_
// type: int()
pub fn stub_0xf397a4() -> ! {
    todo!("0xf397a4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13DebugSettings14ErrorReportingEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::PriorityMethod>(RBX::TaskScheduler::PriorityMethod const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_")]
// 0xf397b4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_
// type: int()
pub fn stub_0xf397b4() -> ! {
    todo!("0xf397b4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::ThreadPoolConfig>(RBX::TaskScheduler::ThreadPoolConfig const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_")]
// 0xf397c4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_
// type: int()
pub fn stub_0xf397c4() -> ! {
    todo!("0xf397c4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TaskScheduler::Job::SleepAdjustMethod>(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_")]
// 0xf397d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_
// type: int()
pub fn stub_0xf397d4() -> ! {
    todo!("0xf397d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Time::SampleMethod>(RBX::Time::SampleMethod const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4Time12SampleMethodEEERS3_RKT_")]
// 0xf397f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4Time12SampleMethodEEERS3_RKT_
// type: int()
pub fn stub_0xf397f4() -> ! {
    todo!("0xf397f4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_4Time12SampleMethodEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::EThrottle::EThrottleType>(RBX::EThrottle::EThrottleType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9EThrottle13EThrottleTypeEEERS3_RKT_")]
// 0xf39804 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9EThrottle13EThrottleTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf39804() -> ! {
    todo!("0xf39804 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9EThrottle13EThrottleTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<float>(float const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_")]
// 0xf39814 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_
// type: int __fastcall(int, int)
pub fn stub_0xf39814() -> ! {
    todo!("0xf39814 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIfEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DebugSettings::ErrorReporting>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv")]
// 0xf39824 — j___ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv
// type: int()
pub fn stub_0xf39824() -> ! {
    todo!("0xf39824 j___ZN3rbx14implementation12typed_holderIN3RBX13DebugSettings14ErrorReportingEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv")]
// 0xf39834 — j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv
// type: int()
pub fn stub_0xf39834() -> ! {
    todo!("0xf39834 j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv")]
// 0xf39844 — j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv
// type: int()
pub fn stub_0xf39844() -> ! {
    todo!("0xf39844 j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv")]
// 0xf39854 — j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv
// type: int()
pub fn stub_0xf39854() -> ! {
    todo!("0xf39854 j___ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Time::SampleMethod>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE9singletonEv")]
// 0xf39874 — j___ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE9singletonEv
// type: int()
pub fn stub_0xf39874() -> ! {
    todo!("0xf39874 j___ZN3rbx14implementation12typed_holderIN3RBX4Time12SampleMethodEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::EThrottle::EThrottleType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE9singletonEv")]
// 0xf39884 — j___ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE9singletonEv
// type: int()
pub fn stub_0xf39884() -> ! {
    todo!("0xf39884 j___ZN3rbx14implementation12typed_holderIN3RBX9EThrottle13EThrottleTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<float>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIfE9singletonEv")]
// 0xf39894 — j___ZN3rbx14implementation12typed_holderIfE9singletonEv
// type: int(void)
pub fn stub_0xf39894() -> ! {
    todo!("0xf39894 j___ZN3rbx14implementation12typed_holderIfE9singletonEv")
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting const& rbx::any_cast<RBX::DebugSettings::ErrorReporting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf398a4 — j___ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf398a4() -> ! {
    todo!("0xf398a4 j___ZN3rbx8any_castIRKN3RBX13DebugSettings14ErrorReportingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::TaskScheduler::PriorityMethod const& rbx::any_cast<RBX::TaskScheduler::PriorityMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf398b4 — j___ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf398b4() -> ! {
    todo!("0xf398b4 j___ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig const& rbx::any_cast<RBX::TaskScheduler::ThreadPoolConfig const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf398c4 — j___ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf398c4() -> ! {
    todo!("0xf398c4 j___ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod const& rbx::any_cast<RBX::TaskScheduler::Job::SleepAdjustMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf398d4 — j___ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf398d4() -> ! {
    todo!("0xf398d4 j___ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Time::SampleMethod const& rbx::any_cast<RBX::Time::SampleMethod const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX4Time12SampleMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf398e4 — j___ZN3rbx8any_castIRKN3RBX4Time12SampleMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf398e4() -> ! {
    todo!("0xf398e4 j___ZN3rbx8any_castIRKN3RBX4Time12SampleMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::EThrottle::EThrottleType const& rbx::any_cast<RBX::EThrottle::EThrottleType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX9EThrottle13EThrottleTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf398f4 — j___ZN3rbx8any_castIRKN3RBX9EThrottle13EThrottleTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int()
pub fn stub_0xf398f4() -> ! {
    todo!("0xf398f4 j___ZN3rbx8any_castIRKN3RBX9EThrottle13EThrottleTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "float const& rbx::any_cast<float const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf39904 — j___ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(int)
pub fn stub_0xf39904() -> ! {
    todo!("0xf39904 j___ZN3rbx8any_castIRKfN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const")]
#[doc(alias = "j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv")]
// 0xf39bd4 — j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv
// type: int()
pub fn stub_0xf39bd4() -> ! {
    todo!("0xf39bd4 j___ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv")
}

#[doc(alias = "std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm")]
// 0xf39bf4 — j___ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm
pub fn stub_0xf39bf4() -> ! {
    todo!("0xf39bf4 j___ZNSt12_Vector_baseIN3RBX13DebugSettings14ErrorReportingESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm")]
// 0xf39c04 — j___ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm
pub fn stub_0xf39c04() -> ! {
    todo!("0xf39c04 j___ZNSt12_Vector_baseIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm")]
// 0xf39c14 — j___ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf39c14() -> ! {
    todo!("0xf39c14 j___ZNSt12_Vector_baseIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm")]
// 0xf39c24 — j___ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm
// type: int()
pub fn stub_0xf39c24() -> ! {
    todo!("0xf39c24 j___ZNSt12_Vector_baseIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm")]
// 0xf39c34 — j___ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm
pub fn stub_0xf39c34() -> ! {
    todo!("0xf39c34 j___ZNSt12_Vector_baseIN3RBX4Time12SampleMethodESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm")]
// 0xf39c44 — j___ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm
pub fn stub_0xf39c44() -> ! {
    todo!("0xf39c44 j___ZNSt12_Vector_baseIN3RBX9EThrottle13EThrottleTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_")]
// 0xf39c54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf39c54() -> ! {
    todo!("0xf39c54 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13DebugSettings14ErrorReportingES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_")]
// 0xf39c64 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf39c64() -> ! {
    todo!("0xf39c64 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler14PriorityMethodES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_")]
// 0xf39c74 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf39c74() -> ! {
    todo!("0xf39c74 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler16ThreadPoolConfigES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_")]
// 0xf39c84 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_
// type: int()
pub fn stub_0xf39c84() -> ! {
    todo!("0xf39c84 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13TaskScheduler3Job17SleepAdjustMethodES7_EET0_T_S9_S8_")
}

#[doc(alias = "RBX::Time::SampleMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Time::SampleMethod *,RBX::Time::SampleMethod *>(RBX::Time::SampleMethod *,RBX::Time::SampleMethod *,RBX::Time::SampleMethod *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_")]
// 0xf39c94 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf39c94() -> ! {
    todo!("0xf39c94 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4Time12SampleMethodES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_")]
// 0xf39ca4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf39ca4() -> ! {
    todo!("0xf39ca4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9EThrottle13EThrottleTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf39cb4 — j___ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf39cb4() -> ! {
    todo!("0xf39cb4 j___ZNSt3mapIPKN3RBX4NameENS0_13DebugSettings14ErrorReportingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf39cc4 — j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf39cc4() -> ! {
    todo!("0xf39cc4 j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler14PriorityMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf39cd4 — j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf39cd4() -> ! {
    todo!("0xf39cd4 j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler16ThreadPoolConfigESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")]
// 0xf39ce4 — j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_
pub fn stub_0xf39ce4() -> ! {
    todo!("0xf39ce4 j___ZNSt3mapIPKN3RBX4NameENS0_13TaskScheduler3Job17SleepAdjustMethodESt4lessIS3_ESaISt4pairIKS3_S6_EEEixERSA_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Time::SampleMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf39cf4 — j___ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf39cf4() -> ! {
    todo!("0xf39cf4 j___ZNSt3mapIPKN3RBX4NameENS0_4Time12SampleMethodESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf39d04 — j___ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf39d04() -> ! {
    todo!("0xf39d04 j___ZNSt3mapIPKN3RBX4NameENS0_9EThrottle13EThrottleTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf39d14 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf39d14() -> ! {
    todo!("0xf39d14 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf39d24 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf39d24() -> ! {
    todo!("0xf39d24 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_")]
// 0xf39d34 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf39d34() -> ! {
    todo!("0xf39d34 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_")]
// 0xf39d44 — j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf39d44() -> ! {
    todo!("0xf39d44 j___ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf39d54 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf39d54() -> ! {
    todo!("0xf39d54 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf39d64 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf39d64() -> ! {
    todo!("0xf39d64 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_")]
// 0xf39d74 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf39d74() -> ! {
    todo!("0xf39d74 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_")]
// 0xf39d84 — j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf39d84() -> ! {
    todo!("0xf39d84 j___ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf39d94 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf39d94() -> ! {
    todo!("0xf39d94 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf39da4 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf39da4() -> ! {
    todo!("0xf39da4 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_")]
// 0xf39db4 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf39db4() -> ! {
    todo!("0xf39db4 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_")]
// 0xf39dc4 — j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf39dc4() -> ! {
    todo!("0xf39dc4 j___ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")]
// 0xf39dd4 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int()
pub fn stub_0xf39dd4() -> ! {
    todo!("0xf39dd4 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
// 0xf39de4 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: int()
pub fn stub_0xf39de4() -> ! {
    todo!("0xf39de4 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_")]
// 0xf39df4 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_
// type: int()
pub fn stub_0xf39df4() -> ! {
    todo!("0xf39df4 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE6resizeEmS3_")
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_")]
// 0xf39e04 — j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_
// type: int()
pub fn stub_0xf39e04() -> ! {
    todo!("0xf39e04 j___ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,RBX::Time::SampleMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf39e14 — j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf39e14() -> ! {
    todo!("0xf39e14 j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Time::SampleMethod*,std::vector<RBX::Time::SampleMethod,std::allocator<RBX::Time::SampleMethod>>>,unsigned long,RBX::Time::SampleMethod const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf39e24 — j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf39e24() -> ! {
    todo!("0xf39e24 j___ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}
