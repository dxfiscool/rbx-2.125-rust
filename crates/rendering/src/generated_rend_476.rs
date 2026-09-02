//! rendering shard 476 — 120 stubs 0x7505f4..0x756560 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 51610->51730 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc after shard 475 (0x7505f4..0x756560)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// 0x7505f4 — __ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_
#[doc(alias = "RBX::Vector3_2IntsComparer::operator()(RBX::Vector3_2Ints const&,RBX::Vector3_2Ints const&)const")]
#[doc(alias = "__ZNK3RBX21Vector3_2IntsComparerclERKNS_13Vector3_2IntsES3_")]
pub fn stub_7505f4() -> ! {
    todo!("0x7505f4 RBX::Vector3_2IntsComparer::operator()(RBX::Vector3_2Ints const&,RBX::Vector3_2Ints const&)const")
}
// 0x75072c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
pub fn stub_75072c() -> ! {
    todo!("0x75072c std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")
}
// 0x75080c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
pub fn stub_75080c() -> ! {
    todo!("0x75080c std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")
}
// 0x750870 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
pub fn stub_750870() -> ! {
    todo!("0x750870 std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")
}
// 0x7508f0 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
pub fn stub_7508f0() -> ! {
    todo!("0x7508f0 RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")
}
// 0x7509b0 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm")]
pub fn stub_7509b0() -> ! {
    todo!("0x7509b0 RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long)")
}
// 0x750a20 — __ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE
#[doc(alias = "RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE")]
pub fn stub_750a20() -> ! {
    todo!("0x750a20 RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&)")
}
// 0x750b70 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev")]
pub fn stub_750b70() -> ! {
    todo!("0x750b70 RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator(void)")
}
// 0x750bd4 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv")]
pub fn stub_750bd4() -> ! {
    todo!("0x750bd4 RBX::Allocator<RBX::POLY::PyramidMesh>::releaseMemory(void)")
}
// 0x750c58 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
pub fn stub_750c58() -> ! {
    todo!("0x750c58 std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")
}
// 0x750ca8 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")]
pub fn stub_750ca8() -> ! {
    todo!("0x750ca8 RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")
}
// 0x750cac — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
pub fn stub_750cac() -> ! {
    todo!("0x750cac RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")
}
// 0x750dbc — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")]
pub fn stub_750dbc() -> ! {
    todo!("0x750dbc RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")
}
// 0x750e6c — __GLOBAL__I_a_343
#[doc(alias = "global constructor keyed to_a_343")]
#[doc(alias = "__GLOBAL__I_a_343")]
pub fn stub_750e6c() -> ! {
    todo!("0x750e6c global constructor keyed to'_a_343")
}
// 0x750f9c — __ZN3RBX18RightAngleRampPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX18RightAngleRampPoly9buildMeshEv")]
pub fn stub_750f9c() -> ! {
    todo!("0x750f9c RBX::RightAngleRampPoly::buildMesh(void)")
}
// 0x751078 — __ZNK3RBX18RightAngleRampPoly9getMomentEf
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this, float)
#[doc(alias = "RBX::RightAngleRampPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly9getMomentEf")]
pub fn stub_751078() -> ! {
    todo!("0x751078 RBX::RightAngleRampPoly::getMoment(float)const")
}
// 0x7511ac — __ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv")]
pub fn stub_7511ac() -> ! {
    todo!("0x7511ac RBX::RightAngleRampPoly::getCofmOffset(void)const")
}
// 0x7511e0 — __ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::RightAngleRampPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
pub fn stub_7511e0() -> ! {
    todo!("0x7511e0 RBX::RightAngleRampPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")
}
// 0x751398 — __ZN3RBX18RightAngleRampPolyD1Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly()")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD1Ev")]
pub fn stub_751398() -> ! {
    todo!("0x751398 RBX::RightAngleRampPoly::~RightAngleRampPoly()")
}
// 0x7513bc — __ZN3RBX18RightAngleRampPolyD0Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly()")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD0Ev")]
pub fn stub_7513bc() -> ! {
    todo!("0x7513bc RBX::RightAngleRampPoly::~RightAngleRampPoly()")
}
// 0x751a00 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv")]
pub fn stub_751a00() -> ! {
    todo!("0x751a00 RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void *)")
}
// 0x75203c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm")]
pub fn stub_75203c() -> ! {
    todo!("0x75203c RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long)")
}
// 0x7521c8 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev")]
pub fn stub_7521c8() -> ! {
    todo!("0x7521c8 RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator(void)")
}
// 0x75222c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv")]
pub fn stub_75222c() -> ! {
    todo!("0x75222c RBX::Allocator<RBX::POLY::RightAngleRampMesh>::releaseMemory(void)")
}
// 0x752518 — __GLOBAL__I_a_344
#[doc(alias = "global constructor keyed to_a_344")]
#[doc(alias = "__GLOBAL__I_a_344")]
pub fn stub_752518() -> ! {
    todo!("0x752518 global constructor keyed to'_a_344")
}
// 0x752648 — __ZN3RBX10RigidJoint9isAlignedEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
#[doc(alias = "RBX::RigidJoint::isAligned(void)")]
#[doc(alias = "__ZN3RBX10RigidJoint9isAlignedEv")]
pub fn stub_752648() -> ! {
    todo!("0x752648 RBX::RigidJoint::isAligned(void)")
}
// 0x752720 — __ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RigidJoint::align(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_")]
pub fn stub_752720() -> ! {
    todo!("0x752720 RBX::RigidJoint::align(RBX::Primitive *,RBX::Primitive *)")
}
// 0x752884 — __ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RigidJoint::getChildInParent(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_")]
pub fn stub_752884() -> ! {
    todo!("0x752884 RBX::RigidJoint::getChildInParent(RBX::Primitive *,RBX::Primitive *)")
}
// 0x752b14 — __ZNK3RBX10RigidJoint8isBrokenEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
#[doc(alias = "RBX::RigidJoint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX10RigidJoint8isBrokenEv")]
pub fn stub_752b14() -> ! {
    todo!("0x752b14 RBX::RigidJoint::isBroken(void)const")
}
// 0x752b18 — __GLOBAL__I_a_345
#[doc(alias = "global constructor keyed to_a_345")]
#[doc(alias = "__GLOBAL__I_a_345")]
pub fn stub_752b18() -> ! {
    todo!("0x752b18 global constructor keyed to'_a_345")
}
// 0x752be0 — __ZN3RBX11RotateJointC1Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::RotateJoint(void)")]
#[doc(alias = "__ZN3RBX11RotateJointC1Ev")]
pub fn stub_752be0() -> ! {
    todo!("0x752be0 RBX::RotateJoint::RotateJoint(void)")
}
// 0x752c04 — __ZN3RBX11RotateJointC2Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::RotateJoint(void)")]
#[doc(alias = "__ZN3RBX11RotateJointC2Ev")]
pub fn stub_752c04() -> ! {
    todo!("0x752c04 RBX::RotateJoint::RotateJoint(void)")
}
// 0x752c28 — __ZN3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZN3RBX11RotateJointD0Ev")]
pub fn stub_752c28() -> ! {
    todo!("0x752c28 RBX::RotateJoint::~RotateJoint()")
}
// 0x752cc8 — __ZN3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZN3RBX11RotateJointD1Ev")]
pub fn stub_752cc8() -> ! {
    todo!("0x752cc8 RBX::RotateJoint::~RotateJoint()")
}
// 0x752ccc — __ZThn32_N3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD0Ev")]
pub fn stub_752ccc() -> ! {
    todo!("0x752ccc non-virtual thunk toRBX::RotateJoint::~RotateJoint()")
}
// 0x752cd4 — __ZThn32_N3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD1Ev")]
pub fn stub_752cd4() -> ! {
    todo!("0x752cd4 non-virtual thunk toRBX::RotateJoint::~RotateJoint()")
}
// 0x752cdc — __ZN3RBX11RotateJoint21getAxleWorldDirectionEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getAxleWorldDirection(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint21getAxleWorldDirectionEv")]
pub fn stub_752cdc() -> ! {
    todo!("0x752cdc RBX::RotateJoint::getAxleWorldDirection(void)")
}
// 0x752cfc — __ZN3RBX11RotateJoint15getAxleVelocityEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getAxleVelocity(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint15getAxleVelocityEv")]
pub fn stub_752cfc() -> ! {
    todo!("0x752cfc RBX::RotateJoint::getAxleVelocity(void)")
}
// 0x752fe8 — __ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::RotateJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
pub fn stub_752fe8() -> ! {
    todo!("0x752fe8 RBX::RotateJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")
}
// 0x7537c4 — __ZN3RBX11RotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint16removeFromKernelEv")]
pub fn stub_7537c4() -> ! {
    todo!("0x7537c4 RBX::RotateJoint::removeFromKernel(void)")
}
// 0x753828 — __ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, float *, float *)
#[doc(alias = "RBX::RotateJoint::getPrimitivesTorqueArmLength(float &,float &)")]
#[doc(alias = "__ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_")]
pub fn stub_753828() -> ! {
    todo!("0x753828 RBX::RotateJoint::getPrimitivesTorqueArmLength(float &,float &)")
}
// 0x753910 — __ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::RotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE")]
pub fn stub_753910() -> ! {
    todo!("0x753910 RBX::RotateJoint::putInKernel(RBX::Kernel *)")
}
// 0x753c00 — __ZN3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD0Ev")]
pub fn stub_753c00() -> ! {
    todo!("0x753c00 RBX::DynamicRotateJoint::~DynamicRotateJoint()")
}
// 0x753ca0 — __ZN3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD1Ev")]
pub fn stub_753ca0() -> ! {
    todo!("0x753ca0 RBX::DynamicRotateJoint::~DynamicRotateJoint()")
}
// 0x753ca4 — __ZThn32_N3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD0Ev")]
pub fn stub_753ca4() -> ! {
    todo!("0x753ca4 non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")
}
// 0x753cac — __ZN3RBX18DynamicRotateJointD2Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD2Ev")]
pub fn stub_753cac() -> ! {
    todo!("0x753cac RBX::DynamicRotateJoint::~DynamicRotateJoint()")
}
// 0x753dd4 — __ZThn32_N3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD1Ev")]
pub fn stub_753dd4() -> ! {
    todo!("0x753dd4 non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")
}
// 0x753ddc — __ZN3RBX18DynamicRotateJoint10setPhysicsEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::setPhysics(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint10setPhysicsEv")]
pub fn stub_753ddc() -> ! {
    todo!("0x753ddc RBX::DynamicRotateJoint::setPhysics(void)")
}
// 0x753dec — __ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::DynamicRotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE")]
pub fn stub_753dec() -> ! {
    todo!("0x753dec RBX::DynamicRotateJoint::putInKernel(RBX::Kernel *)")
}
// 0x753f94 — __ZN3RBX18DynamicRotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint16removeFromKernelEv")]
pub fn stub_753f94() -> ! {
    todo!("0x753f94 RBX::DynamicRotateJoint::removeFromKernel(void)")
}
// 0x754060 — __ZN3RBX18DynamicRotateJoint6stepUiEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
#[doc(alias = "RBX::DynamicRotateJoint::stepUi(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint6stepUiEd")]
pub fn stub_754060() -> ! {
    todo!("0x754060 RBX::DynamicRotateJoint::stepUi(double)")
}
// 0x75409c — __ZN3RBX18DynamicRotateJoint15getChannelValueEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
#[doc(alias = "RBX::DynamicRotateJoint::getChannelValue(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint15getChannelValueEd")]
pub fn stub_75409c() -> ! {
    todo!("0x75409c RBX::DynamicRotateJoint::getChannelValue(double)")
}
// 0x754170 — __ZN3RBX12RotatePJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotatePJoint9stepWorldEv")]
pub fn stub_754170() -> ! {
    todo!("0x754170 RBX::RotatePJoint::stepWorld(void)")
}
// 0x754184 — __ZN3RBX12RotateVJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotateVJoint9stepWorldEv")]
pub fn stub_754184() -> ! {
    todo!("0x754184 RBX::RotateVJoint::stepWorld(void)")
}
// 0x754198 — __ZNK3RBX11RotateJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX11RotateJoint12getJointTypeEv")]
pub fn stub_754198() -> ! {
    todo!("0x754198 RBX::RotateJoint::getJointType(void)const")
}
// 0x75419c — __ZNK3RBX18DynamicRotateJoint12canStepWorldEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::canStepWorld(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint12canStepWorldEv")]
pub fn stub_75419c() -> ! {
    todo!("0x75419c RBX::DynamicRotateJoint::canStepWorld(void)const")
}
// 0x7541a0 — __ZNK3RBX18DynamicRotateJoint9canStepUiEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint9canStepUiEv")]
pub fn stub_7541a0() -> ! {
    todo!("0x7541a0 RBX::DynamicRotateJoint::canStepUi(void)const")
}
// 0x7541a4 — __ZN3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZN3RBX12RotatePJointD1Ev")]
pub fn stub_7541a4() -> ! {
    todo!("0x7541a4 RBX::RotatePJoint::~RotatePJoint()")
}
// 0x7541a8 — __ZN3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZN3RBX12RotatePJointD0Ev")]
pub fn stub_7541a8() -> ! {
    todo!("0x7541a8 RBX::RotatePJoint::~RotatePJoint()")
}
// 0x754248 — __ZNK3RBX12RotatePJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX12RotatePJoint12getJointTypeEv")]
pub fn stub_754248() -> ! {
    todo!("0x754248 RBX::RotatePJoint::getJointType(void)const")
}
// 0x75424c — __ZThn32_N3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotatePJointD1Ev")]
pub fn stub_75424c() -> ! {
    todo!("0x75424c non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")
}
// 0x754254 — __ZThn32_N3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotatePJointD0Ev")]
pub fn stub_754254() -> ! {
    todo!("0x754254 non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")
}
// 0x7542f8 — __ZN3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZN3RBX12RotateVJointD1Ev")]
pub fn stub_7542f8() -> ! {
    todo!("0x7542f8 RBX::RotateVJoint::~RotateVJoint()")
}
// 0x7542fc — __ZN3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZN3RBX12RotateVJointD0Ev")]
pub fn stub_7542fc() -> ! {
    todo!("0x7542fc RBX::RotateVJoint::~RotateVJoint()")
}
// 0x75439c — __ZNK3RBX12RotateVJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX12RotateVJoint12getJointTypeEv")]
pub fn stub_75439c() -> ! {
    todo!("0x75439c RBX::RotateVJoint::getJointType(void)const")
}
// 0x7543a0 — __ZThn32_N3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotateVJointD1Ev")]
pub fn stub_7543a0() -> ! {
    todo!("0x7543a0 non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")
}
// 0x7543a8 — __ZThn32_N3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotateVJointD0Ev")]
pub fn stub_7543a8() -> ! {
    todo!("0x7543a8 non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")
}
// 0x75444c — __ZNK3RBX14JointConnector22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::JointConnector *__hidden this)
#[doc(alias = "RBX::JointConnector::getConnectorKernelType(void)const")]
#[doc(alias = "__ZNK3RBX14JointConnector22getConnectorKernelTypeEv")]
pub fn stub_75444c() -> ! {
    todo!("0x75444c RBX::JointConnector::getConnectorKernelType(void)const")
}
// 0x754450 — __ZN3RBX26PointToPointBreakConnectorD1Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnectorD1Ev")]
pub fn stub_754450() -> ! {
    todo!("0x754450 RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")
}
// 0x754454 — __ZN3RBX26PointToPointBreakConnectorD0Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnectorD0Ev")]
pub fn stub_754454() -> ! {
    todo!("0x754454 RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")
}
// 0x754458 — __ZN3RBX26PointToPointBreakConnector9getBrokenEv
// type: _DWORD __fastcall(RBX::PointToPointBreakConnector *__hidden this)
#[doc(alias = "RBX::PointToPointBreakConnector::getBroken(void)")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnector9getBrokenEv")]
pub fn stub_754458() -> ! {
    todo!("0x754458 RBX::PointToPointBreakConnector::getBroken(void)")
}
// 0x754460 — __GLOBAL__I_a_346
#[doc(alias = "global constructor keyed to_a_346")]
#[doc(alias = "__GLOBAL__I_a_346")]
pub fn stub_754460() -> ! {
    todo!("0x754460 global constructor keyed to'_a_346")
}
// 0x75462c — __ZN3RBX11SendPhysicsC1Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
#[doc(alias = "__ZN3RBX11SendPhysicsC1Ev")]
pub fn stub_75462c() -> ! {
    todo!("0x75462c RBX::SendPhysics::SendPhysics(void)")
}
// 0x754630 — __ZN3RBX11SendPhysicsC2Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
#[doc(alias = "__ZN3RBX11SendPhysicsC2Ev")]
pub fn stub_754630() -> ! {
    todo!("0x754630 RBX::SendPhysics::SendPhysics(void)")
}
// 0x754824 — __ZN3RBX11SendPhysicsD1Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
#[doc(alias = "__ZN3RBX11SendPhysicsD1Ev")]
pub fn stub_754824() -> ! {
    todo!("0x754824 RBX::SendPhysics::~SendPhysics()")
}
// 0x754828 — __ZN3RBX11SendPhysicsD2Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
#[doc(alias = "__ZN3RBX11SendPhysicsD2Ev")]
pub fn stub_754828() -> ! {
    todo!("0x754828 RBX::SendPhysics::~SendPhysics()")
}
// 0x754abc — __ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SendPhysics::buildSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE")]
pub fn stub_754abc() -> ! {
    todo!("0x754abc RBX::SendPhysics::buildSimJob(RBX::SimJob *)")
}
// 0x754b34 — __ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SendPhysics::destroySimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE")]
pub fn stub_754b34() -> ! {
    todo!("0x754b34 RBX::SendPhysics::destroySimJob(RBX::SimJob *)")
}
// 0x754bd0 — __ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE")]
pub fn stub_754bd0() -> ! {
    todo!("0x754bd0 RBX::SendPhysics::onMovingAssemblyRootAdded(RBX::Assembly *)")
}
// 0x754d1c — __ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE")]
pub fn stub_754d1c() -> ! {
    todo!("0x754d1c RBX::SendPhysics::onMovingAssemblyRootRemoving(RBX::Assembly *)")
}
// 0x754e00 — __ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SendPhysics::nextSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE")]
pub fn stub_754e00() -> ! {
    todo!("0x754e00 RBX::SendPhysics::nextSimJob(RBX::SimJob *)")
}
// 0x754e74 — __GLOBAL__I_a_347
#[doc(alias = "global constructor keyed to_a_347")]
#[doc(alias = "__GLOBAL__I_a_347")]
pub fn stub_754e74() -> ! {
    todo!("0x754e74 global constructor keyed to'_a_347")
}
// 0x754f3c — __ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::SimJob::getConstSimJobFromPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE")]
pub fn stub_754f3c() -> ! {
    todo!("0x754f3c RBX::SimJob::getConstSimJobFromPrimitive(RBX::Primitive const*)")
}
// 0x754f54 — __ZN3RBX13SimJobTracker12stopTrackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
#[doc(alias = "RBX::SimJobTracker::stopTracking(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker12stopTrackingEv")]
pub fn stub_754f54() -> ! {
    todo!("0x754f54 RBX::SimJobTracker::stopTracking(void)")
}
// 0x755034 — __ZN3RBX13SimJobTracker8trackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
#[doc(alias = "RBX::SimJobTracker::tracking(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker8trackingEv")]
pub fn stub_755034() -> ! {
    todo!("0x755034 RBX::SimJobTracker::tracking(void)")
}
// 0x7550bc — __ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SimJobTracker::setSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE")]
pub fn stub_7550bc() -> ! {
    todo!("0x7550bc RBX::SimJobTracker::setSimJob(RBX::SimJob *)")
}
// 0x7551a8 — __ZN3RBX13SimJobTracker9getSimJobEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
#[doc(alias = "RBX::SimJobTracker::getSimJob(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker9getSimJobEv")]
pub fn stub_7551a8() -> ! {
    todo!("0x7551a8 RBX::SimJobTracker::getSimJob(void)")
}
// 0x755264 — __ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *, RBX::SimJob *)
#[doc(alias = "RBX::SimJobTracker::transferTrackers(RBX::SimJob *,RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_")]
pub fn stub_755264() -> ! {
    todo!("0x755264 RBX::SimJobTracker::transferTrackers(RBX::SimJob *,RBX::SimJob *)")
}
// 0x755310 — __ZN3RBX6SimJobC1EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX6SimJobC1EPNS_8AssemblyE")]
pub fn stub_755310() -> ! {
    todo!("0x755310 RBX::SimJob::SimJob(RBX::Assembly *)")
}
// 0x755314 — __ZN3RBX6SimJobC2EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX6SimJobC2EPNS_8AssemblyE")]
pub fn stub_755314() -> ! {
    todo!("0x755314 RBX::SimJob::SimJob(RBX::Assembly *)")
}
// 0x755424 — __ZN3RBX6SimJobD1Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
#[doc(alias = "RBX::SimJob::~SimJob()")]
#[doc(alias = "__ZN3RBX6SimJobD1Ev")]
pub fn stub_755424() -> ! {
    todo!("0x755424 RBX::SimJob::~SimJob()")
}
// 0x755428 — __ZN3RBX6SimJobD2Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
#[doc(alias = "RBX::SimJob::~SimJob()")]
#[doc(alias = "__ZN3RBX6SimJobD2Ev")]
pub fn stub_755428() -> ! {
    todo!("0x755428 RBX::SimJob::~SimJob()")
}
// 0x755580 — __ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_
#[doc(alias = "unsigned long RBX::fastRemoveShort<RBX::SimJobTracker *>(std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>> &,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_")]
pub fn stub_755580() -> ! {
    todo!("0x755580 unsigned long RBX::fastRemoveShort<RBX::SimJobTracker *>(std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>> &,RBX::SimJobTracker * const&)")
}
// 0x7556dc — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::push_back(RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_")]
pub fn stub_7556dc() -> ! {
    todo!("0x7556dc std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::push_back(RBX::SimJobTracker * const&)")
}
// 0x755708 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_755708() -> ! {
    todo!("0x755708 std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&)")
}
// 0x7557e8 — __ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm")]
pub fn stub_7557e8() -> ! {
    todo!("0x7557e8 std::_Vector_base<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_allocate(unsigned long)")
}
// 0x755800 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::resize(unsigned long,RBX::SimJobTracker *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_")]
pub fn stub_755800() -> ! {
    todo!("0x755800 std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::resize(unsigned long,RBX::SimJobTracker *)")
}
// 0x755834 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,unsigned long,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_755834() -> ! {
    todo!("0x755834 std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,unsigned long,RBX::SimJobTracker * const&)")
}
// 0x75599c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker *>(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")]
pub fn stub_75599c() -> ! {
    todo!("0x75599c __gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker *>(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&,std::random_access_iterator_tag)")
}
// 0x755a2c — __GLOBAL__I_a_348
#[doc(alias = "global constructor keyed to_a_348")]
#[doc(alias = "__GLOBAL__I_a_348")]
pub fn stub_755a2c() -> ! {
    todo!("0x755a2c global constructor keyed to'_a_348")
}
// 0x755af4 — __ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_755af4() -> ! {
    todo!("0x755af4 RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")
}
// 0x755af8 — __ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_755af8() -> ! {
    todo!("0x755af8 RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")
}
// 0x755bf0 — __ZN3RBX13SimulateStageD0Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD0Ev")]
pub fn stub_755bf0() -> ! {
    todo!("0x755bf0 RBX::SimulateStage::~SimulateStage()")
}
// 0x755c90 — __ZN3RBX13SimulateStageD1Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD1Ev")]
pub fn stub_755c90() -> ! {
    todo!("0x755c90 RBX::SimulateStage::~SimulateStage()")
}
// 0x755c94 — __ZN3RBX13SimulateStageD2Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD2Ev")]
pub fn stub_755c94() -> ! {
    todo!("0x755c94 RBX::SimulateStage::~SimulateStage()")
}
// 0x755f34 — __ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::onAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE")]
pub fn stub_755f34() -> ! {
    todo!("0x755f34 RBX::SimulateStage::onAssemblyAdded(RBX::Assembly *)")
}
// 0x756070 — __ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::putFirstMovingRootInSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE")]
pub fn stub_756070() -> ! {
    todo!("0x756070 RBX::SimulateStage::putFirstMovingRootInSendPhysics(RBX::Assembly *)")
}
// 0x756130 — __ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::onAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE")]
pub fn stub_756130() -> ! {
    todo!("0x756130 RBX::SimulateStage::onAssemblyRemoving(RBX::Assembly *)")
}
// 0x7561ac — __ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::removeLastMovingRootFromSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE")]
pub fn stub_7561ac() -> ! {
    todo!("0x7561ac RBX::SimulateStage::removeLastMovingRootFromSendPhysics(RBX::Assembly *)")
}
// 0x75627c — __ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::removeFromSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE")]
pub fn stub_75627c() -> ! {
    todo!("0x75627c RBX::SimulateStage::removeFromSendPhysics(RBX::Assembly *)")
}
// 0x7562f8 — __ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SimulateStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE")]
pub fn stub_7562f8() -> ! {
    todo!("0x7562f8 RBX::SimulateStage::onEdgeAdded(RBX::Edge *)")
}
// 0x756320 — __ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SimulateStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE")]
pub fn stub_756320() -> ! {
    todo!("0x756320 RBX::SimulateStage::onEdgeRemoving(RBX::Edge *)")
}
// 0x75633c — __ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v
#[doc(alias = "RBX::Assembly * RBX::IndexedTree::getOneBelowRoot<RBX::Assembly>(void)")]
#[doc(alias = "__ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v")]
pub fn stub_75633c() -> ! {
    todo!("0x75633c RBX::Assembly * RBX::IndexedTree::getOneBelowRoot<RBX::Assembly>(void)")
}
// 0x7563a8 — __ZNK3RBX13SimulateStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX13SimulateStage12getStageTypeEv")]
pub fn stub_7563a8() -> ! {
    todo!("0x7563a8 RBX::SimulateStage::getStageType(void)const")
}
// 0x7563ac — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert_unique(std::pair<RBX::Assembly * const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")]
pub fn stub_7563ac() -> ! {
    todo!("0x7563ac std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert_unique(std::pair<RBX::Assembly * const,int> const&)")
}
// 0x756414 — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Assembly * const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
pub fn stub_756414() -> ! {
    todo!("0x756414 std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Assembly * const,int> const&)")
}
// 0x75646c — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,int>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
pub fn stub_75646c() -> ! {
    todo!("0x75646c std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,int>> *)")
}
// 0x756494 — __GLOBAL__I_a_349
#[doc(alias = "global constructor keyed to_a_349")]
#[doc(alias = "__GLOBAL__I_a_349")]
pub fn stub_756494() -> ! {
    todo!("0x756494 global constructor keyed to'_a_349")
}
// 0x75655c — __ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE")]
pub fn stub_75655c() -> ! {
    todo!("0x75655c RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")
}
// 0x756560 — __ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE")]
pub fn stub_756560() -> ! {
    todo!("0x756560 RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")
}
