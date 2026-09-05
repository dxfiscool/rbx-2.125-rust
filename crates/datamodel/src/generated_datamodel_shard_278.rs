// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x357924..0x35e060 | datamodel distinct 34259->34379 global uncovered 52087->51967, lowest gap EA-sorted asc next 120 after 0x357858 (post shard_277)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed
// Shard: datamodel_shard_278 EA-sorted ascending next uncovered gap after datamodel_shard_277 (distinct check via export.json sorted EA, no overlap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use std::collections::HashMap;

/// Rust model of `G3D::Line` (IDA `0x35a0f4`): point-plus-direction line;
/// layout unmodeled — only the trivial destructor is modeled so far.
#[derive(Default)]
pub struct G3dLine {
    _opaque: (),
}

/// Rust model of `RBX::MD5HasherImpl` (IDA `0x35a6f4`): the vtable plus the
/// owned hex-digest string at `+96`; remaining layout unmodeled.
#[derive(Default)]
pub struct Md5HasherImpl {
    pub digest: String,
}

/// Rust model of `RBX::Name` (IDA `0x35bfec`): the interned-name node;
/// layout unmodeled — only pointer identity travels through the map.
#[derive(Default)]
pub struct Name {
    _opaque: (),
}

/// Rust model of `RBX::Name::NameMap` (IDA `0x35bfec`):
/// `boost::unordered_map<std::string, RBX::Name*>` — the key string plus the
/// node pointer per entry; buckets collapse into the map itself.
#[derive(Default)]
pub struct NameMap {
    pub entries: HashMap<String, *const Name>,
}

// 0x357924 — __ZN3RBX4Math12rotateAboutZERKN3G3D7Matrix3Ef
#[doc(alias = "RBX::Math::rotateAboutZ(G3D::Matrix3 const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_357924 as stub_357924;

// 0x3579d0 — __ZN3RBX4Math10snapToAxesERKN3G3D7Matrix3E
#[doc(alias = "RBX::Math::snapToAxes(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_3579d0 as stub_3579d0;

// 0x357c08 — __ZN3RBX4Math6toGridERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::Math::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357c08 as stub_357c08;

// 0x357c84 — __ZN3RBX4Math13iRoundVector3ERKN3G3D7Vector3E
#[doc(alias = "RBX::Math::iRoundVector3(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357c84 as stub_357c84;

// 0x357ce4 — __ZN3RBX4Math6toGridERKN3G3D7Vector3Ef
#[doc(alias = "RBX::Math::toGrid(G3D::Vector3 const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_357ce4 as stub_357ce4;

// 0x357cfc — __ZN3RBX4Math10snapToGridERKN3G3D15CoordinateFrameEf
#[doc(alias = "RBX::Math::snapToGrid(G3D::CoordinateFrame const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_357cfc as stub_357cfc;

// 0x357d44 — __ZN3RBX4Math10snapToGridERKN3G3D15CoordinateFrameERKNS1_7Vector3E
#[doc(alias = "RBX::Math::snapToGrid(G3D::CoordinateFrame const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357d44 as stub_357d44;

// 0x357d88 — __ZN3RBX4Math13safeDirectionERKN3G3D7Vector3E
#[doc(alias = "RBX::Math::safeDirection(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357d88 as stub_357d88;

// 0x357ee4 — __ZN3RBX4Math5angleERKN3G3D7Vector3ES4_
#[doc(alias = "RBX::Math::angle(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357ee4 as stub_357ee4;

// 0x357f48 — __ZN3RBX4Math14elevationAngleERKN3G3D7Vector3E
#[doc(alias = "RBX::Math::elevationAngle(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357f48 as stub_357f48;

// 0x357fa0 — __ZN3RBX4Math16fuzzyAxisAlignedERKN3G3D7Matrix3ES4_f
#[doc(alias = "RBX::Math::fuzzyAxisAligned(G3D::Matrix3 const&,G3D::Matrix3 const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_357fa0 as stub_357fa0;

// 0x3580b4 — __ZN3RBX4Math13isOrthonormalERKN3G3D7Matrix3E
#[doc(alias = "RBX::Math::isOrthonormal(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_3580b4 as stub_3580b4;

// 0x3580c0 — __ZN3RBX4Math7fuzzyEqERKN3G3D7Vector3ES4_f
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_3580c0 as stub_3580c0;

// 0x35810c — __ZN3RBX4Math7fuzzyEqERKN3G3D7Matrix3ES4_f
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Matrix3 const&,G3D::Matrix3 const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_35810c as stub_35810c;

// 0x35817c — __ZN3RBX4Math7fuzzyEqERKN3G3D7Matrix4ES4_f
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Matrix4 const&,G3D::Matrix4 const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_35817c as stub_35817c;

// 0x3581ec — __ZN3RBX4Math7fuzzyEqERKN3G3D15CoordinateFrameES4_ff
#[doc(alias = "RBX::Math::fuzzyEq(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float)")]
pub use rbx_core::generated_core_shard_hz::stub_3581ec as stub_3581ec;

// 0x358254 — __ZN3RBX4Math18rotateAboutYGlobalERN3G3D15CoordinateFrameEf
#[doc(alias = "RBX::Math::rotateAboutYGlobal(G3D::CoordinateFrame &,float)")]
pub use rbx_core::generated_core_shard_hz::stub_358254 as stub_358254;

// 0x35829c — __ZN3RBX4Math18rotateAboutYGlobalERKN3G3D7Vector3Ef
#[doc(alias = "RBX::Math::rotateAboutYGlobal(G3D::Vector3 const&,float)")]
pub use rbx_core::generated_core_shard_hz::stub_35829c as stub_35829c;

// 0x358314 — __ZN3RBX4Math24getClosestObjectNormalIdERKN3G3D7Vector3ERKNS1_7Matrix3E
#[doc(alias = "RBX::Math::getClosestObjectNormalId(G3D::Vector3 const&,G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_358314 as stub_358314;

// 0x3583cc — __ZN3RBX4Math11sortVector3ERKN3G3D7Vector3E
#[doc(alias = "RBX::Math::sortVector3(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_3583cc as stub_3583cc;

// 0x358430 — __ZN3RBX4Math10vector3AbsERKN3G3D7Vector3E
#[doc(alias = "RBX::Math::vector3Abs(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_358430 as stub_358430;

// 0x358460 — __ZN3RBX4Math17closestPointOnRayERKNS_6RbxRayES3_
#[doc(alias = "RBX::Math::closestPointOnRay(RBX::RbxRay const&,RBX::RbxRay const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x358460 as stub_358460;

// 0x35856c — __ZN3RBX4Math13matrixRotateXEv
#[doc(alias = "RBX::Math::matrixRotateX(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35856c as stub_35856c;

// 0x358658 — __ZN3RBX4Math13matrixRotateYEv
#[doc(alias = "RBX::Math::matrixRotateY(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x358658 as stub_358658;

// 0x358744 — __ZN3RBX4Math11matrixTiltZEv
#[doc(alias = "RBX::Math::matrixTiltZ(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x358744 as stub_358744;

// 0x358830 — __ZN3RBX4Math19matrixTiltNegativeZEv
#[doc(alias = "RBX::Math::matrixTiltNegativeZ(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x358830 as stub_358830;

// 0x358918 — __ZN3RBX4Math18matrixTiltQuadrantEi
#[doc(alias = "RBX::Math::matrixTiltQuadrant(int)")]
pub use rbx_core::generated_core_shard_aq::stub_0x358918 as stub_358918;

// 0x3589e8 — __ZN3RBX4Math17radiansToQuadrantEf
#[doc(alias = "RBX::Math::radiansToQuadrant(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x3589e8 as stub_3589e8;

// 0x358aa4 — __ZN3RBX4Math15toYAxisQuadrantERKN3G3D15CoordinateFrameE
#[doc(alias = "RBX::Math::toYAxisQuadrant(G3D::CoordinateFrame const&)")]
pub use rbx_core::generated_core_shard_hz::stub_358aa4 as stub_358aa4;

// 0x358ae4 — __ZN3RBX4Math25intersectRayConvexPolygonERKNS_6RbxRayERKSt6vectorIN3G3D7Vector3ESaIS6_EERS6_b
#[doc(alias = "RBX::Math::intersectRayConvexPolygon(RBX::RbxRay const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,G3D::Vector3&,bool)")]
pub use rbx_core::generated_core_shard_hz::stub_358ae4 as stub_358ae4;

// 0x358d38 — __ZN3RBX4Math17intersectRayPlaneERKNS_6RbxRayERKN3G3D5PlaneERNS4_7Vector3E
#[doc(alias = "RBX::Math::intersectRayPlane(RBX::RbxRay const&,G3D::Plane const&,G3D::Vector3 &)")]
pub use rbx_core::generated_core_shard_hz::stub_358d38 as stub_358d38;

// 0x358ea0 — __ZN3RBX4Math26spatialPolygonIntersectionERKSt6vectorIN3G3D7Vector3ESaIS3_EES7_
#[doc(alias = "RBX::Math::spatialPolygonIntersection(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
pub use rbx_core::generated_core_shard_hz::stub_358ea0 as stub_358ea0;

// 0x3594fc — __ZN3RBX4Math25planarPolygonIntersectionERKSt6vectorIN3G3D7Vector2ESaIS3_EES7_
#[doc(alias = "RBX::Math::planarPolygonIntersection(std::vector<G3D::Vector2,std::allocator<G3D::Vector2>> const&,std::vector<G3D::Vector2,std::allocator<G3D::Vector2>> const&)")]
pub use rbx_core::generated_core_shard_hz::stub_3594fc as stub_3594fc;

// 0x3596f8 — __ZN3RBX4Math18intersectLinePlaneERKN3G3D4LineERKNS1_5PlaneERNS1_7Vector3E
#[doc(alias = "RBX::Math::intersectLinePlane(G3D::Line const&,G3D::Plane const&,G3D::Vector3 &)")]
pub use rbx_core::generated_core_shard_hz::stub_3596f8 as stub_3596f8;

// 0x359764 — __ZN3RBX4Math21getAxisRotationMatrixEi
#[doc(alias = "RBX::Math::getAxisRotationMatrix(int)")]
pub use rbx_core::generated_core_shard_aq::stub_0x359764 as stub_359764;

// 0x3599b8 — __ZN3RBX4Math29lineSegmentDistanceIfCrossingERKN3G3D7Vector3ES4_S4_S4_Rff
#[doc(alias = "RBX::Math::lineSegmentDistanceIfCrossing(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float &,float)")]
pub use rbx_core::generated_core_shard_hz::stub_3599b8 as stub_3599b8;

// 0x359be0 — __ZN3RBX4Math20polygonStartingPointEif
#[doc(alias = "RBX::Math::polygonStartingPoint(int,float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x359be0 as stub_359be0;

// 0x359d50 — __ZN3RBX4Math26getWellFormedRotForZVectorERKN3G3D7Vector3E
#[doc(alias = "RBX::Math::getWellFormedRotForZVector(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_359d50 as stub_359d50;

// 0x359ed0 — __ZN3RBX4Math20evenWholeNumberFuzzyERKf
#[doc(alias = "RBX::Math::evenWholeNumberFuzzy(float const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x359ed0 as stub_359ed0;

// 0x359f38 — __ZNSt6vectorIN3G3D7Vector2ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>::push_back(G3D::Vector2 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_359f38 as stub_359f38;

// 0x359f64 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::push_back(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_359f64 as stub_359f64;

// 0x359f98 — __ZNK3G3D5Plane17halfSpaceContainsENS_7Vector3E
#[doc(alias = "G3D::Plane::halfSpaceContains(G3D::Vector3)const")]
pub use rbx_core::generated_core_shard_hz::stub_359f98 as stub_359f98;

// 0x35a058 — __ZN3G3D4Line21fromPointAndDirectionERKNS_7Vector3ES3_
#[doc(alias = "G3D::Line::fromPointAndDirection(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_35a058 as stub_35a058;

// 0x35a0f4 — __ZN3G3D4LineD1Ev
#[doc(alias = "G3D::Line::~Line()")]
pub fn stub_35a0f4(_line: &G3dLine) {
    // IDA 0x35a0f4 (decompile: empty body; disasm: single `BX LR`) — D1
    // complete-object destructor of the trivially-destructible `G3D::Line`;
    // Drop glue covers it; no explicit body.
}

// 0x35a0f8 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3*,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>>,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_35a0f8 as stub_35a0f8;

// 0x35a24c — __ZNSt12_Vector_baseIN3G3D7Vector3ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector3,std::allocator<G3D::Vector3>>::_M_allocate(unsigned long)")]
pub use rbx_core::generated_core_shard_hz::stub_35a24c as stub_35a24c;

// 0x35a270 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
pub use rbx_core::generated_core_shard_hz::stub_35a270 as stub_35a270;

// 0x35a2d8 — __ZNSt6vectorIN3G3D7Vector2ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2*,std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>>,G3D::Vector2 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_35a2d8 as stub_35a2d8;

// 0x35a3e4 — __ZNSt12_Vector_baseIN3G3D7Vector2ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector2,std::allocator<G3D::Vector2>>::_M_allocate(unsigned long)")]
pub use rbx_core::generated_core_shard_hz::stub_35a3e4 as stub_35a3e4;

// 0x35a3fc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector2ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector2 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2 *,G3D::Vector2 *>(G3D::Vector2 *,G3D::Vector2 *,G3D::Vector2 *)")]
pub use rbx_core::generated_core_shard_hz::stub_35a3fc as stub_35a3fc;

// 0x35a448 — __ZN3G3D4LineD0Ev
#[doc(alias = "G3D::Line::~Line()")]
pub fn stub_35a448(line: *mut G3dLine) {
    // IDA 0x35a448 (decompile: `operator delete(this)` thunk; disasm: single
    // `B.W __ZdlPv$shim`) — D0 deleting destructor: D1 is empty (0x35a0f4),
    // so this is storage release only. Box reclaim frees the object.
    // SAFETY: `line` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(line));
    }
}

// 0x35a44c — __GLOBAL__I_a_127
#[doc(alias = "global constructor keyed to_a_127")]
pub use rbx_core::generated_core_shard_hz::stub_35a44c as stub_35a44c;

// 0x35a620 — __ZN3RBX9MD5Hasher6createEv
#[doc(alias = "RBX::MD5Hasher::create(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35a620 as stub_35a620;

// 0x35a6f4 — __ZN3RBX13MD5HasherImplD1Ev
#[doc(alias = "RBX::MD5HasherImpl::~MD5HasherImpl()")]
pub fn stub_35a6f4(hasher: &mut Md5HasherImpl) {
    // IDA 0x35a6f4 (decompile: vtable reset + `string::~string(this + 96)`;
    // disasm 0x35a6f4-0x35a716: vtable `STR` then `BLX string::~string`) — D1
    // complete-object destructor; the vtable reset is compiler-managed, the
    // owned digest string is destroyed in place.
    std::mem::take(&mut hasher.digest);
}

// 0x35a718 — __ZN3RBX13MD5HasherImplD0Ev
#[doc(alias = "RBX::MD5HasherImpl::~MD5HasherImpl()")]
pub fn stub_35a718(hasher: *mut Md5HasherImpl) {
    // IDA 0x35a718 (decompile: same vtable reset + string destroy as D1
    // 0x35a6f4, then `operator delete(this)`; disasm 0x35a718-0x35a73e ends
    // `B.W __ZdlPv$shim`) — D0 deleting destructor: D1 member destroy plus
    // storage release.
    // SAFETY: `hasher` must be a live box pointer never used again.
    unsafe {
        stub_35a6f4(&mut *hasher);
        drop(Box::from_raw(hasher));
    }
}

// 0x35a744 — __ZN3RBX13MD5HasherImpl7addDataERSi
#[doc(alias = "RBX::MD5HasherImpl::addData(std::istream &)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35a744 as stub_35a744;

// 0x35a7c4 — __ZN3RBX13MD5HasherImpl7addDataERKSs
#[doc(alias = "RBX::MD5HasherImpl::addData(std::string const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35a7c4 as stub_35a7c4;

// 0x35a7d0 — __ZN3RBX13MD5HasherImpl7addDataEPKcm
#[doc(alias = "RBX::MD5HasherImpl::addData(char const*,unsigned long)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35a7d0 as stub_35a7d0;

// 0x35a7d8 — __ZN3RBX13MD5HasherImpl8toStringEv
#[doc(alias = "RBX::MD5HasherImpl::toString(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35a7d8 as stub_35a7d8;

// 0x35a7f4 — __ZN3RBX13MD5HasherImpl5c_strEv
#[doc(alias = "RBX::MD5HasherImpl::c_str(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35a7f4 as stub_35a7f4;

// 0x35a970 — __GLOBAL__I_a_128
#[doc(alias = "global constructor keyed to_a_128")]
pub use rbx_core::generated_core_shard_hz::stub_35a970 as stub_35a970;

// 0x35aa38 — __ZN3RBX15StringConverterINS_6MeshIdEE14convertToValueERKSsRS1_
#[doc(alias = "RBX::StringConverter<RBX::MeshId>::convertToValue(std::string const&,RBX::MeshId&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35aa38 as stub_35aa38;

// 0x35b3b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6MeshIdEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::MeshId>(RBX::MeshId const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35b3b0 as stub_35b3b0;

// 0x35b6bc — __ZN3rbx8any_castIN3RBX6MeshIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::MeshId * rbx::any_cast<RBX::MeshId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35b6bc as stub_35b6bc;

// 0x35b714 — __ZN3rbx8any_castIRN3RBX6MeshIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::MeshId & rbx::any_cast<RBX::MeshId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35b714 as stub_35b714;

// 0x35b804 — __ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::singleton(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35b804 as stub_35b804;

// 0x35b870 — __ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::construct_func(char const*,char *)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35b870 as stub_35b870;

// 0x35b88c — __ZN3rbx14implementation12typed_holderIN3RBX6MeshIdEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::destruct_func(char *)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35b88c as stub_35b88c;

// 0x35b890 — __GLOBAL__I_a_129
#[doc(alias = "global constructor keyed to_a_129")]
pub use rbx_core::generated_core_shard_hz::stub_35b890 as stub_35b890;

// 0x35ba98 — __ZN3RBX4NameC2ERKPKc
#[doc(alias = "RBX::Name::Name(char const* const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35ba98 as stub_35ba98;

// 0x35bbbc — __ZN3RBX4Name13setOrderIndexEv
#[doc(alias = "RBX::Name::setOrderIndex(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35bbbc as stub_35bbbc;

// 0x35bd48 — __ZN3RBX4Name6lookupERKPKc
#[doc(alias = "RBX::Name::lookup(char const* const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35bd48 as stub_35bd48;

// 0x35be98 — __ZN3RBX4Name11getNullNameEv
#[doc(alias = "RBX::Name::getNullName(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35be98 as stub_35be98;

// 0x35bebc — __ZN3RBX4Name6lookupERKSs
#[doc(alias = "RBX::Name::lookup(std::string const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35bebc as stub_35bebc;

// 0x35bfe8 — __ZN3RBX4Name7NameMapD1Ev
#[doc(alias = "RBX::Name::NameMap::~NameMap()")]
pub fn stub_35bfe8(map: &mut NameMap) {
    // IDA 0x35bfe8 (decompile: tail-call thunk to D2 0x35bfec; disasm: single
    // `B.W NameMap::~NameMap()`) — D1 complete-object destructor delegates
    // to the D2 base destructor.
    stub_35bfec(map);
}

// 0x35bfec — __ZN3RBX4Name7NameMapD2Ev
#[doc(alias = "RBX::Name::NameMap::~NameMap()")]
pub fn stub_35bfec(map: &mut NameMap) {
    // IDA 0x35bfec (decompile: per-node string destroy + node delete, then
    // `delete_buckets`; disasm 0x35bfec-0x35c028: node-walk loop with
    // `string::~string` + `operator delete`, then `delete_buckets`) — D2
    // base-object destructor: `clear` drops every key string and node entry,
    // and releases the buckets with the map.
    map.entries.clear();
}

// 0x35c02c — __ZL7initMoov
#[doc(alias = "initMoo(void)")]
pub use rbx_core::generated_core_shard_hz::stub_35c02c as stub_35c02c;

// 0x35c030 — __ZL4moo2v
#[doc(alias = "moo2(void)")]
pub use rbx_core::generated_core_shard_hz::stub_35c030 as stub_35c030;

// 0x35c10c — __ZN3RBX4Name3mapEv
#[doc(alias = "RBX::Name::map(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35c10c as stub_35c10c;

// 0x35c200 — __ZN3RBX4Name22approximateMemoryUsageEv
#[doc(alias = "RBX::Name::approximateMemoryUsage(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35c200 as stub_35c200;

// 0x35c218 — __ZN3RBX4Name4sizeEv
#[doc(alias = "RBX::Name::size(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35c218 as stub_35c218;

// 0x35c230 — __ZL15declareNullNamev
#[doc(alias = "declareNullName(void)")]
pub use rbx_core::generated_core_shard_hz::stub_35c230 as stub_35c230;

// 0x35c258 — __ZN3RBX4Name7declareERKPKc
#[doc(alias = "RBX::Name::declare(char const* const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35c258 as stub_35c258;

// 0x35c4b8 — __ZNSt6vectorIPN3RBX4NameESaIS2_EED1Ev
#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::~vector()")]
pub fn stub_35c4b8(names: Vec<*const Name>) {
    // IDA 0x35c4b8 (decompile: `v2 = *a1; if (v2) operator delete(v2)`;
    // disasm 0x35c4b8-0x35c4ca: null-checked buffer delete) — `vector<Name*>`
    // D1: elements are trivial pointers, so this frees the buffer. Taking
    // the vec by value drops elements (no-op) plus the buffer.
    drop(names);
}

// 0x35c4cc — __ZNSt6vectorIPN3RBX4NameESaIS2_EE6insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::insert(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35c4cc as stub_35c4cc;

// 0x35caf4 — __ZNSt6vectorIPN3RBX4NameESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35caf4 as stub_35caf4;

// 0x35cbd4 — __ZNSt12_Vector_baseIPN3RBX4NameESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Name *,std::allocator<RBX::Name *>>::_M_allocate(unsigned long)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35cbd4 as stub_35cbd4;

// 0x35ccc0 — __ZN3RBX16queuing_rw_mutexD1Ev
#[doc(alias = "RBX::queuing_rw_mutex::~queuing_rw_mutex()")]
pub use rbx_reflection::generated_refl_wdcron_A::stub_0x35ccc0 as stub_35ccc0;

// 0x35ccd0 — __ZN3RBX16queuing_rw_mutexC2Ev
#[doc(alias = "RBX::queuing_rw_mutex::queuing_rw_mutex(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35ccd0 as stub_35ccd0;

// 0x35ce18 — __GLOBAL__I_a_130
#[doc(alias = "global constructor keyed to_a_130")]
pub use rbx_core::generated_core_shard_hz::stub_35ce18 as stub_35ce18;

// 0x35cee0 — __ZN3RBX14normalIdToMaskENS_8NormalIdE
#[doc(alias = "RBX::normalIdToMask(RBX::NormalId)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35cee0 as stub_35cee0;

// 0x35cef8 — __ZN3RBX13validNormalIdENS_8NormalIdE
#[doc(alias = "RBX::validNormalId(RBX::NormalId)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35cef8 as stub_35cef8;

// 0x35cf04 — __ZN3RBX13intToNormalIdEi
#[doc(alias = "RBX::intToNormalId(int)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35cf04 as stub_35cf04;

// 0x35cf08 — __ZN3RBX16normalIdOppositeENS_8NormalIdE
#[doc(alias = "RBX::normalIdOpposite(RBX::NormalId)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35cf08 as stub_35cf08;

// 0x35cf24 — __ZN3RBX11normalIdToUENS_8NormalIdE
#[doc(alias = "RBX::normalIdToU(RBX::NormalId)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35cf24 as stub_35cf24;

// 0x35cfa8 — __ZN3RBX11uvwToObjectERKN3G3D7Vector3ENS_8NormalIdE
#[doc(alias = "RBX::uvwToObject(G3D::Vector3 const&,RBX::NormalId)")]
pub use rbx_core::generated_core_shard_hz::stub_35cfa8 as stub_35cfa8;

// 0x35d0c8 — __ZN3RBX11objectToUvwERKN3G3D7Vector3ENS_8NormalIdE
#[doc(alias = "RBX::objectToUvw(G3D::Vector3 const&,RBX::NormalId)")]
pub use rbx_core::generated_core_shard_hz::stub_35d0c8 as stub_35d0c8;

// 0x35d1e8 — __ZN3RBX17normalIdToVector3ENS_8NormalIdE
#[doc(alias = "RBX::normalIdToVector3(RBX::NormalId)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35d1e8 as stub_35d1e8;

// 0x35d3a8 — __ZN3RBX25normalIdToMatrix3InternalENS_8NormalIdE
#[doc(alias = "RBX::normalIdToMatrix3Internal(RBX::NormalId)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35d3a8 as stub_35d3a8;

// 0x35d5f4 — __ZN3RBX17normalIdToMatrix3ENS_8NormalIdE
#[doc(alias = "RBX::normalIdToMatrix3(RBX::NormalId)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35d5f4 as stub_35d5f4;

// 0x35d8a0 — __ZN3RBX17Vector3ToNormalIdERKN3G3D7Vector3E
#[doc(alias = "RBX::Vector3ToNormalId(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_35d8a0 as stub_35d8a0;

// 0x35db38 — __ZN3RBX17Matrix3ToNormalIdERKN3G3D7Matrix3E
#[doc(alias = "RBX::Matrix3ToNormalId(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_35db38 as stub_35db38;

// 0x35db54 — __GLOBAL__I_a_131
#[doc(alias = "global constructor keyed to_a_131")]
pub use rbx_core::generated_core_shard_hz::stub_35db54 as stub_35db54;

// 0x35db90 — __ZN3RBX9Profiling4initEb
#[doc(alias = "RBX::Profiling::init(bool)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35db90 as stub_35db90;

// 0x35dbc0 — __ZN3RBX9Profiling10setEnabledEb
#[doc(alias = "RBX::Profiling::setEnabled(bool)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35dbc0 as stub_35dbc0;

// 0x35dbd0 — __ZN3RBX9Profiling9isEnabledEv
#[doc(alias = "RBX::Profiling::isEnabled(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35dbd0 as stub_35dbd0;

// 0x35dbf8 — __ZN3RBX9Profiling8ProfilerC2EPKc
#[doc(alias = "RBX::Profiling::Profiler::Profiler(char const*)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35dbf8 as stub_35dbf8;

// 0x35dc78 — __ZN3RBX9Profiling12CodeProfilerC1EPKc
#[doc(alias = "RBX::Profiling::CodeProfiler::CodeProfiler(char const*)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35dc78 as stub_35dc78;

// 0x35dc9c — __ZN3RBX9Profiling12CodeProfiler3logEbd
#[doc(alias = "RBX::Profiling::CodeProfiler::log(bool,double)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35dc9c as stub_35dc9c;

// 0x35dd68 — __ZNK3RBX9Profiling8Profiler9getWindowEd
#[doc(alias = "RBX::Profiling::Profiler::getWindow(double)const")]
pub use rbx_core::generated_core_shard_aq::stub_0x35dd68 as stub_35dd68;

// 0x35de30 — __ZNK3RBX9Profiling8Profiler9getFramesEi
#[doc(alias = "RBX::Profiling::Profiler::getFrames(int)const")]
pub use rbx_core::generated_core_shard_aq::stub_0x35de30 as stub_35de30;

// 0x35ded0 — __ZNK3RBX9Profiling6Bucket12getActualFPSEv
#[doc(alias = "RBX::Profiling::Bucket::getActualFPS(void)const")]
pub use rbx_core::generated_core_shard_aq::stub_0x35ded0 as stub_35ded0;

// 0x35df00 — __ZNK3RBX9Profiling6Bucket13getNominalFPSEv
#[doc(alias = "RBX::Profiling::Bucket::getNominalFPS(void)const")]
pub use rbx_core::generated_core_shard_aq::stub_0x35df00 as stub_35df00;

// 0x35df30 — __ZNK3RBX9Profiling6Bucket21getNominalFramePeriodEv
#[doc(alias = "RBX::Profiling::Bucket::getNominalFramePeriod(void)const")]
pub use rbx_core::generated_core_shard_aq::stub_0x35df30 as stub_35df30;

// 0x35df60 — __ZN3RBX9Profiling4MarkC1ERNS0_12CodeProfilerEbb
#[doc(alias = "RBX::Profiling::Mark::Mark(RBX::Profiling::CodeProfiler &,bool,bool)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35df60 as stub_35df60;

// 0x35df64 — __ZN3RBX9Profiling4MarkC2ERNS0_12CodeProfilerEbb
#[doc(alias = "RBX::Profiling::Mark::Mark(RBX::Profiling::CodeProfiler &,bool,bool)")]
pub use rbx_core::generated_core_shard_aq::stub_0x35df64 as stub_35df64;

// 0x35dfcc — __ZN3RBX9Profiling4MarkD1Ev
#[doc(alias = "RBX::Profiling::Mark::~Mark()")]
pub use rbx_reflection::generated_refl_wdcron_A::stub_0x35dfcc as stub_35dfcc;

// 0x35dfd0 — __ZN3RBX9Profiling4MarkD2Ev
#[doc(alias = "RBX::Profiling::Mark::~Mark()")]
pub use rbx_reflection::generated_refl_wdcron_A::stub_0x35dfd0 as stub_35dfd0;

// 0x35e03c — __ZN3RBX9Profiling8ProfilerD1Ev
#[doc(alias = "RBX::Profiling::Profiler::~Profiler()")]
pub use rbx_reflection::generated_refl_wdcron_A::stub_0x35e03c as stub_35e03c;

// 0x35e060 — __ZN3RBX9Profiling8ProfilerD0Ev
#[doc(alias = "RBX::Profiling::Profiler::~Profiler()")]
pub use rbx_reflection::generated_refl_wdcron_A::stub_0x35e060 as stub_35e060;

#[cfg(test)]
mod shard_278_native_tests {
    use super::*;

    #[test]
    fn line_d1_is_noop_and_d0_reclaims() {
        let line = G3dLine::default();
        stub_35a0f4(&line);
        let boxed = Box::new(G3dLine::default());
        stub_35a448(Box::into_raw(boxed));
    }

    #[test]
    fn md5_d1_destroys_digest_and_d0_reclaims() {
        let mut hasher = Md5HasherImpl { digest: "d41d8cd98f00b204e9800998ecf8427e".to_string() };
        stub_35a6f4(&mut hasher);
        assert!(hasher.digest.is_empty());
        let boxed = Box::new(Md5HasherImpl { digest: "abc".to_string() });
        stub_35a718(Box::into_raw(boxed));
    }

    #[test]
    fn namemap_d2_clears_entries_and_d1_delegates() {
        let name = Name::default();
        let mut map = NameMap::default();
        map.entries.insert("foo".to_string(), &name);
        map.entries.insert("bar".to_string(), &name);
        stub_35bfe8(&mut map);
        assert!(map.entries.is_empty());
        map.entries.insert("baz".to_string(), &name);
        stub_35bfec(&mut map);
        assert!(map.entries.is_empty());
    }

    #[test]
    fn name_vec_d1_frees_buffer() {
        let name = Name::default();
        stub_35c4b8(vec![&name as *const Name, std::ptr::null()]);
        stub_35c4b8(Vec::new());
    }
}
