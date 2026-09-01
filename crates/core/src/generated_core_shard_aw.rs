//! core shard AW — 100 core stubs EA-sorted, next uncovered after AV 0x3cb538..0x3d0014 (strict RBX|boost|std earliest gap, after AV 0x3c057c..0x3cb538).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x3cb538.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::pop_back(void)")]
// 0x3da7cc — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE8pop_backEv
pub fn stub_0x3da7cc() -> ! {
    todo!("0x3da7cc __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE8pop_backEv")
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::push_back(RBX::ChangeHistoryService::Item * const&)")]
// 0x3da7fc — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE9push_backERKS3_
pub fn stub_0x3da7fc() -> ! {
    todo!("0x3da7fc __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_push_back_aux(RBX::ChangeHistoryService::Item * const&)")]
// 0x3da81c — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE16_M_push_back_auxERKS3_
pub fn stub_0x3da81c() -> ! {
    todo!("0x3da81c __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE16_M_push_back_auxERKS3_")
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reserve_map_at_back(unsigned long)")]
// 0x3da854 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE22_M_reserve_map_at_backEm
pub fn stub_0x3da854() -> ! {
    todo!("0x3da854 __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reallocate_map(unsigned long,bool)")]
// 0x3da870 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_reallocate_mapEmb
pub fn stub_0x3da870() -> ! {
    todo!("0x3da870 __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_allocate_map(unsigned long)")]
// 0x3da948 — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_allocate_mapEm
pub fn stub_0x3da948() -> ! {
    todo!("0x3da948 __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::deque(std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>> const&)")]
// 0x3da960 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EEC2ERKS5_
pub fn stub_0x3da960() -> ! {
    todo!("0x3da960 __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EEC2ERKS5_")
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::~_Deque_base()")]
// 0x3da9f4 — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EED2Ev
pub fn stub_0x3da9f4() -> ! {
    todo!("0x3da9f4 __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EED2Ev")
}

#[doc(alias = "std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>>(std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>)")]
// 0x3daa20 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN3RBX20ChangeHistoryService4ItemERKS7_PS8_ES3_IS7_RS7_PS7_EEET0_T_SG_SF_
pub fn stub_0x3daa20() -> ! {
    todo!("0x3daa20 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN3RBX20ChangeHistoryService4ItemERKS7_PS8_ES3_IS7_RS7_PS7_EEET0_T_SG_SF_")
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_initialize_map(unsigned long)")]
// 0x3daabc — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_initialize_mapEm
pub fn stub_0x3daabc() -> ! {
    todo!("0x3daabc __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_create_nodes(RBX::ChangeHistoryService::Item ***,RBX::ChangeHistoryService::Item ***)")]
// 0x3dac14 — __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_create_nodesEPPS3_S7_
pub fn stub_0x3dac14() -> ! {
    todo!("0x3dac14 __ZNSt11_Deque_baseIPN3RBX20ChangeHistoryService4ItemESaIS3_EE15_M_create_nodesEPPS3_S7_")
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
// 0x3dae68 — __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE8iteratorC2ERKS4_
pub fn stub_0x3dae68() -> ! {
    todo!("0x3dae68 __ZN3RBX5Voxel6RegionINS0_4Grid5ChunkEE8iteratorC2ERKS4_")
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::resize(unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior)")]
// 0x3dbbd0 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE6resizeEmS2_
pub fn stub_0x3dbbd0() -> ! {
    todo!("0x3dbbd0 __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::push_back(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0x3dbc08 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE9push_backERKS2_
pub fn stub_0x3dbc08() -> ! {
    todo!("0x3dbc08 __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::ChangeHistoryService::RuntimeUndoBehavior,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::operator[](RBX::Name const* const&)")]
// 0x3dbc34 — __ZNSt3mapIPKN3RBX4NameENS0_20ChangeHistoryService19RuntimeUndoBehaviorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0x3dbc34() -> ! {
    todo!("0x3dbc34 __ZNSt3mapIPKN3RBX4NameENS0_20ChangeHistoryService19RuntimeUndoBehaviorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// 0x3dbc8c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0x3dbc8c() -> ! {
    todo!("0x3dbc8c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// 0x3dbd40 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0x3dbd40() -> ! {
    todo!("0x3dbd40 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
// 0x3dbd98 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x3dbd98() -> ! {
    todo!("0x3dbd98 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0x3dbe04 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x3dbe04() -> ! {
    todo!("0x3dbe04 __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_allocate(unsigned long)")]
// 0x3dbee8 — __ZNSt12_Vector_baseIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE11_M_allocateEm
pub fn stub_0x3dbee8() -> ! {
    todo!("0x3dbee8 __ZNSt12_Vector_baseIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::ChangeHistoryService::RuntimeUndoBehavior * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *>(RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *)")]
// 0x3dbf00 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ChangeHistoryService19RuntimeUndoBehaviorES6_EET0_T_S8_S7_
pub fn stub_0x3dbf00() -> ! {
    todo!("0x3dbf00 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX20ChangeHistoryService19RuntimeUndoBehaviorES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0x3dbf40 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0x3dbf40() -> ! {
    todo!("0x3dbf40 __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayChange(void)")]
// 0x3dc330 — __ZN3RBX20ChangeHistoryService4Item12unplayChangeEv
pub fn stub_0x3dc330() -> ! {
    todo!("0x3dc330 __ZN3RBX20ChangeHistoryService4Item12unplayChangeEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayClusterChange(void)")]
// 0x3dc698 — __ZN3RBX20ChangeHistoryService4Item19unplayClusterChangeEv
pub fn stub_0x3dc698() -> ! {
    todo!("0x3dc698 __ZN3RBX20ChangeHistoryService4Item19unplayClusterChangeEv")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::vector(std::vector<unsigned int,std::allocator<unsigned int>> const&)")]
// 0x3dc7f4 — __ZNSt6vectorIjSaIjEEC2ERKS1_
pub fn stub_0x3dc7f4() -> ! {
    todo!("0x3dc7f4 __ZNSt6vectorIjSaIjEEC2ERKS1_")
}

#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_Vector_base(unsigned long,std::allocator<unsigned int> const&)")]
// 0x3dc82c — __ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_
pub fn stub_0x3dc82c() -> ! {
    todo!("0x3dc82c __ZNSt12_Vector_baseIjSaIjEEC2EmRKS0_")
}

#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_M_allocate(unsigned long)")]
// 0x3dc85c — __ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm
pub fn stub_0x3dc85c() -> ! {
    todo!("0x3dc85c __ZNSt12_Vector_baseIjSaIjEE11_M_allocateEm")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordClusterDataGetChunk(int)")]
// 0x3dcf44 — __ZN3RBX20ChangeHistoryService4Item25recordClusterDataGetChunkEi
pub fn stub_0x3dcf44() -> ! {
    todo!("0x3dcf44 __ZN3RBX20ChangeHistoryService4Item25recordClusterDataGetChunkEi")
}

#[doc(alias = "std::map<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::operator[](unsigned int const&)")]
// 0x3dd084 — __ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_
pub fn stub_0x3dd084() -> ! {
    todo!("0x3dd084 __ZNSt3mapIjSt6vectorIjSaIjEESt4lessIjESaISt4pairIKjS2_EEEixERS6_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0x3dd1a4 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_0x3dd1a4() -> ! {
    todo!("0x3dd1a4 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0x3dd258 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_0x3dd258() -> ! {
    todo!("0x3dd258 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0x3dd2a4 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_0x3dd2a4() -> ! {
    todo!("0x3dd2a4 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_create_node(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0x3dd30c — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_0x3dd30c() -> ! {
    todo!("0x3dd30c __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::find(unsigned int const&)")]
// 0x3dd3f0 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_
pub fn stub_0x3dd3f0() -> ! {
    todo!("0x3dd3f0 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE4findERS1_")
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::operator==(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)const")]
// 0x3dd430 — __ZNK3RBX5Voxel6RegionINS0_4Grid5ChunkEEeqERKS4_
pub fn stub_0x3dd430() -> ! {
    todo!("0x3dd430 __ZNK3RBX5Voxel6RegionINS0_4Grid5ChunkEEeqERKS4_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// 0x3dd730 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x3dd730() -> ! {
    todo!("0x3dd730 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// 0x3dd758 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x3dd758() -> ! {
    todo!("0x3dd758 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_create_node(RBX::ChangeHistoryService::Item const&)")]
// 0x3dd774 — __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_0x3dd774() -> ! {
    todo!("0x3dd774 __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_Rb_tree(std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>> const&)")]
// 0x3dd900 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_
pub fn stub_0x3dd900() -> ! {
    todo!("0x3dd900 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> const*,std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>*)")]
// 0x3dd944 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
pub fn stub_0x3dd944() -> ! {
    todo!("0x3dd944 __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_")
}

#[doc(alias = "RBX::ChangeHistoryService::getWaypointDataSize(void)const")]
// 0x3ddce4 — __ZNK3RBX20ChangeHistoryService19getWaypointDataSizeEv
pub fn stub_0x3ddce4() -> ! {
    todo!("0x3ddce4 __ZNK3RBX20ChangeHistoryService19getWaypointDataSizeEv")
}

#[doc(alias = "RBX::ChangeHistoryService::getWaypointCount(void)const")]
// 0x3ddcec — __ZNK3RBX20ChangeHistoryService16getWaypointCountEv
pub fn stub_0x3ddcec() -> ! {
    todo!("0x3ddcec __ZNK3RBX20ChangeHistoryService16getWaypointCountEv")
}

#[doc(alias = "RBX::Stats::Item::Item(void)")]
// 0x3dded0 — __ZN3RBX5Stats4ItemC2Ev
pub fn stub_0x3dded0() -> ! {
    todo!("0x3dded0 __ZN3RBX5Stats4ItemC2Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// 0x3de020 — __ZN3RBX5Stats14TypedStatsItemIiED1Ev
pub fn stub_0x3de020() -> ! {
    todo!("0x3de020 __ZN3RBX5Stats14TypedStatsItemIiED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// 0x3de168 — __ZN3RBX5Stats14TypedStatsItemIiED0Ev
pub fn stub_0x3de168() -> ! {
    todo!("0x3de168 __ZN3RBX5Stats14TypedStatsItemIiED0Ev")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::computeDataSize(void)const")]
// 0x3deab0 — __ZNK3RBX20ChangeHistoryService4Item15computeDataSizeEv
pub fn stub_0x3deab0() -> ! {
    todo!("0x3deab0 __ZNK3RBX20ChangeHistoryService4Item15computeDataSizeEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::absorb(RBX::ChangeHistoryService::Item const&)")]
// 0x3deba8 — __ZN3RBX20ChangeHistoryService4Item6absorbERKS1_
pub fn stub_0x3deba8() -> ! {
    todo!("0x3deba8 __ZN3RBX20ChangeHistoryService4Item6absorbERKS1_")
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::addItem(RBX::ChangeHistoryService::Item const&)")]
// 0x3ded00 — __ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE
pub fn stub_0x3ded00() -> ! {
    todo!("0x3ded00 __ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::absorbClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0x3dee10 — __ZN3RBX20ChangeHistoryService4Item17absorbClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
pub fn stub_0x3dee10() -> ! {
    todo!("0x3dee10 __ZN3RBX20ChangeHistoryService4Item17absorbClusterDataERKSt4pairIjSt6vectorIjSaIjEEE")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::push_back(unsigned int const&)")]
// 0x3defec — __ZNSt6vectorIjSaIjEE9push_backERKj
pub fn stub_0x3defec() -> ! {
    todo!("0x3defec __ZNSt6vectorIjSaIjEE9push_backERKj")
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,unsigned int const&)")]
// 0x3df014 — __ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj
pub fn stub_0x3df014() -> ! {
    todo!("0x3df014 __ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj")
}

#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_erase(std::_List_iterator<RBX::ChangeHistoryService::Item>)")]
// 0x3df2d8 — __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
pub fn stub_0x3df2d8() -> ! {
    todo!("0x3df2d8 __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E")
}

#[doc(alias = "std::_List_base<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_clear(void)")]
// 0x3df3fc — __ZNSt10_List_baseIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_clearEv
pub fn stub_0x3df3fc() -> ! {
    todo!("0x3df3fc __ZNSt10_List_baseIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_clearEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::onSetWaypoint(void)")]
// 0x3df55c — __ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv
pub fn stub_0x3df55c() -> ! {
    todo!("0x3df55c __ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv")
}

#[doc(alias = "void std::vector<unsigned int,std::allocator<unsigned int>>::_M_assign_aux<__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>>(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,std::forward_iterator_tag)")]
// 0x3df6fc — __ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag
pub fn stub_0x3df6fc() -> ! {
    todo!("0x3df6fc __ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::playClusterChange(void)")]
// 0x3df7c4 — __ZN3RBX20ChangeHistoryService4Item17playClusterChangeEv
pub fn stub_0x3df7c4() -> ! {
    todo!("0x3df7c4 __ZN3RBX20ChangeHistoryService4Item17playClusterChangeEv")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::applyClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// 0x3df7fc — __ZN3RBX20ChangeHistoryService4Item16applyClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
pub fn stub_0x3df7fc() -> ! {
    todo!("0x3df7fc __ZN3RBX20ChangeHistoryService4Item16applyClusterDataERKSt4pairIjSt6vectorIjSaIjEEE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>> *)")]
// 0x3df920 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x3df920() -> ! {
    todo!("0x3df920 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::Shirt::setTemplate(RBX::TextureId)")]
// 0x3e0048 — __ZN3RBX5Shirt11setTemplateENS_9TextureIdE
pub fn stub_0x3e0048() -> ! {
    todo!("0x3e0048 __ZN3RBX5Shirt11setTemplateENS_9TextureIdE")
}

#[doc(alias = "RBX::Pants::setTemplate(RBX::TextureId)")]
// 0x3e0068 — __ZN3RBX5Pants11setTemplateENS_9TextureIdE
pub fn stub_0x3e0068() -> ! {
    todo!("0x3e0068 __ZN3RBX5Pants11setTemplateENS_9TextureIdE")
}

#[doc(alias = "RBX::ShirtGraphic::ShirtGraphic(void)")]
// 0x3e0088 — __ZN3RBX12ShirtGraphicC2Ev
pub fn stub_0x3e0088() -> ! {
    todo!("0x3e0088 __ZN3RBX12ShirtGraphicC2Ev")
}

#[doc(alias = "RBX::Clothing::Clothing(void)")]
// 0x3e0320 — __ZN3RBX8ClothingC2Ev
pub fn stub_0x3e0320() -> ! {
    todo!("0x3e0320 __ZN3RBX8ClothingC2Ev")
}

#[doc(alias = "RBX::Shirt::Shirt(void)")]
// 0x3e0614 — __ZN3RBX5ShirtC2Ev
pub fn stub_0x3e0614() -> ! {
    todo!("0x3e0614 __ZN3RBX5ShirtC2Ev")
}

#[doc(alias = "RBX::Pants::Pants(void)")]
// 0x3e0798 — __ZN3RBX5PantsC2Ev
pub fn stub_0x3e0798() -> ! {
    todo!("0x3e0798 __ZN3RBX5PantsC2Ev")
}

#[doc(alias = "RBX::ShirtGraphic::applyByMyself(RBX::Humanoid *)")]
// 0x3e091c — __ZN3RBX12ShirtGraphic13applyByMyselfEPNS_8HumanoidE
pub fn stub_0x3e091c() -> ! {
    todo!("0x3e091c __ZN3RBX12ShirtGraphic13applyByMyselfEPNS_8HumanoidE")
}

#[doc(alias = "RBX::Clothing::applyByMyself(RBX::Humanoid *)")]
// 0x3e0a58 — __ZN3RBX8Clothing13applyByMyselfEPNS_8HumanoidE
pub fn stub_0x3e0a58() -> ! {
    todo!("0x3e0a58 __ZN3RBX8Clothing13applyByMyselfEPNS_8HumanoidE")
}

#[doc(alias = "RBX::Skin::Skin(void)")]
// 0x3e0aac — __ZN3RBX4SkinC2Ev
pub fn stub_0x3e0aac() -> ! {
    todo!("0x3e0aac __ZN3RBX4SkinC2Ev")
}

#[doc(alias = "RBX::Skin::applyByMyself(RBX::Humanoid *)")]
// 0x3e0d20 — __ZN3RBX4Skin13applyByMyselfEPNS_8HumanoidE
pub fn stub_0x3e0d20() -> ! {
    todo!("0x3e0d20 __ZN3RBX4Skin13applyByMyselfEPNS_8HumanoidE")
}

#[doc(alias = "RBX::BodyColors::BodyColors(void)")]
// 0x3e0d9c — __ZN3RBX10BodyColorsC2Ev
pub fn stub_0x3e0d9c() -> ! {
    todo!("0x3e0d9c __ZN3RBX10BodyColorsC2Ev")
}

#[doc(alias = "RBX::BodyColors::applyByMyself(RBX::Humanoid *)")]
// 0x3e1028 — __ZN3RBX10BodyColors13applyByMyselfEPNS_8HumanoidE
pub fn stub_0x3e1028() -> ! {
    todo!("0x3e1028 __ZN3RBX10BodyColors13applyByMyselfEPNS_8HumanoidE")
}

#[doc(alias = "RBX::LegacyCharacterAppearance::apply(void)")]
// 0x3e10b0 — __ZN3RBX25LegacyCharacterAppearance5applyEv
pub fn stub_0x3e10b0() -> ! {
    todo!("0x3e10b0 __ZN3RBX25LegacyCharacterAppearance5applyEv")
}

#[doc(alias = "RBX::CharacterAppearance::apply(void)")]
// 0x3e10cc — __ZN3RBX19CharacterAppearance5applyEv
pub fn stub_0x3e10cc() -> ! {
    todo!("0x3e10cc __ZN3RBX19CharacterAppearance5applyEv")
}

#[doc(alias = "RBX::CharacterAppearance::onAncestorChanged(RBX::AncestorChanged const&)")]
// 0x3e10f0 — __ZN3RBX19CharacterAppearance17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_0x3e10f0() -> ! {
    todo!("0x3e10f0 __ZN3RBX19CharacterAppearance17onAncestorChangedERKNS_15AncestorChangedE")
}

#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e122c — __ZN3RBX12ShirtGraphicD1Ev
pub fn stub_0x3e122c() -> ! {
    todo!("0x3e122c __ZN3RBX12ShirtGraphicD1Ev")
}

#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e126c — __ZN3RBX12ShirtGraphicD0Ev
pub fn stub_0x3e126c() -> ! {
    todo!("0x3e126c __ZN3RBX12ShirtGraphicD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e1354 — __ZThn32_N3RBX12ShirtGraphicD1Ev
// was: ``non-virtual thunk to'RBX::ShirtGraphic::~ShirtGraphic()`
pub fn stub_0x3e1354() -> ! {
    todo!("0x3e1354 __ZThn32_N3RBX12ShirtGraphicD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e1394 — __ZThn32_N3RBX12ShirtGraphicD0Ev
// was: ``non-virtual thunk to'RBX::ShirtGraphic::~ShirtGraphic()`
pub fn stub_0x3e1394() -> ! {
    todo!("0x3e1394 __ZThn32_N3RBX12ShirtGraphicD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e1480 — __ZThn36_N3RBX12ShirtGraphicD1Ev
// was: ``non-virtual thunk to'RBX::ShirtGraphic::~ShirtGraphic()`
pub fn stub_0x3e1480() -> ! {
    todo!("0x3e1480 __ZThn36_N3RBX12ShirtGraphicD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e14c0 — __ZThn36_N3RBX12ShirtGraphicD0Ev
// was: ``non-virtual thunk to'RBX::ShirtGraphic::~ShirtGraphic()`
pub fn stub_0x3e14c0() -> ! {
    todo!("0x3e14c0 __ZThn36_N3RBX12ShirtGraphicD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e159c — __ZThn92_N3RBX12ShirtGraphicD1Ev
// was: ``non-virtual thunk to'RBX::ShirtGraphic::~ShirtGraphic()`
pub fn stub_0x3e159c() -> ! {
    todo!("0x3e159c __ZThn92_N3RBX12ShirtGraphicD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")]
// 0x3e15dc — __ZThn92_N3RBX12ShirtGraphicD0Ev
// was: ``non-virtual thunk to'RBX::ShirtGraphic::~ShirtGraphic()`
pub fn stub_0x3e15dc() -> ! {
    todo!("0x3e15dc __ZThn92_N3RBX12ShirtGraphicD0Ev")
}

#[doc(alias = "RBX::Clothing::~Clothing()")]
// 0x3e16b8 — __ZN3RBX8ClothingD1Ev
pub fn stub_0x3e16b8() -> ! {
    todo!("0x3e16b8 __ZN3RBX8ClothingD1Ev")
}

#[doc(alias = "RBX::Clothing::~Clothing()")]
// 0x3e1700 — __ZN3RBX8ClothingD0Ev
pub fn stub_0x3e1700() -> ! {
    todo!("0x3e1700 __ZN3RBX8ClothingD0Ev")
}

#[doc(alias = "RBX::Clothing::getTemplate(void)const")]
// 0x3e1808 — __ZNK3RBX8Clothing11getTemplateEv
pub fn stub_0x3e1808() -> ! {
    todo!("0x3e1808 __ZNK3RBX8Clothing11getTemplateEv")
}

#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing()")]
// 0x3e1864 — __ZThn32_N3RBX8ClothingD1Ev
// was: ``non-virtual thunk to'RBX::Clothing::~Clothing()`
pub fn stub_0x3e1864() -> ! {
    todo!("0x3e1864 __ZThn32_N3RBX8ClothingD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing()")]
// 0x3e18b0 — __ZThn32_N3RBX8ClothingD0Ev
// was: ``non-virtual thunk to'RBX::Clothing::~Clothing()`
pub fn stub_0x3e18b0() -> ! {
    todo!("0x3e18b0 __ZThn32_N3RBX8ClothingD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing()")]
// 0x3e19bc — __ZThn36_N3RBX8ClothingD1Ev
// was: ``non-virtual thunk to'RBX::Clothing::~Clothing()`
pub fn stub_0x3e19bc() -> ! {
    todo!("0x3e19bc __ZThn36_N3RBX8ClothingD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing()")]
// 0x3e1a08 — __ZThn36_N3RBX8ClothingD0Ev
// was: ``non-virtual thunk to'RBX::Clothing::~Clothing()`
pub fn stub_0x3e1a08() -> ! {
    todo!("0x3e1a08 __ZThn36_N3RBX8ClothingD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing()")]
// 0x3e1aec — __ZThn92_N3RBX8ClothingD1Ev
// was: ``non-virtual thunk to'RBX::Clothing::~Clothing()`
pub fn stub_0x3e1aec() -> ! {
    todo!("0x3e1aec __ZThn92_N3RBX8ClothingD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Clothing::~Clothing()")]
// 0x3e1b38 — __ZThn92_N3RBX8ClothingD0Ev
// was: ``non-virtual thunk to'RBX::Clothing::~Clothing()`
pub fn stub_0x3e1b38() -> ! {
    todo!("0x3e1b38 __ZThn92_N3RBX8ClothingD0Ev")
}

#[doc(alias = "RBX::Skin::~Skin()")]
// 0x3e1c1c — __ZN3RBX4SkinD1Ev
pub fn stub_0x3e1c1c() -> ! {
    todo!("0x3e1c1c __ZN3RBX4SkinD1Ev")
}

#[doc(alias = "RBX::Skin::~Skin()")]
// 0x3e1c20 — __ZN3RBX4SkinD0Ev
pub fn stub_0x3e1c20() -> ! {
    todo!("0x3e1c20 __ZN3RBX4SkinD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin()")]
// 0x3e1cd0 — __ZThn32_N3RBX4SkinD1Ev
// was: ``non-virtual thunk to'RBX::Skin::~Skin()`
pub fn stub_0x3e1cd0() -> ! {
    todo!("0x3e1cd0 __ZThn32_N3RBX4SkinD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin()")]
// 0x3e1cd8 — __ZThn32_N3RBX4SkinD0Ev
// was: ``non-virtual thunk to'RBX::Skin::~Skin()`
pub fn stub_0x3e1cd8() -> ! {
    todo!("0x3e1cd8 __ZThn32_N3RBX4SkinD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin()")]
// 0x3e1d8c — __ZThn36_N3RBX4SkinD1Ev
// was: ``non-virtual thunk to'RBX::Skin::~Skin()`
pub fn stub_0x3e1d8c() -> ! {
    todo!("0x3e1d8c __ZThn36_N3RBX4SkinD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin()")]
// 0x3e1d94 — __ZThn36_N3RBX4SkinD0Ev
// was: ``non-virtual thunk to'RBX::Skin::~Skin()`
pub fn stub_0x3e1d94() -> ! {
    todo!("0x3e1d94 __ZThn36_N3RBX4SkinD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin()")]
// 0x3e1e38 — __ZThn92_N3RBX4SkinD1Ev
// was: ``non-virtual thunk to'RBX::Skin::~Skin()`
pub fn stub_0x3e1e38() -> ! {
    todo!("0x3e1e38 __ZThn92_N3RBX4SkinD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Skin::~Skin()")]
// 0x3e1e40 — __ZThn92_N3RBX4SkinD0Ev
// was: ``non-virtual thunk to'RBX::Skin::~Skin()`
pub fn stub_0x3e1e40() -> ! {
    todo!("0x3e1e40 __ZThn92_N3RBX4SkinD0Ev")
}

#[doc(alias = "RBX::BodyColors::~BodyColors()")]
// 0x3e1ee4 — __ZN3RBX10BodyColorsD1Ev
pub fn stub_0x3e1ee4() -> ! {
    todo!("0x3e1ee4 __ZN3RBX10BodyColorsD1Ev")
}

#[doc(alias = "RBX::BodyColors::~BodyColors()")]
// 0x3e1ee8 — __ZN3RBX10BodyColorsD0Ev
pub fn stub_0x3e1ee8() -> ! {
    todo!("0x3e1ee8 __ZN3RBX10BodyColorsD0Ev")
}