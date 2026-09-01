//! core shard BO — 100 core stubs EA-sorted, next uncovered after BN 0x4fa22c (strict RBX|boost|std|rbx earliest gap, after BN 0x4efba0..0x4fa22c).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x4fa22c.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,unsigned long,RBX::Frame::Style const&)")]
// 0x4fa268 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,unsigned long,RBX::Frame::Style const&)
pub fn stub_4fa268() -> ! {
    todo!("0x4fa268 __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::SecurePlayerGame::SecurePlayerGame(RBX::Verb *,char const*,bool)")]
// 0x4faee8 — __ZN3RBX16SecurePlayerGameC1EPNS_4VerbEPKcb — RBX::SecurePlayerGame::SecurePlayerGame(RBX::Verb *,char const*,bool)
pub fn stub_4faee8() -> ! {
    todo!("0x4faee8 __ZN3RBX16SecurePlayerGameC1EPNS_4VerbEPKcb")
}

#[doc(alias = "RBX::SecurePlayerGame::SecurePlayerGame(RBX::Verb *,char const*,bool)")]
// 0x4faeec — __ZN3RBX16SecurePlayerGameC2EPNS_4VerbEPKcb — RBX::SecurePlayerGame::SecurePlayerGame(RBX::Verb *,char const*,bool)
pub fn stub_4faeec() -> ! {
    todo!("0x4faeec __ZN3RBX16SecurePlayerGameC2EPNS_4VerbEPKcb")
}

#[doc(alias = "RBX::Game::Game(RBX::Verb *,char const*,bool)")]
// 0x4fafc4 — __ZN3RBX4GameC2EPNS_4VerbEPKcb — RBX::Game::Game(RBX::Verb *,char const*,bool)
pub fn stub_4fafc4() -> ! {
    todo!("0x4fafc4 __ZN3RBX4GameC2EPNS_4VerbEPKcb")
}

#[doc(alias = "RBX::Game::~Game()")]
// 0x4fb85c — __ZN3RBX4GameD2Ev — RBX::Game::~Game()
pub fn stub_4fb85c() -> ! {
    todo!("0x4fb85c __ZN3RBX4GameD2Ev")
}

#[doc(alias = "RBX::UnsecuredStudioGame::UnsecuredStudioGame(RBX::Verb *,char const*,bool)")]
// 0x4fba28 — __ZN3RBX19UnsecuredStudioGameC1EPNS_4VerbEPKcb — RBX::UnsecuredStudioGame::UnsecuredStudioGame(RBX::Verb *,char const*,bool)
pub fn stub_4fba28() -> ! {
    todo!("0x4fba28 __ZN3RBX19UnsecuredStudioGameC1EPNS_4VerbEPKcb")
}

#[doc(alias = "RBX::UnsecuredStudioGame::UnsecuredStudioGame(RBX::Verb *,char const*,bool)")]
// 0x4fba2c — __ZN3RBX19UnsecuredStudioGameC2EPNS_4VerbEPKcb — RBX::UnsecuredStudioGame::UnsecuredStudioGame(RBX::Verb *,char const*,bool)
pub fn stub_4fba2c() -> ! {
    todo!("0x4fba2c __ZN3RBX19UnsecuredStudioGameC2EPNS_4VerbEPKcb")
}

#[doc(alias = "RBX::Game::globalInit(void)")]
// 0x4fbb04 — __ZN3RBX4Game10globalInitEv — RBX::Game::globalInit(void)
pub fn stub_4fbb04() -> ! {
    todo!("0x4fbb04 __ZN3RBX4Game10globalInitEv")
}

#[doc(alias = "RBX::Game::~Game()")]
// 0x4fc348 — __ZN3RBX4GameD0Ev — RBX::Game::~Game()
pub fn stub_4fc348() -> ! {
    todo!("0x4fc348 __ZN3RBX4GameD0Ev")
}

#[doc(alias = "RBX::Game::~Game()")]
// 0x4fc3e8 — __ZN3RBX4GameD1Ev — RBX::Game::~Game()
pub fn stub_4fc3e8() -> ! {
    todo!("0x4fc3e8 __ZN3RBX4GameD1Ev")
}

#[doc(alias = "RBX::Game::shutdown(void)")]
// 0x4fc3ec — __ZN3RBX4Game8shutdownEv — RBX::Game::shutdown(void)
pub fn stub_4fc3ec() -> ! {
    todo!("0x4fc3ec __ZN3RBX4Game8shutdownEv")
}

#[doc(alias = "RBX::Game::doClearVerbs(void)")]
// 0x4fc420 — __ZN3RBX4Game12doClearVerbsEv — RBX::Game::doClearVerbs(void)
pub fn stub_4fc420() -> ! {
    todo!("0x4fc420 __ZN3RBX4Game12doClearVerbsEv")
}

#[doc(alias = "RBX::Game::clearVerbs(bool)")]
// 0x4fc548 — __ZN3RBX4Game10clearVerbsEb — RBX::Game::clearVerbs(bool)
pub fn stub_4fc548() -> ! {
    todo!("0x4fc548 __ZN3RBX4Game10clearVerbsEb")
}

#[doc(alias = "RBX::Game::getSuppressNavKeys(void)")]
// 0x4fc750 — __ZN3RBX4Game18getSuppressNavKeysEv — RBX::Game::getSuppressNavKeys(void)
pub fn stub_4fc750() -> ! {
    todo!("0x4fc750 __ZN3RBX4Game18getSuppressNavKeysEv")
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::push_back(RBX::Verb * const&)")]
// 0x4fcd30 — __ZNSt6vectorIPN3RBX4VerbESaIS2_EE9push_backERKS2_ — std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::push_back(RBX::Verb * const&)
pub fn stub_4fcd30() -> ! {
    todo!("0x4fcd30 __ZNSt6vectorIPN3RBX4VerbESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "RBX::SecurePlayerGame::~SecurePlayerGame()")]
// 0x4fd304 — __ZN3RBX16SecurePlayerGameD1Ev — RBX::SecurePlayerGame::~SecurePlayerGame()
pub fn stub_4fd304() -> ! {
    todo!("0x4fd304 __ZN3RBX16SecurePlayerGameD1Ev")
}

#[doc(alias = "RBX::SecurePlayerGame::~SecurePlayerGame()")]
// 0x4fd308 — __ZN3RBX16SecurePlayerGameD0Ev — RBX::SecurePlayerGame::~SecurePlayerGame()
pub fn stub_4fd308() -> ! {
    todo!("0x4fd308 __ZN3RBX16SecurePlayerGameD0Ev")
}

#[doc(alias = "RBX::UnsecuredStudioGame::~UnsecuredStudioGame()")]
// 0x4fd3a8 — __ZN3RBX19UnsecuredStudioGameD1Ev — RBX::UnsecuredStudioGame::~UnsecuredStudioGame()
pub fn stub_4fd3a8() -> ! {
    todo!("0x4fd3a8 __ZN3RBX19UnsecuredStudioGameD1Ev")
}

#[doc(alias = "RBX::UnsecuredStudioGame::~UnsecuredStudioGame()")]
// 0x4fd3ac — __ZN3RBX19UnsecuredStudioGameD0Ev — RBX::UnsecuredStudioGame::~UnsecuredStudioGame()
pub fn stub_4fd3ac() -> ! {
    todo!("0x4fd3ac __ZN3RBX19UnsecuredStudioGameD0Ev")
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Verb **,std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>>,RBX::Verb * const&)")]
// 0x4fdf80 — __ZNSt6vectorIPN3RBX4VerbESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Verb **,std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>>,RBX::Verb * const&)
pub fn stub_4fdf80() -> ! {
    todo!("0x4fdf80 __ZNSt6vectorIPN3RBX4VerbESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_allocate(unsigned long)")]
// 0x4fe060 — __ZNSt12_Vector_baseIPN3RBX4VerbESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_allocate(unsigned long)
pub fn stub_4fe060() -> ! {
    todo!("0x4fe060 __ZNSt12_Vector_baseIPN3RBX4VerbESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::CommonVerbs::~CommonVerbs()")]
// 0x4fe258 — __ZN3RBX11CommonVerbsD2Ev — RBX::CommonVerbs::~CommonVerbs()
pub fn stub_4fe258() -> ! {
    todo!("0x4fe258 __ZN3RBX11CommonVerbsD2Ev")
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_s_instance(void)")]
// 0x4ff698 — __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE27safe_static_init_s_instanceEv — RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_s_instance(void)
pub fn stub_4ff698() -> ! {
    todo!("0x4ff698 __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE27safe_static_init_s_instanceEv")
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_s_instance(void)")]
// 0x4ff69c — __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE29safe_static_do_get_s_instanceEv — RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_s_instance(void)
pub fn stub_4ff69c() -> ! {
    todo!("0x4ff69c __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE29safe_static_do_get_s_instanceEv")
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_sync(void)")]
// 0x4ff714 — __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE21safe_static_init_syncEv — RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_init_sync(void)
pub fn stub_4ff714() -> ! {
    todo!("0x4ff714 __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE21safe_static_init_syncEv")
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_sync(void)")]
// 0x4ff718 — __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE23safe_static_do_get_syncEv — RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_sync(void)
pub fn stub_4ff718() -> ! {
    todo!("0x4ff718 __ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE23safe_static_do_get_syncEv")
}

#[doc(alias = "RBX::mutex::~mutex()")]
// 0x4ff808 — __ZN3RBX5mutexD1Ev — RBX::mutex::~mutex()
pub fn stub_4ff808() -> ! {
    todo!("0x4ff808 __ZN3RBX5mutexD1Ev")
}

#[doc(alias = "RBX::NullVerb::~NullVerb()")]
// 0x4ff9dc — __ZN3RBX8NullVerbD1Ev — RBX::NullVerb::~NullVerb()
pub fn stub_4ff9dc() -> ! {
    todo!("0x4ff9dc __ZN3RBX8NullVerbD1Ev")
}

#[doc(alias = "RBX::NullVerb::~NullVerb()")]
// 0x4ff9e0 — __ZN3RBX8NullVerbD0Ev — RBX::NullVerb::~NullVerb()
pub fn stub_4ff9e0() -> ! {
    todo!("0x4ff9e0 __ZN3RBX8NullVerbD0Ev")
}

#[doc(alias = "RBX::NullVerb::isEnabled(void)const")]
// 0x4ffa80 — __ZNK3RBX8NullVerb9isEnabledEv — RBX::NullVerb::isEnabled(void)const
pub fn stub_4ffa80() -> ! {
    todo!("0x4ffa80 __ZNK3RBX8NullVerb9isEnabledEv")
}

#[doc(alias = "RBX::Verb::isChecked(void)const")]
// 0x4ffa84 — __ZNK3RBX4Verb9isCheckedEv — RBX::Verb::isChecked(void)const
pub fn stub_4ffa84() -> ! {
    todo!("0x4ffa84 __ZNK3RBX4Verb9isCheckedEv")
}

#[doc(alias = "RBX::Verb::isSelected(void)const")]
// 0x4ffa88 — __ZNK3RBX4Verb10isSelectedEv — RBX::Verb::isSelected(void)const
pub fn stub_4ffa88() -> ! {
    todo!("0x4ffa88 __ZNK3RBX4Verb10isSelectedEv")
}

#[doc(alias = "RBX::Verb::getText(void)const")]
// 0x4ffa8c — __ZNK3RBX4Verb7getTextEv — RBX::Verb::getText(void)const
pub fn stub_4ffa8c() -> ! {
    todo!("0x4ffa8c __ZNK3RBX4Verb7getTextEv")
}

#[doc(alias = "RBX::NullVerb::doIt(RBX::IDataState *)")]
// 0x4ffaa0 — __ZN3RBX8NullVerb4doItEPNS_10IDataStateE — RBX::NullVerb::doIt(RBX::IDataState *)
pub fn stub_4ffaa0() -> ! {
    todo!("0x4ffaa0 __ZN3RBX8NullVerb4doItEPNS_10IDataStateE")
}

#[doc(alias = "RBX::GameSettings::setVideoQualitySetting(RBX::GameSettings::VideoQuality)")]
// 0x500bcc — __ZN3RBX12GameSettings22setVideoQualitySettingENS0_12VideoQualityE — RBX::GameSettings::setVideoQualitySetting(RBX::GameSettings::VideoQuality)
pub fn stub_500bcc() -> ! {
    todo!("0x500bcc __ZN3RBX12GameSettings22setVideoQualitySettingENS0_12VideoQualityE")
}

#[doc(alias = "RBX::GameSettings::setPostImageSetting(RBX::GameSettings::UploadSetting)")]
// 0x500bec — __ZN3RBX12GameSettings19setPostImageSettingENS0_13UploadSettingE — RBX::GameSettings::setPostImageSetting(RBX::GameSettings::UploadSetting)
pub fn stub_500bec() -> ! {
    todo!("0x500bec __ZN3RBX12GameSettings19setPostImageSettingENS0_13UploadSettingE")
}

#[doc(alias = "RBX::GameSettings::GameSettings(void)")]
// 0x500c0c — __ZN3RBX12GameSettingsC1Ev — RBX::GameSettings::GameSettings(void)
pub fn stub_500c0c() -> ! {
    todo!("0x500c0c __ZN3RBX12GameSettingsC1Ev")
}

#[doc(alias = "RBX::GameSettings::GameSettings(void)")]
// 0x500c10 — __ZN3RBX12GameSettingsC2Ev — RBX::GameSettings::GameSettings(void)
pub fn stub_500c10() -> ! {
    todo!("0x500c10 __ZN3RBX12GameSettingsC2Ev")
}

#[doc(alias = "RBX::GameSettings::getVideoQualitySetting(void)const")]
// 0x50158c — __ZNK3RBX12GameSettings22getVideoQualitySettingEv — RBX::GameSettings::getVideoQualitySetting(void)const
pub fn stub_50158c() -> ! {
    todo!("0x50158c __ZNK3RBX12GameSettings22getVideoQualitySettingEv")
}

#[doc(alias = "RBX::GameSettings::getPostImageSetting(void)const")]
// 0x5015b8 — __ZNK3RBX12GameSettings19getPostImageSettingEv — RBX::GameSettings::getPostImageSetting(void)const
pub fn stub_5015b8() -> ! {
    todo!("0x5015b8 __ZNK3RBX12GameSettings19getPostImageSettingEv")
}

#[doc(alias = "RBX::GameSettings::~GameSettings()")]
// 0x501878 — __ZN3RBX12GameSettingsD1Ev — RBX::GameSettings::~GameSettings()
pub fn stub_501878() -> ! {
    todo!("0x501878 __ZN3RBX12GameSettingsD1Ev")
}

#[doc(alias = "RBX::GameSettings::~GameSettings()")]
// 0x501a00 — __ZN3RBX12GameSettingsD0Ev — RBX::GameSettings::~GameSettings()
pub fn stub_501a00() -> ! {
    todo!("0x501a00 __ZN3RBX12GameSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings()")]
// 0x501ab0 — __ZThn32_N3RBX12GameSettingsD1Ev — non-virtual thunk toRBX::GameSettings::~GameSettings()
pub fn stub_501ab0() -> ! {
    todo!("0x501ab0 __ZThn32_N3RBX12GameSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings()")]
// 0x501c34 — __ZThn32_N3RBX12GameSettingsD0Ev — non-virtual thunk toRBX::GameSettings::~GameSettings()
pub fn stub_501c34() -> ! {
    todo!("0x501c34 __ZThn32_N3RBX12GameSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings()")]
// 0x501de0 — __ZThn36_N3RBX12GameSettingsD1Ev — non-virtual thunk toRBX::GameSettings::~GameSettings()
pub fn stub_501de0() -> ! {
    todo!("0x501de0 __ZThn36_N3RBX12GameSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GameSettings::~GameSettings()")]
// 0x501f64 — __ZThn36_N3RBX12GameSettingsD0Ev — non-virtual thunk toRBX::GameSettings::~GameSettings()
pub fn stub_501f64() -> ! {
    todo!("0x501f64 __ZThn36_N3RBX12GameSettingsD0Ev")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameSettings::UploadSetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::operator[](RBX::Name const* const&)")]
// 0x50402c — __ZNSt3mapIPKN3RBX4NameENS0_12GameSettings13UploadSettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GameSettings::UploadSetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::operator[](RBX::Name const* const&)
pub fn stub_50402c() -> ! {
    todo!("0x50402c __ZNSt3mapIPKN3RBX4NameENS0_12GameSettings13UploadSettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
// 0x504084 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)
pub fn stub_504084() -> ! {
    todo!("0x504084 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
// 0x504138 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)
pub fn stub_504138() -> ! {
    todo!("0x504138 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
// 0x504190 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)
pub fn stub_504190() -> ! {
    todo!("0x504190 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::resize(unsigned long,RBX::GameSettings::UploadSetting)")]
// 0x5041f8 — __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE6resizeEmS2_ — std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::resize(unsigned long,RBX::GameSettings::UploadSetting)
pub fn stub_5041f8() -> ! {
    todo!("0x5041f8 __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::push_back(RBX::GameSettings::UploadSetting const&)")]
// 0x50422c — __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE9push_backERKS2_ — std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::push_back(RBX::GameSettings::UploadSetting const&)
pub fn stub_50422c() -> ! {
    todo!("0x50422c __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,RBX::GameSettings::UploadSetting const&)")]
// 0x504254 — __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,RBX::GameSettings::UploadSetting const&)
pub fn stub_504254() -> ! {
    todo!("0x504254 __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_allocate(unsigned long)")]
// 0x504338 — __ZNSt12_Vector_baseIN3RBX12GameSettings13UploadSettingESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_allocate(unsigned long)
pub fn stub_504338() -> ! {
    todo!("0x504338 __ZNSt12_Vector_baseIN3RBX12GameSettings13UploadSettingESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::GameSettings::UploadSetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *>(RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *)")]
// 0x504350 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12GameSettings13UploadSettingES6_EET0_T_S8_S7_ — RBX::GameSettings::UploadSetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *>(RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *)
pub fn stub_504350() -> ! {
    todo!("0x504350 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12GameSettings13UploadSettingES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,unsigned long,RBX::GameSettings::UploadSetting const&)")]
// 0x50438c — __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,unsigned long,RBX::GameSettings::UploadSetting const&)
pub fn stub_50438c() -> ! {
    todo!("0x50438c __ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameSettings::VideoQuality,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::operator[](RBX::Name const* const&)")]
// 0x50451c — __ZNSt3mapIPKN3RBX4NameENS0_12GameSettings12VideoQualityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GameSettings::VideoQuality,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::operator[](RBX::Name const* const&)
pub fn stub_50451c() -> ! {
    todo!("0x50451c __ZNSt3mapIPKN3RBX4NameENS0_12GameSettings12VideoQualityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
// 0x504574 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)
pub fn stub_504574() -> ! {
    todo!("0x504574 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
// 0x504628 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)
pub fn stub_504628() -> ! {
    todo!("0x504628 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
// 0x504680 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)
pub fn stub_504680() -> ! {
    todo!("0x504680 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::resize(unsigned long,RBX::GameSettings::VideoQuality)")]
// 0x5046e8 — __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE6resizeEmS2_ — std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::resize(unsigned long,RBX::GameSettings::VideoQuality)
pub fn stub_5046e8() -> ! {
    todo!("0x5046e8 __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::push_back(RBX::GameSettings::VideoQuality const&)")]
// 0x50471c — __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE9push_backERKS2_ — std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::push_back(RBX::GameSettings::VideoQuality const&)
pub fn stub_50471c() -> ! {
    todo!("0x50471c __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,RBX::GameSettings::VideoQuality const&)")]
// 0x504744 — __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,RBX::GameSettings::VideoQuality const&)
pub fn stub_504744() -> ! {
    todo!("0x504744 __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_allocate(unsigned long)")]
// 0x504828 — __ZNSt12_Vector_baseIN3RBX12GameSettings12VideoQualityESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_allocate(unsigned long)
pub fn stub_504828() -> ! {
    todo!("0x504828 __ZNSt12_Vector_baseIN3RBX12GameSettings12VideoQualityESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::GameSettings::VideoQuality * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *>(RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *)")]
// 0x504840 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12GameSettings12VideoQualityES6_EET0_T_S8_S7_ — RBX::GameSettings::VideoQuality * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *>(RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *)
pub fn stub_504840() -> ! {
    todo!("0x504840 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12GameSettings12VideoQualityES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,unsigned long,RBX::GameSettings::VideoQuality const&)")]
// 0x50487c — __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,unsigned long,RBX::GameSettings::VideoQuality const&)
pub fn stub_50487c() -> ! {
    todo!("0x50487c __ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::GeometryService::GeometryService(void)")]
// 0x505018 — __ZN3RBX15GeometryServiceC1Ev — RBX::GeometryService::GeometryService(void)
pub fn stub_505018() -> ! {
    todo!("0x505018 __ZN3RBX15GeometryServiceC1Ev")
}

#[doc(alias = "RBX::GeometryService::GeometryService(void)")]
// 0x50501c — __ZN3RBX15GeometryServiceC2Ev — RBX::GeometryService::GeometryService(void)
pub fn stub_50501c() -> ! {
    todo!("0x50501c __ZN3RBX15GeometryServiceC2Ev")
}

#[doc(alias = "RBX::GeometryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x505a48 — __ZN3RBX15GeometryService17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::GeometryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_505a48() -> ! {
    todo!("0x505a48 __ZN3RBX15GeometryService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::FilterDescendents::~FilterDescendents()")]
// 0x505e40 — __ZN3RBX17FilterDescendentsD1Ev — RBX::FilterDescendents::~FilterDescendents()
pub fn stub_505e40() -> ! {
    todo!("0x505e40 __ZN3RBX17FilterDescendentsD1Ev")
}

#[doc(alias = "RBX::GeometryService::~GeometryService()")]
// 0x505e64 — __ZN3RBX15GeometryServiceD1Ev — RBX::GeometryService::~GeometryService()
pub fn stub_505e64() -> ! {
    todo!("0x505e64 __ZN3RBX15GeometryServiceD1Ev")
}

#[doc(alias = "RBX::GeometryService::~GeometryService()")]
// 0x505f48 — __ZN3RBX15GeometryServiceD0Ev — RBX::GeometryService::~GeometryService()
pub fn stub_505f48() -> ! {
    todo!("0x505f48 __ZN3RBX15GeometryServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService()")]
// 0x506068 — __ZThn32_N3RBX15GeometryServiceD1Ev — non-virtual thunk toRBX::GeometryService::~GeometryService()
pub fn stub_506068() -> ! {
    todo!("0x506068 __ZThn32_N3RBX15GeometryServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService()")]
// 0x506148 — __ZThn32_N3RBX15GeometryServiceD0Ev — non-virtual thunk toRBX::GeometryService::~GeometryService()
pub fn stub_506148() -> ! {
    todo!("0x506148 __ZThn32_N3RBX15GeometryServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService()")]
// 0x506268 — __ZThn36_N3RBX15GeometryServiceD1Ev — non-virtual thunk toRBX::GeometryService::~GeometryService()
pub fn stub_506268() -> ! {
    todo!("0x506268 __ZThn36_N3RBX15GeometryServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GeometryService::~GeometryService()")]
// 0x506348 — __ZThn36_N3RBX15GeometryServiceD0Ev — non-virtual thunk toRBX::GeometryService::~GeometryService()
pub fn stub_506348() -> ! {
    todo!("0x506348 __ZThn36_N3RBX15GeometryServiceD0Ev")
}

#[doc(alias = "RBX::FilterDescendents::~FilterDescendents()")]
// 0x507098 — __ZN3RBX17FilterDescendentsD0Ev — RBX::FilterDescendents::~FilterDescendents()
pub fn stub_507098() -> ! {
    todo!("0x507098 __ZN3RBX17FilterDescendentsD0Ev")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::getFVariable(std::string)")]
// 0x5073c0 — __ZN3RBX22GlobalAdvancedSettings12getFVariableESs — RBX::GlobalAdvancedSettings::getFVariable(std::string)
pub fn stub_5073c0() -> ! {
    todo!("0x5073c0 __ZN3RBX22GlobalAdvancedSettings12getFVariableESs")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::getFFlag(std::string)")]
// 0x5075a4 — __ZN3RBX22GlobalAdvancedSettings8getFFlagESs — RBX::GlobalAdvancedSettings::getFFlag(std::string)
pub fn stub_5075a4() -> ! {
    todo!("0x5075a4 __ZN3RBX22GlobalAdvancedSettings8getFFlagESs")
}

#[doc(alias = "RBX::Settings::Settings(std::string const&)")]
// 0x507808 — __ZN3RBX8SettingsC2ERKSs — RBX::Settings::Settings(std::string const&)
pub fn stub_507808() -> ! {
    todo!("0x507808 __ZN3RBX8SettingsC2ERKSs")
}

#[doc(alias = "RBX::Settings::loadState(std::string const&)")]
// 0x5079f8 — __ZN3RBX8Settings9loadStateERKSs — RBX::Settings::loadState(std::string const&)
pub fn stub_5079f8() -> ! {
    todo!("0x5079f8 __ZN3RBX8Settings9loadStateERKSs")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::singleton(void)")]
// 0x5080ec — __ZN3RBX22GlobalAdvancedSettings9singletonEv — RBX::GlobalAdvancedSettings::singleton(void)
pub fn stub_5080ec() -> ! {
    todo!("0x5080ec __ZN3RBX22GlobalAdvancedSettings9singletonEv")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::GlobalAdvancedSettings(void)")]
// 0x508114 — __ZN3RBX22GlobalAdvancedSettingsC2Ev — RBX::GlobalAdvancedSettings::GlobalAdvancedSettings(void)
pub fn stub_508114() -> ! {
    todo!("0x508114 __ZN3RBX22GlobalAdvancedSettingsC2Ev")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// 0x50849c — __ZN3RBX22GlobalAdvancedSettingsD0Ev — RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()
pub fn stub_50849c() -> ! {
    todo!("0x50849c __ZN3RBX22GlobalAdvancedSettingsD0Ev")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// 0x50853c — __ZN3RBX22GlobalAdvancedSettingsD1Ev — RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()
pub fn stub_50853c() -> ! {
    todo!("0x50853c __ZN3RBX22GlobalAdvancedSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// 0x508540 — __ZThn32_N3RBX22GlobalAdvancedSettingsD0Ev — non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()
pub fn stub_508540() -> ! {
    todo!("0x508540 __ZThn32_N3RBX22GlobalAdvancedSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// 0x508548 — __ZThn36_N3RBX22GlobalAdvancedSettingsD0Ev — non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()
pub fn stub_508548() -> ! {
    todo!("0x508548 __ZThn36_N3RBX22GlobalAdvancedSettingsD0Ev")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// 0x508550 — __ZN3RBX22GlobalAdvancedSettingsD2Ev — RBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()
pub fn stub_508550() -> ! {
    todo!("0x508550 __ZN3RBX22GlobalAdvancedSettingsD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// 0x5086d0 — __ZThn32_N3RBX22GlobalAdvancedSettingsD1Ev — non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()
pub fn stub_5086d0() -> ! {
    todo!("0x5086d0 __ZThn32_N3RBX22GlobalAdvancedSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()")]
// 0x5086d8 — __ZThn36_N3RBX22GlobalAdvancedSettingsD1Ev — non-virtual thunk toRBX::GlobalAdvancedSettings::~GlobalAdvancedSettings()
pub fn stub_5086d8() -> ! {
    todo!("0x5086d8 __ZThn36_N3RBX22GlobalAdvancedSettingsD1Ev")
}

#[doc(alias = "RBX::GlobalBasicSettings::singleton(void)")]
// 0x508914 — __ZN3RBX19GlobalBasicSettings9singletonEv — RBX::GlobalBasicSettings::singleton(void)
pub fn stub_508914() -> ! {
    todo!("0x508914 __ZN3RBX19GlobalBasicSettings9singletonEv")
}

#[doc(alias = "RBX::GlobalBasicSettings::reset(void)")]
// 0x50893c — __ZN3RBX19GlobalBasicSettings5resetEv — RBX::GlobalBasicSettings::reset(void)
pub fn stub_50893c() -> ! {
    todo!("0x50893c __ZN3RBX19GlobalBasicSettings5resetEv")
}

#[doc(alias = "RBX::GlobalBasicSettings::GlobalBasicSettings(void)")]
// 0x508958 — __ZN3RBX19GlobalBasicSettingsC2Ev — RBX::GlobalBasicSettings::GlobalBasicSettings(void)
pub fn stub_508958() -> ! {
    todo!("0x508958 __ZN3RBX19GlobalBasicSettingsC2Ev")
}

#[doc(alias = "RBX::GlobalAdvancedSettings::getFVariables(void)")]
// 0x508d14 — __ZN3RBX22GlobalAdvancedSettings13getFVariablesEv — RBX::GlobalAdvancedSettings::getFVariables(void)
pub fn stub_508d14() -> ! {
    todo!("0x508d14 __ZN3RBX22GlobalAdvancedSettings13getFVariablesEv")
}

#[doc(alias = "visit(std::string const&,std::string const&,void *)")]
// 0x508df4 — __ZL5visitRKSsS0_Pv — visit(std::string const&,std::string const&,void *)
pub fn stub_508df4() -> ! {
    todo!("0x508df4 __ZL5visitRKSsS0_Pv")
}

#[doc(alias = "RBX::MergeBinder::resolveRefs(void)")]
// 0x508ebc — __ZN3RBX11MergeBinder11resolveRefsEv — RBX::MergeBinder::resolveRefs(void)
pub fn stub_508ebc() -> ! {
    todo!("0x508ebc __ZN3RBX11MergeBinder11resolveRefsEv")
}

#[doc(alias = "RBX::MergeBinder::~MergeBinder()")]
// 0x508ef4 — __ZN3RBX11MergeBinderD1Ev — RBX::MergeBinder::~MergeBinder()
pub fn stub_508ef4() -> ! {
    todo!("0x508ef4 __ZN3RBX11MergeBinderD1Ev")
}

#[doc(alias = "RBX::Settings::~Settings()")]
// 0x5091cc — __ZN3RBX8SettingsD1Ev — RBX::Settings::~Settings()
pub fn stub_5091cc() -> ! {
    todo!("0x5091cc __ZN3RBX8SettingsD1Ev")
}

#[doc(alias = "RBX::Settings::~Settings()")]
// 0x509208 — __ZN3RBX8SettingsD0Ev — RBX::Settings::~Settings()
pub fn stub_509208() -> ! {
    todo!("0x509208 __ZN3RBX8SettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Settings::~Settings()")]
// 0x509308 — __ZThn32_N3RBX8SettingsD1Ev — non-virtual thunk toRBX::Settings::~Settings()
pub fn stub_509308() -> ! {
    todo!("0x509308 __ZThn32_N3RBX8SettingsD1Ev")
}
