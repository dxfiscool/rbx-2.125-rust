//! core shard DX — 100 core stubs EA-sorted, next uncovered after DW 0x878330 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x878330.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,unsigned long,RBX::Voxel::WaterCellForce const&)")]
// 0x87836c — __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_87836c() -> ! {
    todo!("0x87836c __ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Voxel::CellOrientation * rbx::any_cast<RBX::Voxel::CellOrientation,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x8784fc — __ZN3rbx8any_castIN3RBX5Voxel15CellOrientationENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_8784fc() -> ! {
    todo!("0x8784fc __ZN3rbx8any_castIN3RBX5Voxel15CellOrientationENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellOrientation & rbx::any_cast<RBX::Voxel::CellOrientation &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x878554 — __ZN3rbx8any_castIRN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_878554() -> ! {
    todo!("0x878554 __ZN3rbx8any_castIRN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::resize(unsigned long,RBX::Voxel::CellOrientation)")]
// 0x878644 — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE6resizeEmS2_
pub fn stub_878644() -> ! {
    todo!("0x878644 __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::push_back(RBX::Voxel::CellOrientation const&)")]
// 0x878678 — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE9push_backERKS2_
pub fn stub_878678() -> ! {
    todo!("0x878678 __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellOrientation,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::operator[](RBX::Name const* const&)")]
// 0x8786a0 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel15CellOrientationESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_8786a0() -> ! {
    todo!("0x8786a0 __ZNSt3mapIPKN3RBX4NameENS0_5Voxel15CellOrientationESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
// 0x8786f8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_8786f8() -> ! {
    todo!("0x8786f8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
// 0x8787ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_8787ac() -> ! {
    todo!("0x8787ac __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation> const&)")]
// 0x878804 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_878804() -> ! {
    todo!("0x878804 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,RBX::Voxel::CellOrientation const&)")]
// 0x87886c — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_87886c() -> ! {
    todo!("0x87886c __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_allocate(unsigned long)")]
// 0x878950 — __ZNSt12_Vector_baseIN3RBX5Voxel15CellOrientationESaIS2_EE11_M_allocateEm
pub fn stub_878950() -> ! {
    todo!("0x878950 __ZNSt12_Vector_baseIN3RBX5Voxel15CellOrientationESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Voxel::CellOrientation * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *>(RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *)")]
// 0x878968 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel15CellOrientationES6_EET0_T_S8_S7_
pub fn stub_878968() -> ! {
    todo!("0x878968 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel15CellOrientationES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,unsigned long,RBX::Voxel::CellOrientation const&)")]
// 0x8789a4 — __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_8789a4() -> ! {
    todo!("0x8789a4 __ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Voxel::CellBlock * rbx::any_cast<RBX::Voxel::CellBlock,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x878b34 — __ZN3rbx8any_castIN3RBX5Voxel9CellBlockENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_878b34() -> ! {
    todo!("0x878b34 __ZN3rbx8any_castIN3RBX5Voxel9CellBlockENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellBlock & rbx::any_cast<RBX::Voxel::CellBlock &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x878b8c — __ZN3rbx8any_castIRN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_878b8c() -> ! {
    todo!("0x878b8c __ZN3rbx8any_castIRN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::resize(unsigned long,RBX::Voxel::CellBlock)")]
// 0x878c7c — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE6resizeEmS2_
pub fn stub_878c7c() -> ! {
    todo!("0x878c7c __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::push_back(RBX::Voxel::CellBlock const&)")]
// 0x878cb0 — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE9push_backERKS2_
pub fn stub_878cb0() -> ! {
    todo!("0x878cb0 __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellBlock,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::operator[](RBX::Name const* const&)")]
// 0x878cd8 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel9CellBlockESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_878cd8() -> ! {
    todo!("0x878cd8 __ZNSt3mapIPKN3RBX4NameENS0_5Voxel9CellBlockESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
// 0x878d30 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_878d30() -> ! {
    todo!("0x878d30 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
// 0x878de4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_878de4() -> ! {
    todo!("0x878de4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellBlock> const&)")]
// 0x878e3c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_878e3c() -> ! {
    todo!("0x878e3c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,RBX::Voxel::CellBlock const&)")]
// 0x878ea4 — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_878ea4() -> ! {
    todo!("0x878ea4 __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_allocate(unsigned long)")]
// 0x878f88 — __ZNSt12_Vector_baseIN3RBX5Voxel9CellBlockESaIS2_EE11_M_allocateEm
pub fn stub_878f88() -> ! {
    todo!("0x878f88 __ZNSt12_Vector_baseIN3RBX5Voxel9CellBlockESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Voxel::CellBlock * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *>(RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *)")]
// 0x878fa0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel9CellBlockES6_EET0_T_S8_S7_
pub fn stub_878fa0() -> ! {
    todo!("0x878fa0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel9CellBlockES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,unsigned long,RBX::Voxel::CellBlock const&)")]
// 0x878fdc — __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_878fdc() -> ! {
    todo!("0x878fdc __ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Voxel::CellMaterial * rbx::any_cast<RBX::Voxel::CellMaterial,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x87916c — __ZN3rbx8any_castIN3RBX5Voxel12CellMaterialENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_87916c() -> ! {
    todo!("0x87916c __ZN3rbx8any_castIN3RBX5Voxel12CellMaterialENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellMaterial & rbx::any_cast<RBX::Voxel::CellMaterial &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8791c4 — __ZN3rbx8any_castIRN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_8791c4() -> ! {
    todo!("0x8791c4 __ZN3rbx8any_castIRN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::resize(unsigned long,RBX::Voxel::CellMaterial)")]
// 0x8792b4 — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE6resizeEmS2_
pub fn stub_8792b4() -> ! {
    todo!("0x8792b4 __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::push_back(RBX::Voxel::CellMaterial const&)")]
// 0x8792e8 — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE9push_backERKS2_
pub fn stub_8792e8() -> ! {
    todo!("0x8792e8 __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellMaterial,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::operator[](RBX::Name const* const&)")]
// 0x879310 — __ZNSt3mapIPKN3RBX4NameENS0_5Voxel12CellMaterialESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_879310() -> ! {
    todo!("0x879310 __ZNSt3mapIPKN3RBX4NameENS0_5Voxel12CellMaterialESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
// 0x879368 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_879368() -> ! {
    todo!("0x879368 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
// 0x87941c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_87941c() -> ! {
    todo!("0x87941c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
// 0x879474 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_879474() -> ! {
    todo!("0x879474 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,RBX::Voxel::CellMaterial const&)")]
// 0x8794dc — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_8794dc() -> ! {
    todo!("0x8794dc __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_allocate(unsigned long)")]
// 0x8795c0 — __ZNSt12_Vector_baseIN3RBX5Voxel12CellMaterialESaIS2_EE11_M_allocateEm
pub fn stub_8795c0() -> ! {
    todo!("0x8795c0 __ZNSt12_Vector_baseIN3RBX5Voxel12CellMaterialESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Voxel::CellMaterial * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *>(RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *)")]
// 0x8795d8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel12CellMaterialES6_EET0_T_S8_S7_
pub fn stub_8795d8() -> ! {
    todo!("0x8795d8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel12CellMaterialES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,unsigned long,RBX::Voxel::CellMaterial const&)")]
// 0x879614 — __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_879614() -> ! {
    todo!("0x879614 __ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::TerrainPartition::~TerrainPartition()")]
// 0x8797a4 — __ZN3RBX16TerrainPartitionD2Ev
pub fn stub_8797a4() -> ! {
    todo!("0x8797a4 __ZN3RBX16TerrainPartitionD2Ev")
}

#[doc(alias = "std::vector<std::vector<bool,std::allocator<bool>>,std::allocator<std::vector<bool,std::allocator<bool>>>>::~vector()")]
// 0x8797d8 — __ZNSt6vectorIS_IbSaIbEESaIS1_EED2Ev
pub fn stub_8797d8() -> ! {
    todo!("0x8797d8 __ZNSt6vectorIS_IbSaIbEESaIS1_EED2Ev")
}

#[doc(alias = "std::vector<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::~vector()")]
// 0x879810 — __ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EED2Ev
pub fn stub_879810() -> ! {
    todo!("0x879810 __ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EED2Ev")
}

#[doc(alias = "RBX::CellContact::~CellContact()")]
// 0x87a47c — __ZN3RBX11CellContactD0Ev
pub fn stub_87a47c() -> ! {
    todo!("0x87a47c __ZN3RBX11CellContactD0Ev")
}

#[doc(alias = "RBX::CellContact::~CellContact()")]
// 0x87a51c — __ZN3RBX11CellContactD1Ev
pub fn stub_87a51c() -> ! {
    todo!("0x87a51c __ZN3RBX11CellContactD1Ev")
}

#[doc(alias = "RBX::CellContact::~CellContact()")]
// 0x87a520 — __ZN3RBX11CellContactD2Ev
pub fn stub_87a520() -> ! {
    todo!("0x87a520 __ZN3RBX11CellContactD2Ev")
}

#[doc(alias = "RBX::CellContact::deleteConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x87a650 — __ZN3RBX11CellContact16deleteConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_87a650() -> ! {
    todo!("0x87a650 __ZN3RBX11CellContact16deleteConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::CellContact::getConnector(int)")]
// 0x87a714 — __ZN3RBX11CellContact12getConnectorEi
pub fn stub_87a714() -> ! {
    todo!("0x87a714 __ZN3RBX11CellContact12getConnectorEi")
}

#[doc(alias = "RBX::CellContact::deleteAllConnectors(void)")]
// 0x87a71c — __ZN3RBX11CellContact19deleteAllConnectorsEv
pub fn stub_87a71c() -> ! {
    todo!("0x87a71c __ZN3RBX11CellContact19deleteAllConnectorsEv")
}

#[doc(alias = "RBX::CellContact::removeAllConnectorsFromKernel(void)")]
// 0x87a724 — __ZN3RBX11CellContact29removeAllConnectorsFromKernelEv
pub fn stub_87a724() -> ! {
    todo!("0x87a724 __ZN3RBX11CellContact29removeAllConnectorsFromKernelEv")
}

#[doc(alias = "RBX::CellContact::putAllConnectorsInKernel(void)")]
// 0x87a794 — __ZN3RBX11CellContact24putAllConnectorsInKernelEv
pub fn stub_87a794() -> ! {
    todo!("0x87a794 __ZN3RBX11CellContact24putAllConnectorsInKernelEv")
}

#[doc(alias = "RBX::CellContact::stepContact(void)")]
// 0x87a830 — __ZN3RBX11CellContact11stepContactEv
pub fn stub_87a830() -> ! {
    todo!("0x87a830 __ZN3RBX11CellContact11stepContactEv")
}

#[doc(alias = "RBX::CellContact::computeIsColliding(float)")]
// 0x87a86c — __ZN3RBX11CellContact18computeIsCollidingEf
pub fn stub_87a86c() -> ! {
    todo!("0x87a86c __ZN3RBX11CellContact18computeIsCollidingEf")
}

#[doc(alias = "RBX::CellContact::updateClosestFeatures(void)")]
// 0x87a8d4 — __ZN3RBX11CellContact21updateClosestFeaturesEv
pub fn stub_87a8d4() -> ! {
    todo!("0x87a8d4 __ZN3RBX11CellContact21updateClosestFeaturesEv")
}

#[doc(alias = "RBX::CellContact::worstFeatureOverlap(void)")]
// 0x87a914 — __ZN3RBX11CellContact19worstFeatureOverlapEv
pub fn stub_87a914() -> ! {
    todo!("0x87a914 __ZN3RBX11CellContact19worstFeatureOverlapEv")
}

#[doc(alias = "RBX::CellContact::matchClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x87a9c8 — __ZN3RBX11CellContact20matchClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_87a9c8() -> ! {
    todo!("0x87a9c8 __ZN3RBX11CellContact20matchClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::CellContact::updateContactPoints(void)")]
// 0x87aa20 — __ZN3RBX11CellContact19updateContactPointsEv
pub fn stub_87aa20() -> ! {
    todo!("0x87aa20 __ZN3RBX11CellContact19updateContactPointsEv")
}

#[doc(alias = "RBX::CellContact::matchClosestFeature(RBX::PolyConnector *)")]
// 0x87aa50 — __ZN3RBX11CellContact19matchClosestFeatureEPNS_13PolyConnectorE
pub fn stub_87aa50() -> ! {
    todo!("0x87aa50 __ZN3RBX11CellContact19matchClosestFeatureEPNS_13PolyConnectorE")
}

#[doc(alias = "RBX::Voxel::Grid * RBX::CellContact::getVoxelStore<RBX::Voxel::Grid>(void)")]
// 0x87aaa8 — __ZN3RBX11CellContact13getVoxelStoreINS_5Voxel4GridEEEPT_v
pub fn stub_87aaa8() -> ! {
    todo!("0x87aaa8 __ZN3RBX11CellContact13getVoxelStoreINS_5Voxel4GridEEEPT_v")
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::operator[](unsigned long)")]
// 0x87aac0 — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm
pub fn stub_87aac0() -> ! {
    todo!("0x87aac0 __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EEixEm")
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::replace(unsigned long,RBX::PolyConnector * const&)")]
// 0x87ab20 — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE7replaceEmRKS2_
pub fn stub_87ab20() -> ! {
    todo!("0x87ab20 __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE7replaceEmRKS2_")
}

#[doc(alias = "RBX::PolyConnector::match(RBX::PolyConnector*,RBX::PolyConnector*)")]
// 0x87abd8 — __ZN3RBX13PolyConnector5matchEPS0_S1_
pub fn stub_87abd8() -> ! {
    todo!("0x87abd8 __ZN3RBX13PolyConnector5matchEPS0_S1_")
}

#[doc(alias = "RBX::FixedArray<RBX::PolyConnector *,40ul>::fastRemove(unsigned long)")]
// 0x87ac14 — __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE10fastRemoveEm
pub fn stub_87ac14() -> ! {
    todo!("0x87ac14 __ZN3RBX10FixedArrayIPNS_13PolyConnectorELm40EE10fastRemoveEm")
}

#[doc(alias = "RBX::MegaClusterPoly::buildMesh(void)")]
// 0x87b2d8 — __ZN3RBX15MegaClusterPoly9buildMeshEv
pub fn stub_87b2d8() -> ! {
    todo!("0x87b2d8 __ZN3RBX15MegaClusterPoly9buildMeshEv")
}

#[doc(alias = "RBX::MegaClusterPoly::getSurfaceCoordInBody(unsigned long)const")]
// 0x87b474 — __ZNK3RBX15MegaClusterPoly21getSurfaceCoordInBodyEm
pub fn stub_87b474() -> ! {
    todo!("0x87b474 __ZNK3RBX15MegaClusterPoly21getSurfaceCoordInBodyEm")
}

#[doc(alias = "RBX::MegaClusterPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
// 0x87b480 — __ZNK3RBX15MegaClusterPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
pub fn stub_87b480() -> ! {
    todo!("0x87b480 __ZNK3RBX15MegaClusterPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")
}

#[doc(alias = "RBX::MegaClusterPoly::~MegaClusterPoly()")]
// 0x87fc58 — __ZN3RBX15MegaClusterPolyD1Ev
pub fn stub_87fc58() -> ! {
    todo!("0x87fc58 __ZN3RBX15MegaClusterPolyD1Ev")
}

#[doc(alias = "RBX::MegaClusterPoly::~MegaClusterPoly()")]
// 0x87fc7c — __ZN3RBX15MegaClusterPolyD0Ev
pub fn stub_87fc7c() -> ! {
    todo!("0x87fc7c __ZN3RBX15MegaClusterPolyD0Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator delete(void *)")]
// 0x88067c — __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEdlEPv
pub fn stub_88067c() -> ! {
    todo!("0x88067c __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::operator new(unsigned long)")]
// 0x880cb8 — __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEnwEm
pub fn stub_880cb8() -> ! {
    todo!("0x880cb8 __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::Allocator(void)")]
// 0x880e3c — __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEC2Ev
pub fn stub_880e3c() -> ! {
    todo!("0x880e3c __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::MegaClusterMesh>::releaseMemory(void)")]
// 0x880ea0 — __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEE13releaseMemoryEv
pub fn stub_880ea0() -> ! {
    todo!("0x880ea0 __ZN3RBX9AllocatorINS_4POLY15MegaClusterMeshEE13releaseMemoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0x880ebc — __ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_880ebc() -> ! {
    todo!("0x880ebc __ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0x880eec — __ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_880eec() -> ! {
    todo!("0x880eec __ZN5boost14singleton_poolIN3RBX4POLY15MegaClusterMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "RBX::PolyCellContact::~PolyCellContact()")]
// 0x881898 — __ZN3RBX15PolyCellContactD0Ev
pub fn stub_881898() -> ! {
    todo!("0x881898 __ZN3RBX15PolyCellContactD0Ev")
}

#[doc(alias = "RBX::PolyCellContact::~PolyCellContact()")]
// 0x88194c — __ZN3RBX15PolyCellContactD1Ev
pub fn stub_88194c() -> ! {
    todo!("0x88194c __ZN3RBX15PolyCellContactD1Ev")
}

#[doc(alias = "RBX::PolyCellContact::~PolyCellContact()")]
// 0x881950 — __ZN3RBX15PolyCellContactD2Ev
pub fn stub_881950() -> ! {
    todo!("0x881950 __ZN3RBX15PolyCellContactD2Ev")
}

#[doc(alias = "RBX::PolyCellContact::resetBestPair(RBX::PolyCellPair *)")]
// 0x881a94 — __ZN3RBX15PolyCellContact13resetBestPairEPNS_12PolyCellPairE
pub fn stub_881a94() -> ! {
    todo!("0x881a94 __ZN3RBX15PolyCellContact13resetBestPairEPNS_12PolyCellPairE")
}

#[doc(alias = "RBX::PolyCellContact::findClosestFeatures(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x881ac4 — __ZN3RBX15PolyCellContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_881ac4() -> ! {
    todo!("0x881ac4 __ZN3RBX15PolyCellContact19findClosestFeaturesERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::PolyCellContact::findBestPair(void)")]
// 0x881b38 — __ZN3RBX15PolyCellContact12findBestPairEv
pub fn stub_881b38() -> ! {
    todo!("0x881b38 __ZN3RBX15PolyCellContact12findBestPairEv")
}

#[doc(alias = "RBX::PolyCellContact::generateDataForMovingAssemblyStage(void)")]
// 0x881d50 — __ZN3RBX15PolyCellContact34generateDataForMovingAssemblyStageEv
pub fn stub_881d50() -> ! {
    todo!("0x881d50 __ZN3RBX15PolyCellContact34generateDataForMovingAssemblyStageEv")
}

#[doc(alias = "RBX::CellFaceFacePair::allocateClone(void)")]
// 0x881d58 — __ZN3RBX16CellFaceFacePair13allocateCloneEv
pub fn stub_881d58() -> ! {
    todo!("0x881d58 __ZN3RBX16CellFaceFacePair13allocateCloneEv")
}

#[doc(alias = "RBX::CellFaceFacePair::test(void)")]
// 0x881d98 — __ZN3RBX16CellFaceFacePair4testEv
pub fn stub_881d98() -> ! {
    todo!("0x881d98 __ZN3RBX16CellFaceFacePair4testEv")
}

#[doc(alias = "RBX::CellFaceFacePair::findOtherFace(RBX::POLY::Vertex const*)")]
// 0x88214c — __ZN3RBX16CellFaceFacePair13findOtherFaceEPKNS_4POLY6VertexE
pub fn stub_88214c() -> ! {
    todo!("0x88214c __ZN3RBX16CellFaceFacePair13findOtherFaceEPKNS_4POLY6VertexE")
}

#[doc(alias = "RBX::CellFaceFacePair::pairIsValid(void)")]
// 0x882330 — __ZN3RBX16CellFaceFacePair11pairIsValidEv
pub fn stub_882330() -> ! {
    todo!("0x882330 __ZN3RBX16CellFaceFacePair11pairIsValidEv")
}

#[doc(alias = "RBX::CellFaceFacePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x882510 — __ZN3RBX16CellFaceFacePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_882510() -> ! {
    todo!("0x882510 __ZN3RBX16CellFaceFacePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::CellFaceFacePair::vertexInside(RBX::Primitive *,RBX::Primitive *,RBX::POLY::Vertex const*,RBX::POLY::Face const*,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x882ec0 — __ZN3RBX16CellFaceFacePair12vertexInsideEPNS_9PrimitiveES2_PKNS_4POLY6VertexEPKNS3_4FaceERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_882ec0() -> ! {
    todo!("0x882ec0 __ZN3RBX16CellFaceFacePair12vertexInsideEPNS_9PrimitiveES2_PKNS_4POLY6VertexEPKNS3_4FaceERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::CellFaceFacePair::newFaceEdgeConnector(unsigned long,RBX::POLY::Vertex const*,RBX::POLY::Vertex const*)")]
// 0x8830ec — __ZN3RBX16CellFaceFacePair20newFaceEdgeConnectorEmPKNS_4POLY6VertexES4_
pub fn stub_8830ec() -> ! {
    todo!("0x8830ec __ZN3RBX16CellFaceFacePair20newFaceEdgeConnectorEmPKNS_4POLY6VertexES4_")
}

#[doc(alias = "RBX::CellEdgeEdgePair::allocateClone(void)")]
// 0x88339c — __ZN3RBX16CellEdgeEdgePair13allocateCloneEv
pub fn stub_88339c() -> ! {
    todo!("0x88339c __ZN3RBX16CellEdgeEdgePair13allocateCloneEv")
}

#[doc(alias = "RBX::CellEdgeEdgePair::test(void)")]
// 0x8833dc — __ZN3RBX16CellEdgeEdgePair4testEv
pub fn stub_8833dc() -> ! {
    todo!("0x8833dc __ZN3RBX16CellEdgeEdgePair4testEv")
}

#[doc(alias = "RBX::CellEdgeEdgePair::loadConnectors(RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
// 0x8838cc — __ZN3RBX16CellEdgeEdgePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
pub fn stub_8838cc() -> ! {
    todo!("0x8838cc __ZN3RBX16CellEdgeEdgePair14loadConnectorsERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")
}

#[doc(alias = "RBX::CellEdgeEdgePair::newEdgeEdgeConnector(void)")]
// 0x883940 — __ZN3RBX16CellEdgeEdgePair20newEdgeEdgeConnectorEv
pub fn stub_883940() -> ! {
    todo!("0x883940 __ZN3RBX16CellEdgeEdgePair20newEdgeEdgeConnectorEv")
}

#[doc(alias = "RBX::Allocator<RBX::PolyCellContact>::Allocator(void)")]
// 0x883b7c — __ZN3RBX9AllocatorINS_15PolyCellContactEEC2Ev
pub fn stub_883b7c() -> ! {
    todo!("0x883b7c __ZN3RBX9AllocatorINS_15PolyCellContactEEC2Ev")
}

#[doc(alias = "RBX::CellEdgeEdgePair::~CellEdgeEdgePair()")]
// 0x883be0 — __ZN3RBX16CellEdgeEdgePairD1Ev
pub fn stub_883be0() -> ! {
    todo!("0x883be0 __ZN3RBX16CellEdgeEdgePairD1Ev")
}

#[doc(alias = "RBX::CellFaceFacePair::~CellFaceFacePair()")]
// 0x883be4 — __ZN3RBX16CellFaceFacePairD1Ev
pub fn stub_883be4() -> ! {
    todo!("0x883be4 __ZN3RBX16CellFaceFacePairD1Ev")
}

#[doc(alias = "RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul>::operator[](unsigned long)")]
// 0x883f70 — __ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EEixEm
pub fn stub_883f70() -> ! {
    todo!("0x883f70 __ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EEixEm")
}

#[doc(alias = "RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul>::push_back(RBX::CellFaceFacePair::VertexStatus const&)")]
// 0x883fd0 — __ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EE9push_backERKS2_
pub fn stub_883fd0() -> ! {
    todo!("0x883fd0 __ZN3RBX10FixedArrayINS_16CellFaceFacePair12VertexStatusELm40EE9push_backERKS2_")
}

#[doc(alias = "RBX::Allocator<RBX::FaceVertexConnector>::operator new(unsigned long)")]
// 0x884100 — __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEnwEm
pub fn stub_884100() -> ! {
    todo!("0x884100 __ZN3RBX9AllocatorINS_19FaceVertexConnectorEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::FaceEdgeConnector>::operator new(unsigned long)")]
// 0x884170 — __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEnwEm
pub fn stub_884170() -> ! {
    todo!("0x884170 __ZN3RBX9AllocatorINS_17FaceEdgeConnectorEEnwEm")
}

#[doc(alias = "RBX::POLY::Face::getSidePlane(unsigned long)const")]
// 0x8841e0 — __ZNK3RBX4POLY4Face12getSidePlaneEm
pub fn stub_8841e0() -> ! {
    todo!("0x8841e0 __ZNK3RBX4POLY4Face12getSidePlaneEm")
}

#[doc(alias = "RBX::Allocator<RBX::EdgeEdgeConnector>::operator new(unsigned long)")]
// 0x884264 — __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEnwEm
pub fn stub_884264() -> ! {
    todo!("0x884264 __ZN3RBX9AllocatorINS_17EdgeEdgeConnectorEEnwEm")
}

#[doc(alias = "RBX::CellFaceFacePair::isFaceFace(void)const")]
// 0x8842d4 — __ZNK3RBX16CellFaceFacePair10isFaceFaceEv
pub fn stub_8842d4() -> ! {
    todo!("0x8842d4 __ZNK3RBX16CellFaceFacePair10isFaceFaceEv")
}

#[doc(alias = "RBX::CellFaceFacePair::~CellFaceFacePair()")]
// 0x8842d8 — __ZN3RBX16CellFaceFacePairD0Ev
pub fn stub_8842d8() -> ! {
    todo!("0x8842d8 __ZN3RBX16CellFaceFacePairD0Ev")
}
