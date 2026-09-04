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
// IDA 0x4bc184: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4bc184() {
}

// 0x4bfb48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13CharacterMesh8BodyPartEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CharacterMesh::BodyPart>(RBX::CharacterMesh::BodyPart const&)
// IDA 0x4bfb48: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfb48() {
}

// 0x4bfb98 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::singleton(void)
// IDA 0x4bfb98: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfb98() {
}

// 0x4bfc04 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::construct_func(char const*,char *)
// IDA 0x4bfc04: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfc04() {
}

// 0x4bfc10 — __ZN3rbx14implementation12typed_holderIN3RBX13CharacterMesh8BodyPartEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::CharacterMesh::BodyPart>::destruct_func(char *)
// IDA 0x4bfc10: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4bfc10() {
}

// 0x4bfc14 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToItem(RBX::CharacterMesh::BodyPart const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToItem(RBX::CharacterMesh::BodyPart const&)const
// IDA 0x4bfc14: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfc14() {
}

// 0x4bfce0 — __ZN3rbx8any_castIRKN3RBX13CharacterMesh8BodyPartENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::CharacterMesh::BodyPart const& rbx::any_cast<RBX::CharacterMesh::BodyPart const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4bfce0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfce0() {
}

// 0x4bfdd0 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToValue(RBX::Name const&,RBX::CharacterMesh::BodyPart&)const")]
// was: RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToValue(RBX::Name const&,RBX::CharacterMesh::BodyPart&)const
// IDA 0x4bfdd0: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfdd0() {
}

// 0x4bfe4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>> *)
// IDA 0x4bfe4c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfe4c() {
}

// 0x4bfe74 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18MarketplaceService12CurrencyTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType> const>::initSingleton(void)
// IDA 0x4bfe74: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4bfe74() {
}

// 0x4bfe78 — __ZN3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::~EnumDesc()
// IDA 0x4bfe78: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4bfe78() {
}

// 0x4bfe7c — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToString(unsigned long,std::string &)const
// IDA 0x4bfe7c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bfe7c() {
}

// 0x4bffc0 — __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::construct_func(char const*,char *)
// IDA 0x4bffc0: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bffc0() {
}

// 0x4bffcc — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToItem(RBX::MarketplaceService::CurrencyType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToItem(RBX::MarketplaceService::CurrencyType const&)const
// IDA 0x4bffcc: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4bffcc() {
}

// 0x4c0098 — __ZNK3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToValue(RBX::Name const&,RBX::MarketplaceService::CurrencyType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::MarketplaceService::CurrencyType>::convertToValue(RBX::Name const&,RBX::MarketplaceService::CurrencyType&)const
// IDA 0x4c0098: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0098() {
}

// 0x4c0114 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::initSingleton(void)
// IDA 0x4c0114: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c0114() {
}

// 0x4c0118 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor> const>::doGetSingleton(void)
// IDA 0x4c0118: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0118() {
}

// 0x4c0208 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()
// IDA 0x4c0208: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c0208() {
}

// 0x4c020c — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()
// IDA 0x4c020c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c020c() {
}

// 0x4c03e0 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::~EnumDesc()
// IDA 0x4c03e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c03e0() {
}

// 0x4c0480 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(char const*)const
// IDA 0x4c0480: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0480() {
}

// 0x4c04b0 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c04b0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c04b0() {
}

// 0x4c04d0 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c04d0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c04d0() {
}

// 0x4c052c — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(unsigned long,std::string &)const
// IDA 0x4c052c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c052c() {
}

// 0x4c0670 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(RBX::ChatService::ChatColor const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToString(RBX::ChatService::ChatColor const&)const
// IDA 0x4c0670: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0670() {
}

// 0x4c0810 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11ChatService9ChatColorEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ChatService::ChatColor>(RBX::ChatService::ChatColor const&)
// IDA 0x4c0810: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0810() {
}

// 0x4c0860 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::singleton(void)
// IDA 0x4c0860: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0860() {
}

// 0x4c08cc — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::construct_func(char const*,char *)
// IDA 0x4c08cc: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c08cc() {
}

// 0x4c08d8 — __ZN3rbx14implementation12typed_holderIN3RBX11ChatService9ChatColorEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::ChatService::ChatColor>::destruct_func(char *)
// IDA 0x4c08d8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c08d8() {
}

// 0x4c08dc — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToItem(RBX::ChatService::ChatColor const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToItem(RBX::ChatService::ChatColor const&)const
// IDA 0x4c08dc: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c08dc() {
}

// 0x4c09a8 — __ZN3rbx8any_castIRKN3RBX11ChatService9ChatColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::ChatService::ChatColor const& rbx::any_cast<RBX::ChatService::ChatColor const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c09a8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c09a8() {
}

// 0x4c0a98 — __ZNK3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(RBX::Name const&,RBX::ChatService::ChatColor&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::convertToValue(RBX::Name const&,RBX::ChatService::ChatColor&)const
// IDA 0x4c0a98: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0a98() {
}

// 0x4c0b14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>> *)
// IDA 0x4c0b14: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0b14() {
}

// 0x4c0b3c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::initSingleton(void)
// IDA 0x4c0b3c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c0b3c() {
}

// 0x4c0b40 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::UserInputService::SwipeDirection> const>::doGetSingleton(void)
// IDA 0x4c0b40: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0b40() {
}

// 0x4c0c30 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::initSingleton(void)
// IDA 0x4c0c30: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c0c30() {
}

// 0x4c0c34 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PartInstance::FormFactor> const>::doGetSingleton(void)
// IDA 0x4c0c34: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0c34() {
}

// 0x4c0d24 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::initSingleton(void)
// IDA 0x4c0d24: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c0d24() {
}

// 0x4c0d28 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SurfaceType> const>::doGetSingleton(void)
// IDA 0x4c0d28: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c0d28() {
}

// 0x4c0e18 — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()
// IDA 0x4c0e18: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c0e18() {
}

// 0x4c0e1c — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()
// IDA 0x4c0e1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c0e1c() {
}

// 0x4c0ff0 — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::~EnumDesc()
// IDA 0x4c0ff0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c0ff0() {
}

// 0x4c1090 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(char const*)const
// IDA 0x4c1090: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1090() {
}

// 0x4c10c0 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c10c0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c10c0() {
}

// 0x4c10e0 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c10e0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c10e0() {
}

// 0x4c113c — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(unsigned long,std::string &)const
// IDA 0x4c113c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c113c() {
}

// 0x4c1280 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE15convertToStringERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(RBX::SurfaceType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToString(RBX::SurfaceType const&)const
// IDA 0x4c1280: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1280() {
}

// 0x4c1420 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11SurfaceTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SurfaceType>(RBX::SurfaceType const&)
// IDA 0x4c1420: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1420() {
}

// 0x4c1470 — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::SurfaceType>::singleton(void)
// IDA 0x4c1470: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1470() {
}

// 0x4c14dc — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::SurfaceType>::construct_func(char const*,char *)
// IDA 0x4c14dc: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c14dc() {
}

// 0x4c14e8 — __ZN3rbx14implementation12typed_holderIN3RBX11SurfaceTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::SurfaceType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::SurfaceType>::destruct_func(char *)
// IDA 0x4c14e8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c14e8() {
}

// 0x4c14ec — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToItem(RBX::SurfaceType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToItem(RBX::SurfaceType const&)const
// IDA 0x4c14ec: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c14ec() {
}

// 0x4c15b8 — __ZN3rbx8any_castIRKN3RBX11SurfaceTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SurfaceType const& rbx::any_cast<RBX::SurfaceType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::SurfaceType const& rbx::any_cast<RBX::SurfaceType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c15b8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c15b8() {
}

// 0x4c16a8 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(RBX::Name const&,RBX::SurfaceType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SurfaceType>::convertToValue(RBX::Name const&,RBX::SurfaceType&)const
// IDA 0x4c16a8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c16a8() {
}

// 0x4c1724 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11SurfaceTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SurfaceType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SurfaceType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SurfaceType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SurfaceType>> *)
// IDA 0x4c1724: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1724() {
}

// 0x4c174c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::initSingleton(void)
// IDA 0x4c174c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c174c() {
}

// 0x4c1750 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType> const>::doGetSingleton(void)
// IDA 0x4c1750: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1750() {
}

// 0x4c1840 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()
// IDA 0x4c1840: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c1840() {
}

// 0x4c1844 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()
// IDA 0x4c1844: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c1844() {
}

// 0x4c1a18 — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::~EnumDesc()
// IDA 0x4c1a18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c1a18() {
}

// 0x4c1ab8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(char const*)const
// IDA 0x4c1ab8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1ab8() {
}

// 0x4c1ae8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c1ae8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1ae8() {
}

// 0x4c1b08 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c1b08: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1b08() {
}

// 0x4c1b64 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(unsigned long,std::string &)const
// IDA 0x4c1b64: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1b64() {
}

// 0x4c1ca8 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(RBX::SpecialShape::MeshType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToString(RBX::SpecialShape::MeshType const&)const
// IDA 0x4c1ca8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1ca8() {
}

// 0x4c1e48 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12SpecialShape8MeshTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SpecialShape::MeshType>(RBX::SpecialShape::MeshType const&)
// IDA 0x4c1e48: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1e48() {
}

// 0x4c1e98 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::singleton(void)
// IDA 0x4c1e98: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1e98() {
}

// 0x4c1f04 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::construct_func(char const*,char *)
// IDA 0x4c1f04: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1f04() {
}

// 0x4c1f10 — __ZN3rbx14implementation12typed_holderIN3RBX12SpecialShape8MeshTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::SpecialShape::MeshType>::destruct_func(char *)
// IDA 0x4c1f10: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c1f10() {
}

// 0x4c1f14 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToItem(RBX::SpecialShape::MeshType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToItem(RBX::SpecialShape::MeshType const&)const
// IDA 0x4c1f14: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1f14() {
}

// 0x4c1fe0 — __ZN3rbx8any_castIRKN3RBX12SpecialShape8MeshTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SpecialShape::MeshType const& rbx::any_cast<RBX::SpecialShape::MeshType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::SpecialShape::MeshType const& rbx::any_cast<RBX::SpecialShape::MeshType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c1fe0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c1fe0() {
}

// 0x4c20d0 — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(RBX::Name const&,RBX::SpecialShape::MeshType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SpecialShape::MeshType>::convertToValue(RBX::Name const&,RBX::SpecialShape::MeshType&)const
// IDA 0x4c20d0: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c20d0() {
}

// 0x4c214c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12SpecialShape8MeshTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SpecialShape::MeshType>> *)
// IDA 0x4c214c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c214c() {
}

// 0x4c2174 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::initSingleton(void)
// IDA 0x4c2174: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c2174() {
}

// 0x4c2178 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SoundType> const>::doGetSingleton(void)
// IDA 0x4c2178: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2178() {
}

// 0x4c2268 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()
// IDA 0x4c2268: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c2268() {
}

// 0x4c226c — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()
// IDA 0x4c226c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c226c() {
}

// 0x4c2440 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::~EnumDesc()
// IDA 0x4c2440: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c2440() {
}

// 0x4c24e0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(char const*)const
// IDA 0x4c24e0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c24e0() {
}

// 0x4c2510 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c2510: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2510() {
}

// 0x4c2530 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c2530: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2530() {
}

// 0x4c258c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(unsigned long,std::string &)const
// IDA 0x4c258c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c258c() {
}

// 0x4c26d0 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE15convertToStringERKS2_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToString(RBX::SoundType const&)const
// IDA 0x4c26d0: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c26d0() {
}

// 0x4c2870 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9SoundTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SoundType>(RBX::SoundType const&)
// IDA 0x4c2870: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2870() {
}

// 0x4c28c0 — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::SoundType>::singleton(void)
// IDA 0x4c28c0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c28c0() {
}

// 0x4c292c — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::SoundType>::construct_func(char const*,char *)
// IDA 0x4c292c: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c292c() {
}

// 0x4c2938 — __ZN3rbx14implementation12typed_holderIN3RBX9SoundTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::SoundType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::SoundType>::destruct_func(char *)
// IDA 0x4c2938: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c2938() {
}

// 0x4c293c — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE13convertToItemERKS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToItem(RBX::SoundType const&)const
// IDA 0x4c293c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c293c() {
}

// 0x4c2a08 — __ZN3rbx8any_castIRKN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::SoundType const& rbx::any_cast<RBX::SoundType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c2a08: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2a08() {
}

// 0x4c2af8 — __ZNK3RBX10Reflection8EnumDescINS_9SoundTypeEE14convertToValueERKNS_4NameERS2_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SoundType>::convertToValue(RBX::Name const&,RBX::SoundType&)const
// IDA 0x4c2af8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2af8() {
}

// 0x4c2b74 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SoundType>> *)
// IDA 0x4c2b74: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2b74() {
}

// 0x4c2b9c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::initSingleton(void)
// IDA 0x4c2b9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c2b9c() {
}

// 0x4c2ba0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState> const>::doGetSingleton(void)
// IDA 0x4c2ba0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2ba0() {
}

// 0x4c2c90 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()
// IDA 0x4c2c90: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c2c90() {
}

// 0x4c2c94 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()
// IDA 0x4c2c94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c2c94() {
}

// 0x4c2e68 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED0Ev
// type: void __fastcall(void *, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::~EnumDesc()
// IDA 0x4c2e68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c2e68() {
}

// 0x4c2f08 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(char const*)const
// IDA 0x4c2f08: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2f08() {
}

// 0x4c2f38 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c2f38: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2f38() {
}

// 0x4c2f58 — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c2f58: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c2f58() {
}

// 0x851008 — __ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)
// IDA 0x851008: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_851008() {
}
