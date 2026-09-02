//! core shard DV — 100 core stubs EA-sorted, next uncovered after DU 0x84c314 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::GameBasicSettings::ControlMode * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameBasicSettings::ControlMode *,RBX::GameBasicSettings::ControlMode *>(RBX::GameBasicSettings::ControlMode *,RBX::GameBasicSettings::ControlMode *,RBX::GameBasicSettings::ControlMode *)")]
// 0x84c32c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17GameBasicSettings11ControlModeES6_EET0_T_S8_S7_
pub fn stub_84c32c() -> ! {
    todo!("0x84c32c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17GameBasicSettings11ControlModeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::ControlMode*,std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>>,unsigned long,RBX::GameBasicSettings::ControlMode const&)")]
// 0x84c368 — __ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_84c368() -> ! {
    todo!("0x84c368 __ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::GameBasicSettings::~GameBasicSettings()")]
// 0x84f2f4 — __ZN3RBX17GameBasicSettingsD2Ev
pub fn stub_84f2f4() -> ! {
    todo!("0x84f2f4 __ZN3RBX17GameBasicSettingsD2Ev")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,bool>> *)")]
// 0x84f518 — __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_84f518() -> ! {
    todo!("0x84f518 __ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "RBX::RenderHooksService::captureMetrics(void)")]
// 0x84fa9c — __ZN3RBX18RenderHooksService14captureMetricsEv
pub fn stub_84fa9c() -> ! {
    todo!("0x84fa9c __ZN3RBX18RenderHooksService14captureMetricsEv")
}

#[doc(alias = "RBX::RenderHooksService::resizeWindow(int,int)")]
// 0x84fab0 — __ZN3RBX18RenderHooksService12resizeWindowEii
pub fn stub_84fab0() -> ! {
    todo!("0x84fab0 __ZN3RBX18RenderHooksService12resizeWindowEii")
}

#[doc(alias = "RBX::RenderHooksService::enableAdorns(bool)")]
// 0x84fac0 — __ZN3RBX18RenderHooksService12enableAdornsEb
pub fn stub_84fac0() -> ! {
    todo!("0x84fac0 __ZN3RBX18RenderHooksService12enableAdornsEb")
}

#[doc(alias = "RBX::RenderHooksService::printScene(void)")]
// 0x84fad0 — __ZN3RBX18RenderHooksService10printSceneEv
pub fn stub_84fad0() -> ! {
    todo!("0x84fad0 __ZN3RBX18RenderHooksService10printSceneEv")
}

#[doc(alias = "RBX::RenderHooksService::RenderHooksService(void)")]
// 0x84fae0 — __ZN3RBX18RenderHooksServiceC1Ev
pub fn stub_84fae0() -> ! {
    todo!("0x84fae0 __ZN3RBX18RenderHooksServiceC1Ev")
}

#[doc(alias = "RBX::RenderHooksService::RenderHooksService(void)")]
// 0x84fae4 — __ZN3RBX18RenderHooksServiceC2Ev
pub fn stub_84fae4() -> ! {
    todo!("0x84fae4 __ZN3RBX18RenderHooksServiceC2Ev")
}

#[doc(alias = "RBX::RenderHooksService::reloadShaders(void)")]
// 0x84fea4 — __ZN3RBX18RenderHooksService13reloadShadersEv
pub fn stub_84fea4() -> ! {
    todo!("0x84fea4 __ZN3RBX18RenderHooksService13reloadShadersEv")
}

#[doc(alias = "RBX::RenderHooksService::enableQueue(int)")]
// 0x84fed0 — __ZN3RBX18RenderHooksService11enableQueueEi
pub fn stub_84fed0() -> ! {
    todo!("0x84fed0 __ZN3RBX18RenderHooksService11enableQueueEi")
}

#[doc(alias = "RBX::RenderHooksService::disableQueue(int)")]
// 0x84ff18 — __ZN3RBX18RenderHooksService12disableQueueEi
pub fn stub_84ff18() -> ! {
    todo!("0x84ff18 __ZN3RBX18RenderHooksService12disableQueueEi")
}

#[doc(alias = "RBX::RenderHooksService::getPresentTime(void)")]
// 0x84ff68 — __ZN3RBX18RenderHooksService14getPresentTimeEv
pub fn stub_84ff68() -> ! {
    todo!("0x84ff68 __ZN3RBX18RenderHooksService14getPresentTimeEv")
}

#[doc(alias = "RBX::RenderHooksService::getGPUDelay(void)")]
// 0x84ff98 — __ZN3RBX18RenderHooksService11getGPUDelayEv
pub fn stub_84ff98() -> ! {
    todo!("0x84ff98 __ZN3RBX18RenderHooksService11getGPUDelayEv")
}

#[doc(alias = "RBX::RenderHooksService::getRenderAve(void)")]
// 0x84ffa4 — __ZN3RBX18RenderHooksService12getRenderAveEv
pub fn stub_84ffa4() -> ! {
    todo!("0x84ffa4 __ZN3RBX18RenderHooksService12getRenderAveEv")
}

#[doc(alias = "RBX::RenderHooksService::getRenderConfMin(void)")]
// 0x84ffb0 — __ZN3RBX18RenderHooksService16getRenderConfMinEv
pub fn stub_84ffb0() -> ! {
    todo!("0x84ffb0 __ZN3RBX18RenderHooksService16getRenderConfMinEv")
}

#[doc(alias = "RBX::RenderHooksService::getRenderConfMax(void)")]
// 0x84ffbc — __ZN3RBX18RenderHooksService16getRenderConfMaxEv
pub fn stub_84ffbc() -> ! {
    todo!("0x84ffbc __ZN3RBX18RenderHooksService16getRenderConfMaxEv")
}

#[doc(alias = "RBX::RenderHooksService::getRenderStd(void)")]
// 0x84ffc8 — __ZN3RBX18RenderHooksService12getRenderStdEv
pub fn stub_84ffc8() -> ! {
    todo!("0x84ffc8 __ZN3RBX18RenderHooksService12getRenderStdEv")
}

#[doc(alias = "RBX::RenderHooksService::getDeltaAve(void)")]
// 0x84ffd4 — __ZN3RBX18RenderHooksService11getDeltaAveEv
pub fn stub_84ffd4() -> ! {
    todo!("0x84ffd4 __ZN3RBX18RenderHooksService11getDeltaAveEv")
}

#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
// 0x850020 — __ZN3RBX18RenderHooksServiceD1Ev
pub fn stub_850020() -> ! {
    todo!("0x850020 __ZN3RBX18RenderHooksServiceD1Ev")
}

#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
// 0x850024 — __ZN3RBX18RenderHooksServiceD0Ev
pub fn stub_850024() -> ! {
    todo!("0x850024 __ZN3RBX18RenderHooksServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
// 0x8500ec — __ZThn32_N3RBX18RenderHooksServiceD1Ev
pub fn stub_8500ec() -> ! {
    todo!("0x8500ec __ZThn32_N3RBX18RenderHooksServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
// 0x8500f4 — __ZThn32_N3RBX18RenderHooksServiceD0Ev
pub fn stub_8500f4() -> ! {
    todo!("0x8500f4 __ZThn32_N3RBX18RenderHooksServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
// 0x8501c0 — __ZThn36_N3RBX18RenderHooksServiceD1Ev
pub fn stub_8501c0() -> ! {
    todo!("0x8501c0 __ZThn36_N3RBX18RenderHooksServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::RenderHooksService::~RenderHooksService()")]
// 0x8501c8 — __ZThn36_N3RBX18RenderHooksServiceD0Ev
pub fn stub_8501c8() -> ! {
    todo!("0x8501c8 __ZThn36_N3RBX18RenderHooksServiceD0Ev")
}

#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
// 0x85026c — __ZN3RBX18RenderHooksServiceD2Ev
pub fn stub_85026c() -> ! {
    todo!("0x85026c __ZN3RBX18RenderHooksServiceD2Ev")
}

#[doc(alias = "RBX::CookiesService::SetValue(std::string,std::string)")]
// 0x85183c — __ZN3RBX14CookiesService8SetValueESsSs
pub fn stub_85183c() -> ! {
    todo!("0x85183c __ZN3RBX14CookiesService8SetValueESsSs")
}

#[doc(alias = "RBX::CookiesService::GetValue(std::string)")]
// 0x851b3c — __ZN3RBX14CookiesService8GetValueESs
pub fn stub_851b3c() -> ! {
    todo!("0x851b3c __ZN3RBX14CookiesService8GetValueESs")
}

#[doc(alias = "RBX::CookiesService::DeleteValue(std::string)")]
// 0x851e84 — __ZN3RBX14CookiesService11DeleteValueESs
pub fn stub_851e84() -> ! {
    todo!("0x851e84 __ZN3RBX14CookiesService11DeleteValueESs")
}

#[doc(alias = "RBX::CookiesService::CookiesService(void)")]
// 0x8520f8 — __ZN3RBX14CookiesServiceC1Ev
pub fn stub_8520f8() -> ! {
    todo!("0x8520f8 __ZN3RBX14CookiesServiceC1Ev")
}

#[doc(alias = "RBX::CookiesService::CookiesService(void)")]
// 0x8520fc — __ZN3RBX14CookiesServiceC2Ev
pub fn stub_8520fc() -> ! {
    todo!("0x8520fc __ZN3RBX14CookiesServiceC2Ev")
}

#[doc(alias = "RBX::CookiesService::~CookiesService()")]
// 0x852508 — __ZN3RBX14CookiesServiceD1Ev
pub fn stub_852508() -> ! {
    todo!("0x852508 __ZN3RBX14CookiesServiceD1Ev")
}

#[doc(alias = "RBX::CookiesService::~CookiesService()")]
// 0x852544 — __ZN3RBX14CookiesServiceD0Ev
pub fn stub_852544() -> ! {
    todo!("0x852544 __ZN3RBX14CookiesServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CookiesService::~CookiesService()")]
// 0x852640 — __ZThn32_N3RBX14CookiesServiceD1Ev
pub fn stub_852640() -> ! {
    todo!("0x852640 __ZThn32_N3RBX14CookiesServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CookiesService::~CookiesService()")]
// 0x852680 — __ZThn32_N3RBX14CookiesServiceD0Ev
pub fn stub_852680() -> ! {
    todo!("0x852680 __ZThn32_N3RBX14CookiesServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CookiesService::~CookiesService()")]
// 0x85277c — __ZThn36_N3RBX14CookiesServiceD1Ev
pub fn stub_85277c() -> ! {
    todo!("0x85277c __ZThn36_N3RBX14CookiesServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CookiesService::~CookiesService()")]
// 0x8527bc — __ZThn36_N3RBX14CookiesServiceD0Ev
pub fn stub_8527bc() -> ! {
    todo!("0x8527bc __ZThn36_N3RBX14CookiesServiceD0Ev")
}

#[doc(alias = "RBX::ClientAppSettings::Init(void)")]
// 0x853e9c — __ZN3RBX17ClientAppSettings4InitEv
pub fn stub_853e9c() -> ! {
    todo!("0x853e9c __ZN3RBX17ClientAppSettings4InitEv")
}

#[doc(alias = "RBX::ClientAppSettings::Initialize(void)")]
// 0x854a30 — __ZN3RBX17ClientAppSettings10InitializeEv
pub fn stub_854a30() -> ! {
    todo!("0x854a30 __ZN3RBX17ClientAppSettings10InitializeEv")
}

#[doc(alias = "RBX::ClientAppSettings::singleton(void)")]
// 0x854a54 — __ZN3RBX17ClientAppSettings9singletonEv
pub fn stub_854a54() -> ! {
    todo!("0x854a54 __ZN3RBX17ClientAppSettings9singletonEv")
}

#[doc(alias = "RBX::FastLogJSON::ProcessVariable(std::string const&,std::string const&,FastVarType)")]
// 0x854a64 — __ZN3RBX11FastLogJSON15ProcessVariableERKSsS2_11FastVarType
pub fn stub_854a64() -> ! {
    todo!("0x854a64 __ZN3RBX11FastLogJSON15ProcessVariableERKSsS2_11FastVarType")
}

#[doc(alias = "RBX::FastLogJSON::DefaultHandler(std::string const&,std::string const&)")]
// 0x854a70 — __ZN3RBX11FastLogJSON14DefaultHandlerERKSsS2_
pub fn stub_854a70() -> ! {
    todo!("0x854a70 __ZN3RBX11FastLogJSON14DefaultHandlerERKSsS2_")
}

#[doc(alias = "FLog::FastLogSettingsItem::setVariable(std::string,std::string)")]
// 0x855a28 — __ZN4FLog19FastLogSettingsItem11setVariableESsSs
pub fn stub_855a28() -> ! {
    todo!("0x855a28 __ZN4FLog19FastLogSettingsItem11setVariableESsSs")
}

#[doc(alias = "FLog::FastLogSettingsItem::print(std::string)")]
// 0x855a7c — __ZN4FLog19FastLogSettingsItem5printESs
pub fn stub_855a7c() -> ! {
    todo!("0x855a7c __ZN4FLog19FastLogSettingsItem5printESs")
}

#[doc(alias = "FLog::FastLogSettingsItem::dumpLogs(std::string)")]
// 0x855ae0 — __ZN4FLog19FastLogSettingsItem8dumpLogsESs
pub fn stub_855ae0() -> ! {
    todo!("0x855ae0 __ZN4FLog19FastLogSettingsItem8dumpLogsESs")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueAllowVideoPreRoll(char const*)")]
// 0x855aec — __ZN3RBX17ClientAppSettings26ReadValueAllowVideoPreRollEPKc
pub fn stub_855aec() -> ! {
    todo!("0x855aec __ZN3RBX17ClientAppSettings26ReadValueAllowVideoPreRollEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueStartPageUrl(char const*)")]
// 0x855b04 — __ZN3RBX17ClientAppSettings21ReadValueStartPageUrlEPKc
pub fn stub_855b04() -> ! {
    todo!("0x855b04 __ZN3RBX17ClientAppSettings21ReadValueStartPageUrlEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueWebDocAddressBarEnabled(char const*)")]
// 0x855c3c — __ZN3RBX17ClientAppSettings32ReadValueWebDocAddressBarEnabledEPKc
pub fn stub_855c3c() -> ! {
    todo!("0x855c3c __ZN3RBX17ClientAppSettings32ReadValueWebDocAddressBarEnabledEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueCaptureQTStudioCountersEnabled(char const*)")]
// 0x855c58 — __ZN3RBX17ClientAppSettings39ReadValueCaptureQTStudioCountersEnabledEPKc
pub fn stub_855c58() -> ! {
    todo!("0x855c58 __ZN3RBX17ClientAppSettings39ReadValueCaptureQTStudioCountersEnabledEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueCaptureMFCStudioCountersEnabled(char const*)")]
// 0x855c70 — __ZN3RBX17ClientAppSettings40ReadValueCaptureMFCStudioCountersEnabledEPKc
pub fn stub_855c70() -> ! {
    todo!("0x855c70 __ZN3RBX17ClientAppSettings40ReadValueCaptureMFCStudioCountersEnabledEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueCaptureCountersIntervalInMinutes(char const*)")]
// 0x855c88 — __ZN3RBX17ClientAppSettings41ReadValueCaptureCountersIntervalInMinutesEPKc
pub fn stub_855c88() -> ! {
    todo!("0x855c88 __ZN3RBX17ClientAppSettings41ReadValueCaptureCountersIntervalInMinutesEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueCaptureSlowCountersIntervalInSeconds(char const*)")]
// 0x855ca0 — __ZN3RBX17ClientAppSettings45ReadValueCaptureSlowCountersIntervalInSecondsEPKc
pub fn stub_855ca0() -> ! {
    todo!("0x855ca0 __ZN3RBX17ClientAppSettings45ReadValueCaptureSlowCountersIntervalInSecondsEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValuePublishedProjectsPageUrl(char const*)")]
// 0x855cb8 — __ZN3RBX17ClientAppSettings33ReadValuePublishedProjectsPageUrlEPKc
pub fn stub_855cb8() -> ! {
    todo!("0x855cb8 __ZN3RBX17ClientAppSettings33ReadValuePublishedProjectsPageUrlEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValuePublishedProjectsPageWidth(char const*)")]
// 0x855df0 — __ZN3RBX17ClientAppSettings35ReadValuePublishedProjectsPageWidthEPKc
pub fn stub_855df0() -> ! {
    todo!("0x855df0 __ZN3RBX17ClientAppSettings35ReadValuePublishedProjectsPageWidthEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValuePublishedProjectsPageHeight(char const*)")]
// 0x855e08 — __ZN3RBX17ClientAppSettings36ReadValuePublishedProjectsPageHeightEPKc
pub fn stub_855e08() -> ! {
    todo!("0x855e08 __ZN3RBX17ClientAppSettings36ReadValuePublishedProjectsPageHeightEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueAxisAdornmentGrabSize(char const*)")]
// 0x855e20 — __ZN3RBX17ClientAppSettings30ReadValueAxisAdornmentGrabSizeEPKc
pub fn stub_855e20() -> ! {
    todo!("0x855e20 __ZN3RBX17ClientAppSettings30ReadValueAxisAdornmentGrabSizeEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValuePrizeAwarderURL(char const*)")]
// 0x855e38 — __ZN3RBX17ClientAppSettings24ReadValuePrizeAwarderURLEPKc
pub fn stub_855e38() -> ! {
    todo!("0x855e38 __ZN3RBX17ClientAppSettings24ReadValuePrizeAwarderURLEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValuePrizeAssetIDs(char const*)")]
// 0x855f70 — __ZN3RBX17ClientAppSettings22ReadValuePrizeAssetIDsEPKc
pub fn stub_855f70() -> ! {
    todo!("0x855f70 __ZN3RBX17ClientAppSettings22ReadValuePrizeAssetIDsEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueMinPartsForOptDragging(char const*)")]
// 0x8560c0 — __ZN3RBX17ClientAppSettings31ReadValueMinPartsForOptDraggingEPKc
pub fn stub_8560c0() -> ! {
    todo!("0x8560c0 __ZN3RBX17ClientAppSettings31ReadValueMinPartsForOptDraggingEPKc")
}

#[doc(alias = "RBX::ClientAppSettings::~ClientAppSettings()")]
// 0x8560d8 — __ZN3RBX17ClientAppSettingsD1Ev
pub fn stub_8560d8() -> ! {
    todo!("0x8560d8 __ZN3RBX17ClientAppSettingsD1Ev")
}

#[doc(alias = "RBX::ClientAppSettings::~ClientAppSettings()")]
// 0x8584a4 — __ZN3RBX17ClientAppSettingsD2Ev
pub fn stub_8584a4() -> ! {
    todo!("0x8584a4 __ZN3RBX17ClientAppSettingsD2Ev")
}

#[doc(alias = "RBX::ClientAppSettings::~ClientAppSettings()")]
// 0x8584f8 — __ZN3RBX17ClientAppSettingsD0Ev
pub fn stub_8584f8() -> ! {
    todo!("0x8584f8 __ZN3RBX17ClientAppSettingsD0Ev")
}

#[doc(alias = "RBX::ClientAppSettings::ClientAppSettings(void)")]
// 0x858598 — __ZN3RBX17ClientAppSettingsC2Ev
pub fn stub_858598() -> ! {
    todo!("0x858598 __ZN3RBX17ClientAppSettingsC2Ev")
}

#[doc(alias = "RBX::CustomEvent::getPersistedCurrentValue(void)const")]
// 0x85a64c — __ZNK3RBX11CustomEvent24getPersistedCurrentValueEv
pub fn stub_85a64c() -> ! {
    todo!("0x85a64c __ZNK3RBX11CustomEvent24getPersistedCurrentValueEv")
}

#[doc(alias = "RBX::CustomEvent::setPersistedCurrentValue(float)")]
// 0x85a650 — __ZN3RBX11CustomEvent24setPersistedCurrentValueEf
pub fn stub_85a650() -> ! {
    todo!("0x85a650 __ZN3RBX11CustomEvent24setPersistedCurrentValueEf")
}

#[doc(alias = "RBX::CustomEvent::setCurrentValue(float)")]
// 0x85a698 — __ZN3RBX11CustomEvent15setCurrentValueEf
pub fn stub_85a698() -> ! {
    todo!("0x85a698 __ZN3RBX11CustomEvent15setCurrentValueEf")
}

#[doc(alias = "RBX::CustomEvent::getAttachedReceivers(void)")]
// 0x85a80c — __ZN3RBX11CustomEvent20getAttachedReceiversEv
pub fn stub_85a80c() -> ! {
    todo!("0x85a80c __ZN3RBX11CustomEvent20getAttachedReceiversEv")
}

#[doc(alias = "RBX::CustomEventReceiver::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x85bcc8 — __ZN3RBX19CustomEventReceiver17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_85bcc8() -> ! {
    todo!("0x85bcc8 __ZN3RBX19CustomEventReceiver17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::CustomEventReceiver::getSource(void)const")]
// 0x85be6c — __ZNK3RBX19CustomEventReceiver9getSourceEv
pub fn stub_85be6c() -> ! {
    todo!("0x85be6c __ZNK3RBX19CustomEventReceiver9getSourceEv")
}

#[doc(alias = "RBX::CustomEventReceiver::getCurrentValue(void)")]
// 0x85bee0 — __ZN3RBX19CustomEventReceiver15getCurrentValueEv
pub fn stub_85bee0() -> ! {
    todo!("0x85bee0 __ZN3RBX19CustomEventReceiver15getCurrentValueEv")
}

#[doc(alias = "RBX::CustomEvent::removeReceiver(RBX::CustomEventReceiver *)")]
// 0x85bf2c — __ZN3RBX11CustomEvent14removeReceiverEPNS_19CustomEventReceiverE
pub fn stub_85bf2c() -> ! {
    todo!("0x85bf2c __ZN3RBX11CustomEvent14removeReceiverEPNS_19CustomEventReceiverE")
}

#[doc(alias = "RBX::CustomEvent::addReceiver(RBX::CustomEventReceiver *)")]
// 0x85c088 — __ZN3RBX11CustomEvent11addReceiverEPNS_19CustomEventReceiverE
pub fn stub_85c088() -> ! {
    todo!("0x85c088 __ZN3RBX11CustomEvent11addReceiverEPNS_19CustomEventReceiverE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEventReceiver> RBX::shared_from<RBX::CustomEventReceiver>(RBX::CustomEventReceiver*)")]
// 0x85c248 — __ZN3RBX11shared_fromINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::CustomEventReceiver> RBX::shared_from<RBX::CustomEventReceiver>(RBX::CustomEventReceiver*)
pub fn stub_85c248() -> ! {
    todo!("0x85c248 __ZN3RBX11shared_fromINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0x85c3b8 — __ZN3RBX19CustomEventReceiverD1Ev
pub fn stub_85c3b8() -> ! {
    todo!("0x85c3b8 __ZN3RBX19CustomEventReceiverD1Ev")
}

#[doc(alias = "RBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0x85c3bc — __ZN3RBX19CustomEventReceiverD0Ev
pub fn stub_85c3bc() -> ! {
    todo!("0x85c3bc __ZN3RBX19CustomEventReceiverD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0x85c45c — __ZThn32_N3RBX19CustomEventReceiverD1Ev
pub fn stub_85c45c() -> ! {
    todo!("0x85c45c __ZThn32_N3RBX19CustomEventReceiverD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0x85c464 — __ZThn32_N3RBX19CustomEventReceiverD0Ev
pub fn stub_85c464() -> ! {
    todo!("0x85c464 __ZThn32_N3RBX19CustomEventReceiverD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0x85c508 — __ZThn36_N3RBX19CustomEventReceiverD1Ev
pub fn stub_85c508() -> ! {
    todo!("0x85c508 __ZThn36_N3RBX19CustomEventReceiverD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0x85c510 — __ZThn36_N3RBX19CustomEventReceiverD0Ev
pub fn stub_85c510() -> ! {
    todo!("0x85c510 __ZThn36_N3RBX19CustomEventReceiverD0Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEvent>::shared_ptr<RBX::CustomEvent>(rbx_core::WeakPtr<RBX::CustomEvent> const&,boost::detail::sp_nothrow_tag)")]
// 0x85d6e8 — __ZN5boost10shared_ptrIN3RBX11CustomEventEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::CustomEvent>::shared_ptr<RBX::CustomEvent>(boost::weak_ptr<RBX::CustomEvent> const&,boost::detail::sp_nothrow_tag)
pub fn stub_85d6e8() -> ! {
    todo!("0x85d6e8 __ZN5boost10shared_ptrIN3RBX11CustomEventEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "RBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0x85d764 — __ZN3RBX19CustomEventReceiverD2Ev
pub fn stub_85d764() -> ! {
    todo!("0x85d764 __ZN3RBX19CustomEventReceiverD2Ev")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::CustomEventReceiver>::weak_ptr<RBX::CustomEventReceiver>(rbx_core::SharedPtr<RBX::CustomEventReceiver> const&,boost::detail::sp_enable_if_convertible<RBX::CustomEventReceiver,RBX::CustomEventReceiver>::type)")]
// 0x85da1c — __ZN5boost8weak_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::CustomEventReceiver>::weak_ptr<RBX::CustomEventReceiver>(boost::shared_ptr<RBX::CustomEventReceiver> const&,boost::detail::sp_enable_if_convertible<RBX::CustomEventReceiver,RBX::CustomEventReceiver>::type)
pub fn stub_85da1c() -> ! {
    todo!("0x85da1c __ZN5boost8weak_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_erase(std::_List_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>)")]
// 0x85da6c — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// was: std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_erase(std::_List_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>)
pub fn stub_85da6c() -> ! {
    todo!("0x85da6c __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")
}

#[doc(alias = "RBX::TextureTrail::getFrom(void)const")]
// 0x85ded8 — __ZNK3RBX12TextureTrail7getFromEv
pub fn stub_85ded8() -> ! {
    todo!("0x85ded8 __ZNK3RBX12TextureTrail7getFromEv")
}

#[doc(alias = "RBX::TextureTrail::getTo(void)const")]
// 0x85df10 — __ZNK3RBX12TextureTrail5getToEv
pub fn stub_85df10() -> ! {
    todo!("0x85df10 __ZNK3RBX12TextureTrail5getToEv")
}

#[doc(alias = "RBX::TextureTrail::getTexture(void)const")]
// 0x85df48 — __ZNK3RBX12TextureTrail10getTextureEv
pub fn stub_85df48() -> ! {
    todo!("0x85df48 __ZNK3RBX12TextureTrail10getTextureEv")
}

#[doc(alias = "RBX::TextureTrail::setTexture(RBX::TextureId)")]
// 0x85df60 — __ZN3RBX12TextureTrail10setTextureENS_9TextureIdE
pub fn stub_85df60() -> ! {
    todo!("0x85df60 __ZN3RBX12TextureTrail10setTextureENS_9TextureIdE")
}

#[doc(alias = "RBX::TextureTrail::getTextureSize(void)const")]
// 0x85df78 — __ZNK3RBX12TextureTrail14getTextureSizeEv
pub fn stub_85df78() -> ! {
    todo!("0x85df78 __ZNK3RBX12TextureTrail14getTextureSizeEv")
}

#[doc(alias = "RBX::TextureTrail::getVelocity(void)const")]
// 0x85df98 — __ZNK3RBX12TextureTrail11getVelocityEv
pub fn stub_85df98() -> ! {
    todo!("0x85df98 __ZNK3RBX12TextureTrail11getVelocityEv")
}

#[doc(alias = "RBX::TextureTrail::setVelocity(float)")]
// 0x85dfa0 — __ZN3RBX12TextureTrail11setVelocityEf
pub fn stub_85dfa0() -> ! {
    todo!("0x85dfa0 __ZN3RBX12TextureTrail11setVelocityEf")
}

#[doc(alias = "RBX::TextureTrail::getStudsBetweenTextures(void)const")]
// 0x85dfa8 — __ZNK3RBX12TextureTrail23getStudsBetweenTexturesEv
pub fn stub_85dfa8() -> ! {
    todo!("0x85dfa8 __ZNK3RBX12TextureTrail23getStudsBetweenTexturesEv")
}

#[doc(alias = "RBX::TextureTrail::setStudsBetweenTextures(float)")]
// 0x85dfb0 — __ZN3RBX12TextureTrail23setStudsBetweenTexturesEf
pub fn stub_85dfb0() -> ! {
    todo!("0x85dfb0 __ZN3RBX12TextureTrail23setStudsBetweenTexturesEf")
}

#[doc(alias = "RBX::TextureTrail::getCycleOffset(void)const")]
// 0x85dfb8 — __ZNK3RBX12TextureTrail14getCycleOffsetEv
pub fn stub_85dfb8() -> ! {
    todo!("0x85dfb8 __ZNK3RBX12TextureTrail14getCycleOffsetEv")
}

#[doc(alias = "RBX::TextureTrail::setCycleOffset(float)")]
// 0x85dfc0 — __ZN3RBX12TextureTrail14setCycleOffsetEf
pub fn stub_85dfc0() -> ! {
    todo!("0x85dfc0 __ZN3RBX12TextureTrail14setCycleOffsetEf")
}

#[doc(alias = "RBX::TextureTrail::TextureTrail(void)")]
// 0x85dfc8 — __ZN3RBX12TextureTrailC2Ev
pub fn stub_85dfc8() -> ! {
    todo!("0x85dfc8 __ZN3RBX12TextureTrailC2Ev")
}

#[doc(alias = "RBX::TextureTrail::render3dAdorn(RBX::Adorn *)")]
// 0x85e2bc — __ZN3RBX12TextureTrail13render3dAdornEPNS_5AdornE
pub fn stub_85e2bc() -> ! {
    todo!("0x85e2bc __ZN3RBX12TextureTrail13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::TextureTrail::render3dAdorn(RBX::Adorn *)")]
// 0x85e7f0 — __ZThn96_N3RBX12TextureTrail13render3dAdornEPNS_5AdornE
pub fn stub_85e7f0() -> ! {
    todo!("0x85e7f0 __ZThn96_N3RBX12TextureTrail13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::TextureTrail::~TextureTrail()")]
// 0x85e890 — __ZN3RBX12TextureTrailD1Ev
pub fn stub_85e890() -> ! {
    todo!("0x85e890 __ZN3RBX12TextureTrailD1Ev")
}

#[doc(alias = "RBX::TextureTrail::~TextureTrail()")]
// 0x85e9bc — __ZN3RBX12TextureTrailD0Ev
pub fn stub_85e9bc() -> ! {
    todo!("0x85e9bc __ZN3RBX12TextureTrailD0Ev")
}
