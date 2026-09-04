// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) — next 120 not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x45b560..0x45fd90 | EA-sorted asc distinct, RBX::Instance|DataModel|Workspace only gap filler not yet in rbx_datamodel
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias
// Shard: A EA-sorted ascending next uncovered RBX after watchdog_W

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x45b560 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel16GearGenreSettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::GearGenreSetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9DataModel16GearGenreSettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub use crate::instance::stub_0x45b560 as stub_0x45b560;

// 0x45b5b8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub use crate::instance::stub_0x45b5b8 as stub_0x45b5b8;

// 0x45b66c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x45b66c() -> ! {
    todo!("0x45b66c")
}

// 0x45b6c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel16GearGenreSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub use crate::instance::stub_0x45b6c4 as stub_0x45b6c4;

// 0x45b72c — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::GearGenreSetting*,std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>>,RBX::DataModel::GearGenreSetting const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x45b72c() -> ! {
    todo!("0x45b72c")
}

// 0x45b810 — __ZNSt12_Vector_baseIN3RBX9DataModel16GearGenreSettingESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Vector_base<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9DataModel16GearGenreSettingESaIS2_EE11_M_allocateEm")]
pub use crate::instance::stub_0x45b810 as stub_0x45b810;

// 0x45b828 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel16GearGenreSettingES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::DataModel::GearGenreSetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *>(RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel16GearGenreSettingES6_EET0_T_S8_S7_")]
pub fn stub_0x45b828() -> ! {
    todo!("0x45b828")
}

// 0x45b864 — __ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::GearGenreSetting*,std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>>,unsigned long,RBX::DataModel::GearGenreSetting const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel16GearGenreSettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x45b864() -> ! {
    todo!("0x45b864")
}

// 0x45b9f4 — __ZN3rbx8any_castIN3RBX9DataModel5GenreENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::Genre * rbx::any_cast<RBX::DataModel::Genre,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX9DataModel5GenreENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0x45b9f4() -> ! {
    todo!("0x45b9f4")
}

// 0x45ba4c — __ZN3rbx8any_castIRN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::Genre & rbx::any_cast<RBX::DataModel::Genre &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9DataModel5GenreENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x45ba4c() -> ! {
    todo!("0x45ba4c")
}

// 0x45bb3c — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::resize(unsigned long,RBX::DataModel::Genre)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE6resizeEmS2_")]
pub fn stub_0x45bb3c() -> ! {
    todo!("0x45bb3c")
}

// 0x45bb70 — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::push_back(RBX::DataModel::Genre const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE9push_backERKS2_")]
pub fn stub_0x45bb70() -> ! {
    todo!("0x45bb70")
}

// 0x45bb98 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel5GenreESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::Genre,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9DataModel5GenreESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub use crate::instance::stub_0x45bb98 as stub_0x45bb98;

// 0x45bbf0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub use crate::instance::stub_0x45bbf0 as stub_0x45bbf0;

// 0x45bca4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x45bca4() -> ! {
    todo!("0x45bca4")
}

// 0x45bcfc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel5GenreEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub use crate::instance::stub_0x45bcfc as stub_0x45bcfc;

// 0x45bd64 — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,RBX::DataModel::Genre const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x45bd64() -> ! {
    todo!("0x45bd64")
}

// 0x45be48 — __ZNSt12_Vector_baseIN3RBX9DataModel5GenreESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9DataModel5GenreESaIS2_EE11_M_allocateEm")]
pub use crate::instance::stub_0x45be48 as stub_0x45be48;

// 0x45be60 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel5GenreES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::DataModel::Genre * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::Genre *,RBX::DataModel::Genre *>(RBX::DataModel::Genre *,RBX::DataModel::Genre *,RBX::DataModel::Genre *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel5GenreES6_EET0_T_S8_S7_")]
pub fn stub_0x45be60() -> ! {
    todo!("0x45be60")
}

// 0x45be9c — __ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,unsigned long,RBX::DataModel::Genre const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel5GenreESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x45be9c() -> ! {
    todo!("0x45be9c")
}

// 0x45c02c — __ZN3rbx8any_castIN3RBX9DataModel11CreatorTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::CreatorType * rbx::any_cast<RBX::DataModel::CreatorType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX9DataModel11CreatorTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0x45c02c() -> ! {
    todo!("0x45c02c")
}

// 0x45c084 — __ZN3rbx8any_castIRN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::DataModel::CreatorType & rbx::any_cast<RBX::DataModel::CreatorType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9DataModel11CreatorTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x45c084() -> ! {
    todo!("0x45c084")
}

// 0x45c174 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::resize(unsigned long,RBX::DataModel::CreatorType)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE6resizeEmS2_")]
pub fn stub_0x45c174() -> ! {
    todo!("0x45c174")
}

// 0x45c1a8 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::push_back(RBX::DataModel::CreatorType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE9push_backERKS2_")]
pub fn stub_0x45c1a8() -> ! {
    todo!("0x45c1a8")
}

// 0x45c1d0 — __ZNSt3mapIPKN3RBX4NameENS0_9DataModel11CreatorTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::CreatorType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9DataModel11CreatorTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub use crate::instance::stub_0x45c1d0 as stub_0x45c1d0;

// 0x45c228 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub use crate::instance::stub_0x45c228 as stub_0x45c228;

// 0x45c2dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0x45c2dc() -> ! {
    todo!("0x45c2dc")
}

// 0x45c334 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9DataModel11CreatorTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub use crate::instance::stub_0x45c334 as stub_0x45c334;

// 0x45c39c — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,RBX::DataModel::CreatorType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0x45c39c() -> ! {
    todo!("0x45c39c")
}

// 0x45c480 — __ZNSt12_Vector_baseIN3RBX9DataModel11CreatorTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Vector_base<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9DataModel11CreatorTypeESaIS2_EE11_M_allocateEm")]
pub use crate::instance::stub_0x45c480 as stub_0x45c480;

// 0x45c498 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel11CreatorTypeES6_EET0_T_S8_S7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::DataModel::CreatorType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *>(RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9DataModel11CreatorTypeES6_EET0_T_S8_S7_")]
pub fn stub_0x45c498() -> ! {
    todo!("0x45c498")
}

// 0x45c4d4 — __ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,unsigned long,RBX::DataModel::CreatorType const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9DataModel11CreatorTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0x45c4d4() -> ! {
    todo!("0x45c4d4")
}

// 0x45c664 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EEC2EMS2_FvSsSsEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::BoundFuncDesc(void (RBX::DataModel::*)(std::string,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EEC2EMS2_FvSsSsEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub use crate::instance::stub_0x45c664 as stub_0x45c664;

// 0x45c82c — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
pub fn stub_0x45c82c() -> ! {
    todo!("0x45c82c")
}

// 0x45c878 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EED0Ev")]
pub fn stub_0x45c878() -> ! {
    todo!("0x45c878")
}

// 0x45c94c — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFvSsSsELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x45c94c() -> ! {
    todo!("0x45c94c")
}

// 0x45cb18 — __ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FvSsSsESsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_
// type: int __fastcall(int, int, int, int, std::string *, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string,std::string),std::string,std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call2HelperINS_9DataModelEMS2_FvSsSsESsSsvE4callEPS2_S4_RNS0_7VariantERKSsSA_")]
pub fn stub_0x45cb18() -> ! {
    todo!("0x45cb18")
}

// 0x45ccdc — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::EventDesc(rbx::signal<void ()(bool)> RBX::DataModel::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub use crate::instance::stub_0x45ccdc as stub_0x45ccdc;

// 0x45ce60 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_0x45ce60() -> ! {
    todo!("0x45ce60")
}

// 0x45cf14 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<
pub fn stub_0x45cf14() -> ! {
    todo!("0x45cf14")
}

// 0x45d068 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x45d068() -> ! {
    todo!("0x45d068")
}

// 0x45d0f4 — __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvbEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x45d0f4() -> ! {
    todo!("0x45d0f4")
}

// 0x45d9ec — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::PropDescriptor<std::string (RBX::DataModel::*)(void)const,int>(char const*,char const*,std::string (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub use crate::instance::stub_0x45d9ec as stub_0x45d9ec;

// 0x45dafc — __ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9DataModelESsED0Ev")]
pub fn stub_0x45dafc() -> ! {
    todo!("0x45dafc")
}

// 0x45db28 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE10isReadOnlyEv")]
pub use crate::instance::stub_0x45db28 as stub_0x45db28;

// 0x45db2c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE11isWriteOnlyEv")]
pub use crate::instance::stub_0x45db2c as stub_0x45db2c;

// 0x45db30 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x45db30() -> ! {
    todo!("0x45db30")
}

// 0x45db58 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,std::string>::GetImpl<std::string (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelESsE7GetImplIMS2_KFSsvEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x45db58() -> ! {
    todo!("0x45db58")
}

// 0x45dc78 — __ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_0x45dc78() -> ! {
    todo!("0x45dc78")
}

// 0x45dd2c — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
// was: RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<
pub fn stub_0x45dd2c() -> ! {
    todo!("0x45dd2c")
}

// 0x45df30 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x45df30() -> ! {
    todo!("0x45df30")
}

// 0x45dfa4 — __ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DataModel,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::DataModel::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9DataModelEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x45dfa4() -> ! {
    todo!("0x45dfa4")
}

// 0x45e1f8 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::BoundFuncDesc(bool (RBX::DataModel::*)(RBX::DataModel::GearType),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EEC2EMS2_FbS3_EPKcS9_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub use crate::instance::stub_0x45e1f8 as stub_0x45e1f8;

// 0x45e370 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x45e370() -> ! {
    todo!("0x45e370")
}

// 0x45e3a0 — __ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EED0Ev")]
pub fn stub_0x45e3a0() -> ! {
    todo!("0x45e3a0")
}

// 0x45e474 — __ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,bool ()(RBX::DataModel::GearType),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9DataModelEFbNS2_8GearTypeEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x45e474() -> ! {
    todo!("0x45e474")
}

// 0x45e4b4 — __ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FbNS2_8GearTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::DataModel,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::DataModel::GearType,bool>::call(RBX::DataModel*,bool (RBX::DataModel::*)(RBX::DataModel::GearType),RBX::Reflection::Variant &,RBX::DataModel::GearType const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_9DataModelEMS2_FbNS2_8GearTypeEES3_bE4callEPS2_S5_RNS0_7VariantERKS3_")]
pub use crate::instance::stub_0x45e4b4 as stub_0x45e4b4;

// 0x45e4ec — __ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall ***__fastcall(int (__fastcall ***)(_DWORD), int))(_DWORD)
#[doc(alias = "RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disable_if<boost::is_same<RBX::DataModel::GearType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_9DataModel8GearTypeELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
// was: RBX::DataModel::GearType RBX::Reflection::ArgHelper::getArg<RBX::DataModel::GearType,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::DataModel::GearType> const&,boost::disab
pub fn stub_0x45e4ec() -> ! {
    todo!("0x45e4ec")
}

// 0x45e67c — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// type: int(void)
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearType>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_9DataModel8GearTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
// was: bool RBX::Reflection::ArgHelper::try_enum<1,RBX::DataModel::GearType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::DataModel::GearType &,boost::enable_if<boost::is_enum<RBX::DataModel::GearTy
pub fn stub_0x45e67c() -> ! {
    todo!("0x45e67c")
}

// 0x45e6d0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::EnumPropDescriptor<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub use crate::instance::stub_0x45e6d0 as stub_0x45e6d0;

// 0x45e87c — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEED0Ev")]
pub fn stub_0x45e87c() -> ! {
    todo!("0x45e87c")
}

// 0x45e8a8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10isReadOnlyEv")]
pub fn stub_0x45e8a8() -> ! {
    todo!("0x45e8a8")
}

// 0x45e8b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11isWriteOnlyEv")]
pub fn stub_0x45e8b8() -> ! {
    todo!("0x45e8b8")
}

// 0x45e8c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub use crate::instance::stub_0x45e8c8 as stub_0x45e8c8;

// 0x45e8f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x45e8f0() -> ! {
    todo!("0x45e8f0")
}

// 0x45e914 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x45e914() -> ! {
    todo!("0x45e914")
}

// 0x45ea60 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x45ea60() -> ! {
    todo!("0x45ea60")
}

// 0x45ea88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14hasStringValueEv")]
pub use crate::instance::stub_0x45ea88 as stub_0x45ea88;

// 0x45ea8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14getStringValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45ea8c as stub_0x45ea8c;

// 0x45eab0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub use crate::instance::stub_0x45eab0 as stub_0x45eab0;

// 0x45eaf0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x45eaf0() -> ! {
    todo!("0x45eaf0")
}

// 0x45eb10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x45eb10() -> ! {
    todo!("0x45eb10")
}

// 0x45ed50 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45ed50 as stub_0x45ed50;

// 0x45ed6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub use crate::instance::stub_0x45ed6c as stub_0x45ed6c;

// 0x45eda0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45eda0 as stub_0x45eda0;

// 0x45eda8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub use crate::instance::stub_0x45eda8 as stub_0x45eda8;

// 0x45edf4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45edf4 as stub_0x45edf4;

// 0x45ee14 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub use crate::instance::stub_0x45ee14 as stub_0x45ee14;

// 0x45ee4c — __ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting>::convertToIndex(RBX::DataModel::GearGenreSetting)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel16GearGenreSettingEE14convertToIndexES3_")]
pub use crate::instance::stub_0x45ee4c as stub_0x45ee4c;

// 0x45eebc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_16GearGenreSettingEE11setIntValueEPNS0_13DescribedBaseEi")]
pub use crate::instance::stub_0x45eebc as stub_0x45eebc;

// 0x45ef00 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv")]
pub use crate::instance::stub_0x45ef00 as stub_0x45ef00;

// 0x45ef04 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv")]
pub use crate::instance::stub_0x45ef04 as stub_0x45ef04;

// 0x45ef08 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45ef08 as stub_0x45ef08;

// 0x45ef28 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::GearGenreSetting>::GetImpl<RBX::DataModel::GearGenreSetting (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::GearGenreSetting const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_16GearGenreSettingEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x45ef28() -> ! {
    todo!("0x45ef28")
}

// 0x45f048 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::EnumPropDescriptor<RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::Genre (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub use crate::instance::stub_0x45f048 as stub_0x45f048;

// 0x45f1f4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEED0Ev")]
pub fn stub_0x45f1f4() -> ! {
    todo!("0x45f1f4")
}

// 0x45f220 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10isReadOnlyEv")]
pub fn stub_0x45f220() -> ! {
    todo!("0x45f220")
}

// 0x45f230 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11isWriteOnlyEv")]
pub fn stub_0x45f230() -> ! {
    todo!("0x45f230")
}

// 0x45f240 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub use crate::instance::stub_0x45f240 as stub_0x45f240;

// 0x45f268 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x45f268() -> ! {
    todo!("0x45f268")
}

// 0x45f28c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x45f28c() -> ! {
    todo!("0x45f28c")
}

// 0x45f3d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x45f3d8() -> ! {
    todo!("0x45f3d8")
}

// 0x45f3fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14hasStringValueEv")]
pub use crate::instance::stub_0x45f3fc as stub_0x45f3fc;

// 0x45f400 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14getStringValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45f400 as stub_0x45f400;

// 0x45f424 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub use crate::instance::stub_0x45f424 as stub_0x45f424;

// 0x45f464 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x45f464() -> ! {
    todo!("0x45f464")
}

// 0x45f484 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x45f484() -> ! {
    todo!("0x45f484")
}

// 0x45f6c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45f6c4 as stub_0x45f6c4;

// 0x45f6e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE13setIndexValueEPNS0_13DescribedBaseEm")]
pub use crate::instance::stub_0x45f6e0 as stub_0x45f6e0;

// 0x45f714 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12getEnumValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45f714 as stub_0x45f714;

// 0x45f71c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE12setEnumValueEPNS0_13DescribedBaseEi")]
pub use crate::instance::stub_0x45f71c as stub_0x45f71c;

// 0x45f768 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11getEnumItemEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45f768 as stub_0x45f768;

// 0x45f788 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
pub use crate::instance::stub_0x45f788 as stub_0x45f788;

// 0x45f7bc — __ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DataModel::Genre>::convertToIndex(RBX::DataModel::Genre)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9DataModel5GenreEE14convertToIndexES3_")]
pub use crate::instance::stub_0x45f7bc as stub_0x45f7bc;

// 0x45f82c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_5GenreEE11setIntValueEPNS0_13DescribedBaseEi")]
pub use crate::instance::stub_0x45f82c as stub_0x45f82c;

// 0x45f86c — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv")]
pub use crate::instance::stub_0x45f86c as stub_0x45f86c;

// 0x45f870 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv")]
pub use crate::instance::stub_0x45f870 as stub_0x45f870;

// 0x45f874 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45f874 as stub_0x45f874;

// 0x45f894 — __ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DataModel,RBX::DataModel::Genre>::GetImpl<RBX::DataModel::Genre (RBX::DataModel::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::DataModel::Genre const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9DataModelENS2_5GenreEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_")]
pub fn stub_0x45f894() -> ! {
    todo!("0x45f894")
}

// 0x45f9b4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::EnumPropDescriptor<RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int>(char const*,char const*,RBX::DataModel::CreatorType (RBX::DataModel::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub use crate::instance::stub_0x45f9b4 as stub_0x45f9b4;

// 0x45fb60 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEED0Ev")]
pub fn stub_0x45fb60() -> ! {
    todo!("0x45fb60")
}

// 0x45fb8c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10isReadOnlyEv")]
pub fn stub_0x45fb8c() -> ! {
    todo!("0x45fb8c")
}

// 0x45fb9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11isWriteOnlyEv")]
pub fn stub_0x45fb9c() -> ! {
    todo!("0x45fb9c")
}

// 0x45fbac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub use crate::instance::stub_0x45fbac as stub_0x45fbac;

// 0x45fbd4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x45fbd4() -> ! {
    todo!("0x45fbd4")
}

// 0x45fbf8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x45fbf8() -> ! {
    todo!("0x45fbf8")
}

// 0x45fd44 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x45fd44() -> ! {
    todo!("0x45fd44")
}

// 0x45fd68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14hasStringValueEv")]
pub use crate::instance::stub_0x45fd68 as stub_0x45fd68;

// 0x45fd6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub use crate::instance::stub_0x45fd6c as stub_0x45fd6c;

// 0x45fd90 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DataModel,RBX::DataModel::CreatorType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9DataModelENS2_11CreatorTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub use crate::instance::stub_0x45fd90 as stub_0x45fd90;

