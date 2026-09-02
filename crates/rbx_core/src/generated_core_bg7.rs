//! core bg7 — 100 core stubs EA-sorted asc distinct not yet in rbx_core nor global set.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/rbx_core/src nor /tmp/global_eas.txt — next 100 uncovered after 0x750170 (bg6 max) -> 0x75034c..0x755834.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
// 0x75034c — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
// type: 
pub fn stub_75034c() -> ! {
    todo!("0x75034c __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv")]
// 0x7504a8 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv
// type: void __fastcall(void *)
pub fn stub_7504a8() -> ! {
    todo!("0x7504a8 __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
// 0x7504e4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
// type: 
pub fn stub_7504e4() -> ! {
    todo!("0x7504e4 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
// 0x75050c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_75050c() -> ! {
    todo!("0x75050c __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// 0x75056c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: 
pub fn stub_75056c() -> ! {
    todo!("0x75056c __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
// 0x750594 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
// type: 
pub fn stub_750594() -> ! {
    todo!("0x750594 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
// 0x7505c4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
// type: 
pub fn stub_7505c4() -> ! {
    todo!("0x7505c4 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")
}

#[doc(alias = "RBX::Vector3_2IntsComparer::operator()(RBX::Vector3_2Ints const&,RBX::Vector3_2Ints const&)const")]
#[doc(alias = "__ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_")]
// 0x7505f4 — __ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_
// type: 
pub fn stub_7505f4() -> ! {
    todo!("0x7505f4 __ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
// 0x75072c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// type: 
pub fn stub_75072c() -> ! {
    todo!("0x75072c __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
// 0x75080c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
// type: 
pub fn stub_75080c() -> ! {
    todo!("0x75080c __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
// 0x750870 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
// type: 
pub fn stub_750870() -> ! {
    todo!("0x750870 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
// 0x7508f0 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
// type: 
pub fn stub_7508f0() -> ! {
    todo!("0x7508f0 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm")]
// 0x7509b0 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm
// type: int(void)
pub fn stub_7509b0() -> ! {
    todo!("0x7509b0 __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm")
}

#[doc(alias = "RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE")]
// 0x750a20 — __ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE
// type: 
pub fn stub_750a20() -> ! {
    todo!("0x750a20 __ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev")]
// 0x750b70 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev
// type: int(void)
pub fn stub_750b70() -> ! {
    todo!("0x750b70 __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv")]
// 0x750bd4 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv
// type: 
pub fn stub_750bd4() -> ! {
    todo!("0x750bd4 __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
// 0x750c58 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
// type: 
pub fn stub_750c58() -> ! {
    todo!("0x750c58 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")]
// 0x750ca8 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv
// type: 
pub fn stub_750ca8() -> ! {
    todo!("0x750ca8 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
// 0x750cac — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
// type: 
pub fn stub_750cac() -> ! {
    todo!("0x750cac __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")]
// 0x750dbc — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev
// type: 
pub fn stub_750dbc() -> ! {
    todo!("0x750dbc __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")
}

#[doc(alias = "RBX::RightAngleRampPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX18RightAngleRampPoly9buildMeshEv")]
// 0x750f9c — __ZN3RBX18RightAngleRampPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
pub fn stub_750f9c() -> ! {
    todo!("0x750f9c __ZN3RBX18RightAngleRampPoly9buildMeshEv")
}

#[doc(alias = "RBX::RightAngleRampPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly9getMomentEf")]
// 0x751078 — __ZNK3RBX18RightAngleRampPoly9getMomentEf
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this, float)
pub fn stub_751078() -> ! {
    todo!("0x751078 __ZNK3RBX18RightAngleRampPoly9getMomentEf")
}

#[doc(alias = "RBX::RightAngleRampPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv")]
// 0x7511ac — __ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
pub fn stub_7511ac() -> ! {
    todo!("0x7511ac __ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv")
}

#[doc(alias = "RBX::RightAngleRampPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// 0x7511e0 — __ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
// type: 
pub fn stub_7511e0() -> ! {
    todo!("0x7511e0 __ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")
}

#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly()")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD1Ev")]
// 0x751398 — __ZN3RBX18RightAngleRampPolyD1Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
pub fn stub_751398() -> ! {
    todo!("0x751398 __ZN3RBX18RightAngleRampPolyD1Ev")
}

#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly()")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD0Ev")]
// 0x7513bc — __ZN3RBX18RightAngleRampPolyD0Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
pub fn stub_7513bc() -> ! {
    todo!("0x7513bc __ZN3RBX18RightAngleRampPolyD0Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv")]
// 0x751a00 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv
// type: void __fastcall(void *)
pub fn stub_751a00() -> ! {
    todo!("0x751a00 __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm")]
// 0x75203c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm
// type: 
pub fn stub_75203c() -> ! {
    todo!("0x75203c __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev")]
// 0x7521c8 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev
// type: 
pub fn stub_7521c8() -> ! {
    todo!("0x7521c8 __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv")]
// 0x75222c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv
// type: 
pub fn stub_75222c() -> ! {
    todo!("0x75222c __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv")
}

#[doc(alias = "RBX::RigidJoint::isAligned(void)")]
#[doc(alias = "__ZN3RBX10RigidJoint9isAlignedEv")]
// 0x752648 — __ZN3RBX10RigidJoint9isAlignedEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
pub fn stub_752648() -> ! {
    todo!("0x752648 __ZN3RBX10RigidJoint9isAlignedEv")
}

#[doc(alias = "RBX::RigidJoint::align(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_")]
// 0x752720 — __ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_752720() -> ! {
    todo!("0x752720 __ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::RigidJoint::getChildInParent(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_")]
// 0x752884 — __ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
pub fn stub_752884() -> ! {
    todo!("0x752884 __ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_")
}

#[doc(alias = "RBX::RigidJoint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX10RigidJoint8isBrokenEv")]
// 0x752b14 — __ZNK3RBX10RigidJoint8isBrokenEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
pub fn stub_752b14() -> ! {
    todo!("0x752b14 __ZNK3RBX10RigidJoint8isBrokenEv")
}

#[doc(alias = "RBX::RotateJoint::RotateJoint(void)")]
#[doc(alias = "__ZN3RBX11RotateJointC1Ev")]
// 0x752be0 — __ZN3RBX11RotateJointC1Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752be0() -> ! {
    todo!("0x752be0 __ZN3RBX11RotateJointC1Ev")
}

#[doc(alias = "RBX::RotateJoint::RotateJoint(void)")]
#[doc(alias = "__ZN3RBX11RotateJointC2Ev")]
// 0x752c04 — __ZN3RBX11RotateJointC2Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752c04() -> ! {
    todo!("0x752c04 __ZN3RBX11RotateJointC2Ev")
}

#[doc(alias = "RBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZN3RBX11RotateJointD0Ev")]
// 0x752c28 — __ZN3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752c28() -> ! {
    todo!("0x752c28 __ZN3RBX11RotateJointD0Ev")
}

#[doc(alias = "RBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZN3RBX11RotateJointD1Ev")]
// 0x752cc8 — __ZN3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752cc8() -> ! {
    todo!("0x752cc8 __ZN3RBX11RotateJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD0Ev")]
// 0x752ccc — __ZThn32_N3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752ccc() -> ! {
    todo!("0x752ccc __ZThn32_N3RBX11RotateJointD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD1Ev")]
// 0x752cd4 — __ZThn32_N3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752cd4() -> ! {
    todo!("0x752cd4 __ZThn32_N3RBX11RotateJointD1Ev")
}

#[doc(alias = "RBX::RotateJoint::getAxleWorldDirection(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint21getAxleWorldDirectionEv")]
// 0x752cdc — __ZN3RBX11RotateJoint21getAxleWorldDirectionEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752cdc() -> ! {
    todo!("0x752cdc __ZN3RBX11RotateJoint21getAxleWorldDirectionEv")
}

#[doc(alias = "RBX::RotateJoint::getAxleVelocity(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint15getAxleVelocityEv")]
// 0x752cfc — __ZN3RBX11RotateJoint15getAxleVelocityEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_752cfc() -> ! {
    todo!("0x752cfc __ZN3RBX11RotateJoint15getAxleVelocityEv")
}

#[doc(alias = "RBX::RotateJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
// 0x752fe8 — __ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_752fe8() -> ! {
    todo!("0x752fe8 __ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")
}

#[doc(alias = "RBX::RotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint16removeFromKernelEv")]
// 0x7537c4 — __ZN3RBX11RotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_7537c4() -> ! {
    todo!("0x7537c4 __ZN3RBX11RotateJoint16removeFromKernelEv")
}

#[doc(alias = "RBX::RotateJoint::getPrimitivesTorqueArmLength(float &,float &)")]
#[doc(alias = "__ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_")]
// 0x753828 — __ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, float *, float *)
pub fn stub_753828() -> ! {
    todo!("0x753828 __ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_")
}

#[doc(alias = "RBX::RotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE")]
// 0x753910 — __ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, RBX::Kernel *)
pub fn stub_753910() -> ! {
    todo!("0x753910 __ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE")
}

#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD0Ev")]
// 0x753c00 — __ZN3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_753c00() -> ! {
    todo!("0x753c00 __ZN3RBX18DynamicRotateJointD0Ev")
}

#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD1Ev")]
// 0x753ca0 — __ZN3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_753ca0() -> ! {
    todo!("0x753ca0 __ZN3RBX18DynamicRotateJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD0Ev")]
// 0x753ca4 — __ZThn32_N3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_753ca4() -> ! {
    todo!("0x753ca4 __ZThn32_N3RBX18DynamicRotateJointD0Ev")
}

#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD2Ev")]
// 0x753cac — __ZN3RBX18DynamicRotateJointD2Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_753cac() -> ! {
    todo!("0x753cac __ZN3RBX18DynamicRotateJointD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD1Ev")]
// 0x753dd4 — __ZThn32_N3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_753dd4() -> ! {
    todo!("0x753dd4 __ZThn32_N3RBX18DynamicRotateJointD1Ev")
}

#[doc(alias = "RBX::DynamicRotateJoint::setPhysics(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint10setPhysicsEv")]
// 0x753ddc — __ZN3RBX18DynamicRotateJoint10setPhysicsEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_753ddc() -> ! {
    todo!("0x753ddc __ZN3RBX18DynamicRotateJoint10setPhysicsEv")
}

#[doc(alias = "RBX::DynamicRotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE")]
// 0x753dec — __ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, RBX::Kernel *)
pub fn stub_753dec() -> ! {
    todo!("0x753dec __ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE")
}

#[doc(alias = "RBX::DynamicRotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint16removeFromKernelEv")]
// 0x753f94 — __ZN3RBX18DynamicRotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_753f94() -> ! {
    todo!("0x753f94 __ZN3RBX18DynamicRotateJoint16removeFromKernelEv")
}

#[doc(alias = "RBX::DynamicRotateJoint::stepUi(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint6stepUiEd")]
// 0x754060 — __ZN3RBX18DynamicRotateJoint6stepUiEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
pub fn stub_754060() -> ! {
    todo!("0x754060 __ZN3RBX18DynamicRotateJoint6stepUiEd")
}

#[doc(alias = "RBX::DynamicRotateJoint::getChannelValue(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint15getChannelValueEd")]
// 0x75409c — __ZN3RBX18DynamicRotateJoint15getChannelValueEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
pub fn stub_75409c() -> ! {
    todo!("0x75409c __ZN3RBX18DynamicRotateJoint15getChannelValueEd")
}

#[doc(alias = "RBX::RotatePJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotatePJoint9stepWorldEv")]
// 0x754170 — __ZN3RBX12RotatePJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_754170() -> ! {
    todo!("0x754170 __ZN3RBX12RotatePJoint9stepWorldEv")
}

#[doc(alias = "RBX::RotateVJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotateVJoint9stepWorldEv")]
// 0x754184 — __ZN3RBX12RotateVJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_754184() -> ! {
    todo!("0x754184 __ZN3RBX12RotateVJoint9stepWorldEv")
}

#[doc(alias = "RBX::RotateJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX11RotateJoint12getJointTypeEv")]
// 0x754198 — __ZNK3RBX11RotateJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
pub fn stub_754198() -> ! {
    todo!("0x754198 __ZNK3RBX11RotateJoint12getJointTypeEv")
}

#[doc(alias = "RBX::DynamicRotateJoint::canStepWorld(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint12canStepWorldEv")]
// 0x75419c — __ZNK3RBX18DynamicRotateJoint12canStepWorldEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_75419c() -> ! {
    todo!("0x75419c __ZNK3RBX18DynamicRotateJoint12canStepWorldEv")
}

#[doc(alias = "RBX::DynamicRotateJoint::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint9canStepUiEv")]
// 0x7541a0 — __ZNK3RBX18DynamicRotateJoint9canStepUiEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
pub fn stub_7541a0() -> ! {
    todo!("0x7541a0 __ZNK3RBX18DynamicRotateJoint9canStepUiEv")
}

#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZN3RBX12RotatePJointD1Ev")]
// 0x7541a4 — __ZN3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_7541a4() -> ! {
    todo!("0x7541a4 __ZN3RBX12RotatePJointD1Ev")
}

#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZN3RBX12RotatePJointD0Ev")]
// 0x7541a8 — __ZN3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_7541a8() -> ! {
    todo!("0x7541a8 __ZN3RBX12RotatePJointD0Ev")
}

#[doc(alias = "RBX::RotatePJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX12RotatePJoint12getJointTypeEv")]
// 0x754248 — __ZNK3RBX12RotatePJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_754248() -> ! {
    todo!("0x754248 __ZNK3RBX12RotatePJoint12getJointTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotatePJointD1Ev")]
// 0x75424c — __ZThn32_N3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_75424c() -> ! {
    todo!("0x75424c __ZThn32_N3RBX12RotatePJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotatePJointD0Ev")]
// 0x754254 — __ZThn32_N3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
pub fn stub_754254() -> ! {
    todo!("0x754254 __ZThn32_N3RBX12RotatePJointD0Ev")
}

#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZN3RBX12RotateVJointD1Ev")]
// 0x7542f8 — __ZN3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_7542f8() -> ! {
    todo!("0x7542f8 __ZN3RBX12RotateVJointD1Ev")
}

#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZN3RBX12RotateVJointD0Ev")]
// 0x7542fc — __ZN3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_7542fc() -> ! {
    todo!("0x7542fc __ZN3RBX12RotateVJointD0Ev")
}

#[doc(alias = "RBX::RotateVJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX12RotateVJoint12getJointTypeEv")]
// 0x75439c — __ZNK3RBX12RotateVJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_75439c() -> ! {
    todo!("0x75439c __ZNK3RBX12RotateVJoint12getJointTypeEv")
}

#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotateVJointD1Ev")]
// 0x7543a0 — __ZThn32_N3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_7543a0() -> ! {
    todo!("0x7543a0 __ZThn32_N3RBX12RotateVJointD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotateVJointD0Ev")]
// 0x7543a8 — __ZThn32_N3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
pub fn stub_7543a8() -> ! {
    todo!("0x7543a8 __ZThn32_N3RBX12RotateVJointD0Ev")
}

#[doc(alias = "RBX::JointConnector::getConnectorKernelType(void)const")]
#[doc(alias = "__ZNK3RBX14JointConnector22getConnectorKernelTypeEv")]
// 0x75444c — __ZNK3RBX14JointConnector22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::JointConnector *__hidden this)
pub fn stub_75444c() -> ! {
    todo!("0x75444c __ZNK3RBX14JointConnector22getConnectorKernelTypeEv")
}

#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnectorD1Ev")]
// 0x754450 — __ZN3RBX26PointToPointBreakConnectorD1Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
pub fn stub_754450() -> ! {
    todo!("0x754450 __ZN3RBX26PointToPointBreakConnectorD1Ev")
}

#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnectorD0Ev")]
// 0x754454 — __ZN3RBX26PointToPointBreakConnectorD0Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
pub fn stub_754454() -> ! {
    todo!("0x754454 __ZN3RBX26PointToPointBreakConnectorD0Ev")
}

#[doc(alias = "RBX::PointToPointBreakConnector::getBroken(void)")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnector9getBrokenEv")]
// 0x754458 — __ZN3RBX26PointToPointBreakConnector9getBrokenEv
// type: _DWORD __fastcall(RBX::PointToPointBreakConnector *__hidden this)
pub fn stub_754458() -> ! {
    todo!("0x754458 __ZN3RBX26PointToPointBreakConnector9getBrokenEv")
}

#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
#[doc(alias = "__ZN3RBX11SendPhysicsC1Ev")]
// 0x75462c — __ZN3RBX11SendPhysicsC1Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_75462c() -> ! {
    todo!("0x75462c __ZN3RBX11SendPhysicsC1Ev")
}

#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
#[doc(alias = "__ZN3RBX11SendPhysicsC2Ev")]
// 0x754630 — __ZN3RBX11SendPhysicsC2Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_754630() -> ! {
    todo!("0x754630 __ZN3RBX11SendPhysicsC2Ev")
}

#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
#[doc(alias = "__ZN3RBX11SendPhysicsD1Ev")]
// 0x754824 — __ZN3RBX11SendPhysicsD1Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_754824() -> ! {
    todo!("0x754824 __ZN3RBX11SendPhysicsD1Ev")
}

#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
#[doc(alias = "__ZN3RBX11SendPhysicsD2Ev")]
// 0x754828 — __ZN3RBX11SendPhysicsD2Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
pub fn stub_754828() -> ! {
    todo!("0x754828 __ZN3RBX11SendPhysicsD2Ev")
}

#[doc(alias = "RBX::SendPhysics::buildSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE")]
// 0x754abc — __ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
pub fn stub_754abc() -> ! {
    todo!("0x754abc __ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SendPhysics::destroySimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE")]
// 0x754b34 — __ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
pub fn stub_754b34() -> ! {
    todo!("0x754b34 __ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE")]
// 0x754bd0 — __ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
pub fn stub_754bd0() -> ! {
    todo!("0x754bd0 __ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE")]
// 0x754d1c — __ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
pub fn stub_754d1c() -> ! {
    todo!("0x754d1c __ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE")
}

#[doc(alias = "RBX::SendPhysics::nextSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE")]
// 0x754e00 — __ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
pub fn stub_754e00() -> ! {
    todo!("0x754e00 __ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SimJob::getConstSimJobFromPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE")]
// 0x754f3c — __ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, const RBX::Primitive *)
pub fn stub_754f3c() -> ! {
    todo!("0x754f3c __ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE")
}

#[doc(alias = "RBX::SimJobTracker::stopTracking(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker12stopTrackingEv")]
// 0x754f54 — __ZN3RBX13SimJobTracker12stopTrackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
pub fn stub_754f54() -> ! {
    todo!("0x754f54 __ZN3RBX13SimJobTracker12stopTrackingEv")
}

#[doc(alias = "RBX::SimJobTracker::tracking(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker8trackingEv")]
// 0x755034 — __ZN3RBX13SimJobTracker8trackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
pub fn stub_755034() -> ! {
    todo!("0x755034 __ZN3RBX13SimJobTracker8trackingEv")
}

#[doc(alias = "RBX::SimJobTracker::setSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE")]
// 0x7550bc — __ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *)
pub fn stub_7550bc() -> ! {
    todo!("0x7550bc __ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE")
}

#[doc(alias = "RBX::SimJobTracker::getSimJob(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker9getSimJobEv")]
// 0x7551a8 — __ZN3RBX13SimJobTracker9getSimJobEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
pub fn stub_7551a8() -> ! {
    todo!("0x7551a8 __ZN3RBX13SimJobTracker9getSimJobEv")
}

#[doc(alias = "RBX::SimJobTracker::transferTrackers(RBX::SimJob *,RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_")]
// 0x755264 — __ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *, RBX::SimJob *)
pub fn stub_755264() -> ! {
    todo!("0x755264 __ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_")
}

#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX6SimJobC1EPNS_8AssemblyE")]
// 0x755310 — __ZN3RBX6SimJobC1EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
pub fn stub_755310() -> ! {
    todo!("0x755310 __ZN3RBX6SimJobC1EPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX6SimJobC2EPNS_8AssemblyE")]
// 0x755314 — __ZN3RBX6SimJobC2EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
pub fn stub_755314() -> ! {
    todo!("0x755314 __ZN3RBX6SimJobC2EPNS_8AssemblyE")
}

#[doc(alias = "RBX::SimJob::~SimJob()")]
#[doc(alias = "__ZN3RBX6SimJobD1Ev")]
// 0x755424 — __ZN3RBX6SimJobD1Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
pub fn stub_755424() -> ! {
    todo!("0x755424 __ZN3RBX6SimJobD1Ev")
}

#[doc(alias = "RBX::SimJob::~SimJob()")]
#[doc(alias = "__ZN3RBX6SimJobD2Ev")]
// 0x755428 — __ZN3RBX6SimJobD2Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
pub fn stub_755428() -> ! {
    todo!("0x755428 __ZN3RBX6SimJobD2Ev")
}

#[doc(alias = "unsigned long RBX::fastRemoveShort<RBX::SimJobTracker *>(std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>> &,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_")]
// 0x755580 — __ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_
// type: 
pub fn stub_755580() -> ! {
    todo!("0x755580 __ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::push_back(RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_")]
// 0x7556dc — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_
// type: 
pub fn stub_7556dc() -> ! {
    todo!("0x7556dc __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x755708 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_755708() -> ! {
    todo!("0x755708 __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm")]
// 0x7557e8 — __ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm
// type: 
pub fn stub_7557e8() -> ! {
    todo!("0x7557e8 __ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::resize(unsigned long,RBX::SimJobTracker *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_")]
// 0x755800 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_
// type: 
pub fn stub_755800() -> ! {
    todo!("0x755800 __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,unsigned long,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x755834 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
pub fn stub_755834() -> ! {
    todo!("0x755834 __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}
