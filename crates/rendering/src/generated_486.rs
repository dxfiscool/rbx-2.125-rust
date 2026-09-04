//! rendering shard 486 — 100 stubs EA-sorted asc rendering-filter not in /tmp/global_eas.txt (0xc3cf50..0xc41ee0, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) rendering namespace filter (Ogre|G3D|GLES|ViewRbxGfx|RBX+Render), global EA dedup.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xc3cf50 — __ZNK3G3D15CoordinateFrame4lerpERKS0_f
// type: _DWORD __fastcall(G3D::CoordinateFrame *__hidden this, const G3D::CoordinateFrame *, G3D::Matrix3 *)
#[doc(alias = "G3D::CoordinateFrame::lerp(G3D::CoordinateFrame const&,float)const")]
// was: __ZNK3G3D15CoordinateFrame4lerpERKS0_f
// IDA 0xc3cf50: 88 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3cf50() {
}


// 0xc3d0d8 — __ZN3G3D9_internal18_handleErrorCheck_EPKcRKSsS2_ib
// type: _DWORD __fastcall(G3D::_internal *__hidden this, const char *, const std::string *, const char *, int, bool)
#[doc(alias = "G3D::_internal::_handleErrorCheck_(char const*,std::string const&,char const*,int,bool)")]
// was: __ZN3G3D9_internal18_handleErrorCheck_EPKcRKSsS2_ib
// IDA 0xc3d0d8: 766 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3d0d8() {
}


// 0xc3d9a8 — __ZN3G3D9_internalL18createErrorMessageEPKcRKSsS2_iRSsS5_
// type: _DWORD __fastcall(G3D::_internal *__hidden this, const char *, const std::string *, const char *, std::string *, std::string *, std::string *)
#[doc(alias = "G3D::_internal::createErrorMessage(char const*,std::string const&,char const*,int,std::string &,std::string &)")]
// was: __ZN3G3D9_internalL18createErrorMessageEPKcRKSsS2_iRSsS5_
// IDA 0xc3d9a8: 167 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3d9a8() {
}


// 0xc3db90 — __ZN3G3D9_internal18_releaseInputGrab_Ev
// type: _DWORD __fastcall(G3D::_internal *__hidden this)
#[doc(alias = "G3D::_internal::_releaseInputGrab_(void)")]
// was: __ZN3G3D9_internal18_releaseInputGrab_Ev
// IDA 0xc3db90: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc3db90() {
}


// 0xc3db94 — __ZN3G3D9_internal18_restoreInputGrab_Ev
// type: _DWORD __fastcall(G3D::_internal *__hidden this)
#[doc(alias = "G3D::_internal::_restoreInputGrab_(void)")]
// was: __ZN3G3D9_internal18_restoreInputGrab_Ev
// IDA 0xc3db94: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_0xc3db94() {
}


// 0xc3db98 — __ZN3G3D6formatEPKcz
// type: _DWORD(G3D *__hidden this, const char *, ...)
#[doc(alias = "G3D::format(char const*,...)")]
// was: __ZN3G3D6formatEPKcz
// IDA 0xc3db98: 13 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3db98() {
}


// 0xc3dbb8 — __ZN3G3D7vformatEPKcPv
// type: _DWORD __fastcall(G3D *__hidden this, const char *__format, void *)
#[doc(alias = "G3D::vformat(char const*,void *)")]
// was: __ZN3G3D7vformatEPKcPv
// IDA 0xc3dbb8: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3dbb8() {
}


// 0xc3dd58 — __ZN3G3D3infEv
// type: _DWORD __fastcall(G3D *__hidden this)
#[doc(alias = "G3D::inf(void)")]
// was: __ZN3G3D3infEv
// IDA 0xc3dd58: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3dd58() {
}


// 0xc3dd70 — __ZN3G3D5isNaNEf
// type: _DWORD __fastcall(G3D *__hidden this, float)
#[doc(alias = "G3D::isNaN(float)")]
// was: __ZN3G3D5isNaNEf
// IDA 0xc3dd70: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3dd70() {
}


// 0xc3ddb8 — __ZN3G3D4finfEv
// type: _DWORD __fastcall(G3D *__hidden this)
#[doc(alias = "G3D::finf(void)")]
// was: __ZN3G3D4finfEv
// IDA 0xc3ddb8: 2 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3ddb8() {
}


// 0xc3ddc0 — __ZN3G3D7iRandomEii
// type: _DWORD __fastcall(G3D *__hidden this, int, int)
#[doc(alias = "G3D::iRandom(int,int)")]
// was: __ZN3G3D7iRandomEii
// IDA 0xc3ddc0: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3ddc0() {
}


// 0xc3de20 — __ZN3G3D6GImage13resolveFormatERKSsPKhiNS0_6FormatE
#[doc(alias = "G3D::GImage::resolveFormat(std::string const&,unsigned char const*,int,G3D::GImage::Format)")]
// was: __ZN3G3D6GImage13resolveFormatERKSsPKhiNS0_6FormatE
// IDA 0xc3de20: 337 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3de20() {
}


// 0xc3e1b0 — __ZN3G3D6GImage14stringToFormatERKSs
// type: _DWORD __fastcall(G3D::GImage *__hidden this, const std::string *)
#[doc(alias = "G3D::GImage::stringToFormat(std::string const&)")]
// was: __ZN3G3D6GImage14stringToFormatERKSs
// IDA 0xc3e1b0: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e1b0() {
}


// 0xc3e3a8 — __ZN3G3D6GLight11directionalERKNS_7Vector3ERKNS_6Color3Ebb
// type: _DWORD __fastcall(G3D::GLight *__hidden this, const Vector3 *, const G3D::Color3 *, bool, bool)
#[doc(alias = "G3D::GLight::directional(G3D::Vector3 const&,G3D::Color3 const&,bool,bool)")]
// was: __ZN3G3D6GLight11directionalERKNS_7Vector3ERKNS_6Color3Ebb
// IDA 0xc3e3a8: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e3a8() {
}


// 0xc3e498 — __ZNK3G3D4Line12intersectionERKNS_5PlaneE
// type: _DWORD __fastcall(G3D::Line *__hidden this, const G3D::Plane *)
#[doc(alias = "G3D::Line::intersection(G3D::Plane const&)const")]
// was: __ZNK3G3D4Line12intersectionERKNS_5PlaneE
// IDA 0xc3e498: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e498() {
}


// 0xc3e578 — __ZNK3G3D4Line12closestPointERKNS_7Vector3E
// type: _DWORD __fastcall(G3D::Line *__hidden this, const Vector3 *)
#[doc(alias = "G3D::Line::closestPoint(G3D::Vector3 const&)const")]
// was: __ZNK3G3D4Line12closestPointERKNS_7Vector3E
// IDA 0xc3e578: 28 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e578() {
}


// 0xc3e5e8 — __ZNK3G3D4Line5pointEv
// type: _DWORD __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::point(void)const")]
// was: __ZNK3G3D4Line5pointEv
// IDA 0xc3e5e8: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e5e8() {
}


// 0xc3e5f8 — __ZNK3G3D4Line9directionEv
// type: _DWORD __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::direction(void)const")]
// was: __ZNK3G3D4Line9directionEv
// IDA 0xc3e5f8: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e5f8() {
}


// 0xc3e640 — __ZNK3G3D11LineSegment12closestPointERKNS_7Vector3E
// type: _DWORD __fastcall(G3D::LineSegment *__hidden this, const Vector3 *)
#[doc(alias = "G3D::LineSegment::closestPoint(G3D::Vector3 const&)const")]
// was: __ZNK3G3D11LineSegment12closestPointERKNS_7Vector3E
// IDA 0xc3e640: 75 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e640() {
}


// 0xc3e78c — __ZN3G3D3Log6commonEv
// type: _DWORD __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::common(void)")]
// was: __ZN3G3D3Log6commonEv
// IDA 0xc3e78c: 121 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e78c() {
}


// 0xc3e8f8 — __ZN3G3D3LogC2ERKSsi
// type: _DWORD __fastcall(G3D::Log *__hidden this, const std::string *, int)
#[doc(alias = "G3D::Log::Log(std::string const&,int)")]
// was: __ZN3G3D3LogC2ERKSsi
// IDA 0xc3e8f8: 140 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3e8f8() {
}


// 0xc3ea88 — __ZN3G3D3LogD0Ev
// type: void __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::~Log()")]
// was: __ZN3G3D3LogD0Ev
// IDA 0xc3ea88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc3ea88() {
}


// 0xc3eb28 — __ZN3G3D3LogD1Ev
// type: void __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::~Log()")]
// was: __ZN3G3D3LogD1Ev
// IDA 0xc3eb28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc3eb28() {
}


// 0xc3eb34 — __ZN3G3D3LogD2Ev
// type: void __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::~Log()")]
// was: __ZN3G3D3LogD2Ev
// IDA 0xc3eb34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_0xc3eb34() {
}


// 0xc3eda8 — __ZN3G3D3Log7printlnERKSs
// type: _DWORD __fastcall(G3D::Log *__hidden this, const std::string *)
#[doc(alias = "G3D::Log::println(std::string const&)")]
// was: __ZN3G3D3Log7printlnERKSs
// IDA 0xc3eda8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3eda8() {
}


// 0xc3edd8 — __ZN3G3D3Log20getCommonLogFilenameEv
// type: _DWORD __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::getCommonLogFilename(void)")]
// was: __ZN3G3D3Log20getCommonLogFilenameEv
// IDA 0xc3edd8: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3edd8() {
}


// 0xc3edf0 — __ZN3G3D3Log11printHeaderEv
// type: _DWORD __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::printHeader(void)")]
// was: __ZN3G3D3Log11printHeaderEv
// IDA 0xc3edf0: 119 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3edf0() {
}


// 0xc3ef4c — __ZN3G3D3Log5printERKSs
// type: _DWORD __fastcall(G3D::Log *__hidden this, const std::string *)
#[doc(alias = "G3D::Log::print(std::string const&)")]
// was: __ZN3G3D3Log5printERKSs
// IDA 0xc3ef4c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3ef4c() {
}


// 0xc3ef78 — __ZN3G3D7Matrix34zeroEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::zero(void)")]
// was: __ZN3G3D7Matrix34zeroEv
// IDA 0xc3ef78: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3ef78() {
}


// 0xc3efbc — __ZN3G3D7Matrix3C1Efffffffff
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float, float, float, float, float, float, float, float, float)
#[doc(alias = "G3D::Matrix3::Matrix3(float,float,float,float,float,float,float,float,float)")]
// was: __ZN3G3D7Matrix3C1Efffffffff
// IDA 0xc3efbc: 15 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3efbc() {
}


// 0xc3eff8 — __ZN3G3D7Matrix38identityEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::identity(void)")]
// was: __ZN3G3D7Matrix38identityEv
// IDA 0xc3eff8: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3eff8() {
}


// 0xc3f040 — __ZNK3G3D7Matrix36columnEi
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, int)
#[doc(alias = "G3D::Matrix3::column(int)const")]
// was: __ZNK3G3D7Matrix36columnEi
// IDA 0xc3f040: 9 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f040() {
}


// 0xc3f068 — __ZNK3G3D7Matrix313isOrthonormalEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::isOrthonormal(void)const")]
// was: __ZNK3G3D7Matrix313isOrthonormalEv
// IDA 0xc3f068: 197 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f068() {
}


// 0xc3f348 — __ZN3G3D7Matrix3C1ERKNS_4QuatE
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const G3D::Quat *)
#[doc(alias = "G3D::Matrix3::Matrix3(G3D::Quat const&)")]
// was: __ZN3G3D7Matrix3C1ERKNS_4QuatE
// IDA 0xc3f348: 57 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f348() {
}


// 0xc3f428 — __ZN3G3D7Matrix33setEfffffffff
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float, float, float, float, float, float, float, float, float)
#[doc(alias = "G3D::Matrix3::set(float,float,float,float,float,float,float,float,float)")]
// was: __ZN3G3D7Matrix33setEfffffffff
// IDA 0xc3f428: 15 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f428() {
}


// 0xc3f464 — __ZN3G3D7Matrix3C1ERKS0_
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "G3D::Matrix3::Matrix3(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3C1ERKS0_
// IDA 0xc3f464: 9 insns (ADD.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f464() {
}


// 0xc3f488 — __ZN3G3D7Matrix39setColumnEiRKNS_7Vector3E
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, int, const Vector3 *)
#[doc(alias = "G3D::Matrix3::setColumn(int,G3D::Vector3 const&)")]
// was: __ZN3G3D7Matrix39setColumnEiRKNS_7Vector3E
// IDA 0xc3f488: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f488() {
}


// 0xc3f49c — __ZNK3G3D7Matrix3eqERKS0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Matrix3::operator==(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3eqERKS0_
// IDA 0xc3f49c: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f49c() {
}


// 0xc3f4ec — __ZNK3G3D7Matrix3neERKS0_
#[doc(alias = "G3D::Matrix3::operator!=(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3neERKS0_
// IDA 0xc3f4ec: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f4ec() {
}


// 0xc3f53c — __ZNK3G3D7Matrix3plERKS0_
#[doc(alias = "G3D::Matrix3::operator+(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3plERKS0_
// IDA 0xc3f53c: 38 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f53c() {
}


// 0xc3f5d4 — __ZNK3G3D7Matrix3mlERKS0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Matrix3::operator*(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3mlERKS0_
// IDA 0xc3f5d4: 86 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f5d4() {
}


// 0xc3f720 — __ZN3G3D7Matrix3pLERKS0_
#[doc(alias = "G3D::Matrix3::operator+=(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3pLERKS0_
// IDA 0xc3f720: 38 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f720() {
}


// 0xc3f7b8 — __ZN3G3D7Matrix3mIERKS0_
#[doc(alias = "G3D::Matrix3::operator-=(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3mIERKS0_
// IDA 0xc3f7b8: 38 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f7b8() {
}


// 0xc3f850 — __ZN3G3D7Matrix3mLERKS0_
#[doc(alias = "G3D::Matrix3::operator*=(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3mLERKS0_
// IDA 0xc3f850: 95 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f850() {
}


// 0xc3f9b8 — __ZNK3G3D7Matrix3ngEv
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Matrix3::operator-(void)const")]
// was: __ZNK3G3D7Matrix3ngEv
// IDA 0xc3f9b8: 29 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3f9b8() {
}


// 0xc3fa30 — __ZN3G3DmlEfRKNS_7Matrix3E
#[doc(alias = "G3D::operator*(float,G3D::Matrix3 const&)")]
// was: __ZN3G3DmlEfRKNS_7Matrix3E
// IDA 0xc3fa30: 30 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3fa30() {
}


// 0xc3faa8 — __ZNK3G3D7Matrix39transposeEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::transpose(void)const")]
// was: __ZNK3G3D7Matrix39transposeEv
// IDA 0xc3faa8: 19 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3faa8() {
}


// 0xc3fad0 — __ZNK3G3D7Matrix37inverseERS0_f
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, G3D::Matrix3 *, float)
#[doc(alias = "G3D::Matrix3::inverse(G3D::Matrix3&,float)const")]
// was: __ZNK3G3D7Matrix37inverseERS0_f
// IDA 0xc3fad0: 120 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3fad0() {
}


// 0xc3fc9c — __ZNK3G3D7Matrix37inverseEf
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, G3D::Matrix3 *)
#[doc(alias = "G3D::Matrix3::inverse(float)const")]
// was: __ZNK3G3D7Matrix37inverseEf
// IDA 0xc3fc9c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3fc9c() {
}


// 0xc3fd1c — __ZN3G3D7Matrix314orthonormalizeEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::orthonormalize(void)")]
// was: __ZN3G3D7Matrix314orthonormalizeEv
// IDA 0xc3fd1c: 95 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3fd1c() {
}


// 0xc3fe98 — __ZNK3G3D7Matrix36l1NormEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::l1Norm(void)const")]
// was: __ZNK3G3D7Matrix36l1NormEv
// IDA 0xc3fe98: 51 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3fe98() {
}


// 0xc3ff60 — __ZNK3G3D7Matrix311toAxisAngleERNS_7Vector3ERf
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, G3D::Vector3 *, float *)
#[doc(alias = "G3D::Matrix3::toAxisAngle(G3D::Vector3 &,float &)const")]
// was: __ZNK3G3D7Matrix311toAxisAngleERNS_7Vector3ERf
// IDA 0xc3ff60: 137 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc3ff60() {
}


// 0xc4015c — __ZN3G3D7Matrix317fromAxisAngleFastERKNS_7Vector3Ef
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const Vector3 *, float)
#[doc(alias = "G3D::Matrix3::fromAxisAngleFast(G3D::Vector3 const&,float)")]
// was: __ZN3G3D7Matrix317fromAxisAngleFastERKNS_7Vector3Ef
// IDA 0xc4015c: 67 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc4015c() {
}


// 0xc40250 — __ZNK3G3D7Matrix316toEulerAnglesXYZERfS1_S1_
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float *, float *, float *)
#[doc(alias = "G3D::Matrix3::toEulerAnglesXYZ(float &,float &,float &)const")]
// was: __ZNK3G3D7Matrix316toEulerAnglesXYZERfS1_S1_
// IDA 0xc40250: 90 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40250() {
}


// 0xc403a8 — __ZN3G3D7Matrix318fromEulerAnglesXYZEfff
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float, float, float)
#[doc(alias = "G3D::Matrix3::fromEulerAnglesXYZ(float,float,float)")]
// was: __ZN3G3D7Matrix318fromEulerAnglesXYZEfff
// IDA 0xc403a8: 133 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc403a8() {
}


// 0xc40598 — __ZN3G3D7Matrix34_mulERKS0_S2_RS0_
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *, G3D::Matrix3 *)
#[doc(alias = "G3D::Matrix3::_mul(G3D::Matrix3 const&,G3D::Matrix3 const&,G3D::Matrix3&)")]
// was: __ZN3G3D7Matrix34_mulERKS0_S2_RS0_
// IDA 0xc40598: 110 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40598() {
}


// 0xc40784 — __ZN3G3D7Matrix48identityEv
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this)
#[doc(alias = "G3D::Matrix4::identity(void)")]
// was: __ZN3G3D7Matrix48identityEv
// IDA 0xc40784: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40784() {
}


// 0xc407d8 — __ZN3G3D7Matrix4C1ERKNS_15CoordinateFrameE
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "G3D::Matrix4::Matrix4(G3D::CoordinateFrame const&)")]
// was: __ZN3G3D7Matrix4C1ERKNS_15CoordinateFrameE
// IDA 0xc407d8: 31 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc407d8() {
}


// 0xc40818 — __ZNK3G3D7Matrix48upper3x3Ev
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this)
#[doc(alias = "G3D::Matrix4::upper3x3(void)const")]
// was: __ZNK3G3D7Matrix48upper3x3Ev
// IDA 0xc40818: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40818() {
}


// 0xc40860 — __ZN3G3D7Matrix4C1Ev
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this)
#[doc(alias = "G3D::Matrix4::Matrix4(void)")]
// was: __ZN3G3D7Matrix4C1Ev
// IDA 0xc40860: 9 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40860() {
}


// 0xc40884 — __ZNK3G3D7Matrix4mlERKNS_7Vector4E
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Matrix4::operator*(G3D::Vector4 const&)const")]
// was: __ZNK3G3D7Matrix4mlERKNS_7Vector4E
// IDA 0xc40884: 63 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40884() {
}


// 0xc409ac — __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_S3_
// type: _DWORD __fastcall(G3D::Plane *__hidden this, const Vector3 *, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Plane::Plane(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_S3_
// IDA 0xc409ac: 58 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc409ac() {
}


// 0xc40a94 — __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Plane *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Plane::Plane(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_
// IDA 0xc40a94: 37 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40a94() {
}


// 0xc40b20 — __ZN3G3D5Plane12fromEquationEffff
// type: _DWORD __fastcall(G3D::Plane *__hidden this, float, float, float, float)
#[doc(alias = "G3D::Plane::fromEquation(float,float,float,float)")]
// was: __ZN3G3D5Plane12fromEquationEffff
// IDA 0xc40b20: 29 insns (VMOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40b20() {
}


// 0xc40b8c — __ZNK3G3D5Plane11getEquationERNS_7Vector3ERf
// type: _DWORD __fastcall(G3D::Plane *__hidden this, Vector3 *, float *)
#[doc(alias = "G3D::Plane::getEquation(G3D::Vector3 &,float &)const")]
// was: __ZNK3G3D5Plane11getEquationERNS_7Vector3ERf
// IDA 0xc40b8c: 11 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40b8c() {
}


// 0xc40be4 — __ZN3G3D4QuatC1ERKNS_7Matrix3E
// type: _DWORD __fastcall(G3D::Quat *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "G3D::Quat::Quat(G3D::Matrix3 const&)")]
// was: __ZN3G3D4QuatC1ERKNS_7Matrix3E
// IDA 0xc40be4: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40be4() {
}


// 0xc40bf0 — __ZN3G3D4QuatC2ERKNS_7Matrix3E
// type: _DWORD __fastcall(G3D::Quat *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "G3D::Quat::Quat(G3D::Matrix3 const&)")]
// was: __ZN3G3D4QuatC2ERKNS_7Matrix3E
// IDA 0xc40bf0: 99 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40bf0() {
}


// 0xc40d70 — __ZNK3G3D4Quat16toRotationMatrixEv
// type: _DWORD __fastcall(G3D::Quat *__hidden this)
#[doc(alias = "G3D::Quat::toRotationMatrix(void)const")]
// was: __ZNK3G3D4Quat16toRotationMatrixEv
// IDA 0xc40d70: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40d70() {
}


// 0xc40db4 — __ZNK3G3D4Quat5slerpERKS0_ff
// type: _DWORD __fastcall(G3D::Quat *__hidden this, const G3D::Quat *, float, float)
#[doc(alias = "G3D::Quat::slerp(G3D::Quat const&,float,float)const")]
// was: __ZNK3G3D4Quat5slerpERKS0_ff
// IDA 0xc40db4: 140 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40db4() {
}


// 0xc40ffc — __ZN3G3D3Ray3setERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Ray *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "G3D::Ray::set(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: __ZN3G3D3Ray3setERKNS_7Vector3ES3_
// IDA 0xc40ffc: 212 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc40ffc() {
}


// 0xc412bc — __ZNK3G3D6Sphere8toStringEv
// type: _DWORD __fastcall(G3D::Sphere *__hidden this)
#[doc(alias = "G3D::Sphere::toString(void)const")]
// was: __ZNK3G3D6Sphere8toStringEv
// IDA 0xc412bc: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc412bc() {
}


// 0xc41304 — __ZNK3G3D6Sphere8containsERKNS_7Vector3E
// type: _DWORD __fastcall(G3D::Sphere *__hidden this, const Vector3 *)
#[doc(alias = "G3D::Sphere::contains(G3D::Vector3 const&)const")]
// was: __ZNK3G3D6Sphere8containsERKNS_7Vector3E
// IDA 0xc41304: 22 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41304() {
}


// 0xc41388 — __ZN3G3D10beginsWithERKSsS1_
#[doc(alias = "G3D::beginsWith(std::string const&,std::string const&)")]
// was: __ZN3G3D10beginsWithERKSsS1_
// IDA 0xc41388: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41388() {
}


// 0xc413c0 — __ZN3G3D8endsWithERKSsS1_
#[doc(alias = "G3D::endsWith(std::string const&,std::string const&)")]
// was: __ZN3G3D8endsWithERKSsS1_
// IDA 0xc413c0: 21 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc413c0() {
}


// 0xc413f8 — __ZN3G3D7toUpperERKSs
// type: _DWORD __fastcall(G3D *__hidden this, const std::string *)
#[doc(alias = "G3D::toUpper(std::string const&)")]
// was: __ZN3G3D7toUpperERKSs
// IDA 0xc413f8: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc413f8() {
}


// 0xc4158c — __ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEEC2Ev
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::Table(void)")]
// was: __ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEEC2Ev
// IDA 0xc4158c: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc4158c() {
}


// 0xc417bc — __ZN3G3D7Vector23oneEv
// type: _DWORD __fastcall(G3D::Vector2 *__hidden this)
#[doc(alias = "G3D::Vector2::one(void)")]
// was: __ZN3G3D7Vector23oneEv
// IDA 0xc417bc: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc417bc() {
}


// 0xc417f0 — __ZN3G3D7Vector24zeroEv
// type: _DWORD __fastcall(G3D::Vector2 *__hidden this)
#[doc(alias = "G3D::Vector2::zero(void)")]
// was: __ZN3G3D7Vector24zeroEv
// IDA 0xc417f0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc417f0() {
}


// 0xc41824 — __ZNK3G3D7Vector2dvEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Vector2::operator/(float)const")]
// was: __ZNK3G3D7Vector2dvEf
// IDA 0xc41824: 11 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41824() {
}


// 0xc41850 — __ZN3G3D7Vector2dVEf
#[doc(alias = "G3D::Vector2::operator/=(float)")]
// was: __ZN3G3D7Vector2dVEf
// IDA 0xc41850: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41850() {
}


// 0xc41870 — __ZNK3G3D7Vector22yxEv
// type: _DWORD __fastcall(G3D::Vector2 *__hidden this)
#[doc(alias = "G3D::Vector2::yx(void)const")]
// was: __ZNK3G3D7Vector22yxEv
// IDA 0xc41870: 6 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41870() {
}


// 0xc418bc — __ZN3G3D12Vector2int16C1ERKNS_7Vector2E
// type: _DWORD __fastcall(G3D::Vector2int16 *__hidden this, const G3D::Vector2 *)
#[doc(alias = "G3D::Vector2int16::Vector2int16(G3D::Vector2 const&)")]
// was: __ZN3G3D12Vector2int16C1ERKNS_7Vector2E
// IDA 0xc418bc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc418bc() {
}


// 0xc41918 — __ZN3G3D12Vector2int16C1EPi
// type: _DWORD __fastcall(G3D::Vector2int16 *__hidden this, int *)
#[doc(alias = "G3D::Vector2int16::Vector2int16(int *)")]
// was: __ZN3G3D12Vector2int16C1EPi
// IDA 0xc41918: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41918() {
}


// 0xc41924 — __ZNK3G3D12Vector2int162yxEv
// type: _DWORD __fastcall(G3D::Vector2int16 *__hidden this)
#[doc(alias = "G3D::Vector2int16::yx(void)const")]
// was: __ZNK3G3D12Vector2int162yxEv
// IDA 0xc41924: 5 insns (LDRH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41924() {
}


// 0xc41964 — __ZN3G3D7Vector3C1ERKNS_7Vector2Ef
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, const G3D::Vector2 *, float)
#[doc(alias = "G3D::Vector3::Vector3(G3D::Vector2 const&,float)")]
// was: __ZN3G3D7Vector3C1ERKNS_7Vector2Ef
// IDA 0xc41964: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41964() {
}


// 0xc41970 — __ZN3G3D7Vector34zeroEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::zero(void)")]
// was: __ZN3G3D7Vector34zeroEv
// IDA 0xc41970: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41970() {
}


// 0xc419a8 — __ZN3G3D7Vector33oneEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::one(void)")]
// was: __ZN3G3D7Vector33oneEv
// IDA 0xc419a8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc419a8() {
}


// 0xc419e0 — __ZN3G3D7Vector35unitXEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::unitX(void)")]
// was: __ZN3G3D7Vector35unitXEv
// IDA 0xc419e0: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc419e0() {
}


// 0xc41a1c — __ZN3G3D7Vector35unitYEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::unitY(void)")]
// was: __ZN3G3D7Vector35unitYEv
// IDA 0xc41a1c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41a1c() {
}


// 0xc41a58 — __ZN3G3D7Vector35unitZEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::unitZ(void)")]
// was: __ZN3G3D7Vector35unitZEv
// IDA 0xc41a58: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41a58() {
}


// 0xc41a94 — __ZN3G3D7Vector33infEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::inf(void)")]
// was: __ZN3G3D7Vector33infEv
// IDA 0xc41a94: 80 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41a94() {
}


// 0xc41b94 — __ZN3G3D7Vector39minFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::minFinite(void)")]
// was: __ZN3G3D7Vector39minFiniteEv
// IDA 0xc41b94: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41b94() {
}


// 0xc41bd4 — __ZN3G3D7Vector39maxFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::maxFinite(void)")]
// was: __ZN3G3D7Vector39maxFiniteEv
// IDA 0xc41bd4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41bd4() {
}


// 0xc41c18 — __ZNK3G3D7Vector311primaryAxisEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::primaryAxis(void)const")]
// was: __ZNK3G3D7Vector311primaryAxisEv
// IDA 0xc41c18: 20 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41c18() {
}


// 0xc41c5c — __ZN3G3D7Vector3C1ERKNS_12Vector3int16E
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, const G3D::Vector3int16 *)
#[doc(alias = "G3D::Vector3::Vector3(G3D::Vector3int16 const&)")]
// was: __ZN3G3D7Vector3C1ERKNS_12Vector3int16E
// IDA 0xc41c5c: 14 insns (LDRSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41c5c() {
}


// 0xc41c98 — __ZN3G3D7Vector36randomERNS_6RandomE
#[doc(alias = "G3D::Vector3::random(G3D::Random &)")]
// was: __ZN3G3D7Vector36randomERNS_6RandomE
// IDA 0xc41c98: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41c98() {
}


// 0xc41cc0 — __ZN3G3D7Vector37unitizeEf
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, float)
#[doc(alias = "G3D::Vector3::unitize(float)")]
// was: __ZN3G3D7Vector37unitizeEf
// IDA 0xc41cc0: 26 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41cc0() {
}


// 0xc41d24 — __ZN3G3D7Vector324generateOrthonormalBasisERS0_S1_S1_b
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, G3D::Vector3 *, G3D::Vector3 *, G3D::Vector3 *, bool)
#[doc(alias = "G3D::Vector3::generateOrthonormalBasis(G3D::Vector3&,G3D::Vector3&,G3D::Vector3&,bool)")]
// was: __ZN3G3D7Vector324generateOrthonormalBasisERS0_S1_S1_b
// IDA 0xc41d24: 90 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41d24() {
}


// 0xc41e7c — __ZNK3G3D7Vector314toVector3int16Ev
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::toVector3int16(void)const")]
// was: __ZNK3G3D7Vector314toVector3int16Ev
// IDA 0xc41e7c: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41e7c() {
}


// 0xc41ee0 — __ZNK3G3D7Vector32xyEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::xy(void)const")]
// was: __ZNK3G3D7Vector32xyEv
// IDA 0xc41ee0: 3 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_0xc41ee0() {
}

