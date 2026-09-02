//! core shard DQ — 100 core stubs EA-sorted, next uncovered after DP 0x7fdfe4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::signals::signal<void ()(std::pair<RBX::Primitive *,RBX::Primitive *>)>::slot::~slot()")]
// 0x7fe010 — __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slotD0Ev
pub fn stub_7fe010() -> ! {
    todo!("0x7fe010 __ZN3rbx7signals6signalIFvSt4pairIPN3RBX9PrimitiveES5_EEE4slotD0Ev")
}

#[doc(alias = "RBX::FunctionalTest::warn(std::string)")]
// 0x7fe6e8 — __ZN3RBX14FunctionalTest4warnESs
pub fn stub_7fe6e8() -> ! {
    todo!("0x7fe6e8 __ZN3RBX14FunctionalTest4warnESs")
}

#[doc(alias = "RBX::FunctionalTest::pass(std::string)")]
// 0x7fe914 — __ZN3RBX14FunctionalTest4passESs
pub fn stub_7fe914() -> ! {
    todo!("0x7fe914 __ZN3RBX14FunctionalTest4passESs")
}

#[doc(alias = "RBX::FunctionalTest::error(std::string)")]
// 0x7feb40 — __ZN3RBX14FunctionalTest5errorESs
pub fn stub_7feb40() -> ! {
    todo!("0x7feb40 __ZN3RBX14FunctionalTest5errorESs")
}

#[doc(alias = "RBX::TestService::run(boost::function<void ()(void)>,boost::function<void ()(std::string)>)")]
// 0x7fed6c — __ZN3RBX11TestService3runEN5boost8functionIFvvEEENS2_IFvSsEEE
pub fn stub_7fed6c() -> ! {
    todo!("0x7fed6c __ZN3RBX11TestService3runEN5boost8functionIFvvEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::TestService::done(void)")]
// 0x8005d8 — __ZN3RBX11TestService4doneEv
pub fn stub_8005d8() -> ! {
    todo!("0x8005d8 __ZN3RBX11TestService4doneEv")
}

#[doc(alias = "RBX::TestService::getCommandNames(void)")]
// 0x800744 — __ZN3RBX11TestService15getCommandNamesEv
pub fn stub_800744() -> ! {
    todo!("0x800744 __ZN3RBX11TestService15getCommandNamesEv")
}

#[doc(alias = "RBX::TestService::doCommand(std::string)")]
// 0x8008f0 — __ZN3RBX11TestService9doCommandESs
pub fn stub_8008f0() -> ! {
    todo!("0x8008f0 __ZN3RBX11TestService9doCommandESs")
}

#[doc(alias = "RBX::TestService::isCommandEnabled(std::string)")]
// 0x800a34 — __ZN3RBX11TestService16isCommandEnabledESs
pub fn stub_800a34() -> ! {
    todo!("0x800a34 __ZN3RBX11TestService16isCommandEnabledESs")
}

#[doc(alias = "RBX::TestService::isCommandChecked(std::string)")]
// 0x800b5c — __ZN3RBX11TestService16isCommandCheckedESs
pub fn stub_800b5c() -> ! {
    todo!("0x800b5c __ZN3RBX11TestService16isCommandCheckedESs")
}

#[doc(alias = "RBX::TestService::TestService(void)")]
// 0x800c84 — __ZN3RBX11TestServiceC1Ev
pub fn stub_800c84() -> ! {
    todo!("0x800c84 __ZN3RBX11TestServiceC1Ev")
}

#[doc(alias = "RBX::TestService::TestService(void)")]
// 0x800c88 — __ZN3RBX11TestServiceC2Ev
pub fn stub_800c88() -> ! {
    todo!("0x800c88 __ZN3RBX11TestServiceC2Ev")
}

#[doc(alias = "RBX::TestService::~TestService()")]
// 0x801080 — __ZN3RBX11TestServiceD0Ev
pub fn stub_801080() -> ! {
    todo!("0x801080 __ZN3RBX11TestServiceD0Ev")
}

#[doc(alias = "RBX::TestService::~TestService()")]
// 0x801120 — __ZN3RBX11TestServiceD1Ev
pub fn stub_801120() -> ! {
    todo!("0x801120 __ZN3RBX11TestServiceD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::TestService::~TestService()")]
// 0x801124 — __ZThn32_N3RBX11TestServiceD0Ev
// was: non-virtual thunk to RBX::TestService::~TestService()
pub fn stub_801124() -> ! {
    todo!("0x801124 __ZThn32_N3RBX11TestServiceD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::TestService::~TestService()")]
// 0x80112c — __ZThn36_N3RBX11TestServiceD0Ev
// was: non-virtual thunk to RBX::TestService::~TestService()
pub fn stub_80112c() -> ! {
    todo!("0x80112c __ZThn36_N3RBX11TestServiceD0Ev")
}

#[doc(alias = "RBX::TestService::~TestService()")]
// 0x801134 — __ZN3RBX11TestServiceD2Ev
pub fn stub_801134() -> ! {
    todo!("0x801134 __ZN3RBX11TestServiceD2Ev")
}

#[doc(alias = "non-virtual thunk to RBX::TestService::~TestService()")]
// 0x80143c — __ZThn32_N3RBX11TestServiceD1Ev
// was: non-virtual thunk to RBX::TestService::~TestService()
pub fn stub_80143c() -> ! {
    todo!("0x80143c __ZThn32_N3RBX11TestServiceD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::TestService::~TestService()")]
// 0x801444 — __ZThn36_N3RBX11TestServiceD1Ev
// was: non-virtual thunk to RBX::TestService::~TestService()
pub fn stub_801444() -> ! {
    todo!("0x801444 __ZThn36_N3RBX11TestServiceD1Ev")
}

#[doc(alias = "RBX::TestService::stop(void)")]
// 0x80191c — __ZN3RBX11TestService4stopEv
pub fn stub_80191c() -> ! {
    todo!("0x80191c __ZN3RBX11TestService4stopEv")
}

#[doc(alias = "RBX::TestService::stillWaiting(int,double)")]
// 0x801968 — __ZN3RBX11TestService12stillWaitingEid
pub fn stub_801968() -> ! {
    todo!("0x801968 __ZN3RBX11TestService12stillWaitingEid")
}

#[doc(alias = "RBX::TestService::onTimeout(int,double)")]
// 0x801a7c — __ZN3RBX11TestService9onTimeoutEid
pub fn stub_801a7c() -> ! {
    todo!("0x801a7c __ZN3RBX11TestService9onTimeoutEid")
}

#[doc(alias = "RBX::TestService::setConfiguration(void)")]
// 0x801cd4 — __ZN3RBX11TestService16setConfigurationEv
pub fn stub_801cd4() -> ! {
    todo!("0x801cd4 __ZN3RBX11TestService16setConfigurationEv")
}

#[doc(alias = "RBX::TestService::restoreConfiguration(void)")]
// 0x802af4 — __ZN3RBX11TestService20restoreConfigurationEv
pub fn stub_802af4() -> ! {
    todo!("0x802af4 __ZN3RBX11TestService20restoreConfigurationEv")
}

#[doc(alias = "RBX::TestService::getVerb(std::string)")]
// 0x802b34 — __ZN3RBX11TestService7getVerbESs
pub fn stub_802b34() -> ! {
    todo!("0x802b34 __ZN3RBX11TestService7getVerbESs")
}

#[doc(alias = "RBX::TestService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x802cdc — __ZN3RBX11TestService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_802cdc() -> ! {
    todo!("0x802cdc __ZN3RBX11TestService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::FunctionalTest::FunctionalTest(void)")]
// 0x802fa0 — __ZN3RBX14FunctionalTestC2Ev
pub fn stub_802fa0() -> ! {
    todo!("0x802fa0 __ZN3RBX14FunctionalTestC2Ev")
}

#[doc(alias = "RBX::FunctionalTest::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x803140 — __ZN3RBX14FunctionalTest17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_803140() -> ! {
    todo!("0x803140 __ZN3RBX14FunctionalTest17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::TestService>,int,double>::type> boost::bind<void,RBX::TestService,int,double,rbx_core::SharedPtr<RBX::TestService>,int,double>(void (RBX::TestService::*)(int,double),rbx_core::SharedPtr<RBX::TestService>,int,double)")]
// 0x804324 — __ZN5boost4bindIvN3RBX11TestServiceEidNS_10shared_ptrIS2_EEidEENS_3_bi6bind_tIT_NS_4_mfi3mf2IS7_T0_T1_T2_EENS5_9list_av_3IT3_T4_T5_E4typeEEEMSA_FS7_SB_SC_ESF_SG_SH_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list_av_3<boost::shared_ptr<RBX::TestService>,int,double>::type> boost::bind<void,RBX::TestService,int,double,boost::shared_ptr<RBX::TestService>,int,double>(void (RBX::TestService::*)(int,double),boost::shared_ptr<RBX::TestService>,int,double)
pub fn stub_804324() -> ! {
    todo!("0x804324 __ZN5boost4bindIvN3RBX11TestServiceEidNS_10shared_ptrIS2_EEidEENS_3_bi6bind_tIT_NS_4_mfi3mf2IS7_T0_T1_T2_EENS5_9list_av_3IT3_T4_T5_E4typeEEEMSA_FS7_SB_SC_ESF_SG_SH_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TestService> RBX::shared_from<RBX::TestService>(RBX::TestService*)")]
// 0x804454 — __ZN3RBX11shared_fromINS_11TestServiceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::TestService> RBX::shared_from<RBX::TestService>(RBX::TestService*)
pub fn stub_804454() -> ! {
    todo!("0x804454 __ZN3RBX11shared_fromINS_11TestServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::TestService>,int>::type> boost::bind<void,RBX::TestService,int,rbx_core::SharedPtr<RBX::TestService>,int>(void (RBX::TestService::*)(int),rbx_core::SharedPtr<RBX::TestService>,int)")]
// 0x804740 — __ZN5boost4bindIvN3RBX11TestServiceEiNS_10shared_ptrIS2_EEiEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS7_T0_T1_EENS5_9list_av_2IT2_T3_E4typeEEEMSA_FS7_SB_ESE_SF_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list_av_2<boost::shared_ptr<RBX::TestService>,int>::type> boost::bind<void,RBX::TestService,int,boost::shared_ptr<RBX::TestService>,int>(void (RBX::TestService::*)(int),boost::shared_ptr<RBX::TestService>,int)
pub fn stub_804740() -> ! {
    todo!("0x804740 __ZN5boost4bindIvN3RBX11TestServiceEiNS_10shared_ptrIS2_EEiEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS7_T0_T1_EENS5_9list_av_2IT2_T3_E4typeEEEMSA_FS7_SB_ESE_SF_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TestService>::operator=(rbx_core::SharedPtr<RBX::TestService> const&)")]
// 0x804bb4 — __ZN5boost10shared_ptrIN3RBX11TestServiceEEaSERKS3_
// was: boost::shared_ptr<RBX::TestService>::operator=(boost::shared_ptr<RBX::TestService> const&)
pub fn stub_804bb4() -> ! {
    todo!("0x804bb4 __ZN5boost10shared_ptrIN3RBX11TestServiceEEaSERKS3_")
}

#[doc(alias = "RBX::FunctionalTest::~FunctionalTest()")]
// 0x804c0c — __ZN3RBX14FunctionalTestD1Ev
pub fn stub_804c0c() -> ! {
    todo!("0x804c0c __ZN3RBX14FunctionalTestD1Ev")
}

#[doc(alias = "RBX::FunctionalTest::~FunctionalTest()")]
// 0x804d18 — __ZN3RBX14FunctionalTestD0Ev
pub fn stub_804d18() -> ! {
    todo!("0x804d18 __ZN3RBX14FunctionalTestD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()")]
// 0x804e4c — __ZThn32_N3RBX14FunctionalTestD1Ev
// was: non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()
pub fn stub_804e4c() -> ! {
    todo!("0x804e4c __ZThn32_N3RBX14FunctionalTestD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()")]
// 0x804f54 — __ZThn32_N3RBX14FunctionalTestD0Ev
// was: non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()
pub fn stub_804f54() -> ! {
    todo!("0x804f54 __ZThn32_N3RBX14FunctionalTestD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()")]
// 0x805084 — __ZThn36_N3RBX14FunctionalTestD1Ev
// was: non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()
pub fn stub_805084() -> ! {
    todo!("0x805084 __ZThn36_N3RBX14FunctionalTestD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()")]
// 0x80518c — __ZThn36_N3RBX14FunctionalTestD0Ev
// was: non-virtual thunk to RBX::FunctionalTest::~FunctionalTest()
pub fn stub_80518c() -> ! {
    todo!("0x80518c __ZThn36_N3RBX14FunctionalTestD0Ev")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x8078b8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
pub fn stub_8078b8() -> ! {
    todo!("0x8078b8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>,std::string,std::string const&>::invoke(boost::detail::function::function_buffer &,std::string const&)")]
// 0x807918 — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEESsSA_E6invokeERNS1_15function_bufferESA_
pub fn stub_807918() -> ! {
    todo!("0x807918 __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEESsSA_E6invokeERNS1_15function_bufferESA_")
}

#[doc(alias = "std::string boost::_bi::bind_t<std::string,boost::_mfi::mf1<std::string,RBX::TestService,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::TestService*>,boost::arg<1>>>::operator()<std::string>(std::string const&)")]
// 0x807924 — __ZN5boost3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclISsEESsRKT_
pub fn stub_807924() -> ! {
    todo!("0x807924 __ZN5boost3_bi6bind_tISsNS_4_mfi3mf1ISsN3RBX11TestServiceERKSsEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclISsEESsRKT_")
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// 0x808278 — __ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_
// was: boost::_bi::list6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list6(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub fn stub_808278() -> ! {
    todo!("0x808278 __ZN5boost3_bi5list6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::storage6(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
// 0x808350 — __ZN5boost3_bi8storage6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_
// was: boost::_bi::storage6<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::storage6(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)
pub fn stub_808350() -> ! {
    todo!("0x808350 __ZN5boost3_bi8storage6INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES7_S8_SA_SB_SC_SD_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage5(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// 0x808428 — __ZN5boost3_bi8storage5INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_SA_SB_SC_
// was: boost::_bi::storage5<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage5(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>)
pub fn stub_808428() -> ! {
    todo!("0x808428 __ZN5boost3_bi8storage5INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_SA_SB_SC_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// 0x808500 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_
// was: boost::_bi::storage4<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)
pub fn stub_808500() -> ! {
    todo!("0x808500 __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>)")]
// 0x8085d8 — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_
// was: boost::_bi::storage3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::arg<1>)
pub fn stub_8085d8() -> ! {
    todo!("0x8085d8 __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")]
// 0x8086b0 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>)
pub fn stub_8086b0() -> ! {
    todo!("0x8086b0 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x808a68 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_808a68() -> ! {
    todo!("0x808a68 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x808d50 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::TestService,int>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_808d50() -> ! {
    todo!("0x808d50 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS3_5list2INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>)")]
// 0x808eac — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>>::list2(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>)
pub fn stub_808eac() -> ! {
    todo!("0x808eac __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEEEC2ES7_S8_")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>)")]
// 0x809280 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>)
pub fn stub_809280() -> ! {
    todo!("0x809280 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x809384 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_809384() -> ! {
    todo!("0x809384 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x8093a0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_8093a0() -> ! {
    todo!("0x8093a0 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &)const")]
// 0x8093b8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &)const
pub fn stub_8093b8() -> ! {
    todo!("0x8093b8 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x8094ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_8094ac() -> ! {
    todo!("0x8094ac __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x80959c — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_80959c() -> ! {
    todo!("0x80959c __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS5_5list3INS5_5valueINS_10shared_ptrISA_EEEENSD_IiEENSD_IdEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TestService,int,double> &,boost::_bi::list0 &,int)")]
// 0x809680 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::operator()<boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::TestService,int,double> &,boost::_bi::list0 &,int)
pub fn stub_809680() -> ! {
    todo!("0x809680 __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEclINS_4_mfi3mf2IvS5_idEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x8096a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TestService,int,double>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_8096a4() -> ! {
    todo!("0x8096a4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX11TestServiceEidEENS3_5list3INS3_5valueINS_10shared_ptrIS8_EEEENSB_IiEENSB_IdEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
// 0x80980c — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
// was: boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::list3(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)
pub fn stub_80980c() -> ! {
    todo!("0x80980c __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)")]
// 0x8098ec — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_
// was: boost::_bi::storage3<boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>>::storage3(boost::_bi::value<boost::shared_ptr<RBX::TestService>>,boost::_bi::value<int>,boost::_bi::value<double>)
pub fn stub_8098ec() -> ! {
    todo!("0x8098ec __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX11TestServiceEEEEENS2_IiEENS2_IdEEEC2ES7_S8_S9_")
}

#[doc(alias = "boost::function1<void,double>::dummy::nonnull(void)")]
// 0x8099d0 — __ZN5boost9function1IvdE5dummy7nonnullEv
pub fn stub_8099d0() -> ! {
    todo!("0x8099d0 __ZN5boost9function1IvdE5dummy7nonnullEv")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::resize(unsigned long,RBX::FunctionalTest::Result)")]
// 0x8121c8 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_
pub fn stub_8121c8() -> ! {
    todo!("0x8121c8 __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::push_back(RBX::FunctionalTest::Result const&)")]
// 0x8121fc — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_
pub fn stub_8121fc() -> ! {
    todo!("0x8121fc __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::FunctionalTest::Result,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::operator[](RBX::Name const* const&)")]
// 0x812224 — __ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_812224() -> ! {
    todo!("0x812224 __ZNSt3mapIPKN3RBX4NameENS0_14FunctionalTest6ResultESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
// 0x81227c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_81227c() -> ! {
    todo!("0x81227c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
// 0x812330 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_812330() -> ! {
    todo!("0x812330 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
// 0x812388 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_812388() -> ! {
    todo!("0x812388 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,RBX::FunctionalTest::Result const&)")]
// 0x8123f0 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_8123f0() -> ! {
    todo!("0x8123f0 __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_allocate(unsigned long)")]
// 0x8124d4 — __ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm
pub fn stub_8124d4() -> ! {
    todo!("0x8124d4 __ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::FunctionalTest::Result * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *>(RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *)")]
// 0x8124ec — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_
pub fn stub_8124ec() -> ! {
    todo!("0x8124ec __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,unsigned long,RBX::FunctionalTest::Result const&)")]
// 0x812528 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_812528() -> ! {
    todo!("0x812528 __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::MacroSubstituter::MacroSubstituter(std::string const&)")]
// 0x8126b8 — __ZN3RBX16MacroSubstituterC2ERKSs
pub fn stub_8126b8() -> ! {
    todo!("0x8126b8 __ZN3RBX16MacroSubstituterC2ERKSs")
}

#[doc(alias = "RBX::MacroSubstituter::processLine(int,std::string const&)")]
// 0x812a08 — __ZN3RBX16MacroSubstituter11processLineEiRKSs
pub fn stub_812a08() -> ! {
    todo!("0x812a08 __ZN3RBX16MacroSubstituter11processLineEiRKSs")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Equality(int,std::string const&,char const*,char const*,char const*,char const*)")]
// 0x813180 — __ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_
pub fn stub_813180() -> ! {
    todo!("0x813180 __ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_SimpleSubstitution(int,std::string const&,char const*,char const*)")]
// 0x813924 — __ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_
pub fn stub_813924() -> ! {
    todo!("0x813924 __ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Throw(int,std::string const&,char const*,char const*)")]
// 0x813d10 — __ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_
pub fn stub_813d10() -> ! {
    todo!("0x813d10 __ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_NoThrow(int,std::string const&,char const*,char const*)")]
// 0x81412c — __ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_
pub fn stub_81412c() -> ! {
    todo!("0x81412c __ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_")
}

#[doc(alias = "RBX::MacroSubstituter::doRBX_Test(int,std::string const&,char const*,char const*)")]
// 0x814548 — __ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_
pub fn stub_814548() -> ! {
    todo!("0x814548 __ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_")
}

#[doc(alias = "std::ostream_iterator<char,char,std::char_traits<char>> std::__copy<false,std::random_access_iterator_tag>::copy<char const*,std::ostream_iterator<char,char,std::char_traits<char>>>(char const*,char const*,std::ostream_iterator<char,char,std::char_traits<char>>)")]
// 0x814960 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKcSt16ostream_iteratorIccSt11char_traitsIcEEEET0_T_SA_S9_
pub fn stub_814960() -> ! {
    todo!("0x814960 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKcSt16ostream_iteratorIccSt11char_traitsIcEEEET0_T_SA_S9_")
}

#[doc(alias = "std::ostream_iterator<char,char,std::char_traits<char>>::operator=(char const&)")]
// 0x814998 — __ZNSt16ostream_iteratorIccSt11char_traitsIcEEaSERKc
pub fn stub_814998() -> ! {
    todo!("0x814998 __ZNSt16ostream_iteratorIccSt11char_traitsIcEEaSERKc")
}

#[doc(alias = "RBX::MacroSubstituter::appendArg(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
// 0x815108 — __ZN3RBX16MacroSubstituter9appendArgEPSt6vectorISsSaISsEEN9__gnu_cxx17__normal_iteratorIPKcSsEES9_
pub fn stub_815108() -> ! {
    todo!("0x815108 __ZN3RBX16MacroSubstituter9appendArgEPSt6vectorISsSaISsEEN9__gnu_cxx17__normal_iteratorIPKcSsEES9_")
}

#[doc(alias = "RBX::Region3::Region3(void)")]
// 0x816d04 — __ZN3RBX7Region3C1Ev
pub fn stub_816d04() -> ! {
    todo!("0x816d04 __ZN3RBX7Region3C1Ev")
}

#[doc(alias = "RBX::Region3::init(RBX::Extents const&)")]
// 0x816d64 — __ZN3RBX7Region34initERKNS_7ExtentsE
pub fn stub_816d64() -> ! {
    todo!("0x816d64 __ZN3RBX7Region34initERKNS_7ExtentsE")
}

#[doc(alias = "RBX::Region3::Region3(RBX::Extents const&)")]
// 0x816e3c — __ZN3RBX7Region3C1ERKNS_7ExtentsE
pub fn stub_816e3c() -> ! {
    todo!("0x816e3c __ZN3RBX7Region3C1ERKNS_7ExtentsE")
}

#[doc(alias = "RBX::Region3::minPos(void)const")]
// 0x816e60 — __ZNK3RBX7Region36minPosEv
pub fn stub_816e60() -> ! {
    todo!("0x816e60 __ZNK3RBX7Region36minPosEv")
}

#[doc(alias = "RBX::Region3::maxPos(void)const")]
// 0x816ea8 — __ZNK3RBX7Region36maxPosEv
pub fn stub_816ea8() -> ! {
    todo!("0x816ea8 __ZNK3RBX7Region36maxPosEv")
}

#[doc(alias = "RBX::LibraryService::queueExceptionThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&)")]
// 0x818074 — __ZN3RBX14LibraryService20queueExceptionThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEERKSs
// was: RBX::LibraryService::queueExceptionThread(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>,std::string const&)
pub fn stub_818074() -> ! {
    todo!("0x818074 __ZN3RBX14LibraryService20queueExceptionThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEERKSs")
}

#[doc(alias = "RBX::LibraryService::queueResumeThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
// 0x8182c4 — __ZN3RBX14LibraryService17queueResumeThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEE
// was: RBX::LibraryService::queueResumeThread(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)
pub fn stub_8182c4() -> ! {
    todo!("0x8182c4 __ZN3RBX14LibraryService17queueResumeThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")
}

#[doc(alias = "RBX::LibraryService::resumeAllThreadsWithException(std::string const&)")]
// 0x818408 — __ZN3RBX14LibraryService29resumeAllThreadsWithExceptionERKSs
pub fn stub_818408() -> ! {
    todo!("0x818408 __ZN3RBX14LibraryService29resumeAllThreadsWithExceptionERKSs")
}

#[doc(alias = "RBX::LibraryService::contentReady(std::string const&,std::string const&,RBX::AsyncHttpQueue::RequestResult,std::string const*)")]
// 0x818804 — __ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_
pub fn stub_818804() -> ! {
    todo!("0x818804 __ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_")
}

#[doc(alias = "RBX::LibraryService::contentReadyLocal(std::string const&,boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>)")]
// 0x818d7c — __ZN3RBX14LibraryService17contentReadyLocalERKSsN5boost10flyweights9flyweightINS_15ProtectedStringENS3_9parameter5void_ES8_S8_S8_S8_EE
pub fn stub_818d7c() -> ! {
    todo!("0x818d7c __ZN3RBX14LibraryService17contentReadyLocalERKSsN5boost10flyweights9flyweightINS_15ProtectedStringENS3_9parameter5void_ES8_S8_S8_S8_EE")
}

#[doc(alias = "RBX::LibraryService::onHeartbeat(void)")]
// 0x819200 — __ZN3RBX14LibraryService11onHeartbeatEv
pub fn stub_819200() -> ! {
    todo!("0x819200 __ZN3RBX14LibraryService11onHeartbeatEv")
}

#[doc(alias = "RBX::LibraryService::issueDelayedLibraryRequest(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
// 0x81932c — __ZN3RBX14LibraryService26issueDelayedLibraryRequestEN5boost10shared_ptrINS0_18LibraryStateObjectEEE
// was: RBX::LibraryService::issueDelayedLibraryRequest(boost::shared_ptr<RBX::LibraryService::LibraryStateObject>)
pub fn stub_81932c() -> ! {
    todo!("0x81932c __ZN3RBX14LibraryService26issueDelayedLibraryRequestEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")
}

#[doc(alias = "RBX::DoIt(boost::function<void ()(void)>)")]
// 0x819570 — __ZN3RBXL4DoItEN5boost8functionIFvvEEE
pub fn stub_819570() -> ! {
    todo!("0x819570 __ZN3RBXL4DoItEN5boost8functionIFvvEEE")
}

#[doc(alias = "RBX::LibraryService::markLibrariesLoaded(void)")]
// 0x819574 — __ZN3RBX14LibraryService19markLibrariesLoadedEv
pub fn stub_819574() -> ! {
    todo!("0x819574 __ZN3RBX14LibraryService19markLibrariesLoadedEv")
}

#[doc(alias = "RBX::LibraryService::loadLocalLibrary(std::string const&)")]
// 0x81972c — __ZN3RBX14LibraryService16loadLocalLibraryERKSs
pub fn stub_81972c() -> ! {
    todo!("0x81972c __ZN3RBX14LibraryService16loadLocalLibraryERKSs")
}

#[doc(alias = "RBX::LibraryService::registerLibrary(std::string const&,std::string const&,bool)")]
// 0x819d48 — __ZN3RBX14LibraryService15registerLibraryERKSsS2_b
pub fn stub_819d48() -> ! {
    todo!("0x819d48 __ZN3RBX14LibraryService15registerLibraryERKSsS2_b")
}

#[doc(alias = "RBX::LibraryService::checkForLoadedLibrary(lua_State *,std::string const&)")]
// 0x81a0a8 — __ZN3RBX14LibraryService21checkForLoadedLibraryEP9lua_StateRKSs
pub fn stub_81a0a8() -> ! {
    todo!("0x81a0a8 __ZN3RBX14LibraryService21checkForLoadedLibraryEP9lua_StateRKSs")
}

#[doc(alias = "RBX::LibraryService::tryRequestLibrary(lua_State *,std::string const&,bool)")]
// 0x81a0f8 — __ZN3RBX14LibraryService17tryRequestLibraryEP9lua_StateRKSsb
pub fn stub_81a0f8() -> ! {
    todo!("0x81a0f8 __ZN3RBX14LibraryService17tryRequestLibraryEP9lua_StateRKSsb")
}

#[doc(alias = "RBX::LibraryService::requestLibrary(lua_State *,std::string const&,bool)")]
// 0x81a8d0 — __ZN3RBX14LibraryService14requestLibraryEP9lua_StateRKSsb
pub fn stub_81a8d0() -> ! {
    todo!("0x81a8d0 __ZN3RBX14LibraryService14requestLibraryEP9lua_StateRKSsb")
}