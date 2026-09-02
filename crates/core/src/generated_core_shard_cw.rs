//! core shard CW — 100 core stubs EA-sorted, next uncovered after CV 0x72ac08 (strict RBX|boost|std|rbx earliest gap).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::checkAndReleaseContacts(RBX::Primitive*)")]
// 0x72ad40 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_
pub fn stub_72ad40() -> ! {
    todo!("0x72ad40 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE23checkAndReleaseContactsEPS1_")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::oldExtentsOverlap(RBX::Primitive*,RBX::Primitive*)")]
// 0x72adc4 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_
pub fn stub_72adc4() -> ! {
    todo!("0x72adc4 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE17oldExtentsOverlapEPS1_S5_")
}

#[doc(alias = "RBX::ExtentsInt32::contains(RBX::Vector3int32 const&)const")]
// 0x72aef8 — __ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E
pub fn stub_72aef8() -> ! {
    todo!("0x72aef8 __ZNK3RBX12ExtentsInt328containsERKNS_12Vector3int32E")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addNode(RBX::Primitive*,RBX::Vector3int32 const&,bool)")]
// 0x72af38 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb
pub fn stub_72af38() -> ! {
    todo!("0x72af38 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7addNodeEPS1_RKNS_12Vector3int32Eb")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findNode(RBX::Primitive*,RBX::Vector3int32 const&)")]
// 0x72b3c0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E
pub fn stub_72b3c0() -> ! {
    todo!("0x72b3c0 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8findNodeEPS1_RKNS_12Vector3int32E")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeNodeFromHash(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0x72b494 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE
pub fn stub_72b494() -> ! {
    todo!("0x72b494 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE18removeNodeFromHashEPNS4_11SpatialNodeE")
}

#[doc(alias = "RBX::NodeBase::getLevel(void)")]
// 0x72b4c4 — __ZN3RBX8NodeBase8getLevelEv
pub fn stub_72b4c4() -> ! {
    todo!("0x72b4c4 __ZN3RBX8NodeBase8getLevelEv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findOtherNodesInLevel0Cell(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0x72b528 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE
pub fn stub_72b528() -> ! {
    todo!("0x72b528 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE26findOtherNodesInLevel0CellEPNS4_11SpatialNodeE")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::removeTreeNodeChild(int,RBX::Vector3int32 &)")]
// 0x72b5b8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E
pub fn stub_72b5b8() -> ! {
    todo!("0x72b5b8 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19removeTreeNodeChildEiRNS_12Vector3int32E")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator delete(void *)")]
// 0x72b730 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv
pub fn stub_72b730() -> ! {
    todo!("0x72b730 __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEdlEPv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
// 0x72b7c0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE
pub fn stub_72b7c0() -> ! {
    todo!("0x72b7c0 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS4_8TreeNodeE")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::~TreeNode()")]
// 0x72b92c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev
pub fn stub_72b92c() -> ! {
    todo!("0x72b92c __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeD2Ev")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::newNode(int,int,RBX::Vector3int32 const&)")]
// 0x72ba94 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E
pub fn stub_72ba94() -> ! {
    todo!("0x72ba94 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7newNodeEiiRKNS_12Vector3int32E")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::insertNodeToPrimitive(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *,RBX::Primitive*,RBX::Vector3int32 const&,int)")]
// 0x72bc74 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei
pub fn stub_72bc74() -> ! {
    todo!("0x72bc74 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE21insertNodeToPrimitiveEPNS4_11SpatialNodeEPS1_RKNS_12Vector3int32Ei")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
// 0x72bcf8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
pub fn stub_72bcf8() -> ! {
    todo!("0x72bcf8 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::addContactFromChildren(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *,RBX::Primitive*)")]
// 0x72be14 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_
pub fn stub_72be14() -> ! {
    todo!("0x72be14 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE22addContactFromChildrenEPNS4_8TreeNodeEPS1_")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator new(unsigned long)")]
// 0x72c004 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm
pub fn stub_72c004() -> ! {
    todo!("0x72c004 __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEnwEm")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode::TreeNode(void)")]
// 0x72c2f0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev
pub fn stub_72c2f0() -> ! {
    todo!("0x72c2f0 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::Allocator(void)")]
// 0x72c3e0 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev
pub fn stub_72c3e0() -> ! {
    todo!("0x72c3e0 __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::releaseMemory(void)")]
// 0x72c448 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEE13releaseMemoryEv
pub fn stub_72c448() -> ! {
    todo!("0x72c448 __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEE13releaseMemoryEv")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::operator new(unsigned long)")]
// 0x72c4d4 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm
pub fn stub_72c4d4() -> ! {
    todo!("0x72c4d4 __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::Allocator(void)")]
// 0x72c68c — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev
pub fn stub_72c68c() -> ! {
    todo!("0x72c68c __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode>::releaseMemory(void)")]
// 0x72c6f0 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEE13releaseMemoryEv
pub fn stub_72c6f0() -> ! {
    todo!("0x72c6f0 __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeEE13releaseMemoryEv")
}

#[doc(alias = "RBX::ExtentsInt32::empty(void)")]
// 0x72c778 — __ZN3RBX12ExtentsInt325emptyEv
pub fn stub_72c778() -> ! {
    todo!("0x72c778 __ZN3RBX12ExtentsInt325emptyEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert_unique(RBX::Primitive * const&)")]
// 0x72d108 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_72d108() -> ! {
    todo!("0x72d108 __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::Primitive * const&)")]
// 0x72d170 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_72d170() -> ! {
    todo!("0x72d170 __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::set<RBX::Primitive *,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::set<RBX::Primitive * const*>(RBX::Primitive * const*,RBX::Primitive * const*)")]
// 0x72d1c8 — __ZNSt3setIPN3RBX9PrimitiveESt4lessIS2_ESaIS2_EEC2IPKS2_EET_SA_
pub fn stub_72d1c8() -> ! {
    todo!("0x72d1c8 __ZNSt3setIPN3RBX9PrimitiveESt4lessIS2_ESaIS2_EEC2IPKS2_EET_SA_")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_erase(std::_Rb_tree_node<RBX::Primitive *> *)")]
// 0x72d2a0 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_72d2a0() -> ! {
    todo!("0x72d2a0 __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::_M_insert_unique(std::_Rb_tree_iterator<RBX::Primitive *>,RBX::Primitive * const&)")]
// 0x72d2c8 — __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_72d2c8() -> ! {
    todo!("0x72d2c8 __ZNSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Primitive *,std::allocator<RBX::Primitive *>>::_M_allocate(unsigned long)")]
// 0x72d380 — __ZNSt12_Vector_baseIPN3RBX9PrimitiveESaIS2_EE11_M_allocateEm
pub fn stub_72d380() -> ! {
    todo!("0x72d380 __ZNSt12_Vector_baseIPN3RBX9PrimitiveESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Rb_tree<RBX::Primitive *,RBX::Primitive *,std::_Identity<RBX::Primitive *>,std::less<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::find(RBX::Primitive * const&)const")]
// 0x72d398 — __ZNKSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE4findERKS2_
pub fn stub_72d398() -> ! {
    todo!("0x72d398 __ZNKSt8_Rb_treeIPN3RBX9PrimitiveES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE4findERKS2_")
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::for_each<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearSpatialNode>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearSpatialNode &)")]
// 0x72d3d8 — __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS5_20FastClearSpatialNodeEEEvRT_
pub fn stub_72d3d8() -> ! {
    todo!("0x72d3d8 __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS5_20FastClearSpatialNodeEEEvRT_")
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::for_each<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearTreeNode>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::FastClearTreeNode &)")]
// 0x72d488 — __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS5_17FastClearTreeNodeEEEvRT_
pub fn stub_72d488() -> ! {
    todo!("0x72d488 __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS5_17FastClearTreeNodeEEEvRT_")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::cleanup(void)")]
// 0x72d4f8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7cleanupEv
pub fn stub_72d4f8() -> ! {
    todo!("0x72d4f8 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE7cleanupEv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::setup(void)")]
// 0x72d55c — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE5setupEv
pub fn stub_72d55c() -> ! {
    todo!("0x72d55c __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE5setupEv")
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::resize(unsigned long,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry)")]
// 0x72d590 — __ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE6resizeEmS6_
pub fn stub_72d590() -> ! {
    todo!("0x72d590 __ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE6resizeEmS6_")
}

#[doc(alias = "std::_Vector_base<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_allocate(unsigned long)")]
// 0x72d5c8 — __ZNSt12_Vector_baseIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE11_M_allocateEm
pub fn stub_72d5c8() -> ! {
    todo!("0x72d5c8 __ZNSt12_Vector_baseIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *>(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry *)")]
// 0x72d5e0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SpatialHashINS3_9PrimitiveENS3_7ContactENS3_14ContactManagerELi4EE21SpatialHashTableEntryESA_EET0_T_SC_SB_
pub fn stub_72d5e0() -> ! {
    todo!("0x72d5e0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11SpatialHashINS3_9PrimitiveENS3_7ContactENS3_14ContactManagerELi4EE21SpatialHashTableEntryESA_EET0_T_SC_SB_")
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::for_each<RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::CallDestructor>(RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::CallDestructor &)")]
// 0x72d624 — __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_
pub fn stub_72d624() -> ! {
    todo!("0x72d624 __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_")
}

#[doc(alias = "void RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::for_each<RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::CallDestructor>(RBX::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::CallDestructor &)")]
// 0x72d6dc — __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_
pub fn stub_72d6dc() -> ! {
    todo!("0x72d6dc __ZN3RBX11object_poolINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11SpatialNodeENS_16roblox_allocatorEE8for_eachINS8_14CallDestructorEEEvRT_")
}

#[doc(alias = "RBX::BallBlockContact::BallBlockContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x72d8fc — __ZN3RBX16BallBlockContactC2EPNS_9PrimitiveES2_
pub fn stub_72d8fc() -> ! {
    todo!("0x72d8fc __ZN3RBX16BallBlockContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::Allocator(void)")]
// 0x72d9c8 — __ZN3RBX9AllocatorINS_16BallBlockContactEEC2Ev
pub fn stub_72d9c8() -> ! {
    todo!("0x72d9c8 __ZN3RBX9AllocatorINS_16BallBlockContactEEC2Ev")
}

#[doc(alias = "RBX::BallBlockContact::~BallBlockContact()")]
// 0x72da2c — __ZN3RBX16BallBlockContactD1Ev
pub fn stub_72da2c() -> ! {
    todo!("0x72da2c __ZN3RBX16BallBlockContactD1Ev")
}

#[doc(alias = "RBX::BallBlockContact::~BallBlockContact()")]
// 0x72da30 — __ZN3RBX16BallBlockContactD0Ev
pub fn stub_72da30() -> ! {
    todo!("0x72da30 __ZN3RBX16BallBlockContactD0Ev")
}

#[doc(alias = "RBX::Contact::putInKernel(RBX::Kernel *)")]
// 0x72dae4 — __ZN3RBX7Contact11putInKernelEPNS_6KernelE
pub fn stub_72dae4() -> ! {
    todo!("0x72dae4 __ZN3RBX7Contact11putInKernelEPNS_6KernelE")
}

#[doc(alias = "RBX::Contact::removeFromKernel(void)")]
// 0x72dae8 — __ZN3RBX7Contact16removeFromKernelEv
pub fn stub_72dae8() -> ! {
    todo!("0x72dae8 __ZN3RBX7Contact16removeFromKernelEv")
}

#[doc(alias = "RBX::Contact::getEdgeType(void)const")]
// 0x72db54 — __ZNK3RBX7Contact11getEdgeTypeEv
pub fn stub_72db54() -> ! {
    todo!("0x72db54 __ZNK3RBX7Contact11getEdgeTypeEv")
}

#[doc(alias = "RBX::BallBlockContact::numConnectors(void)const")]
// 0x72db58 — __ZNK3RBX16BallBlockContact13numConnectorsEv
pub fn stub_72db58() -> ! {
    todo!("0x72db58 __ZNK3RBX16BallBlockContact13numConnectorsEv")
}

#[doc(alias = "RBX::BallBlockContact::~BallBlockContact()")]
// 0x72db64 — __ZN3RBX16BallBlockContactD2Ev
pub fn stub_72db64() -> ! {
    todo!("0x72db64 __ZN3RBX16BallBlockContactD2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallBlockContact>::releaseMemory(void)")]
// 0x72dc88 — __ZN3RBX9AllocatorINS_16BallBlockContactEE13releaseMemoryEv
pub fn stub_72dc88() -> ! {
    todo!("0x72dc88 __ZN3RBX9AllocatorINS_16BallBlockContactEE13releaseMemoryEv")
}

#[doc(alias = "RBX::BallBallContact::BallBallContact(RBX::Primitive *,RBX::Primitive *)")]
// 0x72dcd4 — __ZN3RBX15BallBallContactC2EPNS_9PrimitiveES2_
pub fn stub_72dcd4() -> ! {
    todo!("0x72dcd4 __ZN3RBX15BallBallContactC2EPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::Allocator(void)")]
// 0x72dda0 — __ZN3RBX9AllocatorINS_15BallBallContactEEC2Ev
pub fn stub_72dda0() -> ! {
    todo!("0x72dda0 __ZN3RBX9AllocatorINS_15BallBallContactEEC2Ev")
}

#[doc(alias = "RBX::BallBallContact::~BallBallContact()")]
// 0x72de04 — __ZN3RBX15BallBallContactD1Ev
pub fn stub_72de04() -> ! {
    todo!("0x72de04 __ZN3RBX15BallBallContactD1Ev")
}

#[doc(alias = "RBX::BallBallContact::~BallBallContact()")]
// 0x72de08 — __ZN3RBX15BallBallContactD0Ev
pub fn stub_72de08() -> ! {
    todo!("0x72de08 __ZN3RBX15BallBallContactD0Ev")
}

#[doc(alias = "RBX::BallBallContact::numConnectors(void)const")]
// 0x72debc — __ZNK3RBX15BallBallContact13numConnectorsEv
pub fn stub_72debc() -> ! {
    todo!("0x72debc __ZNK3RBX15BallBallContact13numConnectorsEv")
}

#[doc(alias = "RBX::BallBallContact::~BallBallContact()")]
// 0x72dec8 — __ZN3RBX15BallBallContactD2Ev
pub fn stub_72dec8() -> ! {
    todo!("0x72dec8 __ZN3RBX15BallBallContactD2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::BallBallContact>::releaseMemory(void)")]
// 0x72dfec — __ZN3RBX9AllocatorINS_15BallBallContactEE13releaseMemoryEv
pub fn stub_72dfec() -> ! {
    todo!("0x72dfec __ZN3RBX9AllocatorINS_15BallBallContactEE13releaseMemoryEv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::~SpatialHash()")]
// 0x72e098 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EED2Ev
pub fn stub_72e098() -> ! {
    todo!("0x72e098 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EED2Ev")
}

#[doc(alias = "RBX::ContactManagerSpatialHash::ContactManagerSpatialHash(RBX::World *,RBX::ContactManager *)")]
// 0x72eba4 — __ZN3RBX25ContactManagerSpatialHashC1EPNS_5WorldEPNS_14ContactManagerE
pub fn stub_72eba4() -> ! {
    todo!("0x72eba4 __ZN3RBX25ContactManagerSpatialHashC1EPNS_5WorldEPNS_14ContactManagerE")
}

#[doc(alias = "RBX::ContactManagerSpatialHash::onPrimitiveMoved(RBX::Assembly &)")]
// 0x72ebac — __ZN3RBX25ContactManagerSpatialHash16onPrimitiveMovedERNS_8AssemblyE
pub fn stub_72ebac() -> ! {
    todo!("0x72ebac __ZN3RBX25ContactManagerSpatialHash16onPrimitiveMovedERNS_8AssemblyE")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHash(RBX::World *,RBX::ContactManager*,int)")]
// 0x72ec30 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS3_i
pub fn stub_72ec30() -> ! {
    todo!("0x72ec30 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EEC2EPNS_5WorldEPS3_i")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)")]
// 0x72edf0 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E
pub fn stub_72edf0() -> ! {
    todo!("0x72edf0 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E")
}

#[doc(alias = "RBX::Allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode>::operator delete(void *)")]
// 0x72eef0 — __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEdlEPv
pub fn stub_72eef0() -> ! {
    todo!("0x72eef0 __ZN3RBX9AllocatorINS_11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE8TreeNodeEEdlEPv")
}

#[doc(alias = "RBX::BasicSpatialHashPrimitive::getSpatialNodeLevel(void)const")]
// 0x72ef30 — __ZNK3RBX25BasicSpatialHashPrimitive19getSpatialNodeLevelEv
pub fn stub_72ef30() -> ! {
    todo!("0x72ef30 __ZNK3RBX25BasicSpatialHashPrimitive19getSpatialNodeLevelEv")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAssembled(RBX::Primitive*)")]
// 0x72ef90 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE20onPrimitiveAssembledEPS1_
pub fn stub_72ef90() -> ! {
    todo!("0x72ef90 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE20onPrimitiveAssembledEPS1_")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0x72f4d8 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS4_11SpatialNodeE
pub fn stub_72f4d8() -> ! {
    todo!("0x72f4d8 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS4_11SpatialNodeE")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::retireTreeNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
// 0x72f528 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14retireTreeNodeEPNS4_8TreeNodeE
pub fn stub_72f528() -> ! {
    todo!("0x72f528 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE14retireTreeNodeEPNS4_8TreeNodeE")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
// 0x72f568 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS4_11SpatialNodeE
pub fn stub_72f568() -> ! {
    todo!("0x72f568 __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS4_11SpatialNodeE")
}

#[doc(alias = "std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry*,std::vector<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry,std::allocator<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry>>>,unsigned long,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialHashTableEntry const&)")]
// 0x72f6d0 — __ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS6_S8_EEmRKS6_
pub fn stub_72f6d0() -> ! {
    todo!("0x72f6d0 __ZNSt6vectorIN3RBX11SpatialHashINS0_9PrimitiveENS0_7ContactENS0_14ContactManagerELi4EE21SpatialHashTableEntryESaIS6_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS6_S8_EEmRKS6_")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,RBX::NodeInfo,std::less<RBX::NodeInfo>>(__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,int,RBX::NodeInfo,std::less<RBX::NodeInfo>)")]
// 0x72f8b4 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_
pub fn stub_72f8b4() -> ! {
    todo!("0x72f8b4 __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_")
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,RBX::NodeInfo,std::less<RBX::NodeInfo>>(__gnu_cxx::__normal_iterator<RBX::NodeInfo *,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,int,int,RBX::NodeInfo,std::less<RBX::NodeInfo>)")]
// 0x72f94c — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_
pub fn stub_72f94c() -> ! {
    todo!("0x72f94c __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX8NodeInfoESt6vectorIS3_SaIS3_EEEEiS3_St4lessIS3_EEvT_T0_SC_T1_T2_")
}

#[doc(alias = "std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NodeInfo*,std::vector<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>>,RBX::NodeInfo const&)")]
// 0x72f9ac — __ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_72f9ac() -> ! {
    todo!("0x72f9ac __ZNSt6vectorIN3RBX8NodeInfoESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::NodeInfo,std::allocator<RBX::NodeInfo>>::_M_allocate(unsigned long)")]
// 0x72fab8 — __ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm
pub fn stub_72fab8() -> ! {
    todo!("0x72fab8 __ZNSt12_Vector_baseIN3RBX8NodeInfoESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::NodeInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::NodeInfo *,RBX::NodeInfo *>(RBX::NodeInfo *,RBX::NodeInfo *,RBX::NodeInfo *)")]
// 0x72fad0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_
pub fn stub_72fad0() -> ! {
    todo!("0x72fad0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8NodeInfoES5_EET0_T_S7_S6_")
}

#[doc(alias = "RBX::Extents::operator==(RBX::Extents const&)const")]
// 0x72fb18 — __ZNK3RBX7ExtentseqERKS0_
pub fn stub_72fb18() -> ! {
    todo!("0x72fb18 __ZNK3RBX7ExtentseqERKS0_")
}

#[doc(alias = "RBX::Extents::zero(void)")]
// 0x72fb98 — __ZN3RBX7Extents4zeroEv
pub fn stub_72fb98() -> ! {
    todo!("0x72fb98 __ZN3RBX7Extents4zeroEv")
}

#[doc(alias = "RBX::ExtentsInt32::overlapsOrTouches(RBX::ExtentsInt32 const&)const")]
// 0x72fc90 — __ZNK3RBX12ExtentsInt3217overlapsOrTouchesERKS0_
pub fn stub_72fc90() -> ! {
    todo!("0x72fc90 __ZNK3RBX12ExtentsInt3217overlapsOrTouchesERKS0_")
}

#[doc(alias = "RBX::ContactStage::ContactStage(RBX::IStage *,RBX::World *)")]
// 0x72ff44 — __ZN3RBX12ContactStageC1EPNS_6IStageEPNS_5WorldE
pub fn stub_72ff44() -> ! {
    todo!("0x72ff44 __ZN3RBX12ContactStageC1EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::ContactStage::ContactStage(RBX::IStage *,RBX::World *)")]
// 0x72ff48 — __ZN3RBX12ContactStageC2EPNS_6IStageEPNS_5WorldE
pub fn stub_72ff48() -> ! {
    todo!("0x72ff48 __ZN3RBX12ContactStageC2EPNS_6IStageEPNS_5WorldE")
}

#[doc(alias = "RBX::ContactStage::onPrimitiveAdded(RBX::Primitive *)")]
// 0x73001c — __ZN3RBX12ContactStage16onPrimitiveAddedEPNS_9PrimitiveE
pub fn stub_73001c() -> ! {
    todo!("0x73001c __ZN3RBX12ContactStage16onPrimitiveAddedEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactStage::onPrimitiveRemoving(RBX::Primitive *)")]
// 0x730038 — __ZN3RBX12ContactStage19onPrimitiveRemovingEPNS_9PrimitiveE
pub fn stub_730038() -> ! {
    todo!("0x730038 __ZN3RBX12ContactStage19onPrimitiveRemovingEPNS_9PrimitiveE")
}

#[doc(alias = "RBX::ContactStage::onEdgeAdded(RBX::Edge *)")]
// 0x730054 — __ZN3RBX12ContactStage11onEdgeAddedEPNS_4EdgeE
pub fn stub_730054() -> ! {
    todo!("0x730054 __ZN3RBX12ContactStage11onEdgeAddedEPNS_4EdgeE")
}

#[doc(alias = "RBX::ContactStage::onEdgeRemoving(RBX::Edge *)")]
// 0x73017c — __ZN3RBX12ContactStage14onEdgeRemovingEPNS_4EdgeE
pub fn stub_73017c() -> ! {
    todo!("0x73017c __ZN3RBX12ContactStage14onEdgeRemovingEPNS_4EdgeE")
}

#[doc(alias = "RBX::ContactStage::~ContactStage()")]
// 0x7302a4 — __ZN3RBX12ContactStageD1Ev
pub fn stub_7302a4() -> ! {
    todo!("0x7302a4 __ZN3RBX12ContactStageD1Ev")
}

#[doc(alias = "RBX::ContactStage::~ContactStage()")]
// 0x7302c8 — __ZN3RBX12ContactStageD0Ev
pub fn stub_7302c8() -> ! {
    todo!("0x7302c8 __ZN3RBX12ContactStageD0Ev")
}

#[doc(alias = "RBX::ContactStage::getStageType(void)const")]
// 0x730380 — __ZNK3RBX12ContactStage12getStageTypeEv
pub fn stub_730380() -> ! {
    todo!("0x730380 __ZNK3RBX12ContactStage12getStageTypeEv")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::resize(unsigned long,RBX::LegacyController::InputType)")]
// 0x730ad4 — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE6resizeEmS2_
pub fn stub_730ad4() -> ! {
    todo!("0x730ad4 __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::push_back(RBX::LegacyController::InputType const&)")]
// 0x730b08 — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE9push_backERKS2_
pub fn stub_730b08() -> ! {
    todo!("0x730b08 __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::LegacyController::InputType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::operator[](RBX::Name const* const&)")]
// 0x730b30 — __ZNSt3mapIPKN3RBX4NameENS0_16LegacyController9InputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_730b30() -> ! {
    todo!("0x730b30 __ZNSt3mapIPKN3RBX4NameENS0_16LegacyController9InputTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
// 0x730b88 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_730b88() -> ! {
    todo!("0x730b88 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
// 0x730c3c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_730c3c() -> ! {
    todo!("0x730c3c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::LegacyController::InputType> const&)")]
// 0x730c94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_730c94() -> ! {
    todo!("0x730c94 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::LegacyController::InputType*,std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>>,RBX::LegacyController::InputType const&)")]
// 0x730cfc — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_730cfc() -> ! {
    todo!("0x730cfc __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_allocate(unsigned long)")]
// 0x730de0 — __ZNSt12_Vector_baseIN3RBX16LegacyController9InputTypeESaIS2_EE11_M_allocateEm
pub fn stub_730de0() -> ! {
    todo!("0x730de0 __ZNSt12_Vector_baseIN3RBX16LegacyController9InputTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::LegacyController::InputType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::LegacyController::InputType *,RBX::LegacyController::InputType *>(RBX::LegacyController::InputType *,RBX::LegacyController::InputType *,RBX::LegacyController::InputType *)")]
// 0x730df8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16LegacyController9InputTypeES6_EET0_T_S8_S7_
pub fn stub_730df8() -> ! {
    todo!("0x730df8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16LegacyController9InputTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::LegacyController::InputType*,std::vector<RBX::LegacyController::InputType,std::allocator<RBX::LegacyController::InputType>>>,unsigned long,RBX::LegacyController::InputType const&)")]
// 0x730e34 — __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_730e34() -> ! {
    todo!("0x730e34 __ZNSt6vectorIN3RBX16LegacyController9InputTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::CornerWedgePoly::buildMesh(void)")]
// 0x73108c — __ZN3RBX15CornerWedgePoly9buildMeshEv
pub fn stub_73108c() -> ! {
    todo!("0x73108c __ZN3RBX15CornerWedgePoly9buildMeshEv")
}

#[doc(alias = "RBX::CornerWedgePoly::getMoment(float)const")]
// 0x731168 — __ZNK3RBX15CornerWedgePoly9getMomentEf
pub fn stub_731168() -> ! {
    todo!("0x731168 __ZNK3RBX15CornerWedgePoly9getMomentEf")
}

#[doc(alias = "RBX::CornerWedgePoly::getCofmOffset(void)const")]
// 0x73129c — __ZNK3RBX15CornerWedgePoly13getCofmOffsetEv
pub fn stub_73129c() -> ! {
    todo!("0x73129c __ZNK3RBX15CornerWedgePoly13getCofmOffsetEv")
}

#[doc(alias = "RBX::CornerWedgePoly::getSurfaceCoordInBody(unsigned long)const")]
// 0x7312d0 — __ZNK3RBX15CornerWedgePoly21getSurfaceCoordInBodyEm
pub fn stub_7312d0() -> ! {
    todo!("0x7312d0 __ZNK3RBX15CornerWedgePoly21getSurfaceCoordInBodyEm")
}
