//! core bg6 — 100 core stubs EA-sorted asc distinct not yet in rbx_core nor global flawed set.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/rbx_core/src nor /tmp/global_eas.txt (flawed) — next 100 uncovered after 0xb44e58 (bg5 max) gap-filled 0x74c498..0x750170 (lowest uncovered not in rbx_core).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "RBX::Primitive::getFaceInWorld(RBX::NormalId)")]
#[doc(alias = "__ZN3RBX9Primitive14getFaceInWorldENS_8NormalIdE")]
// 0x74c498 — __ZN3RBX9Primitive14getFaceInWorldENS_8NormalIdE
// type: int __fastcall(int, int)
pub fn stub_74c498() -> ! {
    todo!("0x74c498 __ZN3RBX9Primitive14getFaceInWorldENS_8NormalIdE")
}

#[doc(alias = "RBX::Primitive::setPV(RBX::PV const&)")]
#[doc(alias = "__ZN3RBX9Primitive5setPVERKNS_2PVE")]
// 0x74c638 — __ZN3RBX9Primitive5setPVERKNS_2PVE
// type: int __fastcall(int, float *)
pub fn stub_74c638() -> ! {
    todo!("0x74c638 __ZN3RBX9Primitive5setPVERKNS_2PVE")
}

#[doc(alias = "RBX::Primitive::zeroVelocity(void)")]
#[doc(alias = "__ZN3RBX9Primitive12zeroVelocityEv")]
// 0x74c7b0 — __ZN3RBX9Primitive12zeroVelocityEv
// type: RBX::Velocity *__fastcall(RBX::Primitive *this, RBX::Primitive *)
pub fn stub_74c7b0() -> ! {
    todo!("0x74c7b0 __ZN3RBX9Primitive12zeroVelocityEv")
}

#[doc(alias = "RBX::Primitive::setVelocity(RBX::Velocity const&)")]
#[doc(alias = "__ZN3RBX9Primitive11setVelocityERKNS_8VelocityE")]
// 0x74c7e8 — __ZN3RBX9Primitive11setVelocityERKNS_8VelocityE
// type: int __fastcall(RBX::Body **this, const RBX::Velocity *)
pub fn stub_74c7e8() -> ! {
    todo!("0x74c7e8 __ZN3RBX9Primitive11setVelocityERKNS_8VelocityE")
}

#[doc(alias = "RBX::Primitive::getCoordinateFrame(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive18getCoordinateFrameEv")]
// 0x74c840 — __ZNK3RBX9Primitive18getCoordinateFrameEv
// type: int __fastcall(RBX::Body **this)
pub fn stub_74c840() -> ! {
    todo!("0x74c840 __ZNK3RBX9Primitive18getCoordinateFrameEv")
}

#[doc(alias = "RBX::Primitive::getPV(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive5getPVEv")]
// 0x74c848 — __ZNK3RBX9Primitive5getPVEv
// type: int __fastcall(RBX::Body **this)
pub fn stub_74c848() -> ! {
    todo!("0x74c848 __ZNK3RBX9Primitive5getPVEv")
}

#[doc(alias = "RBX::Primitive::getGridCorner(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive13getGridCornerEv")]
// 0x74c850 — __ZNK3RBX9Primitive13getGridCornerEv
// type: int __fastcall(RBX::Primitive *this, int)
pub fn stub_74c850() -> ! {
    todo!("0x74c850 __ZNK3RBX9Primitive13getGridCornerEv")
}

#[doc(alias = "RBX::Primitive::setSurfaceData(RBX::NormalId,RBX::SurfaceData const&)")]
#[doc(alias = "__ZN3RBX9Primitive14setSurfaceDataENS_8NormalIdERKNS_11SurfaceDataE")]
// 0x74c980 — __ZN3RBX9Primitive14setSurfaceDataENS_8NormalIdERKNS_11SurfaceDataE
// type: _DWORD *__fastcall(int, int, RBX::SurfaceData *this)
pub fn stub_74c980() -> ! {
    todo!("0x74c980 __ZN3RBX9Primitive14setSurfaceDataENS_8NormalIdERKNS_11SurfaceDataE")
}

#[doc(alias = "RBX::Primitive::setSurfaceType(RBX::NormalId,RBX::SurfaceType)")]
#[doc(alias = "__ZN3RBX9Primitive14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE")]
// 0x74ca64 — __ZN3RBX9Primitive14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE
// type: _DWORD *__fastcall(int, int, int)
pub fn stub_74ca64() -> ! {
    todo!("0x74ca64 __ZN3RBX9Primitive14setSurfaceTypeENS_8NormalIdENS_11SurfaceTypeE")
}

#[doc(alias = "RBX::Primitive::nextSpanningEdgeFromJoint(RBX::Joint *)")]
#[doc(alias = "__ZN3RBX9Primitive25nextSpanningEdgeFromJointEPNS_5JointE")]
// 0x74ca78 — __ZN3RBX9Primitive25nextSpanningEdgeFromJointEPNS_5JointE
// type: char *__fastcall(RBX::Primitive *this, RBX::Joint *)
pub fn stub_74ca78() -> ! {
    todo!("0x74ca78 __ZN3RBX9Primitive25nextSpanningEdgeFromJointEPNS_5JointE")
}

#[doc(alias = "RBX::Primitive::getFirstSpanningEdge(void)")]
#[doc(alias = "__ZN3RBX9Primitive20getFirstSpanningEdgeEv")]
// 0x74cab0 — __ZN3RBX9Primitive20getFirstSpanningEdgeEv
// type: char *__fastcall(RBX::Primitive *this)
pub fn stub_74cab0() -> ! {
    todo!("0x74cab0 __ZN3RBX9Primitive20getFirstSpanningEdgeEv")
}

#[doc(alias = "non-virtual thunk toRBX::Primitive::getFirstSpanningEdge(void)")]
#[doc(alias = "__ZThn8_N3RBX9Primitive20getFirstSpanningEdgeEv")]
// 0x74cac4 — __ZThn8_N3RBX9Primitive20getFirstSpanningEdgeEv
// type: char *__fastcall(RBX::Primitive *this)
pub fn stub_74cac4() -> ! {
    todo!("0x74cac4 __ZThn8_N3RBX9Primitive20getFirstSpanningEdgeEv")
}

#[doc(alias = "RBX::Primitive::getNextSpanningEdge(RBX::SpanningEdge *)")]
#[doc(alias = "__ZN3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE")]
// 0x74cae0 — __ZN3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE
// type: char *__fastcall(RBX::Primitive *this, RBX::SpanningEdge *)
pub fn stub_74cae0() -> ! {
    todo!("0x74cae0 __ZN3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "non-virtual thunk toRBX::Primitive::getNextSpanningEdge(RBX::SpanningEdge *)")]
#[doc(alias = "__ZThn8_N3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE")]
// 0x74cb08 — __ZThn8_N3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE
// type: char *__fastcall(RBX::Primitive *this, RBX::SpanningEdge *)
pub fn stub_74cb08() -> ! {
    todo!("0x74cb08 __ZThn8_N3RBX9Primitive19getNextSpanningEdgeEPNS_12SpanningEdgeE")
}

#[doc(alias = "RBX::Primitive::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive20isGeometryOrthogonalEv")]
// 0x74cb10 — __ZNK3RBX9Primitive20isGeometryOrthogonalEv
// type: int __fastcall(RBX::Primitive *this)
pub fn stub_74cb10() -> ! {
    todo!("0x74cb10 __ZNK3RBX9Primitive20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::Primitive::getSortSize(void)")]
#[doc(alias = "__ZN3RBX9Primitive11getSortSizeEv")]
// 0x74cb20 — __ZN3RBX9Primitive11getSortSizeEv
// type: int __fastcall(RBX::Primitive *this)
pub fn stub_74cb20() -> ! {
    todo!("0x74cb20 __ZN3RBX9Primitive11getSortSizeEv")
}

#[doc(alias = "RBX::Primitive::calculateSortSize(void)")]
#[doc(alias = "__ZN3RBX9Primitive17calculateSortSizeEv")]
// 0x74cb38 — __ZN3RBX9Primitive17calculateSortSizeEv
// type: int __fastcall(RBX::Primitive *this, int, int)
pub fn stub_74cb38() -> ! {
    todo!("0x74cb38 __ZN3RBX9Primitive17calculateSortSizeEv")
}

#[doc(alias = "RBX::Primitive::setSpecificGravity(float)")]
#[doc(alias = "__ZN3RBX9Primitive18setSpecificGravityEf")]
// 0x74cc78 — __ZN3RBX9Primitive18setSpecificGravityEf
// type: RBX::World *__fastcall(RBX::Primitive *this, float)
pub fn stub_74cc78() -> ! {
    todo!("0x74cc78 __ZN3RBX9Primitive18setSpecificGravityEf")
}

#[doc(alias = "RBX::Primitive::computeIsGrounded(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive17computeIsGroundedEv")]
// 0x74cc90 — __ZNK3RBX9Primitive17computeIsGroundedEv
// type: int __fastcall(RBX::Primitive *this)
pub fn stub_74cc90() -> ! {
    todo!("0x74cc90 __ZNK3RBX9Primitive17computeIsGroundedEv")
}

#[doc(alias = "RBX::Allocator<RBX::Body>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4BodyEEnwEm")]
// 0x74ccb8 — __ZN3RBX9AllocatorINS_4BodyEEnwEm
// type: int __fastcall(unsigned int)
pub fn stub_74ccb8() -> ! {
    todo!("0x74ccb8 __ZN3RBX9AllocatorINS_4BodyEEnwEm")
}

#[doc(alias = "RBX::BasicSpatialHashPrimitive::~BasicSpatialHashPrimitive()")]
#[doc(alias = "__ZN3RBX25BasicSpatialHashPrimitiveD2Ev")]
// 0x74cd28 — __ZN3RBX25BasicSpatialHashPrimitiveD2Ev
// type: void __fastcall(RBX::BasicSpatialHashPrimitive *__hidden this)
pub fn stub_74cd28() -> ! {
    todo!("0x74cd28 __ZN3RBX25BasicSpatialHashPrimitiveD2Ev")
}

#[doc(alias = "RBX::EdgeList::getEdge(int)const")]
#[doc(alias = "__ZNK3RBX8EdgeList7getEdgeEi")]
// 0x74cd8c — __ZNK3RBX8EdgeList7getEdgeEi
// type: _DWORD __fastcall(RBX::EdgeList *__hidden this, int)
pub fn stub_74cd8c() -> ! {
    todo!("0x74cd8c __ZNK3RBX8EdgeList7getEdgeEi")
}

#[doc(alias = "RBX::SurfaceData::isEmpty(void)const")]
#[doc(alias = "__ZNK3RBX11SurfaceData7isEmptyEv")]
// 0x74ce50 — __ZNK3RBX11SurfaceData7isEmptyEv
// type: _DWORD __fastcall(RBX::SurfaceData *__hidden this)
pub fn stub_74ce50() -> ! {
    todo!("0x74ce50 __ZNK3RBX11SurfaceData7isEmptyEv")
}

#[doc(alias = "RBX::Joint::isSpanningTreeJoint(RBX::Edge const*)")]
#[doc(alias = "__ZN3RBX5Joint19isSpanningTreeJointEPKNS_4EdgeE")]
// 0x74ce9c — __ZN3RBX5Joint19isSpanningTreeJointEPKNS_4EdgeE
// type: _DWORD __fastcall(RBX::Joint *__hidden this, const RBX::Edge *)
pub fn stub_74ce9c() -> ! {
    todo!("0x74ce9c __ZN3RBX5Joint19isSpanningTreeJointEPKNS_4EdgeE")
}

#[doc(alias = "RBX::Primitive::getRadius(void)const")]
#[doc(alias = "__ZNK3RBX9Primitive9getRadiusEv")]
// 0x74cf08 — __ZNK3RBX9Primitive9getRadiusEv
// type: _DWORD __fastcall(RBX::Primitive *__hidden this)
pub fn stub_74cf08() -> ! {
    todo!("0x74cf08 __ZNK3RBX9Primitive9getRadiusEv")
}

#[doc(alias = "RBX::Body::getPV_Spin_Lock(void)")]
#[doc(alias = "__ZN3RBX4Body15getPV_Spin_LockEv")]
// 0x74cf9c — __ZN3RBX4Body15getPV_Spin_LockEv
// type: _DWORD __fastcall(RBX::Body *__hidden this)
pub fn stub_74cf9c() -> ! {
    todo!("0x74cf9c __ZN3RBX4Body15getPV_Spin_LockEv")
}

#[doc(alias = "RBX::MegaClusterPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly15getGeometryTypeEv")]
// 0x74d060 — __ZNK3RBX15MegaClusterPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
pub fn stub_74d060() -> ! {
    todo!("0x74d060 __ZNK3RBX15MegaClusterPoly15getGeometryTypeEv")
}

#[doc(alias = "RBX::Poly::getCollideType(void)const")]
#[doc(alias = "__ZNK3RBX4Poly14getCollideTypeEv")]
// 0x74d064 — __ZNK3RBX4Poly14getCollideTypeEv
// type: _DWORD __fastcall(RBX::Poly *__hidden this)
pub fn stub_74d064() -> ! {
    todo!("0x74d064 __ZNK3RBX4Poly14getCollideTypeEv")
}

#[doc(alias = "RBX::MegaClusterPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly20isGeometryOrthogonalEv")]
// 0x74d068 — __ZNK3RBX15MegaClusterPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
pub fn stub_74d068() -> ! {
    todo!("0x74d068 __ZNK3RBX15MegaClusterPoly20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::MegaClusterPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly13getCofmOffsetEv")]
// 0x74d084 — __ZNK3RBX15MegaClusterPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
pub fn stub_74d084() -> ! {
    todo!("0x74d084 __ZNK3RBX15MegaClusterPoly13getCofmOffsetEv")
}

#[doc(alias = "RBX::MegaClusterPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly9getMomentEf")]
// 0x74d09c — __ZNK3RBX15MegaClusterPoly9getMomentEf
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this, float)
pub fn stub_74d09c() -> ! {
    todo!("0x74d09c __ZNK3RBX15MegaClusterPoly9getMomentEf")
}

#[doc(alias = "RBX::Geometry::getVolume(void)const")]
#[doc(alias = "__ZNK3RBX8Geometry9getVolumeEv")]
// 0x74d0b0 — __ZNK3RBX8Geometry9getVolumeEv
// type: _DWORD __fastcall(RBX::Geometry *__hidden this)
pub fn stub_74d0b0() -> ! {
    todo!("0x74d0b0 __ZNK3RBX8Geometry9getVolumeEv")
}

#[doc(alias = "RBX::MegaClusterPoly::getSize(void)const")]
#[doc(alias = "__ZNK3RBX15MegaClusterPoly7getSizeEv")]
// 0x74d0cc — __ZNK3RBX15MegaClusterPoly7getSizeEv
// type: _DWORD __fastcall(RBX::MegaClusterPoly *__hidden this)
pub fn stub_74d0cc() -> ! {
    todo!("0x74d0cc __ZNK3RBX15MegaClusterPoly7getSizeEv")
}

#[doc(alias = "RBX::Geometry::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX8Geometry13getCofmOffsetEv")]
// 0x74d0d0 — __ZNK3RBX8Geometry13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::Geometry *__hidden this)
pub fn stub_74d0d0() -> ! {
    todo!("0x74d0d0 __ZNK3RBX8Geometry13getCofmOffsetEv")
}

#[doc(alias = "RBX::CornerWedgePoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly15getGeometryTypeEv")]
// 0x74d12c — __ZNK3RBX15CornerWedgePoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this)
pub fn stub_74d12c() -> ! {
    todo!("0x74d12c __ZNK3RBX15CornerWedgePoly15getGeometryTypeEv")
}

#[doc(alias = "RBX::CornerWedgePoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX15CornerWedgePoly20isGeometryOrthogonalEv")]
// 0x74d130 — __ZNK3RBX15CornerWedgePoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::CornerWedgePoly *__hidden this)
pub fn stub_74d130() -> ! {
    todo!("0x74d130 __ZNK3RBX15CornerWedgePoly20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::RightAngleRampPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly15getGeometryTypeEv")]
// 0x74d134 — __ZNK3RBX18RightAngleRampPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
pub fn stub_74d134() -> ! {
    todo!("0x74d134 __ZNK3RBX18RightAngleRampPoly15getGeometryTypeEv")
}

#[doc(alias = "RBX::RightAngleRampPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX18RightAngleRampPoly20isGeometryOrthogonalEv")]
// 0x74d138 — __ZNK3RBX18RightAngleRampPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::RightAngleRampPoly *__hidden this)
pub fn stub_74d138() -> ! {
    todo!("0x74d138 __ZNK3RBX18RightAngleRampPoly20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::ParallelRampPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX16ParallelRampPoly15getGeometryTypeEv")]
// 0x74d13c — __ZNK3RBX16ParallelRampPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::ParallelRampPoly *__hidden this)
pub fn stub_74d13c() -> ! {
    todo!("0x74d13c __ZNK3RBX16ParallelRampPoly15getGeometryTypeEv")
}

#[doc(alias = "RBX::ParallelRampPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX16ParallelRampPoly20isGeometryOrthogonalEv")]
// 0x74d140 — __ZNK3RBX16ParallelRampPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::ParallelRampPoly *__hidden this)
pub fn stub_74d140() -> ! {
    todo!("0x74d140 __ZNK3RBX16ParallelRampPoly20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::PyramidPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly15getGeometryTypeEv")]
// 0x74d144 — __ZNK3RBX11PyramidPoly15getGeometryTypeEv
// type: int __fastcall(RBX::PyramidPoly *this)
pub fn stub_74d144() -> ! {
    todo!("0x74d144 __ZNK3RBX11PyramidPoly15getGeometryTypeEv")
}

#[doc(alias = "RBX::PyramidPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly20isGeometryOrthogonalEv")]
// 0x74d148 — __ZNK3RBX11PyramidPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
pub fn stub_74d148() -> ! {
    todo!("0x74d148 __ZNK3RBX11PyramidPoly20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::PrismPoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly15getGeometryTypeEv")]
// 0x74d14c — __ZNK3RBX9PrismPoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
pub fn stub_74d14c() -> ! {
    todo!("0x74d14c __ZNK3RBX9PrismPoly15getGeometryTypeEv")
}

#[doc(alias = "RBX::PrismPoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly20isGeometryOrthogonalEv")]
// 0x74d150 — __ZNK3RBX9PrismPoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
pub fn stub_74d150() -> ! {
    todo!("0x74d150 __ZNK3RBX9PrismPoly20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::WedgePoly::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly15getGeometryTypeEv")]
// 0x74d154 — __ZNK3RBX9WedgePoly15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this)
pub fn stub_74d154() -> ! {
    todo!("0x74d154 __ZNK3RBX9WedgePoly15getGeometryTypeEv")
}

#[doc(alias = "RBX::WedgePoly::isGeometryOrthogonal(void)const")]
#[doc(alias = "__ZNK3RBX9WedgePoly20isGeometryOrthogonalEv")]
// 0x74d158 — __ZNK3RBX9WedgePoly20isGeometryOrthogonalEv
// type: _DWORD __fastcall(RBX::WedgePoly *__hidden this)
pub fn stub_74d158() -> ! {
    todo!("0x74d158 __ZNK3RBX9WedgePoly20isGeometryOrthogonalEv")
}

#[doc(alias = "RBX::Ball::~Ball()")]
#[doc(alias = "__ZN3RBX4BallD1Ev")]
// 0x74d15c — __ZN3RBX4BallD1Ev
// type: void __fastcall(RBX::Ball *__hidden this)
pub fn stub_74d15c() -> ! {
    todo!("0x74d15c __ZN3RBX4BallD1Ev")
}

#[doc(alias = "RBX::Ball::~Ball()")]
#[doc(alias = "__ZN3RBX4BallD0Ev")]
// 0x74d160 — __ZN3RBX4BallD0Ev
// type: void __fastcall(RBX::Ball *__hidden this)
pub fn stub_74d160() -> ! {
    todo!("0x74d160 __ZN3RBX4BallD0Ev")
}

#[doc(alias = "RBX::Ball::getGeometryType(void)const")]
#[doc(alias = "__ZNK3RBX4Ball15getGeometryTypeEv")]
// 0x74d164 — __ZNK3RBX4Ball15getGeometryTypeEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
pub fn stub_74d164() -> ! {
    todo!("0x74d164 __ZNK3RBX4Ball15getGeometryTypeEv")
}

#[doc(alias = "RBX::Ball::getCollideType(void)const")]
#[doc(alias = "__ZNK3RBX4Ball14getCollideTypeEv")]
// 0x74d168 — __ZNK3RBX4Ball14getCollideTypeEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
pub fn stub_74d168() -> ! {
    todo!("0x74d168 __ZNK3RBX4Ball14getCollideTypeEv")
}

#[doc(alias = "RBX::Ball::getRadius(void)const")]
#[doc(alias = "__ZNK3RBX4Ball9getRadiusEv")]
// 0x74d16c — __ZNK3RBX4Ball9getRadiusEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
pub fn stub_74d16c() -> ! {
    todo!("0x74d16c __ZNK3RBX4Ball9getRadiusEv")
}

#[doc(alias = "RBX::Ball::getNumSurfaces(void)const")]
#[doc(alias = "__ZNK3RBX4Ball14getNumSurfacesEv")]
// 0x74d170 — __ZNK3RBX4Ball14getNumSurfacesEv
// type: _DWORD __fastcall(RBX::Ball *__hidden this)
pub fn stub_74d170() -> ! {
    todo!("0x74d170 __ZNK3RBX4Ball14getNumSurfacesEv")
}

#[doc(alias = "RBX::Ball::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX4Ball9getMomentEf")]
// 0x74d28c — __ZNK3RBX4Ball9getMomentEf
// type: _DWORD __fastcall(RBX::Ball *__hidden this, float)
pub fn stub_74d28c() -> ! {
    todo!("0x74d28c __ZNK3RBX4Ball9getMomentEf")
}

#[doc(alias = "RBX::EdgeList::~EdgeList()")]
#[doc(alias = "__ZN3RBX8EdgeListD2Ev")]
// 0x74d298 — __ZN3RBX8EdgeListD2Ev
// type: void __fastcall(RBX::EdgeList *__hidden this)
pub fn stub_74d298() -> ! {
    todo!("0x74d298 __ZN3RBX8EdgeListD2Ev")
}

#[doc(alias = "RBX::EdgeList::EdgeList(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8EdgeListC2EPNS_9PrimitiveE")]
// 0x74d420 — __ZN3RBX8EdgeListC2EPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::EdgeList *__hidden this, RBX::Primitive *)
pub fn stub_74d420() -> ! {
    todo!("0x74d420 __ZN3RBX8EdgeListC2EPNS_9PrimitiveE")
}

#[doc(alias = "RBX::SpanningNode::~SpanningNode()")]
#[doc(alias = "__ZN3RBX12SpanningNodeD1Ev")]
// 0x74d4d8 — __ZN3RBX12SpanningNodeD1Ev
// type: void __fastcall(RBX::SpanningNode *__hidden this)
pub fn stub_74d4d8() -> ! {
    todo!("0x74d4d8 __ZN3RBX12SpanningNodeD1Ev")
}

#[doc(alias = "RBX::SpanningNode::~SpanningNode()")]
#[doc(alias = "__ZN3RBX12SpanningNodeD0Ev")]
// 0x74d4dc — __ZN3RBX12SpanningNodeD0Ev
// type: void __fastcall(RBX::SpanningNode *__hidden this)
pub fn stub_74d4dc() -> ! {
    todo!("0x74d4dc __ZN3RBX12SpanningNodeD0Ev")
}

#[doc(alias = "RBX::PrismPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX9PrismPoly9buildMeshEv")]
// 0x74da88 — __ZN3RBX9PrismPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
pub fn stub_74da88() -> ! {
    todo!("0x74da88 __ZN3RBX9PrismPoly9buildMeshEv")
}

#[doc(alias = "RBX::PrismPoly::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX9PrismPoly20setGeometryParameterERKSsi")]
// 0x74db6c — __ZN3RBX9PrismPoly20setGeometryParameterERKSsi
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, const std::string *, int)
pub fn stub_74db6c() -> ! {
    todo!("0x74db6c __ZN3RBX9PrismPoly20setGeometryParameterERKSsi")
}

#[doc(alias = "RBX::PrismPoly::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly20getGeometryParameterERKSs")]
// 0x74dc10 — __ZNK3RBX9PrismPoly20getGeometryParameterERKSs
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, const std::string *)
pub fn stub_74dc10() -> ! {
    todo!("0x74dc10 __ZNK3RBX9PrismPoly20getGeometryParameterERKSs")
}

#[doc(alias = "RBX::PrismPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly9getMomentEf")]
// 0x74dc9c — __ZNK3RBX9PrismPoly9getMomentEf
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, float)
pub fn stub_74dc9c() -> ! {
    todo!("0x74dc9c __ZNK3RBX9PrismPoly9getMomentEf")
}

#[doc(alias = "RBX::PrismPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly13getCofmOffsetEv")]
// 0x74ddd0 — __ZNK3RBX9PrismPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this)
pub fn stub_74ddd0() -> ! {
    todo!("0x74ddd0 __ZNK3RBX9PrismPoly13getCofmOffsetEv")
}

#[doc(alias = "RBX::PrismPoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly21getSurfaceCoordInBodyEm")]
// 0x74de04 — __ZNK3RBX9PrismPoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::PrismPoly *__hidden this, unsigned int)
pub fn stub_74de04() -> ! {
    todo!("0x74de04 __ZNK3RBX9PrismPoly21getSurfaceCoordInBodyEm")
}

#[doc(alias = "RBX::PrismPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX9PrismPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// 0x74e174 — __ZNK3RBX9PrismPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
pub fn stub_74e174() -> ! {
    todo!("0x74e174 __ZNK3RBX9PrismPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")]
// 0x74e2cc — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
pub fn stub_74e2cc() -> ! {
    todo!("0x74e2cc __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")
}

#[doc(alias = "RBX::PrismPoly::~PrismPoly()")]
#[doc(alias = "__ZN3RBX9PrismPolyD1Ev")]
// 0x74e430 — __ZN3RBX9PrismPolyD1Ev
// type: void __fastcall(RBX::PrismPoly *__hidden this)
pub fn stub_74e430() -> ! {
    todo!("0x74e430 __ZN3RBX9PrismPolyD1Ev")
}

#[doc(alias = "RBX::PrismPoly::~PrismPoly()")]
#[doc(alias = "__ZN3RBX9PrismPolyD0Ev")]
// 0x74e454 — __ZN3RBX9PrismPolyD0Ev
// type: void __fastcall(RBX::PrismPoly *__hidden this)
pub fn stub_74e454() -> ! {
    todo!("0x74e454 __ZN3RBX9PrismPolyD0Ev")
}

#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
// 0x74e508 — __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
// type: int(void)
pub fn stub_74e508() -> ! {
    todo!("0x74e508 __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
// 0x74e734 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
pub fn stub_74e734() -> ! {
    todo!("0x74e734 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::~ValueCount()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")]
// 0x74e910 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev
pub fn stub_74e910() -> ! {
    todo!("0x74e910 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountD2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator delete(void *)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv")]
// 0x74ea6c — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv
// type: void __fastcall(_DWORD *)
pub fn stub_74ea6c() -> ! {
    todo!("0x74ea6c __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEdlEPv")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")]
// 0x74eaa8 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_
// type: int(void)
pub fn stub_74eaa8() -> ! {
    todo!("0x74eaa8 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")]
// 0x74ead0 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_74ead0() -> ! {
    todo!("0x74ead0 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE5eraseESt17_Rb_tree_iteratorISB_ESH_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// 0x74eb30 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: int(void)
pub fn stub_74eb30() -> ! {
    todo!("0x74eb30 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::lower_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")]
// 0x74eb58 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_
// type: int(void)
pub fn stub_74eb58() -> ! {
    todo!("0x74eb58 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11lower_boundERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::upper_bound(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")]
// 0x74eb88 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_
// type: int(void)
pub fn stub_74eb88() -> ! {
    todo!("0x74eb88 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE11upper_boundERS3_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")]
// 0x74ec70 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// type: int(void)
pub fn stub_74ec70() -> ! {
    todo!("0x74ec70 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")]
// 0x74ed50 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_
// type: int __fastcall(int, int, _Rb_tree_node_base *, _QWORD *)
pub fn stub_74ed50() -> ! {
    todo!("0x74ed50 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKSB_")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::_M_insert_unique(std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")]
// 0x74edb4 — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_
// type: int(void)
pub fn stub_74edb4() -> ! {
    todo!("0x74edb4 __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE16_M_insert_uniqueERKSB_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount::ValueCount(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")]
// 0x74ee34 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_
pub fn stub_74ee34() -> ! {
    todo!("0x74ee34 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10ValueCountC2ERKS1_")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::operator new(unsigned long)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm")]
// 0x74eef4 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm
// type: int(void)
pub fn stub_74eef4() -> ! {
    todo!("0x74eef4 __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEnwEm")
}

#[doc(alias = "RBX::POLY::PrismMesh::PrismMesh(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE")]
// 0x74ef64 — __ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE
pub fn stub_74ef64() -> ! {
    todo!("0x74ef64 __ZN3RBX4POLY9PrismMeshC2ERKNS_13Vector3_2IntsE")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::Allocator(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev")]
// 0x74f0b4 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev
// type: int(void)
pub fn stub_74f0b4() -> ! {
    todo!("0x74f0b4 __ZN3RBX9AllocatorINS_4POLY9PrismMeshEEC2Ev")
}

#[doc(alias = "RBX::Allocator<RBX::POLY::PrismMesh>::releaseMemory(void)")]
#[doc(alias = "__ZN3RBX9AllocatorINS_4POLY9PrismMeshEE13releaseMemoryEv")]
// 0x74f118 — __ZN3RBX9AllocatorINS_4POLY9PrismMeshEE13releaseMemoryEv
pub fn stub_74f118() -> ! {
    todo!("0x74f118 __ZN3RBX9AllocatorINS_4POLY9PrismMeshEE13releaseMemoryEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Vector3_2Ints,std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>,std::_Select1st<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::find(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")]
// 0x74f19c — __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_
// type: int(void)
pub fn stub_74f19c() -> ! {
    todo!("0x74f19c __ZNSt8_Rb_treeIN3RBX13Vector3_2IntsESt4pairIKS1_PNS0_12GeometryPoolIS1_NS0_4POLY9PrismMeshENS0_21Vector3_2IntsComparerEE10ValueCountEESt10_Select1stISB_ES7_SaISB_EE4findERS3_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_init_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")]
// 0x74f1ec — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv
pub fn stub_74f1ec() -> ! {
    todo!("0x74f1ec __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE27safe_static_init_staticDataEv")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::safe_static_do_get_staticData(void)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")]
// 0x74f1f0 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv
pub fn stub_74f1f0() -> ! {
    todo!("0x74f1f0 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE29safe_static_do_get_staticDataEv")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::StaticData::~StaticData()")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")]
// 0x74f300 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev
pub fn stub_74f300() -> ! {
    todo!("0x74f300 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY9PrismMeshENS_21Vector3_2IntsComparerEE10StaticDataD1Ev")
}

#[doc(alias = "RBX::PyramidPoly::buildMesh(void)")]
#[doc(alias = "__ZN3RBX11PyramidPoly9buildMeshEv")]
// 0x74f4e0 — __ZN3RBX11PyramidPoly9buildMeshEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
pub fn stub_74f4e0() -> ! {
    todo!("0x74f4e0 __ZN3RBX11PyramidPoly9buildMeshEv")
}

#[doc(alias = "RBX::PyramidPoly::setGeometryParameter(std::string const&,int)")]
#[doc(alias = "__ZN3RBX11PyramidPoly20setGeometryParameterERKSsi")]
// 0x74f5c4 — __ZN3RBX11PyramidPoly20setGeometryParameterERKSsi
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, const std::string *, int)
pub fn stub_74f5c4() -> ! {
    todo!("0x74f5c4 __ZN3RBX11PyramidPoly20setGeometryParameterERKSsi")
}

#[doc(alias = "RBX::PyramidPoly::getGeometryParameter(std::string const&)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly20getGeometryParameterERKSs")]
// 0x74f668 — __ZNK3RBX11PyramidPoly20getGeometryParameterERKSs
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, const std::string *)
pub fn stub_74f668() -> ! {
    todo!("0x74f668 __ZNK3RBX11PyramidPoly20getGeometryParameterERKSs")
}

#[doc(alias = "RBX::PyramidPoly::getMoment(float)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly9getMomentEf")]
// 0x74f6f4 — __ZNK3RBX11PyramidPoly9getMomentEf
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, float)
pub fn stub_74f6f4() -> ! {
    todo!("0x74f6f4 __ZNK3RBX11PyramidPoly9getMomentEf")
}

#[doc(alias = "RBX::PyramidPoly::getCofmOffset(void)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly13getCofmOffsetEv")]
// 0x74f828 — __ZNK3RBX11PyramidPoly13getCofmOffsetEv
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this)
pub fn stub_74f828() -> ! {
    todo!("0x74f828 __ZNK3RBX11PyramidPoly13getCofmOffsetEv")
}

#[doc(alias = "RBX::PyramidPoly::getSurfaceCoordInBody(unsigned long)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly21getSurfaceCoordInBodyEm")]
// 0x74f85c — __ZNK3RBX11PyramidPoly21getSurfaceCoordInBodyEm
// type: _DWORD __fastcall(RBX::PyramidPoly *__hidden this, unsigned int)
pub fn stub_74f85c() -> ! {
    todo!("0x74f85c __ZNK3RBX11PyramidPoly21getSurfaceCoordInBodyEm")
}

#[doc(alias = "RBX::PyramidPoly::getFaceFromLegacyNormalId(RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX11PyramidPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")]
// 0x74fbc8 — __ZNK3RBX11PyramidPoly25getFaceFromLegacyNormalIdENS_8NormalIdE
pub fn stub_74fbc8() -> ! {
    todo!("0x74fbc8 __ZNK3RBX11PyramidPoly25getFaceFromLegacyNormalIdENS_8NormalIdE")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::getToken(RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")]
// 0x74fd08 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
pub fn stub_74fd08() -> ! {
    todo!("0x74fd08 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE8getTokenERKS1_")
}

#[doc(alias = "RBX::PyramidPoly::~PyramidPoly()")]
#[doc(alias = "__ZN3RBX11PyramidPolyD1Ev")]
// 0x74fe6c — __ZN3RBX11PyramidPolyD1Ev
// type: void __fastcall(RBX::PyramidPoly *__hidden this)
pub fn stub_74fe6c() -> ! {
    todo!("0x74fe6c __ZN3RBX11PyramidPolyD1Ev")
}

#[doc(alias = "RBX::PyramidPoly::~PyramidPoly()")]
#[doc(alias = "__ZN3RBX11PyramidPolyD0Ev")]
// 0x74fe90 — __ZN3RBX11PyramidPolyD0Ev
// type: void __fastcall(RBX::PyramidPoly *__hidden this)
pub fn stub_74fe90() -> ! {
    todo!("0x74fe90 __ZN3RBX11PyramidPolyD0Ev")
}

#[doc(alias = "std::map<RBX::Vector3_2Ints,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *,RBX::Vector3_2IntsComparer,std::allocator<std::pair<RBX::Vector3_2Ints const,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *>>>::operator[](RBX::Vector3_2Ints const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")]
// 0x74ff44 — __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_
pub fn stub_74ff44() -> ! {
    todo!("0x74ff44 __ZNSt3mapIN3RBX13Vector3_2IntsEPNS0_12GeometryPoolIS1_NS0_4POLY11PyramidMeshENS0_21Vector3_2IntsComparerEE10ValueCountES5_SaISt4pairIKS1_S8_EEEixERSA_")
}

#[doc(alias = "RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::returnToken(RBX::Vector3_2Ints const&,RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PyramidMesh,RBX::Vector3_2IntsComparer>::ValueCount *)")]
#[doc(alias = "__ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")]
// 0x750170 — __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, int, int)
pub fn stub_750170() -> ! {
    todo!("0x750170 __ZN3RBX12GeometryPoolINS_13Vector3_2IntsENS_4POLY11PyramidMeshENS_21Vector3_2IntsComparerEE11returnTokenERKS1_PNS5_10ValueCountE")
}
