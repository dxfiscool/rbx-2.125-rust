//! core shard BK — 100 core stubs EA-sorted, next uncovered after BJ 0x4bf170 (strict RBX|boost|std|rbx earliest gap, after BJ 0x4bf1dc..0x4cdf30).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x4bf170.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::construct_func(char const*,char *)")]
// 0x4bf1dc — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings12VideoQualityEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::construct_func(char const*,char *)
pub fn stub_4bf1dc() {
    // IDA 0x4bf1dc: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::destruct_func(char *)")]
// 0x4bf1e8 — __ZN3rbx14implementation12typed_holderIN3RBX12GameSettings12VideoQualityEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GameSettings::VideoQuality>::destruct_func(char *)
pub fn stub_4bf1e8() {
    // IDA 0x4bf1e8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::GameSettings::VideoQuality const& rbx::any_cast<RBX::GameSettings::VideoQuality const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bf2b8 — __ZN3rbx8any_castIRKN3RBX12GameSettings12VideoQualityENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GameSettings::VideoQuality const& rbx::any_cast<RBX::GameSettings::VideoQuality const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bf2b8() {
    // IDA 0x4bf2b8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>> *)")]
// 0x4bf424 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>> *)
pub fn stub_4bf424() {
    // IDA 0x4bf424: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)")]
// 0x4bfb48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13CharacterMesh8BodyPartEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)
pub fn stub_4bfb48() {
    // IDA 0x4bfb48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)")]
// 0x4bfb98 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE9singletonEv — rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)
pub fn stub_4bfb98() {
    // IDA 0x4bfb98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)")]
// 0x4bfc04 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)
pub fn stub_4bfc04() {
    // IDA 0x4bfc04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)")]
// 0x4bfc10 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)
pub fn stub_4bfc10() {
    // IDA 0x4bfc10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4bfce0 — __ZN3rbx8any_castIRKN3RBX13CharacterMesh8BodyPartENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bfce0() {
    // IDA 0x4bfce0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)")]
// 0x4bfe4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)
pub fn stub_4bfe4c() {
    // IDA 0x4bfe4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::construct_func(char const*,char *)")]
// 0x4bffc0 — __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::construct_func(char const*,char *)
pub fn stub_4bffc0() {
    // IDA 0x4bffc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)")]
// 0x4c0810 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)
pub fn stub_4c0810() {
    // IDA 0x4c0810: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)")]
// 0x4c0860 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE9singletonEv — rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)
pub fn stub_4c0860() {
    // IDA 0x4c0860: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)")]
// 0x4c08cc — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)
pub fn stub_4c08cc() {
    // IDA 0x4c08cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)")]
// 0x4c08d8 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)
pub fn stub_4c08d8() {
    // IDA 0x4c08d8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c09a8 — __ZN3rbx8any_castIRKN3RBX11ChatService9ChatColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c09a8() {
    // IDA 0x4c09a8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)")]
// 0x4c0b14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)
pub fn stub_4c0b14() {
    // IDA 0x4c0b14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)")]
// 0x4c1420 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)
pub fn stub_4c1420() {
    // IDA 0x4c1420: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)")]
// 0x4c1470 — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)
pub fn stub_4c1470() {
    // IDA 0x4c1470: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::construct_func(char const*,char *)")]
// 0x4c14dc — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::SurfaceType>::construct_func(char const*,char *)
pub fn stub_4c14dc() {
    // IDA 0x4c14dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::destruct_func(char *)")]
// 0x4c14e8 — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::SurfaceType>::destruct_func(char *)
pub fn stub_4c14e8() {
    // IDA 0x4c14e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SurfaceType const& rbx::any_cast<RBX::SurfaceType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c15b8 — __ZN3rbx8any_castIRKN3RBX11SurfaceTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::SurfaceType const& rbx::any_cast<RBX::SurfaceType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c15b8() {
    // IDA 0x4c15b8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)")]
// 0x4c1724 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)
pub fn stub_4c1724() {
    // IDA 0x4c1724: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)")]
// 0x4c1e48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12SpecialShape8MeshTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)
pub fn stub_4c1e48() {
    // IDA 0x4c1e48: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::singleton(void)")]
// 0x4c1e98 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::singleton(void)
pub fn stub_4c1e98() {
    // IDA 0x4c1e98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::construct_func(char const*,char *)")]
// 0x4c1f04 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::construct_func(char const*,char *)
pub fn stub_4c1f04() {
    // IDA 0x4c1f04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::destruct_func(char *)")]
// 0x4c1f10 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::destruct_func(char *)
pub fn stub_4c1f10() {
    // IDA 0x4c1f10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SpecialShape::MeshType const& rbx::any_cast<RBX::SpecialShape::MeshType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c1fe0 — __ZN3rbx8any_castIRKN3RBX12SpecialShape8MeshTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::SpecialShape::MeshType const& rbx::any_cast<RBX::SpecialShape::MeshType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c1fe0() {
    // IDA 0x4c1fe0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)")]
// 0x4c214c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)
pub fn stub_4c214c() {
    // IDA 0x4c214c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&)")]
// 0x4c3298 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18SkateboardPlatform9MoveStateEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SkateboardPlatform::MoveState>(RBX::SkateboardPlatform::MoveState const&)
pub fn stub_4c3298() {
    // IDA 0x4c3298: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::singleton(void)")]
// 0x4c32e8 — __ZN3rbx14implementation12typed_holderIN3RBX18SkateboardPlatform9MoveStateEE9singletonEv — rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::singleton(void)
pub fn stub_4c32e8() {
    // IDA 0x4c32e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::construct_func(char const*,char *)")]
// 0x4c3354 — __ZN3rbx14implementation12typed_holderIN3RBX18SkateboardPlatform9MoveStateEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::construct_func(char const*,char *)
pub fn stub_4c3354() {
    // IDA 0x4c3354: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::destruct_func(char *)")]
// 0x4c3360 — __ZN3rbx14implementation12typed_holderIN3RBX18SkateboardPlatform9MoveStateEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::SkateboardPlatform::MoveState>::destruct_func(char *)
pub fn stub_4c3360() {
    // IDA 0x4c3360: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SkateboardPlatform::MoveState const& rbx::any_cast<RBX::SkateboardPlatform::MoveState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c3430 — __ZN3rbx8any_castIRKN3RBX18SkateboardPlatform9MoveStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::SkateboardPlatform::MoveState const& rbx::any_cast<RBX::SkateboardPlatform::MoveState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c3430() {
    // IDA 0x4c3430: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>> *)")]
// 0x4c359c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_18SkateboardPlatform9MoveStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SkateboardPlatform::MoveState>> *)
pub fn stub_4c359c() {
    // IDA 0x4c359c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Handles::VisualStyle>(RBX::Handles::VisualStyle const&)")]
// 0x4c3cc0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Handles11VisualStyleEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Handles::VisualStyle>(RBX::Handles::VisualStyle const&)
pub fn stub_4c3cc0() {
    // IDA 0x4c3cc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::singleton(void)")]
// 0x4c3d10 — __ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE9singletonEv — rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::singleton(void)
pub fn stub_4c3d10() {
    // IDA 0x4c3d10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::construct_func(char const*,char *)")]
// 0x4c3d7c — __ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::construct_func(char const*,char *)
pub fn stub_4c3d7c() {
    // IDA 0x4c3d7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::destruct_func(char *)")]
// 0x4c3d88 — __ZN3rbx14implementation12typed_holderIN3RBX7Handles11VisualStyleEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Handles::VisualStyle>::destruct_func(char *)
pub fn stub_4c3d88() {
    // IDA 0x4c3d88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Handles::VisualStyle const& rbx::any_cast<RBX::Handles::VisualStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c3e58 — __ZN3rbx8any_castIRKN3RBX7Handles11VisualStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Handles::VisualStyle const& rbx::any_cast<RBX::Handles::VisualStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c3e58() {
    // IDA 0x4c3e58: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>> *)")]
// 0x4c3fc4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>> *)
pub fn stub_4c3fc4() {
    // IDA 0x4c3fc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendEventType>(RBX::FriendService::FriendEventType const&)")]
// 0x4c46e8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendEventType>(RBX::FriendService::FriendEventType const&)
pub fn stub_4c46e8() {
    // IDA 0x4c46e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::singleton(void)")]
// 0x4c4738 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::singleton(void)
pub fn stub_4c4738() {
    // IDA 0x4c4738: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FriendService::FriendEventType const& rbx::any_cast<RBX::FriendService::FriendEventType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c4870 — __ZN3rbx8any_castIRKN3RBX13FriendService15FriendEventTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::FriendService::FriendEventType const& rbx::any_cast<RBX::FriendService::FriendEventType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c4870() {
    // IDA 0x4c4870: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>> *)")]
// 0x4c49dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>> *)
pub fn stub_4c49dc() {
    // IDA 0x4c49dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendStatus>(RBX::FriendService::FriendStatus const&)")]
// 0x4c5100 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendStatus>(RBX::FriendService::FriendStatus const&)
pub fn stub_4c5100() {
    // IDA 0x4c5100: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::singleton(void)")]
// 0x4c5150 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE9singletonEv — rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::singleton(void)
pub fn stub_4c5150() {
    // IDA 0x4c5150: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::construct_func(char const*,char *)")]
// 0x4c51bc — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::construct_func(char const*,char *)
pub fn stub_4c51bc() {
    // IDA 0x4c51bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::destruct_func(char *)")]
// 0x4c51c8 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::destruct_func(char *)
pub fn stub_4c51c8() {
    // IDA 0x4c51c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::FriendService::FriendStatus const& rbx::any_cast<RBX::FriendService::FriendStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c5298 — __ZN3rbx8any_castIRKN3RBX13FriendService12FriendStatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::FriendService::FriendStatus const& rbx::any_cast<RBX::FriendService::FriendStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c5298() {
    // IDA 0x4c5298: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>> *)")]
// 0x4c5404 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>> *)
pub fn stub_4c5404() {
    // IDA 0x4c5404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PersonalServerService::PrivilegeType>(RBX::PersonalServerService::PrivilegeType const&)")]
// 0x4c79a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_21PersonalServerService13PrivilegeTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PersonalServerService::PrivilegeType>(RBX::PersonalServerService::PrivilegeType const&)
pub fn stub_4c79a0() {
    // IDA 0x4c79a0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::singleton(void)")]
// 0x4c79f0 — __ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::singleton(void)
pub fn stub_4c79f0() {
    // IDA 0x4c79f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::construct_func(char const*,char *)")]
// 0x4c7a5c — __ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::construct_func(char const*,char *)
pub fn stub_4c7a5c() {
    // IDA 0x4c7a5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::destruct_func(char *)")]
// 0x4c7a68 — __ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::destruct_func(char *)
pub fn stub_4c7a68() {
    // IDA 0x4c7a68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::PersonalServerService::PrivilegeType const& rbx::any_cast<RBX::PersonalServerService::PrivilegeType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c7b38 — __ZN3rbx8any_castIRKN3RBX21PersonalServerService13PrivilegeTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::PersonalServerService::PrivilegeType const& rbx::any_cast<RBX::PersonalServerService::PrivilegeType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c7b38() {
    // IDA 0x4c7b38: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>> *)")]
// 0x4c7ca4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>> *)
pub fn stub_4c7ca4() {
    // IDA 0x4c7ca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SocialService::StuffType>(RBX::SocialService::StuffType const&)")]
// 0x4c83c8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SocialService9StuffTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SocialService::StuffType>(RBX::SocialService::StuffType const&)
pub fn stub_4c83c8() {
    // IDA 0x4c83c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SocialService::StuffType>::singleton(void)")]
// 0x4c8418 — __ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::SocialService::StuffType>::singleton(void)
pub fn stub_4c8418() {
    // IDA 0x4c8418: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SocialService::StuffType>::construct_func(char const*,char *)")]
// 0x4c8484 — __ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::SocialService::StuffType>::construct_func(char const*,char *)
pub fn stub_4c8484() {
    // IDA 0x4c8484: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::SocialService::StuffType>::destruct_func(char *)")]
// 0x4c8490 — __ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::SocialService::StuffType>::destruct_func(char *)
pub fn stub_4c8490() {
    // IDA 0x4c8490: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::SocialService::StuffType const& rbx::any_cast<RBX::SocialService::StuffType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c8560 — __ZN3rbx8any_castIRKN3RBX13SocialService9StuffTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::SocialService::StuffType const& rbx::any_cast<RBX::SocialService::StuffType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c8560() {
    // IDA 0x4c8560: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>> *)")]
// 0x4c86cc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>> *)
pub fn stub_4c86cc() {
    // IDA 0x4c86cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeyframeSequence::Priority>(RBX::KeyframeSequence::Priority const&)")]
// 0x4c8df0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16KeyframeSequence8PriorityEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeyframeSequence::Priority>(RBX::KeyframeSequence::Priority const&)
pub fn stub_4c8df0() {
    // IDA 0x4c8df0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::singleton(void)")]
// 0x4c8e40 — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE9singletonEv — rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::singleton(void)
pub fn stub_4c8e40() {
    // IDA 0x4c8e40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::construct_func(char const*,char *)")]
// 0x4c8eac — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::construct_func(char const*,char *)
pub fn stub_4c8eac() {
    // IDA 0x4c8eac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::destruct_func(char *)")]
// 0x4c8eb8 — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::destruct_func(char *)
pub fn stub_4c8eb8() {
    // IDA 0x4c8eb8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::KeyframeSequence::Priority const& rbx::any_cast<RBX::KeyframeSequence::Priority const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4c8f88 — __ZN3rbx8any_castIRKN3RBX16KeyframeSequence8PriorityENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::KeyframeSequence::Priority const& rbx::any_cast<RBX::KeyframeSequence::Priority const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c8f88() {
    // IDA 0x4c8f88: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>> *)")]
// 0x4c90f4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>> *)
pub fn stub_4c90f4() {
    // IDA 0x4c90f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::NameOcclusion>(RBX::Humanoid::NameOcclusion const&)")]
// 0x4ca240 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid13NameOcclusionEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::NameOcclusion>(RBX::Humanoid::NameOcclusion const&)
pub fn stub_4ca240() {
    // IDA 0x4ca240: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::singleton(void)")]
// 0x4ca290 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE9singletonEv — rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::singleton(void)
pub fn stub_4ca290() {
    // IDA 0x4ca290: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::construct_func(char const*,char *)")]
// 0x4ca2fc — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::construct_func(char const*,char *)
pub fn stub_4ca2fc() {
    // IDA 0x4ca2fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::destruct_func(char *)")]
// 0x4ca308 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::destruct_func(char *)
pub fn stub_4ca308() {
    // IDA 0x4ca308: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Humanoid::NameOcclusion const& rbx::any_cast<RBX::Humanoid::NameOcclusion const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4ca3d8 — __ZN3rbx8any_castIRKN3RBX8Humanoid13NameOcclusionENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Humanoid::NameOcclusion const& rbx::any_cast<RBX::Humanoid::NameOcclusion const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4ca3d8() {
    // IDA 0x4ca3d8: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>> *)")]
// 0x4ca544 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>> *)
pub fn stub_4ca544() {
    // IDA 0x4ca544: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::Status>(RBX::Humanoid::Status const&)")]
// 0x4cac68 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid6StatusEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::Status>(RBX::Humanoid::Status const&)
pub fn stub_4cac68() {
    // IDA 0x4cac68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::singleton(void)")]
// 0x4cacb8 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE9singletonEv — rbx::implementation::typed_holder<RBX::Humanoid::Status>::singleton(void)
pub fn stub_4cacb8() {
    // IDA 0x4cacb8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::construct_func(char const*,char *)")]
// 0x4cad24 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Humanoid::Status>::construct_func(char const*,char *)
pub fn stub_4cad24() {
    // IDA 0x4cad24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::destruct_func(char *)")]
// 0x4cad30 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Humanoid::Status>::destruct_func(char *)
pub fn stub_4cad30() {
    // IDA 0x4cad30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Humanoid::Status const& rbx::any_cast<RBX::Humanoid::Status const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4cae00 — __ZN3rbx8any_castIRKN3RBX8Humanoid6StatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Humanoid::Status const& rbx::any_cast<RBX::Humanoid::Status const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4cae00() {
    // IDA 0x4cae00: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::Status>> *)")]
// 0x4caf6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::Status>> *)
pub fn stub_4caf6c() {
    // IDA 0x4caf6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Legacy::SurfaceConstraint>(RBX::Legacy::SurfaceConstraint const&)")]
// 0x4cc0b8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Legacy::SurfaceConstraint>(RBX::Legacy::SurfaceConstraint const&)
pub fn stub_4cc0b8() {
    // IDA 0x4cc0b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::singleton(void)")]
// 0x4cc108 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE9singletonEv — rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::singleton(void)
pub fn stub_4cc108() {
    // IDA 0x4cc108: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::construct_func(char const*,char *)")]
// 0x4cc174 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::construct_func(char const*,char *)
pub fn stub_4cc174() {
    // IDA 0x4cc174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::destruct_func(char *)")]
// 0x4cc180 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::destruct_func(char *)
pub fn stub_4cc180() {
    // IDA 0x4cc180: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Legacy::SurfaceConstraint const& rbx::any_cast<RBX::Legacy::SurfaceConstraint const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4cc250 — __ZN3rbx8any_castIRKN3RBX6Legacy17SurfaceConstraintENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Legacy::SurfaceConstraint const& rbx::any_cast<RBX::Legacy::SurfaceConstraint const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4cc250() {
    // IDA 0x4cc250: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>> *)")]
// 0x4cc3bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>> *)
pub fn stub_4cc3bc() {
    // IDA 0x4cc3bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeywordFilterType>(RBX::KeywordFilterType const&)")]
// 0x4ccae0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17KeywordFilterTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeywordFilterType>(RBX::KeywordFilterType const&)
pub fn stub_4ccae0() {
    // IDA 0x4ccae0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeywordFilterType>::singleton(void)")]
// 0x4ccb30 — __ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::KeywordFilterType>::singleton(void)
pub fn stub_4ccb30() {
    // IDA 0x4ccb30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeywordFilterType>::construct_func(char const*,char *)")]
// 0x4ccb9c — __ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::KeywordFilterType>::construct_func(char const*,char *)
pub fn stub_4ccb9c() {
    // IDA 0x4ccb9c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::KeywordFilterType>::destruct_func(char *)")]
// 0x4ccba8 — __ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::KeywordFilterType>::destruct_func(char *)
pub fn stub_4ccba8() {
    // IDA 0x4ccba8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::KeywordFilterType const& rbx::any_cast<RBX::KeywordFilterType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4ccc78 — __ZN3rbx8any_castIRKN3RBX17KeywordFilterTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::KeywordFilterType const& rbx::any_cast<RBX::KeywordFilterType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4ccc78() {
    // IDA 0x4ccc78: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeywordFilterType>> *)")]
// 0x4ccde4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeywordFilterType>> *)
pub fn stub_4ccde4() {
    // IDA 0x4ccde4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Joint::JointType>(RBX::Joint::JointType const&)")]
// 0x4cd508 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Joint9JointTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Joint::JointType>(RBX::Joint::JointType const&)
pub fn stub_4cd508() {
    // IDA 0x4cd508: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Joint::JointType>::singleton(void)")]
// 0x4cd558 — __ZN3rbx14implementation12typed_holderIN3RBX5Joint9JointTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::Joint::JointType>::singleton(void)
pub fn stub_4cd558() {
    // IDA 0x4cd558: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Joint::JointType>::construct_func(char const*,char *)")]
// 0x4cd5c4 — __ZN3rbx14implementation12typed_holderIN3RBX5Joint9JointTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Joint::JointType>::construct_func(char const*,char *)
pub fn stub_4cd5c4() {
    // IDA 0x4cd5c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Joint::JointType>::destruct_func(char *)")]
// 0x4cd5d0 — __ZN3rbx14implementation12typed_holderIN3RBX5Joint9JointTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Joint::JointType>::destruct_func(char *)
pub fn stub_4cd5d0() {
    // IDA 0x4cd5d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Joint::JointType const& rbx::any_cast<RBX::Joint::JointType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x4cd6a0 — __ZN3rbx8any_castIRKN3RBX5Joint9JointTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Joint::JointType const& rbx::any_cast<RBX::Joint::JointType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4cd6a0() {
    // IDA 0x4cd6a0: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Joint::JointType>> *)")]
// 0x4cd80c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Joint::JointType>> *)
pub fn stub_4cd80c() {
    // IDA 0x4cd80c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::TopBottom>(RBX::Feature::TopBottom const&)")]
// 0x4cdf30 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::TopBottom>(RBX::Feature::TopBottom const&)
pub fn stub_4cdf30() {
    // IDA 0x4cdf30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
