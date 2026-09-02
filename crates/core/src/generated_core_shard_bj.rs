//! core shard BJ — 100 core stubs EA-sorted, next uncovered after BI 0x4b48e8 (strict RBX|boost|std|rbx earliest gap, after BI 0x4b4954..0x4bf170).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x4b48e8.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::construct_func(char const*,char *)")]
// 0x4b4954 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::construct_func(char const*,char *)
pub fn stub_4b4954() -> ! {
    todo!("0x4b4954 __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::destruct_func(char *)")]
// 0x4b4960 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::destruct_func(char *)
pub fn stub_4b4960() -> ! {
    todo!("0x4b4960 __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::HttpService::HttpContentType const& rbx::any_cast<RBX::HttpService::HttpContentType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b4a30 — __ZN3rbx8any_castIRKN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::HttpService::HttpContentType const& rbx::any_cast<RBX::HttpService::HttpContentType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b4a30() -> ! {
    todo!("0x4b4a30 __ZN3rbx8any_castIRKN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>> *)")]
// 0x4b4b9c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>> *)
pub fn stub_4b4b9c() -> ! {
    todo!("0x4b4b9c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)")]
// 0x4b52c0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)
pub fn stub_4b52c0() -> ! {
    todo!("0x4b52c0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void)")]
// 0x4b5310 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void)
pub fn stub_4b5310() -> ! {
    todo!("0x4b5310 __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char const*,char *)")]
// 0x4b537c — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char const*,char *)
pub fn stub_4b537c() -> ! {
    todo!("0x4b537c __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char *)")]
// 0x4b5388 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char *)
pub fn stub_4b5388() -> ! {
    todo!("0x4b5388 __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::AssetService::AccessType const& rbx::any_cast<RBX::AssetService::AccessType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b5458 — __ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::AssetService::AccessType const& rbx::any_cast<RBX::AssetService::AccessType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b5458() -> ! {
    todo!("0x4b5458 __ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *)")]
// 0x4b55c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *)
pub fn stub_4b55c4() -> ! {
    todo!("0x4b55c4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&)")]
// 0x4b5ce8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&)
pub fn stub_4b5ce8() -> ! {
    todo!("0x4b5ce8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void)")]
// 0x4b5d38 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void)
pub fn stub_4b5d38() -> ! {
    todo!("0x4b5d38 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char const*,char *)")]
// 0x4b5da4 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char const*,char *)
pub fn stub_4b5da4() -> ! {
    todo!("0x4b5da4 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char *)")]
// 0x4b5db0 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char *)
pub fn stub_4b5db0() -> ! {
    todo!("0x4b5db0 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc")
}

#[doc(alias = "RBX::InputObject::UserInputState const& rbx::any_cast<RBX::InputObject::UserInputState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b5e80 — __ZN3rbx8any_castIRKN3RBX11InputObject14UserInputStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::InputObject::UserInputState const& rbx::any_cast<RBX::InputObject::UserInputState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b5e80() -> ! {
    todo!("0x4b5e80 __ZN3rbx8any_castIRKN3RBX11InputObject14UserInputStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>> *)")]
// 0x4b5fec — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>> *)
pub fn stub_4b5fec() -> ! {
    todo!("0x4b5fec __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputType>(RBX::InputObject::UserInputType const&)")]
// 0x4b6710 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputType>(RBX::InputObject::UserInputType const&)
pub fn stub_4b6710() -> ! {
    todo!("0x4b6710 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::singleton(void)")]
// 0x4b6760 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::singleton(void)
pub fn stub_4b6760() -> ! {
    todo!("0x4b6760 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::construct_func(char const*,char *)")]
// 0x4b67cc — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::construct_func(char const*,char *)
pub fn stub_4b67cc() -> ! {
    todo!("0x4b67cc __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::destruct_func(char *)")]
// 0x4b67d8 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::destruct_func(char *)
pub fn stub_4b67d8() -> ! {
    todo!("0x4b67d8 __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::InputObject::UserInputType const& rbx::any_cast<RBX::InputObject::UserInputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b68a8 — __ZN3rbx8any_castIRKN3RBX11InputObject13UserInputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::InputObject::UserInputType const& rbx::any_cast<RBX::InputObject::UserInputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b68a8() -> ! {
    todo!("0x4b68a8 __ZN3rbx8any_castIRKN3RBX11InputObject13UserInputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>> *)")]
// 0x4b6a14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>> *)
pub fn stub_4b6a14() -> ! {
    todo!("0x4b6a14 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Explosion::ExplosionType>(RBX::Explosion::ExplosionType const&)")]
// 0x4b6e18 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Explosion::ExplosionType>(RBX::Explosion::ExplosionType const&)
pub fn stub_4b6e18() -> ! {
    todo!("0x4b6e18 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::singleton(void)")]
// 0x4b6e68 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::singleton(void)
pub fn stub_4b6e68() -> ! {
    todo!("0x4b6e68 __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::destruct_func(char *)")]
// 0x4b6ed4 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::destruct_func(char *)
pub fn stub_4b6ed4() -> ! {
    todo!("0x4b6ed4 __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Explosion::ExplosionType const& rbx::any_cast<RBX::Explosion::ExplosionType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b6ed8 — __ZN3rbx8any_castIRKN3RBX9Explosion13ExplosionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Explosion::ExplosionType const& rbx::any_cast<RBX::Explosion::ExplosionType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b6ed8() -> ! {
    todo!("0x4b6ed8 __ZN3rbx8any_castIRKN3RBX9Explosion13ExplosionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellDirection>(RBX::Voxel::WaterCellDirection const&)")]
// 0x4b7740 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellDirection>(RBX::Voxel::WaterCellDirection const&)
pub fn stub_4b7740() -> ! {
    todo!("0x4b7740 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::singleton(void)")]
// 0x4b7790 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::singleton(void)
pub fn stub_4b7790() -> ! {
    todo!("0x4b7790 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::construct_func(char const*,char *)")]
// 0x4b77fc — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::construct_func(char const*,char *)
pub fn stub_4b77fc() -> ! {
    todo!("0x4b77fc __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::destruct_func(char *)")]
// 0x4b7808 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::destruct_func(char *)
pub fn stub_4b7808() -> ! {
    todo!("0x4b7808 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Voxel::WaterCellDirection const& rbx::any_cast<RBX::Voxel::WaterCellDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b78d8 — __ZN3rbx8any_castIRKN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::WaterCellDirection const& rbx::any_cast<RBX::Voxel::WaterCellDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b78d8() -> ! {
    todo!("0x4b78d8 __ZN3rbx8any_castIRKN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>> *)")]
// 0x4b7a44 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>> *)
pub fn stub_4b7a44() -> ! {
    todo!("0x4b7a44 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellForce>(RBX::Voxel::WaterCellForce const&)")]
// 0x4b8168 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellForce>(RBX::Voxel::WaterCellForce const&)
pub fn stub_4b8168() -> ! {
    todo!("0x4b8168 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::singleton(void)")]
// 0x4b81b8 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::singleton(void)
pub fn stub_4b81b8() -> ! {
    todo!("0x4b81b8 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::construct_func(char const*,char *)")]
// 0x4b8224 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::construct_func(char const*,char *)
pub fn stub_4b8224() -> ! {
    todo!("0x4b8224 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::destruct_func(char *)")]
// 0x4b8230 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::destruct_func(char *)
pub fn stub_4b8230() -> ! {
    todo!("0x4b8230 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Voxel::WaterCellForce const& rbx::any_cast<RBX::Voxel::WaterCellForce const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b8300 — __ZN3rbx8any_castIRKN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::WaterCellForce const& rbx::any_cast<RBX::Voxel::WaterCellForce const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b8300() -> ! {
    todo!("0x4b8300 __ZN3rbx8any_castIRKN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>> *)")]
// 0x4b846c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>> *)
pub fn stub_4b846c() -> ! {
    todo!("0x4b846c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)")]
// 0x4b8b90 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)
pub fn stub_4b8b90() -> ! {
    todo!("0x4b8b90 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void)")]
// 0x4b8be0 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void)
pub fn stub_4b8be0() -> ! {
    todo!("0x4b8be0 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::construct_func(char const*,char *)")]
// 0x4b8c4c — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::construct_func(char const*,char *)
pub fn stub_4b8c4c() -> ! {
    todo!("0x4b8c4c __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::destruct_func(char *)")]
// 0x4b8c58 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::destruct_func(char *)
pub fn stub_4b8c58() -> ! {
    todo!("0x4b8c58 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Voxel::CellOrientation const& rbx::any_cast<RBX::Voxel::CellOrientation const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b8d28 — __ZN3rbx8any_castIRKN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellOrientation const& rbx::any_cast<RBX::Voxel::CellOrientation const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b8d28() -> ! {
    todo!("0x4b8d28 __ZN3rbx8any_castIRKN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *)")]
// 0x4b8e94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *)
pub fn stub_4b8e94() -> ! {
    todo!("0x4b8e94 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)")]
// 0x4b95b8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)
pub fn stub_4b95b8() -> ! {
    todo!("0x4b95b8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void)")]
// 0x4b9608 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void)
pub fn stub_4b9608() -> ! {
    todo!("0x4b9608 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::construct_func(char const*,char *)")]
// 0x4b9674 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::construct_func(char const*,char *)
pub fn stub_4b9674() -> ! {
    todo!("0x4b9674 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)")]
// 0x4b9680 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)
pub fn stub_4b9680() -> ! {
    todo!("0x4b9680 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4b9750 — __ZN3rbx8any_castIRKN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b9750() -> ! {
    todo!("0x4b9750 __ZN3rbx8any_castIRKN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)")]
// 0x4b98bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)
pub fn stub_4b98bc() -> ! {
    todo!("0x4b98bc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)")]
// 0x4b9fe0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)
pub fn stub_4b9fe0() -> ! {
    todo!("0x4b9fe0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)")]
// 0x4ba030 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)
pub fn stub_4ba030() -> ! {
    todo!("0x4ba030 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)")]
// 0x4ba09c — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)
pub fn stub_4ba09c() -> ! {
    todo!("0x4ba09c __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)")]
// 0x4ba0a8 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)
pub fn stub_4ba0a8() -> ! {
    todo!("0x4ba0a8 __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4ba178 — __ZN3rbx8any_castIRKN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4ba178() -> ! {
    todo!("0x4ba178 __ZN3rbx8any_castIRKN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)")]
// 0x4ba2e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)
pub fn stub_4ba2e4() -> ! {
    todo!("0x4ba2e4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)")]
// 0x4baa08 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)
pub fn stub_4baa08() -> ! {
    todo!("0x4baa08 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)")]
// 0x4baa58 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)
pub fn stub_4baa58() -> ! {
    todo!("0x4baa58 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)")]
// 0x4baac4 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)
pub fn stub_4baac4() -> ! {
    todo!("0x4baac4 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)")]
// 0x4baad0 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)
pub fn stub_4baad0() -> ! {
    todo!("0x4baad0 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE13destruct_funcEPc")
}

#[doc(alias = "RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4baba0 — __ZN3rbx8any_castIRKN3RBX10DialogRoot10DialogToneENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4baba0() -> ! {
    todo!("0x4baba0 __ZN3rbx8any_castIRKN3RBX10DialogRoot10DialogToneENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)")]
// 0x4bad0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)
pub fn stub_4bad0c() -> ! {
    todo!("0x4bad0c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)")]
// 0x4bb430 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)
pub fn stub_4bb430() -> ! {
    todo!("0x4bb430 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)")]
// 0x4bb480 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)
pub fn stub_4bb480() -> ! {
    todo!("0x4bb480 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)")]
// 0x4bb4ec — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)
pub fn stub_4bb4ec() -> ! {
    todo!("0x4bb4ec __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)")]
// 0x4bb4f8 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)
pub fn stub_4bb4f8() -> ! {
    todo!("0x4bb4f8 __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bb5c8 — __ZN3rbx8any_castIRKN3RBX10DialogRoot13DialogPurposeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bb5c8() -> ! {
    todo!("0x4bb5c8 __ZN3rbx8any_castIRKN3RBX10DialogRoot13DialogPurposeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)")]
// 0x4bb734 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)
pub fn stub_4bb734() -> ! {
    todo!("0x4bb734 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)")]
// 0x4bbe58 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)
pub fn stub_4bbe58() -> ! {
    todo!("0x4bbe58 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)")]
// 0x4bbea8 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)
pub fn stub_4bbea8() -> ! {
    todo!("0x4bbea8 __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)")]
// 0x4bbf14 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)
pub fn stub_4bbf14() -> ! {
    todo!("0x4bbf14 __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)")]
// 0x4bbf20 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)
pub fn stub_4bbf20() -> ! {
    todo!("0x4bbf20 __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bbff0 — __ZN3rbx8any_castIRKN3RBX9GuiButton5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bbff0() -> ! {
    todo!("0x4bbff0 __ZN3rbx8any_castIRKN3RBX9GuiButton5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)")]
// 0x4bc15c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)
pub fn stub_4bc15c() -> ! {
    todo!("0x4bc15c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Frame::Style>(RBX::Frame::Style const&)")]
// 0x4bc880 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Frame::Style>(RBX::Frame::Style const&)
pub fn stub_4bc880() -> ! {
    todo!("0x4bc880 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::singleton(void)")]
// 0x4bc8d0 — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv — rbx::implementation::typed_holder<RBX::Frame::Style>::singleton(void)
pub fn stub_4bc8d0() -> ! {
    todo!("0x4bc8d0 __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::construct_func(char const*,char *)")]
// 0x4bc93c — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Frame::Style>::construct_func(char const*,char *)
pub fn stub_4bc93c() -> ! {
    todo!("0x4bc93c __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::destruct_func(char *)")]
// 0x4bc948 — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Frame::Style>::destruct_func(char *)
pub fn stub_4bc948() -> ! {
    todo!("0x4bc948 __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE13destruct_funcEPc")
}

#[doc(alias = "RBX::Frame::Style const& rbx::any_cast<RBX::Frame::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bca18 — __ZN3rbx8any_castIRKN3RBX5Frame5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Frame::Style const& rbx::any_cast<RBX::Frame::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bca18() -> ! {
    todo!("0x4bca18 __ZN3rbx8any_castIRKN3RBX5Frame5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Frame::Style>> *)")]
// 0x4bcb84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Frame::Style>> *)
pub fn stub_4bcb84() -> ! {
    todo!("0x4bcb84 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// 0x4bd2a8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings20RenderQualitySettingEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)
pub fn stub_4bd2a8() -> ! {
    todo!("0x4bd2a8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings20RenderQualitySettingEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::singleton(void)")]
// 0x4bd2f8 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE9singletonEv — rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::singleton(void)
pub fn stub_4bd2f8() -> ! {
    todo!("0x4bd2f8 __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::construct_func(char const*,char *)")]
// 0x4bd364 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::construct_func(char const*,char *)
pub fn stub_4bd364() -> ! {
    todo!("0x4bd364 __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::destruct_func(char *)")]
// 0x4bd370 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::destruct_func(char *)
pub fn stub_4bd370() -> ! {
    todo!("0x4bd370 __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GameBasicSettings::RenderQualitySetting const& rbx::any_cast<RBX::GameBasicSettings::RenderQualitySetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bd440 — __ZN3rbx8any_castIRKN3RBX17GameBasicSettings20RenderQualitySettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GameBasicSettings::RenderQualitySetting const& rbx::any_cast<RBX::GameBasicSettings::RenderQualitySetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bd440() -> ! {
    todo!("0x4bd440 __ZN3rbx8any_castIRKN3RBX17GameBasicSettings20RenderQualitySettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)")]
// 0x4bd5ac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)
pub fn stub_4bd5ac() -> ! {
    todo!("0x4bd5ac __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::ControlMode>(RBX::GameBasicSettings::ControlMode const&)")]
// 0x4bdcd0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings11ControlModeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::ControlMode>(RBX::GameBasicSettings::ControlMode const&)
pub fn stub_4bdcd0() -> ! {
    todo!("0x4bdcd0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings11ControlModeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::singleton(void)")]
// 0x4bdd20 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE9singletonEv — rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::singleton(void)
pub fn stub_4bdd20() -> ! {
    todo!("0x4bdd20 __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::construct_func(char const*,char *)")]
// 0x4bdd8c — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::construct_func(char const*,char *)
pub fn stub_4bdd8c() -> ! {
    todo!("0x4bdd8c __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::destruct_func(char *)")]
// 0x4bdd98 — __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GameBasicSettings::ControlMode>::destruct_func(char *)
pub fn stub_4bdd98() -> ! {
    todo!("0x4bdd98 __ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings11ControlModeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GameBasicSettings::ControlMode const& rbx::any_cast<RBX::GameBasicSettings::ControlMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bde68 — __ZN3rbx8any_castIRKN3RBX17GameBasicSettings11ControlModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GameBasicSettings::ControlMode const& rbx::any_cast<RBX::GameBasicSettings::ControlMode const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bde68() -> ! {
    todo!("0x4bde68 __ZN3rbx8any_castIRKN3RBX17GameBasicSettings11ControlModeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>> *)")]
// 0x4bdfd4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::ControlMode>> *)
pub fn stub_4bdfd4() -> ! {
    todo!("0x4bdfd4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings11ControlModeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::UploadSetting>(RBX::GameSettings::UploadSetting const&)")]
// 0x4be6f8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings13UploadSettingEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::UploadSetting>(RBX::GameSettings::UploadSetting const&)
pub fn stub_4be6f8() -> ! {
    todo!("0x4be6f8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings13UploadSettingEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::singleton(void)")]
// 0x4be748 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE9singletonEv — rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::singleton(void)
pub fn stub_4be748() -> ! {
    todo!("0x4be748 __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::construct_func(char const*,char *)")]
// 0x4be7b4 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::construct_func(char const*,char *)
pub fn stub_4be7b4() -> ! {
    todo!("0x4be7b4 __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::destruct_func(char *)")]
// 0x4be7c0 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GameSettings::UploadSetting>::destruct_func(char *)
pub fn stub_4be7c0() -> ! {
    todo!("0x4be7c0 __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings13UploadSettingEE13destruct_funcEPc")
}

#[doc(alias = "RBX::GameSettings::UploadSetting const& rbx::any_cast<RBX::GameSettings::UploadSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4be890 — __ZN3rbx8any_castIRKN3RBX12GameSettings13UploadSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GameSettings::UploadSetting const& rbx::any_cast<RBX::GameSettings::UploadSetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4be890() -> ! {
    todo!("0x4be890 __ZN3rbx8any_castIRKN3RBX12GameSettings13UploadSettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>> *)")]
// 0x4be9fc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>> *)
pub fn stub_4be9fc() -> ! {
    todo!("0x4be9fc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::VideoQuality>(RBX::GameSettings::VideoQuality const&)")]
// 0x4bf120 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings12VideoQualityEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameSettings::VideoQuality>(RBX::GameSettings::VideoQuality const&)
pub fn stub_4bf120() -> ! {
    todo!("0x4bf120 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12GameSettings12VideoQualityEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::singleton(void)")]
// 0x4bf170 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings12VideoQualityEE9singletonEv — rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::singleton(void)
pub fn stub_4bf170() -> ! {
    todo!("0x4bf170 __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings12VideoQualityEE9singletonEv")
}
