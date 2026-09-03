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
pub fn stub_0xc3cf50() -> ! {
    todo!("0xc3cf50 G3D::CoordinateFrame::lerp(G3D::CoordinateFrame const&,float)const")
}


// 0xc3d0d8 — __ZN3G3D9_internal18_handleErrorCheck_EPKcRKSsS2_ib
// type: _DWORD __fastcall(G3D::_internal *__hidden this, const char *, const std::string *, const char *, int, bool)
#[doc(alias = "G3D::_internal::_handleErrorCheck_(char const*,std::string const&,char const*,int,bool)")]
// was: __ZN3G3D9_internal18_handleErrorCheck_EPKcRKSsS2_ib
pub fn stub_0xc3d0d8() -> ! {
    todo!("0xc3d0d8 G3D::_internal::_handleErrorCheck_(char const*,std::string const&,char const*,int,bool)")
}


// 0xc3d9a8 — __ZN3G3D9_internalL18createErrorMessageEPKcRKSsS2_iRSsS5_
// type: _DWORD __fastcall(G3D::_internal *__hidden this, const char *, const std::string *, const char *, std::string *, std::string *, std::string *)
#[doc(alias = "G3D::_internal::createErrorMessage(char const*,std::string const&,char const*,int,std::string &,std::string &)")]
// was: __ZN3G3D9_internalL18createErrorMessageEPKcRKSsS2_iRSsS5_
pub fn stub_0xc3d9a8() -> ! {
    todo!("0xc3d9a8 G3D::_internal::createErrorMessage(char const*,std::string const&,char const*,int,std::string &,std::string &)")
}


// 0xc3db90 — __ZN3G3D9_internal18_releaseInputGrab_Ev
// type: _DWORD __fastcall(G3D::_internal *__hidden this)
#[doc(alias = "G3D::_internal::_releaseInputGrab_(void)")]
// was: __ZN3G3D9_internal18_releaseInputGrab_Ev
pub fn stub_0xc3db90() -> ! {
    todo!("0xc3db90 G3D::_internal::_releaseInputGrab_(void)")
}


// 0xc3db94 — __ZN3G3D9_internal18_restoreInputGrab_Ev
// type: _DWORD __fastcall(G3D::_internal *__hidden this)
#[doc(alias = "G3D::_internal::_restoreInputGrab_(void)")]
// was: __ZN3G3D9_internal18_restoreInputGrab_Ev
pub fn stub_0xc3db94() -> ! {
    todo!("0xc3db94 G3D::_internal::_restoreInputGrab_(void)")
}


// 0xc3db98 — __ZN3G3D6formatEPKcz
// type: _DWORD(G3D *__hidden this, const char *, ...)
#[doc(alias = "G3D::format(char const*,...)")]
// was: __ZN3G3D6formatEPKcz
pub fn stub_0xc3db98() -> ! {
    todo!("0xc3db98 G3D::format(char const*,...)")
}


// 0xc3dbb8 — __ZN3G3D7vformatEPKcPv
// type: _DWORD __fastcall(G3D *__hidden this, const char *__format, void *)
#[doc(alias = "G3D::vformat(char const*,void *)")]
// was: __ZN3G3D7vformatEPKcPv
pub fn stub_0xc3dbb8() -> ! {
    todo!("0xc3dbb8 G3D::vformat(char const*,void *)")
}


// 0xc3dd58 — __ZN3G3D3infEv
// type: _DWORD __fastcall(G3D *__hidden this)
#[doc(alias = "G3D::inf(void)")]
// was: __ZN3G3D3infEv
pub fn stub_0xc3dd58() -> ! {
    todo!("0xc3dd58 G3D::inf(void)")
}


// 0xc3dd70 — __ZN3G3D5isNaNEf
// type: _DWORD __fastcall(G3D *__hidden this, float)
#[doc(alias = "G3D::isNaN(float)")]
// was: __ZN3G3D5isNaNEf
pub fn stub_0xc3dd70() -> ! {
    todo!("0xc3dd70 G3D::isNaN(float)")
}


// 0xc3ddb8 — __ZN3G3D4finfEv
// type: _DWORD __fastcall(G3D *__hidden this)
#[doc(alias = "G3D::finf(void)")]
// was: __ZN3G3D4finfEv
pub fn stub_0xc3ddb8() -> ! {
    todo!("0xc3ddb8 G3D::finf(void)")
}


// 0xc3ddc0 — __ZN3G3D7iRandomEii
// type: _DWORD __fastcall(G3D *__hidden this, int, int)
#[doc(alias = "G3D::iRandom(int,int)")]
// was: __ZN3G3D7iRandomEii
pub fn stub_0xc3ddc0() -> ! {
    todo!("0xc3ddc0 G3D::iRandom(int,int)")
}


// 0xc3de20 — __ZN3G3D6GImage13resolveFormatERKSsPKhiNS0_6FormatE
#[doc(alias = "G3D::GImage::resolveFormat(std::string const&,unsigned char const*,int,G3D::GImage::Format)")]
// was: __ZN3G3D6GImage13resolveFormatERKSsPKhiNS0_6FormatE
pub fn stub_0xc3de20() -> ! {
    todo!("0xc3de20 G3D::GImage::resolveFormat(std::string const&,unsigned char const*,int,G3D::GImage::Format)")
}


// 0xc3e1b0 — __ZN3G3D6GImage14stringToFormatERKSs
// type: _DWORD __fastcall(G3D::GImage *__hidden this, const std::string *)
#[doc(alias = "G3D::GImage::stringToFormat(std::string const&)")]
// was: __ZN3G3D6GImage14stringToFormatERKSs
pub fn stub_0xc3e1b0() -> ! {
    todo!("0xc3e1b0 G3D::GImage::stringToFormat(std::string const&)")
}


// 0xc3e3a8 — __ZN3G3D6GLight11directionalERKNS_7Vector3ERKNS_6Color3Ebb
// type: _DWORD __fastcall(G3D::GLight *__hidden this, const Vector3 *, const G3D::Color3 *, bool, bool)
#[doc(alias = "G3D::GLight::directional(G3D::Vector3 const&,G3D::Color3 const&,bool,bool)")]
// was: __ZN3G3D6GLight11directionalERKNS_7Vector3ERKNS_6Color3Ebb
pub fn stub_0xc3e3a8() -> ! {
    todo!("0xc3e3a8 G3D::GLight::directional(G3D::Vector3 const&,G3D::Color3 const&,bool,bool)")
}


// 0xc3e498 — __ZNK3G3D4Line12intersectionERKNS_5PlaneE
// type: _DWORD __fastcall(G3D::Line *__hidden this, const G3D::Plane *)
#[doc(alias = "G3D::Line::intersection(G3D::Plane const&)const")]
// was: __ZNK3G3D4Line12intersectionERKNS_5PlaneE
pub fn stub_0xc3e498() -> ! {
    todo!("0xc3e498 G3D::Line::intersection(G3D::Plane const&)const")
}


// 0xc3e578 — __ZNK3G3D4Line12closestPointERKNS_7Vector3E
// type: _DWORD __fastcall(G3D::Line *__hidden this, const Vector3 *)
#[doc(alias = "G3D::Line::closestPoint(G3D::Vector3 const&)const")]
// was: __ZNK3G3D4Line12closestPointERKNS_7Vector3E
pub fn stub_0xc3e578() -> ! {
    todo!("0xc3e578 G3D::Line::closestPoint(G3D::Vector3 const&)const")
}


// 0xc3e5e8 — __ZNK3G3D4Line5pointEv
// type: _DWORD __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::point(void)const")]
// was: __ZNK3G3D4Line5pointEv
pub fn stub_0xc3e5e8() -> ! {
    todo!("0xc3e5e8 G3D::Line::point(void)const")
}


// 0xc3e5f8 — __ZNK3G3D4Line9directionEv
// type: _DWORD __fastcall(G3D::Line *__hidden this)
#[doc(alias = "G3D::Line::direction(void)const")]
// was: __ZNK3G3D4Line9directionEv
pub fn stub_0xc3e5f8() -> ! {
    todo!("0xc3e5f8 G3D::Line::direction(void)const")
}


// 0xc3e640 — __ZNK3G3D11LineSegment12closestPointERKNS_7Vector3E
// type: _DWORD __fastcall(G3D::LineSegment *__hidden this, const Vector3 *)
#[doc(alias = "G3D::LineSegment::closestPoint(G3D::Vector3 const&)const")]
// was: __ZNK3G3D11LineSegment12closestPointERKNS_7Vector3E
pub fn stub_0xc3e640() -> ! {
    todo!("0xc3e640 G3D::LineSegment::closestPoint(G3D::Vector3 const&)const")
}


// 0xc3e78c — __ZN3G3D3Log6commonEv
// type: _DWORD __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::common(void)")]
// was: __ZN3G3D3Log6commonEv
pub fn stub_0xc3e78c() -> ! {
    todo!("0xc3e78c G3D::Log::common(void)")
}


// 0xc3e8f8 — __ZN3G3D3LogC2ERKSsi
// type: _DWORD __fastcall(G3D::Log *__hidden this, const std::string *, int)
#[doc(alias = "G3D::Log::Log(std::string const&,int)")]
// was: __ZN3G3D3LogC2ERKSsi
pub fn stub_0xc3e8f8() -> ! {
    todo!("0xc3e8f8 G3D::Log::Log(std::string const&,int)")
}


// 0xc3ea88 — __ZN3G3D3LogD0Ev
// type: void __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::~Log()")]
// was: __ZN3G3D3LogD0Ev
pub fn stub_0xc3ea88() -> ! {
    todo!("0xc3ea88 G3D::Log::~Log()")
}


// 0xc3eb28 — __ZN3G3D3LogD1Ev
// type: void __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::~Log()")]
// was: __ZN3G3D3LogD1Ev
pub fn stub_0xc3eb28() -> ! {
    todo!("0xc3eb28 G3D::Log::~Log()")
}


// 0xc3eb34 — __ZN3G3D3LogD2Ev
// type: void __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::~Log()")]
// was: __ZN3G3D3LogD2Ev
pub fn stub_0xc3eb34() -> ! {
    todo!("0xc3eb34 G3D::Log::~Log()")
}


// 0xc3eda8 — __ZN3G3D3Log7printlnERKSs
// type: _DWORD __fastcall(G3D::Log *__hidden this, const std::string *)
#[doc(alias = "G3D::Log::println(std::string const&)")]
// was: __ZN3G3D3Log7printlnERKSs
pub fn stub_0xc3eda8() -> ! {
    todo!("0xc3eda8 G3D::Log::println(std::string const&)")
}


// 0xc3edd8 — __ZN3G3D3Log20getCommonLogFilenameEv
// type: _DWORD __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::getCommonLogFilename(void)")]
// was: __ZN3G3D3Log20getCommonLogFilenameEv
pub fn stub_0xc3edd8() -> ! {
    todo!("0xc3edd8 G3D::Log::getCommonLogFilename(void)")
}


// 0xc3edf0 — __ZN3G3D3Log11printHeaderEv
// type: _DWORD __fastcall(G3D::Log *__hidden this)
#[doc(alias = "G3D::Log::printHeader(void)")]
// was: __ZN3G3D3Log11printHeaderEv
pub fn stub_0xc3edf0() -> ! {
    todo!("0xc3edf0 G3D::Log::printHeader(void)")
}


// 0xc3ef4c — __ZN3G3D3Log5printERKSs
// type: _DWORD __fastcall(G3D::Log *__hidden this, const std::string *)
#[doc(alias = "G3D::Log::print(std::string const&)")]
// was: __ZN3G3D3Log5printERKSs
pub fn stub_0xc3ef4c() -> ! {
    todo!("0xc3ef4c G3D::Log::print(std::string const&)")
}


// 0xc3ef78 — __ZN3G3D7Matrix34zeroEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::zero(void)")]
// was: __ZN3G3D7Matrix34zeroEv
pub fn stub_0xc3ef78() -> ! {
    todo!("0xc3ef78 G3D::Matrix3::zero(void)")
}


// 0xc3efbc — __ZN3G3D7Matrix3C1Efffffffff
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float, float, float, float, float, float, float, float, float)
#[doc(alias = "G3D::Matrix3::Matrix3(float,float,float,float,float,float,float,float,float)")]
// was: __ZN3G3D7Matrix3C1Efffffffff
pub fn stub_0xc3efbc() -> ! {
    todo!("0xc3efbc G3D::Matrix3::Matrix3(float,float,float,float,float,float,float,float,float)")
}


// 0xc3eff8 — __ZN3G3D7Matrix38identityEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::identity(void)")]
// was: __ZN3G3D7Matrix38identityEv
pub fn stub_0xc3eff8() -> ! {
    todo!("0xc3eff8 G3D::Matrix3::identity(void)")
}


// 0xc3f040 — __ZNK3G3D7Matrix36columnEi
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, int)
#[doc(alias = "G3D::Matrix3::column(int)const")]
// was: __ZNK3G3D7Matrix36columnEi
pub fn stub_0xc3f040() -> ! {
    todo!("0xc3f040 G3D::Matrix3::column(int)const")
}


// 0xc3f068 — __ZNK3G3D7Matrix313isOrthonormalEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::isOrthonormal(void)const")]
// was: __ZNK3G3D7Matrix313isOrthonormalEv
pub fn stub_0xc3f068() -> ! {
    todo!("0xc3f068 G3D::Matrix3::isOrthonormal(void)const")
}


// 0xc3f348 — __ZN3G3D7Matrix3C1ERKNS_4QuatE
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const G3D::Quat *)
#[doc(alias = "G3D::Matrix3::Matrix3(G3D::Quat const&)")]
// was: __ZN3G3D7Matrix3C1ERKNS_4QuatE
pub fn stub_0xc3f348() -> ! {
    todo!("0xc3f348 G3D::Matrix3::Matrix3(G3D::Quat const&)")
}


// 0xc3f428 — __ZN3G3D7Matrix33setEfffffffff
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float, float, float, float, float, float, float, float, float)
#[doc(alias = "G3D::Matrix3::set(float,float,float,float,float,float,float,float,float)")]
// was: __ZN3G3D7Matrix33setEfffffffff
pub fn stub_0xc3f428() -> ! {
    todo!("0xc3f428 G3D::Matrix3::set(float,float,float,float,float,float,float,float,float)")
}


// 0xc3f464 — __ZN3G3D7Matrix3C1ERKS0_
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "G3D::Matrix3::Matrix3(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3C1ERKS0_
pub fn stub_0xc3f464() -> ! {
    todo!("0xc3f464 G3D::Matrix3::Matrix3(G3D::Matrix3 const&)")
}


// 0xc3f488 — __ZN3G3D7Matrix39setColumnEiRKNS_7Vector3E
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, int, const Vector3 *)
#[doc(alias = "G3D::Matrix3::setColumn(int,G3D::Vector3 const&)")]
// was: __ZN3G3D7Matrix39setColumnEiRKNS_7Vector3E
pub fn stub_0xc3f488() -> ! {
    todo!("0xc3f488 G3D::Matrix3::setColumn(int,G3D::Vector3 const&)")
}


// 0xc3f49c — __ZNK3G3D7Matrix3eqERKS0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Matrix3::operator==(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3eqERKS0_
pub fn stub_0xc3f49c() -> ! {
    todo!("0xc3f49c G3D::Matrix3::operator==(G3D::Matrix3 const&)const")
}


// 0xc3f4ec — __ZNK3G3D7Matrix3neERKS0_
#[doc(alias = "G3D::Matrix3::operator!=(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3neERKS0_
pub fn stub_0xc3f4ec() -> ! {
    todo!("0xc3f4ec G3D::Matrix3::operator!=(G3D::Matrix3 const&)const")
}


// 0xc3f53c — __ZNK3G3D7Matrix3plERKS0_
#[doc(alias = "G3D::Matrix3::operator+(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3plERKS0_
pub fn stub_0xc3f53c() -> ! {
    todo!("0xc3f53c G3D::Matrix3::operator+(G3D::Matrix3 const&)const")
}


// 0xc3f5d4 — __ZNK3G3D7Matrix3mlERKS0_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Matrix3::operator*(G3D::Matrix3 const&)const")]
// was: __ZNK3G3D7Matrix3mlERKS0_
pub fn stub_0xc3f5d4() -> ! {
    todo!("0xc3f5d4 G3D::Matrix3::operator*(G3D::Matrix3 const&)const")
}


// 0xc3f720 — __ZN3G3D7Matrix3pLERKS0_
#[doc(alias = "G3D::Matrix3::operator+=(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3pLERKS0_
pub fn stub_0xc3f720() -> ! {
    todo!("0xc3f720 G3D::Matrix3::operator+=(G3D::Matrix3 const&)")
}


// 0xc3f7b8 — __ZN3G3D7Matrix3mIERKS0_
#[doc(alias = "G3D::Matrix3::operator-=(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3mIERKS0_
pub fn stub_0xc3f7b8() -> ! {
    todo!("0xc3f7b8 G3D::Matrix3::operator-=(G3D::Matrix3 const&)")
}


// 0xc3f850 — __ZN3G3D7Matrix3mLERKS0_
#[doc(alias = "G3D::Matrix3::operator*=(G3D::Matrix3 const&)")]
// was: __ZN3G3D7Matrix3mLERKS0_
pub fn stub_0xc3f850() -> ! {
    todo!("0xc3f850 G3D::Matrix3::operator*=(G3D::Matrix3 const&)")
}


// 0xc3f9b8 — __ZNK3G3D7Matrix3ngEv
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Matrix3::operator-(void)const")]
// was: __ZNK3G3D7Matrix3ngEv
pub fn stub_0xc3f9b8() -> ! {
    todo!("0xc3f9b8 G3D::Matrix3::operator-(void)const")
}


// 0xc3fa30 — __ZN3G3DmlEfRKNS_7Matrix3E
#[doc(alias = "G3D::operator*(float,G3D::Matrix3 const&)")]
// was: __ZN3G3DmlEfRKNS_7Matrix3E
pub fn stub_0xc3fa30() -> ! {
    todo!("0xc3fa30 G3D::operator*(float,G3D::Matrix3 const&)")
}


// 0xc3faa8 — __ZNK3G3D7Matrix39transposeEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::transpose(void)const")]
// was: __ZNK3G3D7Matrix39transposeEv
pub fn stub_0xc3faa8() -> ! {
    todo!("0xc3faa8 G3D::Matrix3::transpose(void)const")
}


// 0xc3fad0 — __ZNK3G3D7Matrix37inverseERS0_f
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, G3D::Matrix3 *, float)
#[doc(alias = "G3D::Matrix3::inverse(G3D::Matrix3&,float)const")]
// was: __ZNK3G3D7Matrix37inverseERS0_f
pub fn stub_0xc3fad0() -> ! {
    todo!("0xc3fad0 G3D::Matrix3::inverse(G3D::Matrix3&,float)const")
}


// 0xc3fc9c — __ZNK3G3D7Matrix37inverseEf
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, G3D::Matrix3 *)
#[doc(alias = "G3D::Matrix3::inverse(float)const")]
// was: __ZNK3G3D7Matrix37inverseEf
pub fn stub_0xc3fc9c() -> ! {
    todo!("0xc3fc9c G3D::Matrix3::inverse(float)const")
}


// 0xc3fd1c — __ZN3G3D7Matrix314orthonormalizeEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::orthonormalize(void)")]
// was: __ZN3G3D7Matrix314orthonormalizeEv
pub fn stub_0xc3fd1c() -> ! {
    todo!("0xc3fd1c G3D::Matrix3::orthonormalize(void)")
}


// 0xc3fe98 — __ZNK3G3D7Matrix36l1NormEv
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this)
#[doc(alias = "G3D::Matrix3::l1Norm(void)const")]
// was: __ZNK3G3D7Matrix36l1NormEv
pub fn stub_0xc3fe98() -> ! {
    todo!("0xc3fe98 G3D::Matrix3::l1Norm(void)const")
}


// 0xc3ff60 — __ZNK3G3D7Matrix311toAxisAngleERNS_7Vector3ERf
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, G3D::Vector3 *, float *)
#[doc(alias = "G3D::Matrix3::toAxisAngle(G3D::Vector3 &,float &)const")]
// was: __ZNK3G3D7Matrix311toAxisAngleERNS_7Vector3ERf
pub fn stub_0xc3ff60() -> ! {
    todo!("0xc3ff60 G3D::Matrix3::toAxisAngle(G3D::Vector3 &,float &)const")
}


// 0xc4015c — __ZN3G3D7Matrix317fromAxisAngleFastERKNS_7Vector3Ef
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const Vector3 *, float)
#[doc(alias = "G3D::Matrix3::fromAxisAngleFast(G3D::Vector3 const&,float)")]
// was: __ZN3G3D7Matrix317fromAxisAngleFastERKNS_7Vector3Ef
pub fn stub_0xc4015c() -> ! {
    todo!("0xc4015c G3D::Matrix3::fromAxisAngleFast(G3D::Vector3 const&,float)")
}


// 0xc40250 — __ZNK3G3D7Matrix316toEulerAnglesXYZERfS1_S1_
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float *, float *, float *)
#[doc(alias = "G3D::Matrix3::toEulerAnglesXYZ(float &,float &,float &)const")]
// was: __ZNK3G3D7Matrix316toEulerAnglesXYZERfS1_S1_
pub fn stub_0xc40250() -> ! {
    todo!("0xc40250 G3D::Matrix3::toEulerAnglesXYZ(float &,float &,float &)const")
}


// 0xc403a8 — __ZN3G3D7Matrix318fromEulerAnglesXYZEfff
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, float, float, float)
#[doc(alias = "G3D::Matrix3::fromEulerAnglesXYZ(float,float,float)")]
// was: __ZN3G3D7Matrix318fromEulerAnglesXYZEfff
pub fn stub_0xc403a8() -> ! {
    todo!("0xc403a8 G3D::Matrix3::fromEulerAnglesXYZ(float,float,float)")
}


// 0xc40598 — __ZN3G3D7Matrix34_mulERKS0_S2_RS0_
// type: _DWORD __fastcall(G3D::Matrix3 *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *, G3D::Matrix3 *)
#[doc(alias = "G3D::Matrix3::_mul(G3D::Matrix3 const&,G3D::Matrix3 const&,G3D::Matrix3&)")]
// was: __ZN3G3D7Matrix34_mulERKS0_S2_RS0_
pub fn stub_0xc40598() -> ! {
    todo!("0xc40598 G3D::Matrix3::_mul(G3D::Matrix3 const&,G3D::Matrix3 const&,G3D::Matrix3&)")
}


// 0xc40784 — __ZN3G3D7Matrix48identityEv
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this)
#[doc(alias = "G3D::Matrix4::identity(void)")]
// was: __ZN3G3D7Matrix48identityEv
pub fn stub_0xc40784() -> ! {
    todo!("0xc40784 G3D::Matrix4::identity(void)")
}


// 0xc407d8 — __ZN3G3D7Matrix4C1ERKNS_15CoordinateFrameE
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "G3D::Matrix4::Matrix4(G3D::CoordinateFrame const&)")]
// was: __ZN3G3D7Matrix4C1ERKNS_15CoordinateFrameE
pub fn stub_0xc407d8() -> ! {
    todo!("0xc407d8 G3D::Matrix4::Matrix4(G3D::CoordinateFrame const&)")
}


// 0xc40818 — __ZNK3G3D7Matrix48upper3x3Ev
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this)
#[doc(alias = "G3D::Matrix4::upper3x3(void)const")]
// was: __ZNK3G3D7Matrix48upper3x3Ev
pub fn stub_0xc40818() -> ! {
    todo!("0xc40818 G3D::Matrix4::upper3x3(void)const")
}


// 0xc40860 — __ZN3G3D7Matrix4C1Ev
// type: _DWORD __fastcall(G3D::Matrix4 *__hidden this)
#[doc(alias = "G3D::Matrix4::Matrix4(void)")]
// was: __ZN3G3D7Matrix4C1Ev
pub fn stub_0xc40860() -> ! {
    todo!("0xc40860 G3D::Matrix4::Matrix4(void)")
}


// 0xc40884 — __ZNK3G3D7Matrix4mlERKNS_7Vector4E
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Matrix4::operator*(G3D::Vector4 const&)const")]
// was: __ZNK3G3D7Matrix4mlERKNS_7Vector4E
pub fn stub_0xc40884() -> ! {
    todo!("0xc40884 G3D::Matrix4::operator*(G3D::Vector4 const&)const")
}


// 0xc409ac — __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_S3_
// type: _DWORD __fastcall(G3D::Plane *__hidden this, const Vector3 *, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Plane::Plane(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_S3_
pub fn stub_0xc409ac() -> ! {
    todo!("0xc409ac G3D::Plane::Plane(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")
}


// 0xc40a94 — __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Plane *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "G3D::Plane::Plane(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: __ZN3G3D5PlaneC1ERKNS_7Vector3ES3_
pub fn stub_0xc40a94() -> ! {
    todo!("0xc40a94 G3D::Plane::Plane(G3D::Vector3 const&,G3D::Vector3 const&)")
}


// 0xc40b20 — __ZN3G3D5Plane12fromEquationEffff
// type: _DWORD __fastcall(G3D::Plane *__hidden this, float, float, float, float)
#[doc(alias = "G3D::Plane::fromEquation(float,float,float,float)")]
// was: __ZN3G3D5Plane12fromEquationEffff
pub fn stub_0xc40b20() -> ! {
    todo!("0xc40b20 G3D::Plane::fromEquation(float,float,float,float)")
}


// 0xc40b8c — __ZNK3G3D5Plane11getEquationERNS_7Vector3ERf
// type: _DWORD __fastcall(G3D::Plane *__hidden this, Vector3 *, float *)
#[doc(alias = "G3D::Plane::getEquation(G3D::Vector3 &,float &)const")]
// was: __ZNK3G3D5Plane11getEquationERNS_7Vector3ERf
pub fn stub_0xc40b8c() -> ! {
    todo!("0xc40b8c G3D::Plane::getEquation(G3D::Vector3 &,float &)const")
}


// 0xc40be4 — __ZN3G3D4QuatC1ERKNS_7Matrix3E
// type: _DWORD __fastcall(G3D::Quat *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "G3D::Quat::Quat(G3D::Matrix3 const&)")]
// was: __ZN3G3D4QuatC1ERKNS_7Matrix3E
pub fn stub_0xc40be4() -> ! {
    todo!("0xc40be4 G3D::Quat::Quat(G3D::Matrix3 const&)")
}


// 0xc40bf0 — __ZN3G3D4QuatC2ERKNS_7Matrix3E
// type: _DWORD __fastcall(G3D::Quat *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "G3D::Quat::Quat(G3D::Matrix3 const&)")]
// was: __ZN3G3D4QuatC2ERKNS_7Matrix3E
pub fn stub_0xc40bf0() -> ! {
    todo!("0xc40bf0 G3D::Quat::Quat(G3D::Matrix3 const&)")
}


// 0xc40d70 — __ZNK3G3D4Quat16toRotationMatrixEv
// type: _DWORD __fastcall(G3D::Quat *__hidden this)
#[doc(alias = "G3D::Quat::toRotationMatrix(void)const")]
// was: __ZNK3G3D4Quat16toRotationMatrixEv
pub fn stub_0xc40d70() -> ! {
    todo!("0xc40d70 G3D::Quat::toRotationMatrix(void)const")
}


// 0xc40db4 — __ZNK3G3D4Quat5slerpERKS0_ff
// type: _DWORD __fastcall(G3D::Quat *__hidden this, const G3D::Quat *, float, float)
#[doc(alias = "G3D::Quat::slerp(G3D::Quat const&,float,float)const")]
// was: __ZNK3G3D4Quat5slerpERKS0_ff
pub fn stub_0xc40db4() -> ! {
    todo!("0xc40db4 G3D::Quat::slerp(G3D::Quat const&,float,float)const")
}


// 0xc40ffc — __ZN3G3D3Ray3setERKNS_7Vector3ES3_
// type: _DWORD __fastcall(G3D::Ray *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "G3D::Ray::set(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: __ZN3G3D3Ray3setERKNS_7Vector3ES3_
pub fn stub_0xc40ffc() -> ! {
    todo!("0xc40ffc G3D::Ray::set(G3D::Vector3 const&,G3D::Vector3 const&)")
}


// 0xc412bc — __ZNK3G3D6Sphere8toStringEv
// type: _DWORD __fastcall(G3D::Sphere *__hidden this)
#[doc(alias = "G3D::Sphere::toString(void)const")]
// was: __ZNK3G3D6Sphere8toStringEv
pub fn stub_0xc412bc() -> ! {
    todo!("0xc412bc G3D::Sphere::toString(void)const")
}


// 0xc41304 — __ZNK3G3D6Sphere8containsERKNS_7Vector3E
// type: _DWORD __fastcall(G3D::Sphere *__hidden this, const Vector3 *)
#[doc(alias = "G3D::Sphere::contains(G3D::Vector3 const&)const")]
// was: __ZNK3G3D6Sphere8containsERKNS_7Vector3E
pub fn stub_0xc41304() -> ! {
    todo!("0xc41304 G3D::Sphere::contains(G3D::Vector3 const&)const")
}


// 0xc41388 — __ZN3G3D10beginsWithERKSsS1_
#[doc(alias = "G3D::beginsWith(std::string const&,std::string const&)")]
// was: __ZN3G3D10beginsWithERKSsS1_
pub fn stub_0xc41388() -> ! {
    todo!("0xc41388 G3D::beginsWith(std::string const&,std::string const&)")
}


// 0xc413c0 — __ZN3G3D8endsWithERKSsS1_
#[doc(alias = "G3D::endsWith(std::string const&,std::string const&)")]
// was: __ZN3G3D8endsWithERKSsS1_
pub fn stub_0xc413c0() -> ! {
    todo!("0xc413c0 G3D::endsWith(std::string const&,std::string const&)")
}


// 0xc413f8 — __ZN3G3D7toUpperERKSs
// type: _DWORD __fastcall(G3D *__hidden this, const std::string *)
#[doc(alias = "G3D::toUpper(std::string const&)")]
// was: __ZN3G3D7toUpperERKSs
pub fn stub_0xc413f8() -> ! {
    todo!("0xc413f8 G3D::toUpper(std::string const&)")
}


// 0xc4158c — __ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEEC2Ev
#[doc(alias = "G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::Table(void)")]
// was: __ZN3G3D5TableISsb9HashTraitISsE11EqualsTraitISsEEC2Ev
pub fn stub_0xc4158c() -> ! {
    todo!("0xc4158c G3D::Table<std::string,bool,HashTrait<std::string>,EqualsTrait<std::string>>::Table(void)")
}


// 0xc417bc — __ZN3G3D7Vector23oneEv
// type: _DWORD __fastcall(G3D::Vector2 *__hidden this)
#[doc(alias = "G3D::Vector2::one(void)")]
// was: __ZN3G3D7Vector23oneEv
pub fn stub_0xc417bc() -> ! {
    todo!("0xc417bc G3D::Vector2::one(void)")
}


// 0xc417f0 — __ZN3G3D7Vector24zeroEv
// type: _DWORD __fastcall(G3D::Vector2 *__hidden this)
#[doc(alias = "G3D::Vector2::zero(void)")]
// was: __ZN3G3D7Vector24zeroEv
pub fn stub_0xc417f0() -> ! {
    todo!("0xc417f0 G3D::Vector2::zero(void)")
}


// 0xc41824 — __ZNK3G3D7Vector2dvEf
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Vector2::operator/(float)const")]
// was: __ZNK3G3D7Vector2dvEf
pub fn stub_0xc41824() -> ! {
    todo!("0xc41824 G3D::Vector2::operator/(float)const")
}


// 0xc41850 — __ZN3G3D7Vector2dVEf
#[doc(alias = "G3D::Vector2::operator/=(float)")]
// was: __ZN3G3D7Vector2dVEf
pub fn stub_0xc41850() -> ! {
    todo!("0xc41850 G3D::Vector2::operator/=(float)")
}


// 0xc41870 — __ZNK3G3D7Vector22yxEv
// type: _DWORD __fastcall(G3D::Vector2 *__hidden this)
#[doc(alias = "G3D::Vector2::yx(void)const")]
// was: __ZNK3G3D7Vector22yxEv
pub fn stub_0xc41870() -> ! {
    todo!("0xc41870 G3D::Vector2::yx(void)const")
}


// 0xc418bc — __ZN3G3D12Vector2int16C1ERKNS_7Vector2E
// type: _DWORD __fastcall(G3D::Vector2int16 *__hidden this, const G3D::Vector2 *)
#[doc(alias = "G3D::Vector2int16::Vector2int16(G3D::Vector2 const&)")]
// was: __ZN3G3D12Vector2int16C1ERKNS_7Vector2E
pub fn stub_0xc418bc() -> ! {
    todo!("0xc418bc G3D::Vector2int16::Vector2int16(G3D::Vector2 const&)")
}


// 0xc41918 — __ZN3G3D12Vector2int16C1EPi
// type: _DWORD __fastcall(G3D::Vector2int16 *__hidden this, int *)
#[doc(alias = "G3D::Vector2int16::Vector2int16(int *)")]
// was: __ZN3G3D12Vector2int16C1EPi
pub fn stub_0xc41918() -> ! {
    todo!("0xc41918 G3D::Vector2int16::Vector2int16(int *)")
}


// 0xc41924 — __ZNK3G3D12Vector2int162yxEv
// type: _DWORD __fastcall(G3D::Vector2int16 *__hidden this)
#[doc(alias = "G3D::Vector2int16::yx(void)const")]
// was: __ZNK3G3D12Vector2int162yxEv
pub fn stub_0xc41924() -> ! {
    todo!("0xc41924 G3D::Vector2int16::yx(void)const")
}


// 0xc41964 — __ZN3G3D7Vector3C1ERKNS_7Vector2Ef
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, const G3D::Vector2 *, float)
#[doc(alias = "G3D::Vector3::Vector3(G3D::Vector2 const&,float)")]
// was: __ZN3G3D7Vector3C1ERKNS_7Vector2Ef
pub fn stub_0xc41964() -> ! {
    todo!("0xc41964 G3D::Vector3::Vector3(G3D::Vector2 const&,float)")
}


// 0xc41970 — __ZN3G3D7Vector34zeroEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::zero(void)")]
// was: __ZN3G3D7Vector34zeroEv
pub fn stub_0xc41970() -> ! {
    todo!("0xc41970 G3D::Vector3::zero(void)")
}


// 0xc419a8 — __ZN3G3D7Vector33oneEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::one(void)")]
// was: __ZN3G3D7Vector33oneEv
pub fn stub_0xc419a8() -> ! {
    todo!("0xc419a8 G3D::Vector3::one(void)")
}


// 0xc419e0 — __ZN3G3D7Vector35unitXEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::unitX(void)")]
// was: __ZN3G3D7Vector35unitXEv
pub fn stub_0xc419e0() -> ! {
    todo!("0xc419e0 G3D::Vector3::unitX(void)")
}


// 0xc41a1c — __ZN3G3D7Vector35unitYEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::unitY(void)")]
// was: __ZN3G3D7Vector35unitYEv
pub fn stub_0xc41a1c() -> ! {
    todo!("0xc41a1c G3D::Vector3::unitY(void)")
}


// 0xc41a58 — __ZN3G3D7Vector35unitZEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::unitZ(void)")]
// was: __ZN3G3D7Vector35unitZEv
pub fn stub_0xc41a58() -> ! {
    todo!("0xc41a58 G3D::Vector3::unitZ(void)")
}


// 0xc41a94 — __ZN3G3D7Vector33infEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::inf(void)")]
// was: __ZN3G3D7Vector33infEv
pub fn stub_0xc41a94() -> ! {
    todo!("0xc41a94 G3D::Vector3::inf(void)")
}


// 0xc41b94 — __ZN3G3D7Vector39minFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::minFinite(void)")]
// was: __ZN3G3D7Vector39minFiniteEv
pub fn stub_0xc41b94() -> ! {
    todo!("0xc41b94 G3D::Vector3::minFinite(void)")
}


// 0xc41bd4 — __ZN3G3D7Vector39maxFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::maxFinite(void)")]
// was: __ZN3G3D7Vector39maxFiniteEv
pub fn stub_0xc41bd4() -> ! {
    todo!("0xc41bd4 G3D::Vector3::maxFinite(void)")
}


// 0xc41c18 — __ZNK3G3D7Vector311primaryAxisEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::primaryAxis(void)const")]
// was: __ZNK3G3D7Vector311primaryAxisEv
pub fn stub_0xc41c18() -> ! {
    todo!("0xc41c18 G3D::Vector3::primaryAxis(void)const")
}


// 0xc41c5c — __ZN3G3D7Vector3C1ERKNS_12Vector3int16E
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, const G3D::Vector3int16 *)
#[doc(alias = "G3D::Vector3::Vector3(G3D::Vector3int16 const&)")]
// was: __ZN3G3D7Vector3C1ERKNS_12Vector3int16E
pub fn stub_0xc41c5c() -> ! {
    todo!("0xc41c5c G3D::Vector3::Vector3(G3D::Vector3int16 const&)")
}


// 0xc41c98 — __ZN3G3D7Vector36randomERNS_6RandomE
#[doc(alias = "G3D::Vector3::random(G3D::Random &)")]
// was: __ZN3G3D7Vector36randomERNS_6RandomE
pub fn stub_0xc41c98() -> ! {
    todo!("0xc41c98 G3D::Vector3::random(G3D::Random &)")
}


// 0xc41cc0 — __ZN3G3D7Vector37unitizeEf
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, float)
#[doc(alias = "G3D::Vector3::unitize(float)")]
// was: __ZN3G3D7Vector37unitizeEf
pub fn stub_0xc41cc0() -> ! {
    todo!("0xc41cc0 G3D::Vector3::unitize(float)")
}


// 0xc41d24 — __ZN3G3D7Vector324generateOrthonormalBasisERS0_S1_S1_b
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this, G3D::Vector3 *, G3D::Vector3 *, G3D::Vector3 *, bool)
#[doc(alias = "G3D::Vector3::generateOrthonormalBasis(G3D::Vector3&,G3D::Vector3&,G3D::Vector3&,bool)")]
// was: __ZN3G3D7Vector324generateOrthonormalBasisERS0_S1_S1_b
pub fn stub_0xc41d24() -> ! {
    todo!("0xc41d24 G3D::Vector3::generateOrthonormalBasis(G3D::Vector3&,G3D::Vector3&,G3D::Vector3&,bool)")
}


// 0xc41e7c — __ZNK3G3D7Vector314toVector3int16Ev
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::toVector3int16(void)const")]
// was: __ZNK3G3D7Vector314toVector3int16Ev
pub fn stub_0xc41e7c() -> ! {
    todo!("0xc41e7c G3D::Vector3::toVector3int16(void)const")
}


// 0xc41ee0 — __ZNK3G3D7Vector32xyEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::xy(void)const")]
// was: __ZNK3G3D7Vector32xyEv
pub fn stub_0xc41ee0() -> ! {
    todo!("0xc41ee0 G3D::Vector3::xy(void)const")
}

