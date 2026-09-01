//! core shard GJ — 100 core stubs EA-sorted, 0xf4ec04..0xf4f3b4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf4ebf4).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf4ebf4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::map<RBX::Name const*,RBX::LegacyController::InputType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::operator[](RBX::Name const* const&)")]
// 0xf4ec04 — j___ZNSt3mapIPKN3RBX4NameENS0_16LegacyController9InputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f4ec04() -> ! {
    todo!("0xf4ec04 j___ZNSt3mapIPKN3RBX4NameENS0_16LegacyController9InputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LegacyController::InputType*,std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>>,RBX::LegacyController::InputType const&)")]
// 0xf4ec14 — j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4ec14() -> ! {
    todo!("0xf4ec14 j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::LegacyController::InputType*,std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>>,unsigned long,RBX::LegacyController::InputType const&)")]
// 0xf4ec24 — j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f4ec24() -> ! {
    todo!("0xf4ec24 j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::resize(unsigned long,RBX::LegacyController::InputType)")]
// 0xf4ec34 — j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE6resizeEmS2_
pub fn stub_f4ec34() -> ! {
    todo!("0xf4ec34 j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::push_back(RBX::LegacyController::InputType const&)")]
// 0xf4ec44 — j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE9push_backERKS2_
pub fn stub_f4ec44() -> ! {
    todo!("0xf4ec44 j___ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
// 0xf4ec54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f4ec54() -> ! {
    todo!("0xf4ec54 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
// 0xf4ec64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f4ec64() -> ! {
    todo!("0xf4ec64 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
// 0xf4ec74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f4ec74() -> ! {
    todo!("0xf4ec74 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::Allocator(void)")]
// 0xf4ece4 — j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev
pub fn stub_f4ece4() -> ! {
    todo!("0xf4ece4 j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator delete(void *)")]
// 0xf4ecf4 — j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv
pub fn stub_f4ecf4() -> ! {
    todo!("0xf4ecf4 j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::CornerWedgeMesh>::operator new(unsigned long)")]
// 0xf4ed04 — j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm
pub fn stub_f4ed04() -> ! {
    todo!("0xf4ed04 j___ZN3RBX9AllocatorINS_4POLY15CornerWedgeMeshEEnwEm")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4ed34 — j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4ed34() -> ! {
    todo!("0xf4ed34 j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4ed44 — j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4ed44() -> ! {
    todo!("0xf4ed44 j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EdgeBuffer,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::EdgeBuffer*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EdgeBuffer,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::EdgeBuffer*>,boost::arg<1>>>,RBX::Primitive *)")]
// 0xf4ee04 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_10EdgeBufferEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_
pub fn stub_f4ee04() -> ! {
    todo!("0xf4ee04 j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_10EdgeBufferEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,RBX::Edge *>,std::_Select1st<std::pair<RBX::Assembly * const,RBX::Edge *>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,RBX::Edge *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,RBX::Edge *>> *)")]
// 0xf4ee14 — j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_PNS0_4EdgeEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f4ee14() -> ! {
    todo!("0xf4ee14 j___ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_PNS0_4EdgeEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::Allocator(void)")]
// 0xf4ee24 — j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEC2Ev
pub fn stub_f4ee24() -> ! {
    todo!("0xf4ee24 j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::NormalBreakConnector>::operator new(unsigned long)")]
// 0xf4ee34 — j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEnwEm
pub fn stub_f4ee34() -> ! {
    todo!("0xf4ee34 j___ZN3RBX9AllocatorINS_20NormalBreakConnectorEEnwEm")
}

#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
// 0xf4ee54 — j___ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv
pub fn stub_f4ee54() -> ! {
    todo!("0xf4ee54 j___ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE14release_memoryEv")
}

#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
// 0xf4ee64 — j___ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
pub fn stub_f4ee64() -> ! {
    todo!("0xf4ee64 j___ZN5boost14singleton_poolIN3RBX20NormalBreakConnectorELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")
}

#[doc(alias = "RBX::Face::size(void)const")]
// 0xf4ee74 — j___ZNK3RBX4Face4sizeEv
pub fn stub_f4ee74() -> ! {
    todo!("0xf4ee74 j___ZNK3RBX4Face4sizeEv")
}

#[doc(alias = "RBX::AnchorJoint::AnchorJoint(RBX::Primitive *)")]
// 0xf4ee84 — j___ZN3RBX11AnchorJointC2EPNS_9PrimitiveE
pub fn stub_f4ee84() -> ! {
    todo!("0xf4ee84 j___ZN3RBX11AnchorJointC2EPNS_9PrimitiveE")
}

#[doc(alias = "RBX::FreeJoint::FreeJoint(RBX::Primitive *)")]
// 0xf4ee94 — j___ZN3RBX9FreeJointC2EPNS_9PrimitiveE
pub fn stub_f4ee94() -> ! {
    todo!("0xf4ee94 j___ZN3RBX9FreeJointC2EPNS_9PrimitiveE")
}

#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::equal_range(RBX::IMoving * const&)")]
// 0xf4eea4 — j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
pub fn stub_f4eea4() -> ! {
    todo!("0xf4eea4 j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_insert_unique(RBX::IMoving * const&)")]
// 0xf4eeb4 — j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f4eeb4() -> ! {
    todo!("0xf4eeb4 j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::erase(RBX::IMoving * const&)")]
// 0xf4eec4 — j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_f4eec4() -> ! {
    todo!("0xf4eec4 j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::erase(std::_Rb_tree_iterator<RBX::IMoving *>,std::_Rb_tree_iterator<RBX::IMoving *>)")]
// 0xf4eed4 — j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_f4eed4() -> ! {
    todo!("0xf4eed4 j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")
}

#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_erase(std::_Rb_tree_node<RBX::IMoving *> *)")]
// 0xf4eee4 — j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f4eee4() -> ! {
    todo!("0xf4eee4 j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<RBX::IMoving *,RBX::IMoving *,std::_Identity<RBX::IMoving *>,std::less<RBX::IMoving *>,std::allocator<RBX::IMoving *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::IMoving * const&)")]
// 0xf4eef4 — j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f4eef4() -> ! {
    todo!("0xf4eef4 j___ZNSt8_Rb_treeIPN3RBX7IMovingES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "RBX::IPipelined::findWorld(void)")]
// 0xf4ef04 — j___ZN3RBX10IPipelined9findWorldEv
pub fn stub_f4ef04() -> ! {
    todo!("0xf4ef04 j___ZN3RBX10IPipelined9findWorldEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PyramidMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4ef24 — j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4ef24() -> ! {
    todo!("0xf4ef24 j___ZN5boost14singleton_poolIN3RBX4POLY11PyramidMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4ef34 — j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4ef34() -> ! {
    todo!("0xf4ef34 j___ZN5boost14singleton_poolIN3RBX4POLY15CornerWedgeMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4ef44 — j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4ef44() -> ! {
    todo!("0xf4ef44 j___ZN5boost14singleton_poolIN3RBX4POLY16ParallelRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::RightAngleRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4ef54 — j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4ef54() -> ! {
    todo!("0xf4ef54 j___ZN5boost14singleton_poolIN3RBX4POLY18RightAngleRampMeshELj48ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PrismMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4ef64 — j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4ef64() -> ! {
    todo!("0xf4ef64 j___ZN5boost14singleton_poolIN3RBX4POLY9PrismMeshELj56ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf4ef74 — j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f4ef74() -> ! {
    todo!("0xf4ef74 j___ZN5boost14singleton_poolIN3RBX4POLY9WedgeMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "std::_Vector_base<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_allocate(unsigned long)")]
// 0xf4ef84 — j___ZNSt12_Vector_baseIN3RBX5Joint9JointTypeESaIS2_EE11_M_allocateEm
pub fn stub_f4ef84() -> ! {
    todo!("0xf4ef84 j___ZNSt12_Vector_baseIN3RBX5Joint9JointTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Joint::JointType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Joint::JointType *,RBX::Joint::JointType *>(RBX::Joint::JointType *,RBX::Joint::JointType *,RBX::Joint::JointType *)")]
// 0xf4ef94 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Joint9JointTypeES6_EET0_T_S8_S7_
pub fn stub_f4ef94() -> ! {
    todo!("0xf4ef94 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Joint9JointTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Joint::JointType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::operator[](RBX::Name const* const&)")]
// 0xf4efa4 — j___ZNSt3mapIPKN3RBX4NameENS0_5Joint9JointTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f4efa4() -> ! {
    todo!("0xf4efa4 j___ZNSt3mapIPKN3RBX4NameENS0_5Joint9JointTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Joint::JointType*,std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>>,RBX::Joint::JointType const&)")]
// 0xf4efb4 — j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4efb4() -> ! {
    todo!("0xf4efb4 j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Joint::JointType*,std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>>,unsigned long,RBX::Joint::JointType const&)")]
// 0xf4efc4 — j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f4efc4() -> ! {
    todo!("0xf4efc4 j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::resize(unsigned long,RBX::Joint::JointType)")]
// 0xf4efd4 — j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE6resizeEmS2_
pub fn stub_f4efd4() -> ! {
    todo!("0xf4efd4 j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Joint::JointType,std::allocator<RBX::Joint::JointType>>::push_back(RBX::Joint::JointType const&)")]
// 0xf4efe4 — j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE9push_backERKS2_
pub fn stub_f4efe4() -> ! {
    todo!("0xf4efe4 j___ZNSt6vectorIN3RBX5Joint9JointTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
// 0xf4eff4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f4eff4() -> ! {
    todo!("0xf4eff4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
// 0xf4f004 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f4f004() -> ! {
    todo!("0xf4f004 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Joint::JointType> const&)")]
// 0xf4f014 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f4f014() -> ! {
    todo!("0xf4f014 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "RBX::BiMultiMap<RBX::Primitive *,RBX::Joint *>::removePair(RBX::Primitive * const&,RBX::Joint * const&)")]
// 0xf4f024 — j___ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE10removePairERKS2_RKS4_
pub fn stub_f4f024() -> ! {
    todo!("0xf4f024 j___ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE10removePairERKS2_RKS4_")
}

#[doc(alias = "RBX::BiMultiMap<RBX::Primitive *,RBX::Joint *>::pairInMap(RBX::Primitive * const&,RBX::Joint * const&)")]
// 0xf4f034 — j___ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE9pairInMapERKS2_RKS4_
pub fn stub_f4f034() -> ! {
    todo!("0xf4f034 j___ZN3RBX10BiMultiMapIPNS_9PrimitiveEPNS_5JointEE9pairInMapERKS2_RKS4_")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::JointStage *>,boost::arg<1>,boost::arg<2>,boost::reference_wrapper<std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>>::operator()<boost::_mfi::mf3<void,RBX::JointStage,RBX::Primitive *,RBX::Joint *,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>&>,boost::_bi::list2<RBX::Primitive * const&,RBX::Joint * const&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::JointStage,RBX::Primitive *,RBX::Joint *,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>&> const&,boost::_bi::list2<RBX::Primitive * const&,RBX::Joint * const&> &,int)const")]
// 0xf4f044 — j___ZNK5boost3_bi5list4INS0_5valueIPN3RBX10JointStageEEENS_3argILi1EEENS7_ILi2EEENS_17reference_wrapperISt6vectorIPNS3_5JointESaISD_EEEEEclINS_4_mfi3mf3IvS4_PNS3_9PrimitiveESD_RSF_EENS0_5list2IRKSM_RKSD_EEEEvNS0_4typeIvEERKT_RT0_i
pub fn stub_f4f044() -> ! {
    todo!("0xf4f044 j___ZNK5boost3_bi5list4INS0_5valueIPN3RBX10JointStageEEENS_3argILi1EEENS7_ILi2EEENS_17reference_wrapperISt6vectorIPNS3_5JointESaISD_EEEEEclINS_4_mfi3mf3IvS4_PNS3_9PrimitiveESD_RSF_EENS0_5list2IRKSM_RKSD_EEEEvNS0_4typeIvEERKT_RT0_i")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::equal_range(RBX::Primitive * const&)")]
// 0xf4f054 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
pub fn stub_f4f054() -> ! {
    todo!("0xf4f054 j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::erase(RBX::Primitive * const&)")]
// 0xf4f064 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
pub fn stub_f4f064() -> ! {
    todo!("0xf4f064 j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::erase(std::_Rb_tree_iterator<RBX::Primitive *>,std::_Rb_tree_iterator<RBX::Primitive *>)")]
// 0xf4f074 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
pub fn stub_f4f074() -> ! {
    todo!("0xf4f074 j___ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_insert_equal(std::pair<RBX::Primitive * const,RBX::Joint *> const&)")]
// 0xf4f084 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE15_M_insert_equalERKS7_
pub fn stub_f4f084() -> ! {
    todo!("0xf4f084 j___ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE15_M_insert_equalERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Primitive * const,RBX::Joint *>> *)")]
// 0xf4f094 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f4f094() -> ! {
    todo!("0xf4f094 j___ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,std::pair<RBX::Primitive * const,RBX::Joint *>,std::_Select1st<std::pair<RBX::Primitive * const,RBX::Joint *>>,std::less<RBX::Primitive *>,std::allocator<std::pair<RBX::Primitive * const,RBX::Joint *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Primitive * const,RBX::Joint *> const&)")]
// 0xf4f0a4 — j___ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_f4f0a4() -> ! {
    todo!("0xf4f0a4 j___ZNSt8_Rb_treeIPN3RBX9PrimitiveESt4pairIKS2_PNS0_5JointEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "std::_Vector_base<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_allocate(unsigned long)")]
// 0xf4f0c4 — j___ZNSt12_Vector_baseIN3RBX11SurfaceTypeESaIS1_EE11_M_allocateEm
pub fn stub_f4f0c4() -> ! {
    todo!("0xf4f0c4 j___ZNSt12_Vector_baseIN3RBX11SurfaceTypeESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SurfaceType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SurfaceType *,RBX::SurfaceType *>(RBX::SurfaceType *,RBX::SurfaceType *,RBX::SurfaceType *)")]
// 0xf4f0d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SurfaceTypeES5_EET0_T_S7_S6_
pub fn stub_f4f0d4() -> ! {
    todo!("0xf4f0d4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SurfaceTypeES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SurfaceType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::operator[](RBX::Name const* const&)")]
// 0xf4f0e4 — j___ZNSt3mapIPKN3RBX4NameENS0_11SurfaceTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
pub fn stub_f4f0e4() -> ! {
    todo!("0xf4f0e4 j___ZNSt3mapIPKN3RBX4NameENS0_11SurfaceTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,RBX::SurfaceType const&)")]
// 0xf4f0f4 — j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_f4f0f4() -> ! {
    todo!("0xf4f0f4 j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SurfaceType*,std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>>,unsigned long,RBX::SurfaceType const&)")]
// 0xf4f104 — j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_f4f104() -> ! {
    todo!("0xf4f104 j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::resize(unsigned long,RBX::SurfaceType)")]
// 0xf4f114 — j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE6resizeEmS1_
pub fn stub_f4f114() -> ! {
    todo!("0xf4f114 j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::SurfaceType,std::allocator<RBX::SurfaceType>>::push_back(RBX::SurfaceType const&)")]
// 0xf4f124 — j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE9push_backERKS1_
pub fn stub_f4f124() -> ! {
    todo!("0xf4f124 j___ZNSt6vectorIN3RBX11SurfaceTypeESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
// 0xf4f134 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_f4f134() -> ! {
    todo!("0xf4f134 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
// 0xf4f144 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
pub fn stub_f4f144() -> ! {
    todo!("0xf4f144 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SurfaceType> const&)")]
// 0xf4f154 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
pub fn stub_f4f154() -> ! {
    todo!("0xf4f154 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "void RBX::IndexedTree::visitMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AssemblyStage,RBX::Assembly*>,boost::_bi::list2<boost::_bi::value<RBX::AssemblyStage*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AssemblyStage,RBX::Assembly*>,boost::_bi::list2<boost::_bi::value<RBX::AssemblyStage*>,boost::arg<1>>>)")]
// 0xf4f164 — j___ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13AssemblyStageEPS2_EENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEvT0_
pub fn stub_f4f164() -> ! {
    todo!("0xf4f164 j___ZN3RBX11IndexedTree18visitMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_13AssemblyStageEPS2_EENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEvT0_")
}

#[doc(alias = "RBX::POLY::Edge::addFace(RBX::POLY::Face const*)")]
// 0xf4f174 — j___ZN3RBX4POLY4Edge7addFaceEPKNS0_4FaceE
pub fn stub_f4f174() -> ! {
    todo!("0xf4f174 j___ZN3RBX4POLY4Edge7addFaceEPKNS0_4FaceE")
}

#[doc(alias = "RBX::POLY::Face::operator=(RBX::POLY::Face const&)")]
// 0xf4f184 — j___ZN3RBX4POLY4FaceaSERKS1_
pub fn stub_f4f184() -> ! {
    todo!("0xf4f184 j___ZN3RBX4POLY4FaceaSERKS1_")
}

#[doc(alias = "RBX::POLY::Vertex::addEdge(RBX::POLY::Edge *)")]
// 0xf4f194 — j___ZN3RBX4POLY6Vertex7addEdgeEPNS0_4EdgeE
pub fn stub_f4f194() -> ! {
    todo!("0xf4f194 j___ZN3RBX4POLY6Vertex7addEdgeEPNS0_4EdgeE")
}

#[doc(alias = "__gnu_cxx::new_allocator<RBX::POLY::Face>::construct(RBX::POLY::Face*,RBX::POLY::Face const&)")]
// 0xf4f1a4 — j___ZN9__gnu_cxx13new_allocatorIN3RBX4POLY4FaceEE9constructEPS3_RKS3_
pub fn stub_f4f1a4() -> ! {
    todo!("0xf4f1a4 j___ZN9__gnu_cxx13new_allocatorIN3RBX4POLY4FaceEE9constructEPS3_RKS3_")
}

#[doc(alias = "RBX::POLY::Edge::getVertexFace(RBX::POLY::Vertex const*)const")]
// 0xf4f1b4 — j___ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE
pub fn stub_f4f1b4() -> ! {
    todo!("0xf4f1b4 j___ZNK3RBX4POLY4Edge13getVertexFaceEPKNS0_6VertexE")
}

#[doc(alias = "RBX::POLY::Edge::otherFace(RBX::POLY::Face const*)const")]
// 0xf4f1c4 — j___ZNK3RBX4POLY4Edge9otherFaceEPKNS0_4FaceE
pub fn stub_f4f1c4() -> ! {
    todo!("0xf4f1c4 j___ZNK3RBX4POLY4Edge9otherFaceEPKNS0_4FaceE")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_allocate(unsigned long)")]
// 0xf4f1e4 — j___ZNSt12_Vector_baseIN3RBX4POLY4EdgeESaIS2_EE11_M_allocateEm
pub fn stub_f4f1e4() -> ! {
    todo!("0xf4f1e4 j___ZNSt12_Vector_baseIN3RBX4POLY4EdgeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_allocate(unsigned long)")]
// 0xf4f1f4 — j___ZNSt12_Vector_baseIN3RBX4POLY4FaceESaIS2_EE11_M_allocateEm
pub fn stub_f4f1f4() -> ! {
    todo!("0xf4f1f4 j___ZNSt12_Vector_baseIN3RBX4POLY4FaceESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_allocate(unsigned long)")]
// 0xf4f204 — j___ZNSt12_Vector_baseIN3RBX4POLY6VertexESaIS2_EE11_M_allocateEm
pub fn stub_f4f204() -> ! {
    todo!("0xf4f204 j___ZNSt12_Vector_baseIN3RBX4POLY6VertexESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_M_allocate(unsigned long)")]
// 0xf4f214 — j___ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EE11_M_allocateEm
pub fn stub_f4f214() -> ! {
    todo!("0xf4f214 j___ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_Vector_base(unsigned long,std::allocator<RBX::POLY::Edge *> const&)")]
// 0xf4f224 — j___ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EEC2EmRKS4_
pub fn stub_f4f224() -> ! {
    todo!("0xf4f224 j___ZNSt12_Vector_baseIPN3RBX4POLY4EdgeESaIS3_EEC2EmRKS4_")
}

#[doc(alias = "RBX::POLY::Edge * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Edge *,RBX::POLY::Edge *>(RBX::POLY::Edge *,RBX::POLY::Edge *,RBX::POLY::Edge *)")]
// 0xf4f234 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4EdgeES6_EET0_T_S8_S7_
pub fn stub_f4f234() -> ! {
    todo!("0xf4f234 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4EdgeES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::POLY::Face * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Face *,RBX::POLY::Face *>(RBX::POLY::Face *,RBX::POLY::Face *,RBX::POLY::Face *)")]
// 0xf4f244 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4FaceES6_EET0_T_S8_S7_
pub fn stub_f4f244() -> ! {
    todo!("0xf4f244 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY4FaceES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::POLY::Vertex * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::POLY::Vertex *,RBX::POLY::Vertex *>(RBX::POLY::Vertex *,RBX::POLY::Vertex *,RBX::POLY::Vertex *)")]
// 0xf4f254 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY6VertexES6_EET0_T_S8_S7_
pub fn stub_f4f254() -> ! {
    todo!("0xf4f254 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX4POLY6VertexES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Edge*,std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>>,RBX::POLY::Edge const&)")]
// 0xf4f264 — j___ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4f264() -> ! {
    todo!("0xf4f264 j___ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::reserve(unsigned long)")]
// 0xf4f274 — j___ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE7reserveEm
pub fn stub_f4f274() -> ! {
    todo!("0xf4f274 j___ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::POLY::Edge,std::allocator<RBX::POLY::Edge>>::push_back(RBX::POLY::Edge const&)")]
// 0xf4f284 — j___ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE9push_backERKS2_
pub fn stub_f4f284() -> ! {
    todo!("0xf4f284 j___ZNSt6vectorIN3RBX4POLY4EdgeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Face*,std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>>,RBX::POLY::Face const&)")]
// 0xf4f294 — j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4f294() -> ! {
    todo!("0xf4f294 j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_erase_at_end(RBX::POLY::Face*)")]
// 0xf4f2a4 — j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_f4f2a4() -> ! {
    todo!("0xf4f2a4 j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "RBX::POLY::Face* std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::_M_allocate_and_copy<RBX::POLY::Face*>(unsigned long,RBX::POLY::Face*,RBX::POLY::Face*)")]
// 0xf4f2b4 — j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_
pub fn stub_f4f2b4() -> ! {
    todo!("0xf4f2b4 j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::reserve(unsigned long)")]
// 0xf4f2c4 — j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE7reserveEm
pub fn stub_f4f2c4() -> ! {
    todo!("0xf4f2c4 j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::POLY::Face,std::allocator<RBX::POLY::Face>>::push_back(RBX::POLY::Face const&)")]
// 0xf4f2d4 — j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE9push_backERKS2_
pub fn stub_f4f2d4() -> ! {
    todo!("0xf4f2d4 j___ZNSt6vectorIN3RBX4POLY4FaceESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Vertex*,std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>>,RBX::POLY::Vertex const&)")]
// 0xf4f2e4 — j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f4f2e4() -> ! {
    todo!("0xf4f2e4 j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_erase_at_end(RBX::POLY::Vertex*)")]
// 0xf4f2f4 — j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_f4f2f4() -> ! {
    todo!("0xf4f2f4 j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "RBX::POLY::Vertex* std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::_M_allocate_and_copy<RBX::POLY::Vertex*>(unsigned long,RBX::POLY::Vertex*,RBX::POLY::Vertex*)")]
// 0xf4f304 — j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_
pub fn stub_f4f304() -> ! {
    todo!("0xf4f304 j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE20_M_allocate_and_copyIPS2_EES6_mT_S7_")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::reserve(unsigned long)")]
// 0xf4f314 — j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE7reserveEm
pub fn stub_f4f314() -> ! {
    todo!("0xf4f314 j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE7reserveEm")
}

#[doc(alias = "std::vector<RBX::POLY::Vertex,std::allocator<RBX::POLY::Vertex>>::push_back(RBX::POLY::Vertex const&)")]
// 0xf4f324 — j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE9push_backERKS2_
pub fn stub_f4f324() -> ! {
    todo!("0xf4f324 j___ZNSt6vectorIN3RBX4POLY6VertexESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge * const&)")]
// 0xf4f334 — j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_f4f334() -> ! {
    todo!("0xf4f334 j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::push_back(RBX::POLY::Edge * const&)")]
// 0xf4f344 — j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE9push_backERKS3_
pub fn stub_f4f344() -> ! {
    todo!("0xf4f344 j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::vector(std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> const&)")]
// 0xf4f354 — j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEC2ERKS5_
pub fn stub_f4f354() -> ! {
    todo!("0xf4f354 j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEC2ERKS5_")
}

#[doc(alias = "std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>::operator=(std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>> const&)")]
// 0xf4f364 — j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEaSERKS5_
pub fn stub_f4f364() -> ! {
    todo!("0xf4f364 j___ZNSt6vectorIPN3RBX4POLY4EdgeESaIS3_EEaSERKS5_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge *>(__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,__gnu_cxx::__normal_iterator<RBX::POLY::Edge **,std::vector<RBX::POLY::Edge *,std::allocator<RBX::POLY::Edge *>>>,RBX::POLY::Edge * const&,std::random_access_iterator_tag)")]
// 0xf4f374 — j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX4POLY4EdgeESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag
pub fn stub_f4f374() -> ! {
    todo!("0xf4f374 j___ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX4POLY4EdgeESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_St26random_access_iterator_tag")
}

#[doc(alias = "RBX::Allocator<RBX::D6Link>::Allocator(void)")]
// 0xf4f394 — j___ZN3RBX9AllocatorINS_6D6LinkEEC2Ev
pub fn stub_f4f394() -> ! {
    todo!("0xf4f394 j___ZN3RBX9AllocatorINS_6D6LinkEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::D6Link>::operator delete(void *)")]
// 0xf4f3a4 — j___ZN3RBX9AllocatorINS_6D6LinkEEdlEPv
pub fn stub_f4f3a4() -> ! {
    todo!("0xf4f3a4 j___ZN3RBX9AllocatorINS_6D6LinkEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::D6Link>::operator new(unsigned long)")]
// 0xf4f3b4 — j___ZN3RBX9AllocatorINS_6D6LinkEEnwEm
pub fn stub_f4f3b4() -> ! {
    todo!("0xf4f3b4 j___ZN3RBX9AllocatorINS_6D6LinkEEnwEm")
}

