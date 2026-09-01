//! core shard CH — 100 core stubs EA-sorted, next uncovered after CG 0x644a68 (strict RBX|boost|std|rbx earliest gap 0x644b4c).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::_Vector_base<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_allocate(unsigned long)")]
// 0x644b4c — __ZNSt12_Vector_baseIN3RBX12SpecialShape8MeshTypeESaIS2_EE11_M_allocateEm
pub fn stub_644b4c() -> ! {
    todo!("0x644b4c __ZNSt12_Vector_baseIN3RBX12SpecialShape8MeshTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SpecialShape::MeshType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *>(RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *,RBX::SpecialShape::MeshType *)")]
// 0x644b64 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12SpecialShape8MeshTypeES6_EET0_T_S8_S7_
pub fn stub_644b64() -> ! {
    todo!("0x644b64 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12SpecialShape8MeshTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SpecialShape::MeshType*,std::vector<RBX::SpecialShape::MeshType,std::allocator<RBX::SpecialShape::MeshType>>>,unsigned long,RBX::SpecialShape::MeshType const&)")]
// 0x644ba0 — __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_644ba0() -> ! {
    todo!("0x644ba0 __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::Stats::StatsService::reportTaskScheduler(bool)")]
// 0x64528c — __ZN3RBX5Stats12StatsService19reportTaskSchedulerEb
pub fn stub_64528c() -> ! {
    todo!("0x64528c __ZN3RBX5Stats12StatsService19reportTaskSchedulerEb")
}

#[doc(alias = "RBX::Stats::StatsService::reportJobsStepWindow(void)")]
// 0x645860 — __ZN3RBX5Stats12StatsService20reportJobsStepWindowEv
pub fn stub_645860() -> ! {
    todo!("0x645860 __ZN3RBX5Stats12StatsService20reportJobsStepWindowEv")
}

#[doc(alias = "RBX::Stats::StatsService::setReportUrl(std::string)")]
// 0x645d64 — __ZN3RBX5Stats12StatsService12setReportUrlESs
pub fn stub_645d64() -> ! {
    todo!("0x645d64 __ZN3RBX5Stats12StatsService12setReportUrlESs")
}

#[doc(alias = "RBX::Stats::StatsService::addHeader(rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0x646628 — __ZN3RBX5Stats12StatsService9addHeaderEN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE
pub fn stub_646628() -> ! {
    todo!("0x646628 __ZN3RBX5Stats12StatsService9addHeaderEN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE")
}

#[doc(alias = "RBX::Stats::StatsService::getDefaultReportUrl(std::string const&,std::string const&)")]
// 0x646744 — __ZN3RBX5Stats12StatsService19getDefaultReportUrlERKSsS3_
pub fn stub_646744() -> ! {
    todo!("0x646744 __ZN3RBX5Stats12StatsService19getDefaultReportUrlERKSsS3_")
}

#[doc(alias = "RBX::Stats::StatsService::getReportUrl(void)const")]
// 0x646a9c — __ZNK3RBX5Stats12StatsService12getReportUrlEv
pub fn stub_646a9c() -> ! {
    todo!("0x646a9c __ZNK3RBX5Stats12StatsService12getReportUrlEv")
}

#[doc(alias = "RBX::Stats::StatsService::postReportWithUrl(std::string const&,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0x646cbc — __ZN3RBX5Stats12StatsService17postReportWithUrlERKSsN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE
pub fn stub_646cbc() -> ! {
    todo!("0x646cbc __ZN3RBX5Stats12StatsService17postReportWithUrlERKSsN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE")
}

#[doc(alias = "RBX::Stats::reportResult(std::string *,std::exception *)")]
// 0x6470e8 — __ZN3RBX5StatsL12reportResultEPSsPSt9exception
pub fn stub_6470e8() -> ! {
    todo!("0x6470e8 __ZN3RBX5StatsL12reportResultEPSsPSt9exception")
}

#[doc(alias = "RBX::Stats::StatsService::postReport(rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>)")]
// 0x6471c4 — __ZN3RBX5Stats12StatsService10postReportEN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE
pub fn stub_6471c4() -> ! {
    todo!("0x6471c4 __ZN3RBX5Stats12StatsService10postReportEN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEE")
}

#[doc(alias = "RBX::Stats::StatsService::reportJob(rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)")]
// 0x64732c — __ZN3RBX5Stats12StatsService9reportJobEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEENS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERb
pub fn stub_64732c() -> ! {
    todo!("0x64732c __ZN3RBX5Stats12StatsService9reportJobEN5boost10shared_ptrIKNS_13TaskScheduler3JobEEENS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERb")
}

#[doc(alias = "RBX::Stats::StatsService::checkLastReport(std::string const&)")]
// 0x647604 — __ZN3RBX5Stats12StatsService15checkLastReportERKSs
pub fn stub_647604() -> ! {
    todo!("0x647604 __ZN3RBX5Stats12StatsService15checkLastReportERKSs")
}

#[doc(alias = "RBX::Stats::StatsService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x648bc0 — __ZN3RBX5Stats12StatsService17onServiceProviderEPNS_15ServiceProviderES3_
pub fn stub_648bc0() -> ! {
    todo!("0x648bc0 __ZN3RBX5Stats12StatsService17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "RBX::Stats::Item::update(void)")]
// 0x648cdc — __ZN3RBX5Stats4Item6updateEv
pub fn stub_648cdc() -> ! {
    todo!("0x648cdc __ZN3RBX5Stats4Item6updateEv")
}

#[doc(alias = "RBX::Stats::Item::formatMem(unsigned long)")]
// 0x648d1c — __ZN3RBX5Stats4Item9formatMemEm
pub fn stub_648d1c() -> ! {
    todo!("0x648d1c __ZN3RBX5Stats4Item9formatMemEm")
}

#[doc(alias = "RBX::Stats::Item::formatRate(RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1> const&)")]
// 0x648e48 — __ZN3RBX5Stats4Item10formatRateERKNS_26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EEE
pub fn stub_648e48() -> ! {
    todo!("0x648e48 __ZN3RBX5Stats4Item10formatRateERKNS_26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EEE")
}

#[doc(alias = "RBX::Stats::Item::formatValue(double,char const*,...)")]
// 0x648eb0 — __ZN3RBX5Stats4Item11formatValueEdPKcz
pub fn stub_648eb0() -> ! {
    todo!("0x648eb0 __ZN3RBX5Stats4Item11formatValueEdPKcz")
}

#[doc(alias = "RBX::Stats::Item::createChildItem(char const*)")]
// 0x648fe0 — __ZN3RBX5Stats4Item15createChildItemEPKc
pub fn stub_648fe0() -> ! {
    todo!("0x648fe0 __ZN3RBX5Stats4Item15createChildItemEPKc")
}

#[doc(alias = "void RBX::Stats::Item::formatValue<double>(double const&)")]
// 0x64915c — __ZN3RBX5Stats4Item11formatValueIdEEvRKT_
pub fn stub_64915c() -> ! {
    todo!("0x64915c __ZN3RBX5Stats4Item11formatValueIdEEvRKT_")
}

#[doc(alias = "void RBX::Stats::Item::formatValue<float>(float const&)")]
// 0x649180 — __ZN3RBX5Stats4Item11formatValueIfEEvRKT_
pub fn stub_649180() -> ! {
    todo!("0x649180 __ZN3RBX5Stats4Item11formatValueIfEEvRKT_")
}

#[doc(alias = "void RBX::Stats::Item::formatValue<int>(int const&)")]
// 0x6491a8 — __ZN3RBX5Stats4Item11formatValueIiEEvRKT_
pub fn stub_6491a8() -> ! {
    todo!("0x6491a8 __ZN3RBX5Stats4Item11formatValueIiEEvRKT_")
}

#[doc(alias = "void RBX::Stats::Item::formatValue<unsigned long>(unsigned long const&)")]
// 0x6491d8 — __ZN3RBX5Stats4Item11formatValueImEEvRKT_
pub fn stub_6491d8() -> ! {
    todo!("0x6491d8 __ZN3RBX5Stats4Item11formatValueImEEvRKT_")
}

#[doc(alias = "void RBX::Stats::Item::formatValue<unsigned long long>(unsigned long long const&)")]
// 0x649204 — __ZN3RBX5Stats4Item11formatValueIyEEvRKT_
pub fn stub_649204() -> ! {
    todo!("0x649204 __ZN3RBX5Stats4Item11formatValueIyEEvRKT_")
}

#[doc(alias = "void RBX::Stats::Item::formatValue<unsigned int>(unsigned int const&)")]
// 0x649240 — __ZN3RBX5Stats4Item11formatValueIjEEvRKT_
pub fn stub_649240() -> ! {
    todo!("0x649240 __ZN3RBX5Stats4Item11formatValueIjEEvRKT_")
}

#[doc(alias = "void RBX::Stats::Item::formatValue<bool>(bool const&)")]
// 0x64926c — __ZN3RBX5Stats4Item11formatValueIbEEvRKT_
pub fn stub_64926c() -> ! {
    todo!("0x64926c __ZN3RBX5Stats4Item11formatValueIbEEvRKT_")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int,(RBX::Time::SampleMethod)1>(char const*,RBX::TotalCountTimeInterval<int,(RBX::Time::SampleMethod)1> const&)")]
// 0x6492b4 — __ZN3RBX5Stats4Item20createBoundChildItemIiLNS_4Time12SampleMethodE1EEEPS1_PKcRKNS_22TotalCountTimeIntervalIT_XT0_EEE
pub fn stub_6492b4() -> ! {
    todo!("0x6492b4 __ZN3RBX5Stats4Item20createBoundChildItemIiLNS_4Time12SampleMethodE1EEEPS1_PKcRKNS_22TotalCountTimeIntervalIT_XT0_EEE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<int,double>(char const*,RBX::RunningAverage<int,double> const&)")]
// 0x649468 — __ZN3RBX5Stats4Item20createBoundChildItemIidEEPS1_PKcRKNS_14RunningAverageIT_T0_EE
pub fn stub_649468() -> ! {
    todo!("0x649468 __ZN3RBX5Stats4Item20createBoundChildItemIidEEPS1_PKcRKNS_14RunningAverageIT_T0_EE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<double,double>(char const*,RBX::RunningAverage<double,double> const&)")]
// 0x64961c — __ZN3RBX5Stats4Item20createBoundChildItemIddEEPS1_PKcRKNS_14RunningAverageIT_T0_EE
pub fn stub_64961c() -> ! {
    todo!("0x64961c __ZN3RBX5Stats4Item20createBoundChildItemIddEEPS1_PKcRKNS_14RunningAverageIT_T0_EE")
}

#[doc(alias = "RBX::Stats::Item::createBoundChildItem(RBX::Profiling::Profiler const&)")]
// 0x6497d0 — __ZN3RBX5Stats4Item20createBoundChildItemERKNS_9Profiling8ProfilerE
pub fn stub_6497d0() -> ! {
    todo!("0x6497d0 __ZN3RBX5Stats4Item20createBoundChildItemERKNS_9Profiling8ProfilerE")
}

#[doc(alias = "RBX::Stats::Item::createBoundMemChildItem(char const*,unsigned long const&)")]
// 0x649988 — __ZN3RBX5Stats4Item23createBoundMemChildItemEPKcRKm
pub fn stub_649988() -> ! {
    todo!("0x649988 __ZN3RBX5Stats4Item23createBoundMemChildItemEPKcRKm")
}

#[doc(alias = "RBX::Stats::Item::createBoundPercentChildItem(char const*,float const&)")]
// 0x649b3c — __ZN3RBX5Stats4Item27createBoundPercentChildItemEPKcRKf
pub fn stub_649b3c() -> ! {
    todo!("0x649b3c __ZN3RBX5Stats4Item27createBoundPercentChildItemEPKcRKf")
}

#[doc(alias = "RBX::registerStatsClasses(void)")]
// 0x649cf0 — __ZN3RBX20registerStatsClassesEv
pub fn stub_649cf0() -> ! {
    todo!("0x649cf0 __ZN3RBX20registerStatsClassesEv")
}

#[doc(alias = "RBX::Stats::Item::getStringValue2(void)")]
// 0x64a540 — __ZN3RBX5Stats4Item15getStringValue2Ev
pub fn stub_64a540() -> ! {
    todo!("0x64a540 __ZN3RBX5Stats4Item15getStringValue2Ev")
}

#[doc(alias = "RBX::Stats::Item::getValue(void)")]
// 0x64a584 — __ZN3RBX5Stats4Item8getValueEv
pub fn stub_64a584() -> ! {
    todo!("0x64a584 __ZN3RBX5Stats4Item8getValueEv")
}

#[doc(alias = "std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::operator[](std::string const&)")]
// 0x64aa08 — __ZNSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEixERS3_
pub fn stub_64aa08() -> ! {
    todo!("0x64aa08 __ZNSt3mapISsSsSt4lessISsESaISt4pairIKSsSsEEEixERS3_")
}

#[doc(alias = "void RBX::RunningAverage<double,double>::iter<RBX::Stats::JobStepWindowWriter>(RBX::Stats::JobStepWindowWriter &)const")]
// 0x64ac28 — __ZNK3RBX14RunningAverageIddE4iterINS_5Stats19JobStepWindowWriterEEEvRT_
pub fn stub_64ac28() -> ! {
    todo!("0x64ac28 __ZNK3RBX14RunningAverageIddE4iterINS_5Stats19JobStepWindowWriterEEEvRT_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const> *,std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>)")]
// 0x64ac68 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_
pub fn stub_64ac68() -> ! {
    todo!("0x64ac68 __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list_av_4<RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>::type> boost::bind<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &,RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>>(void (RBX::Stats::StatsService::*)(rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &),RBX::Stats::StatsService*,boost::arg<1>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,boost::reference_wrapper<bool>)")]
// 0x64acd4 — __ZN5boost4bindIvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS1_13TaskScheduler3JobEEENS4_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbPS3_NS_3argILi1EEESE_NS_17reference_wrapperIbEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISN_T0_T1_T2_T3_EENSL_9list_av_4IT4_T5_T6_T7_E4typeEEEMSQ_FSN_SR_SS_ST_ESW_SX_SY_SZ_
pub fn stub_64acd4() -> ! {
    todo!("0x64acd4 __ZN5boost4bindIvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS1_13TaskScheduler3JobEEENS4_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbPS3_NS_3argILi1EEESE_NS_17reference_wrapperIbEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISN_T0_T1_T2_T3_EENSL_9list_av_4IT4_T5_T6_T7_E4typeEEEMSQ_FSN_SR_SS_ST_ESW_SX_SY_SZ_")
}

#[doc(alias = "RBX::Stats::Item::~Item()")]
// 0x64bbc0 — __ZN3RBX5Stats4ItemD1Ev
pub fn stub_64bbc0() -> ! {
    todo!("0x64bbc0 __ZN3RBX5Stats4ItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::Item::~Item()")]
// 0x64bc00 — __ZThn32_N3RBX5Stats4ItemD0Ev
pub fn stub_64bc00() -> ! {
    todo!("0x64bc00 __ZThn32_N3RBX5Stats4ItemD0Ev")
}

#[doc(alias = "RBX::Stats::StatsService::~StatsService()")]
// 0x64bcd8 — __ZN3RBX5Stats12StatsServiceD1Ev
pub fn stub_64bcd8() -> ! {
    todo!("0x64bcd8 __ZN3RBX5Stats12StatsServiceD1Ev")
}

#[doc(alias = "RBX::Stats::StatsService::~StatsService()")]
// 0x64bcdc — __ZN3RBX5Stats12StatsServiceD0Ev
pub fn stub_64bcdc() -> ! {
    todo!("0x64bcdc __ZN3RBX5Stats12StatsServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::StatsService::~StatsService()")]
// 0x64bd80 — __ZThn32_N3RBX5Stats12StatsServiceD1Ev
pub fn stub_64bd80() -> ! {
    todo!("0x64bd80 __ZThn32_N3RBX5Stats12StatsServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::StatsService::~StatsService()")]
// 0x64bd88 — __ZThn32_N3RBX5Stats12StatsServiceD0Ev
pub fn stub_64bd88() -> ! {
    todo!("0x64bd88 __ZThn32_N3RBX5Stats12StatsServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::StatsService::~StatsService()")]
// 0x64be2c — __ZThn36_N3RBX5Stats12StatsServiceD1Ev
pub fn stub_64be2c() -> ! {
    todo!("0x64be2c __ZThn36_N3RBX5Stats12StatsServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::StatsService::~StatsService()")]
// 0x64be34 — __ZThn36_N3RBX5Stats12StatsServiceD0Ev
pub fn stub_64be34() -> ! {
    todo!("0x64be34 __ZThn36_N3RBX5Stats12StatsServiceD0Ev")
}

#[doc(alias = "RBX::Stats::TypedPercentItem::~TypedPercentItem()")]
// 0x64c350 — __ZN3RBX5Stats16TypedPercentItemD1Ev
pub fn stub_64c350() -> ! {
    todo!("0x64c350 __ZN3RBX5Stats16TypedPercentItemD1Ev")
}

#[doc(alias = "RBX::Stats::TypedPercentItem::~TypedPercentItem()")]
// 0x64c494 — __ZN3RBX5Stats16TypedPercentItemD0Ev
pub fn stub_64c494() -> ! {
    todo!("0x64c494 __ZN3RBX5Stats16TypedPercentItemD0Ev")
}

#[doc(alias = "RBX::Stats::TypedPercentItem::update(void)")]
// 0x64c5f0 — __ZN3RBX5Stats16TypedPercentItem6updateEv
pub fn stub_64c5f0() -> ! {
    todo!("0x64c5f0 __ZN3RBX5Stats16TypedPercentItem6updateEv")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedPercentItem::~TypedPercentItem()")]
// 0x64c638 — __ZThn32_N3RBX5Stats16TypedPercentItemD1Ev
pub fn stub_64c638() -> ! {
    todo!("0x64c638 __ZThn32_N3RBX5Stats16TypedPercentItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedPercentItem::~TypedPercentItem()")]
// 0x64c77c — __ZThn32_N3RBX5Stats16TypedPercentItemD0Ev
pub fn stub_64c77c() -> ! {
    todo!("0x64c77c __ZThn32_N3RBX5Stats16TypedPercentItemD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedPercentItem::~TypedPercentItem()")]
// 0x64c8d4 — __ZThn36_N3RBX5Stats16TypedPercentItemD1Ev
pub fn stub_64c8d4() -> ! {
    todo!("0x64c8d4 __ZThn36_N3RBX5Stats16TypedPercentItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedPercentItem::~TypedPercentItem()")]
// 0x64ca18 — __ZThn36_N3RBX5Stats16TypedPercentItemD0Ev
pub fn stub_64ca18() -> ! {
    todo!("0x64ca18 __ZThn36_N3RBX5Stats16TypedPercentItemD0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x64cb70 — __ZN3RBX5Stats14TypedStatsItemIfED1Ev
pub fn stub_64cb70() -> ! {
    todo!("0x64cb70 __ZN3RBX5Stats14TypedStatsItemIfED1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x64ccb8 — __ZThn32_N3RBX5Stats14TypedStatsItemIfED1Ev
pub fn stub_64ccb8() -> ! {
    todo!("0x64ccb8 __ZThn32_N3RBX5Stats14TypedStatsItemIfED1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<float>::~TypedStatsItem()")]
// 0x64ce00 — __ZThn36_N3RBX5Stats14TypedStatsItemIfED0Ev
pub fn stub_64ce00() -> ! {
    todo!("0x64ce00 __ZThn36_N3RBX5Stats14TypedStatsItemIfED0Ev")
}

#[doc(alias = "boost::function0<float>::operator()(void)const")]
// 0x64cf58 — __ZNK5boost9function0IfEclEv
pub fn stub_64cf58() -> ! {
    todo!("0x64cf58 __ZNK5boost9function0IfEclEv")
}

#[doc(alias = "boost::function0<float>::clear(void)")]
// 0x64d020 — __ZN5boost9function0IfE5clearEv
pub fn stub_64d020() -> ! {
    todo!("0x64d020 __ZN5boost9function0IfE5clearEv")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<float>::deref(float const*)")]
// 0x64d050 — __ZN3RBX5Stats14TypedStatsItemIfE5derefEPKf
pub fn stub_64d050() -> ! {
    todo!("0x64d050 __ZN3RBX5Stats14TypedStatsItemIfE5derefEPKf")
}

#[doc(alias = "RBX::Stats::TypedMemItem::~TypedMemItem()")]
// 0x64d35c — __ZN3RBX5Stats12TypedMemItemD1Ev
pub fn stub_64d35c() -> ! {
    todo!("0x64d35c __ZN3RBX5Stats12TypedMemItemD1Ev")
}

#[doc(alias = "RBX::Stats::TypedMemItem::~TypedMemItem()")]
// 0x64d4a0 — __ZN3RBX5Stats12TypedMemItemD0Ev
pub fn stub_64d4a0() -> ! {
    todo!("0x64d4a0 __ZN3RBX5Stats12TypedMemItemD0Ev")
}

#[doc(alias = "RBX::Stats::TypedMemItem::update(void)")]
// 0x64d5f8 — __ZN3RBX5Stats12TypedMemItem6updateEv
pub fn stub_64d5f8() -> ! {
    todo!("0x64d5f8 __ZN3RBX5Stats12TypedMemItem6updateEv")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedMemItem::~TypedMemItem()")]
// 0x64d614 — __ZThn32_N3RBX5Stats12TypedMemItemD1Ev
pub fn stub_64d614() -> ! {
    todo!("0x64d614 __ZThn32_N3RBX5Stats12TypedMemItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedMemItem::~TypedMemItem()")]
// 0x64d758 — __ZThn32_N3RBX5Stats12TypedMemItemD0Ev
pub fn stub_64d758() -> ! {
    todo!("0x64d758 __ZThn32_N3RBX5Stats12TypedMemItemD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedMemItem::~TypedMemItem()")]
// 0x64d8b0 — __ZThn36_N3RBX5Stats12TypedMemItemD1Ev
pub fn stub_64d8b0() -> ! {
    todo!("0x64d8b0 __ZThn36_N3RBX5Stats12TypedMemItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedMemItem::~TypedMemItem()")]
// 0x64d9f4 — __ZThn36_N3RBX5Stats12TypedMemItemD0Ev
pub fn stub_64d9f4() -> ! {
    todo!("0x64d9f4 __ZThn36_N3RBX5Stats12TypedMemItemD0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long>::~TypedStatsItem()")]
// 0x64db4c — __ZN3RBX5Stats14TypedStatsItemImED1Ev
pub fn stub_64db4c() -> ! {
    todo!("0x64db4c __ZN3RBX5Stats14TypedStatsItemImED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long>::~TypedStatsItem()")]
// 0x64dc90 — __ZN3RBX5Stats14TypedStatsItemImED0Ev
pub fn stub_64dc90() -> ! {
    todo!("0x64dc90 __ZN3RBX5Stats14TypedStatsItemImED0Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long>::update(void)")]
// 0x64dde8 — __ZN3RBX5Stats14TypedStatsItemImE6updateEv
pub fn stub_64dde8() -> ! {
    todo!("0x64dde8 __ZN3RBX5Stats14TypedStatsItemImE6updateEv")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long>::~TypedStatsItem()")]
// 0x64de1c — __ZThn32_N3RBX5Stats14TypedStatsItemImED1Ev
pub fn stub_64de1c() -> ! {
    todo!("0x64de1c __ZThn32_N3RBX5Stats14TypedStatsItemImED1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long>::~TypedStatsItem()")]
// 0x64df60 — __ZThn32_N3RBX5Stats14TypedStatsItemImED0Ev
pub fn stub_64df60() -> ! {
    todo!("0x64df60 __ZThn32_N3RBX5Stats14TypedStatsItemImED0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long>::~TypedStatsItem()")]
// 0x64e0b8 — __ZThn36_N3RBX5Stats14TypedStatsItemImED1Ev
pub fn stub_64e0b8() -> ! {
    todo!("0x64e0b8 __ZThn36_N3RBX5Stats14TypedStatsItemImED1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<unsigned long>::~TypedStatsItem()")]
// 0x64e1fc — __ZThn36_N3RBX5Stats14TypedStatsItemImED0Ev
pub fn stub_64e1fc() -> ! {
    todo!("0x64e1fc __ZThn36_N3RBX5Stats14TypedStatsItemImED0Ev")
}

#[doc(alias = "boost::function0<unsigned long>::operator()(void)const")]
// 0x64e354 — __ZNK5boost9function0ImEclEv
pub fn stub_64e354() -> ! {
    todo!("0x64e354 __ZNK5boost9function0ImEclEv")
}

#[doc(alias = "boost::function0<unsigned long>::clear(void)")]
// 0x64e418 — __ZN5boost9function0ImE5clearEv
pub fn stub_64e418() -> ! {
    todo!("0x64e418 __ZN5boost9function0ImE5clearEv")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<unsigned long>::deref(unsigned long const*)")]
// 0x64e444 — __ZN3RBX5Stats14TypedStatsItemImE5derefEPKm
pub fn stub_64e444() -> ! {
    todo!("0x64e444 __ZN3RBX5Stats14TypedStatsItemImE5derefEPKm")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long const&,unsigned long const& (*)(unsigned long const*),boost::_bi::list1<boost::_bi::value<unsigned long const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x64e448 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKmPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE
pub fn stub_64e448() -> ! {
    todo!("0x64e448 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIRKmPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEE6manageERKNS1_15function_bufferERSG_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned long const&,unsigned long const& (*)(unsigned long const*),boost::_bi::list1<boost::_bi::value<unsigned long const*>>>,unsigned long>::invoke(boost::detail::function::function_buffer &)")]
// 0x64e4a8 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKmPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEmE6invokeERNS1_15function_bufferE
pub fn stub_64e4a8() -> ! {
    todo!("0x64e4a8 __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIRKmPFS6_PS5_ENS3_5list1INS3_5valueIS7_EEEEEEmE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "RBX::TotalCountTimeInterval<int,(RBX::Time::SampleMethod)1>::getCount(void)const")]
// 0x651214 — __ZNK3RBX22TotalCountTimeIntervalIiLNS_4Time12SampleMethodE1EE8getCountEv
pub fn stub_651214() -> ! {
    todo!("0x651214 __ZNK3RBX22TotalCountTimeIntervalIiLNS_4Time12SampleMethodE1EE8getCountEv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
// 0x652a78 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_
pub fn stub_652a78() -> ! {
    todo!("0x652a78 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)")]
// 0x652cb0 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_
pub fn stub_652cb0() -> ! {
    todo!("0x652cb0 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
// 0x652cd4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
pub fn stub_652cd4() -> ! {
    todo!("0x652cd4 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::~node_constructor()")]
// 0x652d24 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev
pub fn stub_652d24() -> ! {
    todo!("0x652d24 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
// 0x652d40 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
pub fn stub_652d40() -> ! {
    todo!("0x652d40 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
// 0x652e68 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
pub fn stub_652e68() -> ! {
    todo!("0x652e68 __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
// 0x652ef8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
pub fn stub_652ef8() -> ! {
    todo!("0x652ef8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0x652f24 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE
pub fn stub_652f24() -> ! {
    todo!("0x652f24 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct(void)")]
// 0x652f7c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv
pub fn stub_652f7c() -> ! {
    todo!("0x652f7c __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0x652fe0 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_
pub fn stub_652fe0() -> ! {
    todo!("0x652fe0 __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0x65304c — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_65304c() -> ! {
    todo!("0x65304c __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::operator()<boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>&> &,int)")]
// 0x65308c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEclINS_4_mfi3mf3IvS5_NSA_IKNS3_13TaskScheduler3JobEEESG_RbEENS0_5list1IRSR_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_65308c() -> ! {
    todo!("0x65308c __ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEclINS_4_mfi3mf3IvS5_NSA_IKNS3_13TaskScheduler3JobEEESG_RbEENS0_5list1IRSR_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::Stats::StatsService,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>::operator()(RBX::Stats::StatsService*,rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)const")]
// 0x6531ac — __ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_
pub fn stub_6531ac() -> ! {
    todo!("0x6531ac __ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_")
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::list4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)")]
// 0x6532e4 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_
pub fn stub_6532e4() -> ! {
    todo!("0x6532e4 __ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::storage4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)")]
// 0x6533c4 — __ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_
pub fn stub_6533c4() -> ! {
    todo!("0x6533c4 __ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")
}

#[doc(alias = "RBX::Stats::JobStepWindowWriter::operator()(double)")]
// 0x6534b4 — __ZN3RBX5Stats19JobStepWindowWriterclEd
pub fn stub_6534b4() -> ! {
    todo!("0x6534b4 __ZN3RBX5Stats19JobStepWindowWriterclEd")
}

#[doc(alias = "rbx_core::SharedPtr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0x6534f8 — __ZN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEC2IS5_EEPT_
pub fn stub_6534f8() -> ! {
    todo!("0x6534f8 __ZN5boost10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEC2IS5_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> *)")]
// 0x6535cc — __ZN5boost6detail12shared_countC2ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEPT_
pub fn stub_6535cc() -> ! {
    todo!("0x6535cc __ZN5boost6detail12shared_countC2ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>::~sp_counted_impl_p()")]
// 0x6536c4 — __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEED1Ev
pub fn stub_6536c4() -> ! {
    todo!("0x6536c4 __ZN5boost6detail17sp_counted_impl_pISt18basic_stringstreamIcSt11char_traitsIcESaIcEEED1Ev")
}

