//! core bg14 — 100 core stubs EA-sorted asc distinct not in /tmp/global_eas.txt.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua, exclude boost) global distinct not yet in /tmp/global_eas.txt — next 100 uncovered after 0xf287e4 (prior max 0xf287e4) -> 0xf2df14..0xf30bd4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_")]
// 0xf2df14 — j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_
// type: int()
pub fn stub_0xf2df14() -> ! {
    todo!("0xf2df14 j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_")
}

#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev")]
// 0xf2df24 — j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf2df24() -> ! {
    todo!("0xf2df24 j___ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf2df34 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int()
pub fn stub_0xf2df34() -> ! {
    todo!("0xf2df34 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// 0xf2df44 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int()
pub fn stub_0xf2df44() -> ! {
    todo!("0xf2df44 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_")]
// 0xf2df54 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_
// type: int()
pub fn stub_0xf2df54() -> ! {
    todo!("0xf2df54 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_")]
// 0xf2df64 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_
// type: int()
pub fn stub_0xf2df64() -> ! {
    todo!("0xf2df64 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_")]
// 0xf2df74 — j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_
// type: int()
pub fn stub_0xf2df74() -> ! {
    todo!("0xf2df74 j___ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0xf2df84 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int()
pub fn stub_0xf2df84() -> ! {
    todo!("0xf2df84 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf2df94 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf2df94() -> ! {
    todo!("0xf2df94 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2dfa4 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xf2dfa4() -> ! {
    todo!("0xf2dfa4 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// 0xf2dfb4 — j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int()
pub fn stub_0xf2dfb4() -> ! {
    todo!("0xf2dfb4 j___ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")]
#[doc(alias = "j___ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type")]
// 0xf2dfc4 — j___ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, int *, int, int, int, int, int, void *, int)
pub fn stub_0xf2dfc4() -> ! {
    todo!("0xf2dfc4 j___ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")]
#[doc(alias = "j___ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_")]
// 0xf2dfd4 — j___ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_
// type: int()
pub fn stub_0xf2dfd4() -> ! {
    todo!("0xf2dfd4 j___ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_")
}

#[doc(alias = "std::map<std::string,RBX::ContentFilter::ResultEntry,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::operator[](std::string const&)")]
#[doc(alias = "j___ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
// 0xf2e284 — j___ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
// type: int()
pub fn stub_0xf2e284() -> ! {
    todo!("0xf2e284 j___ZNSt3mapISsN3RBX13ContentFilter11ResultEntryESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::lower_bound(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs")]
// 0xf2e294 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs
// type: int __fastcall(int, std::string *)
pub fn stub_0xf2e294() -> ! {
    todo!("0xf2e294 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11lower_boundERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::upper_bound(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs")]
// 0xf2e2a4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs
// type: _DWORD *__fastcall(int, std::string *this)
pub fn stub_0xf2e2a4() -> ! {
    todo!("0xf2e2a4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE11upper_boundERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs")]
// 0xf2e2b4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs
// type: int __fastcall(int, std::string *this)
pub fn stub_0xf2e2b4() -> ! {
    todo!("0xf2e2b4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE4findERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs")]
// 0xf2e2c4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs
// type: int()
pub fn stub_0xf2e2c4() -> ! {
    todo!("0xf2e2c4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseERKSs")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE")]
// 0xf2e2d4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf2e2d4() -> ! {
    todo!("0xf2e2d4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsE")
}

#[doc(alias = "std::_Rb_tree<std::string,std::string,std::_Identity<std::string>,std::less<std::string>,std::allocator<std::string>>::erase(std::_Rb_tree_iterator<std::string>,std::_Rb_tree_iterator<std::string>)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_")]
// 0xf2e2e4 — j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf2e2e4() -> ! {
    todo!("0xf2e2e4 j___ZNSt8_Rb_treeISsSsSt9_IdentityISsESt4lessISsESaISsEE5eraseESt17_Rb_tree_iteratorISsES7_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::lower_bound(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")]
// 0xf2e2f4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
pub fn stub_0xf2e2f4() -> ! {
    todo!("0xf2e2f4 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_create_node(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")]
// 0xf2e304 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf2e304() -> ! {
    todo!("0xf2e304 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
// 0xf2e314 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
pub fn stub_0xf2e314() -> ! {
    todo!("0xf2e314 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
// 0xf2e324 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
pub fn stub_0xf2e324() -> ! {
    todo!("0xf2e324 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::find(std::string const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
// 0xf2e334 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// type: int __fastcall(int, std::string *this)
pub fn stub_0xf2e334() -> ! {
    todo!("0xf2e334 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E")]
// 0xf2e344 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf2e344() -> ! {
    todo!("0xf2e344 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ContentFilter::ResultEntry>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// 0xf2e354 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xf2e354() -> ! {
    todo!("0xf2e354 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ContentFilter::ResultEntry>,std::_Select1st<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ContentFilter::ResultEntry>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ContentFilter::ResultEntry> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// 0xf2e364 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf2e364() -> ! {
    todo!("0xf2e364 j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ContentFilter11ResultEntryEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "RBX::Face::getAxis(int)const")]
#[doc(alias = "j___ZNK3RBX4Face7getAxisEi")]
// 0xf2e6e4 — j___ZNK3RBX4Face7getAxisEi
// type: _DWORD __fastcall(RBX::Face *__hidden this, int)
pub fn stub_0xf2e6e4() -> ! {
    todo!("0xf2e6e4 j___ZNK3RBX4Face7getAxisEi")
}

#[doc(alias = "RBX::IndexedMesh::lowersChanged(void)")]
#[doc(alias = "j___ZN3RBX11IndexedMesh13lowersChangedEv")]
// 0xf2e714 — j___ZN3RBX11IndexedMesh13lowersChangedEv
// type: _DWORD __fastcall(RBX::IndexedMesh *__hidden this)
pub fn stub_0xf2e714() -> ! {
    todo!("0xf2e714 j___ZN3RBX11IndexedMesh13lowersChangedEv")
}

#[doc(alias = "RBX::IndexedMesh * RBX::IndexedTree::getTypedChild<RBX::IndexedMesh>(int)")]
#[doc(alias = "j___ZN3RBX11IndexedTree13getTypedChildINS_11IndexedMeshEEEPT_i")]
// 0xf2e724 — j___ZN3RBX11IndexedTree13getTypedChildINS_11IndexedMeshEEEPT_i
// type: 
pub fn stub_0xf2e724() -> ! {
    todo!("0xf2e724 j___ZN3RBX11IndexedTree13getTypedChildINS_11IndexedMeshEEEPT_i")
}

#[doc(alias = "RBX::IndexArray<RBX::IndexedTree,&RBX::IndexedTree::getIndex>::fastRemove(RBX::IndexedTree*)")]
#[doc(alias = "j___ZN3RBX10IndexArrayINS_11IndexedTreeEXadL_ZNS1_8getIndexEvEEE10fastRemoveEPS1_")]
// 0xf2e784 — j___ZN3RBX10IndexArrayINS_11IndexedTreeEXadL_ZNS1_8getIndexEvEEE10fastRemoveEPS1_
// type: 
pub fn stub_0xf2e784() -> ! {
    todo!("0xf2e784 j___ZN3RBX10IndexArrayINS_11IndexedTreeEXadL_ZNS1_8getIndexEvEEE10fastRemoveEPS1_")
}

#[doc(alias = "std::_Vector_base<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm")]
// 0xf2e844 — j___ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2e844() -> ! {
    todo!("0xf2e844 j___ZNSt12_Vector_baseIN3RBX17KeywordFilterTypeESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::KeywordFilterType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::KeywordFilterType *,RBX::KeywordFilterType *>(RBX::KeywordFilterType *,RBX::KeywordFilterType *,RBX::KeywordFilterType *)")]
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_")]
// 0xf2e854 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2e854() -> ! {
    todo!("0xf2e854 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17KeywordFilterTypeES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::KeywordFilterType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_17KeywordFilterTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")]
// 0xf2e864 — j___ZNSt3mapIPKN3RBX4NameENS0_17KeywordFilterTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: 
pub fn stub_0xf2e864() -> ! {
    todo!("0xf2e864 j___ZNSt3mapIPKN3RBX4NameENS0_17KeywordFilterTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,RBX::KeywordFilterType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// 0xf2e874 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: 
pub fn stub_0xf2e874() -> ! {
    todo!("0xf2e874 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::KeywordFilterType*,std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>>,unsigned long,RBX::KeywordFilterType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// 0xf2e884 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: 
pub fn stub_0xf2e884() -> ! {
    todo!("0xf2e884 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::resize(unsigned long,RBX::KeywordFilterType)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE6resizeEmS1_")]
// 0xf2e894 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE6resizeEmS1_
// type: 
pub fn stub_0xf2e894() -> ! {
    todo!("0xf2e894 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::KeywordFilterType,std::allocator<RBX::KeywordFilterType>>::push_back(RBX::KeywordFilterType const&)")]
#[doc(alias = "j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE9push_backERKS1_")]
// 0xf2e8a4 — j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE9push_backERKS1_
// type: 
pub fn stub_0xf2e8a4() -> ! {
    todo!("0xf2e8a4 j___ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")]
// 0xf2e8b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: 
pub fn stub_0xf2e8b4() -> ! {
    todo!("0xf2e8b4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
// 0xf2e8c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0xf2e8c4() -> ! {
    todo!("0xf2e8c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::KeywordFilterType> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
// 0xf2e8d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: 
pub fn stub_0xf2e8d4() -> ! {
    todo!("0xf2e8d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::MeshId>(RBX::MeshId const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6MeshIdEEERS3_RKT_")]
// 0xf2f4d4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6MeshIdEEERS3_RKT_
// type: 
pub fn stub_0xf2f4d4() -> ! {
    todo!("0xf2f4d4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6MeshIdEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE9singletonEv")]
// 0xf2f4e4 — j___ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE9singletonEv
// type: 
pub fn stub_0xf2f4e4() -> ! {
    todo!("0xf2f4e4 j___ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE9singletonEv")
}

#[doc(alias = "RBX::MeshId * rbx::any_cast<RBX::MeshId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "j___ZN3rbx8any_castIN3RBX6MeshIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0xf2f4f4 — j___ZN3rbx8any_castIN3RBX6MeshIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf2f4f4() -> ! {
    todo!("0xf2f4f4 j___ZN3rbx8any_castIN3RBX6MeshIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::MeshId & rbx::any_cast<RBX::MeshId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0xf2f504 — j___ZN3rbx8any_castIRN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: 
pub fn stub_0xf2f504() -> ! {
    todo!("0xf2f504 j___ZN3rbx8any_castIRN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::queuing_rw_mutex::queuing_rw_mutex(void)")]
#[doc(alias = "j___ZN3RBX16queuing_rw_mutexC2Ev")]
// 0xf2f514 — j___ZN3RBX16queuing_rw_mutexC2Ev
// type: _DWORD __fastcall(RBX::queuing_rw_mutex *__hidden this)
pub fn stub_0xf2f514() -> ! {
    todo!("0xf2f514 j___ZN3RBX16queuing_rw_mutexC2Ev")
}

#[doc(alias = "std::_Vector_base<RBX::Name *,std::allocator<RBX::Name *>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX4NameESaIS2_EE11_M_allocateEm")]
// 0xf2f604 — j___ZNSt12_Vector_baseIPN3RBX4NameESaIS2_EE11_M_allocateEm
// type: 
pub fn stub_0xf2f604() -> ! {
    todo!("0xf2f604 j___ZNSt12_Vector_baseIPN3RBX4NameESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX4NameESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf2f614 — j___ZNSt6vectorIPN3RBX4NameESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_0xf2f614() -> ! {
    todo!("0xf2f614 j___ZNSt6vectorIPN3RBX4NameESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::insert(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
#[doc(alias = "j___ZNSt6vectorIPN3RBX4NameESaIS2_EE6insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0xf2f624 — j___ZNSt6vectorIPN3RBX4NameESaIS2_EE6insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int)
pub fn stub_0xf2f624() -> ! {
    todo!("0xf2f624 j___ZNSt6vectorIPN3RBX4NameESaIS2_EE6insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<int,std::allocator<int>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIiSaIiEE11_M_allocateEm")]
// 0xf2f634 — j___ZNSt12_Vector_baseIiSaIiEE11_M_allocateEm
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2f634() -> ! {
    todo!("0xf2f634 j___ZNSt12_Vector_baseIiSaIiEE11_M_allocateEm")
}

#[doc(alias = "rbx::implementation::typed_holder<double>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIdE9singletonEv")]
// 0xf2f764 — j___ZN3rbx14implementation12typed_holderIdE9singletonEv
// type: int(void)
pub fn stub_0xf2f764() -> ! {
    todo!("0xf2f764 j___ZN3rbx14implementation12typed_holderIdE9singletonEv")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::RunTransition)>::operator()(RBX::RunTransition)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_")]
// 0xf2f774 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_
// type: 
pub fn stub_0xf2f774() -> ! {
    todo!("0xf2f774 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Stepped const&)>::operator()(RBX::Stepped const&)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_")]
// 0xf2f784 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf2f784() -> ! {
    todo!("0xf2f784 j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7SteppedEEEclES5_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Heartbeat const&)>::operator()(RBX::Heartbeat const&)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_")]
// 0xf2f794 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf2f794() -> ! {
    todo!("0xf2f794 j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9HeartbeatEEEclES5_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(double)>::operator()(double)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd")]
// 0xf2f7a4 — j___ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf2f7a4() -> ! {
    todo!("0xf2f7a4 j___ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(double,double)>::operator()(double,double)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd")]
// 0xf2f7c4 — j___ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd
// type: 
pub fn stub_0xf2f7c4() -> ! {
    todo!("0xf2f7c4 j___ZN3rbx7signals16signal_with_argsILi2EFvddEEclEdd")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv")]
// 0xf2f7d4 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2f7d4() -> ! {
    todo!("0xf2f7d4 j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception")]
// 0xf2f7f4 — j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception
// type: 
pub fn stub_0xf2f7f4() -> ! {
    todo!("0xf2f7f4 j___ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv")]
// 0xf2f834 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2f834() -> ! {
    todo!("0xf2f834 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv")]
// 0xf2f844 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf2f844() -> ! {
    todo!("0xf2f844 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception")]
// 0xf2f864 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception
// type: 
pub fn stub_0xf2f864() -> ! {
    todo!("0xf2f864 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv")]
// 0xf2f874 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2f874() -> ! {
    todo!("0xf2f874 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception")]
// 0xf2f894 — j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception
// type: 
pub fn stub_0xf2f894() -> ! {
    todo!("0xf2f894 j___ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvdEE13disconnectAllEv")]
// 0xf2f8a4 — j___ZN3rbx7signals6signalIFvdEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2f8a4() -> ! {
    todo!("0xf2f8a4 j___ZN3rbx7signals6signalIFvdEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv")]
// 0xf2f8b4 — j___ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf2f8b4() -> ! {
    todo!("0xf2f8b4 j___ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv")]
// 0xf2f8d4 — j___ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf2f8d4() -> ! {
    todo!("0xf2f8d4 j___ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::insert(rbx::signals::signal<void ()(double)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE")]
// 0xf2f8e4 — j___ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf2f8e4() -> ! {
    todo!("0xf2f8e4 j___ZN3rbx7signals6signalIFvdEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::remove(rbx::signals::signal<void ()(double)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE")]
// 0xf2f8f4 — j___ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf2f8f4() -> ! {
    todo!("0xf2f8f4 j___ZN3rbx7signals6signalIFvdEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception")]
// 0xf2f914 — j___ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception
// type: 
pub fn stub_0xf2f914() -> ! {
    todo!("0xf2f914 j___ZN3rbx7signals6signalIFvdEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvddEE13disconnectAllEv")]
// 0xf2f924 — j___ZN3rbx7signals6signalIFvddEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2f924() -> ! {
    todo!("0xf2f924 j___ZN3rbx7signals6signalIFvddEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv")]
// 0xf2f934 — j___ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf2f934() -> ! {
    todo!("0xf2f934 j___ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv")]
// 0xf2f954 — j___ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv
// type: 
pub fn stub_0xf2f954() -> ! {
    todo!("0xf2f954 j___ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::insert(rbx::signals::signal<void ()(double,double)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE")]
// 0xf2f964 — j___ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf2f964() -> ! {
    todo!("0xf2f964 j___ZN3rbx7signals6signalIFvddEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::remove(rbx::signals::signal<void ()(double,double)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE")]
// 0xf2f974 — j___ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf2f974() -> ! {
    todo!("0xf2f974 j___ZN3rbx7signals6signalIFvddEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception")]
// 0xf2f994 — j___ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception
// type: 
pub fn stub_0xf2f994() -> ! {
    todo!("0xf2f994 j___ZN3rbx7signals6signalIFvddEE8on_errorERSt9exception")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv")]
// 0xf2fe54 — j___ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv
// type: 
pub fn stub_0xf2fe54() -> ! {
    todo!("0xf2fe54 j___ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m")]
// 0xf2fe64 — j___ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf2fe64() -> ! {
    todo!("0xf2fe64 j___ZN3RBX8LRUCacheISsSsE6insertERKSsS3_m")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::remove(std::string const&)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsSsE6removeERKSs")]
// 0xf2fe74 — j___ZN3RBX8LRUCacheISsSsE6removeERKSs
// type: 
pub fn stub_0xf2fe74() -> ! {
    todo!("0xf2fe74 j___ZN3RBX8LRUCacheISsSsE6removeERKSs")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsSsEC2Ev")]
// 0xf2fe84 — j___ZN3RBX8LRUCacheISsSsEC2Ev
// type: int __fastcall(_DWORD)
pub fn stub_0xf2fe84() -> ! {
    todo!("0xf2fe84 j___ZN3RBX8LRUCacheISsSsEC2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
#[doc(alias = "j___ZN3RBX8LRUCacheISsSsED2Ev")]
// 0xf2fe94 — j___ZN3RBX8LRUCacheISsSsED2Ev
// type: 
pub fn stub_0xf2fe94() -> ! {
    todo!("0xf2fe94 j___ZN3RBX8LRUCacheISsSsED2Ev")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")]
#[doc(alias = "j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_")]
// 0xf301d4 — j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf301d4() -> ! {
    todo!("0xf301d4 j___ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImSsEEE7destroyEPS3_")
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
#[doc(alias = "j___ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv")]
// 0xf302d4 — j___ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf302d4() -> ! {
    todo!("0xf302d4 j___ZNSt10_List_baseISt4pairISsS0_ImSsEESaIS2_EE8_M_clearEv")
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")]
#[doc(alias = "j___ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_")]
// 0xf302f4 — j___ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf302f4() -> ! {
    todo!("0xf302f4 j___ZNSt4listISt4pairISsS0_ImSsEESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")]
#[doc(alias = "j___ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_")]
// 0xf30304 — j___ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf30304() -> ! {
    todo!("0xf30304 j___ZNSt4pairISsS_ImSsEEC2ERKSsRKS0_")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int>(char const*,int const&)")]
#[doc(alias = "j___ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_")]
// 0xf30514 — j___ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf30514() -> ! {
    todo!("0xf30514 j___ZN3RBX5Stats4Item20createBoundChildItemIiEEPS1_PKcRKT_")
}

#[doc(alias = "RBX::FindHeaviest::operator()(RBX::SpanningNode *,RBX::SpanningEdge *)")]
#[doc(alias = "j___ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE")]
// 0xf30ac4 — j___ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE
// type: int __fastcall(int, int, RBX::SpanningEdge *this)
pub fn stub_0xf30ac4() -> ! {
    todo!("0xf30ac4 j___ZN3RBX12FindHeaviestclEPNS_12SpanningNodeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::SpanningNode::getDepth(RBX::SpanningNode*)")]
#[doc(alias = "j___ZN3RBX12SpanningNode8getDepthEPS0_")]
// 0xf30ad4 — j___ZN3RBX12SpanningNode8getDepthEPS0_
// type: _DWORD __fastcall(RBX::SpanningNode *__hidden this, RBX::SpanningNode *)
pub fn stub_0xf30ad4() -> ! {
    todo!("0xf30ad4 j___ZN3RBX12SpanningNode8getDepthEPS0_")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert_unique(RBX::SpanningNode * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")]
// 0xf30ae4 — j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf30ae4() -> ! {
    todo!("0xf30ae4 j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
// 0xf30af4 — j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf30af4() -> ! {
    todo!("0xf30af4 j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::SpanningNode * const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")]
// 0xf30b04 — j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf30b04() -> ! {
    todo!("0xf30b04 j___ZNSt8_Rb_treeIPN3RBX12SpanningNodeES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "RBX::StandardOut::StandardOut(void)")]
#[doc(alias = "j___ZN3RBX11StandardOutC2Ev")]
// 0xf30b14 — j___ZN3RBX11StandardOutC2Ev
// type: _DWORD __fastcall(RBX::StandardOut *__hidden this)
pub fn stub_0xf30b14() -> ! {
    todo!("0xf30b14 j___ZN3RBX11StandardOutC2Ev")
}

#[doc(alias = "RBX::StandardOut::~StandardOut()")]
#[doc(alias = "j___ZN3RBX11StandardOutD2Ev")]
// 0xf30b24 — j___ZN3RBX11StandardOutD2Ev
// type: void __fastcall(RBX::StandardOut *__hidden this)
pub fn stub_0xf30b24() -> ! {
    todo!("0xf30b24 j___ZN3RBX11StandardOutD2Ev")
}

#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")]
#[doc(alias = "j___ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc")]
// 0xf30b34 — j___ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xf30b34() -> ! {
    todo!("0xf30b34 j___ZN3RBX18StandardOutMessageC2ENS_11MessageTypeEPKc")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_")]
// 0xf30b44 — j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf30b44() -> ! {
    todo!("0xf30b44 j___ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX18StandardOutMessageEEEclES5_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv")]
// 0xf30b54 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf30b54() -> ! {
    todo!("0xf30b54 j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception")]
// 0xf30b74 — j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception
// type: 
pub fn stub_0xf30b74() -> ! {
    todo!("0xf30b74 j___ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv")]
// 0xf30bb4 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
pub fn stub_0xf30bb4() -> ! {
    todo!("0xf30bb4 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::insert(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE")]
// 0xf30bc4 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xf30bc4() -> ! {
    todo!("0xf30bc4 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::remove(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE")]
// 0xf30bd4 — j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0xf30bd4() -> ! {
    todo!("0xf30bd4 j___ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE")
}
