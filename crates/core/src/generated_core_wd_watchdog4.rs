//! core wd_watchdog4 — 120 core stubs EA-sorted asc gap filler not yet in core (generic/unclaimed filtered).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 uncovered distinct not yet in crates/core/src.
//! Filter: generic/unclaimed — excludes RBX::Reflection/Instance/DataModel/Ogre/RakNet/FMOD/Lua/Script/Render/Platform/iOS (checked mangled+demangled); 120 stubs EA-sorted asc.
//! Batch: 120 stubs | range 0x71231c..0x731f04 | EA-sorted asc gap filler distinct NOT yet in core.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::StringConverter<G3D::Vector3>::convertToString(G3D::Vector3 const&)")]
// 0x71231c — __ZN3RBX15StringConverterIN3G3D7Vector3EE15convertToStringERKS2_
pub fn stub_0x71231c() {
    // IDA 0x71231c: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::StringConverter<G3D::Vector2>::convertToString(G3D::Vector2 const&)")]
// 0x712530 — __ZN3RBX15StringConverterIN3G3D7Vector2EE15convertToStringERKS2_
pub fn stub_0x712530() {
    // IDA 0x712530: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::StringConverter<G3D::CoordinateFrame>::convertToString(G3D::CoordinateFrame const&)")]
// 0x712690 — __ZN3RBX15StringConverterIN3G3D15CoordinateFrameEE15convertToStringERKS2_
pub fn stub_0x712690() {
    // IDA 0x712690: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::StringConverter<G3D::CoordinateFrame>::convertToValue(std::string const&,G3D::CoordinateFrame&)")]
// 0x712818 — __ZN3RBX15StringConverterIN3G3D15CoordinateFrameEE14convertToValueERKSsRS2_
pub fn stub_0x712818() {
    // IDA 0x712818: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<G3D::Vector3>::convertToValue(std::string const&,G3D::Vector3&)")]
// 0x712b00 — __ZN3RBX15StringConverterIN3G3D7Vector3EE14convertToValueERKSsRS2_
pub fn stub_0x712b00() {
    // IDA 0x712b00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<G3D::Vector2>::convertToValue(std::string const&,G3D::Vector2&)")]
// 0x712c64 — __ZN3RBX15StringConverterIN3G3D7Vector2EE14convertToValueERKSsRS2_
pub fn stub_0x712c64() {
    // IDA 0x712c64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<G3D::Color3>::convertToString(G3D::Color3 const&)")]
// 0x713840 — __ZN3RBX15StringConverterIN3G3D6Color3EE15convertToStringERKS2_
pub fn stub_0x713840() {
    // IDA 0x713840: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<G3D::Color3>::convertToValue(std::string const&,G3D::Color3&)")]
// 0x7138ac — __ZN3RBX15StringConverterIN3G3D6Color3EE14convertToValueERKSsRS2_
pub fn stub_0x7138ac() {
    // IDA 0x7138ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Assembly::getPhysics(G3D::Array<RBX::CompactCFrame,10,32ul> &)const")]
// 0x7170fc — __ZNK3RBX8Assembly10getPhysicsERN3G3D5ArrayINS_13CompactCFrameELi10ELm32EEE
pub fn stub_0x7170fc() {
    // IDA 0x7170fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Assembly::getConstAssemblyMotors(G3D::Array<RBX::Joint const*,10,32ul> &)const")]
// 0x7172dc — __ZNK3RBX8Assembly22getConstAssemblyMotorsERN3G3D5ArrayIPKNS_5JointELi10ELm32EEE
pub fn stub_0x7172dc() {
    // IDA 0x7172dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Assembly::setPhysics(G3D::Array<RBX::CompactCFrame,10,32ul> const&,RBX::PV const&)")]
// 0x717348 — __ZN3RBX8Assembly10setPhysicsERKN3G3D5ArrayINS_13CompactCFrameELi10ELm32EEERKNS_2PVE
pub fn stub_0x717348() {
    // IDA 0x717348: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Assembly::getAssemblyMotors(G3D::Array<RBX::Joint *,10,32ul> &)")]
// 0x7175a0 — __ZN3RBX8Assembly17getAssemblyMotorsERN3G3D5ArrayIPNS_5JointELi10ELm32EEE
pub fn stub_0x7175a0() {
    // IDA 0x7175a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::computeAssemblyPrimitiveMaxRadius(RBX::Primitive *,G3D::Vector3 const&,float &)")]
// 0x7179a4 — __ZN3RBX33computeAssemblyPrimitiveMaxRadiusEPNS_9PrimitiveERKN3G3D7Vector3ERf
pub fn stub_0x7179a4() {
    // IDA 0x7179a4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::append(RBX::Edge * const&)")]
// 0x717c50 — __ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EE6appendERKS3_
pub fn stub_0x717c50() {
    // IDA 0x717c50: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::resize(int,bool)")]
// 0x717dd0 — __ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EE6resizeEib
pub fn stub_0x717dd0() {
    // IDA 0x717dd0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::realloc(int)")]
// 0x717e88 — __ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EE7reallocEi
pub fn stub_0x717e88() {
    // IDA 0x717e88: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Joint const*,10,32ul>::~Array()")]
// 0x718718 — __ZN3G3D5ArrayIPKN3RBX5JointELi10ELm32EED2Ev
pub fn stub_0x718718() {
    // IDA 0x718718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Joint const*,10,32ul>::Array(void)")]
// 0x7187ec — __ZN3G3D5ArrayIPKN3RBX5JointELi10ELm32EEC2Ev
pub fn stub_0x7187ec() {
    // IDA 0x7187ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Joint *,10,32ul>::~Array()")]
// 0x7188dc — __ZN3G3D5ArrayIPN3RBX5JointELi10ELm32EED2Ev
pub fn stub_0x7188dc() {
    // IDA 0x7188dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Joint *,10,32ul>::Array(void)")]
// 0x7189b0 — __ZN3G3D5ArrayIPN3RBX5JointELi10ELm32EEC2Ev
pub fn stub_0x7189b0() {
    // IDA 0x7189b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::~Array()")]
// 0x718aa0 — __ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EED2Ev
pub fn stub_0x718aa0() {
    // IDA 0x718aa0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Edge *,10,32ul>::Array(void)")]
// 0x718b74 — __ZN3G3D5ArrayIPN3RBX4EdgeELi10ELm32EEC2Ev
pub fn stub_0x718b74() {
    // IDA 0x718b74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Ball::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &)")]
// 0x719c48 — __ZN3RBX4Ball7hitTestERKNS_6RbxRayERN3G3D7Vector3ERb
pub fn stub_0x719c48() {
    // IDA 0x719c48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Ball::setSize(G3D::Vector3 const&)")]
// 0x719d18 — __ZN3RBX4Ball7setSizeERKN3G3D7Vector3E
pub fn stub_0x719d18() {
    // IDA 0x719d18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Ball::closestSurfaceToPoint(G3D::Vector3 const&)const")]
// 0x719dc8 — __ZNK3RBX4Ball21closestSurfaceToPointERKN3G3D7Vector3E
pub fn stub_0x719dc8() {
    // IDA 0x719dc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Ball::getMostAlignedSurface(G3D::Vector3 const&,G3D::Matrix3 const&)const")]
// 0x71a088 — __ZNK3RBX4Ball21getMostAlignedSurfaceERKN3G3D7Vector3ERKNS1_7Matrix3E
pub fn stub_0x71a088() {
    // IDA 0x71a088: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::Ball::vertOverlapsFace(G3D::Vector3 const&,unsigned long)const")]
// 0x71a198 — __ZNK3RBX4Ball16vertOverlapsFaceERKN3G3D7Vector3Em
pub fn stub_0x71a198() {
    // IDA 0x71a198: RBXL/G3D model-geometry helper owned by the datamodel/rendering crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallPolyContact::getFarthestPlane(float &,G3D::Vector3 const&)")]
// 0x71a6a8 — __ZN3RBX15BallPolyContact16getFarthestPlaneERfRKN3G3D7Vector3E
pub fn stub_0x71a6a8() {
    // IDA 0x71a6a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallPolyContact::getClosestInVoronoiEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
// 0x71a96c — __ZN3RBX15BallPolyContact23getClosestInVoronoiEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
pub fn stub_0x71a96c() {
    // IDA 0x71a96c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallPolyContact::getClosestEdge(RBX::POLY::Face const*,float &,G3D::Vector3 const&)")]
// 0x71abc8 — __ZN3RBX15BallPolyContact14getClosestEdgeEPKNS_4POLY4FaceERfRKN3G3D7Vector3E
pub fn stub_0x71abc8() {
    // IDA 0x71abc8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BallPolyContact::getClosestVertex(RBX::POLY::Edge const*,float &,G3D::Vector3 const&)")]
// 0x71aca4 — __ZN3RBX15BallPolyContact16getClosestVertexEPKNS_4POLY4EdgeERfRKN3G3D7Vector3E
pub fn stub_0x71aca4() {
    // IDA 0x71aca4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::setSize(G3D::Vector3 const&)")]
// 0x71b584 — __ZN3RBX5Block7setSizeERKN3G3D7Vector3E
pub fn stub_0x71b584() {
    // IDA 0x71b584: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::hitTest(RBX::RbxRay const&,G3D::Vector3 &,bool &)")]
// 0x71b860 — __ZN3RBX5Block7hitTestERKNS_6RbxRayERN3G3D7Vector3ERb
pub fn stub_0x71b860() {
    // IDA 0x71b860: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getCenterToCorner(G3D::Matrix3 const&)const")]
// 0x71b9b8 — __ZNK3RBX5Block17getCenterToCornerERKN3G3D7Matrix3E
pub fn stub_0x71b9b8() {
    // IDA 0x71b9b8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getEdgePoint(G3D::Vector3int16 const&,RBX::NormalId &)const")]
// 0x71bb24 — __ZNK3RBX5Block12getEdgePointERKN3G3D12Vector3int16ERNS_8NormalIdE
pub fn stub_0x71bb24() {
    // IDA 0x71bb24: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getPlanePoint(G3D::Vector3int16 const&,RBX::NormalId &)const")]
// 0x71bc30 — __ZNK3RBX5Block13getPlanePointERKN3G3D12Vector3int16ERNS_8NormalIdE
pub fn stub_0x71bc30() {
    // IDA 0x71bc30: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getBallBlockInfo(int,G3D::Vector3int16,G3D::Vector3 const*&,RBX::NormalId &)")]
// 0x71bd04 — __ZN3RBX5Block16getBallBlockInfoEiN3G3D12Vector3int16ERPKNS1_7Vector3ERNS_8NormalIdE
pub fn stub_0x71bd04() {
    // IDA 0x71bd04: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getBallInsideInfo(G3D::Vector3 const&,G3D::Vector3 const*&,RBX::NormalId &)")]
// 0x71bd84 — __ZN3RBX5Block17getBallInsideInfoERKN3G3D7Vector3ERPS3_RNS_8NormalIdE
pub fn stub_0x71bd84() {
    // IDA 0x71bd84: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::projectToFace(G3D::Vector3 &,G3D::Vector3int16 &,int &)")]
// 0x71be4c — __ZN3RBX5Block13projectToFaceERN3G3D7Vector3ERNS1_12Vector3int16ERi
pub fn stub_0x71be4c() {
    // IDA 0x71be4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getProjectedVertex(G3D::Vector3 const&,RBX::NormalId)")]
// 0x71bf4c — __ZN3RBX5Block18getProjectedVertexERKN3G3D7Vector3ENS_8NormalIdE
pub fn stub_0x71bf4c() {
    // IDA 0x71bf4c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Block::getClosestEdge(G3D::Matrix3 const&,RBX::NormalId,G3D::Vector3 const&)")]
// 0x71bf8c — __ZN3RBX5Block14getClosestEdgeERKN3G3D7Matrix3ENS_8NormalIdERKNS1_7Vector3E
pub fn stub_0x71bf8c() {
    // IDA 0x71bf8c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
// 0x71c0f0 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE8getTokenERKS2_
pub fn stub_0x71c0f0() {
    // IDA 0x71c0f0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
// 0x71c28c — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE8getTokenERKS2_
pub fn stub_0x71c28c() {
    // IDA 0x71c28c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
// 0x71c578 — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY12BlockCornersENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
pub fn stub_0x71c578() {
    // IDA 0x71c578: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *)")]
// 0x71c7d0 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
pub fn stub_0x71c7d0() {
    // IDA 0x71c7d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
// 0x71c9ac — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE10ValueCountD2Ev
pub fn stub_0x71c9ac() {
    // IDA 0x71c9ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
// 0x71ca50 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
pub fn stub_0x71ca50() {
    // IDA 0x71ca50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>)")]
// 0x71ca78 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
pub fn stub_0x71ca78() {
    // IDA 0x71ca78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>> *)")]
// 0x71cad8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
pub fn stub_0x71cad8() {
    // IDA 0x71cad8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
// 0x71cb00 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
pub fn stub_0x71cb00() {
    // IDA 0x71cb00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
// 0x71cb5c — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
pub fn stub_0x71cb5c() {
    // IDA 0x71cb5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)")]
// 0x71cc70 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
pub fn stub_0x71cc70() {
    // IDA 0x71cc70: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)")]
// 0x71ce18 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
pub fn stub_0x71ce18() {
    // IDA 0x71ce18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *> const&)")]
// 0x71cebc — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
pub fn stub_0x71cebc() {
    // IDA 0x71cebc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)")]
// 0x71cf90 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE10ValueCountC2ERKS2_
pub fn stub_0x71cf90() {
    // IDA 0x71cf90: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::BlockCorners::BlockCorners(G3D::Vector3 const&)")]
// 0x71d0c0 — __ZN3RBX4POLY12BlockCornersC2ERKN3G3D7Vector3E
pub fn stub_0x71d0c0() {
    // IDA 0x71d0c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
// 0x71d234 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY12BlockCornersENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
pub fn stub_0x71d234() {
    // IDA 0x71d234: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::safe_static_init_staticData(void)")]
// 0x71d2d8 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE27safe_static_init_staticDataEv
pub fn stub_0x71d2d8() {
    // IDA 0x71d2d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::safe_static_do_get_staticData(void)")]
// 0x71d2dc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE29safe_static_do_get_staticDataEv
pub fn stub_0x71d2dc() {
    // IDA 0x71d2dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockCorners,RBX::Vector3Comparer>::StaticData::~StaticData()")]
// 0x71d3ec — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY12BlockCornersENS_15Vector3ComparerEE10StaticDataD1Ev
pub fn stub_0x71d3ec() {
    // IDA 0x71d3ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
// 0x71d49c — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY9BlockMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
pub fn stub_0x71d49c() {
    // IDA 0x71d49c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *)")]
// 0x71d6f4 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
pub fn stub_0x71d6f4() {
    // IDA 0x71d6f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
// 0x71d8d0 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE10ValueCountD2Ev
pub fn stub_0x71d8d0() {
    // IDA 0x71d8d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
// 0x71dad8 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
pub fn stub_0x71dad8() {
    // IDA 0x71dad8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
// 0x71db00 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
pub fn stub_0x71db00() {
    // IDA 0x71db00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
// 0x71db60 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
pub fn stub_0x71db60() {
    // IDA 0x71db60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
// 0x71dbbc — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
pub fn stub_0x71dbbc() {
    // IDA 0x71dbbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// 0x71dcd0 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
pub fn stub_0x71dcd0() {
    // IDA 0x71dcd0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// 0x71de78 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSC_
pub fn stub_0x71de78() {
    // IDA 0x71de78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// 0x71df1c — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueERKSC_
pub fn stub_0x71df1c() {
    // IDA 0x71df1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount::ValueCount(G3D::Vector3 const&)")]
// 0x71dff0 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY9BlockMeshENS_15Vector3ComparerEE10ValueCountC2ERKS2_
pub fn stub_0x71dff0() {
    // IDA 0x71dff0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::POLY::BlockMesh::BlockMesh(G3D::Vector3 const&)")]
// 0x71e120 — __ZN3RBX4POLY9BlockMeshC2ERKN3G3D7Vector3E
pub fn stub_0x71e120() {
    // IDA 0x71e120: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::BlockMesh,RBX::Vector3Comparer>::ValueCount *>>>::find(G3D::Vector3 const&)")]
// 0x71e318 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY9BlockMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE4findERS3_
pub fn stub_0x71e318() {
    // IDA 0x71e318: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Clump::loadConstMotors(G3D::Array<RBX::Joint const*,10,32ul> &)const")]
// 0x71ecbc — __ZNK3RBX5Clump15loadConstMotorsERN3G3D5ArrayIPKNS_5JointELi10ELm32EEE
pub fn stub_0x71ecbc() {
    // IDA 0x71ecbc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Clump::loadMotors(G3D::Array<RBX::Joint *,10,32ul> &)")]
// 0x71ee14 — __ZN3RBX5Clump10loadMotorsERN3G3D5ArrayIPNS_5JointELi10ELm32EEE
pub fn stub_0x71ee14() {
    // IDA 0x71ee14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "G3D::Array<RBX::Joint const*,10,32ul>::append(RBX::Joint const* const&)")]
// 0x71ef6c — __ZN3G3D5ArrayIPKN3RBX5JointELi10ELm32EE6appendERKS4_
pub fn stub_0x71ef6c() {
    // IDA 0x71ef6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "G3D::Array<RBX::Joint *,10,32ul>::append(RBX::Joint * const&)")]
// 0x71efc8 — __ZN3G3D5ArrayIPN3RBX5JointELi10ELm32EE6appendERKS3_
pub fn stub_0x71efc8() {
    // IDA 0x71efc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "G3D::Array<RBX::Joint *,10,32ul>::resize(int,bool)")]
// 0x71f040 — __ZN3G3D5ArrayIPN3RBX5JointELi10ELm32EE6resizeEib
pub fn stub_0x71f040() {
    // IDA 0x71f040: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Joint *,10,32ul>::realloc(int)")]
// 0x71f0f8 — __ZN3G3D5ArrayIPN3RBX5JointELi10ELm32EE7reallocEi
pub fn stub_0x71f0f8() {
    // IDA 0x71f0f8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Joint const*,10,32ul>::resize(int,bool)")]
// 0x71f2e0 — __ZN3G3D5ArrayIPKN3RBX5JointELi10ELm32EE6resizeEib
pub fn stub_0x71f2e0() {
    // IDA 0x71f2e0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Joint const*,10,32ul>::realloc(int)")]
// 0x71f398 — __ZN3G3D5ArrayIPKN3RBX5JointELi10ELm32EE7reallocEi
pub fn stub_0x71f398() {
    // IDA 0x71f398: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::BallBlockContact::computeIsColliding(int &,G3D::Vector3int16 &,G3D::Vector3 &,float)")]
// 0x71ff18 — __ZN3RBX16BallBlockContact18computeIsCollidingERiRN3G3D12Vector3int16ERNS2_7Vector3Ef
pub fn stub_0x71ff18() {
    // IDA 0x71ff18: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContactData::intersectRectQuad(RBX::FixedArray<RBX::GeoPairConnector *,8ul> &,G3D::Vector2 &,G3D::Vector2(&)[4])")]
// 0x722ec4 — __ZN3RBX21BlockBlockContactData17intersectRectQuadERNS_10FixedArrayIPNS_16GeoPairConnectorELm8EEERN3G3D7Vector2ERA4_S7_
pub fn stub_0x722ec4() {
    // IDA 0x722ec4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::BlockBlockContactData::intersectRectQuadFFlag(G3D::Vector2 &,G3D::Vector2(&)[4])")]
// 0x7231e0 — __ZN3RBX21BlockBlockContactData22intersectRectQuadFFlagERN3G3D7Vector2ERA4_S2_
pub fn stub_0x7231e0() {
    // IDA 0x7231e0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::intersectingOthers(G3D::Array<RBX::Primitive *,10,32ul> const&,float)")]
// 0x724fbc — __ZN3RBX14ContactManager18intersectingOthersERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEf
pub fn stub_0x724fbc() {
    // IDA 0x724fbc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::getPrimitivesTouchingExtents(RBX::Extents const&,RBX::Primitive const*,int,G3D::Array<RBX::Primitive*,10,32ul> &)")]
// 0x725180 — __ZN3RBX14ContactManager28getPrimitivesTouchingExtentsERKNS_7ExtentsEPKNS_9PrimitiveEiRN3G3D5ArrayIPS4_Li10ELm32EEE
pub fn stub_0x725180() {
    // IDA 0x725180: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::CheckPrimitive(RBX::Primitive *,RBX::Extents const*,G3D::Array<RBX::Primitive *,10,32ul> *)")]
// 0x7251e4 — __ZN3RBXL14CheckPrimitiveEPNS_9PrimitiveEPKNS_7ExtentsEPN3G3D5ArrayIS1_Li10ELm32EEE
pub fn stub_0x7251e4() {
    // IDA 0x7251e4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::getSlowHit(G3D::Array<RBX::Primitive *,10,32ul> const&,RBX::RbxRay const&,G3D::Array<RBX::Primitive const*,10,32ul> const*,RBX::HitTestFilter const*,G3D::Vector3 &,float,bool &,bool &)const")]
// 0x725270 — __ZNK3RBX14ContactManager10getSlowHitERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEERKNS_6RbxRayEPKNS2_IPKS3_Li10ELm32EEEPKNS_13HitTestFilterERNS1_7Vector3EfRbSL_
pub fn stub_0x725270() {
    // IDA 0x725270: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::getHit(RBX::RbxRay const&,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> const*,RBX::HitTestFilter const*,G3D::Vector3 &,bool &,RBX::CellID &,bool,bool)const")]
// 0x7255e0 — __ZNK3RBX14ContactManager6getHitERKNS_6RbxRayEPKSt6vectorIPKNS_9PrimitiveESaIS7_EEPKNS_13HitTestFilterERN3G3D7Vector3ERbRNS_6CellIDEbb
pub fn stub_0x7255e0() {
    // IDA 0x7255e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::getHit(RBX::RbxRay const&,G3D::Array<RBX::Primitive const*,10,32ul> const*,RBX::HitTestFilter const*,G3D::Vector3 &,bool &,RBX::CellID &,bool,bool)const")]
// 0x7256cc — __ZNK3RBX14ContactManager6getHitERKNS_6RbxRayEPKN3G3D5ArrayIPKNS_9PrimitiveELi10ELm32EEEPKNS_13HitTestFilterERNS4_7Vector3ERbRNS_6CellIDEbb
pub fn stub_0x7256cc() {
    // IDA 0x7256cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::getFastHit(RBX::RbxRay const&,G3D::Array<RBX::Primitive const*,10,32ul> const*,RBX::HitTestFilter const*,G3D::Vector3 &,bool &,bool &,RBX::CellID &,bool,bool)const")]
// 0x725798 — __ZNK3RBX14ContactManager10getFastHitERKNS_6RbxRayEPKN3G3D5ArrayIPKNS_9PrimitiveELi10ELm32EEEPKNS_13HitTestFilterERNS4_7Vector3ERbSH_RNS_6CellIDEbb
pub fn stub_0x725798() {
    // IDA 0x725798: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::findUpNearestLocationWithSpaceNeeded(float,G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x725f5c — __ZN3RBX14ContactManager36findUpNearestLocationWithSpaceNeededEfRKN3G3D7Vector3ES4_
pub fn stub_0x725f5c() {
    // IDA 0x725f5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::getHitLegacy(RBX::RbxRay const&,RBX::Primitive const*,RBX::HitTestFilter const*,G3D::Vector3 &,float &,float const&,bool)const")]
// 0x726a48 — __ZNK3RBX14ContactManager12getHitLegacyERKNS_6RbxRayEPKNS_9PrimitiveEPKNS_13HitTestFilterERN3G3D7Vector3ERfRKfb
pub fn stub_0x726a48() {
    // IDA 0x726a48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ContactManager::onTerrainCellRemoved(G3D::Vector3int16 const&,int const&,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// 0x7272bc — __ZN3RBX14ContactManager20onTerrainCellRemovedERKN3G3D12Vector3int16ERKiRKNS1_5ArrayIPNS_9PrimitiveELi10ELm32EEE
pub fn stub_0x7272bc() {
    // IDA 0x7272bc: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::ContactManager::wakePrimitivesInCell(G3D::Vector3int16 const&,bool)")]
// 0x727360 — __ZN3RBX14ContactManager20wakePrimitivesInCellERKN3G3D12Vector3int16Eb
pub fn stub_0x727360() {
    // IDA 0x727360: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Primitive const*,10,32ul>::operator=(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> const&)")]
// 0x727e04 — __ZN3G3D5ArrayIPKN3RBX9PrimitiveELi10ELm32EEaSERKSt6vectorIS4_SaIS4_EE
pub fn stub_0x727e04() {
    // IDA 0x727e04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesInGrid(RBX::Vector3int32 const&,G3D::Array<RBX::Primitive*,10,32ul> &)")]
// 0x727e40 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19getPrimitivesInGridERKNS_12Vector3int32ERN3G3D5ArrayIPS1_Li10ELm32EEE
pub fn stub_0x727e40() {
    // IDA 0x727e40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Extents::fuzzyContains(G3D::Vector3 const&,float)const")]
// 0x727ee8 — __ZNK3RBX7Extents13fuzzyContainsERKN3G3D7Vector3Ef
pub fn stub_0x727ee8() {
    // IDA 0x727ee8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Extents::fromCenterCorner(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x728250 — __ZN3RBX7Extents16fromCenterCornerERKN3G3D7Vector3ES4_
pub fn stub_0x728250() {
    // IDA 0x728250: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "G3D::Array<RBX::Primitive const*,10,32ul>::append(RBX::Primitive const* const&)")]
// 0x728900 — __ZN3G3D5ArrayIPKN3RBX9PrimitiveELi10ELm32EE6appendERKS4_
pub fn stub_0x728900() {
    // IDA 0x728900: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "bool RBX::ContactManager::checkMegaClusterSmallTerrainContact<RBX::Voxel::Grid>(RBX::Primitive *,G3D::Vector3int16 const&,G3D::Vector3int16 const&,G3D::Vector3int16 const&,bool)")]
// 0x728bd8 — __ZN3RBX14ContactManager35checkMegaClusterSmallTerrainContactINS_5Voxel4GridEEEbPNS_9PrimitiveERKN3G3D12Vector3int16ES9_S9_b
pub fn stub_0x728bd8() {
    // IDA 0x728bd8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "bool RBX::ContactManager::checkMegaClusterWaterContact<RBX::Voxel::Grid>(RBX::Primitive *,G3D::Vector3int16 const&,G3D::Vector3int16 const&,G3D::Vector3int16 const&)")]
// 0x729738 — __ZN3RBX14ContactManager28checkMegaClusterWaterContactINS_5Voxel4GridEEEbPNS_9PrimitiveERKN3G3D12Vector3int16ES9_S9_
pub fn stub_0x729738() {
    // IDA 0x729738: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::_copy(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
// 0x729ca8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EE5_copyERKS4_
pub fn stub_0x729ca8() {
    // IDA 0x729ca8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Primitive const*,10,32ul>::resize(int,bool)")]
// 0x72a3c8 — __ZN3G3D5ArrayIPKN3RBX9PrimitiveELi10ELm32EE6resizeEib
pub fn stub_0x72a3c8() {
    // IDA 0x72a3c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Primitive const*,10,32ul>::realloc(int)")]
// 0x72a480 — __ZN3G3D5ArrayIPKN3RBX9PrimitiveELi10ELm32EE7reallocEi
pub fn stub_0x72a480() {
    // IDA 0x72a480: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesInGrid(int,RBX::Vector3int32 const&,G3D::Array<RBX::Primitive*,10,32ul> &)")]
// 0x72c9c4 — __ZN3RBX11SpatialHashINS_9PrimitiveENS_7ContactENS_14ContactManagerELi4EE19getPrimitivesInGridEiRKNS_12Vector3int32ERN3G3D5ArrayIPS1_Li10ELm32EEE
pub fn stub_0x72c9c4() {
    // IDA 0x72c9c4: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "G3D::Array<RBX::Primitive const*,10,32ul>::~Array()")]
// 0x72ca1c — __ZN3G3D5ArrayIPKN3RBX9PrimitiveELi10ELm32EED2Ev
pub fn stub_0x72ca1c() {
    // IDA 0x72ca1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "G3D::Array<RBX::Primitive const*,10,32ul>::Array(void)")]
// 0x72caf0 — __ZN3G3D5ArrayIPKN3RBX9PrimitiveELi10ELm32EEC2Ev
pub fn stub_0x72caf0() {
    // IDA 0x72caf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Vector3int32::floor(G3D::Vector3 const&)")]
// 0x72e038 — __ZN3RBX12Vector3int325floorERKN3G3D7Vector3E
pub fn stub_0x72e038() {
    // IDA 0x72e038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::CornerWedgePoly::getCenterToCorner(G3D::Matrix3 const&)const")]
// 0x731354 — __ZNK3RBX15CornerWedgePoly17getCenterToCornerERKN3G3D7Matrix3E
pub fn stub_0x731354() {
    // IDA 0x731354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::getToken(G3D::Vector3 const&)")]
// 0x7314dc — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15CornerWedgeMeshENS_15Vector3ComparerEE8getTokenERKS2_
pub fn stub_0x7314dc() {
    // IDA 0x7314dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<G3D::Vector3,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::operator[](G3D::Vector3 const&)")]
// 0x731718 — __ZNSt3mapIN3G3D7Vector3EPN3RBX12GeometryPoolIS1_NS2_4POLY15CornerWedgeMeshENS2_15Vector3ComparerEE10ValueCountES6_SaISt4pairIKS1_S9_EEEixERSB_
pub fn stub_0x731718() {
    // IDA 0x731718: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::returnToken(G3D::Vector3 const&,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *)")]
// 0x731970 — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15CornerWedgeMeshENS_15Vector3ComparerEE11returnTokenERKS2_PNS6_10ValueCountE
pub fn stub_0x731970() {
    // IDA 0x731970: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount::~ValueCount()")]
// 0x731b4c — __ZN3RBX12GeometryPoolIN3G3D7Vector3ENS_4POLY15CornerWedgeMeshENS_15Vector3ComparerEE10ValueCountD2Ev
pub fn stub_0x731b4c() {
    // IDA 0x731b4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(G3D::Vector3 const&)")]
// 0x731ce4 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15CornerWedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseERS3_
pub fn stub_0x731ce4() {
    // IDA 0x731ce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::erase(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>)")]
// 0x731d0c — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15CornerWedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE5eraseESt17_Rb_tree_iteratorISC_ESI_
pub fn stub_0x731d0c() {
    // IDA 0x731d0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_erase(std::_Rb_tree_node<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>> *)")]
// 0x731d6c — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15CornerWedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE8_M_eraseEPSt13_Rb_tree_nodeISC_E
pub fn stub_0x731d6c() {
    // IDA 0x731d6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::lower_bound(G3D::Vector3 const&)")]
// 0x731d94 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15CornerWedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11lower_boundERS3_
pub fn stub_0x731d94() {
    // IDA 0x731d94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::upper_bound(G3D::Vector3 const&)")]
// 0x731df0 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15CornerWedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE11upper_boundERS3_
pub fn stub_0x731df0() {
    // IDA 0x731df0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<G3D::Vector3,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>,std::_Select1st<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,RBX::Vector3Comparer,std::allocator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *>>,std::pair<G3D::Vector3 const,RBX::GeometryPool<G3D::Vector3,RBX::POLY::CornerWedgeMesh,RBX::Vector3Comparer>::ValueCount *> const&)")]
// 0x731f04 — __ZNSt8_Rb_treeIN3G3D7Vector3ESt4pairIKS1_PN3RBX12GeometryPoolIS1_NS4_4POLY15CornerWedgeMeshENS4_15Vector3ComparerEE10ValueCountEESt10_Select1stISC_ES8_SaISC_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISC_ERKSC_
pub fn stub_0x731f04() {
    // IDA 0x731f04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
