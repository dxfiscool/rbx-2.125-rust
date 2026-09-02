//! core shard HB — 100 core stubs EA-sorted, 0xf56a64..0xf57c24 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HA 0xf56a54).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after HA 0xf56a54 (0xf56a64..0xf57c24, 20514->20614 covered, 1304 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::map<RBX::Name const*,RBX::GameBasicSettings::ControlMode,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::operator[](RBX::Name const* const&)")]
// 0xf56a64 — j___ZNSt3mapIPKN3RBX4NameENS0_17GameBasicSettings11ControlModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf56a64() -> ! {
    todo!("0xf56a64 j___ZNSt3mapIPKN3RBX4NameENS0_17GameBasicSettings11ControlModeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameBasicSettings::RenderQualitySetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::operator[](RBX::Name const* const&)")]
// 0xf56a74 — j___ZNSt3mapIPKN3RBX4NameENS0_17GameBasicSettings20RenderQualitySettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf56a74() -> ! {
    todo!("0xf56a74 j___ZNSt3mapIPKN3RBX4NameENS0_17GameBasicSettings20RenderQualitySettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<std::string,bool,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::operator[](std::string const&)")]
// 0xf56a84 — j___ZNSt3mapISsbSt4lessISsESaISt4pairIKSsbEEEixERS3_
pub fn stub_0xf56a84() -> ! {
    todo!("0xf56a84 j___ZNSt3mapISsbSt4lessISsESaISt4pairIKSsbEEEixERS3_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::ControlMode*,std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>>,RBX::GameBasicSettings::ControlMode const&)")]
// 0xf56a94 — j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf56a94() -> ! {
    todo!("0xf56a94 j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::ControlMode*,std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>>,unsigned long,RBX::GameBasicSettings::ControlMode const&)")]
// 0xf56aa4 — j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf56aa4() -> ! {
    todo!("0xf56aa4 j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::resize(unsigned long,RBX::GameBasicSettings::ControlMode)")]
// 0xf56ab4 — j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE6resizeEmS2_
pub fn stub_0xf56ab4() -> ! {
    todo!("0xf56ab4 j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::ControlMode,std::allocator<RBX::GameBasicSettings::ControlMode>>::push_back(RBX::GameBasicSettings::ControlMode const&)")]
// 0xf56ac4 — j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE9push_backERKS2_
pub fn stub_0xf56ac4() -> ! {
    todo!("0xf56ac4 j___ZNSt6vectorIN3RBX17GameBasicSettings11ControlModeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0xf56ad4 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf56ad4() -> ! {
    todo!("0xf56ad4 j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,unsigned long,RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0xf56ae4 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf56ae4() -> ! {
    todo!("0xf56ae4 j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::resize(unsigned long,RBX::GameBasicSettings::RenderQualitySetting)")]
// 0xf56af4 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE6resizeEmS2_
pub fn stub_0xf56af4() -> ! {
    todo!("0xf56af4 j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::push_back(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0xf56b04 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE9push_backERKS2_
pub fn stub_0xf56b04() -> ! {
    todo!("0xf56b04 j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode> const&)")]
// 0xf56b14 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf56b14() -> ! {
    todo!("0xf56b14 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode> const&)")]
// 0xf56b24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf56b24() -> ! {
    todo!("0xf56b24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode> const&)")]
// 0xf56b34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf56b34() -> ! {
    todo!("0xf56b34 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// 0xf56b44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf56b44() -> ! {
    todo!("0xf56b44 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// 0xf56b54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0xf56b54() -> ! {
    todo!("0xf56b54 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// 0xf56b64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0xf56b64() -> ! {
    todo!("0xf56b64 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::lower_bound(std::string const&)")]
// 0xf56b74 — j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_
pub fn stub_0xf56b74() -> ! {
    todo!("0xf56b74 j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_create_node(std::pair<std::string const,bool> const&)")]
// 0xf56b84 — j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_0xf56b84() -> ! {
    todo!("0xf56b84 j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_insert_unique(std::pair<std::string const,bool> const&)")]
// 0xf56b94 — j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_0xf56b94() -> ! {
    todo!("0xf56b94 j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,bool>>,std::pair<std::string const,bool> const&)")]
// 0xf56ba4 — j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_0xf56ba4() -> ! {
    todo!("0xf56ba4 j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::find(std::string const&)")]
// 0xf56bb4 — j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_
pub fn stub_0xf56bb4() -> ! {
    todo!("0xf56bb4 j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE4findERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,bool>> *)")]
// 0xf56bc4 — j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_0xf56bc4() -> ! {
    todo!("0xf56bc4 j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool>,std::_Select1st<std::pair<std::string const,bool>>,std::less<std::string>,std::allocator<std::pair<std::string const,bool>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,bool> const&)")]
// 0xf56bd4 — j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_0xf56bd4() -> ! {
    todo!("0xf56bd4 j___ZNSt8_Rb_treeISsSt4pairIKSsbESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
// 0xf56c74 — j___ZN3RBX18RenderHooksServiceD2Ev
pub fn stub_0xf56c74() -> ! {
    todo!("0xf56c74 j___ZN3RBX18RenderHooksServiceD2Ev")
}

#[doc(alias = "RBX::ClientAppSettings::ClientAppSettings(void)")]
// 0xf56dc4 — j___ZN3RBX17ClientAppSettingsC2Ev
pub fn stub_0xf56dc4() -> ! {
    todo!("0xf56dc4 j___ZN3RBX17ClientAppSettingsC2Ev")
}

#[doc(alias = "RBX::ClientAppSettings::~ClientAppSettings()")]
// 0xf56dd4 — j___ZN3RBX17ClientAppSettingsD2Ev
pub fn stub_0xf56dd4() -> ! {
    todo!("0xf56dd4 j___ZN3RBX17ClientAppSettingsD2Ev")
}

#[doc(alias = "RBX::CustomEvent::addReceiver(RBX::CustomEventReceiver *)")]
// 0xf56f34 — j___ZN3RBX11CustomEvent11addReceiverEPNS_19CustomEventReceiverE
pub fn stub_0xf56f34() -> ! {
    todo!("0xf56f34 j___ZN3RBX11CustomEvent11addReceiverEPNS_19CustomEventReceiverE")
}

#[doc(alias = "RBX::CustomEvent::removeReceiver(RBX::CustomEventReceiver *)")]
// 0xf56f44 — j___ZN3RBX11CustomEvent14removeReceiverEPNS_19CustomEventReceiverE
pub fn stub_0xf56f44() -> ! {
    todo!("0xf56f44 j___ZN3RBX11CustomEvent14removeReceiverEPNS_19CustomEventReceiverE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEventReceiver> RBX::shared_from<RBX::CustomEventReceiver>(RBX::CustomEventReceiver*)")]
// 0xf56f54 — j___ZN3RBX11shared_fromINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::CustomEventReceiver> RBX::shared_from<RBX::CustomEventReceiver>(RBX::CustomEventReceiver*)
pub fn stub_0xf56f54() -> ! {
    todo!("0xf56f54 j___ZN3RBX11shared_fromINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::CustomEventReceiver::~CustomEventReceiver()")]
// 0xf56f64 — j___ZN3RBX19CustomEventReceiverD2Ev
pub fn stub_0xf56f64() -> ! {
    todo!("0xf56f64 j___ZN3RBX19CustomEventReceiverD2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CustomEvent>::shared_ptr<RBX::CustomEvent>(rbx_core::WeakPtr<RBX::CustomEvent> const&,boost::detail::sp_nothrow_tag)")]
// 0xf56f74 — j___ZN5boost10shared_ptrIN3RBX11CustomEventEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::CustomEvent>::shared_ptr<RBX::CustomEvent>(boost::weak_ptr<RBX::CustomEvent> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0xf56f74() -> ! {
    todo!("0xf56f74 j___ZN5boost10shared_ptrIN3RBX11CustomEventEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::CustomEventReceiver>::weak_ptr<RBX::CustomEventReceiver>(rbx_core::SharedPtr<RBX::CustomEventReceiver> const&,boost::detail::sp_enable_if_convertible<RBX::CustomEventReceiver,RBX::CustomEventReceiver>::type)")]
// 0xf56f84 — j___ZN5boost8weak_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::CustomEventReceiver>::weak_ptr<RBX::CustomEventReceiver>(boost::shared_ptr<RBX::CustomEventReceiver> const&,boost::detail::sp_enable_if_convertible<RBX::CustomEventReceiver,RBX::CustomEventReceiver>::type)
pub fn stub_0xf56f84() -> ! {
    todo!("0xf56f84 j___ZN5boost8weak_ptrIN3RBX19CustomEventReceiverEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "std::list<rbx_core::WeakPtr<RBX::CustomEventReceiver>,std::allocator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>>::_M_erase(std::_List_iterator<rbx_core::WeakPtr<RBX::CustomEventReceiver>>)")]
// 0xf56fa4 — j___ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// was: std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_erase(std::_List_iterator<boost::weak_ptr<RBX::CustomEventReceiver>>)
pub fn stub_0xf56fa4() -> ! {
    todo!("0xf56fa4 j___ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")
}

#[doc(alias = "boost::function<bool ()(std::string,int,std::string)>::operator=(boost::function<bool ()(std::string,int,std::string)> const&)")]
// 0xf57224 — j___ZN5boost8functionIFbSsiSsEEaSERKS2_
pub fn stub_0xf57224() -> ! {
    todo!("0xf57224 j___ZN5boost8functionIFbSsiSsEEaSERKS2_")
}

#[doc(alias = "boost::function<void ()(std::string)>::operator=(boost::function<void ()(std::string)> const&)")]
// 0xf57244 — j___ZN5boost8functionIFvSsEEaSERKS2_
pub fn stub_0xf57244() -> ! {
    todo!("0xf57244 j___ZN5boost8functionIFvSsEEaSERKS2_")
}

#[doc(alias = "boost::function1<void,std::string>::move_assign(boost::function1<void,std::string>&)")]
// 0xf57254 — j___ZN5boost9function1IvSsE11move_assignERS1_
pub fn stub_0xf57254() -> ! {
    todo!("0xf57254 j___ZN5boost9function1IvSsE11move_assignERS1_")
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::move_assign(boost::function3<bool,std::string,int,std::string>&)")]
// 0xf57284 — j___ZN5boost9function3IbSsiSsE11move_assignERS1_
pub fn stub_0xf57284() -> ! {
    todo!("0xf57284 j___ZN5boost9function3IbSsiSsE11move_assignERS1_")
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::assign_to_own(boost::function3<bool,std::string,int,std::string> const&)")]
// 0xf57294 — j___ZN5boost9function3IbSsiSsE13assign_to_ownERKS1_
pub fn stub_0xf57294() -> ! {
    todo!("0xf57294 j___ZN5boost9function3IbSsiSsE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::swap(boost::function3<bool,std::string,int,std::string>&)")]
// 0xf572a4 — j___ZN5boost9function3IbSsiSsE4swapERS1_
pub fn stub_0xf572a4() -> ! {
    todo!("0xf572a4 j___ZN5boost9function3IbSsiSsE4swapERS1_")
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::clear(void)")]
// 0xf572b4 — j___ZN5boost9function3IbSsiSsE5clearEv
pub fn stub_0xf572b4() -> ! {
    todo!("0xf572b4 j___ZN5boost9function3IbSsiSsE5clearEv")
}

#[doc(alias = "boost::function3<bool,std::string,int,std::string>::operator()(std::string,int,std::string)const")]
// 0xf57334 — j___ZNK5boost9function3IbSsiSsEclESsiSs
pub fn stub_0xf57334() -> ! {
    todo!("0xf57334 j___ZNK5boost9function3IbSsiSsEclESsiSs")
}

#[doc(alias = "unsigned int RBX::readCountValue<RBX::StringReadBuffer>(RBX::StringReadBuffer &)")]
// 0xf57804 — j___ZN3RBX14readCountValueINS_16StringReadBufferEEEjRT_
pub fn stub_0xf57804() -> ! {
    todo!("0xf57804 j___ZN3RBX14readCountValueINS_16StringReadBufferEEEjRT_")
}

#[doc(alias = "void RBX::writeCountValue<RBX::StringWriteBuffer>(RBX::StringWriteBuffer &,unsigned int)")]
// 0xf57814 — j___ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j
pub fn stub_0xf57814() -> ! {
    todo!("0xf57814 j___ZN3RBX15writeCountValueINS_17StringWriteBufferEEEvRT_j")
}

#[doc(alias = "RBX::StringReadBuffer::operator>>(unsigned char &)")]
// 0xf57824 — j___ZN3RBX16StringReadBufferrsERh
pub fn stub_0xf57824() -> ! {
    todo!("0xf57824 j___ZN3RBX16StringReadBufferrsERh")
}

#[doc(alias = "RBX::TerrainPartition::~TerrainPartition()")]
// 0xf57844 — j___ZN3RBX16TerrainPartitionD2Ev
pub fn stub_0xf57844() -> ! {
    todo!("0xf57844 j___ZN3RBX16TerrainPartitionD2Ev")
}

#[doc(alias = "RBX::Voxel::CellMaterial * rbx::any_cast<RBX::Voxel::CellMaterial,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf57894 — j___ZN3rbx8any_castIN3RBX5Voxel12CellMaterialENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf57894() -> ! {
    todo!("0xf57894 j___ZN3rbx8any_castIN3RBX5Voxel12CellMaterialENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::WaterCellForce * rbx::any_cast<RBX::Voxel::WaterCellForce,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf578a4 — j___ZN3rbx8any_castIN3RBX5Voxel14WaterCellForceENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf578a4() -> ! {
    todo!("0xf578a4 j___ZN3rbx8any_castIN3RBX5Voxel14WaterCellForceENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellOrientation * rbx::any_cast<RBX::Voxel::CellOrientation,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf578b4 — j___ZN3rbx8any_castIN3RBX5Voxel15CellOrientationENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf578b4() -> ! {
    todo!("0xf578b4 j___ZN3rbx8any_castIN3RBX5Voxel15CellOrientationENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::WaterCellDirection * rbx::any_cast<RBX::Voxel::WaterCellDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf578c4 — j___ZN3rbx8any_castIN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf578c4() -> ! {
    todo!("0xf578c4 j___ZN3rbx8any_castIN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellBlock * rbx::any_cast<RBX::Voxel::CellBlock,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf578d4 — j___ZN3rbx8any_castIN3RBX5Voxel9CellBlockENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_0xf578d4() -> ! {
    todo!("0xf578d4 j___ZN3rbx8any_castIN3RBX5Voxel9CellBlockENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellMaterial & rbx::any_cast<RBX::Voxel::CellMaterial &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf578e4 — j___ZN3rbx8any_castIRN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf578e4() -> ! {
    todo!("0xf578e4 j___ZN3rbx8any_castIRN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::WaterCellForce & rbx::any_cast<RBX::Voxel::WaterCellForce &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf578f4 — j___ZN3rbx8any_castIRN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf578f4() -> ! {
    todo!("0xf578f4 j___ZN3rbx8any_castIRN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellOrientation & rbx::any_cast<RBX::Voxel::CellOrientation &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf57904 — j___ZN3rbx8any_castIRN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf57904() -> ! {
    todo!("0xf57904 j___ZN3rbx8any_castIRN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::WaterCellDirection & rbx::any_cast<RBX::Voxel::WaterCellDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf57914 — j___ZN3rbx8any_castIRN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf57914() -> ! {
    todo!("0xf57914 j___ZN3rbx8any_castIRN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Voxel::CellBlock & rbx::any_cast<RBX::Voxel::CellBlock &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf57924 — j___ZN3rbx8any_castIRN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0xf57924() -> ! {
    todo!("0xf57924 j___ZN3rbx8any_castIRN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_allocate(unsigned long)")]
// 0xf57974 — j___ZNSt12_Vector_baseIN3RBX5Voxel12CellMaterialESaIS2_EE11_M_allocateEm
pub fn stub_0xf57974() -> ! {
    todo!("0xf57974 j___ZNSt12_Vector_baseIN3RBX5Voxel12CellMaterialESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_allocate(unsigned long)")]
// 0xf57984 — j___ZNSt12_Vector_baseIN3RBX5Voxel14WaterCellForceESaIS2_EE11_M_allocateEm
pub fn stub_0xf57984() -> ! {
    todo!("0xf57984 j___ZNSt12_Vector_baseIN3RBX5Voxel14WaterCellForceESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_allocate(unsigned long)")]
// 0xf57994 — j___ZNSt12_Vector_baseIN3RBX5Voxel15CellOrientationESaIS2_EE11_M_allocateEm
pub fn stub_0xf57994() -> ! {
    todo!("0xf57994 j___ZNSt12_Vector_baseIN3RBX5Voxel15CellOrientationESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_allocate(unsigned long)")]
// 0xf579a4 — j___ZNSt12_Vector_baseIN3RBX5Voxel18WaterCellDirectionESaIS2_EE11_M_allocateEm
pub fn stub_0xf579a4() -> ! {
    todo!("0xf579a4 j___ZNSt12_Vector_baseIN3RBX5Voxel18WaterCellDirectionESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_allocate(unsigned long)")]
// 0xf579b4 — j___ZNSt12_Vector_baseIN3RBX5Voxel9CellBlockESaIS2_EE11_M_allocateEm
pub fn stub_0xf579b4() -> ! {
    todo!("0xf579b4 j___ZNSt12_Vector_baseIN3RBX5Voxel9CellBlockESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_allocate(unsigned long)")]
// 0xf579c4 — j___ZNSt12_Vector_baseIPN3RBX5Voxel18CellChangeListenerESaIS3_EE11_M_allocateEm
pub fn stub_0xf579c4() -> ! {
    todo!("0xf579c4 j___ZNSt12_Vector_baseIPN3RBX5Voxel18CellChangeListenerESaIS3_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Voxel::CellMaterial * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *>(RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *,RBX::Voxel::CellMaterial *)")]
// 0xf579d4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel12CellMaterialES6_EET0_T_S8_S7_
pub fn stub_0xf579d4() -> ! {
    todo!("0xf579d4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel12CellMaterialES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Voxel::WaterCellForce * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *>(RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *,RBX::Voxel::WaterCellForce *)")]
// 0xf579e4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel14WaterCellForceES6_EET0_T_S8_S7_
pub fn stub_0xf579e4() -> ! {
    todo!("0xf579e4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel14WaterCellForceES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Voxel::CellOrientation * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *>(RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *,RBX::Voxel::CellOrientation *)")]
// 0xf579f4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel15CellOrientationES6_EET0_T_S8_S7_
pub fn stub_0xf579f4() -> ! {
    todo!("0xf579f4 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel15CellOrientationES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Voxel::WaterCellDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *>(RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *,RBX::Voxel::WaterCellDirection *)")]
// 0xf57a04 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel18WaterCellDirectionES6_EET0_T_S8_S7_
pub fn stub_0xf57a04() -> ! {
    todo!("0xf57a04 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel18WaterCellDirectionES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Voxel::CellBlock * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *>(RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *,RBX::Voxel::CellBlock *)")]
// 0xf57a14 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel9CellBlockES6_EET0_T_S8_S7_
pub fn stub_0xf57a14() -> ! {
    todo!("0xf57a14 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Voxel9CellBlockES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellMaterial,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::operator[](RBX::Name const* const&)")]
// 0xf57a24 — j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel12CellMaterialESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf57a24() -> ! {
    todo!("0xf57a24 j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel12CellMaterialESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellForce,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::operator[](RBX::Name const* const&)")]
// 0xf57a34 — j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel14WaterCellForceESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf57a34() -> ! {
    todo!("0xf57a34 j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel14WaterCellForceESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellOrientation,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::operator[](RBX::Name const* const&)")]
// 0xf57a44 — j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel15CellOrientationESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf57a44() -> ! {
    todo!("0xf57a44 j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel15CellOrientationESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::WaterCellDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::operator[](RBX::Name const* const&)")]
// 0xf57a54 — j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel18WaterCellDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf57a54() -> ! {
    todo!("0xf57a54 j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel18WaterCellDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Voxel::CellBlock,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::operator[](RBX::Name const* const&)")]
// 0xf57a64 — j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel9CellBlockESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_0xf57a64() -> ! {
    todo!("0xf57a64 j___ZNSt3mapIPKN3RBX4NameENS0_5Voxel9CellBlockESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,RBX::Voxel::CellMaterial const&)")]
// 0xf57a74 — j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf57a74() -> ! {
    todo!("0xf57a74 j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellMaterial*,std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>>,unsigned long,RBX::Voxel::CellMaterial const&)")]
// 0xf57a84 — j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf57a84() -> ! {
    todo!("0xf57a84 j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::resize(unsigned long,RBX::Voxel::CellMaterial)")]
// 0xf57a94 — j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE6resizeEmS2_
pub fn stub_0xf57a94() -> ! {
    todo!("0xf57a94 j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellMaterial,std::allocator<RBX::Voxel::CellMaterial>>::push_back(RBX::Voxel::CellMaterial const&)")]
// 0xf57aa4 — j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE9push_backERKS2_
pub fn stub_0xf57aa4() -> ! {
    todo!("0xf57aa4 j___ZNSt6vectorIN3RBX5Voxel12CellMaterialESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,RBX::Voxel::WaterCellForce const&)")]
// 0xf57ab4 — j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf57ab4() -> ! {
    todo!("0xf57ab4 j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellForce*,std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>>,unsigned long,RBX::Voxel::WaterCellForce const&)")]
// 0xf57ac4 — j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf57ac4() -> ! {
    todo!("0xf57ac4 j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::resize(unsigned long,RBX::Voxel::WaterCellForce)")]
// 0xf57ad4 — j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE6resizeEmS2_
pub fn stub_0xf57ad4() -> ! {
    todo!("0xf57ad4 j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellForce,std::allocator<RBX::Voxel::WaterCellForce>>::push_back(RBX::Voxel::WaterCellForce const&)")]
// 0xf57ae4 — j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE9push_backERKS2_
pub fn stub_0xf57ae4() -> ! {
    todo!("0xf57ae4 j___ZNSt6vectorIN3RBX5Voxel14WaterCellForceESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,RBX::Voxel::CellOrientation const&)")]
// 0xf57af4 — j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf57af4() -> ! {
    todo!("0xf57af4 j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellOrientation*,std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>>,unsigned long,RBX::Voxel::CellOrientation const&)")]
// 0xf57b04 — j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf57b04() -> ! {
    todo!("0xf57b04 j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::resize(unsigned long,RBX::Voxel::CellOrientation)")]
// 0xf57b14 — j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE6resizeEmS2_
pub fn stub_0xf57b14() -> ! {
    todo!("0xf57b14 j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellOrientation,std::allocator<RBX::Voxel::CellOrientation>>::push_back(RBX::Voxel::CellOrientation const&)")]
// 0xf57b24 — j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE9push_backERKS2_
pub fn stub_0xf57b24() -> ! {
    todo!("0xf57b24 j___ZNSt6vectorIN3RBX5Voxel15CellOrientationESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,RBX::Voxel::WaterCellDirection const&)")]
// 0xf57b34 — j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf57b34() -> ! {
    todo!("0xf57b34 j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::WaterCellDirection*,std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>>,unsigned long,RBX::Voxel::WaterCellDirection const&)")]
// 0xf57b44 — j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf57b44() -> ! {
    todo!("0xf57b44 j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::resize(unsigned long,RBX::Voxel::WaterCellDirection)")]
// 0xf57b54 — j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE6resizeEmS2_
pub fn stub_0xf57b54() -> ! {
    todo!("0xf57b54 j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::WaterCellDirection,std::allocator<RBX::Voxel::WaterCellDirection>>::push_back(RBX::Voxel::WaterCellDirection const&)")]
// 0xf57b64 — j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE9push_backERKS2_
pub fn stub_0xf57b64() -> ! {
    todo!("0xf57b64 j___ZNSt6vectorIN3RBX5Voxel18WaterCellDirectionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::Cell*,std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>>,unsigned long,RBX::Voxel::Cell const&)")]
// 0xf57b74 — j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf57b74() -> ! {
    todo!("0xf57b74 j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::Cell,std::allocator<RBX::Voxel::Cell>>::resize(unsigned long,RBX::Voxel::Cell)")]
// 0xf57b84 — j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE6resizeEmS2_
pub fn stub_0xf57b84() -> ! {
    todo!("0xf57b84 j___ZNSt6vectorIN3RBX5Voxel4CellESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue,std::allocator<RBX::Voxel::ChunkMap<RBX::Voxel::Grid::Chunk>::StoredValue>>::~vector()")]
// 0xf57b94 — j___ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EED2Ev
pub fn stub_0xf57b94() -> ! {
    todo!("0xf57b94 j___ZNSt6vectorIN3RBX5Voxel8ChunkMapINS1_4Grid5ChunkEE11StoredValueESaIS6_EED2Ev")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,RBX::Voxel::CellBlock const&)")]
// 0xf57ba4 — j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0xf57ba4() -> ! {
    todo!("0xf57ba4 j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Voxel::CellBlock*,std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>>,unsigned long,RBX::Voxel::CellBlock const&)")]
// 0xf57bb4 — j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0xf57bb4() -> ! {
    todo!("0xf57bb4 j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::resize(unsigned long,RBX::Voxel::CellBlock)")]
// 0xf57bc4 — j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE6resizeEmS2_
pub fn stub_0xf57bc4() -> ! {
    todo!("0xf57bc4 j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellBlock,std::allocator<RBX::Voxel::CellBlock>>::push_back(RBX::Voxel::CellBlock const&)")]
// 0xf57bd4 — j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE9push_backERKS2_
pub fn stub_0xf57bd4() -> ! {
    todo!("0xf57bd4 j___ZNSt6vectorIN3RBX5Voxel9CellBlockESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Voxel::CellChangeListener **,std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>>,RBX::Voxel::CellChangeListener * const&)")]
// 0xf57be4 — j___ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
pub fn stub_0xf57be4() -> ! {
    todo!("0xf57be4 j___ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_")
}

#[doc(alias = "std::vector<RBX::Voxel::CellChangeListener *,std::allocator<RBX::Voxel::CellChangeListener *>>::push_back(RBX::Voxel::CellChangeListener * const&)")]
// 0xf57bf4 — j___ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE9push_backERKS3_
pub fn stub_0xf57bf4() -> ! {
    todo!("0xf57bf4 j___ZNSt6vectorIPN3RBX5Voxel18CellChangeListenerESaIS3_EE9push_backERKS3_")
}

#[doc(alias = "std::vector<std::vector<bool,std::allocator<bool>>,std::allocator<std::vector<bool,std::allocator<bool>>>>::~vector()")]
// 0xf57c04 — j___ZNSt6vectorIS_IbSaIbEESaIS1_EED2Ev
pub fn stub_0xf57c04() -> ! {
    todo!("0xf57c04 j___ZNSt6vectorIS_IbSaIbEESaIS1_EED2Ev")
}

#[doc(alias = "std::vector<unsigned char,std::allocator<unsigned char>>::resize(unsigned long,unsigned char)")]
// 0xf57c14 — j___ZNSt6vectorIhSaIhEE6resizeEmh
pub fn stub_0xf57c14() -> ! {
    todo!("0xf57c14 j___ZNSt6vectorIhSaIhEE6resizeEmh")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial> const&)")]
// 0xf57c24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0xf57c24() -> ! {
    todo!("0xf57c24 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

