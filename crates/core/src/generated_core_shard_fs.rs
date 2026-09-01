//! core shard FS — 100 core stubs EA-sorted, 0xf3a344..0xf3c6e4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after FR 0xf3a264).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf3a264.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)")]
// 0xf3a344 — j___ZN3RBX11shared_fromINS_10DialogRootEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::DialogRoot> RBX::shared_from<RBX::DialogRoot>(RBX::DialogRoot*)
pub fn stub_f3a344() -> ! {
    todo!("0xf3a344 j___ZN3RBX11shared_fromINS_10DialogRootEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)")]
// 0xf3a454 — j___ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm
pub fn stub_f3a454() -> ! {
    todo!("0xf3a454 j___ZNSt12_Vector_baseIN3RBX10DialogRoot10DialogToneESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_allocate(unsigned long)")]
// 0xf3a464 — j___ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm
pub fn stub_f3a464() -> ! {
    todo!("0xf3a464 j___ZNSt12_Vector_baseIN3RBX10DialogRoot13DialogPurposeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)")]
// 0xf3a474 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_
pub fn stub_f3a474() -> ! {
    todo!("0xf3a474 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot10DialogToneES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::DialogRoot::DialogPurpose * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *>(RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *,RBX::DialogRoot::DialogPurpose *)")]
// 0xf3a484 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_
pub fn stub_f3a484() -> ! {
    todo!("0xf3a484 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10DialogRoot13DialogPurposeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)")]
// 0xf3a494 — j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3a494() -> ! {
    todo!("0xf3a494 j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot10DialogToneESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogPurpose,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::operator[](RBX::Name const* const&)")]
// 0xf3a4a4 — j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3a4a4() -> ! {
    todo!("0xf3a4a4 j___ZNSt3mapIPKN3RBX4NameENS0_10DialogRoot13DialogPurposeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)")]
// 0xf3a4b4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3a4b4() -> ! {
    todo!("0xf3a4b4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)")]
// 0xf3a4c4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3a4c4() -> ! {
    todo!("0xf3a4c4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)")]
// 0xf3a4d4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_
pub fn stub_f3a4d4() -> ! {
    todo!("0xf3a4d4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)")]
// 0xf3a4e4 — j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_
pub fn stub_f3a4e4() -> ! {
    todo!("0xf3a4e4 j___ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,RBX::DialogRoot::DialogPurpose const&)")]
// 0xf3a4f4 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3a4f4() -> ! {
    todo!("0xf3a4f4 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogPurpose*,std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>>,unsigned long,RBX::DialogRoot::DialogPurpose const&)")]
// 0xf3a504 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3a504() -> ! {
    todo!("0xf3a504 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::resize(unsigned long,RBX::DialogRoot::DialogPurpose)")]
// 0xf3a514 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_
pub fn stub_f3a514() -> ! {
    todo!("0xf3a514 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogPurpose,std::allocator<RBX::DialogRoot::DialogPurpose>>::push_back(RBX::DialogRoot::DialogPurpose const&)")]
// 0xf3a524 — j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_
pub fn stub_f3a524() -> ! {
    todo!("0xf3a524 j___ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0xf3a534 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3a534() -> ! {
    todo!("0xf3a534 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0xf3a544 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3a544() -> ! {
    todo!("0xf3a544 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
// 0xf3a554 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3a554() -> ! {
    todo!("0xf3a554 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// 0xf3a564 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3a564() -> ! {
    todo!("0xf3a564 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// 0xf3a574 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3a574() -> ! {
    todo!("0xf3a574 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose> const&)")]
// 0xf3a584 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3a584() -> ! {
    todo!("0xf3a584 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Vector_base<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_allocate(unsigned long)")]
// 0xf3a634 — j___ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm
pub fn stub_f3a634() -> ! {
    todo!("0xf3a634 j___ZNSt12_Vector_baseIN3RBX7Handles11VisualStyleESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_allocate(unsigned long)")]
// 0xf3a644 — j___ZNSt12_Vector_baseIN3RBX9GuiObject14SizeConstraintESaIS2_EE11_M_allocateEm
pub fn stub_f3a644() -> ! {
    todo!("0xf3a644 j___ZNSt12_Vector_baseIN3RBX9GuiObject14SizeConstraintESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Handles::VisualStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *>(RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *,RBX::Handles::VisualStyle *)")]
// 0xf3a694 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_
pub fn stub_f3a694() -> ! {
    todo!("0xf3a694 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX7Handles11VisualStyleES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::GuiObject::SizeConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *>(RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *,RBX::GuiObject::SizeConstraint *)")]
// 0xf3a6a4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject14SizeConstraintES6_EET0_T_S8_S7_
pub fn stub_f3a6a4() -> ! {
    todo!("0xf3a6a4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject14SizeConstraintES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Handles::VisualStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::operator[](RBX::Name const* const&)")]
// 0xf3a6f4 — j___ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3a6f4() -> ! {
    todo!("0xf3a6f4 j___ZNSt3mapIPKN3RBX4NameENS0_7Handles11VisualStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::SizeConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::operator[](RBX::Name const* const&)")]
// 0xf3a704 — j___ZNSt3mapIPKN3RBX4NameENS0_9GuiObject14SizeConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3a704() -> ! {
    todo!("0xf3a704 j___ZNSt3mapIPKN3RBX4NameENS0_9GuiObject14SizeConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,RBX::Handles::VisualStyle const&)")]
// 0xf3a814 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3a814() -> ! {
    todo!("0xf3a814 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Handles::VisualStyle*,std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>>,unsigned long,RBX::Handles::VisualStyle const&)")]
// 0xf3a824 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3a824() -> ! {
    todo!("0xf3a824 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::resize(unsigned long,RBX::Handles::VisualStyle)")]
// 0xf3a834 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_
pub fn stub_f3a834() -> ! {
    todo!("0xf3a834 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Handles::VisualStyle,std::allocator<RBX::Handles::VisualStyle>>::push_back(RBX::Handles::VisualStyle const&)")]
// 0xf3a844 — j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_
pub fn stub_f3a844() -> ! {
    todo!("0xf3a844 j___ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,RBX::GuiObject::SizeConstraint const&)")]
// 0xf3a854 — j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3a854() -> ! {
    todo!("0xf3a854 j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::SizeConstraint*,std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>>,unsigned long,RBX::GuiObject::SizeConstraint const&)")]
// 0xf3a864 — j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3a864() -> ! {
    todo!("0xf3a864 j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::resize(unsigned long,RBX::GuiObject::SizeConstraint)")]
// 0xf3a874 — j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE6resizeEmS2_
pub fn stub_f3a874() -> ! {
    todo!("0xf3a874 j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GuiObject::SizeConstraint,std::allocator<RBX::GuiObject::SizeConstraint>>::push_back(RBX::GuiObject::SizeConstraint const&)")]
// 0xf3a884 — j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE9push_backERKS2_
pub fn stub_f3a884() -> ! {
    todo!("0xf3a884 j___ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// 0xf3a954 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3a954() -> ! {
    todo!("0xf3a954 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// 0xf3a964 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3a964() -> ! {
    todo!("0xf3a964 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle> const&)")]
// 0xf3a974 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3a974() -> ! {
    todo!("0xf3a974 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// 0xf3a984 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3a984() -> ! {
    todo!("0xf3a984 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// 0xf3a994 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3a994() -> ! {
    todo!("0xf3a994 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint> const&)")]
// 0xf3a9a4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3a9a4() -> ! {
    todo!("0xf3a9a4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Explosion> RBX::shared_from<RBX::Explosion>(RBX::Explosion*)")]
// 0xf3aa24 — j___ZN3RBX11shared_fromINS_9ExplosionEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::Explosion> RBX::shared_from<RBX::Explosion>(RBX::Explosion*)
pub fn stub_f3aa24() -> ! {
    todo!("0xf3aa24 j___ZN3RBX11shared_fromINS_9ExplosionEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "std::_Vector_base<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_allocate(unsigned long)")]
// 0xf3ad94 — j___ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm
pub fn stub_f3ad94() -> ! {
    todo!("0xf3ad94 j___ZNSt12_Vector_baseIN3RBX9Explosion13ExplosionTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Explosion::ExplosionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *>(RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *,RBX::Explosion::ExplosionType *)")]
// 0xf3ada4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_
pub fn stub_f3ada4() -> ! {
    todo!("0xf3ada4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9Explosion13ExplosionTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Explosion::ExplosionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::operator[](RBX::Name const* const&)")]
// 0xf3adb4 — j___ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3adb4() -> ! {
    todo!("0xf3adb4 j___ZNSt3mapIPKN3RBX4NameENS0_9Explosion13ExplosionTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,RBX::Explosion::ExplosionType const&)")]
// 0xf3adc4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3adc4() -> ! {
    todo!("0xf3adc4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Explosion::ExplosionType*,std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>>,unsigned long,RBX::Explosion::ExplosionType const&)")]
// 0xf3add4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3add4() -> ! {
    todo!("0xf3add4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::resize(unsigned long,RBX::Explosion::ExplosionType)")]
// 0xf3ade4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_
pub fn stub_f3ade4() -> ! {
    todo!("0xf3ade4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Explosion::ExplosionType,std::allocator<RBX::Explosion::ExplosionType>>::push_back(RBX::Explosion::ExplosionType const&)")]
// 0xf3adf4 — j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_
pub fn stub_f3adf4() -> ! {
    todo!("0xf3adf4 j___ZNSt6vectorIN3RBX9Explosion13ExplosionTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0xf3ae14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3ae14() -> ! {
    todo!("0xf3ae14 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0xf3ae24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3ae24() -> ! {
    todo!("0xf3ae24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>> *)")]
// 0xf3ae34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f3ae34() -> ! {
    todo!("0xf3ae34 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Explosion::ExplosionType> const&)")]
// 0xf3ae44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3ae44() -> ! {
    todo!("0xf3ae44 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9Explosion13ExplosionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::CustomEvent::CustomEvent(void)")]
// 0xf3c0f4 — j___ZN3RBX11CustomEventC2Ev
pub fn stub_f3c0f4() -> ! {
    todo!("0xf3c0f4 j___ZN3RBX11CustomEventC2Ev")
}

#[doc(alias = "RBX::CustomEvent::~CustomEvent()")]
// 0xf3c104 — j___ZN3RBX11CustomEventD2Ev
pub fn stub_f3c104() -> ! {
    todo!("0xf3c104 j___ZN3RBX11CustomEventD2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEvent> RBX::shared_from<RBX::CustomEvent>(RBX::CustomEvent*)")]
// 0xf3c114 — j___ZN3RBX11shared_fromINS_11CustomEventEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::CustomEvent> RBX::shared_from<RBX::CustomEvent>(RBX::CustomEvent*)
pub fn stub_f3c114() -> ! {
    todo!("0xf3c114 j___ZN3RBX11shared_fromINS_11CustomEventEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::BindableEvent::BindableEvent(void)")]
// 0xf3c124 — j___ZN3RBX13BindableEventC2Ev
pub fn stub_f3c124() -> ! {
    todo!("0xf3c124 j___ZN3RBX13BindableEventC2Ev")
}

#[doc(alias = "RBX::BindableFunction::BindableFunction(void)")]
// 0xf3c264 — j___ZN3RBX16BindableFunctionC2Ev
pub fn stub_f3c264() -> ! {
    todo!("0xf3c264 j___ZN3RBX16BindableFunctionC2Ev")
}

#[doc(alias = "RBX::CustomEventReceiver::CustomEventReceiver(void)")]
// 0xf3c274 — j___ZN3RBX19CustomEventReceiverC2Ev
pub fn stub_f3c274() -> ! {
    todo!("0xf3c274 j___ZN3RBX19CustomEventReceiverC2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)")]
// 0xf3c414 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_
pub fn stub_f3c414() -> ! {
    todo!("0xf3c414 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)")]
// 0xf3c424 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_
pub fn stub_f3c424() -> ! {
    todo!("0xf3c424 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)")]
// 0xf3c434 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_
pub fn stub_f3c434() -> ! {
    todo!("0xf3c434 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HttpService::HttpContentType>(RBX::HttpService::HttpContentType const&)")]
// 0xf3c444 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11HttpService15HttpContentTypeEEERS3_RKT_
pub fn stub_f3c444() -> ! {
    todo!("0xf3c444 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11HttpService15HttpContentTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputType>(RBX::InputObject::UserInputType const&)")]
// 0xf3c454 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_
pub fn stub_f3c454() -> ! {
    todo!("0xf3c454 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&)")]
// 0xf3c464 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_
pub fn stub_f3c464() -> ! {
    todo!("0xf3c464 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)")]
// 0xf3c474 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_
pub fn stub_f3c474() -> ! {
    todo!("0xf3c474 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)")]
// 0xf3c484 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_
pub fn stub_f3c484() -> ! {
    todo!("0xf3c484 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::VideoQuality>(RBX::GameSettings::VideoQuality const&)")]
// 0xf3c494 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings12VideoQualityEEERS3_RKT_
pub fn stub_f3c494() -> ! {
    todo!("0xf3c494 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings12VideoQualityEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::UploadSetting>(RBX::GameSettings::UploadSetting const&)")]
// 0xf3c4a4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings13UploadSettingEEERS3_RKT_
pub fn stub_f3c4a4() -> ! {
    todo!("0xf3c4a4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings13UploadSettingEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)")]
// 0xf3c4b4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12SpecialShape8MeshTypeEEERS3_RKT_
pub fn stub_f3c4b4() -> ! {
    todo!("0xf3c4b4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12SpecialShape8MeshTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)")]
// 0xf3c4c4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13CharacterMesh8BodyPartEEERS3_RKT_
pub fn stub_f3c4c4() -> ! {
    todo!("0xf3c4c4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13CharacterMesh8BodyPartEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendStatus>(RBX::FriendService::FriendStatus const&)")]
// 0xf3c4e4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_
pub fn stub_f3c4e4() -> ! {
    todo!("0xf3c4e4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendEventType>(RBX::FriendService::FriendEventType const&)")]
// 0xf3c4f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_
pub fn stub_f3c4f4() -> ! {
    todo!("0xf3c4f4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SocialService::StuffType>(RBX::SocialService::StuffType const&)")]
// 0xf3c514 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SocialService9StuffTypeEEERS3_RKT_
pub fn stub_f3c514() -> ! {
    todo!("0xf3c514 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SocialService9StuffTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FunctionalTest::Result>(RBX::FunctionalTest::Result const&)")]
// 0xf3c524 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_14FunctionalTest6ResultEEERS3_RKT_
pub fn stub_f3c524() -> ! {
    todo!("0xf3c524 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_14FunctionalTest6ResultEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeyframeSequence::Priority>(RBX::KeyframeSequence::Priority const&)")]
// 0xf3c544 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16KeyframeSequence8PriorityEEERS3_RKT_
pub fn stub_f3c544() -> ! {
    todo!("0xf3c544 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16KeyframeSequence8PriorityEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::LegacyController::InputType>(RBX::LegacyController::InputType const&)")]
// 0xf3c554 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16LegacyController9InputTypeEEERS3_RKT_
pub fn stub_f3c554() -> ! {
    todo!("0xf3c554 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16LegacyController9InputTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::ControlMode>(RBX::GameBasicSettings::ControlMode const&)")]
// 0xf3c574 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings11ControlModeEEERS3_RKT_
pub fn stub_f3c574() -> ! {
    todo!("0xf3c574 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings11ControlModeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0xf3c584 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings20RenderQualitySettingEEERS3_RKT_
pub fn stub_f3c584() -> ! {
    todo!("0xf3c584 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings20RenderQualitySettingEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeywordFilterType>(RBX::KeywordFilterType const&)")]
// 0xf3c594 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17KeywordFilterTypeEEERS3_RKT_
pub fn stub_f3c594() -> ! {
    todo!("0xf3c594 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17KeywordFilterTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&)")]
// 0xf3c5a4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18SkateboardPlatform9MoveStateEEERS3_RKT_
pub fn stub_f3c5a4() -> ! {
    todo!("0xf3c5a4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18SkateboardPlatform9MoveStateEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChangeHistoryService::RuntimeUndoBehavior>(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0xf3c5b4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ChangeHistoryService19RuntimeUndoBehaviorEEERS3_RKT_
pub fn stub_f3c5b4() -> ! {
    todo!("0xf3c5b4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ChangeHistoryService19RuntimeUndoBehaviorEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PersonalServerService::PrivilegeType>(RBX::PersonalServerService::PrivilegeType const&)")]
// 0xf3c5d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_21PersonalServerService13PrivilegeTypeEEERS3_RKT_
pub fn stub_f3c5d4() -> ! {
    todo!("0xf3c5d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_21PersonalServerService13PrivilegeTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Frame::Style>(RBX::Frame::Style const&)")]
// 0xf3c5e4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_
pub fn stub_f3c5e4() -> ! {
    todo!("0xf3c5e4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Joint::JointType>(RBX::Joint::JointType const&)")]
// 0xf3c5f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Joint9JointTypeEEERS3_RKT_
pub fn stub_f3c5f4() -> ! {
    todo!("0xf3c5f4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Joint9JointTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)")]
// 0xf3c604 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_
pub fn stub_f3c604() -> ! {
    todo!("0xf3c604 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellForce>(RBX::Voxel::WaterCellForce const&)")]
// 0xf3c614 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_
pub fn stub_f3c614() -> ! {
    todo!("0xf3c614 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)")]
// 0xf3c624 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_
pub fn stub_f3c624() -> ! {
    todo!("0xf3c624 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellDirection>(RBX::Voxel::WaterCellDirection const&)")]
// 0xf3c634 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_
pub fn stub_f3c634() -> ! {
    todo!("0xf3c634 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)")]
// 0xf3c644 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_
pub fn stub_f3c644() -> ! {
    todo!("0xf3c644 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Action::ActionType>(RBX::Action::ActionType const&)")]
// 0xf3c654 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Action10ActionTypeEEERS3_RKT_
pub fn stub_f3c654() -> ! {
    todo!("0xf3c654 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Action10ActionTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Legacy::SurfaceConstraint>(RBX::Legacy::SurfaceConstraint const&)")]
// 0xf3c664 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_
pub fn stub_f3c664() -> ! {
    todo!("0xf3c664 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::InOut>(RBX::Feature::InOut const&)")]
// 0xf3c674 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature5InOutEEERS3_RKT_
pub fn stub_f3c674() -> ! {
    todo!("0xf3c674 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature5InOutEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::LeftRight>(RBX::Feature::LeftRight const&)")]
// 0xf3c684 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_
pub fn stub_f3c684() -> ! {
    todo!("0xf3c684 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::TopBottom>(RBX::Feature::TopBottom const&)")]
// 0xf3c694 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_
pub fn stub_f3c694() -> ! {
    todo!("0xf3c694 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Handles::VisualStyle>(RBX::Handles::VisualStyle const&)")]
// 0xf3c6a4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Handles11VisualStyleEEERS3_RKT_
pub fn stub_f3c6a4() -> ! {
    todo!("0xf3c6a4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Handles11VisualStyleEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::NameOcclusion>(RBX::Humanoid::NameOcclusion const&)")]
// 0xf3c6b4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid13NameOcclusionEEERS3_RKT_
pub fn stub_f3c6b4() -> ! {
    todo!("0xf3c6b4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid13NameOcclusionEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::Status>(RBX::Humanoid::Status const&)")]
// 0xf3c6c4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid6StatusEEERS3_RKT_
pub fn stub_f3c6c4() -> ! {
    todo!("0xf3c6c4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid6StatusEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Explosion::ExplosionType>(RBX::Explosion::ExplosionType const&)")]
// 0xf3c6d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_
pub fn stub_f3c6d4() -> ! {
    todo!("0xf3c6d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)")]
// 0xf3c6e4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_
pub fn stub_f3c6e4() -> ! {
    todo!("0xf3c6e4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_")
}
