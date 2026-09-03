//! rendering generated_38 — Ogre::|G3D:: strict 13333 total, 4806 prior, 120 this batch — 0x7522b0..0x816a6c
//! EA-sorted ascending earliest gap after 0x7520ac (next after 0x7522b0); rbx_core::SharedPtr not boost
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x7522b0 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY18RightAngleRampMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)
// IDA 0x7522b0: 57 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7522b0() {
}

// 0x752354 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY18RightAngleRampMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)
// IDA 0x752354: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_752354() {
}

// 0x752358 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY18RightAngleRampMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)
// IDA 0x752358: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752358() {
}

// 0x752468 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY18RightAngleRampMeshENS_15Vector3ComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::StaticData::~StaticData()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::RightAngleRampMesh,RBX::Vector3Comparer>::StaticData::~StaticData()
// IDA 0x752468: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_752468() {
}

// 0x7529e8 — __ZN3RBX10RigidJoint14faceIdToCoordsEPNS_9PrimitiveES2_NS_8NormalIdES3_RN3G3D15CoordinateFrameES6_
#[doc(alias = "RBX::RigidJoint::faceIdToCoords(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
// was: RBX::RigidJoint::faceIdToCoords(RBX::Primitive *,RBX::Primitive *,RBX::NormalId,RBX::NormalId,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
// IDA 0x7529e8: 106 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7529e8() {
}

// 0x752d94 — __ZN3RBX11RotateJoint18surfaceTypeToJointENS_11SurfaceTypeEPNS_9PrimitiveES3_RKN3G3D15CoordinateFrameES7_
#[doc(alias = "RBX::RotateJoint::surfaceTypeToJoint(RBX::SurfaceType,RBX::Primitive *,RBX::Primitive *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&)")]
// was: RBX::RotateJoint::surfaceTypeToJoint(RBX::SurfaceType,RBX::Primitive *,RBX::Primitive *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&)
// IDA 0x752d94: 215 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_752d94() {
}

// 0x7536b8 — __ZN3RBXL16axleOverlapsHoleERN3G3D7Vector3ES2_S2_S2_
#[doc(alias = "RBX::axleOverlapsHole(G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)")]
// was: RBX::axleOverlapsHole(G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &,G3D::Vector3 &)
// IDA 0x7536b8: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7536b8() {
}

// 0x75a244 — __ZN3G3D5ArrayIPN3RBX7ContactELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Contact *,10,32ul>::append(RBX::Contact * const&)")]
// was: G3D::Array<RBX::Contact *,10,32ul>::append(RBX::Contact * const&)
// IDA 0x75a244: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75a244() {
}

// 0x75a2a0 — __ZN3G3D5ArrayIPN3RBX7ContactELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Contact *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Contact *,10,32ul>::resize(int,bool)
// IDA 0x75a2a0: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75a2a0() {
}

// 0x75a358 — __ZN3G3D5ArrayIPN3RBX7ContactELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Contact *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Contact *,10,32ul>::realloc(int)
// IDA 0x75a358: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75a358() {
}

// 0x75af24 — __ZN3G3D5ArrayIPN3RBX7ContactELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Contact *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Contact *,10,32ul>::~Array()
// IDA 0x75af24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75af24() {
}

// 0x75aff8 — __ZN3G3D5ArrayIPN3RBX7ContactELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Contact *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Contact *,10,32ul>::Array(void)
// IDA 0x75aff8: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75aff8() {
}

// 0x75c6b4 — __ZN3G3D5ArrayIN3RBX13SpatialFilter16MoveInstructionsELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::append(RBX::SpatialFilter::MoveInstructions const&)")]
// was: G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::append(RBX::SpatialFilter::MoveInstructions const&)
// IDA 0x75c6b4: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75c6b4() {
}

// 0x75c8cc — __ZN3G3D5ArrayIN3RBX13SpatialFilter16MoveInstructionsELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::resize(int,bool)
// IDA 0x75c8cc: 78 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75c8cc() {
}

// 0x75c99c — __ZN3G3D5ArrayIN3RBX13SpatialFilter16MoveInstructionsELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::realloc(int)
// IDA 0x75c99c: 152 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75c99c() {
}

// 0x75cd54 — __ZN3G3D5ArrayIN3RBX13SpatialFilter16MoveInstructionsELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::~Array()")]
// was: G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::~Array()
// IDA 0x75cd54: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75cd54() {
}

// 0x75ce28 — __ZN3G3D5ArrayIN3RBX13SpatialFilter16MoveInstructionsELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::SpatialFilter::MoveInstructions,10,32ul>::Array(void)
// IDA 0x75ce28: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ce28() {
}

// 0x75cff4 — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::~Array()")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::~Array()
// IDA 0x75cff4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75cff4() {
}

// 0x75d0c8 — __ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::Array(void)
// IDA 0x75d0c8: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75d0c8() {
}

// 0x75ff68 — __ZNK3RBX9WedgePoly17getCenterToCornerERKN3G3D7Matrix3E
#[doc(alias = "RBX::WedgePoly::getCenterToCorner(G3D::Matrix3 const&)const")]
// was: RBX::WedgePoly::getCenterToCorner(G3D::Matrix3 const&)const
// IDA 0x75ff68: 103 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75ff68() {
}

// 0x7600c4 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY9WedgeMeshENS1_15Vector3ComparerEE5TokenEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token> const&)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token> const&)
// IDA 0x7600c4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7600c4() {
}

// 0x7600fc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9WedgeMeshENS_15Vector3ComparerEE8getTokenERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)
// IDA 0x7600fc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7600fc() {
}

// 0x760338 — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY9WedgeMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
// was: std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)
// IDA 0x760338: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760338() {
}

// 0x7603ac — __ZN5boost10shared_ptrIN3RBX12GeometryPoolIN3G3D7Vector3ENS1_4POLY9WedgeMeshENS1_15Vector3ComparerEE5TokenEEC2IS9_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token *)")]
// was: boost::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::shared_ptr<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token *)
// IDA 0x7603ac: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7603ac() {
}

// 0x760480 — __ZN5boost6detail12shared_countC2IN3RBX12GeometryPoolIN3G3D7Vector3ENS3_4POLY9WedgeMeshENS3_15Vector3ComparerEE5TokenEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token *)")]
// was: boost::detail::shared_count::shared_count<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>(RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token *)
// IDA 0x760480: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760480() {
}

// 0x760590 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9WedgeMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *)
// IDA 0x760590: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760590() {
}

// 0x76076c — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9WedgeMeshENS_15Vector3ComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()
// IDA 0x76076c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_76076c() {
}

// 0x760904 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)
// IDA 0x760904: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760904() {
}

// 0x76092c — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>)
// IDA 0x76092c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76092c() {
}

// 0x76098c — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>> *)
// IDA 0x76098c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76098c() {
}

// 0x7609b4 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)
// IDA 0x7609b4: 34 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7609b4() {
}

// 0x760a10 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)
// IDA 0x760a10: 34 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760a10() {
}

// 0x760a6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9WedgeMeshENS2_15Vector3ComparerEE5TokenEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
// IDA 0x760a6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_760a6c() {
}

// 0x760a70 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9WedgeMeshENS2_15Vector3ComparerEE5TokenEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::~sp_counted_impl_p()
// IDA 0x760a70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_760a70() {
}

// 0x760a74 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9WedgeMeshENS2_15Vector3ComparerEE5TokenEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::dispose(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::dispose(void)
// IDA 0x760a74: 57 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760a74() {
}

// 0x760b1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9WedgeMeshENS2_15Vector3ComparerEE5TokenEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::get_deleter(std::type_info const&)
// IDA 0x760b1c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760b1c() {
}

// 0x760b20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GeometryPoolIN3G3D7Vector3ENS2_4POLY9WedgeMeshENS2_15Vector3ComparerEE5TokenEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_p<RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::Token>::get_untyped_deleter(void)
// IDA 0x760b20: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760b20() {
}

// 0x760b24 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0x760b24: 147 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760b24() {
}

// 0x760ccc — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0x760ccc: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760ccc() {
}

// 0x760d70 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *> const&)
// IDA 0x760d70: 73 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760d70() {
}

// 0x760e44 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9WedgeMeshENS_15Vector3ComparerEE10ValueCountC2ERKS2_
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)
// IDA 0x760e44: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760e44() {
}

// 0x760f74 — __ZN3RBX4POLY9WedgeMeshC2ERKN3G3D7Vector3E
#[doc(alias = "RBX::POLY::WedgeMesh::WedgeMesh(G3D::Vector3 const&)")]
// was: RBX::POLY::WedgeMesh::WedgeMesh(G3D::Vector3 const&)
// IDA 0x760f74: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_760f74() {
}

// 0x76116c — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9WedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
// was: std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)
// IDA 0x76116c: 57 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76116c() {
}

// 0x761210 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9WedgeMeshENS_15Vector3ComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::safe_static_init_staticData(void)
// IDA 0x761210: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_761210() {
}

// 0x761214 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9WedgeMeshENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)
// IDA 0x761214: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_761214() {
}

// 0x761324 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9WedgeMeshENS_15Vector3ComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::StaticData::~StaticData()")]
// was: RBX::GeometryPool<G3D::Vector3,RBX::POLY::WedgeMesh,RBX::Vector3Comparer>::StaticData::~StaticData()
// IDA 0x761324: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_761324() {
}

// 0x764090 — __ZN3RBX12AppendFallenEPNS_9PrimitiveEPN3G3D5ArrayIS1_Li10ELm32EEE
#[doc(alias = "RBX::AppendFallen(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> *)")]
// was: RBX::AppendFallen(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> *)
// IDA 0x764090: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_764090() {
}

// 0x7640a8 — __ZNK3RBX5World13computeFallenERN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::World::computeFallen(G3D::Array<RBX::Primitive *,10,32ul> &)const")]
// was: RBX::World::computeFallen(G3D::Array<RBX::Primitive *,10,32ul> &)const
// IDA 0x7640a8: 124 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7640a8() {
}

// 0x76526c — __ZN3RBX5World24destroyTerrainWeldJointsEPNS_9PrimitiveERiRKN3G3D5ArrayIS2_Li10ELm32EEE
#[doc(alias = "RBX::World::destroyTerrainWeldJoints(RBX::Primitive *,int &,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::World::destroyTerrainWeldJoints(RBX::Primitive *,int &,G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x76526c: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76526c() {
}

// 0x76542c — __ZN3RBX5World24destroyAutoJointsToWorldERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::World::destroyAutoJointsToWorld(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::World::destroyAutoJointsToWorld(G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x76542c: 138 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76542c() {
}

// 0x7656a0 — __ZN3RBX5World23createAutoJointsToWorldERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::World::createAutoJointsToWorld(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::World::createAutoJointsToWorld(G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x7656a0: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7656a0() {
}

// 0x765810 — __ZN3RBX5World28createAutoJointsToPrimitivesERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
#[doc(alias = "RBX::World::createAutoJointsToPrimitives(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// was: RBX::World::createAutoJointsToPrimitives(G3D::Array<RBX::Primitive *,10,32ul> const&)
// IDA 0x765810: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_765810() {
}

// 0x765a2c — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::append(RBX::World::TouchInfo const&)")]
// was: G3D::Array<RBX::World::TouchInfo,10,32ul>::append(RBX::World::TouchInfo const&)
// IDA 0x765a2c: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_765a2c() {
}

// 0x766198 — __ZN3G3D5ArrayIN5boost10shared_ptrIN3RBX13JointInstanceEEELi10ELm32EE6removeEPS5_i
#[doc(alias = "G3D::Array<rbx_core::SharedPtr<RBX::JointInstance>,10,32ul>::remove(rbx_core::SharedPtr<RBX::JointInstance>*,int)")]
// was: G3D::Array<boost::shared_ptr<RBX::JointInstance>,10,32ul>::remove(boost::shared_ptr<RBX::JointInstance>*,int)
// IDA 0x766198: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_766198() {
}

// 0x76621c — __ZN3G3D5ArrayIN5boost10shared_ptrIN3RBX13JointInstanceEEELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<rbx_core::SharedPtr<RBX::JointInstance>,10,32ul>::resize(int,bool)")]
// was: G3D::Array<boost::shared_ptr<RBX::JointInstance>,10,32ul>::resize(int,bool)
// IDA 0x76621c: 89 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76621c() {
}

// 0x76630c — __ZN3G3D5ArrayIN5boost10shared_ptrIN3RBX13JointInstanceEEELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<rbx_core::SharedPtr<RBX::JointInstance>,10,32ul>::realloc(int)")]
// was: G3D::Array<boost::shared_ptr<RBX::JointInstance>,10,32ul>::realloc(int)
// IDA 0x76630c: 162 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_76630c() {
}

// 0x766534 — __ZN3G3D5ArrayIN5boost10shared_ptrIN3RBX13JointInstanceEEELi10ELm32EE6appendERKS5_
#[doc(alias = "G3D::Array<rbx_core::SharedPtr<RBX::JointInstance>,10,32ul>::append(rbx_core::SharedPtr<RBX::JointInstance> const&)")]
// was: G3D::Array<boost::shared_ptr<RBX::JointInstance>,10,32ul>::append(boost::shared_ptr<RBX::JointInstance> const&)
// IDA 0x766534: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_766534() {
}

// 0x7669b8 — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveEPN3G3D5ArrayIS6_Li10ELm32EEEENS3_5list2INS2_3argILi1EEENS3_5valueISA_EEEEEEEEvT_S6_
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Array<RBX::Primitive *,10,32ul> *>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Array<RBX::Primitive *,10,32ul> *>>>,RBX::Primitive *)")]
// was: void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Array<RBX::Primitive *,10,32ul> *>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> *),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Array<RBX::Primitive *,10,32ul> *>>>,RBX::Primitive *)
// IDA 0x7669b8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7669b8() {
}

// 0x766e14 — __ZN3G3D5ArrayIN5boost10shared_ptrIN3RBX13JointInstanceEEELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<rbx_core::SharedPtr<RBX::JointInstance>,10,32ul>::~Array()")]
// was: G3D::Array<boost::shared_ptr<RBX::JointInstance>,10,32ul>::~Array()
// IDA 0x766e14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_766e14() {
}

// 0x766f24 — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::~Array()")]
// was: G3D::Array<RBX::World::TouchInfo,10,32ul>::~Array()
// IDA 0x766f24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_766f24() {
}

// 0x766ff8 — __ZN3G3D5ArrayIN5boost10shared_ptrIN3RBX13JointInstanceEEELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<rbx_core::SharedPtr<RBX::JointInstance>,10,32ul>::Array(void)")]
// was: G3D::Array<boost::shared_ptr<RBX::JointInstance>,10,32ul>::Array(void)
// IDA 0x766ff8: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_766ff8() {
}

// 0x7670e8 — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::World::TouchInfo,10,32ul>::Array(void)
// IDA 0x7670e8: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7670e8() {
}

// 0x7a2554 — __ZN3RBXL20renderClassicChatBoxEPNS_5AdornEN3G3D7Vector2ERSt5dequeIN5boost10shared_ptrINS_8ChatLineEEESaIS8_EERKNS2_6Color4E
#[doc(alias = "RBX::renderClassicChatBox(RBX::Adorn *,G3D::Vector2,std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>> &,G3D::Color4 const&)")]
// was: RBX::renderClassicChatBox(RBX::Adorn *,G3D::Vector2,std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>> &,G3D::Color4 const&)
// IDA 0x7a2554: 525 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a2554() {
}

// 0x7a2db0 — __ZN3RBX10ChatOutput13renderBubblesEPNS_5AdornEN5boost8weak_ptrIKNS_8InstanceEEENS4_INS_12PartInstanceEEEbN3G3D7Vector3ESB_
#[doc(alias = "RBX::ChatOutput::renderBubbles(RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3)")]
// was: RBX::ChatOutput::renderBubbles(RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3)
// IDA 0x7a2db0: 779 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a2db0() {
}

// 0x7a3f74 — __ZN3RBX16AdornBillboarder10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE
#[doc(alias = "RBX::AdornBillboarder::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)")]
// was: RBX::AdornBillboarder::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)
// IDA 0x7a3f74: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a3f74() {
}

// 0x7a45f4 — __ZN5boost4bindIvN3RBX10ChatOutputEPNS1_5AdornENS_8weak_ptrIKNS1_8InstanceEEENS5_INS1_12PartInstanceEEEbN3G3D7Vector3ESC_PS2_NS_3argILi2EEES8_SA_bSC_SC_EENS_3_bi6bind_tIT_NS_4_mfi3mf6ISI_T0_T1_T2_T3_T4_T5_T6_EENSG_9list_av_7IT7_T8_T9_T10_T11_T12_T13_E4typeEEEMSL_FSI_SM_SN_SO_SP_SQ_SR_ESU_SV_SW_SX_SY_SZ_S10_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list_av_7<RBX::ChatOutput*,boost::arg<2>,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>::type> boost::bind<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3,RBX::ChatOutput*,boost::arg<2>,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>(void (RBX::ChatOutput::*)(RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3),RBX::ChatOutput*,boost::arg<2>,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list_av_7<RBX::ChatOutput*,boost::arg<2>,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>::type> boost::bind<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3,RBX::ChatOutput*,boost::arg<2>,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>(void (RBX::ChatOutput::*)(RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3),RBX::ChatOutput*,boost::arg<2>,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3)
// IDA 0x7a45f4: 228 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a45f4() {
}

// 0x7a4d7c — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEbN3G3D7Vector3ESK_EENS8_5list7INS8_5valueIPSC_EENS_3argILi2EEENSN_ISG_EENSN_ISI_EENSN_IbEENSN_ISK_EESV_EEEEEEvT_
#[doc(alias = "void boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>)")]
// was: void boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>)
// IDA 0x7a4d7c: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a4d7c() {
}

// 0x7a4f1c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf6IvN3RBX10ChatOutputEPNS7_5AdornENS_8weak_ptrIKNS7_8InstanceEEENSB_INS7_12PartInstanceEEEbN3G3D7Vector3ESI_EENS3_5list7INS3_5valueIPS8_EENS_3argILi2EEENSL_ISE_EENSL_ISG_EENSL_IbEENSL_ISI_EEST_EEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x7a4f1c: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a4f1c() {
}

// 0x7a4f38 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf6IvN3RBX10ChatOutputEPNS7_5AdornENS_8weak_ptrIKNS7_8InstanceEEENSB_INS7_12PartInstanceEEEbN3G3D7Vector3ESI_EENS3_5list7INS3_5valueIPS8_EENS_3argILi2EEENSL_ISE_EENSL_ISG_EENSL_IbEENSL_ISI_EEST_EEEEvPNS7_12BillboardGuiESA_E6invokeERNS1_15function_bufferESX_SA_
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,void,RBX::BillboardGui *,RBX::Adorn *>::invoke(boost::detail::function::function_buffer &,RBX::BillboardGui *,RBX::Adorn *)")]
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,void,RBX::BillboardGui *,RBX::Adorn *>::invoke(boost::detail::function::function_buffer &,RBX::BillboardGui *,RBX::Adorn *)
// IDA 0x7a4f38: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a4f38() {
}

// 0x7a4f5c — __ZNK5boost6detail8function13basic_vtable2IvPN3RBX12BillboardGuiEPNS3_5AdornEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf6IvNS3_10ChatOutputES7_NS_8weak_ptrIKNS3_8InstanceEEENSF_INS3_12PartInstanceEEEbN3G3D7Vector3ESM_EENSA_5list7INSA_5valueIPSE_EENS_3argILi2EEENSP_ISI_EENSP_ISK_EENSP_IbEENSP_ISM_EESX_EEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const
// IDA 0x7a4f5c: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a4f5c() {
}

// 0x7a50e8 — __ZNK5boost6detail8function13basic_vtable2IvPN3RBX12BillboardGuiEPNS3_5AdornEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf6IvNS3_10ChatOutputES7_NS_8weak_ptrIKNS3_8InstanceEEENSF_INS3_12PartInstanceEEEbN3G3D7Vector3ESM_EENSA_5list7INSA_5valueIPSE_EENS_3argILi2EEENSP_ISI_EENSP_ISK_EENSP_IbEENSP_ISM_EESX_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x7a50e8: 146 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a50e8() {
}

// 0x7a5270 — __ZNK5boost6detail8function13basic_vtable2IvPN3RBX12BillboardGuiEPNS3_5AdornEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf6IvNS3_10ChatOutputES7_NS_8weak_ptrIKNS3_8InstanceEEENSF_INS3_12PartInstanceEEEbN3G3D7Vector3ESM_EENSA_5list7INSA_5valueIPSE_EENS_3argILi2EEENSP_ISI_EENSP_ISK_EENSP_IbEENSP_ISM_EESX_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x7a5270: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a5270() {
}

// 0x7a53b0 — __ZN5boost3_bi5list7INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEENS2_IbEENS2_IN3G3D7Vector3EEESK_EclINS_4_mfi3mf6IvS4_PNS3_5AdornESC_SF_bSJ_SJ_EENS0_5list2IRPNS3_12BillboardGuiERSQ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list7<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>::operator()<boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list2<RBX::BillboardGui *&,RBX::Adorn *&>>(boost::_bi::type<void>,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3> &,boost::_bi::list2<RBX::BillboardGui *&,RBX::Adorn *&> &,int)")]
// was: void boost::_bi::list7<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>::operator()<boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list2<RBX::BillboardGui *&,RBX::Adorn *&>>(boost::_bi::type<void>,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3> &,boost::_bi::list2<RBX::BillboardGui *&,RBX::Adorn *&> &,int)
// IDA 0x7a53b0: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a53b0() {
}

// 0x7a54ec — __ZNK5boost4_mfi3mf6IvN3RBX10ChatOutputEPNS2_5AdornENS_8weak_ptrIKNS2_8InstanceEEENS6_INS2_12PartInstanceEEEbN3G3D7Vector3ESD_EclEPS3_S5_S9_SB_bSD_SD_
#[doc(alias = "boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>::operator()(RBX::ChatOutput*,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3)const")]
// was: boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>::operator()(RBX::ChatOutput*,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3)const
// IDA 0x7a54ec: 129 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a54ec() {
}

// 0x7a5640 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf6IvN3RBX10ChatOutputEPNS7_5AdornENS_8weak_ptrIKNS7_8InstanceEEENSB_INS7_12PartInstanceEEEbN3G3D7Vector3ESI_EENS3_5list7INS3_5valueIPS8_EENS_3argILi2EEENSL_ISE_EENSL_ISG_EENSL_IbEENSL_ISI_EEST_EEEEE7managerERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,rbx_core::WeakPtr<RBX::Instance const>,rbx_core::WeakPtr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf6<void,RBX::ChatOutput,RBX::Adorn *,boost::weak_ptr<RBX::Instance const>,boost::weak_ptr<RBX::PartInstance>,bool,G3D::Vector3,G3D::Vector3>,boost::_bi::list7<boost::_bi::value<RBX::ChatOutput*>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x7a5640: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a5640() {
}

// 0x7a582c — __ZN5boost3_bi5list7INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEENS2_IbEENS2_IN3G3D7Vector3EEESK_EC2ES6_S8_SD_SG_SH_SK_SK_
#[doc(alias = "boost::_bi::list7<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>::list7(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>)")]
// was: boost::_bi::list7<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>::list7(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>)
// IDA 0x7a582c: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a582c() {
}

// 0x7a5974 — __ZN5boost3_bi8storage7INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEENS2_IbEENS2_IN3G3D7Vector3EEESK_EC2ES6_S8_SD_SG_SH_SK_SK_
#[doc(alias = "boost::_bi::storage7<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>::storage7(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>)")]
// was: boost::_bi::storage7<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>>::storage7(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>,boost::_bi::value<G3D::Vector3>)
// IDA 0x7a5974: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a5974() {
}

// 0x7a5ab8 — __ZN5boost3_bi8storage6INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEENS2_IbEENS2_IN3G3D7Vector3EEEEC2ES6_S8_SD_SG_SH_SK_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>>::storage6(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<rbx_core::WeakPtr<RBX::Instance const>>,boost::_bi::value<rbx_core::WeakPtr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>)")]
// was: boost::_bi::storage6<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>>::storage6(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>,boost::_bi::value<G3D::Vector3>)
// IDA 0x7a5ab8: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a5ab8() {
}

// 0x7a9c00 — __ZN3RBX5Adorn10drawFont2DERKSsRKN3G3D7Vector2EfRKNS3_6Color4ES9_NS_4Text4FontENSA_6XAlignENSA_6YAlignES6_RKNS3_6Rect2DE
#[doc(alias = "RBX::Adorn::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)")]
// was: RBX::Adorn::drawFont2D(std::string const&,G3D::Vector2 const&,float,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::Font,RBX::Text::XAlign,RBX::Text::YAlign,G3D::Vector2 const&,G3D::Rect2D const&)
// IDA 0x7a9c00: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7a9c00() {
}

// 0x7ad2b0 — __ZNK3RBX7GuiItem7label2dEPNS_5AdornERKSsRKN3G3D6Color4ES8_NS_4Text6XAlignE
#[doc(alias = "RBX::GuiItem::label2d(RBX::Adorn *,std::string const&,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign)const")]
// was: RBX::GuiItem::label2d(RBX::Adorn *,std::string const&,G3D::Color4 const&,G3D::Color4 const&,RBX::Text::XAlign)const
// IDA 0x7ad2b0: 96 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ad2b0() {
}

// 0x7ad5d4 — __ZNK3RBX6Canvas11toPixelSizeERKN3G3D7Vector2E
#[doc(alias = "RBX::Canvas::toPixelSize(G3D::Vector2 const&)const")]
// was: RBX::Canvas::toPixelSize(G3D::Vector2 const&)const
// IDA 0x7ad5d4: 23 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ad5d4() {
}

// 0x7afdbc — __ZN3RBX12GuiDrawImage8setImageEPNS_5AdornERKNS_9TextureIdEjPN3G3D7Vector2E
#[doc(alias = "RBX::GuiDrawImage::setImage(RBX::Adorn *,RBX::TextureId const&,unsigned int,G3D::Vector2 *)")]
// was: RBX::GuiDrawImage::setImage(RBX::Adorn *,RBX::TextureId const&,unsigned int,G3D::Vector2 *)
// IDA 0x7afdbc: 1505 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7afdbc() {
}

// 0x7b132c — __ZN3RBX12GuiDrawImage4drawEPNS_5AdornEN5boost10shared_ptrINS_16TextureProxyBaseEEERKNS_4RectES9_RKN3G3D7Vector2ESD_RKNSA_6Color4ESG_
#[doc(alias = "RBX::GuiDrawImage::draw(RBX::Adorn *,rbx_core::SharedPtr<RBX::TextureProxyBase>,RBX::Rect const&,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Color4 const&,G3D::Color4 const&)")]
// was: RBX::GuiDrawImage::draw(RBX::Adorn *,boost::shared_ptr<RBX::TextureProxyBase>,RBX::Rect const&,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Color4 const&,G3D::Color4 const&)
// IDA 0x7b132c: 250 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b132c() {
}

// 0x7b163c — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectERKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)")]
// was: RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)
// IDA 0x7b163c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b163c() {
}

// 0x7b1658 — __ZN3RBX12GuiDrawImage8render2dEPNS_5AdornEbRKNS_4RectES5_RKN3G3D7Vector2ES9_NS_3Gui11WidgetStateEb
#[doc(alias = "RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)")]
// was: RBX::GuiDrawImage::render2d(RBX::Adorn *,bool,RBX::Rect const&,RBX::Rect const&,G3D::Vector2 const&,G3D::Vector2 const&,RBX::Gui::WidgetState,bool)
// IDA 0x7b1658: 437 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b1658() {
}

// 0x7b1a9c — __ZN3RBX12GuiDrawImage9computeUVERN3G3D7Vector2ES3_RKS2_S5_S5_
#[doc(alias = "RBX::GuiDrawImage::computeUV(G3D::Vector2 &,G3D::Vector2 &,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Vector2 const&)")]
// was: RBX::GuiDrawImage::computeUV(G3D::Vector2 &,G3D::Vector2 &,G3D::Vector2 const&,G3D::Vector2 const&,G3D::Vector2 const&)
// IDA 0x7b1a9c: 82 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b1a9c() {
}

// 0x7b6384 — __ZN3RBX8Humanoid14setWalkToPointERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setWalkToPoint(G3D::Vector3 const&)")]
// was: RBX::Humanoid::setWalkToPoint(G3D::Vector3 const&)
// IDA 0x7b6384: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6384() {
}

// 0x7b6464 — __ZN3RBX8Humanoid19setTargetPointLocalERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setTargetPointLocal(G3D::Vector3 const&)")]
// was: RBX::Humanoid::setTargetPointLocal(G3D::Vector3 const&)
// IDA 0x7b6464: 63 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6464() {
}

// 0x7b65ec — __ZN3RBX8Humanoid16setWalkDirectionERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setWalkDirection(G3D::Vector3 const&)")]
// was: RBX::Humanoid::setWalkDirection(G3D::Vector3 const&)
// IDA 0x7b65ec: 121 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b65ec() {
}

// 0x7b6820 — __ZN3RBX8Humanoid7moveTo2EN3G3D7Vector3EN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Humanoid::moveTo2(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Humanoid::moveTo2(G3D::Vector3,boost::shared_ptr<RBX::Instance>)
// IDA 0x7b6820: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7b6820() {
}

// 0x7bacac — __ZNK3RBX8Humanoid14hasWalkToPointERN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::hasWalkToPoint(G3D::Vector3 &)const")]
// was: RBX::Humanoid::hasWalkToPoint(G3D::Vector3 &)const
// IDA 0x7bacac: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bacac() {
}

// 0x7bb51c — __ZN3RBX8Humanoid14setTargetPointERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::setTargetPoint(G3D::Vector3 const&)")]
// was: RBX::Humanoid::setTargetPoint(G3D::Vector3 const&)
// IDA 0x7bb51c: 71 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bb51c() {
}

// 0x7bb628 — __ZN3RBX8Humanoid6moveToERKN3G3D7Vector3EPNS_12PartInstanceE
#[doc(alias = "RBX::Humanoid::moveTo(G3D::Vector3 const&,RBX::PartInstance *)")]
// was: RBX::Humanoid::moveTo(G3D::Vector3 const&,RBX::PartInstance *)
// IDA 0x7bb628: 82 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bb628() {
}

// 0x7bc6bc — __ZN3RBX8Humanoid14renderWaypointEPNS_5AdornERKN3G3D7Vector3E
#[doc(alias = "RBX::Humanoid::renderWaypoint(RBX::Adorn *,G3D::Vector3 const&)")]
// was: RBX::Humanoid::renderWaypoint(RBX::Adorn *,G3D::Vector3 const&)
// IDA 0x7bc6bc: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bc6bc() {
}

// 0x7bd338 — __ZN3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb
#[doc(alias = "RBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)")]
// was: RBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)
// IDA 0x7bd338: 164 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bd338() {
}

// 0x7bd574 — __ZThn292_N3RBX8Humanoid32setFirstPersonRotationalVelocityERKN3G3D7Vector3Eb
#[doc(alias = "non-virtual thunk to RBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)")]
// was: non-virtual thunk to RBX::Humanoid::setFirstPersonRotationalVelocity(G3D::Vector3 const&,bool)
// IDA 0x7bd574: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7bd574() {
}

// 0x7bdbd0 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::~PropDescriptor()
// IDA 0x7bdbd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7bdbd0() {
}

// 0x7bdce0 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN3G3D7Vector3EN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// IDA 0x7bdce0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7bdce0() {
}

// 0x7c9e88 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN3G3D7Vector3EN5boost10shared_ptrINS_8InstanceEEEELi2EEC2EMS2_FvS4_S8_EPKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Humanoid::*)(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,boost::shared_ptr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::Humanoid::*)(G3D::Vector3,boost::shared_ptr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x7c9e88: 178 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7c9e88() {
}

// 0x7ca054 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN3G3D7Vector3EN5boost10shared_ptrINS_8InstanceEEEELi2EE16declareSignatureEPKcNS0_7VariantESC_SD_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,boost::shared_ptr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// IDA 0x7ca054: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ca054() {
}

// 0x7ca0a0 — __ZN3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN3G3D7Vector3EN5boost10shared_ptrINS_8InstanceEEEELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// IDA 0x7ca0a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7ca0a0() {
}

// 0x7ca1c0 — __ZNK3RBX10Reflection13BoundFuncDescINS_8HumanoidEFvN3G3D7Vector3EN5boost10shared_ptrINS_8InstanceEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Humanoid,void ()(G3D::Vector3,boost::shared_ptr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0x7ca1c0: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ca1c0() {
}

// 0x7ca2bc — __ZN3RBX10Reflection11Call2HelperINS_8HumanoidEMS2_FvN3G3D7Vector3EN5boost10shared_ptrINS_8InstanceEEEES4_S8_vE4callEPS2_SA_RNS0_7VariantERKS4_RKS8_
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::Humanoid,void (RBX::Humanoid::*)(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Humanoid*,void (RBX::Humanoid::*)(G3D::Vector3,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,G3D::Vector3 const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call2Helper<RBX::Humanoid,void (RBX::Humanoid::*)(G3D::Vector3,boost::shared_ptr<RBX::Instance>),G3D::Vector3,boost::shared_ptr<RBX::Instance>,void>::call(RBX::Humanoid*,void (RBX::Humanoid::*)(G3D::Vector3,boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,G3D::Vector3 const&,boost::shared_ptr<RBX::Instance> const&)
// IDA 0x7ca2bc: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7ca2bc() {
}

// 0x7caaac — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EEC2IMS2_KFS4_vEMS2_FvRKS4_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::PropDescriptor<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::PropDescriptor<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x7caaac: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7caaac() {
}

// 0x7cabc0 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::~PropDescriptor()
// IDA 0x7cabc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7cabc0() {
}

// 0x7cabec — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isReadOnly(void)const
// IDA 0x7cabec: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cabec() {
}

// 0x7cabf0 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isWriteOnly(void)const
// IDA 0x7cabf0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cabf0() {
}

// 0x7cabf4 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x7cabf4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cabf4() {
}

// 0x7cac1c — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFS4_vEMS2_FvRKS4_EE8setValueEPNS0_13DescribedBaseESA_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const
// IDA 0x7cac1c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cac1c() {
}

// 0x7cadd0 — __ZN3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x7cadd0: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7cadd0() {
}

// 0x7caee4 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isReadOnly(void)const
// IDA 0x7caee4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7caee4() {
}

// 0x7caee8 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::isWriteOnly(void)const
// IDA 0x7caee8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7caee8() {
}

// 0x7caeec — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x7caeec: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7caeec() {
}

// 0x7caf20 — __ZNK3RBX10Reflection14PropDescriptorINS_8HumanoidEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Humanoid,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::Humanoid::*)(void)const,void (RBX::Humanoid::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const
// IDA 0x7caf20: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7caf20() {
}

// 0x7d0708 — __ZN3RBX5HUMAN13HumanoidState19AverageFloorRayCastERN5boost10shared_ptrINS_12PartInstanceEEERN3G3D7Vector3ES9_RibRKS8_fPNS_8AssemblyERKNS7_15CoordinateFrameE
#[doc(alias = "RBX::HUMAN::HumanoidState::AverageFloorRayCast(rbx_core::SharedPtr<RBX::PartInstance> &,G3D::Vector3 &,G3D::Vector3 &,int &,bool,G3D::Vector3 const&,float,RBX::Assembly *,G3D::CoordinateFrame const&)")]
// was: RBX::HUMAN::HumanoidState::AverageFloorRayCast(boost::shared_ptr<RBX::PartInstance> &,G3D::Vector3 &,G3D::Vector3 &,int &,bool,G3D::Vector3 const&,float,RBX::Assembly *,G3D::CoordinateFrame const&)
// IDA 0x7d0708: 212 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d0708() {
}

// 0x7d09a8 — __ZN3RBX5HUMAN13HumanoidState8tryFloorERKNS_6RbxRayERN3G3D7Vector3EfPNS_8AssemblyE
#[doc(alias = "RBX::HUMAN::HumanoidState::tryFloor(RBX::RbxRay const&,G3D::Vector3 &,float,RBX::Assembly *)")]
// was: RBX::HUMAN::HumanoidState::tryFloor(RBX::RbxRay const&,G3D::Vector3 &,float,RBX::Assembly *)
// IDA 0x7d09a8: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d09a8() {
}

// 0x7d1230 — __ZN3RBX5HUMAN13HumanoidState15doLadderRaycastEPNS_15GeometryServiceERKNS_6RbxRayEPNS_8HumanoidEPPNS_9PrimitiveEPN3G3D7Vector3E
#[doc(alias = "RBX::HUMAN::HumanoidState::doLadderRaycast(RBX::GeometryService *,RBX::RbxRay const&,RBX::Humanoid *,RBX::Primitive **,G3D::Vector3 *)")]
// was: RBX::HUMAN::HumanoidState::doLadderRaycast(RBX::GeometryService *,RBX::RbxRay const&,RBX::Humanoid *,RBX::Primitive **,G3D::Vector3 *)
// IDA 0x7d1230: 202 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7d1230() {
}

// 0x7f964c — __ZN3RBX10Soundscape14CollisionSound4PlayEPN4FMOD6SystemEPNS2_12ChannelGroupERKN3G3D7Vector3ESA_f
#[doc(alias = "RBX::Soundscape::CollisionSound::Play(FMOD::System *,FMOD::ChannelGroup *,G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// was: RBX::Soundscape::CollisionSound::Play(FMOD::System *,FMOD::ChannelGroup *,G3D::Vector3 const&,G3D::Vector3 const&,float)
// IDA 0x7f964c: 203 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7f964c() {
}

// 0x816a34 — __ZN3RBX7Region216getRelativeErrorERKN3G3D7Vector2ERKNS0_13WeightedPointE
#[doc(alias = "RBX::Region2::getRelativeError(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&)")]
// was: RBX::Region2::getRelativeError(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&)
// IDA 0x816a34: 14 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816a34() {
}

// 0x816a6c — __ZN3RBX7Region212pointInRangeERKN3G3D7Vector2ERKNS0_13WeightedPointEf
#[doc(alias = "RBX::Region2::pointInRange(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,float)")]
// was: RBX::Region2::pointInRange(G3D::Vector2 const&,RBX::Region2::WeightedPoint const&,float)
// IDA 0x816a6c: 19 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_816a6c() {
}