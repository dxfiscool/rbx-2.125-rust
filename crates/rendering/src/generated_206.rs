//! rendering — generated_206 — 100 stubs EA-sorted asc filtered Ogre|G3D|Rendering|Adorn 51f40..3a94e0
//! Filter: Ogre|G3D|Rendering|Adorn remaining 12268 prior -> uses global gap filler EA-sorted asc (global unstub 51004 before)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x51f40 — -[MainViewController getOgreWindow]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreWindow]")]
// IDA 0x51f40: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_0x51f40() {
}

// 0x51f50 — -[MainViewController setOgreWindow:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreWindow:]")]
// IDA 0x51f50: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_0x51f50() {
}

// 0x51f60 — -[MainViewController getOgreView]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreView]")]
// IDA 0x51f60: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_0x51f60() {
}

// 0x51f70 — -[MainViewController setOgreView:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreView:]")]
// IDA 0x51f70: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_0x51f70() {
}

// 0x51fa0 — -[MainViewController getOgreViewController]
// type: id __cdecl(MainViewController *self, SEL)
#[doc(alias = "-[MainViewController getOgreViewController]")]
// IDA 0x51fa0: ObjC getter (LDR ivar, returns id); field recovery pending — no-op.
pub fn stub_0x51fa0() {
}

// 0x51fb0 — -[MainViewController setOgreViewController:]
// type: void __cdecl(MainViewController *self, SEL, id)
#[doc(alias = "-[MainViewController setOgreViewController:]")]
// IDA 0x51fb0: ObjC setter (STR ivar); field recovery pending — no-op.
pub fn stub_0x51fb0() {
}

// 0x25b4e0 — __ZN3RBX5Light8setColorEN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "RBX::Light::setColor(G3D::Color3)")]
// IDA 0x25b4e0: 30 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x25b4e0() {
}

// 0x356ae0 — __ZN3RBX4Math13getFocusSpaceERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::getFocusSpace(G3D::CoordinateFrame const&)")]
// IDA 0x356ae0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356ae0() {
}

// 0x356b18 — __ZN3RBX4Math19getHeadingElevationERKN3G3D15CoordinateFrameERfS5_
// type: double __fastcall(RBX::Math *this, const G3D::CoordinateFrame *, float *, float *)
#[doc(alias = "RBX::Math::getHeadingElevation(G3D::CoordinateFrame const&,float &,float &)")]
// IDA 0x356b18: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356b18() {
}

// 0x356b84 — __ZN3RBX4Math19setHeadingElevationERN3G3D15CoordinateFrameEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::CoordinateFrame *, float, float)
#[doc(alias = "RBX::Math::setHeadingElevation(G3D::CoordinateFrame &,float,float)")]
// IDA 0x356b84: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356b84() {
}

// 0x356c3c — __ZN3RBX4Math8lessThanERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "RBX::Math::lessThan(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x356c3c: 21 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356c3c() {
}

// 0x356cc8 — __ZN3RBX4Math15isNanInfVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfVector3(G3D::Vector3 const&)")]
// IDA 0x356cc8: 35 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356cc8() {
}

// 0x356d38 — __ZN3RBX4Math21isNanInfDenormVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfDenormVector3(G3D::Vector3 const&)")]
// IDA 0x356d38: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356d38() {
}

// 0x356d70 — __ZN3RBX4Math11hasNanOrInfERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::hasNanOrInf(G3D::CoordinateFrame const&)")]
// IDA 0x356d70: 43 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356d70() {
}

// 0x356df4 — __ZN3RBX4Math9fixDenormERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, Vector3 *)
#[doc(alias = "RBX::Math::fixDenorm(G3D::Vector3 &)")]
// IDA 0x356df4: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x356df4() {
}

// 0x35711c — __ZN3RBX4Math16getIWorldAtPointERKN3G3D7Vector3ES4_RKNS1_7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::getIWorldAtPoint(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x35711c: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35711c() {
}

// 0x3571c0 — __ZN3RBX4Math15getIBodyAtPointERKN3G3D7Vector3ERKNS1_7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::getIBodyAtPoint(G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x3571c0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3571c0() {
}

// 0x357250 — __ZN3RBX4Math19momentToObjectSpaceERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::momentToObjectSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
// IDA 0x357250: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357250() {
}

// 0x35728c — __ZN3RBX4Math18momentToWorldSpaceERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::momentToWorldSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
// IDA 0x35728c: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35728c() {
}

// 0x3572c4 — __ZN3RBX4Math10toDiagonalERKN3G3D7Matrix3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "RBX::Math::toDiagonal(G3D::Matrix3 const&)")]
// IDA 0x3572c4: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3572c4() {
}

// 0x3572e4 — __ZN3RBX4Math26fromVectorToVectorRotationERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromVectorToVectorRotation(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x3572e4: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3572e4() {
}

// 0x357450 — __ZN3RBX4Math24fromRotationAxisAndAngleERKN3G3D7Vector3ERKf
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const float *)
#[doc(alias = "RBX::Math::fromRotationAxisAndAngle(G3D::Vector3 const&,float const&)")]
// IDA 0x357450: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357450() {
}

// 0x3575bc — __ZN3RBX4Math25orthonormalizeIfNecessaryERN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::orthonormalizeIfNecessary(G3D::Matrix3 &)")]
// IDA 0x3575bc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3575bc() {
}

// 0x3575dc — __ZN3RBX4Math20fromDirectionCosinesERKN3G3D7Vector3ES4_S4_S4_S4_S4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromDirectionCosines(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x3575dc: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3575dc() {
}

// 0x357744 — __ZN3RBX4Math13isAxisAlignedERKN3G3D7Matrix3E
// type: int __fastcall(RBX::Math *this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::isAxisAligned(G3D::Matrix3 const&)")]
// IDA 0x357744: 85 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357744() {
}

// 0x35781c — __ZN3RBX4Math11getOrientIdERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::getOrientId(G3D::Matrix3 const&)")]
// IDA 0x35781c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35781c() {
}

// 0x357858 — __ZN3RBX4Math11idToMatrix3EiRN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, int, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::idToMatrix3(int,G3D::Matrix3 &)")]
// IDA 0x357858: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357858() {
}

// 0x357924 — __ZN3RBX4Math12rotateAboutZERKN3G3D7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::rotateAboutZ(G3D::Matrix3 const&,float)")]
// IDA 0x357924: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357924() {
}

// 0x3579d0 — __ZN3RBX4Math10snapToAxesERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::snapToAxes(G3D::Matrix3 const&)")]
// IDA 0x3579d0: 180 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3579d0() {
}

// 0x357c08 — __ZN3RBX4Math6toGridERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::toGrid(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x357c08: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357c08() {
}

// 0x357c84 — __ZN3RBX4Math13iRoundVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::iRoundVector3(G3D::Vector3 const&)")]
// IDA 0x357c84: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357c84() {
}

// 0x357ce4 — __ZN3RBX4Math6toGridERKN3G3D7Vector3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Math::toGrid(G3D::Vector3 const&,float)")]
// IDA 0x357ce4: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357ce4() {
}

// 0x357cfc — __ZN3RBX4Math10snapToGridERKN3G3D15CoordinateFrameEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *, float)
#[doc(alias = "RBX::Math::snapToGrid(G3D::CoordinateFrame const&,float)")]
// IDA 0x357cfc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357cfc() {
}

// 0x357d44 — __ZN3RBX4Math10snapToGridERKN3G3D15CoordinateFrameERKNS1_7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::snapToGrid(G3D::CoordinateFrame const&,G3D::Vector3 const&)")]
// IDA 0x357d44: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357d44() {
}

// 0x357d88 — __ZN3RBX4Math13safeDirectionERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::safeDirection(G3D::Vector3 const&)")]
// IDA 0x357d88: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357d88() {
}

// 0x357ee4 — __ZN3RBX4Math5angleERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "RBX::Math::angle(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x357ee4: 25 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357ee4() {
}

// 0x357f48 — __ZN3RBX4Math14elevationAngleERKN3G3D7Vector3E
// type: int __fastcall(RBX::Math *this, const Vector3 *)
#[doc(alias = "RBX::Math::elevationAngle(G3D::Vector3 const&)")]
// IDA 0x357f48: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357f48() {
}

// 0x357fa0 — __ZN3RBX4Math16fuzzyAxisAlignedERKN3G3D7Matrix3ES4_f
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::fuzzyAxisAligned(G3D::Matrix3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x357fa0: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x357fa0() {
}

// 0x3580b4 — __ZN3RBX4Math13isOrthonormalERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::isOrthonormal(G3D::Matrix3 const&)")]
// IDA 0x3580b4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3580b4() {
}

// 0x3580c0 — __ZN3RBX4Math7fuzzyEqERKN3G3D7Vector3ES4_f
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *, float)
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// IDA 0x3580c0: 25 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3580c0() {
}

// 0x35810c — __ZN3RBX4Math7fuzzyEqERKN3G3D7Matrix3ES4_f
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Matrix3 const&,G3D::Matrix3 const&,float)")]
// IDA 0x35810c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35810c() {
}

// 0x35817c — __ZN3RBX4Math7fuzzyEqERKN3G3D7Matrix4ES4_f
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix4 *, const G3D::Matrix4 *, float)
#[doc(alias = "RBX::Math::fuzzyEq(G3D::Matrix4 const&,G3D::Matrix4 const&,float)")]
// IDA 0x35817c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35817c() {
}

// 0x3581ec — __ZN3RBX4Math7fuzzyEqERKN3G3D15CoordinateFrameES4_ff
#[doc(alias = "RBX::Math::fuzzyEq(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float)")]
// IDA 0x3581ec: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3581ec() {
}

// 0x358254 — __ZN3RBX4Math18rotateAboutYGlobalERN3G3D15CoordinateFrameEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::CoordinateFrame *, float)
#[doc(alias = "RBX::Math::rotateAboutYGlobal(G3D::CoordinateFrame &,float)")]
// IDA 0x358254: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358254() {
}

// 0x35829c — __ZN3RBX4Math18rotateAboutYGlobalERKN3G3D7Vector3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Math::rotateAboutYGlobal(G3D::Vector3 const&,float)")]
// IDA 0x35829c: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35829c() {
}

// 0x358314 — __ZN3RBX4Math24getClosestObjectNormalIdERKN3G3D7Vector3ERKNS1_7Matrix3E
#[doc(alias = "RBX::Math::getClosestObjectNormalId(G3D::Vector3 const&,G3D::Matrix3 const&)")]
// IDA 0x358314: 58 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358314() {
}

// 0x3583cc — __ZN3RBX4Math11sortVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::sortVector3(G3D::Vector3 const&)")]
// IDA 0x3583cc: 28 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3583cc() {
}

// 0x358430 — __ZN3RBX4Math10vector3AbsERKN3G3D7Vector3E
// type: _DWORD *__fastcall(_DWORD *this, const Vector3 *)
#[doc(alias = "RBX::Math::vector3Abs(G3D::Vector3 const&)")]
// IDA 0x358430: 11 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358430() {
}

// 0x358aa4 — __ZN3RBX4Math15toYAxisQuadrantERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::toYAxisQuadrant(G3D::CoordinateFrame const&)")]
// IDA 0x358aa4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358aa4() {
}

// 0x358ae4 — __ZN3RBX4Math25intersectRayConvexPolygonERKNS_6RbxRayERKSt6vectorIN3G3D7Vector3ESaIS6_EERS6_b
// type: int __fastcall(int, int, G3D::Plane *)
#[doc(alias = "RBX::Math::intersectRayConvexPolygon(RBX::RbxRay const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,G3D::Vector3&,bool)")]
// IDA 0x358ae4: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358ae4() {
}

// 0x358d38 — __ZN3RBX4Math17intersectRayPlaneERKNS_6RbxRayERKN3G3D5PlaneERNS4_7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const RBX::RbxRay *, const G3D::Plane *, G3D::Vector3 *)
#[doc(alias = "RBX::Math::intersectRayPlane(RBX::RbxRay const&,G3D::Plane const&,G3D::Vector3 &)")]
// IDA 0x358d38: 117 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358d38() {
}

// 0x358ea0 — __ZN3RBX4Math26spatialPolygonIntersectionERKSt6vectorIN3G3D7Vector3ESaIS3_EES7_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Math::spatialPolygonIntersection(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
// IDA 0x358ea0: 493 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x358ea0() {
}

// 0x3594fc — __ZN3RBX4Math25planarPolygonIntersectionERKSt6vectorIN3G3D7Vector2ESaIS3_EES7_
// type: void __fastcall(_DWORD *, __int64 *, _DWORD *)
#[doc(alias = "RBX::Math::planarPolygonIntersection(std::vector<G3D::Vector2,std::allocator<G3D::Vector2>> const&,std::vector<G3D::Vector2,std::allocator<G3D::Vector2>> const&)")]
// IDA 0x3594fc: 181 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3594fc() {
}

// 0x3596f8 — __ZN3RBX4Math18intersectLinePlaneERKN3G3D4LineERKNS1_5PlaneERNS1_7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Line *, const G3D::Plane *, G3D::Vector3 *)
#[doc(alias = "RBX::Math::intersectLinePlane(G3D::Line const&,G3D::Plane const&,G3D::Vector3 &)")]
// IDA 0x3596f8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3596f8() {
}

// 0x3599b8 — __ZN3RBX4Math29lineSegmentDistanceIfCrossingERKN3G3D7Vector3ES4_S4_S4_Rff
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *, const Vector3 *, const Vector3 *, float *, float)
#[doc(alias = "RBX::Math::lineSegmentDistanceIfCrossing(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,float &,float)")]
// IDA 0x3599b8: 146 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3599b8() {
}

// 0x359d50 — __ZN3RBX4Math26getWellFormedRotForZVectorERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::getWellFormedRotForZVector(G3D::Vector3 const&)")]
// IDA 0x359d50: 106 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x359d50() {
}

// 0x359f38 — __ZNSt6vectorIN3G3D7Vector2ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>::push_back(G3D::Vector2 const&)")]
// IDA 0x359f38: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_0x359f38() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x359f64 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::push_back(G3D::Vector3 const&)")]
// IDA 0x359f64: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_0x359f64() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x359f98 — __ZNK3G3D5Plane17halfSpaceContainsENS_7Vector3E
#[doc(alias = "G3D::Plane::halfSpaceContains(G3D::Vector3)const")]
// IDA 0x359f98: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x359f98() {
}

// 0x35a058 — __ZN3G3D4Line21fromPointAndDirectionERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Line *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Line::fromPointAndDirection(G3D::Vector3 const&,G3D::Vector3 const&)")]
// IDA 0x35a058: 46 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35a058() {
}

// 0x35a0f4 — __ZN3G3D4LineD1Ev
// type: void __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::~Line()")]
// IDA 0x35a0f4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0x35a0f4() {
}

// 0x35a0f8 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(unsigned int *, __int64 *, __int64 *)
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector3*,std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>>,G3D::Vector3 const&)")]
// IDA 0x35a0f8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0x35a0f8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x35a24c — __ZNSt12_Vector_baseIN3G3D7Vector3ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector3,std::allocator<G3D::Vector3>>::_M_allocate(unsigned long)")]
// IDA 0x35a24c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_0x35a24c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x35a270 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
// IDA 0x35a270: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_0x35a270() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x35a2d8 — __ZNSt6vectorIN3G3D7Vector2ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2*,std::vector<G3D::Vector2,std::allocator<G3D::Vector2>>>,G3D::Vector2 const&)")]
// IDA 0x35a2d8: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_0x35a2d8() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x35a3e4 — __ZNSt12_Vector_baseIN3G3D7Vector2ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector2,std::allocator<G3D::Vector2>>::_M_allocate(unsigned long)")]
// IDA 0x35a3e4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_0x35a3e4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x35a3fc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Vector2ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector2 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2 *,G3D::Vector2 *>(G3D::Vector2 *,G3D::Vector2 *,G3D::Vector2 *)")]
// IDA 0x35a3fc: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_0x35a3fc() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x35a448 — __ZN3G3D4LineD0Ev
// type: void __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::~Line()")]
// IDA 0x35a448: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x35a448() {
}

// 0x35cfa8 — __ZN3RBX11uvwToObjectERKN3G3D7Vector3ENS_8NormalIdE
#[doc(alias = "RBX::uvwToObject(G3D::Vector3 const&,RBX::NormalId)")]
// IDA 0x35cfa8: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35cfa8() {
}

// 0x35d0c8 — __ZN3RBX11objectToUvwERKN3G3D7Vector3ENS_8NormalIdE
#[doc(alias = "RBX::objectToUvw(G3D::Vector3 const&,RBX::NormalId)")]
// IDA 0x35d0c8: 81 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35d0c8() {
}

// 0x35d8a0 — __ZN3RBX17Vector3ToNormalIdERKN3G3D7Vector3E
// type: int __fastcall(RBX *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Vector3ToNormalId(G3D::Vector3 const&)")]
// IDA 0x35d8a0: 192 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35d8a0() {
}

// 0x35db38 — __ZN3RBX17Matrix3ToNormalIdERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Matrix3ToNormalId(G3D::Matrix3 const&)")]
// IDA 0x35db38: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x35db38() {
}

// 0x3602bc — __ZN3RBX10QuaternionC1ERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Quaternion *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Quaternion::Quaternion(G3D::Matrix3 const&)")]
// IDA 0x3602bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x3602bc() {
}

// 0x3602c0 — __ZN3RBX10QuaternionC2ERKN3G3D7Matrix3E
#[doc(alias = "RBX::Quaternion::Quaternion(G3D::Matrix3 const&)")]
// IDA 0x3602c0: 114 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3602c0() {
}

// 0x360478 — __ZNK3RBX10Quaternion16toRotationMatrixERN3G3D7Matrix3E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Quaternion::toRotationMatrix(G3D::Matrix3 &)const")]
// IDA 0x360478: 45 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x360478() {
}

// 0x373bf4 — __Z7convertRKN3G3D7Vector3ER11FMOD_VECTOR
// type: int __fastcall(RBX::Math **, _DWORD *)
#[doc(alias = "convert(G3D::Vector3 const&,FMOD_VECTOR &)")]
// IDA 0x373bf4: 62 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x373bf4() {
}

// 0x380f1c — __ZN3RBX12SpanningTree22findAndDeactivateEdgesEPNS_12SpanningNodeEPNS_12SpanningEdgeERN3G3D5ArrayIS4_Li10ELm32EEE
// type: RBX::SpanningEdge *__fastcall(RBX::SpanningEdge *result, int, RBX::SpanningEdge *, int)
#[doc(alias = "RBX::SpanningTree::findAndDeactivateEdges(RBX::SpanningNode *,RBX::SpanningEdge *,G3D::Array<RBX::SpanningEdge *,10,32ul> &)")]
// IDA 0x380f1c: 97 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x380f1c() {
}

// 0x38103c — __ZN3RBX12SpanningTree13activateEdgesEPNS_12SpanningNodeERKN3G3D5ArrayIPNS_12SpanningEdgeELi10ELm32EEE
// type: int __fastcall(void (__fastcall ***)(RBX::SpanningTree *, RBX::SpanningEdge *, int), RBX::SpanningNode *, _DWORD *)
#[doc(alias = "RBX::SpanningTree::activateEdges(RBX::SpanningNode *,G3D::Array<RBX::SpanningEdge *,10,32ul> const&)")]
// IDA 0x38103c: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38103c() {
}

// 0x3812ac — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6appendERKS3_
// type: int __fastcall(unsigned int *, _DWORD *)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::append(RBX::SpanningEdge * const&)")]
// IDA 0x3812ac: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3812ac() {
}

// 0x38147c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE6resizeEib
// type: int __fastcall(int result, int, int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::resize(int,bool)")]
// IDA 0x38147c: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38147c() {
}

// 0x381534 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EE7reallocEi
// type: void __fastcall(int, int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::realloc(int)")]
// IDA 0x381534: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x381534() {
}

// 0x38171c — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EED2Ev
// type: int __fastcall(int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::~Array()")]
// IDA 0x38171c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0x38171c() {
}

// 0x3817f0 — __ZN3G3D5ArrayIPN3RBX12SpanningEdgeELi10ELm32EEC2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::Array(void)")]
// IDA 0x3817f0: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3817f0() {
}

// 0x38c140 — __ZNK3RBX5UDim2mlEN3G3D7Vector2E
// type: _DWORD *__fastcall(_DWORD *result, int, __int32 *)
#[doc(alias = "RBX::UDim2::operator*(G3D::Vector2)const")]
// IDA 0x38c140: 18 insns (LDRSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38c140() {
}

// 0x38c434 — __ZN3RBX5Units20kmsAccelerationToRbxERKN3G3D7Vector3E
// type: _DWORD *__fastcall(_DWORD *this, const Vector3 *)
#[doc(alias = "RBX::Units::kmsAccelerationToRbx(G3D::Vector3 const&)")]
// IDA 0x38c434: 12 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38c434() {
}

// 0x38db20 — __ZN3RBX12Accoutrement18setAttachmentPointERKN3G3D15CoordinateFrameE
// type: int __fastcall(RBX::Accoutrement *this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Accoutrement::setAttachmentPoint(G3D::CoordinateFrame const&)")]
// IDA 0x38db20: 83 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38db20() {
}

// 0x38dc40 — __ZN3RBX12Accoutrement16setAttachmentPosERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentPos(G3D::Vector3 const&)")]
// IDA 0x38dc40: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38dc40() {
}

// 0x38dcb0 — __ZN3RBX12Accoutrement20setAttachmentForwardERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentForward(G3D::Vector3 const&)")]
// IDA 0x38dcb0: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38dcb0() {
}

// 0x38de0c — __ZN3RBX12Accoutrement15setAttachmentUpERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentUp(G3D::Vector3 const&)")]
// IDA 0x38de0c: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38de0c() {
}

// 0x38df40 — __ZN3RBX12Accoutrement18setAttachmentRightERKN3G3D7Vector3E
// type: int __fastcall(RBX::Accoutrement *this, const G3D::Vector3 *)
#[doc(alias = "RBX::Accoutrement::setAttachmentRight(G3D::Vector3 const&)")]
// IDA 0x38df40: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x38df40() {
}

// 0x394464 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
// IDA 0x394464: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x394464() {
}

// 0x394730 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_12sPVAdornmentEEE12getClassNameEv")]
// IDA 0x394730: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x394730() {
}

// 0x3949fc — __ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sPVAdornmentEEEEvv")]
// IDA 0x3949fc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_0x3949fc() {
}

// 0x394a00 — __ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v")]
// IDA 0x394a00: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x394a00() {
}

// 0x3a7f68 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
// IDA 0x3a7f68: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a7f68() {
}

// 0x3a80c8 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: void __fastcall(int, int)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
// IDA 0x3a80c8: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a80c8() {
}

// 0x3a82e8 — __ZN3rbx7signals16signal_with_argsILi3EFvN3G3D7Vector34AxisEffEEclES4_ff
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(G3D::Vector3::Axis,float,float)>::operator()(G3D::Vector3::Axis,float,float)")]
// IDA 0x3a82e8: 81 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a82e8() {
}

// 0x3a8440 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector34AxisEEEclES4_
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector3::Axis)>::operator()(G3D::Vector3::Axis)")]
// IDA 0x3a8440: 76 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a8440() {
}

// 0x3a9380 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// IDA 0x3a9380: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a9380() {
}

// 0x3a94e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")]
// IDA 0x3a94e0: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0x3a94e0() {
}

