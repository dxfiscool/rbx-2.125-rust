//! core bg18 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct not yet in /tmp/global_eas.txt — next 100 uncovered after 0xf39e24 -> 0xf3a454..0xf3ca84.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm")]
// 0xf3a454 — j___ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf3a454() -> ! {
    todo!("0xf3a454 j___ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm")]
// 0xf3a464 — j___ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf3a464() -> ! {
    todo!("0xf3a464 j___ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_")]
// 0xf3a474 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf3a474() -> ! {
    todo!("0xf3a474 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::DialogRoot::DialogPurpose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *>(RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_")]
// 0xf3a484 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf3a484() -> ! {
    todo!("0xf3a484 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf3a494 — j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf3a494() -> ! {
    todo!("0xf3a494 j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogPurpose,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf3a4a4 — j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf3a4a4() -> ! {
    todo!("0xf3a4a4 j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf3a4b4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf3a4b4() -> ! {
    todo!("0xf3a4b4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf3a4c4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf3a4c4() -> ! {
    todo!("0xf3a4c4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_")]
// 0xf3a4d4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf3a4d4() -> ! {
    todo!("0xf3a4d4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_")]
// 0xf3a4e4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf3a4e4() -> ! {
    todo!("0xf3a4e4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,RBX::DialogRoot::DialogPurpose const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf3a4f4 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf3a4f4() -> ! {
    todo!("0xf3a4f4 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,unsigned long,RBX::DialogRoot::DialogPurpose const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf3a504 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf3a504() -> ! {
    todo!("0xf3a504 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::resize(unsigned long,RBX::DialogRoot::DialogPurpose)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_")]
// 0xf3a514 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf3a514() -> ! {
    todo!("0xf3a514 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::push_back(RBX::DialogRoot::DialogPurpose const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_")]
// 0xf3a524 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf3a524() -> ! {
    todo!("0xf3a524 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf3a534 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf3a534() -> ! {
    todo!("0xf3a534 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf3a544 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf3a544() -> ! {
    todo!("0xf3a544 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf3a554 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf3a554() -> ! {
    todo!("0xf3a554 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf3a564 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf3a564() -> ! {
    todo!("0xf3a564 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf3a574 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf3a574() -> ! {
    todo!("0xf3a574 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf3a584 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf3a584() -> ! {
    todo!("0xf3a584 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm")]
// 0xf3a634 — j___ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf3a634() -> ! {
    todo!("0xf3a634 j___ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_")]
// 0xf3a694 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf3a694() -> ! {
    todo!("0xf3a694 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf3a6f4 — j___ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(int, int)
pub fn stub_0xf3a6f4() -> ! {
    todo!("0xf3a6f4 j___ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf3a814 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf3a814() -> ! {
    todo!("0xf3a814 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf3a824 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf3a824() -> ! {
    todo!("0xf3a824 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_")]
// 0xf3a834 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_
// type: int __fastcall(int, int, int)
pub fn stub_0xf3a834() -> ! {
    todo!("0xf3a834 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_")]
// 0xf3a844 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
pub fn stub_0xf3a844() -> ! {
    todo!("0xf3a844 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf3a954 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int)
pub fn stub_0xf3a954() -> ! {
    todo!("0xf3a954 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf3a964 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0xf3a964() -> ! {
    todo!("0xf3a964 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf3a974 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf3a974() -> ! {
    todo!("0xf3a974 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Vector_base<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm")]
// 0xf3ad94 — j___ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm
// type: int()
pub fn stub_0xf3ad94() -> ! {
    todo!("0xf3ad94 j___ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Explosion::ExplosionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *>(RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_")]
// 0xf3ada4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_
// type: int()
pub fn stub_0xf3ada4() -> ! {
    todo!("0xf3ada4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Explosion::ExplosionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
// 0xf3adb4 — j___ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int()
pub fn stub_0xf3adb4() -> ! {
    todo!("0xf3adb4 j___ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,RBX::Explosion::ExplosionType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf3adc4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int()
pub fn stub_0xf3adc4() -> ! {
    todo!("0xf3adc4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,unsigned long,RBX::Explosion::ExplosionType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0xf3add4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int()
pub fn stub_0xf3add4() -> ! {
    todo!("0xf3add4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::resize(unsigned long,RBX::Explosion::ExplosionType)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_")]
// 0xf3ade4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_
// type: int()
pub fn stub_0xf3ade4() -> ! {
    todo!("0xf3ade4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::push_back(RBX::Explosion::ExplosionType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_")]
// 0xf3adf4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_
// type: int()
pub fn stub_0xf3adf4() -> ! {
    todo!("0xf3adf4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0xf3ae14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int()
pub fn stub_0xf3ae14() -> ! {
    todo!("0xf3ae14 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0xf3ae24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf3ae24() -> ! {
    todo!("0xf3ae24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0xf3ae34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: int()
pub fn stub_0xf3ae34() -> ! {
    todo!("0xf3ae34 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0xf3ae44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int()
pub fn stub_0xf3ae44() -> ! {
    todo!("0xf3ae44 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::CustomEvent::CustomEvent(void)")]
#[doc(alias = "j___ZN3RBX11CustomEventC2Ev")]
// 0xf3c0f4 — j___ZN3RBX11CustomEventC2Ev
// type: _DWORD __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0xf3c0f4() -> ! {
    todo!("0xf3c0f4 j___ZN3RBX11CustomEventC2Ev")
}

#[doc(alias = "RBX::CustomEvent::~CustomEvent()")]
#[doc(alias = "j___ZN3RBX11CustomEventD2Ev")]
// 0xf3c104 — j___ZN3RBX11CustomEventD2Ev
// type: void __fastcall(RBX::CustomEvent *__hidden this)
pub fn stub_0xf3c104() -> ! {
    todo!("0xf3c104 j___ZN3RBX11CustomEventD2Ev")
}

#[doc(alias = "RBX::BindableEvent::BindableEvent(void)")]
#[doc(alias = "j___ZN3RBX13BindableEventC2Ev")]
// 0xf3c124 — j___ZN3RBX13BindableEventC2Ev
// type: _DWORD __fastcall(RBX::BindableEvent *__hidden this)
pub fn stub_0xf3c124() -> ! {
    todo!("0xf3c124 j___ZN3RBX13BindableEventC2Ev")
}

#[doc(alias = "RBX::BindableFunction::BindableFunction(void)")]
#[doc(alias = "j___ZN3RBX16BindableFunctionC2Ev")]
// 0xf3c264 — j___ZN3RBX16BindableFunctionC2Ev
// type: _DWORD __fastcall(RBX::BindableFunction *__hidden this)
pub fn stub_0xf3c264() -> ! {
    todo!("0xf3c264 j___ZN3RBX16BindableFunctionC2Ev")
}

#[doc(alias = "RBX::CustomEventReceiver::CustomEventReceiver(void)")]
#[doc(alias = "j___ZN3RBX19CustomEventReceiverC2Ev")]
// 0xf3c274 — j___ZN3RBX19CustomEventReceiverC2Ev
// type: _DWORD __fastcall(RBX::CustomEventReceiver *__hidden this)
pub fn stub_0xf3c274() -> ! {
    todo!("0xf3c274 j___ZN3RBX19CustomEventReceiverC2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_")]
// 0xf3c414 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_
// type: int()
pub fn stub_0xf3c414() -> ! {
    todo!("0xf3c414 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_")]
// 0xf3c424 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c424() -> ! {
    todo!("0xf3c424 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_")]
// 0xf3c434 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_
// type: int()
pub fn stub_0xf3c434() -> ! {
    todo!("0xf3c434 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_")]
// 0xf3c474 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c474() -> ! {
    todo!("0xf3c474 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_")]
// 0xf3c484 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c484() -> ! {
    todo!("0xf3c484 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendStatus>(RBX::FriendService::FriendStatus const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_")]
// 0xf3c4e4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf3c4e4() -> ! {
    todo!("0xf3c4e4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendEventType>(RBX::FriendService::FriendEventType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_")]
// 0xf3c4f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf3c4f4() -> ! {
    todo!("0xf3c4f4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SocialService::StuffType>(RBX::SocialService::StuffType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SocialService9StuffTypeEEERS3_RKT_")]
// 0xf3c514 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SocialService9StuffTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c514() -> ! {
    todo!("0xf3c514 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SocialService9StuffTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FunctionalTest::Result>(RBX::FunctionalTest::Result const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_14FunctionalTest6ResultEEERS3_RKT_")]
// 0xf3c524 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_14FunctionalTest6ResultEEERS3_RKT_
// type: int()
pub fn stub_0xf3c524() -> ! {
    todo!("0xf3c524 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_14FunctionalTest6ResultEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeywordFilterType>(RBX::KeywordFilterType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17KeywordFilterTypeEEERS3_RKT_")]
// 0xf3c594 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17KeywordFilterTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c594() -> ! {
    todo!("0xf3c594 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17KeywordFilterTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChangeHistoryService::RuntimeUndoBehavior>(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ChangeHistoryService19RuntimeUndoBehaviorEEERS3_RKT_")]
// 0xf3c5b4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ChangeHistoryService19RuntimeUndoBehaviorEEERS3_RKT_
// type: int()
pub fn stub_0xf3c5b4() -> ! {
    todo!("0xf3c5b4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ChangeHistoryService19RuntimeUndoBehaviorEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PersonalServerService::PrivilegeType>(RBX::PersonalServerService::PrivilegeType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_21PersonalServerService13PrivilegeTypeEEERS3_RKT_")]
// 0xf3c5d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_21PersonalServerService13PrivilegeTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c5d4() -> ! {
    todo!("0xf3c5d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_21PersonalServerService13PrivilegeTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Frame::Style>(RBX::Frame::Style const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_")]
// 0xf3c5e4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_
// type: int()
pub fn stub_0xf3c5e4() -> ! {
    todo!("0xf3c5e4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_")]
// 0xf3c604 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_
// type: int()
pub fn stub_0xf3c604() -> ! {
    todo!("0xf3c604 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellForce>(RBX::Voxel::WaterCellForce const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_")]
// 0xf3c614 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_
// type: int()
pub fn stub_0xf3c614() -> ! {
    todo!("0xf3c614 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_")]
// 0xf3c624 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_
// type: int()
pub fn stub_0xf3c624() -> ! {
    todo!("0xf3c624 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellDirection>(RBX::Voxel::WaterCellDirection const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_")]
// 0xf3c634 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_
// type: int()
pub fn stub_0xf3c634() -> ! {
    todo!("0xf3c634 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_")]
// 0xf3c644 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_
// type: int()
pub fn stub_0xf3c644() -> ! {
    todo!("0xf3c644 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Action::ActionType>(RBX::Action::ActionType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Action10ActionTypeEEERS3_RKT_")]
// 0xf3c654 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Action10ActionTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c654() -> ! {
    todo!("0xf3c654 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Action10ActionTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Legacy::SurfaceConstraint>(RBX::Legacy::SurfaceConstraint const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_")]
// 0xf3c664 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_
// type: int()
pub fn stub_0xf3c664() -> ! {
    todo!("0xf3c664 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::InOut>(RBX::Feature::InOut const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature5InOutEEERS3_RKT_")]
// 0xf3c674 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature5InOutEEERS3_RKT_
// type: int()
pub fn stub_0xf3c674() -> ! {
    todo!("0xf3c674 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature5InOutEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::LeftRight>(RBX::Feature::LeftRight const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_")]
// 0xf3c684 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_
// type: int()
pub fn stub_0xf3c684() -> ! {
    todo!("0xf3c684 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::TopBottom>(RBX::Feature::TopBottom const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_")]
// 0xf3c694 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_
// type: int()
pub fn stub_0xf3c694() -> ! {
    todo!("0xf3c694 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Handles::VisualStyle>(RBX::Handles::VisualStyle const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Handles11VisualStyleEEERS3_RKT_")]
// 0xf3c6a4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Handles11VisualStyleEEERS3_RKT_
// type: int()
pub fn stub_0xf3c6a4() -> ! {
    todo!("0xf3c6a4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Handles11VisualStyleEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Explosion::ExplosionType>(RBX::Explosion::ExplosionType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_")]
// 0xf3c6d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c6d4() -> ! {
    todo!("0xf3c6d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HopperBin::BinType>(RBX::HopperBin::BinType const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9HopperBin7BinTypeEEERS3_RKT_")]
// 0xf3c734 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9HopperBin7BinTypeEEERS3_RKT_
// type: int()
pub fn stub_0xf3c734() -> ! {
    todo!("0xf3c734 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9HopperBin7BinTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv")]
// 0xf3c764 — j___ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv
// type: int()
pub fn stub_0xf3c764() -> ! {
    todo!("0xf3c764 j___ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv")]
// 0xf3c774 — j___ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv
// type: int()
pub fn stub_0xf3c774() -> ! {
    todo!("0xf3c774 j___ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE9singletonEv")]
// 0xf3c784 — j___ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE9singletonEv
// type: int(void)
pub fn stub_0xf3c784() -> ! {
    todo!("0xf3c784 j___ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv")]
// 0xf3c7a4 — j___ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv
// type: int()
pub fn stub_0xf3c7a4() -> ! {
    todo!("0xf3c7a4 j___ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv")]
// 0xf3c7b4 — j___ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv
// type: int(void)
pub fn stub_0xf3c7b4() -> ! {
    todo!("0xf3c7b4 j___ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE9singletonEv")]
// 0xf3c7c4 — j___ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE9singletonEv
// type: int()
pub fn stub_0xf3c7c4() -> ! {
    todo!("0xf3c7c4 j___ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv")]
// 0xf3c7d4 — j___ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv
// type: int()
pub fn stub_0xf3c7d4() -> ! {
    todo!("0xf3c7d4 j___ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE9singletonEv")]
// 0xf3c834 — j___ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE9singletonEv
// type: int()
pub fn stub_0xf3c834() -> ! {
    todo!("0xf3c834 j___ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE9singletonEv")]
// 0xf3c844 — j___ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE9singletonEv
// type: int()
pub fn stub_0xf3c844() -> ! {
    todo!("0xf3c844 j___ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SocialService::StuffType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE9singletonEv")]
// 0xf3c864 — j___ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE9singletonEv
// type: int(void)
pub fn stub_0xf3c864() -> ! {
    todo!("0xf3c864 j___ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FunctionalTest::Result>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE9singletonEv")]
// 0xf3c874 — j___ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE9singletonEv
// type: int()
pub fn stub_0xf3c874() -> ! {
    todo!("0xf3c874 j___ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeywordFilterType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE9singletonEv")]
// 0xf3c8e4 — j___ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE9singletonEv
// type: int(void)
pub fn stub_0xf3c8e4() -> ! {
    todo!("0xf3c8e4 j___ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE9singletonEv")]
// 0xf3c904 — j___ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE9singletonEv
// type: int()
pub fn stub_0xf3c904() -> ! {
    todo!("0xf3c904 j___ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE9singletonEv")]
// 0xf3c924 — j___ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE9singletonEv
// type: int(void)
pub fn stub_0xf3c924() -> ! {
    todo!("0xf3c924 j___ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv")]
// 0xf3c934 — j___ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv
// type: int()
pub fn stub_0xf3c934() -> ! {
    todo!("0xf3c934 j___ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv")]
// 0xf3c954 — j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv
// type: int()
pub fn stub_0xf3c954() -> ! {
    todo!("0xf3c954 j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv")]
// 0xf3c964 — j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv
// type: int(void)
pub fn stub_0xf3c964() -> ! {
    todo!("0xf3c964 j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv")]
// 0xf3c974 — j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv
// type: int()
pub fn stub_0xf3c974() -> ! {
    todo!("0xf3c974 j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv")]
// 0xf3c984 — j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv
// type: int()
pub fn stub_0xf3c984() -> ! {
    todo!("0xf3c984 j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv")]
// 0xf3c994 — j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv
// type: int()
pub fn stub_0xf3c994() -> ! {
    todo!("0xf3c994 j___ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Action::ActionType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE9singletonEv")]
// 0xf3c9a4 — j___ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE9singletonEv
// type: int()
pub fn stub_0xf3c9a4() -> ! {
    todo!("0xf3c9a4 j___ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE9singletonEv")]
// 0xf3c9b4 — j___ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE9singletonEv
// type: int(void)
pub fn stub_0xf3c9b4() -> ! {
    todo!("0xf3c9b4 j___ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::InOut>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE9singletonEv")]
// 0xf3c9c4 — j___ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE9singletonEv
// type: int()
pub fn stub_0xf3c9c4() -> ! {
    todo!("0xf3c9c4 j___ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE9singletonEv")]
// 0xf3c9d4 — j___ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE9singletonEv
// type: int()
pub fn stub_0xf3c9d4() -> ! {
    todo!("0xf3c9d4 j___ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE9singletonEv")]
// 0xf3c9e4 — j___ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE9singletonEv
// type: int(void)
pub fn stub_0xf3c9e4() -> ! {
    todo!("0xf3c9e4 j___ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE9singletonEv")]
// 0xf3c9f4 — j___ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE9singletonEv
// type: int()
pub fn stub_0xf3c9f4() -> ! {
    todo!("0xf3c9f4 j___ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv")]
// 0xf3ca24 — j___ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv
// type: int()
pub fn stub_0xf3ca24() -> ! {
    todo!("0xf3ca24 j___ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::HopperBin::BinType>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE9singletonEv")]
// 0xf3ca84 — j___ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE9singletonEv
// type: int()
pub fn stub_0xf3ca84() -> ! {
    todo!("0xf3ca84 j___ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE9singletonEv")
}
