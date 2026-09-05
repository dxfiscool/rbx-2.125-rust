// Auto-generated skeletons for rbx-script — gap filler c9c1d0 — Lua/VM/Script priority + EA gap fill
// Filter: demangled/mangled contains 'Lua'|'Script'|'VM'|'Luau'|'YieldFunction' (case-insensitive), EA not in /tmp/global_eas.txt
// Fallback: genuine script namespace exhausted in global set (17 naive hits are Ogre/boost false positives via EPvm/Description substrings), so lowest-EA global gaps EA-sorted asc, take 100
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xc9c1d0..0xca38b8 | EA-sorted asc not yet in any crate
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xc9c1d0 — __ZNK4Ogre7Frustum17getFrustumExtentsERfS1_S1_S1_
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, float *, float *, float *, float *)
#[doc(alias = "Ogre::Frustum::getFrustumExtents(float &,float &,float &,float &)const")]
#[doc(alias = "__ZNK4Ogre7Frustum17getFrustumExtentsERfS1_S1_S1_")]
pub fn stub_0xc9c1d0() -> crate::slot::PortedFn {
// IDA 0xc9c1d0: Ogre::Frustum::getFrustumExtents(float&, float&, float&, float&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c1d0, "Ogre::Frustum::getFrustumExtents(float&, float&, float&, float&) const")
}

// 0xc9c20c — __ZN4Ogre7Frustum18setOrientationModeENS_15OrientationModeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "Ogre::Frustum::setOrientationMode(Ogre::OrientationMode)")]
#[doc(alias = "__ZN4Ogre7Frustum18setOrientationModeENS_15OrientationModeE")]
pub fn stub_0xc9c20c() -> crate::slot::PortedFn {
// IDA 0xc9c20c: Ogre::Frustum::setOrientationMode(Ogre::OrientationMode).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c20c, "Ogre::Frustum::setOrientationMode(Ogre::OrientationMode)")
}

// 0xc9c220 — __ZNK4Ogre7Matrix4mlERKNS_5PlaneE
#[doc(alias = "Ogre::Matrix4::operator*(Ogre::Plane const&)const")]
#[doc(alias = "__ZNK4Ogre7Matrix4mlERKNS_5PlaneE")]
pub fn stub_0xc9c220() -> crate::slot::PortedFn {
// IDA 0xc9c220: Ogre::Matrix4::operator*(Ogre::Plane const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c220, "Ogre::Matrix4::operator*(Ogre::Plane const&) const")
}

// 0xc9c3e0 — __ZN4Ogre10GpuProgramC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, Ogre::ResourceManager *, const std::string *, unsigned __int64, const std::string *, bool, Ogre::ManualResourceLoader *)
#[doc(alias = "Ogre::GpuProgram::GpuProgram(Ogre::ResourceManager *,std::string const&,unsigned long long,std::string const&,bool,Ogre::ManualResourceLoader *)")]
#[doc(alias = "__ZN4Ogre10GpuProgramC2EPNS_15ResourceManagerERKSsyS4_bPNS_20ManualResourceLoaderE")]
pub fn stub_0xc9c3e0() -> crate::slot::PortedFn {
// IDA 0xc9c3e0: Ogre::GpuProgram::GpuProgram(Ogre::ResourceManager*, std::string const&, unsigned long long, std::string const&, bool, O~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c3e0, "Ogre::GpuProgram::GpuProgram(Ogre::ResourceManager*, std::string const&, unsigned long long, std::st~")
}

// 0xc9c71c — __ZNK4Ogre10GpuProgram32createParameterMappingStructuresEb
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, bool)
#[doc(alias = "Ogre::GpuProgram::createParameterMappingStructures(bool)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram32createParameterMappingStructuresEb")]
pub fn stub_0xc9c71c() -> crate::slot::PortedFn {
// IDA 0xc9c71c: Ogre::GpuProgram::createParameterMappingStructures(bool) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c71c, "Ogre::GpuProgram::createParameterMappingStructures(bool) const")
}

// 0xc9c734 — __ZN4Ogre10GpuProgram7setTypeENS_14GpuProgramTypeE
#[doc(alias = "Ogre::GpuProgram::setType(Ogre::GpuProgramType)")]
#[doc(alias = "__ZN4Ogre10GpuProgram7setTypeENS_14GpuProgramTypeE")]
pub fn stub_0xc9c734() -> crate::slot::PortedFn {
// IDA 0xc9c734: Ogre::GpuProgram::setType(Ogre::GpuProgramType).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c734, "Ogre::GpuProgram::setType(Ogre::GpuProgramType)")
}

// 0xc9c738 — __ZN4Ogre10GpuProgram13setSyntaxCodeERKSs
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgram::setSyntaxCode(std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram13setSyntaxCodeERKSs")]
pub fn stub_0xc9c738() -> crate::slot::PortedFn {
// IDA 0xc9c738: Ogre::GpuProgram::setSyntaxCode(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c738, "Ogre::GpuProgram::setSyntaxCode(std::string const&)")
}

// 0xc9c744 — __ZN4Ogre10GpuProgram13setSourceFileERKSs
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgram::setSourceFile(std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram13setSourceFileERKSs")]
pub fn stub_0xc9c744() -> crate::slot::PortedFn {
// IDA 0xc9c744: Ogre::GpuProgram::setSourceFile(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c744, "Ogre::GpuProgram::setSourceFile(std::string const&)")
}

// 0xc9c774 — __ZN4Ogre10GpuProgram9setSourceERKSs
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgram::setSource(std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram9setSourceERKSs")]
pub fn stub_0xc9c774() -> crate::slot::PortedFn {
// IDA 0xc9c774: Ogre::GpuProgram::setSource(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c774, "Ogre::GpuProgram::setSource(std::string const&)")
}

// 0xc9c7a0 — __ZN4Ogre10GpuProgram8loadImplEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::loadImpl(void)")]
#[doc(alias = "__ZN4Ogre10GpuProgram8loadImplEv")]
pub fn stub_0xc9c7a0() -> crate::slot::PortedFn {
// IDA 0xc9c7a0: Ogre::GpuProgram::loadImpl().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9c7a0, "Ogre::GpuProgram::loadImpl()")
}

// 0xc9cbf4 — __ZNK4Ogre10GpuProgram11isSupportedEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::isSupported(void)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram11isSupportedEv")]
pub fn stub_0xc9cbf4() -> crate::slot::PortedFn {
// IDA 0xc9cbf4: Ogre::GpuProgram::isSupported() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9cbf4, "Ogre::GpuProgram::isSupported() const")
}

// 0xc9cc58 — __ZNK4Ogre10GpuProgram39createLogicalParameterMappingStructuresEb
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, bool)
#[doc(alias = "Ogre::GpuProgram::createLogicalParameterMappingStructures(bool)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram39createLogicalParameterMappingStructuresEb")]
pub fn stub_0xc9cc58() -> crate::slot::PortedFn {
// IDA 0xc9cc58: Ogre::GpuProgram::createLogicalParameterMappingStructures(bool) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9cc58, "Ogre::GpuProgram::createLogicalParameterMappingStructures(bool) const")
}

// 0xc9ceb4 — __ZNK4Ogre10GpuProgram37createNamedParameterMappingStructuresEb
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, bool)
#[doc(alias = "Ogre::GpuProgram::createNamedParameterMappingStructures(bool)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram37createNamedParameterMappingStructuresEb")]
pub fn stub_0xc9ceb4() -> crate::slot::PortedFn {
// IDA 0xc9ceb4: Ogre::GpuProgram::createNamedParameterMappingStructures(bool) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ceb4, "Ogre::GpuProgram::createNamedParameterMappingStructures(bool) const")
}

// 0xc9d014 — __ZN4Ogre10GpuProgram27setManualNamedConstantsFileERKSs
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgram::setManualNamedConstantsFile(std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram27setManualNamedConstantsFileERKSs")]
pub fn stub_0xc9d014() -> crate::slot::PortedFn {
// IDA 0xc9d014: Ogre::GpuProgram::setManualNamedConstantsFile(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9d014, "Ogre::GpuProgram::setManualNamedConstantsFile(std::string const&)")
}

// 0xc9d02c — __ZN4Ogre10GpuProgram23setManualNamedConstantsERKNS_17GpuNamedConstantsE
#[doc(alias = "Ogre::GpuProgram::setManualNamedConstants(Ogre::GpuNamedConstants const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram23setManualNamedConstantsERKNS_17GpuNamedConstantsE")]
pub fn stub_0xc9d02c() -> crate::slot::PortedFn {
// IDA 0xc9d02c: Ogre::GpuProgram::setManualNamedConstants(Ogre::GpuNamedConstants const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9d02c, "Ogre::GpuProgram::setManualNamedConstants(Ogre::GpuNamedConstants const&)")
}

// 0xc9d16c — __ZN4Ogre10GpuProgram16createParametersEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::createParameters(void)")]
#[doc(alias = "__ZN4Ogre10GpuProgram16createParametersEv")]
pub fn stub_0xc9d16c() -> crate::slot::PortedFn {
// IDA 0xc9d16c: Ogre::GpuProgram::createParameters().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9d16c, "Ogre::GpuProgram::createParameters()")
}

// 0xc9d4d8 — __ZN4Ogre10GpuProgram20getDefaultParametersEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::getDefaultParameters(void)")]
#[doc(alias = "__ZN4Ogre10GpuProgram20getDefaultParametersEv")]
pub fn stub_0xc9d4d8() -> crate::slot::PortedFn {
// IDA 0xc9d4d8: Ogre::GpuProgram::getDefaultParameters().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9d4d8, "Ogre::GpuProgram::getDefaultParameters()")
}

// 0xc9d610 — __ZN4Ogre10GpuProgram24setupBaseParamDictionaryEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::setupBaseParamDictionary(void)")]
#[doc(alias = "__ZN4Ogre10GpuProgram24setupBaseParamDictionaryEv")]
pub fn stub_0xc9d610() -> crate::slot::PortedFn {
// IDA 0xc9d610: Ogre::GpuProgram::setupBaseParamDictionary().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9d610, "Ogre::GpuProgram::setupBaseParamDictionary()")
}

// 0xc9eb68 — __ZNK4Ogre10GpuProgram11getLanguageEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::getLanguage(void)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram11getLanguageEv")]
pub fn stub_0xc9eb68() -> crate::slot::PortedFn {
// IDA 0xc9eb68: Ogre::GpuProgram::getLanguage() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9eb68, "Ogre::GpuProgram::getLanguage() const")
}

// 0xc9ec5c — __ZNK4Ogre10GpuProgram7CmdType5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdType *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdType::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram7CmdType5doGetEPKv")]
pub fn stub_0xc9ec5c() -> crate::slot::PortedFn {
// IDA 0xc9ec5c: Ogre::GpuProgram::CmdType::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ec5c, "Ogre::GpuProgram::CmdType::doGet(void const*) const")
}

// 0xc9ed58 — __ZN4Ogre10GpuProgram7CmdType5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdType *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::GpuProgram::CmdType::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram7CmdType5doSetEPvRKSs")]
pub fn stub_0xc9ed58() -> crate::slot::PortedFn {
// IDA 0xc9ed58: Ogre::GpuProgram::CmdType::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ed58, "Ogre::GpuProgram::CmdType::doSet(void*, std::string const&)")
}

// 0xc9edac — __ZNK4Ogre10GpuProgram9CmdSyntax5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdSyntax *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdSyntax::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram9CmdSyntax5doGetEPKv")]
pub fn stub_0xc9edac() -> crate::slot::PortedFn {
// IDA 0xc9edac: Ogre::GpuProgram::CmdSyntax::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9edac, "Ogre::GpuProgram::CmdSyntax::doGet(void const*) const")
}

// 0xc9edc8 — __ZN4Ogre10GpuProgram9CmdSyntax5doSetEPvRKSs
#[doc(alias = "Ogre::GpuProgram::CmdSyntax::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram9CmdSyntax5doSetEPvRKSs")]
pub fn stub_0xc9edc8() -> crate::slot::PortedFn {
// IDA 0xc9edc8: Ogre::GpuProgram::CmdSyntax::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9edc8, "Ogre::GpuProgram::CmdSyntax::doSet(void*, std::string const&)")
}

// 0xc9eddc — __ZNK4Ogre10GpuProgram11CmdSkeletal5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdSkeletal *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdSkeletal::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram11CmdSkeletal5doGetEPKv")]
pub fn stub_0xc9eddc() -> crate::slot::PortedFn {
// IDA 0xc9eddc: Ogre::GpuProgram::CmdSkeletal::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9eddc, "Ogre::GpuProgram::CmdSkeletal::doGet(void const*) const")
}

// 0xc9edf8 — __ZN4Ogre10GpuProgram11CmdSkeletal5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdSkeletal *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::GpuProgram::CmdSkeletal::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram11CmdSkeletal5doSetEPvRKSs")]
pub fn stub_0xc9edf8() -> crate::slot::PortedFn {
// IDA 0xc9edf8: Ogre::GpuProgram::CmdSkeletal::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9edf8, "Ogre::GpuProgram::CmdSkeletal::doSet(void*, std::string const&)")
}

// 0xc9ee14 — __ZNK4Ogre10GpuProgram8CmdMorph5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdMorph *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdMorph::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram8CmdMorph5doGetEPKv")]
pub fn stub_0xc9ee14() -> crate::slot::PortedFn {
// IDA 0xc9ee14: Ogre::GpuProgram::CmdMorph::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ee14, "Ogre::GpuProgram::CmdMorph::doGet(void const*) const")
}

// 0xc9ee30 — __ZN4Ogre10GpuProgram8CmdMorph5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdMorph *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::GpuProgram::CmdMorph::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram8CmdMorph5doSetEPvRKSs")]
pub fn stub_0xc9ee30() -> crate::slot::PortedFn {
// IDA 0xc9ee30: Ogre::GpuProgram::CmdMorph::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ee30, "Ogre::GpuProgram::CmdMorph::doSet(void*, std::string const&)")
}

// 0xc9ee4c — __ZNK4Ogre10GpuProgram7CmdPose5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdPose *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdPose::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram7CmdPose5doGetEPKv")]
pub fn stub_0xc9ee4c() -> crate::slot::PortedFn {
// IDA 0xc9ee4c: Ogre::GpuProgram::CmdPose::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ee4c, "Ogre::GpuProgram::CmdPose::doGet(void const*) const")
}

// 0xc9ee74 — __ZN4Ogre10GpuProgram7CmdPose5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdPose *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::GpuProgram::CmdPose::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram7CmdPose5doSetEPvRKSs")]
pub fn stub_0xc9ee74() -> crate::slot::PortedFn {
// IDA 0xc9ee74: Ogre::GpuProgram::CmdPose::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ee74, "Ogre::GpuProgram::CmdPose::doSet(void*, std::string const&)")
}

// 0xc9ee90 — __ZNK4Ogre10GpuProgram6CmdVTF5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdVTF *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdVTF::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram6CmdVTF5doGetEPKv")]
pub fn stub_0xc9ee90() -> crate::slot::PortedFn {
// IDA 0xc9ee90: Ogre::GpuProgram::CmdVTF::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ee90, "Ogre::GpuProgram::CmdVTF::doGet(void const*) const")
}

// 0xc9eeac — __ZN4Ogre10GpuProgram6CmdVTF5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdVTF *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::GpuProgram::CmdVTF::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram6CmdVTF5doSetEPvRKSs")]
pub fn stub_0xc9eeac() -> crate::slot::PortedFn {
// IDA 0xc9eeac: Ogre::GpuProgram::CmdVTF::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9eeac, "Ogre::GpuProgram::CmdVTF::doSet(void*, std::string const&)")
}

// 0xc9eec8 — __ZNK4Ogre10GpuProgram24CmdManualNamedConstsFile5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdManualNamedConstsFile *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdManualNamedConstsFile::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram24CmdManualNamedConstsFile5doGetEPKv")]
pub fn stub_0xc9eec8() -> crate::slot::PortedFn {
// IDA 0xc9eec8: Ogre::GpuProgram::CmdManualNamedConstsFile::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9eec8, "Ogre::GpuProgram::CmdManualNamedConstsFile::doGet(void const*) const")
}

// 0xc9eee4 — __ZN4Ogre10GpuProgram24CmdManualNamedConstsFile5doSetEPvRKSs
#[doc(alias = "Ogre::GpuProgram::CmdManualNamedConstsFile::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram24CmdManualNamedConstsFile5doSetEPvRKSs")]
pub fn stub_0xc9eee4() -> crate::slot::PortedFn {
// IDA 0xc9eee4: Ogre::GpuProgram::CmdManualNamedConstsFile::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9eee4, "Ogre::GpuProgram::CmdManualNamedConstsFile::doSet(void*, std::string const&)")
}

// 0xc9eef8 — __ZNK4Ogre10GpuProgram12CmdAdjacency5doGetEPKv
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdAdjacency *__hidden this, const void *)
#[doc(alias = "Ogre::GpuProgram::CmdAdjacency::doGet(void const*)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram12CmdAdjacency5doGetEPKv")]
pub fn stub_0xc9eef8() -> crate::slot::PortedFn {
// IDA 0xc9eef8: Ogre::GpuProgram::CmdAdjacency::doGet(void const*) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9eef8, "Ogre::GpuProgram::CmdAdjacency::doGet(void const*) const")
}

// 0xc9ef14 — __ZN4Ogre10GpuProgram12CmdAdjacency5doSetEPvRKSs
// type: _DWORD __fastcall(Ogre::GpuProgram::CmdAdjacency *__hidden this, void *, const std::string *)
#[doc(alias = "Ogre::GpuProgram::CmdAdjacency::doSet(void *,std::string const&)")]
#[doc(alias = "__ZN4Ogre10GpuProgram12CmdAdjacency5doSetEPvRKSs")]
pub fn stub_0xc9ef14() -> crate::slot::PortedFn {
// IDA 0xc9ef14: Ogre::GpuProgram::CmdAdjacency::doSet(void*, std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef14, "Ogre::GpuProgram::CmdAdjacency::doSet(void*, std::string const&)")
}

// 0xc9ef30 — __ZN4Ogre10GpuProgram7CmdTypeD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdType *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdType::~CmdType()")]
#[doc(alias = "__ZN4Ogre10GpuProgram7CmdTypeD1Ev")]
pub fn stub_0xc9ef30() -> crate::slot::PortedFn {
// IDA 0xc9ef30: Ogre::GpuProgram::CmdType::~CmdType().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef30, "Ogre::GpuProgram::CmdType::~CmdType()")
}

// 0xc9ef34 — __ZN4Ogre10GpuProgram9CmdSyntaxD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdSyntax *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdSyntax::~CmdSyntax()")]
#[doc(alias = "__ZN4Ogre10GpuProgram9CmdSyntaxD1Ev")]
pub fn stub_0xc9ef34() -> crate::slot::PortedFn {
// IDA 0xc9ef34: Ogre::GpuProgram::CmdSyntax::~CmdSyntax().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef34, "Ogre::GpuProgram::CmdSyntax::~CmdSyntax()")
}

// 0xc9ef38 — __ZN4Ogre10GpuProgram11CmdSkeletalD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdSkeletal *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdSkeletal::~CmdSkeletal()")]
#[doc(alias = "__ZN4Ogre10GpuProgram11CmdSkeletalD1Ev")]
pub fn stub_0xc9ef38() -> crate::slot::PortedFn {
// IDA 0xc9ef38: Ogre::GpuProgram::CmdSkeletal::~CmdSkeletal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef38, "Ogre::GpuProgram::CmdSkeletal::~CmdSkeletal()")
}

// 0xc9ef3c — __ZN4Ogre10GpuProgram8CmdMorphD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdMorph *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdMorph::~CmdMorph()")]
#[doc(alias = "__ZN4Ogre10GpuProgram8CmdMorphD1Ev")]
pub fn stub_0xc9ef3c() -> crate::slot::PortedFn {
// IDA 0xc9ef3c: Ogre::GpuProgram::CmdMorph::~CmdMorph().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef3c, "Ogre::GpuProgram::CmdMorph::~CmdMorph()")
}

// 0xc9ef40 — __ZN4Ogre10GpuProgram7CmdPoseD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdPose *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdPose::~CmdPose()")]
#[doc(alias = "__ZN4Ogre10GpuProgram7CmdPoseD1Ev")]
pub fn stub_0xc9ef40() -> crate::slot::PortedFn {
// IDA 0xc9ef40: Ogre::GpuProgram::CmdPose::~CmdPose().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef40, "Ogre::GpuProgram::CmdPose::~CmdPose()")
}

// 0xc9ef44 — __ZN4Ogre10GpuProgram6CmdVTFD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdVTF *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdVTF::~CmdVTF()")]
#[doc(alias = "__ZN4Ogre10GpuProgram6CmdVTFD1Ev")]
pub fn stub_0xc9ef44() -> crate::slot::PortedFn {
// IDA 0xc9ef44: Ogre::GpuProgram::CmdVTF::~CmdVTF().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef44, "Ogre::GpuProgram::CmdVTF::~CmdVTF()")
}

// 0xc9ef48 — __ZN4Ogre10GpuProgram24CmdManualNamedConstsFileD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdManualNamedConstsFile *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdManualNamedConstsFile::~CmdManualNamedConstsFile()")]
#[doc(alias = "__ZN4Ogre10GpuProgram24CmdManualNamedConstsFileD1Ev")]
pub fn stub_0xc9ef48() -> crate::slot::PortedFn {
// IDA 0xc9ef48: Ogre::GpuProgram::CmdManualNamedConstsFile::~CmdManualNamedConstsFile().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef48, "Ogre::GpuProgram::CmdManualNamedConstsFile::~CmdManualNamedConstsFile()")
}

// 0xc9ef4c — __ZN4Ogre10GpuProgram12CmdAdjacencyD1Ev
// type: void __fastcall(Ogre::GpuProgram::CmdAdjacency *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdAdjacency::~CmdAdjacency()")]
#[doc(alias = "__ZN4Ogre10GpuProgram12CmdAdjacencyD1Ev")]
pub fn stub_0xc9ef4c() -> crate::slot::PortedFn {
// IDA 0xc9ef4c: Ogre::GpuProgram::CmdAdjacency::~CmdAdjacency().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9ef4c, "Ogre::GpuProgram::CmdAdjacency::~CmdAdjacency()")
}

// 0xc9ef50 — __ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct>::operator=(Ogre::SharedPtr<Ogre::GpuLogicalBufferStruct> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_22GpuLogicalBufferStructEEaSERKS2_")]
pub fn stub_0xc9ef50(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0xc9f05c — __ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEEaSERKS2_
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuNamedConstants>::operator=(Ogre::SharedPtr<Ogre::GpuNamedConstants> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_17GpuNamedConstantsEEaSERKS2_")]
pub fn stub_0xc9f05c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0xc9f168 — __ZN4Ogre10GpuProgram7CmdTypeD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdType *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdType::~CmdType() [0xc9f168]")]
#[doc(alias = "__ZN4Ogre10GpuProgram7CmdTypeD0Ev")]
pub fn stub_0xc9f168() -> crate::slot::PortedFn {
// IDA 0xc9f168: Ogre::GpuProgram::CmdType::~CmdType().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f168, "Ogre::GpuProgram::CmdType::~CmdType()")
}

// 0xc9f174 — __ZN4Ogre10GpuProgram9CmdSyntaxD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdSyntax *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdSyntax::~CmdSyntax() [0xc9f174]")]
#[doc(alias = "__ZN4Ogre10GpuProgram9CmdSyntaxD0Ev")]
pub fn stub_0xc9f174() -> crate::slot::PortedFn {
// IDA 0xc9f174: Ogre::GpuProgram::CmdSyntax::~CmdSyntax().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f174, "Ogre::GpuProgram::CmdSyntax::~CmdSyntax()")
}

// 0xc9f180 — __ZN4Ogre10GpuProgram11CmdSkeletalD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdSkeletal *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdSkeletal::~CmdSkeletal() [0xc9f180]")]
#[doc(alias = "__ZN4Ogre10GpuProgram11CmdSkeletalD0Ev")]
pub fn stub_0xc9f180() -> crate::slot::PortedFn {
// IDA 0xc9f180: Ogre::GpuProgram::CmdSkeletal::~CmdSkeletal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f180, "Ogre::GpuProgram::CmdSkeletal::~CmdSkeletal()")
}

// 0xc9f18c — __ZN4Ogre10GpuProgram8CmdMorphD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdMorph *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdMorph::~CmdMorph() [0xc9f18c]")]
#[doc(alias = "__ZN4Ogre10GpuProgram8CmdMorphD0Ev")]
pub fn stub_0xc9f18c() -> crate::slot::PortedFn {
// IDA 0xc9f18c: Ogre::GpuProgram::CmdMorph::~CmdMorph().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f18c, "Ogre::GpuProgram::CmdMorph::~CmdMorph()")
}

// 0xc9f198 — __ZN4Ogre10GpuProgram7CmdPoseD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdPose *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdPose::~CmdPose() [0xc9f198]")]
#[doc(alias = "__ZN4Ogre10GpuProgram7CmdPoseD0Ev")]
pub fn stub_0xc9f198() -> crate::slot::PortedFn {
// IDA 0xc9f198: Ogre::GpuProgram::CmdPose::~CmdPose().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f198, "Ogre::GpuProgram::CmdPose::~CmdPose()")
}

// 0xc9f1a4 — __ZN4Ogre10GpuProgram6CmdVTFD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdVTF *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdVTF::~CmdVTF() [0xc9f1a4]")]
#[doc(alias = "__ZN4Ogre10GpuProgram6CmdVTFD0Ev")]
pub fn stub_0xc9f1a4() -> crate::slot::PortedFn {
// IDA 0xc9f1a4: Ogre::GpuProgram::CmdVTF::~CmdVTF().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f1a4, "Ogre::GpuProgram::CmdVTF::~CmdVTF()")
}

// 0xc9f1b0 — __ZN4Ogre10GpuProgram24CmdManualNamedConstsFileD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdManualNamedConstsFile *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdManualNamedConstsFile::~CmdManualNamedConstsFile() [0xc9f1b0]")]
#[doc(alias = "__ZN4Ogre10GpuProgram24CmdManualNamedConstsFileD0Ev")]
pub fn stub_0xc9f1b0() -> crate::slot::PortedFn {
// IDA 0xc9f1b0: Ogre::GpuProgram::CmdManualNamedConstsFile::~CmdManualNamedConstsFile().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f1b0, "Ogre::GpuProgram::CmdManualNamedConstsFile::~CmdManualNamedConstsFile()")
}

// 0xc9f1bc — __ZN4Ogre10GpuProgram12CmdAdjacencyD0Ev
// type: void __fastcall(Ogre::GpuProgram::CmdAdjacency *__hidden this)
#[doc(alias = "Ogre::GpuProgram::CmdAdjacency::~CmdAdjacency() [0xc9f1bc]")]
#[doc(alias = "__ZN4Ogre10GpuProgram12CmdAdjacencyD0Ev")]
pub fn stub_0xc9f1bc() -> crate::slot::PortedFn {
// IDA 0xc9f1bc: Ogre::GpuProgram::CmdAdjacency::~CmdAdjacency().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f1bc, "Ogre::GpuProgram::CmdAdjacency::~CmdAdjacency()")
}

// 0xc9f1c8 — __ZN4Ogre10GpuProgramD1Ev
// type: void __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::~GpuProgram()")]
#[doc(alias = "__ZN4Ogre10GpuProgramD1Ev")]
pub fn stub_0xc9f1c8() -> crate::slot::PortedFn {
// IDA 0xc9f1c8: Ogre::GpuProgram::~GpuProgram().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f1c8, "Ogre::GpuProgram::~GpuProgram()")
}

// 0xc9f1d4 — __ZN4Ogre10GpuProgramD0Ev
// type: void __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::~GpuProgram() [0xc9f1d4]")]
#[doc(alias = "__ZN4Ogre10GpuProgramD0Ev")]
pub fn stub_0xc9f1d4() -> crate::slot::PortedFn {
// IDA 0xc9f1d4: Ogre::GpuProgram::~GpuProgram().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f1d4, "Ogre::GpuProgram::~GpuProgram()")
}

// 0xc9f264 — __ZN4Ogre10GpuProgram19_getBindingDelegateEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::_getBindingDelegate(void)")]
#[doc(alias = "__ZN4Ogre10GpuProgram19_getBindingDelegateEv")]
pub fn stub_0xc9f264() -> crate::slot::PortedFn {
// IDA 0xc9f264: Ogre::GpuProgram::_getBindingDelegate().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f264, "Ogre::GpuProgram::_getBindingDelegate()")
}

// 0xc9f268 — __ZNK4Ogre10GpuProgram17getNamedConstantsEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::getNamedConstants(void)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram17getNamedConstantsEv")]
pub fn stub_0xc9f268() -> crate::slot::PortedFn {
// IDA 0xc9f268: Ogre::GpuProgram::getNamedConstants() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f268, "Ogre::GpuProgram::getNamedConstants() const")
}

// 0xc9f270 — __ZNK4Ogre10GpuProgram22getConstantDefinitionsEv
// type: _DWORD __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::getConstantDefinitions(void)const")]
#[doc(alias = "__ZNK4Ogre10GpuProgram22getConstantDefinitionsEv")]
pub fn stub_0xc9f270() -> crate::slot::PortedFn {
// IDA 0xc9f270: Ogre::GpuProgram::getConstantDefinitions() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f270, "Ogre::GpuProgram::getConstantDefinitions() const")
}

// 0xc9f278 — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryC")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS4_")]
pub fn stub_0xc9f278(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

// 0xc9f37c — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryC [0xc9f37c]")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev")]
pub fn stub_0xc9f37c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

// 0xc9f380 — __ZN4Ogre10GpuProgramD2Ev
// type: void __fastcall(Ogre::GpuProgram *__hidden this)
#[doc(alias = "Ogre::GpuProgram::~GpuProgram() [0xc9f380]")]
#[doc(alias = "__ZN4Ogre10GpuProgramD2Ev")]
pub fn stub_0xc9f380() -> crate::slot::PortedFn {
// IDA 0xc9f380: Ogre::GpuProgram::~GpuProgram().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f380, "Ogre::GpuProgram::~GpuProgram()")
}

// 0xc9f5bc — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCa")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED1Ev")]
pub fn stub_0xc9f5bc(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

// 0xc9f5c0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCa [0xc9f5c0]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE7_M_copyEPKSt13_Rb_tree_nodeIS4_EPSG_")]
pub fn stub_0xc9f5c0() -> crate::slot::PortedFn {
// IDA 0xc9f5c0: std::_Rb_tree<std::string, std::pair<std::string const, Ogre::GpuConstantDefinition>, std::_Select1st<std::pair<std::str~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f5c0, "std::_Rb_tree<std::string, std::pair<std::string const, Ogre::GpuConstantDefinition>, std::_Select1s~")
}

// 0xc9f7d0 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::GpuConstantDefinition>,std::_Select1st<std::pair<std::string const,Ogre::GpuConstantDefinition>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::GpuConstantDefinition>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCa [0xc9f7d0]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre21GpuConstantDefinitionEESt10_Select1stIS4_ESt4lessISsENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev")]
pub fn stub_0xc9f7d0(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

// 0xc9f7dc — __ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev
#[doc(alias = "std::_Rb_tree<unsigned long,std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,std::_Select1st<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>>,std::less<unsigned long>,Ogre::STLAllocator<std::pair<unsigned long const,Ogre::GpuLogicalIndexUse>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryC [0xc9f7dc]")]
#[doc(alias = "__ZNSt8_Rb_treeImSt4pairIKmN4Ogre18GpuLogicalIndexUseEESt10_Select1stIS4_ESt4lessImENS2_12STLAllocatorIS4_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS8_Lb0EED0Ev")]
pub fn stub_0xc9f7dc(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

// 0xc9f8fc — __ZN4Ogre17GpuProgramManager12getSingletonEv
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::getSingleton(void)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager12getSingletonEv")]
pub fn stub_0xc9f8fc() -> crate::slot::PortedFn {
// IDA 0xc9f8fc: Ogre::GpuProgramManager::getSingleton().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f8fc, "Ogre::GpuProgramManager::getSingleton()")
}

// 0xc9f90c — __ZN4Ogre17GpuProgramManagerC2Ev
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::GpuProgramManager(void)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManagerC2Ev")]
pub fn stub_0xc9f90c() -> crate::slot::PortedFn {
// IDA 0xc9f90c: Ogre::GpuProgramManager::GpuProgramManager().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9f90c, "Ogre::GpuProgramManager::GpuProgramManager()")
}

// 0xc9fa90 — __ZN4Ogre17GpuProgramManagerD0Ev
// type: void __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::~GpuProgramManager()")]
#[doc(alias = "__ZN4Ogre17GpuProgramManagerD0Ev")]
pub fn stub_0xc9fa90() -> crate::slot::PortedFn {
// IDA 0xc9fa90: Ogre::GpuProgramManager::~GpuProgramManager().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9fa90, "Ogre::GpuProgramManager::~GpuProgramManager()")
}

// 0xc9fb6c — __ZN4Ogre17GpuProgramManagerD1Ev
// type: void __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::~GpuProgramManager() [0xc9fb6c]")]
#[doc(alias = "__ZN4Ogre17GpuProgramManagerD1Ev")]
pub fn stub_0xc9fb6c() -> crate::slot::PortedFn {
// IDA 0xc9fb6c: Ogre::GpuProgramManager::~GpuProgramManager().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9fb6c, "Ogre::GpuProgramManager::~GpuProgramManager()")
}

// 0xc9fc3c — __ZN4Ogre17GpuProgramManagerD2Ev
// type: void __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::~GpuProgramManager() [0xc9fc3c]")]
#[doc(alias = "__ZN4Ogre17GpuProgramManagerD2Ev")]
pub fn stub_0xc9fc3c() -> crate::slot::PortedFn {
// IDA 0xc9fc3c: Ogre::GpuProgramManager::~GpuProgramManager().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9fc3c, "Ogre::GpuProgramManager::~GpuProgramManager()")
}

// 0xc9fd0c — __ZN4Ogre17GpuProgramManager4loadERKSsS2_S2_NS_14GpuProgramTypeES2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int
#[doc(alias = "Ogre::GpuProgramManager::load(std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::string const&)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager4loadERKSsS2_S2_NS_14GpuProgramTypeES2_")]
pub fn stub_0xc9fd0c() -> crate::slot::PortedFn {
// IDA 0xc9fd0c: Ogre::GpuProgramManager::load(std::string const&, std::string const&, std::string const&, Ogre::GpuProgramType, std::str~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xc9fd0c, "Ogre::GpuProgramManager::load(std::string const&, std::string const&, std::string const&, Ogre::GpuP~")
}

// 0xca009c — __ZN4Ogre17GpuProgramManager9getByNameERKSsb
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *, bool)
#[doc(alias = "Ogre::GpuProgramManager::getByName(std::string const&,bool)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager9getByNameERKSsb")]
pub fn stub_0xca009c() -> crate::slot::PortedFn {
// IDA 0xca009c: Ogre::GpuProgramManager::getByName(std::string const&, bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca009c, "Ogre::GpuProgramManager::getByName(std::string const&, bool)")
}

// 0xca0418 — __ZN4Ogre17GpuProgramManager14loadFromStringERKSsS2_S2_NS_14GpuProgramTypeES2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int
#[doc(alias = "Ogre::GpuProgramManager::loadFromString(std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::string const&)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager14loadFromStringERKSsS2_S2_NS_14GpuProgramTypeES2_")]
pub fn stub_0xca0418() -> crate::slot::PortedFn {
// IDA 0xca0418: Ogre::GpuProgramManager::loadFromString(std::string const&, std::string const&, std::string const&, Ogre::GpuProgramType~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca0418, "Ogre::GpuProgramManager::loadFromString(std::string const&, std::string const&, std::string const&, ~")
}

// 0xca07a8 — __ZN4Ogre17GpuProgramManager6createERKSsS2_NS_14GpuProgramTypeES2_bPNS_20ManualResourceLoaderE
// type: int __fastcall(int, Ogre::ResourceManager *this, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int)
#[doc(alias = "Ogre::GpuProgramManager::create(std::string const&,std::string const&,Ogre::GpuProgramType,std::string const&,bool,Ogre::ManualResourceLoader *)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager6createERKSsS2_NS_14GpuProgramTypeES2_bPNS_20ManualResourceLoaderE")]
pub fn stub_0xca07a8() -> crate::slot::PortedFn {
// IDA 0xca07a8: Ogre::GpuProgramManager::create(std::string const&, std::string const&, Ogre::GpuProgramType, std::string const&, bool, ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca07a8, "Ogre::GpuProgramManager::create(std::string const&, std::string const&, Ogre::GpuProgramType, std::s~")
}

// 0xca0944 — __ZN4Ogre17GpuProgramManager13createProgramERKSsS2_S2_NS_14GpuProgramTypeES2_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::GpuProgramManager::createProgram(std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::string const&)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager13createProgramERKSsS2_S2_NS_14GpuProgramTypeES2_")]
pub fn stub_0xca0944() -> crate::slot::PortedFn {
// IDA 0xca0944: Ogre::GpuProgramManager::createProgram(std::string const&, std::string const&, std::string const&, Ogre::GpuProgramType,~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca0944, "Ogre::GpuProgramManager::createProgram(std::string const&, std::string const&, std::string const&, O~")
}

// 0xca0b1c — __ZN4Ogre17GpuProgramManager23createProgramFromStringERKSsS2_S2_NS_14GpuProgramTypeES2_
// type: void __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int)
#[doc(alias = "Ogre::GpuProgramManager::createProgramFromString(std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::string const&)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager23createProgramFromStringERKSsS2_S2_NS_14GpuProgramTypeES2_")]
pub fn stub_0xca0b1c() -> crate::slot::PortedFn {
// IDA 0xca0b1c: Ogre::GpuProgramManager::createProgramFromString(std::string const&, std::string const&, std::string const&, Ogre::GpuPr~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca0b1c, "Ogre::GpuProgramManager::createProgramFromString(std::string const&, std::string const&, std::string~")
}

// 0xca0cf4 — __ZNK4Ogre17GpuProgramManager18getSupportedSyntaxEv
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::getSupportedSyntax(void)const")]
#[doc(alias = "__ZNK4Ogre17GpuProgramManager18getSupportedSyntaxEv")]
pub fn stub_0xca0cf4() -> crate::slot::PortedFn {
// IDA 0xca0cf4: Ogre::GpuProgramManager::getSupportedSyntax() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca0cf4, "Ogre::GpuProgramManager::getSupportedSyntax() const")
}

// 0xca0d08 — __ZNK4Ogre17GpuProgramManager17isSyntaxSupportedERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::isSyntaxSupported(std::string const&)const")]
#[doc(alias = "__ZNK4Ogre17GpuProgramManager17isSyntaxSupportedERKSs")]
pub fn stub_0xca0d08() -> crate::slot::PortedFn {
// IDA 0xca0d08: Ogre::GpuProgramManager::isSyntaxSupported(std::string const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca0d08, "Ogre::GpuProgramManager::isSyntaxSupported(std::string const&) const")
}

// 0xca0d34 — __ZN4Ogre17GpuProgramManager16createParametersEv
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::createParameters(void)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager16createParametersEv")]
pub fn stub_0xca0d34() -> crate::slot::PortedFn {
// IDA 0xca0d34: Ogre::GpuProgramManager::createParameters().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca0d34, "Ogre::GpuProgramManager::createParameters()")
}

// 0xca0e2c — __ZN4Ogre17GpuProgramManager22createSharedParametersERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::createSharedParameters(std::string const&)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager22createSharedParametersERKSs")]
pub fn stub_0xca0e2c() -> crate::slot::PortedFn {
// IDA 0xca0e2c: Ogre::GpuProgramManager::createSharedParameters(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca0e2c, "Ogre::GpuProgramManager::createSharedParameters(std::string const&)")
}

// 0xca1260 — __ZNK4Ogre17GpuProgramManager19getSharedParametersERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::getSharedParameters(std::string const&)const")]
#[doc(alias = "__ZNK4Ogre17GpuProgramManager19getSharedParametersERKSs")]
pub fn stub_0xca1260() -> crate::slot::PortedFn {
// IDA 0xca1260: Ogre::GpuProgramManager::getSharedParameters(std::string const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca1260, "Ogre::GpuProgramManager::getSharedParameters(std::string const&) const")
}

// 0xca1574 — __ZNK4Ogre17GpuProgramManager28getAvailableSharedParametersEv
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::getAvailableSharedParameters(void)const")]
#[doc(alias = "__ZNK4Ogre17GpuProgramManager28getAvailableSharedParametersEv")]
pub fn stub_0xca1574() -> crate::slot::PortedFn {
// IDA 0xca1574: Ogre::GpuProgramManager::getAvailableSharedParameters() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca1574, "Ogre::GpuProgramManager::getAvailableSharedParameters() const")
}

// 0xca1578 — __ZN4Ogre17GpuProgramManager24getSaveMicrocodesToCacheEv
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::getSaveMicrocodesToCache(void)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager24getSaveMicrocodesToCacheEv")]
pub fn stub_0xca1578() -> crate::slot::PortedFn {
// IDA 0xca1578: Ogre::GpuProgramManager::getSaveMicrocodesToCache().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca1578, "Ogre::GpuProgramManager::getSaveMicrocodesToCache()")
}

// 0xca1580 — __ZN4Ogre17GpuProgramManager26canGetCompiledShaderBufferEv
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this)
#[doc(alias = "Ogre::GpuProgramManager::canGetCompiledShaderBuffer(void)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager26canGetCompiledShaderBufferEv")]
pub fn stub_0xca1580() -> crate::slot::PortedFn {
// IDA 0xca1580: Ogre::GpuProgramManager::canGetCompiledShaderBuffer().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca1580, "Ogre::GpuProgramManager::canGetCompiledShaderBuffer()")
}

// 0xca159c — __ZN4Ogre17GpuProgramManager21addRenderSystemToNameERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::addRenderSystemToName(std::string const&)")]
#[doc(alias = "__ZN4Ogre17GpuProgramManager21addRenderSystemToNameERKSs")]
pub fn stub_0xca159c() -> crate::slot::PortedFn {
// IDA 0xca159c: Ogre::GpuProgramManager::addRenderSystemToName(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca159c, "Ogre::GpuProgramManager::addRenderSystemToName(std::string const&)")
}

// 0xca1764 — __ZNK4Ogre17GpuProgramManager27isMicrocodeAvailableInCacheERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::isMicrocodeAvailableInCache(std::string const&)const")]
#[doc(alias = "__ZNK4Ogre17GpuProgramManager27isMicrocodeAvailableInCacheERKSs")]
pub fn stub_0xca1764() -> crate::slot::PortedFn {
// IDA 0xca1764: Ogre::GpuProgramManager::isMicrocodeAvailableInCache(std::string const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca1764, "Ogre::GpuProgramManager::isMicrocodeAvailableInCache(std::string const&) const")
}

// 0xca17d8 — __ZNK4Ogre17GpuProgramManager21getMicrocodeFromCacheERKSs
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, const std::string *)
#[doc(alias = "Ogre::GpuProgramManager::getMicrocodeFromCache(std::string const&)const")]
#[doc(alias = "__ZNK4Ogre17GpuProgramManager21getMicrocodeFromCacheERKSs")]
pub fn stub_0xca17d8() -> crate::slot::PortedFn {
// IDA 0xca17d8: Ogre::GpuProgramManager::getMicrocodeFromCache(std::string const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca17d8, "Ogre::GpuProgramManager::getMicrocodeFromCache(std::string const&) const")
}

// 0xca183c — __ZNK4Ogre17GpuProgramManager15createMicrocodeEm
// type: _DWORD __fastcall(Ogre::GpuProgramManager *__hidden this, unsigned int)
#[doc(alias = "Ogre::GpuProgramManager::createMicrocode(unsigned long)const")]
#[doc(alias = "__ZNK4Ogre17GpuProgramManager15createMicrocodeEm")]
pub fn stub_0xca183c() -> crate::slot::PortedFn {
// IDA 0xca183c: Ogre::GpuProgramManager::createMicrocode(unsigned long) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xca183c, "Ogre::GpuProgramManager::createMicrocode(unsigned long) const")
}

// 0xca2830 — __ZNSt3mapISsN4Ogre9SharedPtrINS0_19GpuSharedParametersEEESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
#[doc(alias = "std::map<std::string,Ogre::SharedPtr<Ogre::GpuSharedParameters>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN4Ogre9SharedPtrINS0_19GpuSharedParametersEEESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_")]
pub fn stub_0xca2830() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca2ad0 — __ZN4Ogre9SharedPtrINS_19GpuSharedParametersEEaSERKS2_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuSharedParameters>::operator=(Ogre::SharedPtr<Ogre::GpuSharedParameters> const&)")]
#[doc(alias = "__ZN4Ogre9SharedPtrINS_19GpuSharedParametersEEaSERKS2_")]
pub fn stub_0xca2ad0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0xca3168 — __ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>")]
#[doc(alias = "__ZNKSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
pub fn stub_0xca3168() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca320c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uni
// type: int __fastcall(int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca320c]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_")]
pub fn stub_0xca320c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca3554 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt1
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca3554]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSI_RKS6_")]
pub fn stub_0xca3554() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca35c8 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uni
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca35c8]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS6_")]
pub fn stub_0xca35c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca36ac — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nod
// type: _DWORD *__fastcall(int, const std::string *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca36ac]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS6_")]
pub fn stub_0xca36ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca37c4 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca37c4]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_")]
pub fn stub_0xca37c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca3878 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_impl
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca3878]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED1Ev")]
pub fn stub_0xca3878(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0xca387c — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_impl
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca387c]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implISA_Lb0EED0Ev")]
pub fn stub_0xca387c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0xca3888 — __ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,std::_Select1st<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>> [0xca3888]")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN4Ogre9SharedPtrINS2_19GpuSharedParametersEEEESt10_Select1stIS6_ESt4lessISsENS2_12STLAllocatorIS6_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS6_E")]
pub fn stub_0xca3888() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}

// 0xca38b8 — __ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_19GpuSharedParametersEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_
#[doc(alias = "Ogre::STLAllocator<std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(std::pair<std::string const,Ogre::SharedPtr<Ogre::GpuSharedParameters>>*)")]
#[doc(alias = "__ZN4Ogre12STLAllocatorISt4pairIKSsNS_9SharedPtrINS_19GpuSharedParametersEEEENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS6_")]
pub fn stub_0xca38b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Ogre::GpuSharedParameters")
}
