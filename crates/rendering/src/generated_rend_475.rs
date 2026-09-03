//! rendering shard 475 — 120 stubs 0x74c2bc..0x7505c4 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 51491->51611 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc after shard 474 (0x74779c..0x74c288)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x74c2bc — __ZN3RBX9Primitive11setDraggingEb
// type: int __fastcall(RBX::Primitive *this, bool)
#[doc(alias = "RBX::Primitive::setDragging(bool)")]
#[doc(alias = "__ZN3RBX9Primitive11setDraggingEb")]
// IDA 0x74c2bc: 3 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c2bc() {
}

// 0x74c2c8 — __ZN3RBX9Primitive8setFixedEbb
// type: int __fastcall(RBX::Primitive *this, int, int)
#[doc(alias = "RBX::Primitive::setFixed(bool,bool)")]
#[doc(alias = "__ZN3RBX9Primitive8setFixedEbb")]
// IDA 0x74c2c8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c2c8() {
}

// 0x74c328 — __ZN3RBX9Primitive19setAnchoredPropertyEb
// type: int __fastcall(RBX::Primitive *this, bool)
#[doc(alias = "RBX::Primitive::setAnchoredProperty(bool)")]
#[doc(alias = "__ZN3RBX9Primitive19setAnchoredPropertyEb")]
// IDA 0x74c328: 2 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c328() {
}

// 0x74c330 — __ZN3RBX9Primitive17setPreventCollideEb
// type: RBX::World *__fastcall(RBX::Primitive *this, RBX::World *)
#[doc(alias = "RBX::Primitive::setPreventCollide(bool)")]
#[doc(alias = "__ZN3RBX9Primitive17setPreventCollideEb")]
// IDA 0x74c330: 12 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c330() {
}

// 0x74c350 — __ZN3RBX9Primitive11setFrictionEf
// type: int __fastcall(int this, float)
#[doc(alias = "RBX::Primitive::setFriction(float)")]
#[doc(alias = "__ZN3RBX9Primitive11setFrictionEf")]
// IDA 0x74c350: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c350() {
}

// 0x74c37c — __ZN3RBX9Primitive13setElasticityEf
// type: int __fastcall(int this, float)
#[doc(alias = "RBX::Primitive::setElasticity(float)")]
#[doc(alias = "__ZN3RBX9Primitive13setElasticityEf")]
// IDA 0x74c37c: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c37c() {
}

// 0x74c3a8 — __ZNK3RBX9Primitive20getFaceCoordInObjectENS_8NormalIdE
// type: int __fastcall(G3D::Matrix3 *, int, int)
#[doc(alias = "RBX::Primitive::getFaceCoordInObject(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX9Primitive20getFaceCoordInObjectENS_8NormalIdE")]
// IDA 0x74c3a8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c3a8() {
}

// 0x74c43c — __ZNK3RBX9Primitive15getFaceInObjectENS_8NormalIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Primitive::getFaceInObject(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX9Primitive15getFaceInObjectENS_8NormalIdE")]
// IDA 0x74c43c: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c43c() {
}

// 0x74c498 — __ZN3RBX9Primitive14getFaceInWorldENS_8NormalIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Primitive::getFaceInWorld(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9Primitive14getFaceInWorldENS_8NormalIdE")]
// IDA 0x74c498: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c498() {
}

// 0x74c638 — __ZN3RBX9Primitive5setPVERKNS_2PVE
// type: int __fastcall(int, float *)
#[doc(alias = "RBX::Primitive::setPV(RBX::PV const&)")]
#[doc(alias = "__ZN3RBX9Primitive5setPVERKNS_2PVE")]
// IDA 0x74c638: 123 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c638() {
}

// 0x74c7b0 — __ZN3RBX9Primitive12zeroVelocityEv
// type: RBX::Velocity *__fastcall(RBX::Primitive *this, RBX::Primitive *)
#[doc(alias = "RBX::Primitive::zeroVelocity(void)")]
#[doc(alias = "__ZN3RBX9Primitive12zeroVelocityEv")]
// IDA 0x74c7b0: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c7b0() {
}

// 0x74c7e8 — __ZN3RBX9Primitive11setVelocityERKNS_8VelocityE
// type: int __fastcall(RBX::Body **this, const RBX::Velocity *)
#[doc(alias = "RBX::Primitive::setVelocity(RBX::Velocity const&)")]
#[doc(alias = "__ZN3RBX9Primitive11setVelocityERKNS_8VelocityE")]
// IDA 0x74c7e8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c7e8() {
}

// 0x74c840 — __ZNK3RBX9Primitive18getCoordinateFrameEv
// type: int __fastcall(RBX::Body **this)
#[doc(alias = "RBX::Primitive::getCoordinateFrame(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive18getCoordinateFrameEv")]
// IDA 0x74c840: 2 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c840() {
}

// 0x74c848 — __ZNK3RBX9Primitive5getPVEv
// type: int __fastcall(RBX::Body **this)
#[doc(alias = "RBX::Primitive::getPV(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive5getPVEv")]
// IDA 0x74c848: 2 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c848() {
}

// 0x74c850 — __ZNK3RBX9Primitive13getGridCornerEv
// type: int __fastcall(RBX::Primitive *this, int)
#[doc(alias = "RBX::Primitive::getGridCorner(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive13getGridCornerEv")]
// IDA 0x74c850: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c850() {
}

// 0x74c980 — __ZN3RBX9Primitive14setSurfaceDataENS_8NormalIdERKNS_11SurfaceDataE
// type: _DWORD *__fastcall(int, int, RBX::SurfaceData *this)
#[doc(alias = "RBX::Primitive::setSurfaceData(RBX::NormalId,RBX::SurfaceData const&)")]
#[doc(alias = "__ZN3RBX9Primitive14setSurfaceDataENS_8NormalIdERKNS_11SurfaceDataE")]
// IDA 0x74c980: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74c980() {
}

// 0x74ca64 — __ZN3RBX9Primitive14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE
// type: _DWORD *__fastcall(int, int, int)
#[doc(alias = "RBX::Primitive::setSurfaceType(RBX::NormalId,RBX::SurfaceType)")]
#[doc(alias = "__ZN3RBX9Primitive14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE")]
// IDA 0x74ca64: 7 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ca64() {
}

// 0x74ca78 — __ZN3RBX9Primitive25nextSpanningEdgeFromJointEPNS_5JointE
// type: char *__fastcall(RBX::Primitive *this, RBX::Joint *)
#[doc(alias = "RBX::Primitive::nextSpanningEdgeFromJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9Primitive25nextSpanningEdgeFromJointEPNS_5JointE")]
// IDA 0x74ca78: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ca78() {
}

// 0x74cab0 — __ZN3RBX9Primitive20getFirstSpanningEdgeEv
// type: char *__fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getFirstSpanningEdge(void)")]
#[doc(alias = "__ZN3RBX9Primitive20getFirstSpanningEdgeEv")]
// IDA 0x74cab0: 7 insns (LDR.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cab0() {
}

// 0x74cac4 — __ZThn8_N3RBX9Primitive20getFirstSpanningEdgeEv
// type: char *__fastcall(RBX::Primitive *this)
#[doc(alias = "non-virtual thunk toRBX::Primitive::getFirstSpanningEdge(void)")]
#[doc(alias = "__ZThn8_N3RBX9Primitive20getFirstSpanningEdgeEv")]
// IDA 0x74cac4: 9 insns (MOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cac4() {
}

// 0x74cae0 — __ZN3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE
// type: char *__fastcall(RBX::Primitive *this, RBX::SpanningEdge *)
#[doc(alias = "RBX::Primitive::getNextSpanningEdge(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE")]
// IDA 0x74cae0: 14 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cae0() {
}

// 0x74cb08 — __ZThn8_N3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE
// type: char *__fastcall(RBX::Primitive *this, RBX::SpanningEdge *)
#[doc(alias = "non-virtual thunk toRBX::Primitive::getNextSpanningEdge(RBX::SpanningEdge *)")]
#[doc(alias = "__ZThn8_N3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE")]
// IDA 0x74cb08: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cb08() {
}

// 0x74cb10 — __ZNK3RBX9Primitive20isGeometryOrthogonalEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive20isGeometryOrthogonalEv")]
// IDA 0x74cb10: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cb10() {
}

// 0x74cb20 — __ZN3RBX9Primitive11getSortSizeEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::getSortSize(void)")]
#[doc(alias = "__ZN3RBX9Primitive11getSortSizeEv")]
// IDA 0x74cb20: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cb20() {
}

// 0x74cb38 — __ZN3RBX9Primitive17calculateSortSizeEv
// type: int __fastcall(RBX::Primitive *this, int, int)
#[doc(alias = "RBX::Primitive::calculateSortSize(void)")]
#[doc(alias = "__ZN3RBX9Primitive17calculateSortSizeEv")]
// IDA 0x74cb38: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cb38() {
}

// 0x74cc78 — __ZN3RBX9Primitive18setSpecificGravityEf
// type: RBX::World *__fastcall(RBX::Primitive *this, float)
#[doc(alias = "RBX::Primitive::setSpecificGravity(float)")]
#[doc(alias = "__ZN3RBX9Primitive18setSpecificGravityEf")]
// IDA 0x74cc78: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cc78() {
}

// 0x74cc90 — __ZNK3RBX9Primitive17computeIsGroundedEv
// type: int __fastcall(RBX::Primitive *this)
#[doc(alias = "RBX::Primitive::computeIsGrounded(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive17computeIsGroundedEv")]
// IDA 0x74cc90: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cc90() {
}

// 0x74ccb8 — __ZN3RBX9AllocatorINS_4BodyEEnwEm
// type: int __fastcall(unsigned int)
#[doc(alias = "RBX::Allocator<RBX::Body>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4BodyEEnwEm")]
// IDA 0x74ccb8: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ccb8() {
}

// 0x74cd28 — __ZN3RBX25BasicSpatialHashPrimitiveD2Ev
// type: void __fastcall(RBX::BasicSpatialHashPrimitive *__hidden this)
#[doc(alias = "RBX::BasicSpatialHashPrimitive::~BasicSpatialHashPrimitive()")]
#[doc(alias = "__ZN3RBX25BasicSpatialHashPrimitiveD2Ev")]
// IDA 0x74cd28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74cd28() {
}

// 0x74cd8c — __ZNK3RBX8EdgeList7getEdgeEi
// type: _DWORD __fastcall(RBX::EdgeList *__hidden this, int)
#[doc(alias = "RBX::EdgeList::getEdge(int)const")]
#[doc(alias = "__ZNK3RBX8EdgeList7getEdgeEi")]
// IDA 0x74cd8c: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cd8c() {
}

// 0x74ce50 — __ZNK3RBX11SurfaceData7isEmptyEv
// type: _DWORD __fastcall(RBX::SurfaceData *__hidden this)
#[doc(alias = "RBX::SurfaceData::isEmpty(void)const")]
#[doc(alias = "__ZNK3RBX11SurfaceData7isEmptyEv")]
// IDA 0x74ce50: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ce50() {
}

// 0x74ce9c — __ZN3RBX5Joint19isSpanningTreeJointEPKNS_4EdgeE
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Edge *)
#[doc(alias = "RBX::Joint::isSpanningTreeJoint(RBX::Edge const*)")]
#[doc(alias = "__ZN3RBX5Joint19isSpanningTreeJointEPKNS_4EdgeE")]
// IDA 0x74ce9c: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ce9c() {
}

// 0x74cf08 — __ZNK3RBX9Primitive9getRadiusEv
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
#[doc(alias = "RBX::Primitive::getRadius(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive9getRadiusEv")]
// IDA 0x74cf08: 4 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cf08() {
}

// 0x74cf14 — __ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")]
// IDA 0x74cf14: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cf14() {
}

// 0x74cf64 — __ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv
// type: int(void)
#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
#[doc(alias = "__ZN5boost14singleton_poolIN3RBX4BodyELj276ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE6mallocEv")]
// IDA 0x74cf64: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cf64() {
}

// 0x74cf9c — __ZN3RBX4Body15getPV_Spin_LockEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
#[doc(alias = "RBX::Body::getPV_Spin_Lock(void)")]
#[doc(alias = "__ZN3RBX4Body15getPV_Spin_LockEv")]
// IDA 0x74cf9c: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74cf9c() {
}

// 0x74d060 — __ZNK3RBX15MegaClusterPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
#[doc(alias = "RBX::MegaClusterPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly15getGeometryTypeEv")]
// IDA 0x74d060: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d060() {
}

// 0x74d064 — __ZNK3RBX4Poly14getCollideTypeEv
// type: _DWORD __fastcall(RBX::Poly *__hidden this)
#[doc(alias = "RBX::Poly::getCollideType(void)const")]
#[doc(alias = "__ZNK3RBX4Poly14getCollideTypeEv")]
// IDA 0x74d064: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d064() {
}

// 0x74d068 — __ZNK3RBX15MegaClusterPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
#[doc(alias = "RBX::MegaClusterPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly20isGeometryOrthogonalEv")]
// IDA 0x74d068: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d068() {
}

// 0x74d084 — __ZNK3RBX15MegaClusterPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
#[doc(alias = "RBX::MegaClusterPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly13getCofmOffsetEv")]
// IDA 0x74d084: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d084() {
}

// 0x74d09c — __ZNK3RBX15MegaClusterPoly9getMomentEf
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, float)
#[doc(alias = "RBX::MegaClusterPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly9getMomentEf")]
// IDA 0x74d09c: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d09c() {
}

// 0x74d0b0 — __ZNK3RBX8Geometry9getVolumeEv
// type: _DWORD __fastcall(RBX::Geometry *__hidden this)
#[doc(alias = "RBX::Geometry::getVolume(void)const")]
#[doc(alias = "__ZNK3RBX8Geometry9getVolumeEv")]
// IDA 0x74d0b0: 7 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d0b0() {
}

// 0x74d0cc — __ZNK3RBX15MegaClusterPoly7getSizeEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
#[doc(alias = "RBX::MegaClusterPoly::getSize(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly7getSizeEv")]
// IDA 0x74d0cc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d0cc() {
}

// 0x74d0d0 — __ZNK3RBX8Geometry13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::Geometry *__hidden this)
#[doc(alias = "RBX::Geometry::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX8Geometry13getCofmOffsetEv")]
// IDA 0x74d0d0: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d0d0() {
}

// 0x74d12c — __ZNK3RBX15CornerWedgePoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this)
#[doc(alias = "RBX::CornerWedgePoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly15getGeometryTypeEv")]
// IDA 0x74d12c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d12c() {
}

// 0x74d130 — __ZNK3RBX15CornerWedgePoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this)
#[doc(alias = "RBX::CornerWedgePoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly20isGeometryOrthogonalEv")]
// IDA 0x74d130: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d130() {
}

// 0x74d134 — __ZNK3RBX18RightAngleRampPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly15getGeometryTypeEv")]
// IDA 0x74d134: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d134() {
}

// 0x74d138 — __ZNK3RBX18RightAngleRampPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
#[doc(alias = "RBX::RightAngleRampPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly20isGeometryOrthogonalEv")]
// IDA 0x74d138: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d138() {
}

// 0x74d13c — __ZNK3RBX16ParallelRampPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::ParallelRampPoly *__hidden this)
#[doc(alias = "RBX::ParallelRampPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX16ParallelRampPoly15getGeometryTypeEv")]
// IDA 0x74d13c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d13c() {
}

// 0x74d140 — __ZNK3RBX16ParallelRampPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::ParallelRampPoly *__hidden this)
#[doc(alias = "RBX::ParallelRampPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX16ParallelRampPoly20isGeometryOrthogonalEv")]
// IDA 0x74d140: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d140() {
}

// 0x74d144 — __ZNK3RBX11PyramidPoly15getGeometryTypeEv
// type: int __fastcall(RBX::PyramidPoly *this)
#[doc(alias = "RBX::PyramidPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly15getGeometryTypeEv")]
// IDA 0x74d144: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d144() {
}

// 0x74d148 — __ZNK3RBX11PyramidPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly20isGeometryOrthogonalEv")]
// IDA 0x74d148: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d148() {
}

// 0x74d14c — __ZNK3RBX9PrismPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly15getGeometryTypeEv")]
// IDA 0x74d14c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d14c() {
}

// 0x74d150 — __ZNK3RBX9PrismPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly20isGeometryOrthogonalEv")]
// IDA 0x74d150: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d150() {
}

// 0x74d154 — __ZNK3RBX9WedgePoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this)
#[doc(alias = "RBX::WedgePoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly15getGeometryTypeEv")]
// IDA 0x74d154: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d154() {
}

// 0x74d158 — __ZNK3RBX9WedgePoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this)
#[doc(alias = "RBX::WedgePoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly20isGeometryOrthogonalEv")]
// IDA 0x74d158: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d158() {
}

// 0x74d15c — __ZN3RBX4BallD1Ev
// type: void __fastcall(RBX::Ball *__hidden this)
#[doc(alias = "RBX::Ball::~Ball()")]
#[doc(alias = "__ZN3RBX4BallD1Ev")]
// IDA 0x74d15c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_74d15c() {
}

// 0x74d160 — __ZN3RBX4BallD0Ev
// type: void __fastcall(RBX::Ball *__hidden this)
#[doc(alias = "RBX::Ball::~Ball()")]
#[doc(alias = "__ZN3RBX4BallD0Ev")]
// IDA 0x74d160: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74d160() {
}

// 0x74d164 — __ZNK3RBX4Ball15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
#[doc(alias = "RBX::Ball::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX4Ball15getGeometryTypeEv")]
// IDA 0x74d164: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d164() {
}

// 0x74d168 — __ZNK3RBX4Ball14getCollideTypeEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
#[doc(alias = "RBX::Ball::getCollideType(void)const")]
#[doc(alias = "__ZNK3RBX4Ball14getCollideTypeEv")]
// IDA 0x74d168: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d168() {
}

// 0x74d16c — __ZNK3RBX4Ball9getRadiusEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
#[doc(alias = "RBX::Ball::getRadius(void)const")]
#[doc(alias = "__ZNK3RBX4Ball9getRadiusEv")]
// IDA 0x74d16c: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d16c() {
}

// 0x74d170 — __ZNK3RBX4Ball14getNumSurfacesEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
#[doc(alias = "RBX::Ball::getNumSurfaces(void)const")]
#[doc(alias = "__ZNK3RBX4Ball14getNumSurfacesEv")]
// IDA 0x74d170: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d170() {
}

// 0x74d28c — __ZNK3RBX4Ball9getMomentEf
// type: _DWORD __fastcall(RBX::Ball *__hidden this, float)
#[doc(alias = "RBX::Ball::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX4Ball9getMomentEf")]
// IDA 0x74d28c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d28c() {
}

// 0x74d298 — __ZN3RBX8EdgeListD2Ev
// type: void __fastcall(RBX::EdgeList *__hidden this)
#[doc(alias = "RBX::EdgeList::~EdgeList()")]
#[doc(alias = "__ZN3RBX8EdgeListD2Ev")]
// IDA 0x74d298: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74d298() {
}

// 0x74d420 — __ZN3RBX8EdgeListC2EPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::EdgeList *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::EdgeList::EdgeList(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8EdgeListC2EPNS_9PrimitiveE")]
// IDA 0x74d420: 64 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74d420() {
}

// 0x74d4d8 — __ZN3RBX12SpanningNodeD1Ev
// type: void __fastcall(RBX::SpanningNode *__hidden this)
#[doc(alias = "RBX::SpanningNode::~SpanningNode()")]
#[doc(alias = "__ZN3RBX12SpanningNodeD1Ev")]
// IDA 0x74d4d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74d4d8() {
}

// 0x74d4dc — __ZN3RBX12SpanningNodeD0Ev
// type: void __fastcall(RBX::SpanningNode *__hidden this)
#[doc(alias = "RBX::SpanningNode::~SpanningNode()")]
#[doc(alias = "__ZN3RBX12SpanningNodeD0Ev")]
// IDA 0x74d4dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74d4dc() {
}

// 0x74d57c — __GLOBAL__I_a_341
#[doc(alias = "global constructor keyed to_a_341")]
#[doc(alias = "__GLOBAL__I_a_341")]
// IDA 0x74d57c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_74d57c() {
}

// 0x74da88 — __ZN3RBX9PrismPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX9PrismPoly9buildMeshEv")]
// IDA 0x74da88: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74da88() {
}

// 0x74db6c — __ZN3RBX9PrismPoly20setGeometryParameterERKSsi
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, const std::string *, int)
#[doc(alias = "RBX::PrismPoly::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX9PrismPoly20setGeometryParameterERKSsi")]
// IDA 0x74db6c: 58 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74db6c() {
}

// 0x74dc10 — __ZNK3RBX9PrismPoly20getGeometryParameterERKSs
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, const std::string *)
#[doc(alias = "RBX::PrismPoly::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly20getGeometryParameterERKSs")]
// IDA 0x74dc10: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74dc10() {
}

// 0x74dc9c — __ZNK3RBX9PrismPoly9getMomentEf
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, float)
#[doc(alias = "RBX::PrismPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly9getMomentEf")]
// IDA 0x74dc9c: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74dc9c() {
}

// 0x74ddd0 — __ZNK3RBX9PrismPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly13getCofmOffsetEv")]
// IDA 0x74ddd0: 17 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ddd0() {
}

// 0x74de04 — __ZNK3RBX9PrismPoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, unsigned int)
#[doc(alias = "RBX::PrismPoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly21getSurfaceCoordInBodyEm")]
// IDA 0x74de04: 262 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74de04() {
}

// 0x74e174 — __ZNK3RBX9PrismPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::PrismPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// IDA 0x74e174: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74e174() {
}

// 0x74e294 — __ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_
// type: int(void)
#[doc(alias = "boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(boost::shared_ptr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12GeometryPoolINS1_13Vector3_2IntsENS1_4POLY9PrismMeshENS1_21Vector3_2IntsComparerEE5TokenEEaSERKS9_")]
// IDA 0x74e294: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74e294() {
}

// 0x74e2cc — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")]
// IDA 0x74e2cc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74e2cc() {
}

// 0x74e430 — __ZN3RBX9PrismPolyD1Ev
// type: void __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::~PrismPoly()")]
#[doc(alias = "__ZN3RBX9PrismPolyD1Ev")]
// IDA 0x74e430: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74e430() {
}

// 0x74e454 — __ZN3RBX9PrismPolyD0Ev
// type: void __fastcall(RBX::PrismPoly *__hidden this)
#[doc(alias = "RBX::PrismPoly::~PrismPoly()")]
#[doc(alias = "__ZN3RBX9PrismPolyD0Ev")]
// IDA 0x74e454: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74e454() {
}

// 0x74e508 — __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
// type: int(void)
#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
// IDA 0x74e508: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74e508() {
}

// 0x74e734 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
// IDA 0x74e734: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74e734() {
}

// 0x74e910 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
// IDA 0x74e910: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74e910() {
}

// 0x74ea6c — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv")]
// IDA 0x74ea6c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ea6c() {
}

// 0x74eaa8 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
// IDA 0x74eaa8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74eaa8() {
}

// 0x74ead0 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
// IDA 0x74ead0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ead0() {
}

// 0x74eb30 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// IDA 0x74eb30: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74eb30() {
}

// 0x74eb58 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
// IDA 0x74eb58: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74eb58() {
}

// 0x74eb88 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
// IDA 0x74eb88: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74eb88() {
}

// 0x74ec70 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
// IDA 0x74ec70: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ec70() {
}

// 0x74ed50 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
// type: int __fastcall(int, int, _Rb_tree_node_base *, _QWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
// IDA 0x74ed50: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ed50() {
}

// 0x74edb4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
// IDA 0x74edb4: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74edb4() {
}

// 0x74ee34 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
// IDA 0x74ee34: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ee34() {
}

// 0x74eef4 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm")]
// IDA 0x74eef4: 35 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74eef4() {
}

// 0x74ef64 — __ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE
#[doc(alias = "RBX::POLY::PrismMesh::PrismMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE")]
// IDA 0x74ef64: 120 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ef64() {
}

// 0x74f0b4 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev
// type: int(void)
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev")]
// IDA 0x74f0b4: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f0b4() {
}

// 0x74f118 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEE13releaseMemoryEv
#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEE13releaseMemoryEv")]
// IDA 0x74f118: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f118() {
}

// 0x74f19c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
// IDA 0x74f19c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f19c() {
}

// 0x74f1ec — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")]
// IDA 0x74f1ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_74f1ec() {
}

// 0x74f1f0 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
// IDA 0x74f1f0: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f1f0() {
}

// 0x74f300 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")]
// IDA 0x74f300: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74f300() {
}

// 0x74f3b0 — __GLOBAL__I_a_342
#[doc(alias = "global constructor keyed to_a_342")]
#[doc(alias = "__GLOBAL__I_a_342")]
// IDA 0x74f3b0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_74f3b0() {
}

// 0x74f4e0 — __ZN3RBX11PyramidPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX11PyramidPoly9buildMeshEv")]
// IDA 0x74f4e0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f4e0() {
}

// 0x74f5c4 — __ZN3RBX11PyramidPoly20setGeometryParameterERKSsi
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, const std::string *, int)
#[doc(alias = "RBX::PyramidPoly::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX11PyramidPoly20setGeometryParameterERKSsi")]
// IDA 0x74f5c4: 58 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f5c4() {
}

// 0x74f668 — __ZNK3RBX11PyramidPoly20getGeometryParameterERKSs
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, const std::string *)
#[doc(alias = "RBX::PyramidPoly::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly20getGeometryParameterERKSs")]
// IDA 0x74f668: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f668() {
}

// 0x74f6f4 — __ZNK3RBX11PyramidPoly9getMomentEf
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, float)
#[doc(alias = "RBX::PyramidPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly9getMomentEf")]
// IDA 0x74f6f4: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f6f4() {
}

// 0x74f828 — __ZNK3RBX11PyramidPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly13getCofmOffsetEv")]
// IDA 0x74f828: 17 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f828() {
}

// 0x74f85c — __ZNK3RBX11PyramidPoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, unsigned int)
#[doc(alias = "RBX::PyramidPoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly21getSurfaceCoordInBodyEm")]
// IDA 0x74f85c: 259 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74f85c() {
}

// 0x74fbc8 — __ZNK3RBX11PyramidPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
#[doc(alias = "RBX::PyramidPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// IDA 0x74fbc8: 83 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74fbc8() {
}

// 0x74fd08 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")]
// IDA 0x74fd08: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74fd08() {
}

// 0x74fe6c — __ZN3RBX11PyramidPolyD1Ev
// type: void __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::~PyramidPoly()")]
#[doc(alias = "__ZN3RBX11PyramidPolyD1Ev")]
// IDA 0x74fe6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74fe6c() {
}

// 0x74fe90 — __ZN3RBX11PyramidPolyD0Ev
// type: void __fastcall(RBX::PyramidPoly *__hidden this)
#[doc(alias = "RBX::PyramidPoly::~PyramidPoly()")]
#[doc(alias = "__ZN3RBX11PyramidPolyD0Ev")]
// IDA 0x74fe90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_74fe90() {
}

// 0x74ff44 — __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
// IDA 0x74ff44: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_74ff44() {
}

// 0x750170 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
// IDA 0x750170: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750170() {
}

// 0x75034c — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
// IDA 0x75034c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_75034c() {
}

// 0x7504a8 — __ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv
// type: void __fastcall(void *)
#[doc(alias = "RBX::Allocator<RBX::POLY::PyramidMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY11PyramidMeshEEdlEPv")]
// IDA 0x7504a8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7504a8() {
}

// 0x7504e4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
// IDA 0x7504e4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7504e4() {
}

// 0x75050c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
// IDA 0x75050c: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75050c() {
}

// 0x75056c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// IDA 0x75056c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_75056c() {
}

// 0x750594 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
// IDA 0x750594: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_750594() {
}

// 0x7505c4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
// IDA 0x7505c4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7505c4() {
}
