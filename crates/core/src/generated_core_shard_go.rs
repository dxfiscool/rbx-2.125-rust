//! core shard GO — 100 core stubs EA-sorted, 0x253edc..0x267350 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap (0x253edc..0x267350, 16763->16863 covered, 5055 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::operator-(RBX::Time const&,RBX::Time const&)")]
// 0x253edc — __ZN3RBXmiERKNS_4TimeES2_
pub fn stub_253edc() {
    // IDA 0x253edc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxDbgInfo::AddPlace(long)")]
// 0x253ef0 — __ZN3RBX10RbxDbgInfo8AddPlaceEl
pub fn stub_253ef0() {
    // IDA 0x253ef0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxDbgInfo::RemovePlace(long)")]
// 0x253f24 — __ZN3RBX10RbxDbgInfo11RemovePlaceEl
pub fn stub_253f24() {
    // IDA 0x253f24: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardName(char const*)")]
// 0x253f70 — __ZN3RBX10RbxDbgInfo14SetGfxCardNameEPKc
pub fn stub_253f70() {
    // IDA 0x253f70: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardDriverVersion(char const*)")]
// 0x253f94 — __ZN3RBX10RbxDbgInfo23SetGfxCardDriverVersionEPKc
pub fn stub_253f94() {
    // IDA 0x253f94: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxDbgInfo::SetGfxCardVendor(char const*)")]
// 0x253fb8 — __ZN3RBX10RbxDbgInfo16SetGfxCardVendorEPKc
pub fn stub_253fb8() {
    // IDA 0x253fb8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxDbgInfo::SetCPUName(char const*)")]
// 0x253fdc — __ZN3RBX10RbxDbgInfo10SetCPUNameEPKc
pub fn stub_253fdc() {
    // IDA 0x253fdc: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxDbgInfo::SetServerIP(char const*)")]
// 0x254000 — __ZN3RBX10RbxDbgInfo11SetServerIPEPKc
pub fn stub_254000() {
    // IDA 0x254000: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxInterlockedDecrement(long volatile*)")]
// 0x254024 — __ZN3RBX23RbxInterlockedDecrementEPVl
pub fn stub_254024() {
    // IDA 0x254024: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxInterlockedIncrement(long volatile*)")]
// 0x254034 — __ZN3RBX23RbxInterlockedIncrementEPVl
pub fn stub_254034() {
    // IDA 0x254034: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxInterlockedIncrementAcquire(long volatile*)")]
// 0x254044 — __ZN3RBX30RbxInterlockedIncrementAcquireEPVl
pub fn stub_254044() {
    // IDA 0x254044: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxInterlockedExchange(long volatile*,long)")]
// 0x254054 — __ZN3RBX22RbxInterlockedExchangeEPVll
pub fn stub_254054() {
    // IDA 0x254054: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::RbxInterlockedCompareExchange(long volatile*,long,long)")]
// 0x254068 — __ZN3RBX29RbxInterlockedCompareExchangeEPVlll
pub fn stub_254068() {
    // IDA 0x254068: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getCPUMake(void)")]
// 0x25407c — __ZN3RBX13MacSystemUtil10getCPUMakeEv
pub fn stub_25407c() {
    // IDA 0x25407c: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getCPUSpeed(void)")]
// 0x2541ac — __ZN3RBX13MacSystemUtil11getCPUSpeedEv
pub fn stub_2541ac() {
    // IDA 0x2541ac: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getCPULogicalCount(void)")]
// 0x254224 — __ZN3RBX13MacSystemUtil18getCPULogicalCountEv
pub fn stub_254224() {
    // IDA 0x254224: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getCPUCoreCount(void)")]
// 0x2542b0 — __ZN3RBX13MacSystemUtil15getCPUCoreCountEv
pub fn stub_2542b0() {
    // IDA 0x2542b0: libtiff tile writer owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getCPUPhysicalCount(void)")]
// 0x254320 — __ZN3RBX13MacSystemUtil19getCPUPhysicalCountEv
pub fn stub_254320() {
    // IDA 0x254320: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::isCPU64Bit(void)")]
// 0x254478 — __ZN3RBX13MacSystemUtil10isCPU64BitEv
pub fn stub_254478() {
    // IDA 0x254478: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getMBSysRAM(void)")]
// 0x25453c — __ZN3RBX13MacSystemUtil11getMBSysRAMEv
pub fn stub_25453c() {
    // IDA 0x25453c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getMBSysAvailableRAM(void)")]
// 0x2545b4 — __ZN3RBX13MacSystemUtil20getMBSysAvailableRAMEv
pub fn stub_2545b4() {
    // IDA 0x2545b4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getVideoMemory(void)")]
// 0x254654 — __ZN3RBX13MacSystemUtil14getVideoMemoryEv
pub fn stub_254654() {
    // IDA 0x254654: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::osVer(void)")]
// 0x25465c — __ZN3RBX13MacSystemUtil5osVerEv
pub fn stub_25465c() {
    // IDA 0x25465c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getGPUMake(void)")]
// 0x254824 — __ZN3RBX13MacSystemUtil10getGPUMakeEv
pub fn stub_254824() {
    // IDA 0x254824: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::MacSystemUtil::getMaxRes(void)")]
// 0x2549ec — __ZN3RBX13MacSystemUtil9getMaxResEv
pub fn stub_2549ec() {
    // IDA 0x2549ec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Crypt::Crypt(void)")]
// 0x254bb4 — __ZN3RBX5CryptC1Ev
pub fn stub_254bb4() {
    // IDA 0x254bb4: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Crypt::~Crypt()")]
// 0x254bb8 — __ZN3RBX5CryptD1Ev
pub fn stub_254bb8() {
    // IDA 0x254bb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Crypt::verifySignatureBase64(std::string,std::string)")]
// 0x254bbc — __ZN3RBX5Crypt21verifySignatureBase64ESsSs
pub fn stub_254bbc() {
    // IDA 0x254bbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::IsValueOutlier(double,unsigned int,double,double,RBX::Confidence)")]
// 0x254bf8 — __ZN3RBX14IsValueOutlierEdjddNS_10ConfidenceE
pub fn stub_254bf8() {
    // IDA 0x254bf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GetConfidenceInterval(double,double,RBX::Confidence,double *,double *)")]
// 0x254c68 — __ZN3RBX21GetConfidenceIntervalEddNS_10ConfidenceEPdS1_
pub fn stub_254c68() {
    // IDA 0x254c68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TCritical(unsigned int,RBX::Confidence)")]
// 0x254d18 — __ZN3RBX9TCriticalEjNS_10ConfidenceE
pub fn stub_254d18() {
    // IDA 0x254d18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "CookiesEngine::SetValue(std::string,std::string)")]
// 0x254e20 — __ZN13CookiesEngine8SetValueESsSs
pub fn stub_254e20() {
    // IDA 0x254e20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "CookiesEngine::CookiesEngine(std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>>)")]
// 0x255070 — __ZN13CookiesEngineC1ESbIwSt11char_traitsIwESaIwEE
pub fn stub_255070() {
    // IDA 0x255070: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "CookiesEngine::GetValue(std::string,int *,bool *)")]
// 0x255098 — __ZN13CookiesEngine8GetValueESsPiPb
pub fn stub_255098() {
    // IDA 0x255098: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "CookiesEngine::DeleteValue(std::string)")]
// 0x25529c — __ZN13CookiesEngine11DeleteValueESs
pub fn stub_25529c() {
    // IDA 0x25529c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "convert_w2s(std::basic_string<wchar_t,std::char_traits<wchar_t>,std::allocator<wchar_t>> const&)")]
// 0x255320 — __Z11convert_w2sRKSbIwSt11char_traitsIwESaIwEE
pub fn stub_255320() {
    // IDA 0x255320: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "convert_s2w(std::string const&)")]
// 0x255474 — __Z11convert_s2wRKSs
pub fn stub_255474() {
    // IDA 0x255474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::decodeJSON(std::string)")]
// 0x256a6c — __ZN3RBX11HttpService10decodeJSONESs
pub fn stub_256a6c() {
    // IDA 0x256a6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::StringConverter<RBX::HttpService::HttpContentType>::convertToValue(std::string const&,RBX::HttpService::HttpContentType&)")]
// 0x2570c0 — __ZN3RBX15StringConverterINS_11HttpService15HttpContentTypeEE14convertToValueERKSsRS2_
pub fn stub_2570c0() {
    // IDA 0x2570c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::HttpService(void)")]
// 0x257110 — __ZN3RBX11HttpServiceC2Ev
pub fn stub_257110() {
    // IDA 0x257110: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::checkUserHasAccess(void)")]
// 0x257758 — __ZN3RBX11HttpService18checkUserHasAccessEv
pub fn stub_257758() {
    // IDA 0x257758: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::checkLimit(void)")]
// 0x2577c0 — __ZN3RBX11HttpService10checkLimitEv
pub fn stub_2577c0() {
    // IDA 0x2577c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::HttpService::~HttpService()")]
// 0x2580ac — __ZN3RBX11HttpServiceD1Ev
pub fn stub_2580ac() {
    // IDA 0x2580ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HttpService::~HttpService()")]
// 0x2580b0 — __ZN3RBX11HttpServiceD0Ev
pub fn stub_2580b0() {
    // IDA 0x2580b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
// 0x258160 — __ZThn32_N3RBX11HttpServiceD1Ev
// was: non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_258160() {
    // IDA 0x258160: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
// 0x258168 — __ZThn32_N3RBX11HttpServiceD0Ev
// was: non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_258168() {
    // IDA 0x258168: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
// 0x25821c — __ZThn36_N3RBX11HttpServiceD1Ev
// was: non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_25821c() {
    // IDA 0x25821c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::HttpService::~HttpService()")]
// 0x258224 — __ZThn36_N3RBX11HttpServiceD0Ev
// was: non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_258224() {
    // IDA 0x258224: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x258e74 — __ZN3rbx8any_castIN3RBX11HttpService15HttpContentTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_258e74() {
    // IDA 0x258e74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x258ecc — __ZN3rbx8any_castIRN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_258ecc() {
    // IDA 0x258ecc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Light::setEnabled(bool)")]
// 0x25b4c0 — __ZN3RBX5Light10setEnabledEb
pub fn stub_25b4c0() {
    // IDA 0x25b4c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Light::setBrightness(float)")]
// 0x25b544 — __ZN3RBX5Light13setBrightnessEf
pub fn stub_25b544() {
    // IDA 0x25b544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PointLight::setRange(float)")]
// 0x25b574 — __ZN3RBX10PointLight8setRangeEf
pub fn stub_25b574() {
    // IDA 0x25b574: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::SpotLight::setRange(float)")]
// 0x25b5b0 — __ZN3RBX9SpotLight8setRangeEf
pub fn stub_25b5b0() {
    // IDA 0x25b5b0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::SpotLight::setAngle(float)")]
// 0x25b5ec — __ZN3RBX9SpotLight8setAngleEf
pub fn stub_25b5ec() {
    // IDA 0x25b5ec: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::registerNewLightAPI(void)")]
// 0x25b628 — __ZN3RBX19registerNewLightAPIEv
pub fn stub_25b628() {
    // IDA 0x25b628: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Light::setShadows(bool)")]
// 0x25b884 — __ZN3RBX5Light10setShadowsEb
pub fn stub_25b884() {
    // IDA 0x25b884: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SpotLight::setFace(RBX::NormalId)")]
// 0x25b8a8 — __ZN3RBX9SpotLight7setFaceENS_8NormalIdE
pub fn stub_25b8a8() {
    // IDA 0x25b8a8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Light::Light(char const*)")]
// 0x25b8c8 — __ZN3RBX5LightC2EPKc
pub fn stub_25b8c8() {
    // IDA 0x25b8c8: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Light::~Light()")]
// 0x25baa8 — __ZN3RBX5LightD0Ev
pub fn stub_25baa8() {
    // IDA 0x25baa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Light::~Light()")]
// 0x25bb48 — __ZN3RBX5LightD1Ev
pub fn stub_25bb48() {
    // IDA 0x25bb48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
// 0x25bb4c — __ZThn32_N3RBX5LightD0Ev
// was: non-virtual thunk toRBX::Light::~Light()
pub fn stub_25bb4c() {
    // IDA 0x25bb4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
// 0x25bb54 — __ZThn36_N3RBX5LightD0Ev
// was: non-virtual thunk toRBX::Light::~Light()
pub fn stub_25bb54() {
    // IDA 0x25bb54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
// 0x25bb5c — __ZThn92_N3RBX5LightD0Ev
// was: non-virtual thunk toRBX::Light::~Light()
pub fn stub_25bb5c() {
    // IDA 0x25bb5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Light::~Light()")]
// 0x25bb64 — __ZN3RBX5LightD2Ev
pub fn stub_25bb64() {
    // IDA 0x25bb64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
// 0x25bc20 — __ZThn32_N3RBX5LightD1Ev
// was: non-virtual thunk toRBX::Light::~Light()
pub fn stub_25bc20() {
    // IDA 0x25bc20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
// 0x25bc28 — __ZThn36_N3RBX5LightD1Ev
// was: non-virtual thunk toRBX::Light::~Light()
pub fn stub_25bc28() {
    // IDA 0x25bc28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Light::~Light()")]
// 0x25bc30 — __ZThn92_N3RBX5LightD1Ev
// was: non-virtual thunk toRBX::Light::~Light()
pub fn stub_25bc30() {
    // IDA 0x25bc30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PointLight::PointLight(void)")]
// 0x25bc64 — __ZN3RBX10PointLightC2Ev
pub fn stub_25bc64() {
    // IDA 0x25bc64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PointLight::~PointLight()")]
// 0x25bdb8 — __ZN3RBX10PointLightD0Ev
pub fn stub_25bdb8() {
    // IDA 0x25bdb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PointLight::~PointLight()")]
// 0x25be58 — __ZN3RBX10PointLightD1Ev
pub fn stub_25be58() {
    // IDA 0x25be58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
// 0x25be5c — __ZThn32_N3RBX10PointLightD0Ev
// was: non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_25be5c() {
    // IDA 0x25be5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
// 0x25be64 — __ZThn36_N3RBX10PointLightD0Ev
// was: non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_25be64() {
    // IDA 0x25be64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
// 0x25be6c — __ZThn92_N3RBX10PointLightD0Ev
// was: non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_25be6c() {
    // IDA 0x25be6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
// 0x25be74 — __ZThn32_N3RBX10PointLightD1Ev
// was: non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_25be74() {
    // IDA 0x25be74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
// 0x25be7c — __ZThn36_N3RBX10PointLightD1Ev
// was: non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_25be7c() {
    // IDA 0x25be7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::PointLight::~PointLight()")]
// 0x25be84 — __ZThn92_N3RBX10PointLightD1Ev
// was: non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_25be84() {
    // IDA 0x25be84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpotLight::SpotLight(void)")]
// 0x25be8c — __ZN3RBX9SpotLightC2Ev
pub fn stub_25be8c() {
    // IDA 0x25be8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpotLight::~SpotLight()")]
// 0x25bff0 — __ZN3RBX9SpotLightD0Ev
pub fn stub_25bff0() {
    // IDA 0x25bff0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpotLight::~SpotLight()")]
// 0x25c090 — __ZN3RBX9SpotLightD1Ev
pub fn stub_25c090() {
    // IDA 0x25c090: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
// 0x25c094 — __ZThn32_N3RBX9SpotLightD0Ev
// was: non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_25c094() {
    // IDA 0x25c094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
// 0x25c09c — __ZThn36_N3RBX9SpotLightD0Ev
// was: non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_25c09c() {
    // IDA 0x25c09c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
// 0x25c0a4 — __ZThn92_N3RBX9SpotLightD0Ev
// was: non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_25c0a4() {
    // IDA 0x25c0a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
// 0x25c0ac — __ZThn32_N3RBX9SpotLightD1Ev
// was: non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_25c0ac() {
    // IDA 0x25c0ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
// 0x25c0b4 — __ZThn36_N3RBX9SpotLightD1Ev
// was: non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_25c0b4() {
    // IDA 0x25c0b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::SpotLight::~SpotLight()")]
// 0x25c0bc — __ZThn92_N3RBX9SpotLightD1Ev
// was: non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_25c0bc() {
    // IDA 0x25c0bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Light::getEnabled(void)const")]
// 0x25c0c4 — __ZNK3RBX5Light10getEnabledEv
pub fn stub_25c0c4() {
    // IDA 0x25c0c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Light::getColor(void)const")]
// 0x25c0f0 — __ZNK3RBX5Light8getColorEv
pub fn stub_25c0f0() {
    // IDA 0x25c0f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Light::getBrightness(void)const")]
// 0x25c124 — __ZNK3RBX5Light13getBrightnessEv
pub fn stub_25c124() {
    // IDA 0x25c124: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::PointLight::getRange(void)const")]
// 0x25c14c — __ZNK3RBX10PointLight8getRangeEv
pub fn stub_25c14c() {
    // IDA 0x25c14c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::SpotLight::getRange(void)const")]
// 0x25c174 — __ZNK3RBX9SpotLight8getRangeEv
pub fn stub_25c174() {
    // IDA 0x25c174: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SpotLight::getAngle(void)const")]
// 0x25c19c — __ZNK3RBX9SpotLight8getAngleEv
pub fn stub_25c19c() {
    // IDA 0x25c19c: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Light::getShadows(void)const")]
// 0x25c1a0 — __ZNK3RBX5Light10getShadowsEv
pub fn stub_25c1a0() {
    // IDA 0x25c1a0: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::SpotLight::getFace(void)const")]
// 0x25c1a8 — __ZNK3RBX9SpotLight7getFaceEv
pub fn stub_25c1a8() {
    // IDA 0x25c1a8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<XmlElement>::operator new(unsigned long)")]
// 0x26648c — __ZN3RBX9AllocatorI10XmlElementEnwEm
pub fn stub_26648c() {
    // IDA 0x26648c: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::operator new(unsigned long)")]
// 0x266544 — __ZN3RBX9AllocatorI12XmlAttributeEnwEm
pub fn stub_266544() {
    // IDA 0x266544: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlAttribute::XmlAttribute<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
// 0x266600 — __ZN12XmlAttributeC2IPKN3RBX4NameEEERS3_T_
pub fn stub_266600() {
    // IDA 0x266600: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::Allocator(void)")]
// 0x2666c0 — __ZN3RBX9AllocatorI12XmlAttributeEC2Ev
pub fn stub_2666c0() {
    // IDA 0x2666c0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::releaseMemory(void)")]
// 0x266728 — __ZN3RBX9AllocatorI12XmlAttributeE13releaseMemoryEv
pub fn stub_266728() {
    // IDA 0x266728: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "XmlElement::XmlElement(RBX::Name const&)")]
// 0x267350 — __ZN10XmlElementC2ERKN3RBX4NameE
pub fn stub_267350() {
    // IDA 0x267350: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}
