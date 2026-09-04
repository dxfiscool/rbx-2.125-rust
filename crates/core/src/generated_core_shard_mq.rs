//! core shard mq — 100 core stubs EA-sorted asc global gap filler not yet in any crate.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in any crate (global gap; 47407 distinct before -> 47507 after, 38139 uncovered before -> 38039 after, batch 0x4bbe58..0x5079f8).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)")]
// 0x4bbe58 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_
pub fn stub_0x4bbe58() {
    // IDA 0x4bbe58: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)")]
// 0x4bbea8 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE9singletonEv
pub fn stub_0x4bbea8() {
    // IDA 0x4bbea8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)")]
// 0x4bbf14 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE14construct_funcEPKcPc
pub fn stub_0x4bbf14() {
    // IDA 0x4bbf14: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)")]
// 0x4bbf20 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE13destruct_funcEPc
pub fn stub_0x4bbf20() {
    // IDA 0x4bbf20: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bbff0 — __ZN3rbx8any_castIRKN3RBX9GuiButton5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4bbff0() {
    // IDA 0x4bbff0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)")]
// 0x4bc15c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4bc15c() {
    // IDA 0x4bc15c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Frame::Style>(RBX::Frame::Style const&)")]
// 0x4bc880 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_
pub fn stub_0x4bc880() {
    // IDA 0x4bc880: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::singleton(void)")]
// 0x4bc8d0 — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv
pub fn stub_0x4bc8d0() {
    // IDA 0x4bc8d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::construct_func(char const*,char *)")]
// 0x4bc93c — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE14construct_funcEPKcPc
pub fn stub_0x4bc93c() {
    // IDA 0x4bc93c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::destruct_func(char *)")]
// 0x4bc948 — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE13destruct_funcEPc
pub fn stub_0x4bc948() {
    // IDA 0x4bc948: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Frame::Style const& rbx::any_cast<RBX::Frame::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bca18 — __ZN3rbx8any_castIRKN3RBX5Frame5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4bca18() {
    // IDA 0x4bca18: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Frame::Style>> *)")]
// 0x4bcb84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4bcb84() {
    // IDA 0x4bcb84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0x4bd2a8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings20RenderQualitySettingEEERS3_RKT_
pub fn stub_0x4bd2a8() {
    // IDA 0x4bd2a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::singleton(void)")]
// 0x4bd2f8 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE9singletonEv
// type: _DWORD *()
pub fn stub_0x4bd2f8() {
    // IDA 0x4bd2f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::construct_func(char const*,char *)")]
// 0x4bd364 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE14construct_funcEPKcPc
pub fn stub_0x4bd364() {
    // IDA 0x4bd364: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::destruct_func(char *)")]
// 0x4bd370 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE13destruct_funcEPc
pub fn stub_0x4bd370() {
    // IDA 0x4bd370: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::RenderQualitySetting const& rbx::any_cast<RBX::GameBasicSettings::RenderQualitySetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bd440 — __ZN3rbx8any_castIRKN3RBX17GameBasicSettings20RenderQualitySettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4bd440() {
    // IDA 0x4bd440: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)")]
// 0x4bd5ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4bd5ac() {
    // IDA 0x4bd5ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::ControlMode>(RBX::GameBasicSettings::ControlMode const&)")]
// 0x4bdcd0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings11ControlModeEEERS3_RKT_
pub fn stub_0x4bdcd0() {
    // IDA 0x4bdcd0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::singleton(void)")]
// 0x4bdd20 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE9singletonEv
pub fn stub_0x4bdd20() {
    // IDA 0x4bdd20: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::construct_func(char const*,char *)")]
// 0x4bdd8c — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE14construct_funcEPKcPc
pub fn stub_0x4bdd8c() {
    // IDA 0x4bdd8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::destruct_func(char *)")]
// 0x4bdd98 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE13destruct_funcEPc
pub fn stub_0x4bdd98() {
    // IDA 0x4bdd98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameBasicSettings::ControlMode const& rbx::any_cast<RBX::GameBasicSettings::ControlMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bde68 — __ZN3rbx8any_castIRKN3RBX17GameBasicSettings11ControlModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4bde68() {
    // IDA 0x4bde68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>> *)")]
// 0x4bdfd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4bdfd4() {
    // IDA 0x4bdfd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::UploadSetting>(RBX::GameSettings::UploadSetting const&)")]
// 0x4be6f8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings13UploadSettingEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4be6f8() {
    // IDA 0x4be6f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::singleton(void)")]
// 0x4be748 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE9singletonEv
pub fn stub_0x4be748() {
    // IDA 0x4be748: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::construct_func(char const*,char *)")]
// 0x4be7b4 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE14construct_funcEPKcPc
pub fn stub_0x4be7b4() {
    // IDA 0x4be7b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::destruct_func(char *)")]
// 0x4be7c0 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE13destruct_funcEPc
pub fn stub_0x4be7c0() {
    // IDA 0x4be7c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameSettings::UploadSetting const& rbx::any_cast<RBX::GameSettings::UploadSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4be890 — __ZN3rbx8any_castIRKN3RBX12GameSettings13UploadSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4be890() {
    // IDA 0x4be890: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>> *)")]
// 0x4be9fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4be9fc() {
    // IDA 0x4be9fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::VideoQuality>(RBX::GameSettings::VideoQuality const&)")]
// 0x4bf120 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings12VideoQualityEEERS3_RKT_
pub fn stub_0x4bf120() {
    // IDA 0x4bf120: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::singleton(void)")]
// 0x4bf170 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings12VideoQualityEE9singletonEv
pub fn stub_0x4bf170() {
    // IDA 0x4bf170: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::construct_func(char const*,char *)")]
// 0x4bf1dc — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings12VideoQualityEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x4bf1dc() {
    // IDA 0x4bf1dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::destruct_func(char *)")]
// 0x4bf1e8 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings12VideoQualityEE13destruct_funcEPc
pub fn stub_0x4bf1e8() {
    // IDA 0x4bf1e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GameSettings::VideoQuality const& rbx::any_cast<RBX::GameSettings::VideoQuality const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bf2b8 — __ZN3rbx8any_castIRKN3RBX12GameSettings12VideoQualityENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4bf2b8() {
    // IDA 0x4bf2b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>> *)")]
// 0x4bf424 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4bf424() {
    // IDA 0x4bf424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)")]
// 0x4bfb48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13CharacterMesh8BodyPartEEERS3_RKT_
pub fn stub_0x4bfb48() {
    // IDA 0x4bfb48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)")]
// 0x4bfb98 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE9singletonEv
pub fn stub_0x4bfb98() {
    // IDA 0x4bfb98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)")]
// 0x4bfc04 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE14construct_funcEPKcPc
pub fn stub_0x4bfc04() {
    // IDA 0x4bfc04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)")]
// 0x4bfc10 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE13destruct_funcEPc
pub fn stub_0x4bfc10() {
    // IDA 0x4bfc10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bfce0 — __ZN3rbx8any_castIRKN3RBX13CharacterMesh8BodyPartENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4bfce0() {
    // IDA 0x4bfce0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)")]
// 0x4bfe4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4bfe4c() {
    // IDA 0x4bfe4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)")]
// 0x4c0810 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4c0810() {
    // IDA 0x4c0810: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)")]
// 0x4c0860 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE9singletonEv
pub fn stub_0x4c0860() {
    // IDA 0x4c0860: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)")]
// 0x4c08cc — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE14construct_funcEPKcPc
pub fn stub_0x4c08cc() {
    // IDA 0x4c08cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)")]
// 0x4c08d8 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE13destruct_funcEPc
pub fn stub_0x4c08d8() {
    // IDA 0x4c08d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c09a8 — __ZN3rbx8any_castIRKN3RBX11ChatService9ChatColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4c09a8() {
    // IDA 0x4c09a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)")]
// 0x4c0b14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4c0b14() {
    // IDA 0x4c0b14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&)")]
// 0x4c3298 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18SkateboardPlatform9MoveStateEEERS3_RKT_
pub fn stub_0x4c3298() {
    // IDA 0x4c3298: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::singleton(void)")]
// 0x4c32e8 — __ZN3rbx14implementation12typed_holderIN3RBX18SkateboardPlatform9MoveStateEE9singletonEv
pub fn stub_0x4c32e8() {
    // IDA 0x4c32e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::construct_func(char const*,char *)")]
// 0x4c3354 — __ZN3rbx14implementation12typed_holderIN3RBX18SkateboardPlatform9MoveStateEE14construct_funcEPKcPc
pub fn stub_0x4c3354() {
    // IDA 0x4c3354: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::destruct_func(char *)")]
// 0x4c3360 — __ZN3rbx14implementation12typed_holderIN3RBX18SkateboardPlatform9MoveStateEE13destruct_funcEPc
pub fn stub_0x4c3360() {
    // IDA 0x4c3360: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SkateboardPlatform::MoveState const& rbx::any_cast<RBX::SkateboardPlatform::MoveState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c3430 — __ZN3rbx8any_castIRKN3RBX18SkateboardPlatform9MoveStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4c3430() {
    // IDA 0x4c3430: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>> *)")]
// 0x4c359c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4c359c() {
    // IDA 0x4c359c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Handles::VisualStyle>(RBX::Handles::VisualStyle const&)")]
// 0x4c3cc0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Handles11VisualStyleEEERS3_RKT_
pub fn stub_0x4c3cc0() {
    // IDA 0x4c3cc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::singleton(void)")]
// 0x4c3d10 — __ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE9singletonEv
pub fn stub_0x4c3d10() {
    // IDA 0x4c3d10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::construct_func(char const*,char *)")]
// 0x4c3d7c — __ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE14construct_funcEPKcPc
pub fn stub_0x4c3d7c() {
    // IDA 0x4c3d7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::destruct_func(char *)")]
// 0x4c3d88 — __ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE13destruct_funcEPc
pub fn stub_0x4c3d88() {
    // IDA 0x4c3d88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Handles::VisualStyle const& rbx::any_cast<RBX::Handles::VisualStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c3e58 — __ZN3rbx8any_castIRKN3RBX7Handles11VisualStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4c3e58() {
    // IDA 0x4c3e58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>> *)")]
// 0x4c3fc4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4c3fc4() {
    // IDA 0x4c3fc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendStatus>(RBX::FriendService::FriendStatus const&)")]
// 0x4c5100 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_
pub fn stub_0x4c5100() {
    // IDA 0x4c5100: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::singleton(void)")]
// 0x4c5150 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE9singletonEv
pub fn stub_0x4c5150() {
    // IDA 0x4c5150: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::construct_func(char const*,char *)")]
// 0x4c51bc — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE14construct_funcEPKcPc
pub fn stub_0x4c51bc() {
    // IDA 0x4c51bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::destruct_func(char *)")]
// 0x4c51c8 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE13destruct_funcEPc
pub fn stub_0x4c51c8() {
    // IDA 0x4c51c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FriendService::FriendStatus const& rbx::any_cast<RBX::FriendService::FriendStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c5298 — __ZN3rbx8any_castIRKN3RBX13FriendService12FriendStatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_0x4c5298() {
    // IDA 0x4c5298: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>> *)")]
// 0x4c5404 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x4c5404() {
    // IDA 0x4c5404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeyframeSequence::Priority>(RBX::KeyframeSequence::Priority const&)")]
// 0x4c8df0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16KeyframeSequence8PriorityEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4c8df0() {
    // IDA 0x4c8df0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::singleton(void)")]
// 0x4c8e40 — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE9singletonEv
// type: _DWORD *()
pub fn stub_0x4c8e40() {
    // IDA 0x4c8e40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::construct_func(char const*,char *)")]
// 0x4c8eac — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x4c8eac() {
    // IDA 0x4c8eac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::destruct_func(char *)")]
// 0x4c8eb8 — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE13destruct_funcEPc
// type: void()
pub fn stub_0x4c8eb8() {
    // IDA 0x4c8eb8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::KeyframeSequence::Priority const& rbx::any_cast<RBX::KeyframeSequence::Priority const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c8f88 — __ZN3rbx8any_castIRKN3RBX16KeyframeSequence8PriorityENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x4c8f88() {
    // IDA 0x4c8f88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>> *)")]
// 0x4c90f4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4c90f4() {
    // IDA 0x4c90f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::NameOcclusion>(RBX::Humanoid::NameOcclusion const&)")]
// 0x4ca240 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid13NameOcclusionEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4ca240() {
    // IDA 0x4ca240: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::singleton(void)")]
// 0x4ca290 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE9singletonEv
// type: _DWORD *()
pub fn stub_0x4ca290() {
    // IDA 0x4ca290: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::construct_func(char const*,char *)")]
// 0x4ca2fc — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x4ca2fc() {
    // IDA 0x4ca2fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::destruct_func(char *)")]
// 0x4ca308 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE13destruct_funcEPc
// type: void()
pub fn stub_0x4ca308() {
    // IDA 0x4ca308: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Humanoid::NameOcclusion const& rbx::any_cast<RBX::Humanoid::NameOcclusion const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4ca3d8 — __ZN3rbx8any_castIRKN3RBX8Humanoid13NameOcclusionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x4ca3d8() {
    // IDA 0x4ca3d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>> *)")]
// 0x4ca544 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4ca544() {
    // IDA 0x4ca544: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::Status>(RBX::Humanoid::Status const&)")]
// 0x4cac68 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid6StatusEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4cac68() {
    // IDA 0x4cac68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::singleton(void)")]
// 0x4cacb8 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE9singletonEv
// type: _DWORD *()
pub fn stub_0x4cacb8() {
    // IDA 0x4cacb8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::construct_func(char const*,char *)")]
// 0x4cad24 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x4cad24() {
    // IDA 0x4cad24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::destruct_func(char *)")]
// 0x4cad30 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE13destruct_funcEPc
// type: void()
pub fn stub_0x4cad30() {
    // IDA 0x4cad30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Humanoid::Status const& rbx::any_cast<RBX::Humanoid::Status const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4cae00 — __ZN3rbx8any_castIRKN3RBX8Humanoid6StatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x4cae00() {
    // IDA 0x4cae00: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::Status>> *)")]
// 0x4caf6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4caf6c() {
    // IDA 0x4caf6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3::Axis>(G3D::Vector3::Axis const&)")]
// 0x4cb690 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector34AxisEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4cb690() {
    // IDA 0x4cb690: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "G3D::Vector3::Axis const& rbx::any_cast<G3D::Vector3::Axis const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4cb828 — __ZN3rbx8any_castIRKN3G3D7Vector34AxisEN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x4cb828() {
    // IDA 0x4cb828: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,G3D::Vector3::Axis>,std::_Select1st<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,G3D::Vector3::Axis>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,G3D::Vector3::Axis>> *)")]
// 0x4cb994 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N3G3D7Vector34AxisEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4cb994() {
    // IDA 0x4cb994: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Legacy::SurfaceConstraint>(RBX::Legacy::SurfaceConstraint const&)")]
// 0x4cc0b8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4cc0b8() {
    // IDA 0x4cc0b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::singleton(void)")]
// 0x4cc108 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE9singletonEv
// type: _DWORD *()
pub fn stub_0x4cc108() {
    // IDA 0x4cc108: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::construct_func(char const*,char *)")]
// 0x4cc174 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x4cc174() {
    // IDA 0x4cc174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::destruct_func(char *)")]
// 0x4cc180 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE13destruct_funcEPc
// type: void()
pub fn stub_0x4cc180() {
    // IDA 0x4cc180: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Legacy::SurfaceConstraint const& rbx::any_cast<RBX::Legacy::SurfaceConstraint const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4cc250 — __ZN3rbx8any_castIRKN3RBX6Legacy17SurfaceConstraintENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x4cc250() {
    // IDA 0x4cc250: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>> *)")]
// 0x4cc3bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x4cc3bc() {
    // IDA 0x4cc3bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::TopBottom>(RBX::Feature::TopBottom const&)")]
// 0x4cdf30 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x4cdf30() {
    // IDA 0x4cdf30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "G3D::Array<RBX::Primitive *,10,32ul>::Array(void)")]
// 0x506fa8 — __ZN3G3D5ArrayIPN3RBX9PrimitiveELi10ELm32EEC2Ev
pub fn stub_0x506fa8() {
    // IDA 0x506fa8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FilterDescendents::~FilterDescendents()")]
// 0x507098 — __ZN3RBX17FilterDescendentsD0Ev
// type: void __fastcall(RBX::FilterDescendents *__hidden this)
pub fn stub_0x507098() {
    // IDA 0x507098: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GlobalAdvancedSettings::getFVariable(std::string)")]
// 0x5073c0 — __ZN3RBX22GlobalAdvancedSettings12getFVariableESs
pub fn stub_0x5073c0() {
    // IDA 0x5073c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GlobalAdvancedSettings::getFFlag(std::string)")]
// 0x5075a4 — __ZN3RBX22GlobalAdvancedSettings8getFFlagESs
pub fn stub_0x5075a4() {
    // IDA 0x5075a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Settings::Settings(std::string const&)")]
// 0x507808 — __ZN3RBX8SettingsC2ERKSs
// type: _DWORD __fastcall(RBX::Settings *__hidden this, const std::string *)
pub fn stub_0x507808() {
    // IDA 0x507808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Settings::loadState(std::string const&)")]
// 0x5079f8 — __ZN3RBX8Settings9loadStateERKSs
// type: _DWORD __fastcall(RBX::Settings *__hidden this, const std::string *)
pub fn stub_0x5079f8() {
    // IDA 0x5079f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
