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
// IDA 0x7505f4: 44 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7505f4() {
}
// 0x75072c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
// IDA 0x75072c: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75072c() {
}
// 0x75080c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
// IDA 0x75080c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75080c() {
}
// 0x750870 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
// IDA 0x750870: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750870() {
}
// 0x7508f0 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
// IDA 0x7508f0: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7508f0() {
}
// 0x7509b0 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEnwEm")]
// IDA 0x7509b0: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7509b0() {
}
// 0x750a20 — __ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE
#[doc(alias = "RBX::POLY::PyramidMesh::PyramidMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX4POLY11PyramidMeshC2ERKNS_13Vector3_2IntsE")]
// IDA 0x750a20: 120 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750a20() {
}
// 0x750b70 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEC2Ev")]
// IDA 0x750b70: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750b70() {
}
// 0x750bd4 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEE13releaseMemoryEv")]
// IDA 0x750bd4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750bd4() {
}
// 0x750c58 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
// IDA 0x750c58: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750c58() {
}
// 0x750ca8 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")]
// IDA 0x750ca8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_750ca8() {
}
// 0x750cac — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
// IDA 0x750cac: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750cac() {
}
// 0x750dbc — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")]
// IDA 0x750dbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_750dbc() {
}
// 0x750e6c — __GLOBAL__I_a_343
#[doc(alias = "global constructor keyed to_a_343")]
#[doc(alias = "__GLOBAL__I_a_343")]
// IDA 0x750e6c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_750e6c() {
}
// 0x750f9c — __ZN3RBX18RightAngleRampPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX18RightAngleRampPoly9buildMeshEv")]
// IDA 0x750f9c: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750f9c() {
}
// 0x751078 — __ZNK3RBX18RightAngleRampPoly9getMomentEf
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this, float)
#[doc(alias = "RBX::RightAngleRampPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly9getMomentEf")]
// IDA 0x751078: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_751078() {
}
// 0x7511ac — __ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly13getCofmOffsetEv")]
// IDA 0x7511ac: 17 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7511ac() {
}
// 0x7511e0 — __ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::RightAngleRampPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// IDA 0x7511e0: 9 insns (SUBS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7511e0() {
}
// 0x751398 — __ZN3RBX18RightAngleRampPolyD1Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly()")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD1Ev")]
// IDA 0x751398: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_751398() {
}
// 0x7513bc — __ZN3RBX18RightAngleRampPolyD0Ev
// type: void __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::~RightAngleRampPoly()")]
#[doc(alias = "__ZN3RBX18RightAngleRampPolyD0Ev")]
// IDA 0x7513bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7513bc() {
}
// 0x751a00 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEdlEPv")]
// IDA 0x751a00: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_751a00() {
}
// 0x75203c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEnwEm")]
// IDA 0x75203c: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75203c() {
}
// 0x7521c8 — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEEC2Ev")]
// IDA 0x7521c8: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7521c8() {
}
// 0x75222c — __ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::RightAngleRampMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY18RightAngleRampMeshEE13releaseMemoryEv")]
// IDA 0x75222c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75222c() {
}
// 0x752518 — __GLOBAL__I_a_344
#[doc(alias = "global constructor keyed to_a_344")]
#[doc(alias = "__GLOBAL__I_a_344")]
// IDA 0x752518: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_752518() {
}
// 0x752648 — __ZN3RBX10RigidJoint9isAlignedEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
#[doc(alias = "RBX::RigidJoint::isAligned(void)")]
#[doc(alias = "__ZN3RBX10RigidJoint9isAlignedEv")]
// IDA 0x752648: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752648() {
}
// 0x752720 — __ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RigidJoint::align(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint5alignEPNS_9PrimitiveES2_")]
// IDA 0x752720: 125 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752720() {
}
// 0x752884 — __ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::RigidJoint::getChildInParent(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10RigidJoint16getChildInParentEPNS_9PrimitiveES2_")]
// IDA 0x752884: 123 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752884() {
}
// 0x752b14 — __ZNK3RBX10RigidJoint8isBrokenEv
// type: _DWORD __fastcall(RBX::RigidJoint *__hidden this)
#[doc(alias = "RBX::RigidJoint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX10RigidJoint8isBrokenEv")]
// IDA 0x752b14: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752b14() {
}
// 0x752b18 — __GLOBAL__I_a_345
#[doc(alias = "global constructor keyed to_a_345")]
#[doc(alias = "__GLOBAL__I_a_345")]
// IDA 0x752b18: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_752b18() {
}
// 0x752be0 — __ZN3RBX11RotateJointC1Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::RotateJoint(void)")]
#[doc(alias = "__ZN3RBX11RotateJointC1Ev")]
// IDA 0x752be0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752be0() {
}
// 0x752c04 — __ZN3RBX11RotateJointC2Ev
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::RotateJoint(void)")]
#[doc(alias = "__ZN3RBX11RotateJointC2Ev")]
// IDA 0x752c04: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752c04() {
}
// 0x752c28 — __ZN3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZN3RBX11RotateJointD0Ev")]
// IDA 0x752c28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_752c28() {
}
// 0x752cc8 — __ZN3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZN3RBX11RotateJointD1Ev")]
// IDA 0x752cc8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_752cc8() {
}
// 0x752ccc — __ZThn32_N3RBX11RotateJointD0Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD0Ev")]
// IDA 0x752ccc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_752ccc() {
}
// 0x752cd4 — __ZThn32_N3RBX11RotateJointD1Ev
// type: void __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateJoint::~RotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX11RotateJointD1Ev")]
// IDA 0x752cd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_752cd4() {
}
// 0x752cdc — __ZN3RBX11RotateJoint21getAxleWorldDirectionEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getAxleWorldDirection(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint21getAxleWorldDirectionEv")]
// IDA 0x752cdc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752cdc() {
}
// 0x752cfc — __ZN3RBX11RotateJoint15getAxleVelocityEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getAxleVelocity(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint15getAxleVelocityEv")]
// IDA 0x752cfc: 46 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752cfc() {
}
// 0x752fe8 — __ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::RotateJoint::canBuildJoint(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX11RotateJoint13canBuildJointEPNS_9PrimitiveES2_NS_8NormalIdES3_")]
// IDA 0x752fe8: 519 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752fe8() {
}
// 0x7537c4 — __ZN3RBX11RotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX11RotateJoint16removeFromKernelEv")]
// IDA 0x7537c4: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7537c4() {
}
// 0x753828 — __ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, float *, float *)
#[doc(alias = "RBX::RotateJoint::getPrimitivesTorqueArmLength(float &,float &)")]
#[doc(alias = "__ZN3RBX11RotateJoint28getPrimitivesTorqueArmLengthERfS1_")]
// IDA 0x753828: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_753828() {
}
// 0x753910 — __ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::RotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX11RotateJoint11putInKernelEPNS_6KernelE")]
// IDA 0x753910: 233 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_753910() {
}
// 0x753c00 — __ZN3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD0Ev")]
// IDA 0x753c00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_753c00() {
}
// 0x753ca0 — __ZN3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD1Ev")]
// IDA 0x753ca0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_753ca0() {
}
// 0x753ca4 — __ZThn32_N3RBX18DynamicRotateJointD0Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD0Ev")]
// IDA 0x753ca4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_753ca4() {
}
// 0x753cac — __ZN3RBX18DynamicRotateJointD2Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZN3RBX18DynamicRotateJointD2Ev")]
// IDA 0x753cac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_753cac() {
}
// 0x753dd4 — __ZThn32_N3RBX18DynamicRotateJointD1Ev
// type: void __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::DynamicRotateJoint::~DynamicRotateJoint()")]
#[doc(alias = "__ZThn32_N3RBX18DynamicRotateJointD1Ev")]
// IDA 0x753dd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_753dd4() {
}
// 0x753ddc — __ZN3RBX18DynamicRotateJoint10setPhysicsEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::setPhysics(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint10setPhysicsEv")]
// IDA 0x753ddc: 5 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_753ddc() {
}
// 0x753dec — __ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, RBX::Kernel *)
#[doc(alias = "RBX::DynamicRotateJoint::putInKernel(RBX::Kernel *)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint11putInKernelEPNS_6KernelE")]
// IDA 0x753dec: 138 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_753dec() {
}
// 0x753f94 — __ZN3RBX18DynamicRotateJoint16removeFromKernelEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::removeFromKernel(void)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint16removeFromKernelEv")]
// IDA 0x753f94: 64 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_753f94() {
}
// 0x754060 — __ZN3RBX18DynamicRotateJoint6stepUiEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
#[doc(alias = "RBX::DynamicRotateJoint::stepUi(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint6stepUiEd")]
// IDA 0x754060: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754060() {
}
// 0x75409c — __ZN3RBX18DynamicRotateJoint15getChannelValueEd
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this, double)
#[doc(alias = "RBX::DynamicRotateJoint::getChannelValue(double)")]
#[doc(alias = "__ZN3RBX18DynamicRotateJoint15getChannelValueEd")]
// IDA 0x75409c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75409c() {
}
// 0x754170 — __ZN3RBX12RotatePJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotatePJoint9stepWorldEv")]
// IDA 0x754170: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754170() {
}
// 0x754184 — __ZN3RBX12RotateVJoint9stepWorldEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX12RotateVJoint9stepWorldEv")]
// IDA 0x754184: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754184() {
}
// 0x754198 — __ZNK3RBX11RotateJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateJoint *__hidden this)
#[doc(alias = "RBX::RotateJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX11RotateJoint12getJointTypeEv")]
// IDA 0x754198: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754198() {
}
// 0x75419c — __ZNK3RBX18DynamicRotateJoint12canStepWorldEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::canStepWorld(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint12canStepWorldEv")]
// IDA 0x75419c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75419c() {
}
// 0x7541a0 — __ZNK3RBX18DynamicRotateJoint9canStepUiEv
// type: _DWORD __fastcall(RBX::DynamicRotateJoint *__hidden this)
#[doc(alias = "RBX::DynamicRotateJoint::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX18DynamicRotateJoint9canStepUiEv")]
// IDA 0x7541a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7541a0() {
}
// 0x7541a4 — __ZN3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZN3RBX12RotatePJointD1Ev")]
// IDA 0x7541a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7541a4() {
}
// 0x7541a8 — __ZN3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZN3RBX12RotatePJointD0Ev")]
// IDA 0x7541a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7541a8() {
}
// 0x754248 — __ZNK3RBX12RotatePJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "RBX::RotatePJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX12RotatePJoint12getJointTypeEv")]
// IDA 0x754248: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754248() {
}
// 0x75424c — __ZThn32_N3RBX12RotatePJointD1Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotatePJointD1Ev")]
// IDA 0x75424c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75424c() {
}
// 0x754254 — __ZThn32_N3RBX12RotatePJointD0Ev
// type: void __fastcall(RBX::RotatePJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotatePJoint::~RotatePJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotatePJointD0Ev")]
// IDA 0x754254: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_754254() {
}
// 0x7542f8 — __ZN3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZN3RBX12RotateVJointD1Ev")]
// IDA 0x7542f8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7542f8() {
}
// 0x7542fc — __ZN3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZN3RBX12RotateVJointD0Ev")]
// IDA 0x7542fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7542fc() {
}
// 0x75439c — __ZNK3RBX12RotateVJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "RBX::RotateVJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX12RotateVJoint12getJointTypeEv")]
// IDA 0x75439c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75439c() {
}
// 0x7543a0 — __ZThn32_N3RBX12RotateVJointD1Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotateVJointD1Ev")]
// IDA 0x7543a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7543a0() {
}
// 0x7543a8 — __ZThn32_N3RBX12RotateVJointD0Ev
// type: void __fastcall(RBX::RotateVJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::RotateVJoint::~RotateVJoint()")]
#[doc(alias = "__ZThn32_N3RBX12RotateVJointD0Ev")]
// IDA 0x7543a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7543a8() {
}
// 0x75444c — __ZNK3RBX14JointConnector22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::JointConnector *__hidden this)
#[doc(alias = "RBX::JointConnector::getConnectorKernelType(void)const")]
#[doc(alias = "__ZNK3RBX14JointConnector22getConnectorKernelTypeEv")]
// IDA 0x75444c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75444c() {
}
// 0x754450 — __ZN3RBX26PointToPointBreakConnectorD1Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnectorD1Ev")]
// IDA 0x754450: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_754450() {
}
// 0x754454 — __ZN3RBX26PointToPointBreakConnectorD0Ev
// type: void __fastcall(RBX::PointToPointBreakConnector *__hidden this)
#[doc(alias = "RBX::PointToPointBreakConnector::~PointToPointBreakConnector()")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnectorD0Ev")]
// IDA 0x754454: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_754454() {
}
// 0x754458 — __ZN3RBX26PointToPointBreakConnector9getBrokenEv
// type: _DWORD __fastcall(RBX::PointToPointBreakConnector *__hidden this)
#[doc(alias = "RBX::PointToPointBreakConnector::getBroken(void)")]
#[doc(alias = "__ZN3RBX26PointToPointBreakConnector9getBrokenEv")]
// IDA 0x754458: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754458() {
}
// 0x754460 — __GLOBAL__I_a_346
#[doc(alias = "global constructor keyed to_a_346")]
#[doc(alias = "__GLOBAL__I_a_346")]
// IDA 0x754460: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_754460() {
}
// 0x75462c — __ZN3RBX11SendPhysicsC1Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
#[doc(alias = "__ZN3RBX11SendPhysicsC1Ev")]
// IDA 0x75462c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_75462c() {
}
// 0x754630 — __ZN3RBX11SendPhysicsC2Ev
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::SendPhysics(void)")]
#[doc(alias = "__ZN3RBX11SendPhysicsC2Ev")]
// IDA 0x754630: 199 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754630() {
}
// 0x754824 — __ZN3RBX11SendPhysicsD1Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
#[doc(alias = "__ZN3RBX11SendPhysicsD1Ev")]
// IDA 0x754824: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_754824() {
}
// 0x754828 — __ZN3RBX11SendPhysicsD2Ev
// type: void __fastcall(RBX::SendPhysics *__hidden this)
#[doc(alias = "RBX::SendPhysics::~SendPhysics()")]
#[doc(alias = "__ZN3RBX11SendPhysicsD2Ev")]
// IDA 0x754828: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_754828() {
}
// 0x754abc — __ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SendPhysics::buildSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics11buildSimJobEPNS_6SimJobE")]
// IDA 0x754abc: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754abc() {
}
// 0x754b34 — __ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SendPhysics::destroySimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics13destroySimJobEPNS_6SimJobE")]
// IDA 0x754b34: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754b34() {
}
// 0x754bd0 — __ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX11SendPhysics25onMovingAssemblyRootAddedEPNS_8AssemblyE")]
// IDA 0x754bd0: 120 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754bd0() {
}
// 0x754d1c — __ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SendPhysics::onMovingAssemblyRootRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX11SendPhysics28onMovingAssemblyRootRemovingEPNS_8AssemblyE")]
// IDA 0x754d1c: 82 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754d1c() {
}
// 0x754e00 — __ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SendPhysics *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SendPhysics::nextSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX11SendPhysics10nextSimJobEPNS_6SimJobE")]
// IDA 0x754e00: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754e00() {
}
// 0x754e74 — __GLOBAL__I_a_347
#[doc(alias = "global constructor keyed to_a_347")]
#[doc(alias = "__GLOBAL__I_a_347")]
// IDA 0x754e74: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_754e74() {
}
// 0x754f3c — __ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, const RBX::Primitive *)
#[doc(alias = "RBX::SimJob::getConstSimJobFromPrimitive(RBX::Primitive const*)")]
#[doc(alias = "__ZN3RBX6SimJob27getConstSimJobFromPrimitiveEPKNS_9PrimitiveE")]
// IDA 0x754f3c: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754f3c() {
}
// 0x754f54 — __ZN3RBX13SimJobTracker12stopTrackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
#[doc(alias = "RBX::SimJobTracker::stopTracking(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker12stopTrackingEv")]
// IDA 0x754f54: 80 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_754f54() {
}
// 0x755034 — __ZN3RBX13SimJobTracker8trackingEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
#[doc(alias = "RBX::SimJobTracker::tracking(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker8trackingEv")]
// IDA 0x755034: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755034() {
}
// 0x7550bc — __ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *)
#[doc(alias = "RBX::SimJobTracker::setSimJob(RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX13SimJobTracker9setSimJobEPNS_6SimJobE")]
// IDA 0x7550bc: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7550bc() {
}
// 0x7551a8 — __ZN3RBX13SimJobTracker9getSimJobEv
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this)
#[doc(alias = "RBX::SimJobTracker::getSimJob(void)")]
#[doc(alias = "__ZN3RBX13SimJobTracker9getSimJobEv")]
// IDA 0x7551a8: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7551a8() {
}
// 0x755264 — __ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_
// type: _DWORD __fastcall(RBX::SimJobTracker *__hidden this, RBX::SimJob *, RBX::SimJob *)
#[doc(alias = "RBX::SimJobTracker::transferTrackers(RBX::SimJob *,RBX::SimJob *)")]
#[doc(alias = "__ZN3RBX13SimJobTracker16transferTrackersEPNS_6SimJobES2_")]
// IDA 0x755264: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755264() {
}
// 0x755310 — __ZN3RBX6SimJobC1EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX6SimJobC1EPNS_8AssemblyE")]
// IDA 0x755310: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_755310() {
}
// 0x755314 — __ZN3RBX6SimJobC2EPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimJob *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimJob::SimJob(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX6SimJobC2EPNS_8AssemblyE")]
// IDA 0x755314: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755314() {
}
// 0x755424 — __ZN3RBX6SimJobD1Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
#[doc(alias = "RBX::SimJob::~SimJob()")]
#[doc(alias = "__ZN3RBX6SimJobD1Ev")]
// IDA 0x755424: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_755424() {
}
// 0x755428 — __ZN3RBX6SimJobD2Ev
// type: void __fastcall(RBX::SimJob *__hidden this)
#[doc(alias = "RBX::SimJob::~SimJob()")]
#[doc(alias = "__ZN3RBX6SimJobD2Ev")]
// IDA 0x755428: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_755428() {
}
// 0x755580 — __ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_
#[doc(alias = "unsigned long RBX::fastRemoveShort<RBX::SimJobTracker *>(std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>> &,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZN3RBX15fastRemoveShortIPNS_13SimJobTrackerEEEmRSt6vectorIT_SaIS4_EERKS4_")]
// IDA 0x755580: 108 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755580() {
}
// 0x7556dc — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::push_back(RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE9push_backERKS2_")]
// IDA 0x7556dc: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_7556dc() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}
// 0x755708 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// IDA 0x755708: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_755708() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}
// 0x7557e8 — __ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIPN3RBX13SimJobTrackerESaIS2_EE11_M_allocateEm")]
// IDA 0x7557e8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_7557e8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}
// 0x755800 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::resize(unsigned long,RBX::SimJobTracker *)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE6resizeEmS2_")]
// IDA 0x755800: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755800() {
}
// 0x755834 — __ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,unsigned long,RBX::SimJobTracker * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX13SimJobTrackerESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// IDA 0x755834: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755834() {
}
// 0x75599c — __ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker *>(__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,__gnu_cxx::__normal_iterator<RBX::SimJobTracker **,std::vector<RBX::SimJobTracker *,std::allocator<RBX::SimJobTracker *>>>,RBX::SimJobTracker * const&,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt6__findIN9__gnu_cxx17__normal_iteratorIPPN3RBX13SimJobTrackerESt6vectorIS4_SaIS4_EEEES4_ET_SA_SA_RKT0_St26random_access_iterator_tag")]
// IDA 0x75599c: 59 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75599c() {
}
// 0x755a2c — __GLOBAL__I_a_348
#[doc(alias = "global constructor keyed to_a_348")]
#[doc(alias = "__GLOBAL__I_a_348")]
// IDA 0x755a2c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_755a2c() {
}
// 0x755af4 — __ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SimulateStageC1EPNS_6IStageEPNS_5WorldE")]
// IDA 0x755af4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_755af4() {
}
// 0x755af8 — __ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SimulateStage::SimulateStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX13SimulateStageC2EPNS_6IStageEPNS_5WorldE")]
// IDA 0x755af8: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755af8() {
}
// 0x755bf0 — __ZN3RBX13SimulateStageD0Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD0Ev")]
// IDA 0x755bf0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_755bf0() {
}
// 0x755c90 — __ZN3RBX13SimulateStageD1Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD1Ev")]
// IDA 0x755c90: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_755c90() {
}
// 0x755c94 — __ZN3RBX13SimulateStageD2Ev
// type: void __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::~SimulateStage()")]
#[doc(alias = "__ZN3RBX13SimulateStageD2Ev")]
// IDA 0x755c94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_755c94() {
}
// 0x755f34 — __ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::onAssemblyAdded(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage15onAssemblyAddedEPNS_8AssemblyE")]
// IDA 0x755f34: 111 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_755f34() {
}
// 0x756070 — __ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::putFirstMovingRootInSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage31putFirstMovingRootInSendPhysicsEPNS_8AssemblyE")]
// IDA 0x756070: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_756070() {
}
// 0x756130 — __ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::onAssemblyRemoving(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage18onAssemblyRemovingEPNS_8AssemblyE")]
// IDA 0x756130: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_756130() {
}
// 0x7561ac — __ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::removeLastMovingRootFromSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage35removeLastMovingRootFromSendPhysicsEPNS_8AssemblyE")]
// IDA 0x7561ac: 70 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7561ac() {
}
// 0x75627c — __ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::SimulateStage::removeFromSendPhysics(RBX::Assembly *)")]
#[doc(alias = "__ZN3RBX13SimulateStage21removeFromSendPhysicsEPNS_8AssemblyE")]
// IDA 0x75627c: 48 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75627c() {
}
// 0x7562f8 — __ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SimulateStage::onEdgeAdded(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX13SimulateStage11onEdgeAddedEPNS_4EdgeE")]
// IDA 0x7562f8: 18 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7562f8() {
}
// 0x756320 — __ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this, RBX::Edge *)
#[doc(alias = "RBX::SimulateStage::onEdgeRemoving(RBX::Edge *)")]
#[doc(alias = "__ZN3RBX13SimulateStage14onEdgeRemovingEPNS_4EdgeE")]
// IDA 0x756320: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_756320() {
}
// 0x75633c — __ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v
#[doc(alias = "RBX::Assembly * RBX::IndexedTree::getOneBelowRoot<RBX::Assembly>(void)")]
#[doc(alias = "__ZN3RBX11IndexedTree15getOneBelowRootINS_8AssemblyEEEPT_v")]
// IDA 0x75633c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75633c() {
}
// 0x7563a8 — __ZNK3RBX13SimulateStage12getStageTypeEv
// type: _DWORD __fastcall(RBX::SimulateStage *__hidden this)
#[doc(alias = "RBX::SimulateStage::getStageType(void)const")]
#[doc(alias = "__ZNK3RBX13SimulateStage12getStageTypeEv")]
// IDA 0x7563a8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7563a8() {
}
// 0x7563ac — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert_unique(std::pair<RBX::Assembly * const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_")]
// IDA 0x7563ac: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7563ac() {
}
// 0x756414 — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Assembly * const,int> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
// IDA 0x756414: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_756414() {
}
// 0x75646c — __ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<RBX::Assembly *,std::pair<RBX::Assembly * const,int>,std::_Select1st<std::pair<RBX::Assembly * const,int>>,std::less<RBX::Assembly *>,std::allocator<std::pair<RBX::Assembly * const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Assembly * const,int>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPN3RBX8AssemblyESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
// IDA 0x75646c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75646c() {
}
// 0x756494 — __GLOBAL__I_a_349
#[doc(alias = "global constructor keyed to_a_349")]
#[doc(alias = "__GLOBAL__I_a_349")]
// IDA 0x756494: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_756494() {
}
// 0x75655c — __ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10SleepStageC1EPNS_6IStageEPNS_5WorldE")]
// IDA 0x75655c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_75655c() {
}
// 0x756560 — __ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE
// type: _DWORD __fastcall(RBX::SleepStage *__hidden this, RBX::IStage *, RBX::World *)
#[doc(alias = "RBX::SleepStage::SleepStage(RBX::IStage *,RBX::World *)")]
#[doc(alias = "__ZN3RBX10SleepStageC2EPNS_6IStageEPNS_5WorldE")]
// IDA 0x756560: 476 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_756560() {
}
