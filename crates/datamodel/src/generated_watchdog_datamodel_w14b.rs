//! datamodel — generated_watchdog_datamodel_w14b — 100 stubs (watchdog w14b datamodel)
//! Source: ida/export.json (85545 funcs) EA-sorted asc, RBX:: datamodel filter (comprehensive) + RBX fallback
//! Filter: RBX:: + instance|datamodel|workspace|humanoid|joint|keyframe|lighting|selection|gui|controller|terrain|voxel|spatial|mega|cluster|attachment|constraint|etc, SKIP /tmp/global_eas.txt (88231 EAs), UNIQUE vs datamodel stubs (31416)
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost::shared_ptr.
//! Range: 0xf54174..0xf65b04 | strict 76 rbx 87 -> picked 100

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf54174 — j___ZN3rbx14implementation12typed_holderISsE9singletonEv
// type: int __fastcall(_DWORD)
// rbx::implementation::typed_holder<std::string>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<std::string>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderISsE9singletonEv")]
pub fn stub_0xf54174() -> ! {
    todo!("0xf54174 rbx::implementation::typed_holder<std::string>::singleton(void)")
}

// 0xf54184 — j___ZN3rbx14implementation12typed_holderIiE9singletonEv
// rbx::implementation::typed_holder<int>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<int>::singleton(void)")]
#[doc(alias = "j___ZN3rbx14implementation12typed_holderIiE9singletonEv")]
pub fn stub_0xf54184() -> ! {
    todo!("0xf54184 rbx::implementation::typed_holder<int>::singleton(void)")
}

// 0xf57ee4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_")]
pub fn stub_0xf57ee4() -> ! {
    todo!("0xf57ee4 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")
}

// 0xf57ef4 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_")]
pub fn stub_0xf57ef4() -> ! {
    todo!("0xf57ef4 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")
}

// 0xf57f04 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_")]
pub fn stub_0xf57f04() -> ! {
    todo!("0xf57f04 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")
}

// 0xf57f14 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
// type: int __fastcall(int, _Rb_tree_node_base *)
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")]
pub fn stub_0xf57f14() -> ! {
    todo!("0xf57f14 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")
}

// 0xf57f24 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_")]
pub fn stub_0xf57f24() -> ! {
    todo!("0xf57f24 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")
}

// 0xf57f34 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_")]
pub fn stub_0xf57f34() -> ! {
    todo!("0xf57f34 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")
}

// 0xf57f44 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_")]
pub fn stub_0xf57f44() -> ! {
    todo!("0xf57f44 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)")
}

// 0xf57f54 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
pub fn stub_0xf57f54() -> ! {
    todo!("0xf57f54 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)")
}

// 0xf57f64 — j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
// std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_")]
pub fn stub_0xf57f64() -> ! {
    todo!("0xf57f64 std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")
}

// 0xf57f74 — j___ZN3RBX10FixedArrayIN3G3D7Vector3ELm40EE9push_backERKS2_
// type: int __fastcall(_DWORD, _DWORD)
// RBX::FixedArray<G3D::Vector3,40ul>::push_back(G3D::Vector3 const&)
#[doc(alias = "RBX::FixedArray<G3D::Vector3,40ul>::push_back(G3D::Vector3 const&)")]
#[doc(alias = "j___ZN3RBX10FixedArrayIN3G3D7Vector3ELm40EE9push_backERKS2_")]
pub fn stub_0xf57f74() -> ! {
    todo!("0xf57f74 RBX::FixedArray<G3D::Vector3,40ul>::push_back(G3D::Vector3 const&)")
}

// 0xf57fb4 — j___ZN3RBX11CellContact18cellFaceIsInteriorINS_5Voxel4GridEEEbRKN3G3D12Vector3int16ENS2_13FaceDirectionE
// bool RBX::CellContact::cellFaceIsInterior<RBX::Voxel::Grid>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)
#[doc(alias = "bool RBX::CellContact::cellFaceIsInterior<RBX::Voxel::Grid>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)")]
#[doc(alias = "j___ZN3RBX11CellContact18cellFaceIsInteriorINS_5Voxel4GridEEEbRKN3G3D12Vector3int16ENS2_13FaceDirectionE")]
pub fn stub_0xf57fb4() -> ! {
    todo!("0xf57fb4 bool RBX::CellContact::cellFaceIsInterior<RBX::Voxel::Grid>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)")
}

// 0xf580b4 — j___ZNK3RBX10FixedArrayIN3G3D7Vector3ELm40EEixEm
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// RBX::FixedArray<G3D::Vector3,40ul>::operator[](unsigned long)const
#[doc(alias = "RBX::FixedArray<G3D::Vector3,40ul>::operator[](unsigned long)const")]
#[doc(alias = "j___ZNK3RBX10FixedArrayIN3G3D7Vector3ELm40EEixEm")]
pub fn stub_0xf580b4() -> ! {
    todo!("0xf580b4 RBX::FixedArray<G3D::Vector3,40ul>::operator[](unsigned long)const")
}

// 0xf58784 — j___ZNK3RBX4POLY4Face16pointInExtrusionERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this, const G3D::Vector3 *)
// RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const
#[doc(alias = "RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const")]
#[doc(alias = "j___ZNK3RBX4POLY4Face16pointInExtrusionERKN3G3D7Vector3E")]
pub fn stub_0xf58784() -> ! {
    todo!("0xf58784 RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const")
}

// 0xf59334 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector2EEEclES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
// rbx::signals::signal_with_args<1,void ()(G3D::Vector2)>::operator()(G3D::Vector2)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector2)>::operator()(G3D::Vector2)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector2EEEclES3_")]
pub fn stub_0xf59334() -> ! {
    todo!("0xf59334 rbx::signals::signal_with_args<1,void ()(G3D::Vector2)>::operator()(G3D::Vector2)")
}

// 0xf59394 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// rbx::signals::signal<void ()(G3D::Vector2)>::disconnectAll(void)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::disconnectAll(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13disconnectAllEv")]
pub fn stub_0xf59394() -> ! {
    todo!("0xf59394 rbx::signals::signal<void ()(G3D::Vector2)>::disconnectAll(void)")
}

// 0xf593a4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
// rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE24safe_static_do_get_mutexEv")]
pub fn stub_0xf593a4() -> ! {
    todo!("0xf593a4 rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_do_get_mutex(void)")
}

// 0xf593b4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// rbx::signals::signal<void ()(G3D::Vector2)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> &)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
pub fn stub_0xf593b4() -> ! {
    todo!("0xf593b4 rbx::signals::signal<void ()(G3D::Vector2)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> &)")
}

// 0xf593c4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
// rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0xf593c4() -> ! {
    todo!("0xf593c4 rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)")
}

// 0xf593d4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
// rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6insertEPNS5_4slotE")]
pub fn stub_0xf593d4() -> ! {
    todo!("0xf593d4 rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")
}

// 0xf593e4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
// rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6removeEPNS5_4slotE")]
pub fn stub_0xf593e4() -> ! {
    todo!("0xf593e4 rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")
}

// 0xf593f4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
// rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
pub fn stub_0xf593f4() -> ! {
    todo!("0xf593f4 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)")
}

// 0xf59404 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE8on_errorERSt9exception
// rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE8on_errorERSt9exception")]
pub fn stub_0xf59404() -> ! {
    todo!("0xf59404 rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)")
}

// 0xf59624 — j___ZN3rbx8any_castIRKN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
// G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "j___ZN3rbx8any_castIRKN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf59624() -> ! {
    todo!("0xf59624 G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf59654 — j___ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")]
pub fn stub_0xf59654() -> ! {
    todo!("0xf59654 rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)")
}

// 0xf596f4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSEPS8_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSEPS8_")]
pub fn stub_0xf596f4() -> ! {
    todo!("0xf596f4 boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)")
}

// 0xf59704 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSERKS9_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSERKS9_")]
pub fn stub_0xf59704() -> ! {
    todo!("0xf59704 boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)")
}

// 0xf5b524 — j___ZN3RBX5Voxel5Water12cellHasWaterINS0_4Grid5ChunkEEEbPKT_RKNS0_4CellERKN3G3D12Vector3int16E
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)
#[doc(alias = "bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)")]
#[doc(alias = "j___ZN3RBX5Voxel5Water12cellHasWaterINS0_4Grid5ChunkEEEbPKT_RKNS0_4CellERKN3G3D12Vector3int16E")]
pub fn stub_0xf5b524() -> ! {
    todo!("0xf5b524 bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)")
}

// 0xf5f8a4 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE6resizeEib")]
pub fn stub_0xf5f8a4() -> ! {
    todo!("0xf5f8a4 G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)")
}

// 0xf5f8b4 — j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE6appendERKS3_
// type: int __fastcall(_DWORD, _DWORD)
// G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE6appendERKS3_")]
pub fn stub_0xf5f8b4() -> ! {
    todo!("0xf5f8b4 G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)")
}

// 0xf5f8c4 — j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE7reallocEi
// type: int __fastcall(_DWORD)
// G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE7reallocEi")]
pub fn stub_0xf5f8c4() -> ! {
    todo!("0xf5f8c4 G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)")
}

// 0xf5f8e4 — j___ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::CompactCFrame *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
// RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "j___ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_")]
pub fn stub_0xf5f8e4() -> ! {
    todo!("0xf5f8e4 RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0xf5fb44 — j___ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_f
// type: _DWORD __fastcall(RBX::CompactCFrame *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, float)
// RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
#[doc(alias = "j___ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_f")]
pub fn stub_0xf5fb44() -> ! {
    todo!("0xf5fb44 RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)")
}

// 0xf5fb94 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE7reallocEi
// type: int __fastcall(_DWORD, _DWORD)
// G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE7reallocEi")]
pub fn stub_0xf5fb94() -> ! {
    todo!("0xf5fb94 G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)")
}

// 0xf5fba4 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EEC2Ev
// type: int __fastcall(_DWORD)
// G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EEC2Ev")]
pub fn stub_0xf5fba4() -> ! {
    todo!("0xf5fba4 G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)")
}

// 0xf5fbb4 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EED2Ev
// type: int __fastcall(_DWORD)
// G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EED2Ev")]
pub fn stub_0xf5fbb4() -> ! {
    todo!("0xf5fbb4 G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()")
}

// 0xf607c4 — j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6appendERKS3_
// type: int __fastcall(_DWORD, _DWORD)
// G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)")]
#[doc(alias = "j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6appendERKS3_")]
pub fn stub_0xf607c4() -> ! {
    todo!("0xf607c4 G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)")
}

// 0xf607d4 — j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)")]
#[doc(alias = "j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6resizeEib")]
pub fn stub_0xf607d4() -> ! {
    todo!("0xf607d4 G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)")
}

// 0xf607e4 — j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE7reallocEi
// type: int __fastcall(_DWORD, _DWORD)
// G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)")]
#[doc(alias = "j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE7reallocEi")]
pub fn stub_0xf607e4() -> ! {
    todo!("0xf607e4 G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)")
}

// 0xf60e34 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE5mutexEv
// type: int __fastcall(_DWORD)
// rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE5mutexEv")]
pub fn stub_0xf60e34() -> ! {
    todo!("0xf60e34 rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)")
}

// 0xf60e44 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
// rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6insertEPNS5_4slotE")]
pub fn stub_0xf60e44() -> ! {
    todo!("0xf60e44 rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")
}

// 0xf60e54 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
// rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6removeEPNS5_4slotE")]
pub fn stub_0xf60e54() -> ! {
    todo!("0xf60e54 rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")
}

// 0xf60f44 — j___ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs
// rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_E4callESs")]
pub fn stub_0xf60f44() -> ! {
    todo!("0xf60f44 rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::call(std::string)")
}

// 0xf60f54 — j___ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED2Ev
// type: int __fastcall(_DWORD)
// rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")]
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost8functionIS3_EELi1ES3_ED2Ev")]
pub fn stub_0xf60f54() -> ! {
    todo!("0xf60f54 rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::function<void ()(std::string)>,1,void ()(std::string)>::~callable()")
}

// 0xf60ff4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSERKS7_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> const&)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsEE4slotEEaSERKS7_")]
pub fn stub_0xf60ff4() -> ! {
    todo!("0xf60ff4 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string)>::slot> const&)")
}

// 0xf61004 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSEPS8_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSEPS8_")]
pub fn stub_0xf61004() -> ! {
    todo!("0xf61004 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)")
}

// 0xf61014 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSERKS9_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSERKS9_")]
pub fn stub_0xf61014() -> ! {
    todo!("0xf61014 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)")
}

// 0xf61034 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSEPS6_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot*)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSEPS6_")]
pub fn stub_0xf61034() -> ! {
    todo!("0xf61034 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot*)")
}

// 0xf61044 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSERKS7_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> const&)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsSsEE4slotEEaSERKS7_")]
pub fn stub_0xf61044() -> ! {
    todo!("0xf61044 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> const&)")
}

// 0xf61054 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSEPS6_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,int)>::slot*)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,int)>::slot*)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSEPS6_")]
pub fn stub_0xf61054() -> ! {
    todo!("0xf61054 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(rbx::signals::signal<void ()(bool,int)>::slot*)")
}

// 0xf61064 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSERKS7_
// type: int __fastcall(_DWORD, _DWORD)
// boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot> const&)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbiEE4slotEEaSERKS7_")]
pub fn stub_0xf61064() -> ! {
    todo!("0xf61064 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot> const&)")
}

// 0xf61174 — j___ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEEEC2ES5_S8_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>)")]
#[doc(alias = "j___ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEEEC2ES5_S8_")]
pub fn stub_0xf61174() -> ! {
    todo!("0xf61174 boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>)")
}

// 0xf611b4 — j___ZN5boost3_bi8storage3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// boost::_bi::storage3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")]
#[doc(alias = "j___ZN5boost3_bi8storage3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_")]
pub fn stub_0xf611b4() -> ! {
    todo!("0xf611b4 boost::_bi::storage3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")
}

// 0xf61274 — j___ZN5boost4bindIvRKNS_9function0IvEEN3RBX11MessageTypeEbS2_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list_av_3<boost::function0<void>,RBX::MessageType,bool>::type> boost::bind<void,boost::function0<void> const&,RBX::MessageType,bool,boost::function0<void>,RBX::MessageType,bool>(void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::function0<void>,RBX::MessageType,bool)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list_av_3<boost::function0<void>,RBX::MessageType,bool>::type> boost::bind<void,boost::function0<void> const&,RBX::MessageType,bool,boost::function0<void>,RBX::MessageType,bool>(void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::function0<void>,RBX::MessageType,bool)")]
#[doc(alias = "j___ZN5boost4bindIvRKNS_9function0IvEEN3RBX11MessageTypeEbS2_S6_bEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_")]
pub fn stub_0xf61274() -> ! {
    todo!("0xf61274 boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list_av_3<boost::function0<void>,RBX::MessageType,bool>::type> boost::bind<void,boost::function0<void> const&,RBX::MessageType,bool,boost::function0<void>,RBX::MessageType,bool>(void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::function0<void>,RBX::MessageType,bool)")
}

// 0xf61314 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf61314() -> ! {
    todo!("0xf61314 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf61394 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEEvT_
// type: int __fastcall(_DWORD, _DWORD)
// void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>)")]
#[doc(alias = "j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEEvT_")]
pub fn stub_0xf61394() -> ! {
    todo!("0xf61394 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>)")
}

// 0xf61704 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
// type: int(void)
// bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf61704() -> ! {
    todo!("0xf61704 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")
}

// 0xf61714 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, int, char, int, int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
// bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf61714() -> ! {
    todo!("0xf61714 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf617d4 — j___ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEE5eraseERSA_
// type: int __fastcall(_DWORD, _DWORD)
// std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(RBX::Name const* const&)
#[doc(alias = "std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(RBX::Name const* const&)")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameEPKNS0_8ICreatorESt4lessIS3_ESaISt4pairIKS3_S6_EEE5eraseERSA_")]
pub fn stub_0xf617d4() -> ! {
    todo!("0xf617d4 std::map<RBX::Name const*,RBX::ICreator const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(RBX::Name const* const&)")
}

// 0xf61834 — j___ZNSt4pairIKSsN3RBX10GuiBuilder4DataEED2Ev
// type: int __fastcall(_DWORD)
// std::pair<std::string const,RBX::GuiBuilder::Data>::~pair()
#[doc(alias = "std::pair<std::string const,RBX::GuiBuilder::Data>::~pair()")]
#[doc(alias = "j___ZNSt4pairIKSsN3RBX10GuiBuilder4DataEED2Ev")]
pub fn stub_0xf61834() -> ! {
    todo!("0xf61834 std::pair<std::string const,RBX::GuiBuilder::Data>::~pair()")
}

// 0xf619b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, char, void *, int, int, int, int, void *, int)
// std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_create_node(std::pair<std::string const,RBX::GuiBuilder::Data> const&)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_create_node(std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")]
pub fn stub_0xf619b4() -> ! {
    todo!("0xf619b4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_create_node(std::pair<std::string const,RBX::GuiBuilder::Data> const&)")
}

// 0xf619c4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
// std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>>*)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>>*)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_")]
pub fn stub_0xf619c4() -> ! {
    todo!("0xf619c4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>>*)")
}

// 0xf619d4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
// std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_0xf619d4() -> ! {
    todo!("0xf619d4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")
}

// 0xf62264 — j___ZN3rbx13remote_signalIFvSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// rbx::signals::connection rbx::remote_signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_")]
pub fn stub_0xf62264() -> ! {
    todo!("0xf62264 rbx::signals::connection rbx::remote_signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")
}

// 0xf62274 — j___ZN3rbx13remote_signalIFvSsEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// rbx::remote_signal<void ()(std::string)>::~remote_signal()
#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsEED1Ev")]
pub fn stub_0xf62274() -> ! {
    todo!("0xf62274 rbx::remote_signal<void ()(std::string)>::~remote_signal()")
}

// 0xf62284 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEE7connectIN5boost8functionIS3_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEE7connectIN5boost8functionIS3_EEEENS_7signals10connectionERKT_")]
pub fn stub_0xf62284() -> ! {
    todo!("0xf62284 rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")
}

// 0xf62294 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEEC2Ev
// type: int __fastcall(_DWORD)
// rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEEC2Ev")]
pub fn stub_0xf62294() -> ! {
    todo!("0xf62294 rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")
}

// 0xf622a4 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEED1Ev")]
pub fn stub_0xf622a4() -> ! {
    todo!("0xf622a4 rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")
}

// 0xf622b4 — j___ZN3rbx13remote_signalIFvSsSsSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
// rbx::signals::connection rbx::remote_signal<void ()(std::string,std::string,std::string)>::connect<boost::function<void ()(std::string,std::string,std::string)>>(boost::function<void ()(std::string,std::string,std::string)> const&)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,std::string,std::string)>::connect<boost::function<void ()(std::string,std::string,std::string)>>(boost::function<void ()(std::string,std::string,std::string)> const&)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsSsSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_")]
pub fn stub_0xf622b4() -> ! {
    todo!("0xf622b4 rbx::signals::connection rbx::remote_signal<void ()(std::string,std::string,std::string)>::connect<boost::function<void ()(std::string,std::string,std::string)>>(boost::function<void ()(std::string,std::string,std::string)> const&)")
}

// 0xf622c4 — j___ZN3rbx13remote_signalIFvSsSsSsEEC2Ev
// type: int __fastcall(_DWORD)
// rbx::remote_signal<void ()(std::string,std::string,std::string)>::remote_signal(void)
#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string,std::string)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsSsSsEEC2Ev")]
pub fn stub_0xf622c4() -> ! {
    todo!("0xf622c4 rbx::remote_signal<void ()(std::string,std::string,std::string)>::remote_signal(void)")
}

// 0xf622d4 — j___ZN3rbx13remote_signalIFvSsSsSsEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// rbx::remote_signal<void ()(std::string,std::string,std::string)>::~remote_signal()
#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string,std::string)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvSsSsSsEED1Ev")]
pub fn stub_0xf622d4() -> ! {
    todo!("0xf622d4 rbx::remote_signal<void ()(std::string,std::string,std::string)>::~remote_signal()")
}

// 0xf622f4 — j___ZN3rbx13remote_signalIFvbiEEC2Ev
// type: int __fastcall(_DWORD)
// rbx::remote_signal<void ()(bool,int)>::remote_signal(void)
#[doc(alias = "rbx::remote_signal<void ()(bool,int)>::remote_signal(void)")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvbiEEC2Ev")]
pub fn stub_0xf622f4() -> ! {
    todo!("0xf622f4 rbx::remote_signal<void ()(bool,int)>::remote_signal(void)")
}

// 0xf62304 — j___ZN3rbx13remote_signalIFvbiEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// rbx::remote_signal<void ()(bool,int)>::~remote_signal()
#[doc(alias = "rbx::remote_signal<void ()(bool,int)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvbiEED1Ev")]
pub fn stub_0xf62304() -> ! {
    todo!("0xf62304 rbx::remote_signal<void ()(bool,int)>::~remote_signal()")
}

// 0xf62324 — j___ZN3rbx13remote_signalIFvvEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
// rbx::remote_signal<void ()(void)>::~remote_signal()
#[doc(alias = "rbx::remote_signal<void ()(void)>::~remote_signal()")]
#[doc(alias = "j___ZN3rbx13remote_signalIFvvEED1Ev")]
pub fn stub_0xf62324() -> ! {
    todo!("0xf62324 rbx::remote_signal<void ()(void)>::~remote_signal()")
}

// 0xf630e4 — j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EEC2Ev
// type: int __fastcall(_DWORD)
// G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EEC2Ev")]
pub fn stub_0xf630e4() -> ! {
    todo!("0xf630e4 G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)")
}

// 0xf630f4 — j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EED2Ev
// G3D::Array<RBX::AssemblyItem *,10,32ul>::~Array()
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::~Array()")]
#[doc(alias = "j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EED2Ev")]
pub fn stub_0xf630f4() -> ! {
    todo!("0xf630f4 G3D::Array<RBX::AssemblyItem *,10,32ul>::~Array()")
}

// 0xf64c14 — j___ZN3RBX24FastClusterMeshGenerator9getBoundsERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::FastClusterMeshGenerator *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
// RBX::FastClusterMeshGenerator::getBounds(G3D::Vector3 const&,G3D::Vector3 const&)
#[doc(alias = "RBX::FastClusterMeshGenerator::getBounds(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "j___ZN3RBX24FastClusterMeshGenerator9getBoundsERKN3G3D7Vector3ES4_")]
pub fn stub_0xf64c14() -> ! {
    todo!("0xf64c14 RBX::FastClusterMeshGenerator::getBounds(G3D::Vector3 const&,G3D::Vector3 const&)")
}

// 0xf64ee4 — j___ZN3RBX16TrussQuadBuilder4emitERKN3G3D7Vector3ES4_fS4_f
// type: int __fastcall(RBX::TrussQuadBuilder *this, const Vector3 *, const Vector3 *, float32_t, const Vector3 *, float32_t)
// RBX::TrussQuadBuilder::emit(G3D::Vector3 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float)
#[doc(alias = "RBX::TrussQuadBuilder::emit(G3D::Vector3 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float)")]
#[doc(alias = "j___ZN3RBX16TrussQuadBuilder4emitERKN3G3D7Vector3ES4_fS4_f")]
pub fn stub_0xf64ee4() -> ! {
    todo!("0xf64ee4 RBX::TrussQuadBuilder::emit(G3D::Vector3 const&,G3D::Vector3 const&,float,G3D::Vector3 const&,float)")
}

// 0xf64f44 — j___ZN3RBX18getDecalUVVerticalILb0EEEN3G3D7Vector2EPNS_5DecalERKNS1_7Vector3Eb
// type: int __fastcall(_DWORD)
// G3D::Vector2 RBX::getDecalUVVertical<false>(RBX::Decal *,G3D::Vector3 const&,bool)
#[doc(alias = "G3D::Vector2 RBX::getDecalUVVertical<false>(RBX::Decal *,G3D::Vector3 const&,bool)")]
#[doc(alias = "j___ZN3RBX18getDecalUVVerticalILb0EEEN3G3D7Vector2EPNS_5DecalERKNS1_7Vector3Eb")]
pub fn stub_0xf64f44() -> ! {
    todo!("0xf64f44 G3D::Vector2 RBX::getDecalUVVertical<false>(RBX::Decal *,G3D::Vector3 const&,bool)")
}

// 0xf64f54 — j___ZN3RBX18getDecalUVVerticalILb1EEEN3G3D7Vector2EPNS_5DecalERKNS1_7Vector3Eb
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// G3D::Vector2 RBX::getDecalUVVertical<true>(RBX::Decal *,G3D::Vector3 const&,bool)
#[doc(alias = "G3D::Vector2 RBX::getDecalUVVertical<true>(RBX::Decal *,G3D::Vector3 const&,bool)")]
#[doc(alias = "j___ZN3RBX18getDecalUVVerticalILb1EEEN3G3D7Vector2EPNS_5DecalERKNS1_7Vector3Eb")]
pub fn stub_0xf64f54() -> ! {
    todo!("0xf64f54 G3D::Vector2 RBX::getDecalUVVertical<true>(RBX::Decal *,G3D::Vector3 const&,bool)")
}

// 0xf64f84 — j___ZN3RBX22TextureCompositorLayerC2ERKNS_6MeshIdERKN3G3D6Color3E
// type: std::string *__fastcall(std::string *, const std::string *, _DWORD *, int, char, int, int, int, struct _Unwind_Exception *lpuexcpt)
// RBX::TextureCompositorLayer::TextureCompositorLayer(RBX::MeshId const&,G3D::Color3 const&)
#[doc(alias = "RBX::TextureCompositorLayer::TextureCompositorLayer(RBX::MeshId const&,G3D::Color3 const&)")]
#[doc(alias = "j___ZN3RBX22TextureCompositorLayerC2ERKNS_6MeshIdERKN3G3D6Color3E")]
pub fn stub_0xf64f84() -> ! {
    todo!("0xf64f84 RBX::TextureCompositorLayer::TextureCompositorLayer(RBX::MeshId const&,G3D::Color3 const&)")
}

// 0xf64f94 — j___ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::pair(Ogre::TexturePtr const&,boost::shared_ptr<RBX::TextureCompositor::Job> const&)
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::pair(Ogre::TexturePtr const&,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")]
#[doc(alias = "j___ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEEC2ERKS1_RKS7_")]
pub fn stub_0xf64f94() -> ! {
    todo!("0xf64f94 std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::pair(Ogre::TexturePtr const&,boost::shared_ptr<RBX::TextureCompositor::Job> const&)")
}

// 0xf64fa4 — j___ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev
// std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::~pair()
#[doc(alias = "std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::~pair()")]
#[doc(alias = "j___ZNSt4pairIN4Ogre10TexturePtrEN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEEED1Ev")]
pub fn stub_0xf64fa4() -> ! {
    todo!("0xf64fa4 std::pair<Ogre::TexturePtr,boost::shared_ptr<RBX::TextureCompositor::Job>>::~pair()")
}

// 0xf64ff4 — j___ZN3RBX26ManualObjectMeshGenAdapterC2EPN4Ogre12ManualObjectE
// type: _DWORD __fastcall(RBX::ManualObjectMeshGenAdapter *__hidden this, Ogre::ManualObject *)
// RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)
#[doc(alias = "RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)")]
#[doc(alias = "j___ZN3RBX26ManualObjectMeshGenAdapterC2EPN4Ogre12ManualObjectE")]
pub fn stub_0xf64ff4() -> ! {
    todo!("0xf64ff4 RBX::ManualObjectMeshGenAdapter::ManualObjectMeshGenAdapter(Ogre::ManualObject *)")
}

// 0xf65074 — j___ZNSt3mapIN3RBX12Vector3int32EPN4Ogre9SceneNodeESt4lessIS1_ESaISt4pairIKS1_S4_EEEixERS8_
// type: int __fastcall(_DWORD, _DWORD)
// std::map<RBX::Vector3int32,Ogre::SceneNode *,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::operator[](RBX::Vector3int32 const&)
#[doc(alias = "std::map<RBX::Vector3int32,Ogre::SceneNode *,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::operator[](RBX::Vector3int32 const&)")]
#[doc(alias = "j___ZNSt3mapIN3RBX12Vector3int32EPN4Ogre9SceneNodeESt4lessIS1_ESaISt4pairIKS1_S4_EEEixERS8_")]
pub fn stub_0xf65074() -> ! {
    todo!("0xf65074 std::map<RBX::Vector3int32,Ogre::SceneNode *,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::operator[](RBX::Vector3int32 const&)")
}

// 0xf650b4 — j___ZNSt6vectorISt17_Rb_tree_iteratorISt4pairIKN3RBX12Vector3int32EPN4Ogre9SceneNodeEEESaIS9_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS9_SB_EERKS9_
// type: int __fastcall(_DWORD)
// std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>*,std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> const&)
#[doc(alias = "std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>*,std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> const&)")]
#[doc(alias = "j___ZNSt6vectorISt17_Rb_tree_iteratorISt4pairIKN3RBX12Vector3int32EPN4Ogre9SceneNodeEEESaIS9_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS9_SB_EERKS9_")]
pub fn stub_0xf650b4() -> ! {
    todo!("0xf650b4 std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>*,std::vector<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::allocator<std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> const&)")
}

// 0xf650c4 — j___ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)
#[doc(alias = "std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_0xf650c4() -> ! {
    todo!("0xf650c4 std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)")
}

// 0xf650d4 — j___ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
// std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)
#[doc(alias = "std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_0xf650d4() -> ! {
    todo!("0xf650d4 std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *> const&)")
}

// 0xf650e4 — j___ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
// std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> *)")]
#[doc(alias = "j___ZNSt8_Rb_treeIN3RBX12Vector3int32ESt4pairIKS1_PN4Ogre9SceneNodeEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_0xf650e4() -> ! {
    todo!("0xf650e4 std::_Rb_tree<RBX::Vector3int32,std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>,std::_Select1st<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>,std::less<RBX::Vector3int32>,std::allocator<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3int32 const,Ogre::SceneNode *>> *)")
}

// 0xf65184 — j___ZN3RBX15ScopedSingletonIN4Ogre11RootManagerEE4syncEv
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// RBX::ScopedSingleton<Ogre::RootManager>::sync(void)
#[doc(alias = "RBX::ScopedSingleton<Ogre::RootManager>::sync(void)")]
#[doc(alias = "j___ZN3RBX15ScopedSingletonIN4Ogre11RootManagerEE4syncEv")]
pub fn stub_0xf65184() -> ! {
    todo!("0xf65184 RBX::ScopedSingleton<Ogre::RootManager>::sync(void)")
}

// 0xf65494 — j___ZN3RBX11SpatialGridINS_11FastClusterEEC2ERKN3G3D7Vector3Ef
// RBX::SpatialGrid<RBX::FastCluster>::SpatialGrid(G3D::Vector3 const&,float)
#[doc(alias = "RBX::SpatialGrid<RBX::FastCluster>::SpatialGrid(G3D::Vector3 const&,float)")]
#[doc(alias = "j___ZN3RBX11SpatialGridINS_11FastClusterEEC2ERKN3G3D7Vector3Ef")]
pub fn stub_0xf65494() -> ! {
    todo!("0xf65494 RBX::SpatialGrid<RBX::FastCluster>::SpatialGrid(G3D::Vector3 const&,float)")
}

// 0xf65a94 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS5_11SpatialNodeE
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE10returnNodeEPNS5_11SpatialNodeE")]
pub fn stub_0xf65a94() -> ! {
    todo!("0xf65a94 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::returnNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")
}

// 0xf65aa4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS5_11SpatialNodeE
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE11destroyNodeEPNS5_11SpatialNodeE")]
pub fn stub_0xf65aa4() -> ! {
    todo!("0xf65aa4 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::destroyNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::SpatialNode *)")
}

// 0xf65ab4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE12findTreeNodeEiiRKNS_12Vector3int32E")]
pub fn stub_0xf65ab4() -> ! {
    todo!("0xf65ab4 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::findTreeNode(int,int,RBX::Vector3int32 const&)")
}

// 0xf65ac4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E
// type: int __fastcall(int, int, int, int)
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14createTreeNodeEiiRKNS_12Vector3int32E")]
pub fn stub_0xf65ac4() -> ! {
    todo!("0xf65ac4 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::createTreeNode(int,int,RBX::Vector3int32 const&)")
}

// 0xf65ad4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS2_b
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(Ogre::RbxCullableSceneNode*,bool)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(Ogre::RbxCullableSceneNode*,bool)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE14primitiveAddedEPS2_b")]
pub fn stub_0xf65ad4() -> ! {
    todo!("0xf65ad4 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveAdded(Ogre::RbxCullableSceneNode*,bool)")
}

// 0xf65ae4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS5_8TreeNodeE
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode *)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE15_retireTreeNodeEPNS5_8TreeNodeE")]
pub fn stub_0xf65ae4() -> ! {
    todo!("0xf65ae4 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::_retireTreeNode(RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::TreeNode *)")
}

// 0xf65af4 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS2_b
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(Ogre::RbxCullableSceneNode*,bool)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(Ogre::RbxCullableSceneNode*,bool)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16onPrimitiveAddedEPS2_b")]
pub fn stub_0xf65af4() -> ! {
    todo!("0xf65af4 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::onPrimitiveAdded(Ogre::RbxCullableSceneNode*,bool)")
}

// 0xf65b04 — j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS2_
// type: int(void)
// RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(Ogre::RbxCullableSceneNode*)
#[doc(alias = "RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(Ogre::RbxCullableSceneNode*)")]
#[doc(alias = "j___ZN3RBX11SpatialHashIN4Ogre20RbxCullableSceneNodeENS_7ContactENS_14ContactManagerELi4EE16primitiveRemovedEPS2_")]
pub fn stub_0xf65b04() -> ! {
    todo!("0xf65b04 RBX::SpatialHash<Ogre::RbxCullableSceneNode,RBX::Contact,RBX::ContactManager,4>::primitiveRemoved(Ogre::RbxCullableSceneNode*)")
}

