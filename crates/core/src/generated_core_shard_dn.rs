//! core shard DN — 100 core stubs EA-sorted, next uncovered after DM 0x7da8d0 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TextService::XAlignment> const&)")]
// 0x7da928 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_7da928() -> ! {
    todo!("0x7da928 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextService::XAlignment*,std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>>,RBX::TextService::XAlignment const&)")]
// 0x7da990 — __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7da990() -> ! {
    todo!("0x7da990 __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::_M_allocate(unsigned long)")]
// 0x7daa74 — __ZNSt12_Vector_baseIN3RBX11TextService10XAlignmentESaIS2_EE11_M_allocateEm
pub fn stub_7daa74() -> ! {
    todo!("0x7daa74 __ZNSt12_Vector_baseIN3RBX11TextService10XAlignmentESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TextService::XAlignment * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::XAlignment *,RBX::TextService::XAlignment *>(RBX::TextService::XAlignment *,RBX::TextService::XAlignment *,RBX::TextService::XAlignment *)")]
// 0x7daa8c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10XAlignmentES6_EET0_T_S8_S7_
pub fn stub_7daa8c() -> ! {
    todo!("0x7daa8c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10XAlignmentES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TextService::XAlignment*,std::vector<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>>,unsigned long,RBX::TextService::XAlignment const&)")]
// 0x7daac8 — __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_7daac8() -> ! {
    todo!("0x7daac8 __ZNSt6vectorIN3RBX11TextService10XAlignmentESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::resize(unsigned long,RBX::TextService::Font)")]
// 0x7dac58 — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE6resizeEmS2_
pub fn stub_7dac58() -> ! {
    todo!("0x7dac58 __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::push_back(RBX::TextService::Font const&)")]
// 0x7dac8c — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE9push_backERKS2_
pub fn stub_7dac8c() -> ! {
    todo!("0x7dac8c __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TextService::Font,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::operator[](RBX::Name const* const&)")]
// 0x7dacb4 — __ZNSt3mapIPKN3RBX4NameENS0_11TextService4FontESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_7dacb4() -> ! {
    todo!("0x7dacb4 __ZNSt3mapIPKN3RBX4NameENS0_11TextService4FontESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::pair<RBX::Name const* const,RBX::TextService::Font> const&)")]
// 0x7dad0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_7dad0c() -> ! {
    todo!("0x7dad0c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TextService::Font> const&)")]
// 0x7dadc0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_7dadc0() -> ! {
    todo!("0x7dadc0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TextService::Font> const&)")]
// 0x7dae18 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_7dae18() -> ! {
    todo!("0x7dae18 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextService::Font*,std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>>,RBX::TextService::Font const&)")]
// 0x7dae80 — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7dae80() -> ! {
    todo!("0x7dae80 __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_allocate(unsigned long)")]
// 0x7daf64 — __ZNSt12_Vector_baseIN3RBX11TextService4FontESaIS2_EE11_M_allocateEm
pub fn stub_7daf64() -> ! {
    todo!("0x7daf64 __ZNSt12_Vector_baseIN3RBX11TextService4FontESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TextService::Font * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::Font *,RBX::TextService::Font *>(RBX::TextService::Font *,RBX::TextService::Font *,RBX::TextService::Font *)")]
// 0x7daf7c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService4FontES6_EET0_T_S8_S7_
pub fn stub_7daf7c() -> ! {
    todo!("0x7daf7c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService4FontES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TextService::Font*,std::vector<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>>,unsigned long,RBX::TextService::Font const&)")]
// 0x7dafb8 — __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_7dafb8() -> ! {
    todo!("0x7dafb8 __ZNSt6vectorIN3RBX11TextService4FontESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::resize(unsigned long,RBX::TextService::FontSize)")]
// 0x7db148 — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE6resizeEmS2_
pub fn stub_7db148() -> ! {
    todo!("0x7db148 __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::push_back(RBX::TextService::FontSize const&)")]
// 0x7db17c — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE9push_backERKS2_
pub fn stub_7db17c() -> ! {
    todo!("0x7db17c __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TextService::FontSize,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::operator[](RBX::Name const* const&)")]
// 0x7db1a4 — __ZNSt3mapIPKN3RBX4NameENS0_11TextService8FontSizeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_7db1a4() -> ! {
    todo!("0x7db1a4 __ZNSt3mapIPKN3RBX4NameENS0_11TextService8FontSizeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::pair<RBX::Name const* const,RBX::TextService::FontSize> const&)")]
// 0x7db1fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_7db1fc() -> ! {
    todo!("0x7db1fc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TextService::FontSize> const&)")]
// 0x7db2b0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_7db2b0() -> ! {
    todo!("0x7db2b0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TextService::FontSize> const&)")]
// 0x7db308 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_7db308() -> ! {
    todo!("0x7db308 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TextService::FontSize*,std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>>,RBX::TextService::FontSize const&)")]
// 0x7db370 — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_7db370() -> ! {
    todo!("0x7db370 __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_allocate(unsigned long)")]
// 0x7db454 — __ZNSt12_Vector_baseIN3RBX11TextService8FontSizeESaIS2_EE11_M_allocateEm
pub fn stub_7db454() -> ! {
    todo!("0x7db454 __ZNSt12_Vector_baseIN3RBX11TextService8FontSizeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TextService::FontSize * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::FontSize *,RBX::TextService::FontSize *>(RBX::TextService::FontSize *,RBX::TextService::FontSize *,RBX::TextService::FontSize *)")]
// 0x7db46c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService8FontSizeES6_EET0_T_S8_S7_
pub fn stub_7db46c() -> ! {
    todo!("0x7db46c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService8FontSizeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TextService::FontSize*,std::vector<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>>,unsigned long,RBX::TextService::FontSize const&)")]
// 0x7db4a8 — __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_7db4a8() -> ! {
    todo!("0x7db4a8 __ZNSt6vectorIN3RBX11TextService8FontSizeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "createSanitizedURL(std::string)")]
// 0x7dcbc8 — __ZL18createSanitizedURLSs
pub fn stub_7dcbc8() -> ! {
    todo!("0x7dcbc8 __ZL18createSanitizedURLSs")
}

#[doc(alias = "RBX::Cocoa::httpGetPostCocoa(std::string const&,std::string const&,bool,std::istream &,bool,std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&,std::string &)")]
// 0x7dd5d4 — __ZN3RBX5Cocoa16httpGetPostCocoaERKSsS2_bRSibRKSt3mapISsSsSt4lessISsESaISt4pairIS1_SsEEERSs
pub fn stub_7dd5d4() -> ! {
    todo!("0x7dd5d4 __ZN3RBX5Cocoa16httpGetPostCocoaERKSsS2_bRSibRKSt3mapISsSsSt4lessISsESaISt4pairIS1_SsEEERSs")
}

#[doc(alias = "virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e34a4 — __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
// was: `virtual thunk to'boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()
pub fn stub_7e34a4() -> ! {
    todo!("0x7e34a4 __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev")
}

#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e34b0 — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
pub fn stub_7e34b0() -> ! {
    todo!("0x7e34b0 __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::notify(void)")]
// 0x7e3550 — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EE6notifyEv
pub fn stub_7e3550() -> ! {
    todo!("0x7e3550 __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EE6notifyEv")
}

#[doc(alias = "non-virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e3568 — __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
// was: `non-virtual thunk to'boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()
pub fn stub_7e3568() -> ! {
    todo!("0x7e3568 __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev")
}

#[doc(alias = "virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e3570 — __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
// was: `virtual thunk to'boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()
pub fn stub_7e3570() -> ! {
    todo!("0x7e3570 __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e357c — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev
pub fn stub_7e357c() -> ! {
    todo!("0x7e357c __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3660 — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev
pub fn stub_7e3660() -> ! {
    todo!("0x7e3660 __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3754 — __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev
// was: `non-virtual thunk to'boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()
pub fn stub_7e3754() -> ! {
    todo!("0x7e3754 __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3830 — __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev
// was: `non-virtual thunk to'boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()
pub fn stub_7e3830() -> ! {
    todo!("0x7e3830 __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev")
}

#[doc(alias = "virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3928 — __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev
// was: `virtual thunk to'boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()
pub fn stub_7e3928() -> ! {
    todo!("0x7e3928 __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev")
}

#[doc(alias = "virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3a08 — __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev
// was: `virtual thunk to'boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()
pub fn stub_7e3a08() -> ! {
    todo!("0x7e3a08 __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::filtering_stream_base(void)")]
// 0x7e3b04 — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EEC2Ev
pub fn stub_7e3b04() -> ! {
    todo!("0x7e3b04 __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EEC2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::shared_ptr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>(boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl *)")]
// 0x7e3c20 — __ZN5boost10shared_ptrINS_9iostreams6detail10chain_baseINS1_5chainINS1_6outputEcSt11char_traitsIcESaIcEEEcS7_S8_S5_E10chain_implEEC2ISB_EEPT_
// was: boost::shared_ptr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::shared_ptr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>(boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl *)
pub fn stub_7e3c20() -> ! {
    todo!("0x7e3c20 __ZN5boost10shared_ptrINS_9iostreams6detail10chain_baseINS1_5chainINS1_6outputEcSt11char_traitsIcESaIcEEEcS7_S8_S5_E10chain_implEEC2ISB_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>(boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl *)")]
// 0x7e3cf4 — __ZN5boost6detail12shared_countC2INS_9iostreams6detail10chain_baseINS3_5chainINS3_6outputEcSt11char_traitsIcESaIcEEEcS9_SA_S7_E10chain_implEEEPT_
pub fn stub_7e3cf4() -> ! {
    todo!("0x7e3cf4 __ZN5boost6detail12shared_countC2INS_9iostreams6detail10chain_baseINS3_5chainINS3_6outputEcSt11char_traitsIcESaIcEEEcS9_SA_S7_E10chain_implEEEPT_")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::~chain_impl()")]
// 0x7e3e00 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_implD2Ev
pub fn stub_7e3e00() -> ! {
    todo!("0x7e3e00 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_implD2Ev")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::reset(void)")]
// 0x7e3ef8 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5resetEv
pub fn stub_7e3ef8() -> ! {
    todo!("0x7e3ef8 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5resetEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::~sp_counted_impl_p()")]
// 0x7e3f50 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED1Ev
pub fn stub_7e3f50() -> ! {
    todo!("0x7e3f50 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::~sp_counted_impl_p()")]
// 0x7e3f54 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED0Ev
pub fn stub_7e3f54() -> ! {
    todo!("0x7e3f54 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::dispose(void)")]
// 0x7e3f58 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE7disposeEv
pub fn stub_7e3f58() -> ! {
    todo!("0x7e3f58 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::get_deleter(std::type_info const&)")]
// 0x7e3ffc — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE11get_deleterERKSt9type_info
pub fn stub_7e3ffc() -> ! {
    todo!("0x7e3ffc __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::get_untyped_deleter(void)")]
// 0x7e4000 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE19get_untyped_deleterEv
pub fn stub_7e4000() -> ! {
    todo!("0x7e4000 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::iostreams::access_control<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>,boost::iostreams::public_,boost::iostreams::detail::pub_<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>>>::~access_control()")]
// 0x7e4004 — __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED1Ev
pub fn stub_7e4004() -> ! {
    todo!("0x7e4004 __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED1Ev")
}

#[doc(alias = "boost::iostreams::access_control<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>,boost::iostreams::public_,boost::iostreams::detail::pub_<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>>>::~access_control()")]
// 0x7e4008 — __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED0Ev
pub fn stub_7e4008() -> ! {
    todo!("0x7e4008 __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>::notify(void)")]
// 0x7e400c — __ZN5boost9iostreams6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEE6notifyEv
pub fn stub_7e400c() -> ! {
    todo!("0x7e400c __ZN5boost9iostreams6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEE6notifyEv")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::operator=(std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&)")]
// 0x7e4010 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EEaSERKS8_
pub fn stub_7e4010() -> ! {
    todo!("0x7e4010 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EEaSERKS8_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)")]
// 0x7e405c — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSA_
pub fn stub_7e405c() -> ! {
    todo!("0x7e405c __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSA_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_create_node(std::pair<std::string const,std::string> const&)")]
// 0x7e41b0 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_7e41b0() -> ! {
    todo!("0x7e41b0 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "boost::iostreams::gzip_params::gzip_params(int,int,int,int,int,std::string,std::string,long)")]
// 0x7e42a8 — __ZN5boost9iostreams11gzip_paramsC2EiiiiiSsSsl
pub fn stub_7e42a8() -> ! {
    todo!("0x7e42a8 __ZN5boost9iostreams11gzip_paramsC2EiiiiiSsSsl")
}

#[doc(alias = "RBX::ContentProviderJob::setExecutionMode(RBX::ContentProviderJob::ExecutionMode)")]
// 0x7e4634 — __ZN3RBX18ContentProviderJob16setExecutionModeENS0_13ExecutionModeE
pub fn stub_7e4634() -> ! {
    todo!("0x7e4634 __ZN3RBX18ContentProviderJob16setExecutionModeENS0_13ExecutionModeE")
}

#[doc(alias = "RBX::ContentProviderJob::abort(void)")]
// 0x7e463c — __ZN3RBX18ContentProviderJob5abortEv
pub fn stub_7e463c() -> ! {
    todo!("0x7e463c __ZN3RBX18ContentProviderJob5abortEv")
}

#[doc(alias = "RBX::ContentProviderJob::addTask(std::string const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// 0x7e4644 — __ZN3RBX18ContentProviderJob7addTaskERKSsNS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIS1_EE
// was: RBX::ContentProviderJob::addTask(std::string const&,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
pub fn stub_7e4644() -> ! {
    todo!("0x7e4644 __ZN3RBX18ContentProviderJob7addTaskERKSsNS_14AsyncHttpQueue13RequestResultEPSiN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::ContentProviderJob::processTask(RBX::ContentProviderJob::ContentProviderTask const&)")]
// 0x7e4a24 — __ZN3RBX18ContentProviderJob11processTaskERKNS0_19ContentProviderTaskE
pub fn stub_7e4a24() -> ! {
    todo!("0x7e4a24 __ZN3RBX18ContentProviderJob11processTaskERKNS0_19ContentProviderTaskE")
}

#[doc(alias = "RBX::ContentProviderJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x7e4ce4 — __ZN3RBX18ContentProviderJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_7e4ce4() -> ! {
    todo!("0x7e4ce4 __ZN3RBX18ContentProviderJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::ContentProviderJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x7e4d00 — __ZN3RBX18ContentProviderJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_7e4d00() -> ! {
    todo!("0x7e4d00 __ZN3RBX18ContentProviderJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "rbx::safe_queue<RBX::ContentProviderJob::ContentProviderTask>::push(RBX::ContentProviderJob::ContentProviderTask const&)")]
// 0x7e4f34 — __ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE4pushERKS3_
pub fn stub_7e4f34() -> ! {
    todo!("0x7e4f34 __ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE4pushERKS3_")
}

#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::operator()(std::string,rbx_core::SharedPtr<std::string const>)const")]
// 0x7e4ff8 — __ZNK5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEclESsS6_
// was: boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::operator()(std::string,boost::shared_ptr<std::string const>)const
pub fn stub_7e4ff8() -> ! {
    todo!("0x7e4ff8 __ZNK5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEclESsS6_")
}

#[doc(alias = "rbx::safe_queue<RBX::ContentProviderJob::ContentProviderTask>::pop_if_present(RBX::ContentProviderJob::ContentProviderTask&)")]
// 0x7e51a4 — __ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE14pop_if_presentERS3_
pub fn stub_7e51a4() -> ! {
    todo!("0x7e51a4 __ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE14pop_if_presentERS3_")
}

#[doc(alias = "RBX::ContentProviderJob::~ContentProviderJob()")]
// 0x7e5298 — __ZN3RBX18ContentProviderJobD1Ev
pub fn stub_7e5298() -> ! {
    todo!("0x7e5298 __ZN3RBX18ContentProviderJobD1Ev")
}

#[doc(alias = "RBX::ContentProviderJob::~ContentProviderJob()")]
// 0x7e53d8 — __ZN3RBX18ContentProviderJobD0Ev
pub fn stub_7e53d8() -> ! {
    todo!("0x7e53d8 __ZN3RBX18ContentProviderJobD0Ev")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::pop_front(void)")]
// 0x7e5528 — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9pop_frontEv
pub fn stub_7e5528() -> ! {
    todo!("0x7e5528 __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9pop_frontEv")
}

#[doc(alias = "__gnu_cxx::new_allocator<RBX::ContentProviderJob::ContentProviderTask>::destroy(RBX::ContentProviderJob::ContentProviderTask*)")]
// 0x7e5560 — __ZN9__gnu_cxx13new_allocatorIN3RBX18ContentProviderJob19ContentProviderTaskEE7destroyEPS3_
pub fn stub_7e5560() -> ! {
    todo!("0x7e5560 __ZN9__gnu_cxx13new_allocatorIN3RBX18ContentProviderJob19ContentProviderTaskEE7destroyEPS3_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::push_back(RBX::ContentProviderJob::ContentProviderTask const&)")]
// 0x7e5604 — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9push_backERKS2_
pub fn stub_7e5604() -> ! {
    todo!("0x7e5604 __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_push_back_aux(RBX::ContentProviderJob::ContentProviderTask const&)")]
// 0x7e56f4 — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE16_M_push_back_auxERKS2_
pub fn stub_7e56f4() -> ! {
    todo!("0x7e56f4 __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE16_M_push_back_auxERKS2_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_reserve_map_at_back(unsigned long)")]
// 0x7e59b0 — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE22_M_reserve_map_at_backEm
pub fn stub_7e59b0() -> ! {
    todo!("0x7e59b0 __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_reallocate_map(unsigned long,bool)")]
// 0x7e59cc — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_reallocate_mapEmb
pub fn stub_7e59cc() -> ! {
    todo!("0x7e59cc __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_allocate_map(unsigned long)")]
// 0x7e5aa4 — __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_allocate_mapEm
pub fn stub_7e5aa4() -> ! {
    todo!("0x7e5aa4 __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_allocate_mapEm")
}

#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::assign_to_own(boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>> const&)")]
// 0x7e5abc — __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE13assign_to_ownERKS7_
// was: boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>>::assign_to_own(boost::function2<RBX::TaskScheduler::StepResult,std::string,boost::shared_ptr<std::string const>> const&)
pub fn stub_7e5abc() -> ! {
    todo!("0x7e5abc __ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE13assign_to_ownERKS7_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::~deque()")]
// 0x7e5aec — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev
pub fn stub_7e5aec() -> ! {
    todo!("0x7e5aec __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::~_Deque_base()")]
// 0x7e5bd4 — __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev
pub fn stub_7e5bd4() -> ! {
    todo!("0x7e5bd4 __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>)")]
// 0x7e5c00 — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_
pub fn stub_7e5c00() -> ! {
    todo!("0x7e5c00 __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_initialize_map(unsigned long)")]
// 0x7e5d84 — __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_initialize_mapEm
pub fn stub_7e5d84() -> ! {
    todo!("0x7e5d84 __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_create_nodes(RBX::ContentProviderJob::ContentProviderTask**,RBX::ContentProviderJob::ContentProviderTask**)")]
// 0x7e5f04 — __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_7e5f04() -> ! {
    todo!("0x7e5f04 __ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_create_nodesEPPS2_S6_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::deque(std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>> const&)")]
// 0x7e5ff8 — __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EEC2ERKS4_
pub fn stub_7e5ff8() -> ! {
    todo!("0x7e5ff8 __ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EEC2ERKS4_")
}

#[doc(alias = "std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask const&,RBX::ContentProviderJob::ContentProviderTask const*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>>(std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask const&,RBX::ContentProviderJob::ContentProviderTask const*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask const&,RBX::ContentProviderJob::ContentProviderTask const*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>,std::__false_type)")]
// 0x7e612c — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX18ContentProviderJob19ContentProviderTaskERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type
pub fn stub_7e612c() -> ! {
    todo!("0x7e612c __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX18ContentProviderJob19ContentProviderTaskERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type")
}

#[doc(alias = "RBX::MeshContentProvider::MeshContentProvider(void)")]
// 0x7e6498 — __ZN3RBX19MeshContentProviderC1Ev
pub fn stub_7e6498() -> ! {
    todo!("0x7e6498 __ZN3RBX19MeshContentProviderC1Ev")
}

#[doc(alias = "RBX::MeshContentProvider::MeshContentProvider(void)")]
// 0x7e649c — __ZN3RBX19MeshContentProviderC2Ev
pub fn stub_7e649c() -> ! {
    todo!("0x7e649c __ZN3RBX19MeshContentProviderC2Ev")
}

#[doc(alias = "RBX::MeshContentProvider::ProcessTask(std::string const&,rbx_core::SharedPtr<std::string const>)")]
// 0x7e66a0 — __ZN3RBX19MeshContentProvider11ProcessTaskERKSsN5boost10shared_ptrIS1_EE
// was: RBX::MeshContentProvider::ProcessTask(std::string const&,boost::shared_ptr<std::string const>)
pub fn stub_7e66a0() -> ! {
    todo!("0x7e66a0 __ZN3RBX19MeshContentProvider11ProcessTaskERKSsN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::MeshContentProvider::updateContent(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>)")]
// 0x7e68ec — __ZN3RBX19MeshContentProvider13updateContentERKSsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEE
// was: RBX::MeshContentProvider::updateContent(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>)
pub fn stub_7e68ec() -> ! {
    todo!("0x7e68ec __ZN3RBX19MeshContentProvider13updateContentERKSsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEE")
}

#[doc(alias = "void rbx_core::SharedPtr<void>::reset<RBX::FileMeshData>(RBX::FileMeshData *)")]
// 0x7e6a94 — __ZN5boost10shared_ptrIvE5resetIN3RBX12FileMeshDataEEEvPT_
// was: void boost::shared_ptr<void>::reset<RBX::FileMeshData>(RBX::FileMeshData *)
pub fn stub_7e6a94() -> ! {
    todo!("0x7e6a94 __ZN5boost10shared_ptrIvE5resetIN3RBX12FileMeshDataEEEvPT_")
}

#[doc(alias = "RBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6ac0 — __ZN3RBX19MeshContentProviderD1Ev
pub fn stub_7e6ac0() -> ! {
    todo!("0x7e6ac0 __ZN3RBX19MeshContentProviderD1Ev")
}

#[doc(alias = "RBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6ac4 — __ZN3RBX19MeshContentProviderD0Ev
pub fn stub_7e6ac4() -> ! {
    todo!("0x7e6ac4 __ZN3RBX19MeshContentProviderD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6b8c — __ZThn32_N3RBX19MeshContentProviderD1Ev
// was: `non-virtual thunk to'RBX::MeshContentProvider::~MeshContentProvider()
pub fn stub_7e6b8c() -> ! {
    todo!("0x7e6b8c __ZThn32_N3RBX19MeshContentProviderD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6b94 — __ZThn32_N3RBX19MeshContentProviderD0Ev
// was: `non-virtual thunk to'RBX::MeshContentProvider::~MeshContentProvider()
pub fn stub_7e6b94() -> ! {
    todo!("0x7e6b94 __ZThn32_N3RBX19MeshContentProviderD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6bc4 — __ZThn36_N3RBX19MeshContentProviderD1Ev
// was: `non-virtual thunk to'RBX::MeshContentProvider::~MeshContentProvider()
pub fn stub_7e6bc4() -> ! {
    todo!("0x7e6bc4 __ZThn36_N3RBX19MeshContentProviderD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6bcc — __ZThn36_N3RBX19MeshContentProviderD0Ev
// was: `non-virtual thunk to'RBX::MeshContentProvider::~MeshContentProvider()
pub fn stub_7e6bcc() -> ! {
    todo!("0x7e6bcc __ZThn36_N3RBX19MeshContentProviderD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6bd4 — __ZThn96_N3RBX19MeshContentProviderD1Ev
// was: `non-virtual thunk to'RBX::MeshContentProvider::~MeshContentProvider()
pub fn stub_7e6bd4() -> ! {
    todo!("0x7e6bd4 __ZThn96_N3RBX19MeshContentProviderD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::MeshContentProvider::~MeshContentProvider()")]
// 0x7e6bdc — __ZThn96_N3RBX19MeshContentProviderD0Ev
// was: `non-virtual thunk to'RBX::MeshContentProvider::~MeshContentProvider()
pub fn stub_7e6bdc() -> ! {
    todo!("0x7e6bdc __ZThn96_N3RBX19MeshContentProviderD0Ev")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
// 0x7e6cc8 — __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m
// was: RBX::ControlledLRUCache<std::string,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)
pub fn stub_7e6cc8() -> ! {
    todo!("0x7e6cc8 __ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0x7e6e0c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
pub fn stub_7e6e0c() -> ! {
    todo!("0x7e6e0c __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0x7e6e38 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const
pub fn stub_7e6e38() -> ! {
    todo!("0x7e6e38 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0x7e6e78 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSK_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEmRKT_RKT0_
// was: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,boost::shared_ptr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const
pub fn stub_7e6e78() -> ! {
    todo!("0x7e6e78 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSK_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEmRKT_RKT0_")
}

#[doc(alias = "rbx_core::SharedPtr<void>::shared_ptr<RBX::FileMeshData>(RBX::FileMeshData *)")]
// 0x7e6ee4 — __ZN5boost10shared_ptrIvEC2IN3RBX12FileMeshDataEEEPT_
// was: boost::shared_ptr<void>::shared_ptr<RBX::FileMeshData>(RBX::FileMeshData *)
pub fn stub_7e6ee4() -> ! {
    todo!("0x7e6ee4 __ZN5boost10shared_ptrIvEC2IN3RBX12FileMeshDataEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FileMeshData>(RBX::FileMeshData *)")]
// 0x7e6fb8 — __ZN5boost6detail12shared_countC2IN3RBX12FileMeshDataEEEPT_
pub fn stub_7e6fb8() -> ! {
    todo!("0x7e6fb8 __ZN5boost6detail12shared_countC2IN3RBX12FileMeshDataEEEPT_")
}