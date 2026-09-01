//! core shard BL — 100 core stubs EA-sorted, next uncovered after BK 0x4cdf30 (strict RBX|boost|std|rbx earliest gap, after BK 0x4cdf80..0x4e5810).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x4cdf30.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::singleton(void)")]
// 0x4cdf80 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE9singletonEv — rbx::implementation::typed_holder<RBX::Feature::TopBottom>::singleton(void)
pub fn stub_4cdf80() -> ! {
    todo!("0x4cdf80 __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::construct_func(char const*,char *)")]
// 0x4cdfec — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Feature::TopBottom>::construct_func(char const*,char *)
pub fn stub_4cdfec() -> ! {
    todo!("0x4cdfec __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::destruct_func(char *)")]
// 0x4cdff8 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Feature::TopBottom>::destruct_func(char *)
pub fn stub_4cdff8() -> ! {
    todo!("0x4cdff8 __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Feature::TopBottom const& rbx::any_cast<RBX::Feature::TopBottom const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4ce0c8 — __ZN3rbx8any_castIRKN3RBX7Feature9TopBottomENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Feature::TopBottom const& rbx::any_cast<RBX::Feature::TopBottom const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4ce0c8() -> ! {
    todo!("0x4ce0c8 __ZN3rbx8any_castIRKN3RBX7Feature9TopBottomENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>> *)")]
// 0x4ce234 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>> *)
pub fn stub_4ce234() -> ! {
    todo!("0x4ce234 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::LeftRight>(RBX::Feature::LeftRight const&)")]
// 0x4ce958 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::LeftRight>(RBX::Feature::LeftRight const&)
pub fn stub_4ce958() -> ! {
    todo!("0x4ce958 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::singleton(void)")]
// 0x4ce9a8 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE9singletonEv — rbx::implementation::typed_holder<RBX::Feature::LeftRight>::singleton(void)
pub fn stub_4ce9a8() -> ! {
    todo!("0x4ce9a8 __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::construct_func(char const*,char *)")]
// 0x4cea14 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Feature::LeftRight>::construct_func(char const*,char *)
pub fn stub_4cea14() -> ! {
    todo!("0x4cea14 __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::destruct_func(char *)")]
// 0x4cea20 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Feature::LeftRight>::destruct_func(char *)
pub fn stub_4cea20() -> ! {
    todo!("0x4cea20 __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Feature::LeftRight const& rbx::any_cast<RBX::Feature::LeftRight const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4ceaf0 — __ZN3rbx8any_castIRKN3RBX7Feature9LeftRightENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Feature::LeftRight const& rbx::any_cast<RBX::Feature::LeftRight const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4ceaf0() -> ! {
    todo!("0x4ceaf0 __ZN3rbx8any_castIRKN3RBX7Feature9LeftRightENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>> *)")]
// 0x4cec5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>> *)
pub fn stub_4cec5c() -> ! {
    todo!("0x4cec5c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::InOut>(RBX::Feature::InOut const&)")]
// 0x4cf380 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature5InOutEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::InOut>(RBX::Feature::InOut const&)
pub fn stub_4cf380() -> ! {
    todo!("0x4cf380 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature5InOutEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::InOut>::singleton(void)")]
// 0x4cf3d0 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE9singletonEv — rbx::implementation::typed_holder<RBX::Feature::InOut>::singleton(void)
pub fn stub_4cf3d0() -> ! {
    todo!("0x4cf3d0 __ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::InOut>::construct_func(char const*,char *)")]
// 0x4cf43c — __ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Feature::InOut>::construct_func(char const*,char *)
pub fn stub_4cf43c() -> ! {
    todo!("0x4cf43c __ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::InOut>::destruct_func(char *)")]
// 0x4cf448 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Feature::InOut>::destruct_func(char *)
pub fn stub_4cf448() -> ! {
    todo!("0x4cf448 __ZN3rbx14implementation12typed_holderIN3RBX7Feature5InOutEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Feature::InOut const& rbx::any_cast<RBX::Feature::InOut const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4cf518 — __ZN3rbx8any_castIRKN3RBX7Feature5InOutENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Feature::InOut const& rbx::any_cast<RBX::Feature::InOut const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4cf518() -> ! {
    todo!("0x4cf518 __ZN3rbx8any_castIRKN3RBX7Feature5InOutENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::InOut>> *)")]
// 0x4cf684 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::InOut>> *)
pub fn stub_4cf684() -> ! {
    todo!("0x4cf684 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature5InOutEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::LegacyController::InputType>(RBX::LegacyController::InputType const&)")]
// 0x4d08c4 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16LegacyController9InputTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::LegacyController::InputType>(RBX::LegacyController::InputType const&)
pub fn stub_4d08c4() -> ! {
    todo!("0x4d08c4 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16LegacyController9InputTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::LegacyController::InputType>::singleton(void)")]
// 0x4d0914 — __ZN3rbx14implementation12typed_holderIN3RBX16LegacyController9InputTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::LegacyController::InputType>::singleton(void)
pub fn stub_4d0914() -> ! {
    todo!("0x4d0914 __ZN3rbx14implementation12typed_holderIN3RBX16LegacyController9InputTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::LegacyController::InputType>::construct_func(char const*,char *)")]
// 0x4d0980 — __ZN3rbx14implementation12typed_holderIN3RBX16LegacyController9InputTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::LegacyController::InputType>::construct_func(char const*,char *)
pub fn stub_4d0980() -> ! {
    todo!("0x4d0980 __ZN3rbx14implementation12typed_holderIN3RBX16LegacyController9InputTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::LegacyController::InputType>::destruct_func(char *)")]
// 0x4d098c — __ZN3rbx14implementation12typed_holderIN3RBX16LegacyController9InputTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::LegacyController::InputType>::destruct_func(char *)
pub fn stub_4d098c() -> ! {
    todo!("0x4d098c __ZN3rbx14implementation12typed_holderIN3RBX16LegacyController9InputTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::LegacyController::InputType const& rbx::any_cast<RBX::LegacyController::InputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d0a5c — __ZN3rbx8any_castIRKN3RBX16LegacyController9InputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::LegacyController::InputType const& rbx::any_cast<RBX::LegacyController::InputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d0a5c() -> ! {
    todo!("0x4d0a5c __ZN3rbx8any_castIRKN3RBX16LegacyController9InputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>> *)")]
// 0x4d0bc8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::LegacyController::InputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::LegacyController::InputType>> *)
pub fn stub_4d0bc8() -> ! {
    todo!("0x4d0bc8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16LegacyController9InputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenEasingDirection>(RBX::GuiObject::TweenEasingDirection const&)")]
// 0x4d13e0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject20TweenEasingDirectionEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenEasingDirection>(RBX::GuiObject::TweenEasingDirection const&)
pub fn stub_4d13e0() -> ! {
    todo!("0x4d13e0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject20TweenEasingDirectionEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::singleton(void)")]
// 0x4d1430 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject20TweenEasingDirectionEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::singleton(void)
pub fn stub_4d1430() -> ! {
    todo!("0x4d1430 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject20TweenEasingDirectionEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::construct_func(char const*,char *)")]
// 0x4d149c — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject20TweenEasingDirectionEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::construct_func(char const*,char *)
pub fn stub_4d149c() -> ! {
    todo!("0x4d149c __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject20TweenEasingDirectionEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::destruct_func(char *)")]
// 0x4d14a8 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject20TweenEasingDirectionEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingDirection>::destruct_func(char *)
pub fn stub_4d14a8() -> ! {
    todo!("0x4d14a8 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject20TweenEasingDirectionEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GuiObject::TweenEasingDirection const& rbx::any_cast<RBX::GuiObject::TweenEasingDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d1578 — __ZN3rbx8any_castIRKN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingDirection const& rbx::any_cast<RBX::GuiObject::TweenEasingDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d1578() -> ! {
    todo!("0x4d1578 __ZN3rbx8any_castIRKN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>> *)")]
// 0x4d16e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>> *)
pub fn stub_4d16e4() -> ! {
    todo!("0x4d16e4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenStatus>(RBX::GuiObject::TweenStatus const&)")]
// 0x4d1e08 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject11TweenStatusEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenStatus>(RBX::GuiObject::TweenStatus const&)
pub fn stub_4d1e08() -> ! {
    todo!("0x4d1e08 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject11TweenStatusEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::singleton(void)")]
// 0x4d1e58 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject11TweenStatusEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::singleton(void)
pub fn stub_4d1e58() -> ! {
    todo!("0x4d1e58 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject11TweenStatusEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::construct_func(char const*,char *)")]
// 0x4d1ec4 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject11TweenStatusEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::construct_func(char const*,char *)
pub fn stub_4d1ec4() -> ! {
    todo!("0x4d1ec4 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject11TweenStatusEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::destruct_func(char *)")]
// 0x4d1ed0 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject11TweenStatusEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiObject::TweenStatus>::destruct_func(char *)
pub fn stub_4d1ed0() -> ! {
    todo!("0x4d1ed0 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject11TweenStatusEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GuiObject::TweenStatus const& rbx::any_cast<RBX::GuiObject::TweenStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d1fa0 — __ZN3rbx8any_castIRKN3RBX9GuiObject11TweenStatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiObject::TweenStatus const& rbx::any_cast<RBX::GuiObject::TweenStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d1fa0() -> ! {
    todo!("0x4d1fa0 __ZN3rbx8any_castIRKN3RBX9GuiObject11TweenStatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>> *)")]
// 0x4d210c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>> *)
pub fn stub_4d210c() -> ! {
    todo!("0x4d210c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenEasingStyle>(RBX::GuiObject::TweenEasingStyle const&)")]
// 0x4d2830 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject16TweenEasingStyleEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::TweenEasingStyle>(RBX::GuiObject::TweenEasingStyle const&)
pub fn stub_4d2830() -> ! {
    todo!("0x4d2830 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject16TweenEasingStyleEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::singleton(void)")]
// 0x4d2880 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject16TweenEasingStyleEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::singleton(void)
pub fn stub_4d2880() -> ! {
    todo!("0x4d2880 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject16TweenEasingStyleEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::construct_func(char const*,char *)")]
// 0x4d28ec — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject16TweenEasingStyleEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::construct_func(char const*,char *)
pub fn stub_4d28ec() -> ! {
    todo!("0x4d28ec __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject16TweenEasingStyleEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::destruct_func(char *)")]
// 0x4d28f8 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject16TweenEasingStyleEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiObject::TweenEasingStyle>::destruct_func(char *)
pub fn stub_4d28f8() -> ! {
    todo!("0x4d28f8 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject16TweenEasingStyleEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GuiObject::TweenEasingStyle const& rbx::any_cast<RBX::GuiObject::TweenEasingStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d29c8 — __ZN3rbx8any_castIRKN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiObject::TweenEasingStyle const& rbx::any_cast<RBX::GuiObject::TweenEasingStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d29c8() -> ! {
    todo!("0x4d29c8 __ZN3rbx8any_castIRKN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>> *)")]
// 0x4d2b34 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>> *)
pub fn stub_4d2b34() -> ! {
    todo!("0x4d2b34 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::SizeConstraint>(RBX::GuiObject::SizeConstraint const&)")]
// 0x4d3258 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject14SizeConstraintEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiObject::SizeConstraint>(RBX::GuiObject::SizeConstraint const&)
pub fn stub_4d3258() -> ! {
    todo!("0x4d3258 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiObject14SizeConstraintEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::singleton(void)")]
// 0x4d32a8 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject14SizeConstraintEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::singleton(void)
pub fn stub_4d32a8() -> ! {
    todo!("0x4d32a8 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject14SizeConstraintEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::construct_func(char const*,char *)")]
// 0x4d3314 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject14SizeConstraintEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::construct_func(char const*,char *)
pub fn stub_4d3314() -> ! {
    todo!("0x4d3314 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject14SizeConstraintEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::destruct_func(char *)")]
// 0x4d3320 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject14SizeConstraintEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiObject::SizeConstraint>::destruct_func(char *)
pub fn stub_4d3320() -> ! {
    todo!("0x4d3320 __ZN3rbx14implementation12typed_holderIN3RBX9GuiObject14SizeConstraintEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GuiObject::SizeConstraint const& rbx::any_cast<RBX::GuiObject::SizeConstraint const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d33f0 — __ZN3rbx8any_castIRKN3RBX9GuiObject14SizeConstraintENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiObject::SizeConstraint const& rbx::any_cast<RBX::GuiObject::SizeConstraint const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d33f0() -> ! {
    todo!("0x4d33f0 __ZN3rbx8any_castIRKN3RBX9GuiObject14SizeConstraintENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>> *)")]
// 0x4d355c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiObject::SizeConstraint>> *)
pub fn stub_4d355c() -> ! {
    todo!("0x4d355c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject14SizeConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HopperBin::BinType>(RBX::HopperBin::BinType const&)")]
// 0x4d3c80 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9HopperBin7BinTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HopperBin::BinType>(RBX::HopperBin::BinType const&)
pub fn stub_4d3c80() -> ! {
    todo!("0x4d3c80 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9HopperBin7BinTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::HopperBin::BinType>::singleton(void)")]
// 0x4d3cd0 — __ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::HopperBin::BinType>::singleton(void)
pub fn stub_4d3cd0() -> ! {
    todo!("0x4d3cd0 __ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::HopperBin::BinType>::construct_func(char const*,char *)")]
// 0x4d3d3c — __ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::HopperBin::BinType>::construct_func(char const*,char *)
pub fn stub_4d3d3c() -> ! {
    todo!("0x4d3d3c __ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::HopperBin::BinType>::destruct_func(char *)")]
// 0x4d3d48 — __ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::HopperBin::BinType>::destruct_func(char *)
pub fn stub_4d3d48() -> ! {
    todo!("0x4d3d48 __ZN3rbx14implementation12typed_holderIN3RBX9HopperBin7BinTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::HopperBin::BinType const& rbx::any_cast<RBX::HopperBin::BinType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d3e18 — __ZN3rbx8any_castIRKN3RBX9HopperBin7BinTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::HopperBin::BinType const& rbx::any_cast<RBX::HopperBin::BinType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d3e18() -> ! {
    todo!("0x4d3e18 __ZN3rbx8any_castIRKN3RBX9HopperBin7BinTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>> *)")]
// 0x4d3f84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HopperBin::BinType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HopperBin::BinType>> *)
pub fn stub_4d3f84() -> ! {
    todo!("0x4d3f84 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9HopperBin7BinTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Action::ActionType>(RBX::Action::ActionType const&)")]
// 0x4d46a8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Action10ActionTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Action::ActionType>(RBX::Action::ActionType const&)
pub fn stub_4d46a8() -> ! {
    todo!("0x4d46a8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Action10ActionTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Action::ActionType>::singleton(void)")]
// 0x4d46f8 — __ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::Action::ActionType>::singleton(void)
pub fn stub_4d46f8() -> ! {
    todo!("0x4d46f8 __ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Action::ActionType>::construct_func(char const*,char *)")]
// 0x4d4764 — __ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Action::ActionType>::construct_func(char const*,char *)
pub fn stub_4d4764() -> ! {
    todo!("0x4d4764 __ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Action::ActionType>::destruct_func(char *)")]
// 0x4d4770 — __ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Action::ActionType>::destruct_func(char *)
pub fn stub_4d4770() -> ! {
    todo!("0x4d4770 __ZN3rbx14implementation12typed_holderIN3RBX6Action10ActionTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Action::ActionType const& rbx::any_cast<RBX::Action::ActionType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d4840 — __ZN3rbx8any_castIRKN3RBX6Action10ActionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Action::ActionType const& rbx::any_cast<RBX::Action::ActionType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d4840() -> ! {
    todo!("0x4d4840 __ZN3rbx8any_castIRKN3RBX6Action10ActionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Action::ActionType>> *)")]
// 0x4d49ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Action::ActionType>> *)
pub fn stub_4d49ac() -> ! {
    todo!("0x4d49ac __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Action10ActionTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FunctionalTest::Result>(RBX::FunctionalTest::Result const&)")]
// 0x4d50d0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_14FunctionalTest6ResultEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FunctionalTest::Result>(RBX::FunctionalTest::Result const&)
pub fn stub_4d50d0() -> ! {
    todo!("0x4d50d0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_14FunctionalTest6ResultEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FunctionalTest::Result>::singleton(void)")]
// 0x4d5120 — __ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE9singletonEv — rbx::implementation::typed_holder<RBX::FunctionalTest::Result>::singleton(void)
pub fn stub_4d5120() -> ! {
    todo!("0x4d5120 __ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FunctionalTest::Result>::construct_func(char const*,char *)")]
// 0x4d518c — __ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::FunctionalTest::Result>::construct_func(char const*,char *)
pub fn stub_4d518c() -> ! {
    todo!("0x4d518c __ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FunctionalTest::Result>::destruct_func(char *)")]
// 0x4d5198 — __ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::FunctionalTest::Result>::destruct_func(char *)
pub fn stub_4d5198() -> ! {
    todo!("0x4d5198 __ZN3rbx14implementation12typed_holderIN3RBX14FunctionalTest6ResultEE13destruct_funcEPc")
}

#[doc(alias = "RBX::FunctionalTest::Result const& rbx::any_cast<RBX::FunctionalTest::Result const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d5268 — __ZN3rbx8any_castIRKN3RBX14FunctionalTest6ResultENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::FunctionalTest::Result const& rbx::any_cast<RBX::FunctionalTest::Result const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d5268() -> ! {
    todo!("0x4d5268 __ZN3rbx8any_castIRKN3RBX14FunctionalTest6ResultENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>> *)")]
// 0x4d53d4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>> *)
pub fn stub_4d53d4() -> ! {
    todo!("0x4d53d4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChangeHistoryService::RuntimeUndoBehavior>(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
// 0x4d57dc — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ChangeHistoryService19RuntimeUndoBehaviorEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChangeHistoryService::RuntimeUndoBehavior>(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)
pub fn stub_4d57dc() -> ! {
    todo!("0x4d57dc __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ChangeHistoryService19RuntimeUndoBehaviorEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::singleton(void)")]
// 0x4d582c — __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE9singletonEv — rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::singleton(void)
pub fn stub_4d582c() -> ! {
    todo!("0x4d582c __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::destruct_func(char *)")]
// 0x4d5898 — __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::destruct_func(char *)
pub fn stub_4d5898() -> ! {
    todo!("0x4d5898 __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE13destruct_funcEPc")
}

#[doc(alias = "RBX::ChangeHistoryService::RuntimeUndoBehavior const& rbx::any_cast<RBX::ChangeHistoryService::RuntimeUndoBehavior const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4d589c — __ZN3rbx8any_castIRKN3RBX20ChangeHistoryService19RuntimeUndoBehaviorENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::ChangeHistoryService::RuntimeUndoBehavior const& rbx::any_cast<RBX::ChangeHistoryService::RuntimeUndoBehavior const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4d589c() -> ! {
    todo!("0x4d589c __ZN3rbx8any_castIRKN3RBX20ChangeHistoryService19RuntimeUndoBehaviorENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "boost::function1<void,std::exception &>::swap(boost::function1<void,std::exception &>&)")]
// 0x4d5a08 — __ZN5boost9function1IvRSt9exceptionE4swapERS3_ — boost::function1<void,std::exception &>::swap(boost::function1<void,std::exception &>&)
pub fn stub_4d5a08() -> ! {
    todo!("0x4d5a08 __ZN5boost9function1IvRSt9exceptionE4swapERS3_")
}

#[doc(alias = "boost::function1<void,std::exception &>::clear(void)")]
// 0x4d5ae4 — __ZN5boost9function1IvRSt9exceptionE5clearEv — boost::function1<void,std::exception &>::clear(void)
pub fn stub_4d5ae4() -> ! {
    todo!("0x4d5ae4 __ZN5boost9function1IvRSt9exceptionE5clearEv")
}

#[doc(alias = "boost::function1<void,std::exception &>::move_assign(boost::function1<void,std::exception &>&)")]
// 0x4d5b10 — __ZN5boost9function1IvRSt9exceptionE11move_assignERS3_ — boost::function1<void,std::exception &>::move_assign(boost::function1<void,std::exception &>&)
pub fn stub_4d5b10() -> ! {
    todo!("0x4d5b10 __ZN5boost9function1IvRSt9exceptionE11move_assignERS3_")
}

#[doc(alias = "boost::detail::function::functor_manager<void (*)(std::exception &)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x4d5c14 — __ZN5boost6detail8function15functor_managerIPFvRSt9exceptionEE6manageERKNS1_15function_bufferERS8_NS1_30functor_manager_operation_typeE — boost::detail::function::functor_manager<void (*)(std::exception &)>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_4d5c14() -> ! {
    todo!("0x4d5c14 __ZN5boost6detail8function15functor_managerIPFvRSt9exceptionEE6manageERKNS1_15function_bufferERS8_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_invoker1<void (*)(std::exception &),void,std::exception &>::invoke(boost::detail::function::function_buffer &,std::exception &)")]
// 0x4d5c70 — __ZN5boost6detail8function22void_function_invoker1IPFvRSt9exceptionEvS4_E6invokeERNS1_15function_bufferES4_ — boost::detail::function::void_function_invoker1<void (*)(std::exception &),void,std::exception &>::invoke(boost::detail::function::function_buffer &,std::exception &)
pub fn stub_4d5c70() -> ! {
    todo!("0x4d5c70 __ZN5boost6detail8function22void_function_invoker1IPFvRSt9exceptionEvS4_E6invokeERNS1_15function_bufferES4_")
}

#[doc(alias = "RBX::VelocityMotor::getHole(void)const")]
// 0x4e46a8 — __ZNK3RBX13VelocityMotor7getHoleEv — RBX::VelocityMotor::getHole(void)const
pub fn stub_4e46a8() -> ! {
    todo!("0x4e46a8 __ZNK3RBX13VelocityMotor7getHoleEv")
}

#[doc(alias = "RBX::VelocityMotor::setHole(RBX::Hole *)")]
// 0x4e46b0 — __ZN3RBX13VelocityMotor7setHoleEPNS_4HoleE — RBX::VelocityMotor::setHole(RBX::Hole *)
pub fn stub_4e46b0() -> ! {
    todo!("0x4e46b0 __ZN3RBX13VelocityMotor7setHoleEPNS_4HoleE")
}

#[doc(alias = "RBX::VelocityMotor::getMaxVelocity(void)const")]
// 0x4e4838 — __ZNK3RBX13VelocityMotor14getMaxVelocityEv — RBX::VelocityMotor::getMaxVelocity(void)const
pub fn stub_4e4838() -> ! {
    todo!("0x4e4838 __ZNK3RBX13VelocityMotor14getMaxVelocityEv")
}

#[doc(alias = "RBX::VelocityMotor::setMaxVelocity(float)")]
// 0x4e4844 — __ZN3RBX13VelocityMotor14setMaxVelocityEf — RBX::VelocityMotor::setMaxVelocity(float)
pub fn stub_4e4844() -> ! {
    todo!("0x4e4844 __ZN3RBX13VelocityMotor14setMaxVelocityEf")
}

#[doc(alias = "RBX::VelocityMotor::getDesiredAngle(void)const")]
// 0x4e4870 — __ZNK3RBX13VelocityMotor15getDesiredAngleEv — RBX::VelocityMotor::getDesiredAngle(void)const
pub fn stub_4e4870() -> ! {
    todo!("0x4e4870 __ZNK3RBX13VelocityMotor15getDesiredAngleEv")
}

#[doc(alias = "RBX::VelocityMotor::setDesiredAngle(float)")]
// 0x4e487c — __ZN3RBX13VelocityMotor15setDesiredAngleEf — RBX::VelocityMotor::setDesiredAngle(float)
pub fn stub_4e487c() -> ! {
    todo!("0x4e487c __ZN3RBX13VelocityMotor15setDesiredAngleEf")
}

#[doc(alias = "RBX::VelocityMotor::getCurrentAngle(void)const")]
// 0x4e48a8 — __ZNK3RBX13VelocityMotor15getCurrentAngleEv — RBX::VelocityMotor::getCurrentAngle(void)const
pub fn stub_4e48a8() -> ! {
    todo!("0x4e48a8 __ZNK3RBX13VelocityMotor15getCurrentAngleEv")
}

#[doc(alias = "RBX::VelocityMotor::setCurrentAngle(float)")]
// 0x4e48b4 — __ZN3RBX13VelocityMotor15setCurrentAngleEf — RBX::VelocityMotor::setCurrentAngle(float)
pub fn stub_4e48b4() -> ! {
    todo!("0x4e48b4 __ZN3RBX13VelocityMotor15setCurrentAngleEf")
}

#[doc(alias = "RBX::Feature::setFaceId(RBX::NormalId)")]
// 0x4e4e88 — __ZN3RBX7Feature9setFaceIdENS_8NormalIdE — RBX::Feature::setFaceId(RBX::NormalId)
pub fn stub_4e4e88() -> ! {
    todo!("0x4e4e88 __ZN3RBX7Feature9setFaceIdENS_8NormalIdE")
}

#[doc(alias = "RBX::Feature::setTopBottom(RBX::Feature::TopBottom)")]
// 0x4e4ea4 — __ZN3RBX7Feature12setTopBottomENS0_9TopBottomE — RBX::Feature::setTopBottom(RBX::Feature::TopBottom)
pub fn stub_4e4ea4() -> ! {
    todo!("0x4e4ea4 __ZN3RBX7Feature12setTopBottomENS0_9TopBottomE")
}

#[doc(alias = "RBX::Feature::setLeftRight(RBX::Feature::LeftRight)")]
// 0x4e4ec0 — __ZN3RBX7Feature12setLeftRightENS0_9LeftRightE — RBX::Feature::setLeftRight(RBX::Feature::LeftRight)
pub fn stub_4e4ec0() -> ! {
    todo!("0x4e4ec0 __ZN3RBX7Feature12setLeftRightENS0_9LeftRightE")
}

#[doc(alias = "RBX::Feature::setInOut(RBX::Feature::InOut)")]
// 0x4e4edc — __ZN3RBX7Feature8setInOutENS0_5InOutE — RBX::Feature::setInOut(RBX::Feature::InOut)
pub fn stub_4e4edc() -> ! {
    todo!("0x4e4edc __ZN3RBX7Feature8setInOutENS0_5InOutE")
}

#[doc(alias = "RBX::Feature::Feature(void)")]
// 0x4e4efc — __ZN3RBX7FeatureC2Ev — RBX::Feature::Feature(void)
pub fn stub_4e4efc() -> ! {
    todo!("0x4e4efc __ZN3RBX7FeatureC2Ev")
}

#[doc(alias = "RBX::Feature::~Feature()")]
// 0x4e5138 — __ZN3RBX7FeatureD0Ev — RBX::Feature::~Feature()
pub fn stub_4e5138() -> ! {
    todo!("0x4e5138 __ZN3RBX7FeatureD0Ev")
}

#[doc(alias = "RBX::Feature::~Feature()")]
// 0x4e51d8 — __ZN3RBX7FeatureD1Ev — RBX::Feature::~Feature()
pub fn stub_4e51d8() -> ! {
    todo!("0x4e51d8 __ZN3RBX7FeatureD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Feature::~Feature()")]
// 0x4e51dc — __ZThn32_N3RBX7FeatureD0Ev — non-virtual thunk toRBX::Feature::~Feature()
pub fn stub_4e51dc() -> ! {
    todo!("0x4e51dc __ZThn32_N3RBX7FeatureD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Feature::~Feature()")]
// 0x4e51e4 — __ZThn36_N3RBX7FeatureD0Ev — non-virtual thunk toRBX::Feature::~Feature()
pub fn stub_4e51e4() -> ! {
    todo!("0x4e51e4 __ZThn36_N3RBX7FeatureD0Ev")
}

#[doc(alias = "RBX::Feature::~Feature()")]
// 0x4e51ec — __ZN3RBX7FeatureD2Ev — RBX::Feature::~Feature()
pub fn stub_4e51ec() -> ! {
    todo!("0x4e51ec __ZN3RBX7FeatureD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Feature::~Feature()")]
// 0x4e52a8 — __ZThn32_N3RBX7FeatureD1Ev — non-virtual thunk toRBX::Feature::~Feature()
pub fn stub_4e52a8() -> ! {
    todo!("0x4e52a8 __ZThn32_N3RBX7FeatureD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Feature::~Feature()")]
// 0x4e52b0 — __ZThn36_N3RBX7FeatureD1Ev — non-virtual thunk toRBX::Feature::~Feature()
pub fn stub_4e52b0() -> ! {
    todo!("0x4e52b0 __ZThn36_N3RBX7FeatureD1Ev")
}

#[doc(alias = "RBX::Feature::computeLocalCoordinateFrame(void)const")]
// 0x4e5344 — __ZNK3RBX7Feature27computeLocalCoordinateFrameEv — RBX::Feature::computeLocalCoordinateFrame(void)const
pub fn stub_4e5344() -> ! {
    todo!("0x4e5344 __ZNK3RBX7Feature27computeLocalCoordinateFrameEv")
}

#[doc(alias = "RBX::Feature::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// 0x4e555c — __ZN3RBX7Feature14render3dSelectEPNS_5AdornENS_11SelectStateE — RBX::Feature::render3dSelect(RBX::Adorn *,RBX::SelectState)
pub fn stub_4e555c() -> ! {
    todo!("0x4e555c __ZN3RBX7Feature14render3dSelectEPNS_5AdornENS_11SelectStateE")
}

#[doc(alias = "non-virtual thunk toRBX::Feature::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
// 0x4e55ac — __ZThn92_N3RBX7Feature14render3dSelectEPNS_5AdornENS_11SelectStateE — non-virtual thunk toRBX::Feature::render3dSelect(RBX::Adorn *,RBX::SelectState)
pub fn stub_4e55ac() -> ! {
    todo!("0x4e55ac __ZThn92_N3RBX7Feature14render3dSelectEPNS_5AdornENS_11SelectStateE")
}

#[doc(alias = "RBX::Hole::Hole(void)")]
// 0x4e55b4 — __ZN3RBX4HoleC2Ev — RBX::Hole::Hole(void)
pub fn stub_4e55b4() -> ! {
    todo!("0x4e55b4 __ZN3RBX4HoleC2Ev")
}

#[doc(alias = "RBX::Hole::render3dAdorn(RBX::Adorn *)")]
// 0x4e57b4 — __ZN3RBX4Hole13render3dAdornEPNS_5AdornE — RBX::Hole::render3dAdorn(RBX::Adorn *)
pub fn stub_4e57b4() -> ! {
    todo!("0x4e57b4 __ZN3RBX4Hole13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::Hole::render3dAdorn(RBX::Adorn *)")]
// 0x4e5810 — __ZThn92_N3RBX4Hole13render3dAdornEPNS_5AdornE — non-virtual thunk toRBX::Hole::render3dAdorn(RBX::Adorn *)
pub fn stub_4e5810() -> ! {
    todo!("0x4e5810 __ZThn92_N3RBX4Hole13render3dAdornEPNS_5AdornE")
}

