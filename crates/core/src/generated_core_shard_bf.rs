//! core shard BF — 100 core stubs EA-sorted, next uncovered after BE 0x458d68 (strict RBX|boost|std|rbx earliest gap, after BE 0x440304..0x458d68).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x458d68.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChangeHistoryService>(void)")]
// 0x458d6c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ChangeHistoryServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChangeHistoryService>(void)
pub fn stub_458d6c() -> ! {
    todo!("0x458d6c __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ChangeHistoryServiceEEEmv")
}

#[doc(alias = "RBX::Visit * RBX::ServiceProvider::find<RBX::Visit>(void)const")]
// 0x4594f8 — __ZNK3RBX15ServiceProvider4findINS_5VisitEEEPT_v — RBX::Visit * RBX::ServiceProvider::find<RBX::Visit>(void)const
pub fn stub_4594f8() -> ! {
    todo!("0x4594f8 __ZNK3RBX15ServiceProvider4findINS_5VisitEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Visit>(void)")]
// 0x4598e0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_5VisitEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::Visit>(void)
pub fn stub_4598e0() -> ! {
    todo!("0x4598e0 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_5VisitEEEvv")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Visit>(void)")]
// 0x4598e4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5VisitEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Visit>(void)
pub fn stub_4598e4() -> ! {
    todo!("0x4598e4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_5VisitEEEmv")
}

#[doc(alias = "std::string rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x45a690 — __ZN3rbx8any_castISsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE — std::string rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_45a690() -> ! {
    todo!("0x45a690 __ZN3rbx8any_castISsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter>(RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter &)const")]
// 0x46d0e4 — __ZNK3RBX13WindowAverageIddE4iterINS_22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE9GTCounterEEEvRT_ — void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter>(RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter &)const
pub fn stub_46d0e4() -> ! {
    todo!("0x46d0e4 __ZNK3RBX13WindowAverageIddE4iterINS_22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE9GTCounterEEEvRT_")
}

#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
// 0x46d128 — __ZNK3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE8getStatsEm — RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const
pub fn stub_46d128() -> ! {
    todo!("0x46d128 __ZNK3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE8getStatsEm")
}

#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
// 0x46d1b0 — __ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm — RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const
pub fn stub_46d1b0() -> ! {
    todo!("0x46d1b0 __ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm")
}

#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum>(RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum &)const")]
// 0x46d208 — __ZNK3RBX13WindowAverageIddE4iterINS_25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE4FSumEEEvRT_ — void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum>(RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum &)const
pub fn stub_46d208() -> ! {
    todo!("0x46d208 __ZNK3RBX13WindowAverageIddE4iterINS_25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE4FSumEEEvRT_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)")]
// 0x46f67c — __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E — std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)
pub fn stub_46f67c() -> ! {
    todo!("0x46f67c __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "RBX::GuiImageMixin::getImageRectOffset(void)const")]
// 0x46f704 — __ZNK3RBX13GuiImageMixin18getImageRectOffsetEv — RBX::GuiImageMixin::getImageRectOffset(void)const
pub fn stub_46f704() -> ! {
    todo!("0x46f704 __ZNK3RBX13GuiImageMixin18getImageRectOffsetEv")
}

#[doc(alias = "RBX::GuiImageMixin::getImageRectSize(void)const")]
// 0x46f734 — __ZNK3RBX13GuiImageMixin16getImageRectSizeEv — RBX::GuiImageMixin::getImageRectSize(void)const
pub fn stub_46f734() -> ! {
    todo!("0x46f734 __ZNK3RBX13GuiImageMixin16getImageRectSizeEv")
}

#[doc(alias = "RBX::IMetric::~IMetric()")]
// 0x46feac — __ZN3RBX7IMetricD1Ev — RBX::IMetric::~IMetric()
pub fn stub_46feac() -> ! {
    todo!("0x46feac __ZN3RBX7IMetricD1Ev")
}

#[doc(alias = "RBX::IMetric::~IMetric()")]
// 0x46feb0 — __ZN3RBX7IMetricD0Ev — RBX::IMetric::~IMetric()
pub fn stub_46feb0() -> ! {
    todo!("0x46feb0 __ZN3RBX7IMetricD0Ev")
}

#[doc(alias = "RBX::SimpleThrottlingArbiter::isThrottled(void)")]
// 0x473790 — __ZN3RBX23SimpleThrottlingArbiter11isThrottledEv — RBX::SimpleThrottlingArbiter::isThrottled(void)
pub fn stub_473790() -> ! {
    todo!("0x473790 __ZN3RBX23SimpleThrottlingArbiter11isThrottledEv")
}

#[doc(alias = "RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)")]
// 0x473858 — __ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv — RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)
pub fn stub_473858() -> ! {
    todo!("0x473858 __ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv")
}

#[doc(alias = "std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")]
// 0x473f98 — __ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_ — std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)
pub fn stub_473f98() -> ! {
    todo!("0x473f98 __ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)")]
// 0x474350 — __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs — std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)
pub fn stub_474350() -> ! {
    todo!("0x474350 __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs")
}

#[doc(alias = "std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)")]
// 0x4749a8 — __ZNSt12_Vector_baseImSaImEE11_M_allocateEm — std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)
pub fn stub_4749a8() -> ! {
    todo!("0x4749a8 __ZNSt12_Vector_baseImSaImEE11_M_allocateEm")
}

#[doc(alias = "RBX::ActivityMeter<2>::updateBuckets(void)")]
// 0x474c38 — __ZN3RBX13ActivityMeterILi2EE13updateBucketsEv — RBX::ActivityMeter<2>::updateBuckets(void)
pub fn stub_474c38() -> ! {
    todo!("0x474c38 __ZN3RBX13ActivityMeterILi2EE13updateBucketsEv")
}

#[doc(alias = "RBX::OnScreenProfiler::GetInst(void)")]
// 0x474cf0 — __ZN3RBX16OnScreenProfiler7GetInstEv — RBX::OnScreenProfiler::GetInst(void)
pub fn stub_474cf0() -> ! {
    todo!("0x474cf0 __ZN3RBX16OnScreenProfiler7GetInstEv")
}

#[doc(alias = "RBX::OnScreenProfiler::Create(void)")]
// 0x474d54 — __ZN3RBX16OnScreenProfiler6CreateEv — RBX::OnScreenProfiler::Create(void)
pub fn stub_474d54() -> ! {
    todo!("0x474d54 __ZN3RBX16OnScreenProfiler6CreateEv")
}

#[doc(alias = "RBX::DebrisService::setMaxItems(int)")]
// 0x4770dc — __ZN3RBX13DebrisService11setMaxItemsEi — RBX::DebrisService::setMaxItems(int)
pub fn stub_4770dc() -> ! {
    todo!("0x4770dc __ZN3RBX13DebrisService11setMaxItemsEi")
}

#[doc(alias = "RBX::DebrisService::setLegacyMaxItems(bool)")]
// 0x477410 — __ZN3RBX13DebrisService17setLegacyMaxItemsEb — RBX::DebrisService::setLegacyMaxItems(bool)
pub fn stub_477410() -> ! {
    todo!("0x477410 __ZN3RBX13DebrisService17setLegacyMaxItemsEb")
}

#[doc(alias = "RBX::DebrisService::DebrisService(void)")]
// 0x477418 — __ZN3RBX13DebrisServiceC1Ev — RBX::DebrisService::DebrisService(void)
pub fn stub_477418() -> ! {
    todo!("0x477418 __ZN3RBX13DebrisServiceC1Ev")
}

#[doc(alias = "RBX::DebrisService::DebrisService(void)")]
// 0x47741c — __ZN3RBX13DebrisServiceC2Ev — RBX::DebrisService::DebrisService(void)
pub fn stub_47741c() -> ! {
    todo!("0x47741c __ZN3RBX13DebrisServiceC2Ev")
}

#[doc(alias = "RBX::DebrisService::cleanup(void)")]
// 0x4775e4 — __ZN3RBX13DebrisService7cleanupEv — RBX::DebrisService::cleanup(void)
pub fn stub_4775e4() -> ! {
    todo!("0x4775e4 __ZN3RBX13DebrisService7cleanupEv")
}

#[doc(alias = "RBX::DebrisService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x477864 — __ZN3RBX13DebrisService17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::DebrisService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_477864() -> ! {
    todo!("0x477864 __ZN3RBX13DebrisService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::DebrisService::getMaxItems(void)const")]
// 0x477a0c — __ZNK3RBX13DebrisService11getMaxItemsEv — RBX::DebrisService::getMaxItems(void)const
pub fn stub_477a0c() -> ! {
    todo!("0x477a0c __ZNK3RBX13DebrisService11getMaxItemsEv")
}

#[doc(alias = "RBX::DebrisService::~DebrisService()")]
// 0x477ed8 — __ZN3RBX13DebrisServiceD1Ev — RBX::DebrisService::~DebrisService()
pub fn stub_477ed8() -> ! {
    todo!("0x477ed8 __ZN3RBX13DebrisServiceD1Ev")
}

#[doc(alias = "RBX::DebrisService::~DebrisService()")]
// 0x477fe4 — __ZN3RBX13DebrisServiceD0Ev — RBX::DebrisService::~DebrisService()
pub fn stub_477fe4() -> ! {
    todo!("0x477fe4 __ZN3RBX13DebrisServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x478128 — __ZThn32_N3RBX13DebrisServiceD1Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
pub fn stub_478128() -> ! {
    todo!("0x478128 __ZThn32_N3RBX13DebrisServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x478234 — __ZThn32_N3RBX13DebrisServiceD0Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
pub fn stub_478234() -> ! {
    todo!("0x478234 __ZThn32_N3RBX13DebrisServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x47837c — __ZThn36_N3RBX13DebrisServiceD1Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
pub fn stub_47837c() -> ! {
    todo!("0x47837c __ZThn36_N3RBX13DebrisServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::DebrisService::~DebrisService()")]
// 0x478484 — __ZThn36_N3RBX13DebrisServiceD0Ev — non-virtual thunk toRBX::DebrisService::~DebrisService()
pub fn stub_478484() -> ! {
    todo!("0x478484 __ZThn36_N3RBX13DebrisServiceD0Ev")
}

#[doc(alias = "RBX::DebugSettings::getVertexShaderModel(void)const")]
// 0x47b4cc — __ZNK3RBX13DebugSettings20getVertexShaderModelEv — RBX::DebugSettings::getVertexShaderModel(void)const
pub fn stub_47b4cc() -> ! {
    todo!("0x47b4cc __ZNK3RBX13DebugSettings20getVertexShaderModelEv")
}

#[doc(alias = "RBX::DebugSettings::getPixelShaderModel(void)const")]
// 0x47b4d0 — __ZNK3RBX13DebugSettings19getPixelShaderModelEv — RBX::DebugSettings::getPixelShaderModel(void)const
pub fn stub_47b4d0() -> ! {
    todo!("0x47b4d0 __ZNK3RBX13DebugSettings19getPixelShaderModelEv")
}

#[doc(alias = "RBX::DebugSettings::videoMemory(void)const")]
// 0x47b4d4 — __ZNK3RBX13DebugSettings11videoMemoryEv — RBX::DebugSettings::videoMemory(void)const
pub fn stub_47b4d4() -> ! {
    todo!("0x47b4d4 __ZNK3RBX13DebugSettings11videoMemoryEv")
}

#[doc(alias = "RBX::DebugSettings::cpuSpeed(void)const")]
// 0x47b564 — __ZNK3RBX13DebugSettings8cpuSpeedEv — RBX::DebugSettings::cpuSpeed(void)const
pub fn stub_47b564() -> ! {
    todo!("0x47b564 __ZNK3RBX13DebugSettings8cpuSpeedEv")
}

#[doc(alias = "RBX::DebugSettings::cpuCount(void)const")]
// 0x47b5f4 — __ZNK3RBX13DebugSettings8cpuCountEv — RBX::DebugSettings::cpuCount(void)const
pub fn stub_47b5f4() -> ! {
    todo!("0x47b5f4 __ZNK3RBX13DebugSettings8cpuCountEv")
}

#[doc(alias = "RBX::DebugSettings::osPlatformId(void)const")]
// 0x47b684 — __ZNK3RBX13DebugSettings12osPlatformIdEv — RBX::DebugSettings::osPlatformId(void)const
pub fn stub_47b684() -> ! {
    todo!("0x47b684 __ZNK3RBX13DebugSettings12osPlatformIdEv")
}

#[doc(alias = "RBX::DebugSettings::osPlatform(void)const")]
// 0x47b688 — __ZNK3RBX13DebugSettings10osPlatformEv — RBX::DebugSettings::osPlatform(void)const
pub fn stub_47b688() -> ! {
    todo!("0x47b688 __ZNK3RBX13DebugSettings10osPlatformEv")
}

#[doc(alias = "RBX::DebugSettings::osVer(void)const")]
// 0x47b6a4 — __ZNK3RBX13DebugSettings5osVerEv — RBX::DebugSettings::osVer(void)const
pub fn stub_47b6a4() -> ! {
    todo!("0x47b6a4 __ZNK3RBX13DebugSettings5osVerEv")
}

#[doc(alias = "RBX::DebugSettings::osIs64Bit(void)const")]
// 0x47b6b0 — __ZNK3RBX13DebugSettings9osIs64BitEv — RBX::DebugSettings::osIs64Bit(void)const
pub fn stub_47b6b0() -> ! {
    todo!("0x47b6b0 __ZNK3RBX13DebugSettings9osIs64BitEv")
}

#[doc(alias = "RBX::DebugSettings::systemProductName(void)const")]
// 0x47b6bc — __ZNK3RBX13DebugSettings17systemProductNameEv — RBX::DebugSettings::systemProductName(void)const
pub fn stub_47b6bc() -> ! {
    todo!("0x47b6bc __ZNK3RBX13DebugSettings17systemProductNameEv")
}

#[doc(alias = "RBX::DebugSettings::gfxcard(void)const")]
// 0x47b6d8 — __ZNK3RBX13DebugSettings7gfxcardEv — RBX::DebugSettings::gfxcard(void)const
pub fn stub_47b6d8() -> ! {
    todo!("0x47b6d8 __ZNK3RBX13DebugSettings7gfxcardEv")
}

#[doc(alias = "RBX::DebugSettings::cpu(void)const")]
// 0x47b6e4 — __ZNK3RBX13DebugSettings3cpuEv — RBX::DebugSettings::cpu(void)const
pub fn stub_47b6e4() -> ! {
    todo!("0x47b6e4 __ZNK3RBX13DebugSettings3cpuEv")
}

#[doc(alias = "RBX::DebugSettings::simd(void)const")]
// 0x47b894 — __ZNK3RBX13DebugSettings4simdEv — RBX::DebugSettings::simd(void)const
pub fn stub_47b894() -> ! {
    todo!("0x47b894 __ZNK3RBX13DebugSettings4simdEv")
}

#[doc(alias = "RBX::DebugSettings::totalPhysicalMemory(void)const")]
// 0x47b9a4 — __ZNK3RBX13DebugSettings19totalPhysicalMemoryEv — RBX::DebugSettings::totalPhysicalMemory(void)const
pub fn stub_47b9a4() -> ! {
    todo!("0x47b9a4 __ZNK3RBX13DebugSettings19totalPhysicalMemoryEv")
}

#[doc(alias = "RBX::DebugSettings::resolution(void)const")]
// 0x47ba34 — __ZNK3RBX13DebugSettings10resolutionEv — RBX::DebugSettings::resolution(void)const
pub fn stub_47ba34() -> ! {
    todo!("0x47ba34 __ZNK3RBX13DebugSettings10resolutionEv")
}

#[doc(alias = "RBX::DebugSettings::availablePhysicalMemory(void)const")]
// 0x47bbb4 — __ZNK3RBX13DebugSettings23availablePhysicalMemoryEv — RBX::DebugSettings::availablePhysicalMemory(void)const
pub fn stub_47bbb4() -> ! {
    todo!("0x47bbb4 __ZNK3RBX13DebugSettings23availablePhysicalMemoryEv")
}

#[doc(alias = "RBX::DebugSettings::getElapsedTime(void)const")]
// 0x47bc44 — __ZNK3RBX13DebugSettings14getElapsedTimeEv — RBX::DebugSettings::getElapsedTime(void)const
pub fn stub_47bc44() -> ! {
    todo!("0x47bc44 __ZNK3RBX13DebugSettings14getElapsedTimeEv")
}

#[doc(alias = "RBX::DebugSettings::processCores(void)const")]
// 0x47bc50 — __ZNK3RBX13DebugSettings12processCoresEv — RBX::DebugSettings::processCores(void)const
pub fn stub_47bc50() -> ! {
    todo!("0x47bc50 __ZNK3RBX13DebugSettings12processCoresEv")
}

#[doc(alias = "RBX::DebugSettings::totalProcessorTime(void)const")]
// 0x47bc8c — __ZNK3RBX13DebugSettings18totalProcessorTimeEv — RBX::DebugSettings::totalProcessorTime(void)const
pub fn stub_47bc8c() -> ! {
    todo!("0x47bc8c __ZNK3RBX13DebugSettings18totalProcessorTimeEv")
}

#[doc(alias = "RBX::DebugSettings::processorTime(void)const")]
// 0x47bcb0 — __ZNK3RBX13DebugSettings13processorTimeEv — RBX::DebugSettings::processorTime(void)const
pub fn stub_47bcb0() -> ! {
    todo!("0x47bcb0 __ZNK3RBX13DebugSettings13processorTimeEv")
}

#[doc(alias = "RBX::DebugSettings::privateBytes(void)const")]
// 0x47bcb8 — __ZNK3RBX13DebugSettings12privateBytesEv — RBX::DebugSettings::privateBytes(void)const
pub fn stub_47bcb8() -> ! {
    todo!("0x47bcb8 __ZNK3RBX13DebugSettings12privateBytesEv")
}

#[doc(alias = "RBX::DebugSettings::privateWorkingSetBytes(void)const")]
// 0x47bcdc — __ZNK3RBX13DebugSettings22privateWorkingSetBytesEv — RBX::DebugSettings::privateWorkingSetBytes(void)const
pub fn stub_47bcdc() -> ! {
    todo!("0x47bcdc __ZNK3RBX13DebugSettings22privateWorkingSetBytesEv")
}

#[doc(alias = "RBX::DebugSettings::GetVirtualBytes(void)const")]
// 0x47bcfc — __ZNK3RBX13DebugSettings15GetVirtualBytesEv — RBX::DebugSettings::GetVirtualBytes(void)const
pub fn stub_47bcfc() -> ! {
    todo!("0x47bcfc __ZNK3RBX13DebugSettings15GetVirtualBytesEv")
}

#[doc(alias = "RBX::DebugSettings::GetPageFileBytes(void)const")]
// 0x47bd1c — __ZNK3RBX13DebugSettings16GetPageFileBytesEv — RBX::DebugSettings::GetPageFileBytes(void)const
pub fn stub_47bd1c() -> ! {
    todo!("0x47bd1c __ZNK3RBX13DebugSettings16GetPageFileBytesEv")
}

#[doc(alias = "RBX::DebugSettings::GetPageFaultsPerSecond(void)const")]
// 0x47bd24 — __ZNK3RBX13DebugSettings22GetPageFaultsPerSecondEv — RBX::DebugSettings::GetPageFaultsPerSecond(void)const
pub fn stub_47bd24() -> ! {
    todo!("0x47bd24 __ZNK3RBX13DebugSettings22GetPageFaultsPerSecondEv")
}

#[doc(alias = "RBX::DebugSettings::getPlayerCount(void)const")]
// 0x47bd50 — __ZNK3RBX13DebugSettings14getPlayerCountEv — RBX::DebugSettings::getPlayerCount(void)const
pub fn stub_47bd50() -> ! {
    todo!("0x47bd50 __ZNK3RBX13DebugSettings14getPlayerCountEv")
}

#[doc(alias = "RBX::DebugSettings::getCdnSuccessCount(void)const")]
// 0x47bd70 — __ZNK3RBX13DebugSettings18getCdnSuccessCountEv — RBX::DebugSettings::getCdnSuccessCount(void)const
pub fn stub_47bd70() -> ! {
    todo!("0x47bd70 __ZNK3RBX13DebugSettings18getCdnSuccessCountEv")
}

#[doc(alias = "RBX::DebugSettings::getCdnFailureCount(void)const")]
// 0x47bd80 — __ZNK3RBX13DebugSettings18getCdnFailureCountEv — RBX::DebugSettings::getCdnFailureCount(void)const
pub fn stub_47bd80() -> ! {
    todo!("0x47bd80 __ZNK3RBX13DebugSettings18getCdnFailureCountEv")
}

#[doc(alias = "RBX::DebugSettings::getAlternateCdnSuccessCount(void)const")]
// 0x47bd90 — __ZNK3RBX13DebugSettings27getAlternateCdnSuccessCountEv — RBX::DebugSettings::getAlternateCdnSuccessCount(void)const
pub fn stub_47bd90() -> ! {
    todo!("0x47bd90 __ZNK3RBX13DebugSettings27getAlternateCdnSuccessCountEv")
}

#[doc(alias = "RBX::DebugSettings::getAlternateCdnFailureCount(void)const")]
// 0x47bda0 — __ZNK3RBX13DebugSettings27getAlternateCdnFailureCountEv — RBX::DebugSettings::getAlternateCdnFailureCount(void)const
pub fn stub_47bda0() -> ! {
    todo!("0x47bda0 __ZNK3RBX13DebugSettings27getAlternateCdnFailureCountEv")
}

#[doc(alias = "RBX::DebugSettings::getBlockMeshMapCount(void)const")]
// 0x47bdb0 — __ZNK3RBX13DebugSettings20getBlockMeshMapCountEv — RBX::DebugSettings::getBlockMeshMapCount(void)const
pub fn stub_47bdb0() -> ! {
    todo!("0x47bdb0 __ZNK3RBX13DebugSettings20getBlockMeshMapCountEv")
}

#[doc(alias = "RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const")]
// 0x47bdb4 — __ZNK3RBX13DebugSettings25getLastCdnFailureTimeSpanEv — RBX::DebugSettings::getLastCdnFailureTimeSpan(void)const
pub fn stub_47bdb4() -> ! {
    todo!("0x47bdb4 __ZNK3RBX13DebugSettings25getLastCdnFailureTimeSpanEv")
}

#[doc(alias = "RBX::DebugSettings::getRobloxSuccessCount(void)const")]
// 0x47bdcc — __ZNK3RBX13DebugSettings21getRobloxSuccessCountEv — RBX::DebugSettings::getRobloxSuccessCount(void)const
pub fn stub_47bdcc() -> ! {
    todo!("0x47bdcc __ZNK3RBX13DebugSettings21getRobloxSuccessCountEv")
}

#[doc(alias = "RBX::DebugSettings::getRobloxFalureCount(void)const")]
// 0x47bddc — __ZNK3RBX13DebugSettings20getRobloxFalureCountEv — RBX::DebugSettings::getRobloxFalureCount(void)const
pub fn stub_47bddc() -> ! {
    todo!("0x47bddc __ZNK3RBX13DebugSettings20getRobloxFalureCountEv")
}

#[doc(alias = "RBX::DebugSettings::getRobloxResponce(void)const")]
// 0x47bdf0 — __ZNK3RBX13DebugSettings17getRobloxResponceEv — RBX::DebugSettings::getRobloxResponce(void)const
pub fn stub_47bdf0() -> ! {
    todo!("0x47bdf0 __ZNK3RBX13DebugSettings17getRobloxResponceEv")
}

#[doc(alias = "RBX::DebugSettings::getCdnRespoce(void)const")]
// 0x47be48 — __ZNK3RBX13DebugSettings13getCdnRespoceEv — RBX::DebugSettings::getCdnRespoce(void)const
pub fn stub_47be48() -> ! {
    todo!("0x47be48 __ZNK3RBX13DebugSettings13getCdnRespoceEv")
}

#[doc(alias = "RBX::DebugSettings::resetCdnFailureCounts(void)")]
// 0x47bea0 — __ZN3RBX13DebugSettings21resetCdnFailureCountsEv — RBX::DebugSettings::resetCdnFailureCounts(void)
pub fn stub_47bea0() -> ! {
    todo!("0x47bea0 __ZN3RBX13DebugSettings21resetCdnFailureCountsEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::addDummyJob(bool,double)")]
// 0x47c2a8 — __ZN3RBX21TaskSchedulerSettings11addDummyJobEbd — RBX::TaskSchedulerSettings::addDummyJob(bool,double)
pub fn stub_47c2a8() -> ! {
    todo!("0x47c2a8 __ZN3RBX21TaskSchedulerSettings11addDummyJobEbd")
}

#[doc(alias = "RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)")]
// 0x47c3f8 — __ZN3RBX13DebugSettings17setErrorReportingENS0_14ErrorReportingE — RBX::DebugSettings::setErrorReporting(RBX::DebugSettings::ErrorReporting)
pub fn stub_47c3f8() -> ! {
    todo!("0x47c3f8 __ZN3RBX13DebugSettings17setErrorReportingENS0_14ErrorReportingE")
}

#[doc(alias = "RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const")]
// 0x47c414 — __ZNK3RBX21TaskSchedulerSettings19getThreadPoolConfigEv — RBX::TaskSchedulerSettings::getThreadPoolConfig(void)const
pub fn stub_47c414() -> ! {
    todo!("0x47c414 __ZNK3RBX21TaskSchedulerSettings19getThreadPoolConfigEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThreadPoolConfig(RBX::TaskScheduler::ThreadPoolConfig)")]
// 0x47c418 — __ZN3RBX21TaskSchedulerSettings19setThreadPoolConfigENS_13TaskScheduler16ThreadPoolConfigE — RBX::TaskSchedulerSettings::setThreadPoolConfig(RBX::TaskScheduler::ThreadPoolConfig)
pub fn stub_47c418() -> ! {
    todo!("0x47c418 __ZN3RBX21TaskSchedulerSettings19setThreadPoolConfigENS_13TaskScheduler16ThreadPoolConfigE")
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThreadShare(double,int)")]
// 0x47c460 — __ZN3RBX21TaskSchedulerSettings14setThreadShareEdi — RBX::TaskSchedulerSettings::setThreadShare(double,int)
pub fn stub_47c460() -> ! {
    todo!("0x47c460 __ZN3RBX21TaskSchedulerSettings14setThreadShareEdi")
}

#[doc(alias = "RBX::TaskSchedulerSettings::setPriorityMethod(RBX::TaskScheduler::PriorityMethod)")]
// 0x47c464 — __ZN3RBX21TaskSchedulerSettings17setPriorityMethodENS_13TaskScheduler14PriorityMethodE — RBX::TaskSchedulerSettings::setPriorityMethod(RBX::TaskScheduler::PriorityMethod)
pub fn stub_47c464() -> ! {
    todo!("0x47c464 __ZN3RBX21TaskSchedulerSettings17setPriorityMethodENS_13TaskScheduler14PriorityMethodE")
}

#[doc(alias = "RBX::TaskSchedulerSettings::setSleepAdjustMethod(RBX::TaskScheduler::Job::SleepAdjustMethod)")]
// 0x47c4a0 — __ZN3RBX21TaskSchedulerSettings20setSleepAdjustMethodENS_13TaskScheduler3Job17SleepAdjustMethodE — RBX::TaskSchedulerSettings::setSleepAdjustMethod(RBX::TaskScheduler::Job::SleepAdjustMethod)
pub fn stub_47c4a0() -> ! {
    todo!("0x47c4a0 __ZN3RBX21TaskSchedulerSettings20setSleepAdjustMethodENS_13TaskScheduler3Job17SleepAdjustMethodE")
}

#[doc(alias = "RBX::TaskSchedulerSettings::setIsArbiterThrottled(bool)")]
// 0x47c518 — __ZN3RBX21TaskSchedulerSettings21setIsArbiterThrottledEb — RBX::TaskSchedulerSettings::setIsArbiterThrottled(bool)
pub fn stub_47c518() -> ! {
    todo!("0x47c518 __ZN3RBX21TaskSchedulerSettings21setIsArbiterThrottledEb")
}

#[doc(alias = "RBX::TaskSchedulerSettings::setThrottledJobSleepTime(double)")]
// 0x47c53c — __ZN3RBX21TaskSchedulerSettings24setThrottledJobSleepTimeEd — RBX::TaskSchedulerSettings::setThrottledJobSleepTime(double)
pub fn stub_47c53c() -> ! {
    todo!("0x47c53c __ZN3RBX21TaskSchedulerSettings24setThrottledJobSleepTimeEd")
}

#[doc(alias = "RBX::DebugSettings::getIsProfilingEnabled(void)const")]
// 0x47c564 — __ZNK3RBX13DebugSettings21getIsProfilingEnabledEv — RBX::DebugSettings::getIsProfilingEnabled(void)const
pub fn stub_47c564() -> ! {
    todo!("0x47c564 __ZNK3RBX13DebugSettings21getIsProfilingEnabledEv")
}

#[doc(alias = "RBX::DebugSettings::setIsProfilingEnabled(bool)")]
// 0x47c570 — __ZN3RBX13DebugSettings21setIsProfilingEnabledEb — RBX::DebugSettings::setIsProfilingEnabled(bool)
pub fn stub_47c570() -> ! {
    todo!("0x47c570 __ZN3RBX13DebugSettings21setIsProfilingEnabledEb")
}

#[doc(alias = "RBX::DebugSettings::getProfilingWindow(void)const")]
// 0x47c578 — __ZNK3RBX13DebugSettings18getProfilingWindowEv — RBX::DebugSettings::getProfilingWindow(void)const
pub fn stub_47c578() -> ! {
    todo!("0x47c578 __ZNK3RBX13DebugSettings18getProfilingWindowEv")
}

#[doc(alias = "RBX::DebugSettings::setProfilingWindow(double)")]
// 0x47c590 — __ZN3RBX13DebugSettings18setProfilingWindowEd — RBX::DebugSettings::setProfilingWindow(double)
pub fn stub_47c590() -> ! {
    todo!("0x47c590 __ZN3RBX13DebugSettings18setProfilingWindowEd")
}

#[doc(alias = "RBX::DebugSettings::DebugSettings(void)")]
// 0x47c608 — __ZN3RBX13DebugSettingsC1Ev — RBX::DebugSettings::DebugSettings(void)
pub fn stub_47c608() -> ! {
    todo!("0x47c608 __ZN3RBX13DebugSettingsC1Ev")
}

#[doc(alias = "RBX::DebugSettings::DebugSettings(void)")]
// 0x47c60c — __ZN3RBX13DebugSettingsC2Ev — RBX::DebugSettings::DebugSettings(void)
pub fn stub_47c60c() -> ! {
    todo!("0x47c60c __ZN3RBX13DebugSettingsC2Ev")
}

#[doc(alias = "DummyArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)")]
// 0x47c7e4 — __ZN12DummyArbiter12areExclusiveEPN3RBX13TaskScheduler3JobES3_ — DummyArbiter::areExclusive(RBX::TaskScheduler::Job *,RBX::TaskScheduler::Job *)
pub fn stub_47c7e4() -> ! {
    todo!("0x47c7e4 __ZN12DummyArbiter12areExclusiveEPN3RBX13TaskScheduler3JobES3_")
}

#[doc(alias = "RBX::TaskSchedulerSettings::TaskSchedulerSettings(void)")]
// 0x47c800 — __ZN3RBX21TaskSchedulerSettingsC2Ev — RBX::TaskSchedulerSettings::TaskSchedulerSettings(void)
pub fn stub_47c800() -> ! {
    todo!("0x47c800 __ZN3RBX21TaskSchedulerSettingsC2Ev")
}

#[doc(alias = "RBX::DebugSettings::getRobloxVersion(void)const")]
// 0x47de54 — __ZNK3RBX13DebugSettings16getRobloxVersionEv — RBX::DebugSettings::getRobloxVersion(void)const
pub fn stub_47de54() -> ! {
    todo!("0x47de54 __ZNK3RBX13DebugSettings16getRobloxVersionEv")
}

#[doc(alias = "RBX::DebugSettings::getRobloxProductName(void)const")]
// 0x47deec — __ZNK3RBX13DebugSettings20getRobloxProductNameEv — RBX::DebugSettings::getRobloxProductName(void)const
pub fn stub_47deec() -> ! {
    todo!("0x47deec __ZNK3RBX13DebugSettings20getRobloxProductNameEv")
}

#[doc(alias = "RBX::DebugSettings::nameDatabaseSize(void)const")]
// 0x47dfcc — __ZNK3RBX13DebugSettings16nameDatabaseSizeEv — RBX::DebugSettings::nameDatabaseSize(void)const
pub fn stub_47dfcc() -> ! {
    todo!("0x47dfcc __ZNK3RBX13DebugSettings16nameDatabaseSizeEv")
}

#[doc(alias = "RBX::DebugSettings::nameDatabaseBytes(void)const")]
// 0x47dfd0 — __ZNK3RBX13DebugSettings17nameDatabaseBytesEv — RBX::DebugSettings::nameDatabaseBytes(void)const
pub fn stub_47dfd0() -> ! {
    todo!("0x47dfd0 __ZNK3RBX13DebugSettings17nameDatabaseBytesEv")
}

#[doc(alias = "RBX::DebugSettings::instanceCount(void)const")]
// 0x47dff8 — __ZNK3RBX13DebugSettings13instanceCountEv — RBX::DebugSettings::instanceCount(void)const
pub fn stub_47dff8() -> ! {
    todo!("0x47dff8 __ZNK3RBX13DebugSettings13instanceCountEv")
}

#[doc(alias = "RBX::DebugSettings::jobCount(void)const")]
// 0x47e008 — __ZNK3RBX13DebugSettings8jobCountEv — RBX::DebugSettings::jobCount(void)const
pub fn stub_47e008() -> ! {
    todo!("0x47e008 __ZNK3RBX13DebugSettings8jobCountEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::threadPoolSize(void)const")]
// 0x47e03c — __ZNK3RBX21TaskSchedulerSettings14threadPoolSizeEv — RBX::TaskSchedulerSettings::threadPoolSize(void)const
pub fn stub_47e03c() -> ! {
    todo!("0x47e03c __ZNK3RBX21TaskSchedulerSettings14threadPoolSizeEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::threadAffinity(void)const")]
// 0x47e074 — __ZNK3RBX21TaskSchedulerSettings14threadAffinityEv — RBX::TaskSchedulerSettings::threadAffinity(void)const
pub fn stub_47e074() -> ! {
    todo!("0x47e074 __ZNK3RBX21TaskSchedulerSettings14threadAffinityEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::numSleepingJobs(void)const")]
// 0x47e0ac — __ZNK3RBX21TaskSchedulerSettings15numSleepingJobsEv — RBX::TaskSchedulerSettings::numSleepingJobs(void)const
pub fn stub_47e0ac() -> ! {
    todo!("0x47e0ac __ZNK3RBX21TaskSchedulerSettings15numSleepingJobsEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::numWaitingJobs(void)const")]
// 0x47e0c0 — __ZNK3RBX21TaskSchedulerSettings14numWaitingJobsEv — RBX::TaskSchedulerSettings::numWaitingJobs(void)const
pub fn stub_47e0c0() -> ! {
    todo!("0x47e0c0 __ZNK3RBX21TaskSchedulerSettings14numWaitingJobsEv")
}

#[doc(alias = "RBX::TaskSchedulerSettings::numRunningJobs(void)const")]
// 0x47e0d4 — __ZNK3RBX21TaskSchedulerSettings14numRunningJobsEv — RBX::TaskSchedulerSettings::numRunningJobs(void)const
pub fn stub_47e0d4() -> ! {
    todo!("0x47e0d4 __ZNK3RBX21TaskSchedulerSettings14numRunningJobsEv")
}
