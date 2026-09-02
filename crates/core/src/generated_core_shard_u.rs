//! core shard U — 100 core stubs EA-sorted, RBX:: not Reflection/DataModel/Ogre/RakNet/Lua.
//! Source: ida/export.json filtered where demangled contains RBX:: but not Reflection/DataModel/Ogre/RakNet/Lua/Instance/Workspace, EA-sorted, next 100 uncovered workspace-wide.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

#[doc(alias = "__ZN3RBX13runtime_errorEPKcz")]
// 0x23e1f8 — RBX::runtime_error(char const*,...)
pub fn stub_0x23e1f8() {
    todo!("0x23e1f8")
}

#[doc(alias = "__ZN3RBX7vformatEPKcPv")]
// 0x23e324 — RBX::vformat(char const*,void *)
pub fn stub_0x23e324() {
    todo!("0x23e324")
}

#[doc(alias = "__ZN3RBX6formatEPKcz")]
// 0x23e50c — RBX::format(char const*,...)
pub fn stub_0x23e50c() {
    todo!("0x23e50c")
}

#[doc(alias = "__ZN3RBX9Debugable7doCrashEv")]
// 0x23e5c0 — RBX::Debugable::doCrash(void)
pub fn stub_0x23e5c0() {
    todo!("0x23e5c0")
}

#[doc(alias = "__ZN3RBX9Debugable7doCrashEPKc")]
// 0x23e5dc — RBX::Debugable::doCrash(char const*)
pub fn stub_0x23e5dc() {
    todo!("0x23e5dc")
}

#[doc(alias = "__ZN3RBX15DebugNameStringC1EPKci")]
// 0x23e638 — RBX::DebugNameString::DebugNameString(char const*,int)
pub fn stub_0x23e638() {
    todo!("0x23e638")
}

#[doc(alias = "__ZN3RBX15DebugNameString16getNameIncrementEPKc")]
// 0x23e644 — RBX::DebugNameString::getNameIncrement(char const*)
pub fn stub_0x23e644() {
    todo!("0x23e644")
}

#[doc(alias = "__ZN3RBX3Log10writeEntryENS0_8SeverityEPKc")]
// 0x23e988 — RBX::Log::writeEntry(RBX::Log::Severity,char const*)
pub fn stub_0x23e988() {
    todo!("0x23e988")
}

#[doc(alias = "__ZN3RBX3Log9formatMemEj")]
// 0x23ea18 — RBX::Log::formatMem(unsigned int)
pub fn stub_0x23ea18() {
    todo!("0x23ea18")
}

#[doc(alias = "__ZN3RBX3Log10formatTimeEd")]
// 0x23eb48 — RBX::Log::formatTime(double)
pub fn stub_0x23eb48() {
    todo!("0x23eb48")
}

#[doc(alias = "__ZN3RBX15set_thread_nameEPKc")]
// 0x23f42c — RBX::set_thread_name(char const*)
pub fn stub_0x23f42c() {
    todo!("0x23f42c")
}

#[doc(alias = "__ZN3RBX13worker_threadD1Ev")]
// 0x2400f4 — RBX::worker_thread::~worker_thread()
pub fn stub_0x2400f4() {
    todo!("0x2400f4")
}

#[doc(alias = "__ZN3RBX13worker_threadD2Ev")]
// 0x240100 — RBX::worker_thread::~worker_thread()
pub fn stub_0x240100() {
    todo!("0x240100")
}

#[doc(alias = "__ZN3RBX13worker_thread4wakeEv")]
// 0x2402c4 — RBX::worker_thread::wake(void)
pub fn stub_0x2402c4() {
    todo!("0x2402c4")
}

#[doc(alias = "__ZN3RBX5mutexC2Ev")]
// 0x248940 — RBX::mutex::mutex(void)
pub fn stub_0x248940() {
    todo!("0x248940")
}

#[doc(alias = "__ZN3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE6sampleENS1_8IntervalE")]
// 0x24ad90 — RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::sample(RBX::Time::Interval)
pub fn stub_0x24ad90() {
    todo!("0x24ad90")
}

#[doc(alias = "__ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")]
// 0x24ae08 — RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)
pub fn stub_0x24ae08() {
    todo!("0x24ae08")
}

#[doc(alias = "__ZN3RBX13WindowAverageIddE6sampleINS_13FOnBeforeDropEEEvdRT_")]
// 0x24b2c8 — void RBX::WindowAverage<double,double>::sample<RBX::FOnBeforeDrop>(double,RBX::FOnBeforeDrop &)
pub fn stub_0x24b2c8() {
    todo!("0x24b2c8")
}

#[doc(alias = "__ZN3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EEC2ENS1_8IntervalE")]
// 0x24b364 — RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::WindowAverageTimeInterval(RBX::Time::Interval)
pub fn stub_0x24b364() {
    todo!("0x24b364")
}

#[doc(alias = "__ZN3RBX13TaskScheduler6ThreadD2Ev")]
// 0x2501bc — RBX::TaskScheduler::Thread::~Thread()
pub fn stub_0x2501bc() {
    todo!("0x2501bc")
}

#[doc(alias = "__ZN3RBX4Time3nowILNS0_12SampleMethodE2EEES0_v")]
// 0x253d50 — RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)2>(void)
pub fn stub_0x253d50() {
    todo!("0x253d50")
}

#[doc(alias = "__ZN3RBX4Time3nowILNS0_12SampleMethodE0EEES0_v")]
// 0x253ea4 — RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)0>(void)
pub fn stub_0x253ea4() {
    todo!("0x253ea4")
}

#[doc(alias = "__ZN3RBX4Time7nowFastEv")]
// 0x253eb0 — RBX::Time::nowFast(void)
pub fn stub_0x253eb0() {
    todo!("0x253eb0")
}

#[doc(alias = "__ZN3RBX4Time10nowFastSecEv")]
// 0x253ebc — RBX::Time::nowFastSec(void)
pub fn stub_0x253ebc() {
    todo!("0x253ebc")
}

#[doc(alias = "__ZN3RBX4Time3nowILNS0_12SampleMethodE1EEES0_v")]
// 0x253ecc — RBX::Time RBX::Time::now<(RBX::Time::SampleMethod)1>(void)
pub fn stub_0x253ecc() {
    todo!("0x253ecc")
}

#[doc(alias = "__ZN3RBXmiERKNS_4TimeES2_")]
// 0x253edc — RBX::operator-(RBX::Time const&,RBX::Time const&)
pub fn stub_0x253edc() {
    todo!("0x253edc")
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo8AddPlaceEl")]
// 0x253ef0 — RBX::RbxDbgInfo::AddPlace(long)
pub fn stub_0x253ef0() {
    todo!("0x253ef0")
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo11RemovePlaceEl")]
// 0x253f24 — RBX::RbxDbgInfo::RemovePlace(long)
pub fn stub_0x253f24() {
    todo!("0x253f24")
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo10SetCPUNameEPKc")]
// 0x253fdc — RBX::RbxDbgInfo::SetCPUName(char const*)
pub fn stub_0x253fdc() {
    todo!("0x253fdc")
}

#[doc(alias = "__ZN3RBX10RbxDbgInfo11SetServerIPEPKc")]
// 0x254000 — RBX::RbxDbgInfo::SetServerIP(char const*)
pub fn stub_0x254000() {
    todo!("0x254000")
}

#[doc(alias = "__ZN3RBX23RbxInterlockedDecrementEPVl")]
// 0x254024 — RBX::RbxInterlockedDecrement(long volatile*)
pub fn stub_0x254024() {
    todo!("0x254024")
}

#[doc(alias = "__ZN3RBX23RbxInterlockedIncrementEPVl")]
// 0x254034 — RBX::RbxInterlockedIncrement(long volatile*)
pub fn stub_0x254034() {
    todo!("0x254034")
}

#[doc(alias = "__ZN3RBX30RbxInterlockedIncrementAcquireEPVl")]
// 0x254044 — RBX::RbxInterlockedIncrementAcquire(long volatile*)
pub fn stub_0x254044() {
    todo!("0x254044")
}

#[doc(alias = "__ZN3RBX22RbxInterlockedExchangeEPVll")]
// 0x254054 — RBX::RbxInterlockedExchange(long volatile*,long)
pub fn stub_0x254054() {
    todo!("0x254054")
}

#[doc(alias = "__ZN3RBX29RbxInterlockedCompareExchangeEPVlll")]
// 0x254068 — RBX::RbxInterlockedCompareExchange(long volatile*,long,long)
pub fn stub_0x254068() {
    todo!("0x254068")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil10getCPUMakeEv")]
// 0x25407c — RBX::MacSystemUtil::getCPUMake(void)
pub fn stub_0x25407c() {
    todo!("0x25407c")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil11getCPUSpeedEv")]
// 0x2541ac — RBX::MacSystemUtil::getCPUSpeed(void)
pub fn stub_0x2541ac() {
    todo!("0x2541ac")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil18getCPULogicalCountEv")]
// 0x254224 — RBX::MacSystemUtil::getCPULogicalCount(void)
pub fn stub_0x254224() {
    todo!("0x254224")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil15getCPUCoreCountEv")]
// 0x2542b0 — RBX::MacSystemUtil::getCPUCoreCount(void)
pub fn stub_0x2542b0() {
    todo!("0x2542b0")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil19getCPUPhysicalCountEv")]
// 0x254320 — RBX::MacSystemUtil::getCPUPhysicalCount(void)
pub fn stub_0x254320() {
    todo!("0x254320")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil10isCPU64BitEv")]
// 0x254478 — RBX::MacSystemUtil::isCPU64Bit(void)
pub fn stub_0x254478() {
    todo!("0x254478")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil11getMBSysRAMEv")]
// 0x25453c — RBX::MacSystemUtil::getMBSysRAM(void)
pub fn stub_0x25453c() {
    todo!("0x25453c")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil20getMBSysAvailableRAMEv")]
// 0x2545b4 — RBX::MacSystemUtil::getMBSysAvailableRAM(void)
pub fn stub_0x2545b4() {
    todo!("0x2545b4")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil14getVideoMemoryEv")]
// 0x254654 — RBX::MacSystemUtil::getVideoMemory(void)
pub fn stub_0x254654() {
    todo!("0x254654")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil5osVerEv")]
// 0x25465c — RBX::MacSystemUtil::osVer(void)
pub fn stub_0x25465c() {
    todo!("0x25465c")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil10getGPUMakeEv")]
// 0x254824 — RBX::MacSystemUtil::getGPUMake(void)
pub fn stub_0x254824() {
    todo!("0x254824")
}

#[doc(alias = "__ZN3RBX13MacSystemUtil9getMaxResEv")]
// 0x2549ec — RBX::MacSystemUtil::getMaxRes(void)
pub fn stub_0x2549ec() {
    todo!("0x2549ec")
}

#[doc(alias = "__ZN3RBX5CryptC1Ev")]
// 0x254bb4 — RBX::Crypt::Crypt(void)
pub fn stub_0x254bb4() {
    todo!("0x254bb4")
}

#[doc(alias = "__ZN3RBX5CryptD1Ev")]
// 0x254bb8 — RBX::Crypt::~Crypt()
pub fn stub_0x254bb8() {
    todo!("0x254bb8")
}

#[doc(alias = "__ZN3RBX5Crypt21verifySignatureBase64ESsSs")]
// 0x254bbc — RBX::Crypt::verifySignatureBase64(std::string,std::string)
pub fn stub_0x254bbc() {
    todo!("0x254bbc")
}

#[doc(alias = "__ZN3RBX14IsValueOutlierEdjddNS_10ConfidenceE")]
// 0x254bf8 — RBX::IsValueOutlier(double,unsigned int,double,double,RBX::Confidence)
pub fn stub_0x254bf8() {
    todo!("0x254bf8")
}

#[doc(alias = "__ZN3RBX21GetConfidenceIntervalEddNS_10ConfidenceEPdS1_")]
// 0x254c68 — RBX::GetConfidenceInterval(double,double,RBX::Confidence,double *,double *)
pub fn stub_0x254c68() {
    todo!("0x254c68")
}

#[doc(alias = "__ZN3RBX9TCriticalEjNS_10ConfidenceE")]
// 0x254d18 — RBX::TCritical(unsigned int,RBX::Confidence)
pub fn stub_0x254d18() {
    todo!("0x254d18")
}

#[doc(alias = "__ZN3RBX11HttpService10decodeJSONESs")]
// 0x256a6c — RBX::HttpService::decodeJSON(std::string)
pub fn stub_0x256a6c() {
    todo!("0x256a6c")
}

#[doc(alias = "__ZN3RBX15StringConverterINS_11HttpService15HttpContentTypeEE14convertToValueERKSsRS2_")]
// 0x2570c0 — RBX::StringConverter<RBX::HttpService::HttpContentType>::convertToValue(std::string const&,RBX::HttpService::HttpContentType&)
pub fn stub_0x2570c0() {
    todo!("0x2570c0")
}

#[doc(alias = "__ZN3RBX11HttpServiceC2Ev")]
// 0x257110 — RBX::HttpService::HttpService(void)
pub fn stub_0x257110() {
    todo!("0x257110")
}

#[doc(alias = "__ZN3RBX11HttpService18checkUserHasAccessEv")]
// 0x257758 — RBX::HttpService::checkUserHasAccess(void)
pub fn stub_0x257758() {
    todo!("0x257758")
}

#[doc(alias = "__ZN3RBX11HttpService10checkLimitEv")]
// 0x2577c0 — RBX::HttpService::checkLimit(void)
pub fn stub_0x2577c0() {
    todo!("0x2577c0")
}

#[doc(alias = "__ZN3RBX11HttpServiceD1Ev")]
// 0x2580ac — RBX::HttpService::~HttpService()
pub fn stub_0x2580ac() {
    todo!("0x2580ac")
}

#[doc(alias = "__ZN3RBX11HttpServiceD0Ev")]
// 0x2580b0 — RBX::HttpService::~HttpService()
pub fn stub_0x2580b0() {
    todo!("0x2580b0")
}

#[doc(alias = "__ZThn32_N3RBX11HttpServiceD1Ev")]
// 0x258160 — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x258160() {
    todo!("0x258160")
}

#[doc(alias = "__ZThn32_N3RBX11HttpServiceD0Ev")]
// 0x258168 — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x258168() {
    todo!("0x258168")
}

#[doc(alias = "__ZThn36_N3RBX11HttpServiceD1Ev")]
// 0x25821c — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x25821c() {
    todo!("0x25821c")
}

#[doc(alias = "__ZThn36_N3RBX11HttpServiceD0Ev")]
// 0x258224 — non-virtual thunk toRBX::HttpService::~HttpService()
pub fn stub_0x258224() {
    todo!("0x258224")
}

#[doc(alias = "__ZN3rbx8any_castIN3RBX11HttpService15HttpContentTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
// 0x258e74 — RBX::HttpService::HttpContentType * rbx::any_cast<RBX::HttpService::HttpContentType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
pub fn stub_0x258e74() {
    todo!("0x258e74")
}

#[doc(alias = "__ZN3rbx8any_castIRN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x258ecc — RBX::HttpService::HttpContentType & rbx::any_cast<RBX::HttpService::HttpContentType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_0x258ecc() {
    todo!("0x258ecc")
}

#[doc(alias = "__ZN3RBX5Light10setEnabledEb")]
// 0x25b4c0 — RBX::Light::setEnabled(bool)
pub fn stub_0x25b4c0() {
    todo!("0x25b4c0")
}

#[doc(alias = "__ZN3RBX5Light13setBrightnessEf")]
// 0x25b544 — RBX::Light::setBrightness(float)
pub fn stub_0x25b544() {
    todo!("0x25b544")
}

#[doc(alias = "__ZN3RBX10PointLight8setRangeEf")]
// 0x25b574 — RBX::PointLight::setRange(float)
pub fn stub_0x25b574() {
    todo!("0x25b574")
}

#[doc(alias = "__ZN3RBX9SpotLight8setRangeEf")]
// 0x25b5b0 — RBX::SpotLight::setRange(float)
pub fn stub_0x25b5b0() {
    todo!("0x25b5b0")
}

#[doc(alias = "__ZN3RBX9SpotLight8setAngleEf")]
// 0x25b5ec — RBX::SpotLight::setAngle(float)
pub fn stub_0x25b5ec() {
    todo!("0x25b5ec")
}

#[doc(alias = "__ZN3RBX19registerNewLightAPIEv")]
// 0x25b628 — RBX::registerNewLightAPI(void)
pub fn stub_0x25b628() {
    todo!("0x25b628")
}

#[doc(alias = "__ZN3RBX5Light10setShadowsEb")]
// 0x25b884 — RBX::Light::setShadows(bool)
pub fn stub_0x25b884() {
    todo!("0x25b884")
}

#[doc(alias = "__ZN3RBX9SpotLight7setFaceENS_8NormalIdE")]
// 0x25b8a8 — RBX::SpotLight::setFace(RBX::NormalId)
pub fn stub_0x25b8a8() {
    todo!("0x25b8a8")
}

#[doc(alias = "__ZN3RBX5LightC2EPKc")]
// 0x25b8c8 — RBX::Light::Light(char const*)
pub fn stub_0x25b8c8() {
    todo!("0x25b8c8")
}

#[doc(alias = "__ZN3RBX5LightD0Ev")]
// 0x25baa8 — RBX::Light::~Light()
pub fn stub_0x25baa8() {
    todo!("0x25baa8")
}

#[doc(alias = "__ZN3RBX5LightD1Ev")]
// 0x25bb48 — RBX::Light::~Light()
pub fn stub_0x25bb48() {
    todo!("0x25bb48")
}

#[doc(alias = "__ZThn32_N3RBX5LightD0Ev")]
// 0x25bb4c — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bb4c() {
    todo!("0x25bb4c")
}

#[doc(alias = "__ZThn36_N3RBX5LightD0Ev")]
// 0x25bb54 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bb54() {
    todo!("0x25bb54")
}

#[doc(alias = "__ZThn92_N3RBX5LightD0Ev")]
// 0x25bb5c — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bb5c() {
    todo!("0x25bb5c")
}

#[doc(alias = "__ZN3RBX5LightD2Ev")]
// 0x25bb64 — RBX::Light::~Light()
pub fn stub_0x25bb64() {
    todo!("0x25bb64")
}

#[doc(alias = "__ZThn32_N3RBX5LightD1Ev")]
// 0x25bc20 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bc20() {
    todo!("0x25bc20")
}

#[doc(alias = "__ZThn36_N3RBX5LightD1Ev")]
// 0x25bc28 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bc28() {
    todo!("0x25bc28")
}

#[doc(alias = "__ZThn92_N3RBX5LightD1Ev")]
// 0x25bc30 — non-virtual thunk toRBX::Light::~Light()
pub fn stub_0x25bc30() {
    todo!("0x25bc30")
}

#[doc(alias = "__ZN3RBX10PointLightC2Ev")]
// 0x25bc64 — RBX::PointLight::PointLight(void)
pub fn stub_0x25bc64() {
    todo!("0x25bc64")
}

#[doc(alias = "__ZN3RBX10PointLightD0Ev")]
// 0x25bdb8 — RBX::PointLight::~PointLight()
pub fn stub_0x25bdb8() {
    todo!("0x25bdb8")
}

#[doc(alias = "__ZN3RBX10PointLightD1Ev")]
// 0x25be58 — RBX::PointLight::~PointLight()
pub fn stub_0x25be58() {
    todo!("0x25be58")
}

#[doc(alias = "__ZThn32_N3RBX10PointLightD0Ev")]
// 0x25be5c — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be5c() {
    todo!("0x25be5c")
}

#[doc(alias = "__ZThn36_N3RBX10PointLightD0Ev")]
// 0x25be64 — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be64() {
    todo!("0x25be64")
}

#[doc(alias = "__ZThn92_N3RBX10PointLightD0Ev")]
// 0x25be6c — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be6c() {
    todo!("0x25be6c")
}

#[doc(alias = "__ZThn32_N3RBX10PointLightD1Ev")]
// 0x25be74 — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be74() {
    todo!("0x25be74")
}

#[doc(alias = "__ZThn36_N3RBX10PointLightD1Ev")]
// 0x25be7c — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be7c() {
    todo!("0x25be7c")
}

#[doc(alias = "__ZThn92_N3RBX10PointLightD1Ev")]
// 0x25be84 — non-virtual thunk toRBX::PointLight::~PointLight()
pub fn stub_0x25be84() {
    todo!("0x25be84")
}

#[doc(alias = "__ZN3RBX9SpotLightC2Ev")]
// 0x25be8c — RBX::SpotLight::SpotLight(void)
pub fn stub_0x25be8c() {
    todo!("0x25be8c")
}

#[doc(alias = "__ZN3RBX9SpotLightD0Ev")]
// 0x25bff0 — RBX::SpotLight::~SpotLight()
pub fn stub_0x25bff0() {
    todo!("0x25bff0")
}

#[doc(alias = "__ZN3RBX9SpotLightD1Ev")]
// 0x25c090 — RBX::SpotLight::~SpotLight()
pub fn stub_0x25c090() {
    todo!("0x25c090")
}

#[doc(alias = "__ZThn32_N3RBX9SpotLightD0Ev")]
// 0x25c094 — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c094() {
    todo!("0x25c094")
}

#[doc(alias = "__ZThn36_N3RBX9SpotLightD0Ev")]
// 0x25c09c — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c09c() {
    todo!("0x25c09c")
}

#[doc(alias = "__ZThn92_N3RBX9SpotLightD0Ev")]
// 0x25c0a4 — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c0a4() {
    todo!("0x25c0a4")
}

#[doc(alias = "__ZThn32_N3RBX9SpotLightD1Ev")]
// 0x25c0ac — non-virtual thunk toRBX::SpotLight::~SpotLight()
pub fn stub_0x25c0ac() {
    todo!("0x25c0ac")
}
