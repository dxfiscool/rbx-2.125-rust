//! rendering shard 352 — 100 stubs 0x4bc184..0x851008 EA-sorted asc filtered+gap Ogre|G3D|Gfx|Render|Adorn remaining + gap filler not yet in rbx_rendering
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 total, 15585->15586 filtered stubbed after batch (remaining 0), distinct 38340->38440
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc filtered+gap not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4bc184 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::initSingleton(void)
pub fn stub_4bc184() -> ! {
    todo!("0x4bc184 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::initSingleton(void)")
}

// 0x4bfb48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13CharacterMesh8BodyPartEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)
pub fn stub_4bfb48() -> ! {
    todo!("0x4bfb48 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)")
}

// 0x4bfb98 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)
pub fn stub_4bfb98() -> ! {
    todo!("0x4bfb98 rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)")
}

// 0x4bfc04 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)
pub fn stub_4bfc04() -> ! {
    todo!("0x4bfc04 rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)")
}

// 0x4bfc10 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)
pub fn stub_4bfc10() -> ! {
    todo!("0x4bfc10 rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)")
}

// 0x4bfc14 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToItem(RBX::CharacterMesh::BodyPart const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToItem(RBX::CharacterMesh::BodyPart const&)const
pub fn stub_4bfc14() -> ! {
    todo!("0x4bfc14 RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToItem(RBX::CharacterMesh::BodyPart const&)const")
}

// 0x4bfce0 — __ZN3rbx8any_castIRKN3RBX13CharacterMesh8BodyPartENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bfce0() -> ! {
    todo!("0x4bfce0 RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4bfdd0 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToValue(RBX::Name const&,RBX::CharacterMesh::BodyPart&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToValue(RBX::Name const&,RBX::CharacterMesh::BodyPart&)const
pub fn stub_4bfdd0() -> ! {
    todo!("0x4bfdd0 RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToValue(RBX::Name const&,RBX::CharacterMesh::BodyPart&)const")
}

// 0x4bfe4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)
pub fn stub_4bfe4c() -> ! {
    todo!("0x4bfe4c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)")
}

// 0x4bfe74 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18MarketplaceService12CurrencyTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType> const>::initSingleton(void)
pub fn stub_4bfe74() -> ! {
    todo!("0x4bfe74 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType> const>::initSingleton(void)")
}

// 0x4bfe78 — __ZN3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::~EnumDesc()
pub fn stub_4bfe78() -> ! {
    todo!("0x4bfe78 RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::~EnumDesc()")
}

// 0x4bfe7c — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToString(unsigned long,std::string &)const
pub fn stub_4bfe7c() -> ! {
    todo!("0x4bfe7c RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToString(unsigned long,std::string &)const")
}

// 0x4bffc0 — __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::construct_func(char const*,char *)
pub fn stub_4bffc0() -> ! {
    todo!("0x4bffc0 rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::construct_func(char const*,char *)")
}

// 0x4bffcc — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToItem(RBX::MarketplaceService::CurrencyType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToItem(RBX::MarketplaceService::CurrencyType const&)const
pub fn stub_4bffcc() -> ! {
    todo!("0x4bffcc RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToItem(RBX::MarketplaceService::CurrencyType const&)const")
}

// 0x4c0098 — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToValue(RBX::Name const&,RBX::MarketplaceService::CurrencyType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToValue(RBX::Name const&,RBX::MarketplaceService::CurrencyType&)const
pub fn stub_4c0098() -> ! {
    todo!("0x4c0098 RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToValue(RBX::Name const&,RBX::MarketplaceService::CurrencyType&)const")
}

// 0x4c0114 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::initSingleton(void)
pub fn stub_4c0114() -> ! {
    todo!("0x4c0114 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::initSingleton(void)")
}

// 0x4c0118 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::doGetSingleton(void)
pub fn stub_4c0118() -> ! {
    todo!("0x4c0118 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::doGetSingleton(void)")
}

// 0x4c0208 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()
pub fn stub_4c0208() -> ! {
    todo!("0x4c0208 RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")
}

// 0x4c020c — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()
pub fn stub_4c020c() -> ! {
    todo!("0x4c020c RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")
}

// 0x4c03e0 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()
pub fn stub_4c03e0() -> ! {
    todo!("0x4c03e0 RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")
}

// 0x4c0480 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(char const*)const
pub fn stub_4c0480() -> ! {
    todo!("0x4c0480 RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(char const*)const")
}

// 0x4c04b0 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4c04b0() -> ! {
    todo!("0x4c04b0 RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4c04d0 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4c04d0() -> ! {
    todo!("0x4c04d0 RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4c052c — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(unsigned long,std::string &)const
pub fn stub_4c052c() -> ! {
    todo!("0x4c052c RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(unsigned long,std::string &)const")
}

// 0x4c0670 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(RBX::ChatService::ChatColor const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(RBX::ChatService::ChatColor const&)const
pub fn stub_4c0670() -> ! {
    todo!("0x4c0670 RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(RBX::ChatService::ChatColor const&)const")
}

// 0x4c0810 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)
pub fn stub_4c0810() -> ! {
    todo!("0x4c0810 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)")
}

// 0x4c0860 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)
pub fn stub_4c0860() -> ! {
    todo!("0x4c0860 rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)")
}

// 0x4c08cc — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)
pub fn stub_4c08cc() -> ! {
    todo!("0x4c08cc rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)")
}

// 0x4c08d8 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)
pub fn stub_4c08d8() -> ! {
    todo!("0x4c08d8 rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)")
}

// 0x4c08dc — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToItem(RBX::ChatService::ChatColor const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToItem(RBX::ChatService::ChatColor const&)const
pub fn stub_4c08dc() -> ! {
    todo!("0x4c08dc RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToItem(RBX::ChatService::ChatColor const&)const")
}

// 0x4c09a8 — __ZN3rbx8any_castIRKN3RBX11ChatService9ChatColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c09a8() -> ! {
    todo!("0x4c09a8 RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4c0a98 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(RBX::Name const&,RBX::ChatService::ChatColor&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(RBX::Name const&,RBX::ChatService::ChatColor&)const
pub fn stub_4c0a98() -> ! {
    todo!("0x4c0a98 RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(RBX::Name const&,RBX::ChatService::ChatColor&)const")
}

// 0x4c0b14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)
pub fn stub_4c0b14() -> ! {
    todo!("0x4c0b14 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)")
}

// 0x4c0b3c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::initSingleton(void)
pub fn stub_4c0b3c() -> ! {
    todo!("0x4c0b3c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::initSingleton(void)")
}

// 0x4c0b40 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::doGetSingleton(void)
pub fn stub_4c0b40() -> ! {
    todo!("0x4c0b40 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::doGetSingleton(void)")
}

// 0x4c0c30 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::initSingleton(void)
pub fn stub_4c0c30() -> ! {
    todo!("0x4c0c30 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::initSingleton(void)")
}

// 0x4c0c34 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::doGetSingleton(void)
pub fn stub_4c0c34() -> ! {
    todo!("0x4c0c34 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::doGetSingleton(void)")
}

// 0x4c0d24 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::initSingleton(void)
pub fn stub_4c0d24() -> ! {
    todo!("0x4c0d24 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::initSingleton(void)")
}

// 0x4c0d28 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::doGetSingleton(void)
pub fn stub_4c0d28() -> ! {
    todo!("0x4c0d28 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::doGetSingleton(void)")
}

// 0x4c0e18 — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()
pub fn stub_4c0e18() -> ! {
    todo!("0x4c0e18 RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")
}

// 0x4c0e1c — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()
pub fn stub_4c0e1c() -> ! {
    todo!("0x4c0e1c RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")
}

// 0x4c0ff0 — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()
pub fn stub_4c0ff0() -> ! {
    todo!("0x4c0ff0 RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")
}

// 0x4c1090 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(char const*)const
pub fn stub_4c1090() -> ! {
    todo!("0x4c1090 RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(char const*)const")
}

// 0x4c10c0 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4c10c0() -> ! {
    todo!("0x4c10c0 RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4c10e0 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4c10e0() -> ! {
    todo!("0x4c10e0 RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4c113c — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(unsigned long,std::string &)const
pub fn stub_4c113c() -> ! {
    todo!("0x4c113c RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(unsigned long,std::string &)const")
}

// 0x4c1280 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(RBX::SurfaceType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(RBX::SurfaceType const&)const
pub fn stub_4c1280() -> ! {
    todo!("0x4c1280 RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(RBX::SurfaceType const&)const")
}

// 0x4c1420 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)
pub fn stub_4c1420() -> ! {
    todo!("0x4c1420 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)")
}

// 0x4c1470 — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)
pub fn stub_4c1470() -> ! {
    todo!("0x4c1470 rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)")
}

// 0x4c14dc — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::SurfaceType>::construct_func(char const*,char *)
pub fn stub_4c14dc() -> ! {
    todo!("0x4c14dc rbx::implementation::typed_holder<RBX::SurfaceType>::construct_func(char const*,char *)")
}

// 0x4c14e8 — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::SurfaceType>::destruct_func(char *)
pub fn stub_4c14e8() -> ! {
    todo!("0x4c14e8 rbx::implementation::typed_holder<RBX::SurfaceType>::destruct_func(char *)")
}

// 0x4c14ec — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToItem(RBX::SurfaceType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToItem(RBX::SurfaceType const&)const
pub fn stub_4c14ec() -> ! {
    todo!("0x4c14ec RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToItem(RBX::SurfaceType const&)const")
}

// 0x4c15b8 — __ZN3rbx8any_castIRKN3RBX11SurfaceTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SurfaceType const& rbx::any_cast<RBX::SurfaceType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::SurfaceType const& rbx::any_cast<RBX::SurfaceType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c15b8() -> ! {
    todo!("0x4c15b8 RBX::SurfaceType const& rbx::any_cast<RBX::SurfaceType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4c16a8 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(RBX::Name const&,RBX::SurfaceType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(RBX::Name const&,RBX::SurfaceType&)const
pub fn stub_4c16a8() -> ! {
    todo!("0x4c16a8 RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(RBX::Name const&,RBX::SurfaceType&)const")
}

// 0x4c1724 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)
pub fn stub_4c1724() -> ! {
    todo!("0x4c1724 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)")
}

// 0x4c174c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::initSingleton(void)
pub fn stub_4c174c() -> ! {
    todo!("0x4c174c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::initSingleton(void)")
}

// 0x4c1750 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::doGetSingleton(void)
pub fn stub_4c1750() -> ! {
    todo!("0x4c1750 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::doGetSingleton(void)")
}

// 0x4c1840 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()
pub fn stub_4c1840() -> ! {
    todo!("0x4c1840 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")
}

// 0x4c1844 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()
pub fn stub_4c1844() -> ! {
    todo!("0x4c1844 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")
}

// 0x4c1a18 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()
pub fn stub_4c1a18() -> ! {
    todo!("0x4c1a18 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")
}

// 0x4c1ab8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(char const*)const
pub fn stub_4c1ab8() -> ! {
    todo!("0x4c1ab8 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(char const*)const")
}

// 0x4c1ae8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4c1ae8() -> ! {
    todo!("0x4c1ae8 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4c1b08 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4c1b08() -> ! {
    todo!("0x4c1b08 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4c1b64 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(unsigned long,std::string &)const
pub fn stub_4c1b64() -> ! {
    todo!("0x4c1b64 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(unsigned long,std::string &)const")
}

// 0x4c1ca8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(RBX::SpecialShape::MeshType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(RBX::SpecialShape::MeshType const&)const
pub fn stub_4c1ca8() -> ! {
    todo!("0x4c1ca8 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(RBX::SpecialShape::MeshType const&)const")
}

// 0x4c1e48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12SpecialShape8MeshTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)
pub fn stub_4c1e48() -> ! {
    todo!("0x4c1e48 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)")
}

// 0x4c1e98 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::singleton(void)
pub fn stub_4c1e98() -> ! {
    todo!("0x4c1e98 rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::singleton(void)")
}

// 0x4c1f04 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::construct_func(char const*,char *)
pub fn stub_4c1f04() -> ! {
    todo!("0x4c1f04 rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::construct_func(char const*,char *)")
}

// 0x4c1f10 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::destruct_func(char *)
pub fn stub_4c1f10() -> ! {
    todo!("0x4c1f10 rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::destruct_func(char *)")
}

// 0x4c1f14 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToItem(RBX::SpecialShape::MeshType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToItem(RBX::SpecialShape::MeshType const&)const
pub fn stub_4c1f14() -> ! {
    todo!("0x4c1f14 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToItem(RBX::SpecialShape::MeshType const&)const")
}

// 0x4c1fe0 — __ZN3rbx8any_castIRKN3RBX12SpecialShape8MeshTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SpecialShape::MeshType const& rbx::any_cast<RBX::SpecialShape::MeshType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::SpecialShape::MeshType const& rbx::any_cast<RBX::SpecialShape::MeshType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c1fe0() -> ! {
    todo!("0x4c1fe0 RBX::SpecialShape::MeshType const& rbx::any_cast<RBX::SpecialShape::MeshType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4c20d0 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(RBX::Name const&,RBX::SpecialShape::MeshType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(RBX::Name const&,RBX::SpecialShape::MeshType&)const
pub fn stub_4c20d0() -> ! {
    todo!("0x4c20d0 RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(RBX::Name const&,RBX::SpecialShape::MeshType&)const")
}

// 0x4c214c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)
pub fn stub_4c214c() -> ! {
    todo!("0x4c214c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)")
}

// 0x4c2174 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)
pub fn stub_4c2174() -> ! {
    todo!("0x4c2174 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)")
}

// 0x4c2178 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)
pub fn stub_4c2178() -> ! {
    todo!("0x4c2178 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")
}

// 0x4c2268 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()
pub fn stub_4c2268() -> ! {
    todo!("0x4c2268 RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")
}

// 0x4c226c — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()
pub fn stub_4c226c() -> ! {
    todo!("0x4c226c RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")
}

// 0x4c2440 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()
pub fn stub_4c2440() -> ! {
    todo!("0x4c2440 RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")
}

// 0x4c24e0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const
pub fn stub_4c24e0() -> ! {
    todo!("0x4c24e0 RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const")
}

// 0x4c2510 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4c2510() -> ! {
    todo!("0x4c2510 RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4c2530 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4c2530() -> ! {
    todo!("0x4c2530 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4c258c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const
pub fn stub_4c258c() -> ! {
    todo!("0x4c258c RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const")
}

// 0x4c26d0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const
pub fn stub_4c26d0() -> ! {
    todo!("0x4c26d0 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")
}

// 0x4c2870 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9SoundTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)
pub fn stub_4c2870() -> ! {
    todo!("0x4c2870 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)")
}

// 0x4c28c0 — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)
pub fn stub_4c28c0() -> ! {
    todo!("0x4c28c0 rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)")
}

// 0x4c292c — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::SoundType>::construct_func(char const*,char *)
pub fn stub_4c292c() -> ! {
    todo!("0x4c292c rbx::implementation::typed_holder<RBX::SoundType>::construct_func(char const*,char *)")
}

// 0x4c2938 — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::SoundType>::destruct_func(char *)
pub fn stub_4c2938() -> ! {
    todo!("0x4c2938 rbx::implementation::typed_holder<RBX::SoundType>::destruct_func(char *)")
}

// 0x4c293c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const
pub fn stub_4c293c() -> ! {
    todo!("0x4c293c RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")
}

// 0x4c2a08 — __ZN3rbx8any_castIRKN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4c2a08() -> ! {
    todo!("0x4c2a08 RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4c2af8 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const
pub fn stub_4c2af8() -> ! {
    todo!("0x4c2af8 RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")
}

// 0x4c2b74 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)
pub fn stub_4c2b74() -> ! {
    todo!("0x4c2b74 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)")
}

// 0x4c2b9c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::initSingleton(void)
pub fn stub_4c2b9c() -> ! {
    todo!("0x4c2b9c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::initSingleton(void)")
}

// 0x4c2ba0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::doGetSingleton(void)
pub fn stub_4c2ba0() -> ! {
    todo!("0x4c2ba0 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::doGetSingleton(void)")
}

// 0x4c2c90 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()
pub fn stub_4c2c90() -> ! {
    todo!("0x4c2c90 RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")
}

// 0x4c2c94 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()
pub fn stub_4c2c94() -> ! {
    todo!("0x4c2c94 RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")
}

// 0x4c2e68 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED0Ev
// type: void __fastcall(void *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()
pub fn stub_4c2e68() -> ! {
    todo!("0x4c2e68 RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")
}

// 0x4c2f08 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(char const*)const
pub fn stub_4c2f08() -> ! {
    todo!("0x4c2f08 RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(char const*)const")
}

// 0x4c2f38 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4c2f38() -> ! {
    todo!("0x4c2f38 RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4c2f58 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4c2f58() -> ! {
    todo!("0x4c2f58 RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x851008 — __ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_851008() -> ! {
    todo!("0x851008 RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}
