//! rendering — generated_watchdog_renderA — 100 stubs Ogre|G3D|Render EA-sorted asc, globally deduped via /tmp/global_eas.txt
//! Source: ida/export.json (85545 funcs) filter Ogre|G3D|Render|CRenderSettings|WorldModel, skip EAs in /tmp/global_eas.txt — next 100 EA-sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x87b488 — __ZNK3RBX15MegaClusterPoly26findTouchingSurfacesConvexERKN3G3D15CoordinateFrameERmRKNS_8GeometryES4_S5_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::CoordinateFrame *, unsigned int *, const RBX::Geometry *, const G3D::CoordinateFrame *, unsigned int *)
#[doc(alias = "RBX::MegaClusterPoly::findTouchingSurfacesConvex(G3D::CoordinateFrame const&,unsigned long &,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly26findTouchingSurfacesConvexERKN3G3D15CoordinateFrameERmRKNS_8GeometryES4_S5_")]
// IDA 0x87b488: 113 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87b488() {
}

// 0x87b5bc — __ZNK3RBX15MegaClusterPoly35findCellsTouchingGeometryWithBufferERKfRKN3G3D15CoordinateFrameERKNS_8GeometryES6_PSt3mapIiPNS3_12Vector3int16ESt4lessIiESaISt4pairIKiSC_EEE
// type: int __fastcall(int, int, int, int, G3D::CoordinateFrame *, int)
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometryWithBuffer(float const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly35findCellsTouchingGeometryWithBufferERKfRKN3G3D15CoordinateFrameERKNS_8GeometryES6_PSt3mapIiPNS3_12Vector3int16ESt4lessIiESaISt4pairIKiSC_EEE")]
// IDA 0x87b5bc: 135 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87b5bc() {
}

// 0x87b784 — __ZNK3RBX15MegaClusterPoly25findPlanarTouchesWithGeomERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::MegaClusterPoly::findPlanarTouchesWithGeom(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly25findPlanarTouchesWithGeomERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE")]
// IDA 0x87b784: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87b784() {
}

// 0x87b828 — __ZNK3RBX15MegaClusterPoly22hasPlanarTouchWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3int16 *, const G3D::CoordinateFrame *, const RBX::Geometry *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hasPlanarTouchWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly22hasPlanarTouchWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_")]
// IDA 0x87b828: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87b828() {
}

// 0x87b874 — __ZNK3RBX15MegaClusterPoly28findCellIntersectionWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_Rm
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3int16 *, const G3D::CoordinateFrame *, const RBX::Geometry *, const G3D::CoordinateFrame *, unsigned int *)
#[doc(alias = "RBX::MegaClusterPoly::findCellIntersectionWithGeom(G3D::Vector3int16 const&,G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,unsigned long &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly28findCellIntersectionWithGeomERKN3G3D12Vector3int16ERKNS1_15CoordinateFrameERKNS_8GeometryES7_Rm")]
// IDA 0x87b874: 500 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87b874() {
}

// 0x87be18 — __ZNK3RBX15MegaClusterPoly28hitLocationOnCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, const int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly28hitLocationOnCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87be18: 477 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87be18() {
}

// 0x87c450 — __ZNK3RBX15MegaClusterPoly32hitLocationOnHorizontalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: int __fastcall(RBX::MegaClusterPoly *this, const RBX::RbxRay *, const G3D::Vector3int16 *, int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnHorizontalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly32hitLocationOnHorizontalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87c450: 593 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87c450() {
}

// 0x87cc0c — __ZNK3RBX15MegaClusterPoly30hitLocationOnVerticalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: int __fastcall(RBX::MegaClusterPoly *this, const RBX::RbxRay *, const G3D::Vector3int16 *, int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnVerticalWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly30hitLocationOnVerticalWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87cc0c: 599 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87cc0c() {
}

// 0x87d3e0 — __ZNK3RBX15MegaClusterPoly35hitLocationOnInverseCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, const int *, G3D::Vector3 *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnInverseCornerWedgeCell(RBX::RbxRay const&,G3D::Vector3int16 const&,int const&,G3D::Vector3 &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly35hitLocationOnInverseCornerWedgeCellERKNS_6RbxRayERKN3G3D12Vector3int16ERKiRNS4_7Vector3ERNS4_15CoordinateFrameE")]
// IDA 0x87d3e0: 783 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87d3e0() {
}

// 0x87de28 — __ZNK3RBX15MegaClusterPoly22hitLocationOnBlockCellERKNS_6RbxRayERKN3G3D12Vector3int16ERNS4_7Vector3ERiRNS4_15CoordinateFrameE
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const RBX::RbxRay *, const G3D::Vector3int16 *, G3D::Vector3 *, int *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::MegaClusterPoly::hitLocationOnBlockCell(RBX::RbxRay const&,G3D::Vector3int16 const&,G3D::Vector3 &,int &,G3D::CoordinateFrame &)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly22hitLocationOnBlockCellERKNS_6RbxRayERKN3G3D12Vector3int16ERNS4_7Vector3ERiRNS4_15CoordinateFrameE")]
// IDA 0x87de28: 704 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87de28() {
}

// 0x87e738 — __ZNK3RBX15MegaClusterPoly25findCellsTouchingGeometryERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE
// type: int __fastcall(int, int, int, G3D::CoordinateFrame *, int)
#[doc(alias = "RBX::MegaClusterPoly::findCellsTouchingGeometry(G3D::CoordinateFrame const&,RBX::Geometry const&,G3D::CoordinateFrame const&,std::map<int,G3D::Vector3int16 *,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>> *)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly25findCellsTouchingGeometryERKN3G3D15CoordinateFrameERKNS_8GeometryES4_PSt3mapIiPNS1_12Vector3int16ESt4lessIiESaISt4pairIKiSA_EEE")]
// IDA 0x87e738: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87e738() {
}

// 0x87e758 — __ZN3RBX15MegaClusterPoly18cellsInBoundingBoxERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::MegaClusterPoly::cellsInBoundingBox(G3D::Vector3 const&,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15MegaClusterPoly18cellsInBoundingBoxERKN3G3D7Vector3ES4_")]
// IDA 0x87e758: 620 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87e758() {
}

// 0x87edfc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE8getTokenERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE8getTokenERKS2_")]
// IDA 0x87edfc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87edfc() {
}

// 0x87ef60 — __ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_5Voxel4GridEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS7_15CoordinateFrameEfRNS_6CellIDEbb
// type: int __fastcall(int, int, int, int, int, G3D::CoordinateFrame *, float, int, int, int)
#[doc(alias = "bool RBX::MegaClusterPoly::hitTestMC_templated<RBX::Voxel::Grid>(RBX::RbxRay const&,G3D::Vector3 &,bool &,int &,G3D::CoordinateFrame &,float,RBX::CellID &,bool,bool)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly19hitTestMC_templatedINS_5Voxel4GridEEEbRKNS_6RbxRayERN3G3D7Vector3ERbRiRNS7_15CoordinateFrameEfRNS_6CellIDEbb")]
// IDA 0x87ef60: 500 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x87ef60() {
}

// 0x88004c — __ZN3RBX24getRegionForCellLocationINS_5Voxel4GridEEEKNT_6RegionEPKS3_RKN3G3D12Vector3int16EPS4_
// type: int __fastcall(int, int, G3D::Vector3int16 *this)
#[doc(alias = "RBX::Voxel::Grid::Region const RBX::getRegionForCellLocation<RBX::Voxel::Grid>(RBX::Voxel::Grid::Region const*,G3D::Vector3int16 const&,RBX::Voxel::Grid::Region const*)")]
#[doc(alias = "__ZN3RBX24getRegionForCellLocationINS_5Voxel4GridEEEKNT_6RegionEPKS3_RKN3G3D12Vector3int16EPS4_")]
// IDA 0x88004c: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88004c() {
}

// 0x8800ec — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15MegaClusterMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_")]
// IDA 0x8800ec: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8800ec() {
}

// 0x880344 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE")]
// IDA 0x880344: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880344() {
}

// 0x880520 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountD2Ev")]
// IDA 0x880520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x880520() {
}

// 0x8806b8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_")]
// IDA 0x8806b8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8806b8() {
}

// 0x8806e0 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_")]
// IDA 0x8806e0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8806e0() {
}

// 0x880740 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E")]
// IDA 0x880740: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880740() {
}

// 0x880768 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_")]
// IDA 0x880768: 34 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880768() {
}

// 0x8807c4 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_")]
// IDA 0x8807c4: 34 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8807c4() {
}

// 0x8808d8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_")]
// IDA 0x8808d8: 147 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8808d8() {
}

// 0x880a80 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_")]
// IDA 0x880a80: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880a80() {
}

// 0x880b24 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_")]
// IDA 0x880b24: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880b24() {
}

// 0x880bf8 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountC2ERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10ValueCountC2ERKS2_")]
// IDA 0x880bf8: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880bf8() {
}

// 0x880d28 — __ZN3RBX4POLY15MegaClusterMeshC2ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::POLY::MegaClusterMesh *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::POLY::MegaClusterMesh::MegaClusterMesh(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4POLY15MegaClusterMeshC2ERKN3G3D7Vector3E")]
// IDA 0x880d28: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880d28() {
}

// 0x880f24 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15MegaClusterMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_")]
// IDA 0x880f24: 57 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880f24() {
}

// 0x880fc8 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv")]
// IDA 0x880fc8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x880fc8() {
}

// 0x880fcc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv")]
// IDA 0x880fcc: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x880fcc() {
}

// 0x8810dc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::MegaClusterMesh,RBX::Vector3Comparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15MegaClusterMeshENS_15Vector3ComparerEE10StaticDataD1Ev")]
// IDA 0x8810dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x8810dc() {
}

// 0x8815fc — __ZN3RBX15PolyCellContactC1EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
// type: _DWORD __fastcall(RBX::PolyCellContact *__hidden this, RBX::Primitive *, RBX::Primitive *, const G3D::Vector3int16 *)
#[doc(alias = "RBX::PolyCellContact::PolyCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX15PolyCellContactC1EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E")]
// IDA 0x8815fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x8815fc() {
}

// 0x881600 — __ZN3RBX15PolyCellContactC2EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
// type: _DWORD __fastcall(RBX::PolyCellContact *__hidden this, RBX::Primitive *, RBX::Primitive *, const G3D::Vector3int16 *)
#[doc(alias = "RBX::PolyCellContact::PolyCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX15PolyCellContactC2EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E")]
// IDA 0x881600: 233 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x881600() {
}

// 0x881f48 — __ZN3RBX16CellFaceFacePair15computeVerticesERNS_10FixedArrayIN3G3D7Vector3ELm40EEERKNS2_15CoordinateFrameE
#[doc(alias = "RBX::CellFaceFacePair::computeVertices(RBX::FixedArray<G3D::Vector3,40ul> &,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair15computeVerticesERNS_10FixedArrayIN3G3D7Vector3ELm40EEERKNS2_15CoordinateFrameE")]
// IDA 0x881f48: 101 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x881f48() {
}

// 0x882090 — __ZN3RBX16CellFaceFacePair13closestVertexEPKNS_4POLY4FaceERKNS_10FixedArrayIN3G3D7Vector3ELm40EEERPKNS1_6VertexE
// type: int __fastcall(int, RBX::POLY::Face *this)
#[doc(alias = "RBX::CellFaceFacePair::closestVertex(RBX::POLY::Face const*,RBX::FixedArray<G3D::Vector3,40ul> const&,RBX::POLY::Vertex const*&)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair13closestVertexEPKNS_4POLY4FaceERKNS_10FixedArrayIN3G3D7Vector3ELm40EEERPKNS1_6VertexE")]
// IDA 0x882090: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x882090() {
}

// 0x8827b0 — __ZN3RBX16CellFaceFacePair12loadVerticesEPNS_10FixedArrayINS0_12VertexStatusELm40EEEPN3G3D15CoordinateFrameERNS1_IPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::loadVertices(RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul> *,G3D::CoordinateFrame *,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair12loadVerticesEPNS_10FixedArrayINS0_12VertexStatusELm40EEEPN3G3D15CoordinateFrameERNS1_IPNS_13PolyConnectorELm40EEE")]
// IDA 0x8827b0: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8827b0() {
}

// 0x8828fc — __ZN3RBX16CellFaceFacePair24checkOneSideIntersectionEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::checkOneSideIntersection(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair24checkOneSideIntersectionEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x8828fc: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8828fc() {
}

// 0x882a50 — __ZN3RBX16CellFaceFacePair25checkTwoSideIntersectionsEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::checkTwoSideIntersections(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair25checkTwoSideIntersectionsEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x882a50: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x882a50() {
}

// 0x882bd0 — __ZN3RBX16CellFaceFacePair27validateOneSideIntersectionEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE
#[doc(alias = "RBX::CellFaceFacePair::validateOneSideIntersection(RBX::POLY::Vertex const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair27validateOneSideIntersectionEPKNS_4POLY6VertexES4_RKN3G3D15CoordinateFrameERNS_10FixedArrayIPNS_13PolyConnectorELm40EEE")]
// IDA 0x882bd0: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x882bd0() {
}

// 0x882d24 — __ZN3RBX16CellFaceFacePair18testVerticesInsideEmRNS_10FixedArrayINS0_12VertexStatusELm40EEERKN3G3D15CoordinateFrameERNS1_IPNS_13PolyConnectorELm40EEE
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::CellFaceFacePair::testVerticesInside(unsigned long,RBX::FixedArray<RBX::CellFaceFacePair::VertexStatus,40ul> &,G3D::CoordinateFrame const&,RBX::FixedArray<RBX::PolyConnector *,40ul> &)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair18testVerticesInsideEmRNS_10FixedArrayINS0_12VertexStatusELm40EEERKN3G3D15CoordinateFrameERNS1_IPNS_13PolyConnectorELm40EEE")]
// IDA 0x882d24: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x882d24() {
}

// 0x882ddc — __ZN3RBX16CellFaceFacePair12vertexInPolyEPKNS_4POLY4FaceEPKNS1_4MeshEPKNS1_6VertexERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::CellFaceFacePair *__hidden this, const RBX::POLY::Face *, const RBX::POLY::Mesh *, const RBX::POLY::Vertex *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::CellFaceFacePair::vertexInPoly(RBX::POLY::Face const*,RBX::POLY::Mesh const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX16CellFaceFacePair12vertexInPolyEPKNS_4POLY4FaceEPKNS1_4MeshEPKNS1_6VertexERKN3G3D15CoordinateFrameE")]
// IDA 0x882ddc: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x882ddc() {
}

// 0x883820 — __ZN3RBX16CellEdgeEdgePair13computeMinMaxERKN3G3D5PlaneEPKNS_4POLY4MeshERfS9_
#[doc(alias = "RBX::CellEdgeEdgePair::computeMinMax(G3D::Plane const&,RBX::POLY::Mesh const*,float &,float &)")]
#[doc(alias = "__ZN3RBX16CellEdgeEdgePair13computeMinMaxERKN3G3D5PlaneEPKNS_4POLY4MeshERfS9_")]
// IDA 0x883820: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x883820() {
}

// 0x883be8 — __ZN3RBX10FixedArrayIN3G3D7Vector3ELm40EE9push_backERKS2_
#[doc(alias = "RBX::FixedArray<G3D::Vector3,40ul>::push_back(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX10FixedArrayIN3G3D7Vector3ELm40EE9push_backERKS2_")]
// IDA 0x883be8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_0x883be8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x883c70 — __ZNK3RBX10FixedArrayIN3G3D7Vector3ELm40EEixEm
#[doc(alias = "RBX::FixedArray<G3D::Vector3,40ul>::operator[](unsigned long)const")]
#[doc(alias = "__ZNK3RBX10FixedArrayIN3G3D7Vector3ELm40EEixEm")]
// IDA 0x883c70: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x883c70() {
}

// 0x883e44 — __ZN3RBX11CellContact18cellFaceIsInteriorINS_5Voxel4GridEEEbRKN3G3D12Vector3int16ENS2_13FaceDirectionE
// type: bool __fastcall(int, const G3D::Vector3int16 *, int)
#[doc(alias = "bool RBX::CellContact::cellFaceIsInterior<RBX::Voxel::Grid>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)")]
#[doc(alias = "__ZN3RBX11CellContact18cellFaceIsInteriorINS_5Voxel4GridEEEbRKN3G3D12Vector3int16ENS2_13FaceDirectionE")]
// IDA 0x883e44: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x883e44() {
}

// 0x88fa74 — __ZN3RBX15BallCellContactC1EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this, RBX::Primitive *, RBX::Primitive *, const G3D::Vector3int16 *)
#[doc(alias = "RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX15BallCellContactC1EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E")]
// IDA 0x88fa74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x88fa74() {
}

// 0x88fa78 — __ZN3RBX15BallCellContactC2EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this, RBX::Primitive *, RBX::Primitive *, const G3D::Vector3int16 *)
#[doc(alias = "RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX15BallCellContactC2EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E")]
// IDA 0x88fa78: 294 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x88fa78() {
}

// 0x89016c — __ZN3RBX15BallCellContact16getFarthestPlaneERfRKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this, float *, const G3D::Vector3 *)
#[doc(alias = "RBX::BallCellContact::getFarthestPlane(float &,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15BallCellContact16getFarthestPlaneERfRKN3G3D7Vector3E")]
// IDA 0x89016c: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x89016c() {
}

// 0x89047c — __ZN3RBX15BallCellContact23getClosestInVoronoiEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this, const RBX::POLY::Face *, float *, const G3D::Vector3 *)
#[doc(alias = "RBX::BallCellContact::getClosestInVoronoiEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15BallCellContact23getClosestInVoronoiEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E")]
// IDA 0x89047c: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x89047c() {
}

// 0x890734 — __ZN3RBX15BallCellContact14getClosestEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getClosestEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15BallCellContact14getClosestEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E")]
// IDA 0x890734: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x890734() {
}

// 0x890810 — __ZN3RBX15BallCellContact16getClosestVertexEPKNS_4POLY4EdgeERfRKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::BallCellContact *__hidden this, const RBX::POLY::Edge *, float *, const G3D::Vector3 *)
#[doc(alias = "RBX::BallCellContact::getClosestVertex(RBX::POLY::Edge const*,float &,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15BallCellContact16getClosestVertexEPKNS_4POLY4EdgeERfRKN3G3D7Vector3E")]
// IDA 0x890810: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x890810() {
}

// 0x890bac — __ZNK3RBX4POLY4Face16pointInExtrusionERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::POLY::Face *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX4POLY4Face16pointInExtrusionERKN3G3D7Vector3E")]
// IDA 0x890bac: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x890bac() {
}

// 0x8979c8 — __ZN3RBX12Region3int16C1ERKN3G3D12Vector3int16ES4_
// type: int __fastcall(int result, int *, int *)
#[doc(alias = "RBX::Region3int16::Region3int16(G3D::Vector3int16 const&,G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX12Region3int16C1ERKN3G3D12Vector3int16ES4_")]
// IDA 0x8979c8: 13 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8979c8() {
}

// 0x8a3648 — __ZN3RBX15BuoyancyContact30hasDistanceSubmergedUnderWaterERKN3G3D7Vector3ERfS4_
// type: int __fastcall(RBX::BuoyancyContact *this, const G3D::Vector3 *, float *, const G3D::Vector3 *)
#[doc(alias = "RBX::BuoyancyContact::hasDistanceSubmergedUnderWater(G3D::Vector3 const&,float &,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15BuoyancyContact30hasDistanceSubmergedUnderWaterERKN3G3D7Vector3ERfS4_")]
// IDA 0x8a3648: 92 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a3648() {
}

// 0x8a3774 — __ZN3RBX15BuoyancyContact18worldPosUnderWaterEPNS_9PrimitiveERKN3G3D7Vector3E
// type: bool __fastcall(RBX::BuoyancyContact *this, RBX::Primitive *, const G3D::Vector3 *)
#[doc(alias = "RBX::BuoyancyContact::worldPosUnderWater(RBX::Primitive *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15BuoyancyContact18worldPosUnderWaterEPNS_9PrimitiveERKN3G3D7Vector3E")]
// IDA 0x8a3774: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a3774() {
}

// 0x8a3828 — __ZN3RBX15BuoyancyContact18worldPosAboveWaterERKN3G3D7Vector3EiRf
// type: bool __fastcall(RBX::BuoyancyContact *this, const G3D::Vector3 *, int, float *)
#[doc(alias = "RBX::BuoyancyContact::worldPosAboveWater(G3D::Vector3 const&,int,float &)")]
#[doc(alias = "__ZN3RBX15BuoyancyContact18worldPosAboveWaterERKN3G3D7Vector3EiRf")]
// IDA 0x8a3828: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a3828() {
}

// 0x8a39ec — __ZN3RBX15BuoyancyContact12cellVelocityERKN3G3D7Vector3E
// type: int __fastcall(__int64 this, __int32 *)
#[doc(alias = "RBX::BuoyancyContact::cellVelocity(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX15BuoyancyContact12cellVelocityERKN3G3D7Vector3E")]
// IDA 0x8a39ec: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a39ec() {
}

// 0x8a3e4c — __ZN3RBX15BuoyancyContact12getWaterCellEPNS_9PrimitiveEN3G3D12Vector3int16E
// type: int __fastcall(int, int, __int16)
#[doc(alias = "RBX::BuoyancyContact::getWaterCell(RBX::Primitive *,G3D::Vector3int16)")]
#[doc(alias = "__ZN3RBX15BuoyancyContact12getWaterCellEPNS_9PrimitiveEN3G3D12Vector3int16E")]
// IDA 0x8a3e4c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a3e4c() {
}

// 0x8a41c0 — __ZN3RBX19BuoyancyBallContact25getSurfaceAreaInDirectionERKN3G3D7Vector3ERfS5_
// type: int __fastcall(int this, const Vector3 *, float *, float *)
#[doc(alias = "RBX::BuoyancyBallContact::getSurfaceAreaInDirection(G3D::Vector3 const&,float &,float &)")]
#[doc(alias = "__ZN3RBX19BuoyancyBallContact25getSurfaceAreaInDirectionERKN3G3D7Vector3ERfS5_")]
// IDA 0x8a41c0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a41c0() {
}

// 0x8a41d8 — __ZN3RBX19BuoyancyBallContact16getCrossSectionsEiRKN3G3D7Vector3E
// type: unsigned __int32 *__fastcall(unsigned __int32 *this, int, const Vector3 *)
#[doc(alias = "RBX::BuoyancyBallContact::getCrossSections(int,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX19BuoyancyBallContact16getCrossSectionsEiRKN3G3D7Vector3E")]
// IDA 0x8a41d8: 8 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a41d8() {
}

// 0x8a4a98 — __ZN3RBX18BuoyancyBoxContact16getCrossSectionsEiRKN3G3D7Vector3E
// type: int __fastcall(RBX::BuoyancyBoxContact *this, _DWORD *, const G3D::Vector3 *, float *)
#[doc(alias = "RBX::BuoyancyBoxContact::getCrossSections(int,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX18BuoyancyBoxContact16getCrossSectionsEiRKN3G3D7Vector3E")]
// IDA 0x8a4a98: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a4a98() {
}

// 0x8a4c00 — __ZN3RBX18BuoyancyBoxContact25getSurfaceAreaInDirectionERKN3G3D7Vector3ERfS5_
// type: int __fastcall(RBX::BuoyancyBoxContact *this, const G3D::Vector3 *, float *, float *)
#[doc(alias = "RBX::BuoyancyBoxContact::getSurfaceAreaInDirection(G3D::Vector3 const&,float &,float &)")]
#[doc(alias = "__ZN3RBX18BuoyancyBoxContact25getSurfaceAreaInDirectionERKN3G3D7Vector3ERfS5_")]
// IDA 0x8a4c00: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a4c00() {
}

// 0x8a6760 — __ZN3RBX17BuoyancyConnectorC1EPNS_4BodyES2_RKN3G3D7Vector3E
// type: int __fastcall(RBX::BuoyancyConnector *this, RBX::Body *, RBX::Body *, const G3D::Vector3 *)
#[doc(alias = "RBX::BuoyancyConnector::BuoyancyConnector(RBX::Body *,RBX::Body *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX17BuoyancyConnectorC1EPNS_4BodyES2_RKN3G3D7Vector3E")]
// IDA 0x8a6760: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x8a6760() {
}

// 0x8a6764 — __ZN3RBX17BuoyancyConnectorC2EPNS_4BodyES2_RKN3G3D7Vector3E
// type: RBX::BuoyancyConnector *__fastcall(RBX::BuoyancyConnector *this, RBX::Body *, RBX::Body *, const G3D::Vector3 *)
#[doc(alias = "RBX::BuoyancyConnector::BuoyancyConnector(RBX::Body *,RBX::Body *,G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX17BuoyancyConnectorC2EPNS_4BodyES2_RKN3G3D7Vector3E")]
// IDA 0x8a6764: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a6764() {
}

// 0x8a8a44 — __ZN3RBX17ManualJointHelper29createTerrainJointSurfacePairERNS_9PrimitiveERmS2_RN3G3D12Vector3int16E
// type: void __fastcall(RBX::ManualJointHelper *this, const G3D::Vector3int16 **, unsigned int *, const G3D::CoordinateFrame **, G3D::Vector3int16 *)
#[doc(alias = "RBX::ManualJointHelper::createTerrainJointSurfacePair(RBX::Primitive &,unsigned long &,RBX::Primitive &,G3D::Vector3int16 &)")]
#[doc(alias = "__ZN3RBX17ManualJointHelper29createTerrainJointSurfacePairERNS_9PrimitiveERmS2_RN3G3D12Vector3int16E")]
// IDA 0x8a8a44: 342 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8a8a44() {
}

// 0x8abd80 — __ZNK3RBX17ManualJointHelper19shouldRender3dAdornEv
// type: int __fastcall(RBX::ManualJointHelper *this)
#[doc(alias = "RBX::ManualJointHelper::shouldRender3dAdorn(void)const")]
#[doc(alias = "__ZNK3RBX17ManualJointHelper19shouldRender3dAdornEv")]
// IDA 0x8abd80: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8abd80() {
}

// 0x8b078c — __ZN3RBX16UserInputService18moveLocalCharacterEN3G3D7Vector2Ef
// type: void __fastcall(int, _DWORD *, struct _Unwind_Exception *)
#[doc(alias = "RBX::UserInputService::moveLocalCharacter(G3D::Vector2,float)")]
#[doc(alias = "__ZN3RBX16UserInputService18moveLocalCharacterEN3G3D7Vector2Ef")]
// IDA 0x8b078c: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8b078c() {
}

// 0x8b0d38 — __ZN3RBX16UserInputService12rotateCameraEN3G3D7Vector2E
// type: void __fastcall(RBX::DataModel *, RBX::Instance *)
#[doc(alias = "RBX::UserInputService::rotateCamera(G3D::Vector2)")]
#[doc(alias = "__ZN3RBX16UserInputService12rotateCameraEN3G3D7Vector2E")]
// IDA 0x8b0d38: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8b0d38() {
}

// 0x8b0fe0 — __ZN3RBX16UserInputService17rotateCameraSpeedEN3G3D7Vector2EbNS_6Camera13CameraPanModeE
// type: void __fastcall(RBX::DataModel *, RBX::Instance *, boost::detail::sp_counted_base *, struct _Unwind_Exception *, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::UserInputService::rotateCameraSpeed(G3D::Vector2,bool,RBX::Camera::CameraPanMode)")]
#[doc(alias = "__ZN3RBX16UserInputService17rotateCameraSpeedEN3G3D7Vector2EbNS_6Camera13CameraPanModeE")]
// IDA 0x8b0fe0: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8b0fe0() {
}

// 0x8e12cc — __ZN3RBX9GuiBase2d34recalculateAbsoluteSizeAndPositionERKN3G3D6Rect2DE
// type: int __fastcall(RBX::GuiBase2d *, int)
#[doc(alias = "RBX::GuiBase2d::recalculateAbsoluteSizeAndPosition(G3D::Rect2D const&)")]
#[doc(alias = "__ZN3RBX9GuiBase2d34recalculateAbsoluteSizeAndPositionERKN3G3D6Rect2DE")]
// IDA 0x8e12cc: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e12cc() {
}

// 0x8e13a4 — __ZN3RBX9GuiBase2d19setAbsolutePositionERKN3G3D7Vector2E
// type: int __fastcall(RBX::GuiBase2d *this, const G3D::Vector2 *)
#[doc(alias = "RBX::GuiBase2d::setAbsolutePosition(G3D::Vector2 const&)")]
#[doc(alias = "__ZN3RBX9GuiBase2d19setAbsolutePositionERKN3G3D7Vector2E")]
// IDA 0x8e13a4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e13a4() {
}

// 0x8e13f0 — __ZN3RBX9GuiBase2d15setAbsoluteSizeERKN3G3D7Vector2E
// type: int __fastcall(RBX::GuiBase2d *this, const G3D::Vector2 *)
#[doc(alias = "RBX::GuiBase2d::setAbsoluteSize(G3D::Vector2 const&)")]
#[doc(alias = "__ZN3RBX9GuiBase2d15setAbsoluteSizeERKN3G3D7Vector2E")]
// IDA 0x8e13f0: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e13f0() {
}

// 0x8e1480 — __ZN3RBX9GuiBase2d17recursiveRender2dEPNS_5AdornE
// type: void __fastcall(const shared_count *this, RBX::Adorn *)
#[doc(alias = "RBX::GuiBase2d::recursiveRender2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX9GuiBase2d17recursiveRender2dEPNS_5AdornE")]
// IDA 0x8e1480: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e1480() {
}

// 0x8e158c — __ZN3RBX9GuiBase2d12handleResizeERKN3G3D6Rect2DEb
// type: void __fastcall(const shared_count *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::GuiBase2d::handleResize(G3D::Rect2D const&,bool)")]
#[doc(alias = "__ZN3RBX9GuiBase2d12handleResizeERKN3G3D6Rect2DEb")]
// IDA 0x8e158c: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e158c() {
}

// 0x8e997c — __ZNK3RBX5Voxel4Grid23getVoxelLikelyThisChunkERKNS_13SpatialRegion2IdERKNS1_5ChunkERKN3G3D12Vector3int16E
// type: char *__fastcall(int, unsigned __int16 *, int, __int16 *)
#[doc(alias = "RBX::Voxel::Grid::getVoxelLikelyThisChunk(RBX::SpatialRegion::Id const&,RBX::Voxel::Grid::Chunk const&,G3D::Vector3int16 const&)const")]
#[doc(alias = "__ZNK3RBX5Voxel4Grid23getVoxelLikelyThisChunkERKNS_13SpatialRegion2IdERKNS1_5ChunkERKN3G3D12Vector3int16E")]
// IDA 0x8e997c: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e997c() {
}

// 0x8e9a08 — __ZNK3RBX5Voxel4Grid17fillLocalAreaInfoERKN3G3D12Vector3int16ERKNS0_5Water17RelevantNeighborsEPNS6_13LocalAreaInfoE
// type: int __fastcall(int, __int16 *, __int16 *, char *)
#[doc(alias = "RBX::Voxel::Grid::fillLocalAreaInfo(G3D::Vector3int16 const&,RBX::Voxel::Water::RelevantNeighbors const&,RBX::Voxel::Water::LocalAreaInfo *)const")]
#[doc(alias = "__ZNK3RBX5Voxel4Grid17fillLocalAreaInfoERKN3G3D12Vector3int16ERKNS0_5Water17RelevantNeighborsEPNS6_13LocalAreaInfoE")]
// IDA 0x8e9a08: 254 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e9a08() {
}

// 0x8e9cd0 — __ZN3RBX5Voxel4Grid7setCellERKN3G3D12Vector3int16ENS0_4CellENS0_12CellMaterialE
// type: int __fastcall(int, __int16 *, unsigned __int8, int)
#[doc(alias = "RBX::Voxel::Grid::setCell(G3D::Vector3int16 const&,RBX::Voxel::Cell,RBX::Voxel::CellMaterial)")]
#[doc(alias = "__ZN3RBX5Voxel4Grid7setCellERKN3G3D12Vector3int16ENS0_4CellENS0_12CellMaterialE")]
// IDA 0x8e9cd0: 214 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e9cd0() {
}

// 0x8e9f34 — __ZNK3RBX5Voxel4Grid9getRegionERKN3G3D12Vector3int16ES5_
// type: int __fastcall(RBX::Voxel::Grid *this, const G3D::Vector3int16 *, const G3D::Vector3int16 *, __int16 *)
#[doc(alias = "RBX::Voxel::Grid::getRegion(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
#[doc(alias = "__ZNK3RBX5Voxel4Grid9getRegionERKN3G3D12Vector3int16ES5_")]
// IDA 0x8e9f34: 71 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8e9f34() {
}

// 0x8ea008 — __ZNK3RBX5Voxel4Grid15getCellInternalERKN3G3D12Vector3int16E
// type: int __fastcall(RBX::Voxel::Grid *this, const G3D::Vector3int16 *)
#[doc(alias = "RBX::Voxel::Grid::getCellInternal(G3D::Vector3int16 const&)const")]
#[doc(alias = "__ZNK3RBX5Voxel4Grid15getCellInternalERKN3G3D12Vector3int16E")]
// IDA 0x8ea008: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8ea008() {
}

// 0x8ea068 — __ZNK3RBX5Voxel4Grid23getCellMaterialInternalERKN3G3D12Vector3int16E
// type: int __fastcall(RBX::Voxel::Grid *this, const G3D::Vector3int16 *)
#[doc(alias = "RBX::Voxel::Grid::getCellMaterialInternal(G3D::Vector3int16 const&)const")]
#[doc(alias = "__ZNK3RBX5Voxel4Grid23getCellMaterialInternalERKN3G3D12Vector3int16E")]
// IDA 0x8ea068: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8ea068() {
}

// 0x8ea0dc — __ZNK3RBX5Voxel4Grid20getWaterCellInternalERKN3G3D12Vector3int16E
// type: int __fastcall(RBX::Voxel::Grid *this, const G3D::Vector3int16 *)
#[doc(alias = "RBX::Voxel::Grid::getWaterCellInternal(G3D::Vector3int16 const&)const")]
#[doc(alias = "__ZNK3RBX5Voxel4Grid20getWaterCellInternalERKN3G3D12Vector3int16E")]
// IDA 0x8ea0dc: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8ea0dc() {
}

// 0x8ea2f0 — __ZN3RBX5Voxel5Water12cellHasWaterINS0_4Grid5ChunkEEEbPKT_RKNS0_4CellERKN3G3D12Vector3int16E
// type: int __fastcall(int, unsigned __int8 *, __int16 *)
#[doc(alias = "bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3RBX5Voxel5Water12cellHasWaterINS0_4Grid5ChunkEEEbPKT_RKNS0_4CellERKN3G3D12Vector3int16E")]
// IDA 0x8ea2f0: 166 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8ea2f0() {
}

// 0x8ed314 — __ZN3RBX16OnScreenProfiler14GetRandomColorEPN3G3D6Color4E
// type: int __fastcall(_DWORD *, int, __int32 *)
#[doc(alias = "RBX::OnScreenProfiler::GetRandomColor(G3D::Color4 *)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler14GetRandomColorEPN3G3D6Color4E")]
// IDA 0x8ed314: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8ed314() {
}

// 0x8f1c30 — __ZN3RBX16OnScreenProfiler7AddTextEPNS_5AdornEPKcRffffRKN3G3D6Color4EPNS6_7Vector2E
// type: void __fastcall(RBX::OnScreenProfiler *this, RBX::Adorn *, const char *, float *, float, float32_t, float, const G3D::Color4 *, G3D::Vector2 *)
#[doc(alias = "RBX::OnScreenProfiler::AddText(RBX::Adorn *,char const*,float &,float,float,float,G3D::Color4 const&,G3D::Vector2 *)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler7AddTextEPNS_5AdornEPKcRffffRKN3G3D6Color4EPNS6_7Vector2E")]
// IDA 0x8f1c30: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8f1c30() {
}

// 0x8f2008 — __ZN3RBX16OnScreenProfiler7DrawBarEPNS_5AdornERN3G3D7Vector2EfRKNS3_6Color4ES8_ff
// type: int __fastcall(int, int, __int32 *, int, int, int, float32_t, float32_t)
#[doc(alias = "RBX::OnScreenProfiler::DrawBar(RBX::Adorn *,G3D::Vector2 &,float,G3D::Color4 const&,G3D::Color4 const&,float,float)")]
#[doc(alias = "__ZN3RBX16OnScreenProfiler7DrawBarEPNS_5AdornERN3G3D7Vector2EfRKNS3_6Color4ES8_ff")]
// IDA 0x8f2008: 92 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x8f2008() {
}

// 0x949adc — __ZNK3RBX6RbxRay17intersectionPlaneERKN3G3D5PlaneE
// type: _DWORD __fastcall(RBX::RbxRay *__hidden this, const G3D::Plane *)
#[doc(alias = "RBX::RbxRay::intersectionPlane(G3D::Plane const&)const")]
#[doc(alias = "__ZNK3RBX6RbxRay17intersectionPlaneERKN3G3D5PlaneE")]
// IDA 0x949adc: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x949adc() {
}

// 0x949be4 — __ZN3RBX6RbxG3D8SpecDataC1Ev
// type: _DWORD __fastcall(RBX::RbxG3D::SpecData *__hidden this)
#[doc(alias = "RBX::RbxG3D::SpecData::SpecData(void)")]
#[doc(alias = "__ZN3RBX6RbxG3D8SpecDataC1Ev")]
// IDA 0x949be4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x949be4() {
}

// 0x949bf0 — __ZN3RBX6RbxG3D8SpecDataC2Ev
// type: _DWORD __fastcall(RBX::RbxG3D::SpecData *__hidden this)
#[doc(alias = "RBX::RbxG3D::SpecData::SpecData(void)")]
#[doc(alias = "__ZN3RBX6RbxG3D8SpecDataC2Ev")]
// IDA 0x949bf0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x949bf0() {
}

// 0x949d08 — __ZN3RBX7FrustumC1ERKN3G3D7Vector3ES4_S4_ffff
// type: _DWORD __fastcall(RBX::Frustum *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, float, float, float, float)
#[doc(alias = "RBX::Frustum::Frustum(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float,float,float,float)")]
#[doc(alias = "__ZN3RBX7FrustumC1ERKN3G3D7Vector3ES4_S4_ffff")]
// IDA 0x949d08: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x949d08() {
}

// 0x949d40 — __ZN3RBX7FrustumC2ERKN3G3D7Vector3ES4_S4_ffff
// type: _DWORD __fastcall(RBX::Frustum *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, float, float, float, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Frustum::Frustum(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float,float,float,float)")]
#[doc(alias = "__ZN3RBX7FrustumC2ERKN3G3D7Vector3ES4_S4_ffff")]
// IDA 0x949d40: 592 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x949d40() {
}

// 0x94a45c — __ZNK3RBX7Frustum13containsPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Frustum *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Frustum::containsPoint(G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX7Frustum13containsPointERKN3G3D7Vector3E")]
// IDA 0x94a45c: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94a45c() {
}

// 0x94a548 — __ZNK3RBX7Frustum12containsAABBERKNS_7ExtentsERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Frustum *__hidden this, const RBX::Extents *, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Frustum::containsAABB(RBX::Extents const&,G3D::CoordinateFrame const&)const")]
#[doc(alias = "__ZNK3RBX7Frustum12containsAABBERKNS_7ExtentsERKN3G3D15CoordinateFrameE")]
// IDA 0x94a548: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94a548() {
}

// 0x94a620 — __ZNK3RBX7Frustum16intersectsSphereERKN3G3D7Vector3Ef
// type: _DWORD __fastcall(RBX::Frustum *__hidden this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Frustum::intersectsSphere(G3D::Vector3 const&,float)const")]
#[doc(alias = "__ZNK3RBX7Frustum16intersectsSphereERKN3G3D7Vector3Ef")]
// IDA 0x94a620: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94a620() {
}

// 0x94b738 — __ZN3RBX9DrawAdorn8cylinderEPNS_5AdornERKN3G3D15CoordinateFrameEiffRKNS3_6Color4Eb
// type: _DWORD __fastcall(RBX::DrawAdorn *__hidden this, RBX::Adorn *, const G3D::CoordinateFrame *, int, float, float, const G3D::Color4 *, bool)
#[doc(alias = "RBX::DrawAdorn::cylinder(RBX::Adorn *,G3D::CoordinateFrame const&,int,float,float,G3D::Color4 const&,bool)")]
#[doc(alias = "__ZN3RBX9DrawAdorn8cylinderEPNS_5AdornERKN3G3D15CoordinateFrameEiffRKNS3_6Color4Eb")]
// IDA 0x94b738: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94b738() {
}

// 0x94b79c — __ZN3RBX9DrawAdorn13surfaceBorderEPNS_5AdornERKN3G3D7Vector3EfiRKNS3_6Color4E
#[doc(alias = "RBX::DrawAdorn::surfaceBorder(RBX::Adorn *,G3D::Vector3 const&,float,int,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn13surfaceBorderEPNS_5AdornERKN3G3D7Vector3EfiRKNS3_6Color4E")]
// IDA 0x94b79c: 154 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94b79c() {
}

// 0x94b974 — __ZN3RBX9DrawAdorn18surfaceGridAtCoordEPNS_5AdornERN3G3D15CoordinateFrameERKNS3_7Vector4ERKNS3_7Vector3ESB_RKNS3_6Color4Ei
// type: _DWORD __fastcall(RBX::DrawAdorn *__hidden this, RBX::Adorn *, G3D::CoordinateFrame *, const G3D::Vector4 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Color4 *, int)
#[doc(alias = "RBX::DrawAdorn::surfaceGridAtCoord(RBX::Adorn *,G3D::CoordinateFrame &,G3D::Vector4 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Color4 const&,int)")]
#[doc(alias = "__ZN3RBX9DrawAdorn18surfaceGridAtCoordEPNS_5AdornERN3G3D15CoordinateFrameERKNS3_7Vector4ERKNS3_7Vector3ESB_RKNS3_6Color4Ei")]
// IDA 0x94b974: 568 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94b974() {
}

// 0x94c058 — __ZN3RBX9DrawAdorn13zeroPlaneGridEPNS_5AdornERKNS_6CameraEiiRKN3G3D6Color4ES9_
// type: _DWORD __fastcall(RBX::DrawAdorn *__hidden this, RBX::Adorn *, const RBX::Camera *, int, int, const G3D::Color4 *, const G3D::Color4 *)
#[doc(alias = "RBX::DrawAdorn::zeroPlaneGrid(RBX::Adorn *,RBX::Camera const&,int,int,G3D::Color4 const&,G3D::Color4 const&)")]
#[doc(alias = "__ZN3RBX9DrawAdorn13zeroPlaneGridEPNS_5AdornERKNS_6CameraEiiRKN3G3D6Color4ES9_")]
// IDA 0x94c058: 711 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94c058() {
}

// 0x94d01c — __ZN3RBX9DrawAdorn19circularGridAtCoordEPNS_5AdornERKN3G3D15CoordinateFrameERKNS3_7Vector3ES9_NS_8NormalIdERKNS3_6Color4Ei
#[doc(alias = "RBX::DrawAdorn::circularGridAtCoord(RBX::Adorn *,G3D::CoordinateFrame const&,G3D::Vector3 const&,G3D::Vector3 const&,RBX::NormalId,G3D::Color4 const&,int)")]
#[doc(alias = "__ZN3RBX9DrawAdorn19circularGridAtCoordEPNS_5AdornERKN3G3D15CoordinateFrameERKNS3_7Vector3ES9_NS_8NormalIdERKNS3_6Color4Ei")]
// IDA 0x94d01c: 246 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94d01c() {
}

// 0x94d374 — __ZN3RBX9DrawAdorn17handlePosInObjectERKN3G3D7Vector3ERKNS_7ExtentsENS_10HandleTypeENS_8NormalIdE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::DrawAdorn::handlePosInObject(G3D::Vector3 const&,RBX::Extents const&,RBX::HandleType,RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9DrawAdorn17handlePosInObjectERKN3G3D7Vector3ERKNS_7ExtentsENS_10HandleTypeENS_8NormalIdE")]
// IDA 0x94d374: 147 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x94d374() {
}

