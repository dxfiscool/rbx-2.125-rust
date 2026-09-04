//! rendering generated_41 — Ogre::|G3D:: strict 13333 total (13663 substr Ogre|G3D), 5248 prior, 100 this batch — 0x882ddc..0x8c4220
//! EA-sorted ascending earliest gap after 0xd79b6c — rbx_core::SharedPtr not boost
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0x882ddc — __ZN3RBX16CellFaceFacePair12vertexInPolyEPKNS_4POLY4FaceEPKNS1_4MeshEPKNS1_6VertexERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::CellFaceFacePair::vertexInPoly(RBX::POLY::Face const*,RBX::POLY::Mesh const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&)")]
// was: RBX::CellFaceFacePair::vertexInPoly(RBX::POLY::Face const*,RBX::POLY::Mesh const*,RBX::POLY::Vertex const*,G3D::CoordinateFrame const&)
// IDA 0x882ddc: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_882ddc() {
}

// 0x883820 — __ZN3RBX16CellEdgeEdgePair13computeMinMaxERKN3G3D5PlaneEPKNS_4POLY4MeshERfS9_
#[doc(alias = "RBX::CellEdgeEdgePair::computeMinMax(G3D::Plane const&,RBX::POLY::Mesh const*,float &,float &)")]
// was: RBX::CellEdgeEdgePair::computeMinMax(G3D::Plane const&,RBX::POLY::Mesh const*,float &,float &)
// IDA 0x883820: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_883820() {
}

// 0x883be8 — __ZN3RBX10FixedArrayIN3G3D7Vector3ELm40EE9push_backERKS2_
#[doc(alias = "RBX::FixedArray<G3D::Vector3,40ul>::push_back(G3D::Vector3 const&)")]
// was: RBX::FixedArray<G3D::Vector3,40ul>::push_back(G3D::Vector3 const&)
// IDA 0x883be8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_883be8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x883c70 — __ZNK3RBX10FixedArrayIN3G3D7Vector3ELm40EEixEm
#[doc(alias = "RBX::FixedArray<G3D::Vector3,40ul>::operator[](unsigned long)const")]
// was: RBX::FixedArray<G3D::Vector3,40ul>::operator[](unsigned long)const
// IDA 0x883c70: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_883c70() {
}

// 0x883cec — __ZN3RBX11CellContact18cellFaceIsInteriorINS_19MegaClusterInstanceEEEbRKN3G3D12Vector3int16ENS_5Voxel13FaceDirectionE
#[doc(alias = "bool RBX::CellContact::cellFaceIsInterior<RBX::MegaClusterInstance>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)")]
// was: bool RBX::CellContact::cellFaceIsInterior<RBX::MegaClusterInstance>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)
// IDA 0x883cec: 114 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_883cec() {
}

// 0x883e44 — __ZN3RBX11CellContact18cellFaceIsInteriorINS_5Voxel4GridEEEbRKN3G3D12Vector3int16ENS2_13FaceDirectionE
#[doc(alias = "bool RBX::CellContact::cellFaceIsInterior<RBX::Voxel::Grid>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)")]
// was: bool RBX::CellContact::cellFaceIsInterior<RBX::Voxel::Grid>(G3D::Vector3int16 const&,RBX::Voxel::FaceDirection)
// IDA 0x883e44: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_883e44() {
}

// 0x884040 — __ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E
#[doc(alias = "G3D::Plane::pointOnOrBehind(G3D::Vector3)const")]
// was: G3D::Plane::pointOnOrBehind(G3D::Vector3)const
// IDA 0x884040: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_884040() {
}

// 0x88e78c — __ZNK3RBX11PluginMouse23getPartByLocalCharacterERKNS_7UIEventEPKNS_13HitTestFilterERN3G3D7Vector3E
#[doc(alias = "RBX::PluginMouse::getPartByLocalCharacter(RBX::UIEvent const&,RBX::HitTestFilter const*,G3D::Vector3 &)const")]
// was: RBX::PluginMouse::getPartByLocalCharacter(RBX::UIEvent const&,RBX::HitTestFilter const*,G3D::Vector3 &)const
// IDA 0x88e78c: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88e78c() {
}

// 0x88ec38 — __ZNK3RBX11PluginMouse7getPartEPKNS_13HitTestFilterERN3G3D7Vector3E
#[doc(alias = "RBX::PluginMouse::getPart(RBX::HitTestFilter const*,G3D::Vector3 &)const")]
// was: RBX::PluginMouse::getPart(RBX::HitTestFilter const*,G3D::Vector3 &)const
// IDA 0x88ec38: 160 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88ec38() {
}

// 0x88f2c8 — __ZNK3RBX11PluginMouse12getMousePartERKNS_6RbxRayERKNS_14ContactManagerERKSt6vectorIPKNS_9PrimitiveESaISA_EEPKNS_13HitTestFilterERN3G3D7Vector3Ef
#[doc(alias = "RBX::PluginMouse::getMousePart(RBX::RbxRay const&,RBX::ContactManager const&,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> const&,RBX::HitTestFilter const*,G3D::Vector3 &,float)const")]
// was: RBX::PluginMouse::getMousePart(RBX::RbxRay const&,RBX::ContactManager const&,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> const&,RBX::HitTestFilter const*,G3D::Vector3 &,float)const
// IDA 0x88f2c8: 120 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88f2c8() {
}

// 0x88fa74 — __ZN3RBX15BallCellContactC1EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
#[doc(alias = "RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
// was: RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)
// IDA 0x88fa74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_88fa74() {
}

// 0x88fa78 — __ZN3RBX15BallCellContactC2EPNS_9PrimitiveES2_RKN3G3D12Vector3int16E
#[doc(alias = "RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)")]
// was: RBX::BallCellContact::BallCellContact(RBX::Primitive *,RBX::Primitive *,G3D::Vector3int16 const&)
// IDA 0x88fa78: 294 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_88fa78() {
}

// 0x89016c — __ZN3RBX15BallCellContact16getFarthestPlaneERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getFarthestPlane(float &,G3D::Vector3 const&)")]
// was: RBX::BallCellContact::getFarthestPlane(float &,G3D::Vector3 const&)
// IDA 0x89016c: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_89016c() {
}

// 0x89047c — __ZN3RBX15BallCellContact23getClosestInVoronoiEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getClosestInVoronoiEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
// was: RBX::BallCellContact::getClosestInVoronoiEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)
// IDA 0x89047c: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_89047c() {
}

// 0x890734 — __ZN3RBX15BallCellContact14getClosestEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getClosestEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
// was: RBX::BallCellContact::getClosestEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)
// IDA 0x890734: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_890734() {
}

// 0x890810 — __ZN3RBX15BallCellContact16getClosestVertexEPKNS_4POLY4EdgeERfRKN3G3D7Vector3E
#[doc(alias = "RBX::BallCellContact::getClosestVertex(RBX::POLY::Edge const*,float &,G3D::Vector3 const&)")]
// was: RBX::BallCellContact::getClosestVertex(RBX::POLY::Edge const*,float &,G3D::Vector3 const&)
// IDA 0x890810: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_890810() {
}

// 0x890bac — __ZNK3RBX4POLY4Face16pointInExtrusionERKN3G3D7Vector3E
#[doc(alias = "RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const")]
// was: RBX::POLY::Face::pointInExtrusion(G3D::Vector3 const&)const
// IDA 0x890bac: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_890bac() {
}

// 0x890c88 — __ZNK3G3D4Line8distanceERKNS_7Vector3E
#[doc(alias = "G3D::Line::distance(G3D::Vector3 const&)const")]
// was: G3D::Line::distance(G3D::Vector3 const&)const
// IDA 0x890c88: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_890c88() {
}

// 0x8913c0 — __ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_
#[doc(alias = "G3D::Line::fromTwoPoints(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: G3D::Line::fromTwoPoints(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0x8913c0: 49 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8913c0() {
}

// 0x8954bc — __ZNK3RBX16TerrainPartition17containsCellBuildEsRKN3G3D12Vector3int16E
#[doc(alias = "RBX::TerrainPartition::containsCellBuild(short,G3D::Vector3int16 const&)const")]
// was: RBX::TerrainPartition::containsCellBuild(short,G3D::Vector3int16 const&)const
// IDA 0x8954bc: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8954bc() {
}

// 0x8955fc — __ZNK3RBX16TerrainPartition15computeUniqueIdERKsRKN3G3D12Vector3int16E
#[doc(alias = "RBX::TerrainPartition::computeUniqueId(short const&,G3D::Vector3int16 const&)const")]
// was: RBX::TerrainPartition::computeUniqueId(short const&,G3D::Vector3int16 const&)const
// IDA 0x8955fc: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8955fc() {
}

// 0x8956a8 — __ZNK3RBX16TerrainPartition15getCellInternalERKN3G3D12Vector3int16E
#[doc(alias = "RBX::TerrainPartition::getCellInternal(G3D::Vector3int16 const&)const")]
// was: RBX::TerrainPartition::getCellInternal(G3D::Vector3int16 const&)const
// IDA 0x8956a8: 10 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8956a8() {
}

// 0x8956c8 — __ZNK3RBX16TerrainPartition16containsCellFastEsRKN3G3D12Vector3int16E
#[doc(alias = "RBX::TerrainPartition::containsCellFast(short,G3D::Vector3int16 const&)const")]
// was: RBX::TerrainPartition::containsCellFast(short,G3D::Vector3int16 const&)const
// IDA 0x8956c8: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8956c8() {
}

// 0x895914 — __ZNK3RBX16TerrainPartition41findBoundingBoxPartitionExtentsIfTouchingERKNS_7ExtentsERKsRKN3G3D12Vector3int16ES9_RS7_SA_
#[doc(alias = "RBX::TerrainPartition::findBoundingBoxPartitionExtentsIfTouching(RBX::Extents const&,short const&,G3D::Vector3int16 const&,G3D::Vector3int16 const&,G3D::Vector3int16&,G3D::Vector3int16&)const")]
// was: RBX::TerrainPartition::findBoundingBoxPartitionExtentsIfTouching(RBX::Extents const&,short const&,G3D::Vector3int16 const&,G3D::Vector3int16 const&,G3D::Vector3int16&,G3D::Vector3int16&)const
// IDA 0x895914: 212 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_895914() {
}

// 0x895ba4 — __ZNK3RBX16TerrainPartition24findCellsTouchingExtentsERKNS_7ExtentsERKsRKN3G3D12Vector3int16EPSt3mapIiPS7_St4lessIiESaISt4pairIKiSB_EEE
#[doc(alias = "RBX::TerrainPartition::findCellsTouchingExtents(RBX::Extents const&,short const&,G3D::Vector3int16 const&,std::map<int,G3D::Vector3int16*,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16*>>> *)const")]
// was: RBX::TerrainPartition::findCellsTouchingExtents(RBX::Extents const&,short const&,G3D::Vector3int16 const&,std::map<int,G3D::Vector3int16*,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16*>>> *)const
// IDA 0x895ba4: 250 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_895ba4() {
}

// 0x895e94 — __ZNK3RBX16TerrainPartition18generateFoundCellsERKN3G3D12Vector3int16ERKNS_7ExtentsES4_PSt3mapIiPS2_St4lessIiESaISt4pairIKiS9_EEE
#[doc(alias = "RBX::TerrainPartition::generateFoundCells(G3D::Vector3int16 const&,RBX::Extents const&,G3D::Vector3int16 const&,std::map<int,G3D::Vector3int16*,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16*>>> *)const")]
// was: RBX::TerrainPartition::generateFoundCells(G3D::Vector3int16 const&,RBX::Extents const&,G3D::Vector3int16 const&,std::map<int,G3D::Vector3int16*,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16*>>> *)const
// IDA 0x895e94: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_895e94() {
}

// 0x895f80 — __ZN3RBX16TerrainPartition18onTerrainCellAddedERKN3G3D12Vector3int16E
#[doc(alias = "RBX::TerrainPartition::onTerrainCellAdded(G3D::Vector3int16 const&)")]
// was: RBX::TerrainPartition::onTerrainCellAdded(G3D::Vector3int16 const&)
// IDA 0x895f80: 131 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_895f80() {
}

// 0x896120 — __ZN3RBX16TerrainPartition20onTerrainCellRemovedERKN3G3D12Vector3int16E
#[doc(alias = "RBX::TerrainPartition::onTerrainCellRemoved(G3D::Vector3int16 const&)")]
// was: RBX::TerrainPartition::onTerrainCellRemoved(G3D::Vector3int16 const&)
// IDA 0x896120: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_896120() {
}

// 0x896210 — __ZNK3RBX16TerrainPartition22getContainingPartitionERKsRKN3G3D12Vector3int16ERS4_
#[doc(alias = "RBX::TerrainPartition::getContainingPartition(short const&,G3D::Vector3int16 const&,G3D::Vector3int16&)const")]
// was: RBX::TerrainPartition::getContainingPartition(short const&,G3D::Vector3int16 const&,G3D::Vector3int16&)const
// IDA 0x896210: 126 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_896210() {
}

// 0x8963e4 — __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::resize(unsigned long,G3D::Vector3int16)")]
// was: std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::resize(unsigned long,G3D::Vector3int16)
// IDA 0x8963e4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8963e4() {
}

// 0x896464 — __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert_unique(std::pair<int const,G3D::Vector3int16 *> const&)")]
// was: std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert_unique(std::pair<int const,G3D::Vector3int16 *> const&)
// IDA 0x896464: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_896464() {
}

// 0x8964cc — __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,G3D::Vector3int16 *> const&)")]
// was: std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,G3D::Vector3int16 *> const&)
// IDA 0x8964cc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8964cc() {
}

// 0x896524 — __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3int16*,std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>>,unsigned long,G3D::Vector3int16 const&)")]
// was: std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3int16*,std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>>,unsigned long,G3D::Vector3int16 const&)
// IDA 0x896524: 203 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_896524() {
}

// 0x896748 — __ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_allocate(unsigned long)
// IDA 0x896748: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_896748() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x89676c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector3int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3int16 *,G3D::Vector3int16 *>(G3D::Vector3int16 *,G3D::Vector3int16 *,G3D::Vector3int16 *)")]
// was: G3D::Vector3int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3int16 *,G3D::Vector3int16 *>(G3D::Vector3int16 *,G3D::Vector3int16 *,G3D::Vector3int16 *)
// IDA 0x89676c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_89676c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x8979c8 — __ZN3RBX12Region3int16C1ERKN3G3D12Vector3int16ES4_
#[doc(alias = "RBX::Region3int16::Region3int16(G3D::Vector3int16 const&,G3D::Vector3int16 const&)")]
// was: RBX::Region3int16::Region3int16(G3D::Vector3int16 const&,G3D::Vector3int16 const&)
// IDA 0x8979c8: 13 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8979c8() {
}

// 0x8a3648 — __ZN3RBX15BuoyancyContact30hasDistanceSubmergedUnderWaterERKN3G3D7Vector3ERfS4_
#[doc(alias = "RBX::BuoyancyContact::hasDistanceSubmergedUnderWater(G3D::Vector3 const&,float &,G3D::Vector3 const&)")]
// was: RBX::BuoyancyContact::hasDistanceSubmergedUnderWater(G3D::Vector3 const&,float &,G3D::Vector3 const&)
// IDA 0x8a3648: 92 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a3648() {
}

// 0x8a3774 — __ZN3RBX15BuoyancyContact18worldPosUnderWaterEPNS_9PrimitiveERKN3G3D7Vector3E
#[doc(alias = "RBX::BuoyancyContact::worldPosUnderWater(RBX::Primitive *,G3D::Vector3 const&)")]
// was: RBX::BuoyancyContact::worldPosUnderWater(RBX::Primitive *,G3D::Vector3 const&)
// IDA 0x8a3774: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a3774() {
}

// 0x8a3828 — __ZN3RBX15BuoyancyContact18worldPosAboveWaterERKN3G3D7Vector3EiRf
#[doc(alias = "RBX::BuoyancyContact::worldPosAboveWater(G3D::Vector3 const&,int,float &)")]
// was: RBX::BuoyancyContact::worldPosAboveWater(G3D::Vector3 const&,int,float &)
// IDA 0x8a3828: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a3828() {
}

// 0x8a39ec — __ZN3RBX15BuoyancyContact12cellVelocityERKN3G3D7Vector3E
#[doc(alias = "RBX::BuoyancyContact::cellVelocity(G3D::Vector3 const&)")]
// was: RBX::BuoyancyContact::cellVelocity(G3D::Vector3 const&)
// IDA 0x8a39ec: 94 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a39ec() {
}

// 0x8a3e4c — __ZN3RBX15BuoyancyContact12getWaterCellEPNS_9PrimitiveEN3G3D12Vector3int16E
#[doc(alias = "RBX::BuoyancyContact::getWaterCell(RBX::Primitive *,G3D::Vector3int16)")]
// was: RBX::BuoyancyContact::getWaterCell(RBX::Primitive *,G3D::Vector3int16)
// IDA 0x8a3e4c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a3e4c() {
}

// 0x8a41c0 — __ZN3RBX19BuoyancyBallContact25getSurfaceAreaInDirectionERKN3G3D7Vector3ERfS5_
#[doc(alias = "RBX::BuoyancyBallContact::getSurfaceAreaInDirection(G3D::Vector3 const&,float &,float &)")]
// was: RBX::BuoyancyBallContact::getSurfaceAreaInDirection(G3D::Vector3 const&,float &,float &)
// IDA 0x8a41c0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a41c0() {
}

// 0x8a41d8 — __ZN3RBX19BuoyancyBallContact16getCrossSectionsEiRKN3G3D7Vector3E
#[doc(alias = "RBX::BuoyancyBallContact::getCrossSections(int,G3D::Vector3 const&)")]
// was: RBX::BuoyancyBallContact::getCrossSections(int,G3D::Vector3 const&)
// IDA 0x8a41d8: 8 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a41d8() {
}

// 0x8a4a98 — __ZN3RBX18BuoyancyBoxContact16getCrossSectionsEiRKN3G3D7Vector3E
#[doc(alias = "RBX::BuoyancyBoxContact::getCrossSections(int,G3D::Vector3 const&)")]
// was: RBX::BuoyancyBoxContact::getCrossSections(int,G3D::Vector3 const&)
// IDA 0x8a4a98: 109 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a4a98() {
}

// 0x8a4c00 — __ZN3RBX18BuoyancyBoxContact25getSurfaceAreaInDirectionERKN3G3D7Vector3ERfS5_
#[doc(alias = "RBX::BuoyancyBoxContact::getSurfaceAreaInDirection(G3D::Vector3 const&,float &,float &)")]
// was: RBX::BuoyancyBoxContact::getSurfaceAreaInDirection(G3D::Vector3 const&,float &,float &)
// IDA 0x8a4c00: 79 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a4c00() {
}

// 0x8a6760 — __ZN3RBX17BuoyancyConnectorC1EPNS_4BodyES2_RKN3G3D7Vector3E
#[doc(alias = "RBX::BuoyancyConnector::BuoyancyConnector(RBX::Body *,RBX::Body *,G3D::Vector3 const&)")]
// was: RBX::BuoyancyConnector::BuoyancyConnector(RBX::Body *,RBX::Body *,G3D::Vector3 const&)
// IDA 0x8a6760: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8a6760() {
}

// 0x8a6764 — __ZN3RBX17BuoyancyConnectorC2EPNS_4BodyES2_RKN3G3D7Vector3E
#[doc(alias = "RBX::BuoyancyConnector::BuoyancyConnector(RBX::Body *,RBX::Body *,G3D::Vector3 const&)")]
// was: RBX::BuoyancyConnector::BuoyancyConnector(RBX::Body *,RBX::Body *,G3D::Vector3 const&)
// IDA 0x8a6764: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a6764() {
}

// 0x8a8a44 — __ZN3RBX17ManualJointHelper29createTerrainJointSurfacePairERNS_9PrimitiveERmS2_RN3G3D12Vector3int16E
#[doc(alias = "RBX::ManualJointHelper::createTerrainJointSurfacePair(RBX::Primitive &,unsigned long &,RBX::Primitive &,G3D::Vector3int16 &)")]
// was: RBX::ManualJointHelper::createTerrainJointSurfacePair(RBX::Primitive &,unsigned long &,RBX::Primitive &,G3D::Vector3int16 &)
// IDA 0x8a8a44: 342 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8a8a44() {
}

// 0x8affc0 — __ZN3RBX16UserInputService26moveLocalCharacterInternalEN5boost8weak_ptrINS_9DataModelEEEN3G3D7Vector2Ef
#[doc(alias = "RBX::UserInputService::moveLocalCharacterInternal(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float)")]
// was: RBX::UserInputService::moveLocalCharacterInternal(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float)
// IDA 0x8affc0: 230 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8affc0() {
}

// 0x8b078c — __ZN3RBX16UserInputService18moveLocalCharacterEN3G3D7Vector2Ef
#[doc(alias = "RBX::UserInputService::moveLocalCharacter(G3D::Vector2,float)")]
// was: RBX::UserInputService::moveLocalCharacter(G3D::Vector2,float)
// IDA 0x8b078c: 159 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8b078c() {
}

// 0x8b0d38 — __ZN3RBX16UserInputService12rotateCameraEN3G3D7Vector2E
#[doc(alias = "RBX::UserInputService::rotateCamera(G3D::Vector2)")]
// was: RBX::UserInputService::rotateCamera(G3D::Vector2)
// IDA 0x8b0d38: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8b0d38() {
}

// 0x8b0fe0 — __ZN3RBX16UserInputService17rotateCameraSpeedEN3G3D7Vector2EbNS_6Camera13CameraPanModeE
#[doc(alias = "RBX::UserInputService::rotateCameraSpeed(G3D::Vector2,bool,RBX::Camera::CameraPanMode)")]
// was: RBX::UserInputService::rotateCameraSpeed(G3D::Vector2,bool,RBX::Camera::CameraPanMode)
// IDA 0x8b0fe0: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8b0fe0() {
}

// 0x8b1fb8 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvN3G3D7Vector2EEN3rbx6signalIS5_EEMS2_S8_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::~EventDesc()
// IDA 0x8b1fb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8b1fb8() {
}

// 0x8b2168 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9DataModelEEEN3G3D7Vector2EfS4_S6_fEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float>::type> boost::bind<void,rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float,rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float>(void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list_av_3<boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float>::type> boost::bind<void,boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float,boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float>(void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float)
// IDA 0x8b2168: 135 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8b2168() {
}

// 0x8b5588 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEEEC2ES7_SA_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>)
// IDA 0x8b5588: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8b5588() {
}

// 0x8bab5c — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS6_5list3INS6_5valueIS9_EENSF_ISB_EENSF_IfEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>)
// IDA 0x8bab5c: 112 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8bab5c() {
}

// 0x8baca4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEN3G3D7Vector2EfENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IfEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x8baca4: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8baca4() {
}

// 0x8bacc0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEN3G3D7Vector2EfENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IfEEEEEEvPS7_E6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)
// IDA 0x8bacc0: 3 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8bacc0() {
}

// 0x8bacc8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN3G3D7Vector2EfENS8_5list3INS8_5valueISB_EENSH_ISD_EENSH_IfEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &)const
// IDA 0x8bacc8: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8bacc8() {
}

// 0x8badfc — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN3G3D7Vector2EfENS8_5list3INS8_5valueISB_EENSH_ISD_EENSH_IfEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x8badfc: 104 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8badfc() {
}

// 0x8baf2c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN3G3D7Vector2EfENS8_5list3INS8_5valueISB_EENSH_ISD_EENSH_IfEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x8baf2c: 106 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8baf2c() {
}

// 0x8bb05c — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEENS2_IfEEEclIPFvS6_S9_fENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::operator()<void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::operator()<void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float) &,boost::_bi::list1<RBX::DataModel*&> &,int)
// IDA 0x8bb05c: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8bb05c() {
}

// 0x8bb178 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEN3G3D7Vector2EfENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IfEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x8bb178: 160 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8bb178() {
}

// 0x8bb328 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEENS2_IfEEEC2ES7_SA_SB_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::list3(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)
// IDA 0x8bb328: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8bb328() {
}

// 0x8bb450 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEENS2_IfEEEC2ES7_SA_SB_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)
// IDA 0x8bb450: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8bb450() {
}

// 0x8c2668 — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvN3G3D7Vector2EEN3rbx6signalIS5_EEMS2_S8_EC2ES9_PKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::EventDesc(rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x8c2668: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2668() {
}

// 0x8c27ec — __ZN3RBX10Reflection9EventDescINS_16UserInputServiceEFvN3G3D7Vector2EEN3rbx6signalIS5_EEMS2_S8_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::~EventDesc()
// IDA 0x8c27ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8c27ec() {
}

// 0x8c28a0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvN3G3D7Vector2EEN3rbx6signalIS5_EEMS2_S8_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// IDA 0x8c28a0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c28a0() {
}

// 0x8c29f4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_16UserInputServiceEFvN3G3D7Vector2EEN3rbx6signalIS5_EEMS2_S8_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// IDA 0x8c29f4: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c29f4() {
}

// 0x8c2a88 — __ZNK3RBX10Reflection13EventDescBaseINS_16UserInputServiceEFvN3G3D7Vector2EEN3rbx6signalIS5_EEMS2_S8_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::UserInputService,void ()(G3D::Vector2),rbx::signal<void ()(G3D::Vector2)>,rbx::signal<void ()(G3D::Vector2)> RBX::UserInputService::*>::disconnectAll(RBX::Reflection::EventSource *)const
// IDA 0x8c2a88: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2a88() {
}

// 0x8c2a9c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::disconnectAll(void)
// IDA 0x8c2a9c: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2a9c() {
}

// 0x8c2c14 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSERKS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)
// IDA 0x8c2c14: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2c14() {
}

// 0x8c2c38 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_init_mutex(void)
// IDA 0x8c2c38: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8c2c38() {
}

// 0x8c2c3c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_do_get_mutex(void)
// IDA 0x8c2c3c: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2c3c() {
}

// 0x8c2d34 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector2EEEclES3_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector2)>::operator()(G3D::Vector2)")]
// was: rbx::signals::signal_with_args<1,void ()(G3D::Vector2)>::operator()(G3D::Vector2)
// IDA 0x8c2d34: 84 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2d34() {
}

// 0x8c2e90 — __ZN3rbx8any_castIRKN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x8c2e90: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2e90() {
}

// 0x8c2f80 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> &)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> &)
// IDA 0x8c2f80: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c2f80() {
}

// 0x8c30e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)
// IDA 0x8c30e0: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c30e0() {
}

// 0x8c3108 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2ENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector2 const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector2 const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
// IDA 0x8c3108: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3108() {
}

// 0x8c3224 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IN3G3D7Vector2EEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector2>(G3D::Vector2 const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector2>(G3D::Vector2 const&)
// IDA 0x8c3224: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3224() {
}

// 0x8c3368 — __ZN5boost9function1IvN3G3D7Vector2EE5clearEv
#[doc(alias = "boost::function1<void,G3D::Vector2>::clear(void)")]
// was: boost::function1<void,G3D::Vector2>::clear(void)
// IDA 0x8c3368: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3368() {
}

// 0x8c3398 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector2>::construct_func(char const*,char *)
// IDA 0x8c3398: 7 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3398() {
}

// 0x8c33a8 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<G3D::Vector2>::destruct_func(char *)
// IDA 0x8c33a8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_8c33a8() {
}

// 0x8c3578 — __ZN5boost9function1IvN3G3D7Vector2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: void boost::function1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
// IDA 0x8c3578: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3578() {
}

// 0x8c3670 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x8c3670: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3670() {
}

// 0x8c368c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector2>::invoke(boost::detail::function::function_buffer &,G3D::Vector2)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector2>::invoke(boost::detail::function::function_buffer &,G3D::Vector2)
// IDA 0x8c368c: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c368c() {
}

// 0x8c3694 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// IDA 0x8c3694: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3694() {
}

// 0x8c377c — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x8c377c: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c377c() {
}

// 0x8c3860 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector2EE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x8c3860: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3860() {
}

// 0x8c3934 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector2>(G3D::Vector2 &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector2>(G3D::Vector2 &)
// IDA 0x8c3934: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3934() {
}

// 0x8c394c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x8c394c: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c394c() {
}

// 0x8c3aa4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)
// IDA 0x8c3aa4: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3aa4() {
}

// 0x8c3b98 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6insertEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)
// IDA 0x8c3b98: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3b98() {
}

// 0x8c3da4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSEPS8_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)
// IDA 0x8c3da4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3da4() {
}

// 0x8c3dc8 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)
// IDA 0x8c3dc8: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c3dc8() {
}

// 0x8c3ec4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()
// IDA 0x8c3ec4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8c3ec4() {
}

// 0x8c3fd4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()
// IDA 0x8c3fd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8c3fd4() {
}

// 0x8c4104 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::slot::disconnect(void)
// IDA 0x8c4104: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c4104() {
}

// 0x8c4214 — __ZNK3rbx7signals6signalIFvN3G3D7Vector2EEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::slot::connected(void)const
// IDA 0x8c4214: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c4214() {
}

// 0x8c4220 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)
// IDA 0x8c4220: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8c4220() {
}
