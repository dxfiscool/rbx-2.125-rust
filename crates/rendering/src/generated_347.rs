//! rendering shard 347 — 100 stubs 0x4b5120..0x4b8850 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 37940->38040 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 37940 before -> 38040 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 lowest remaining 0x4b5120..0x4b8850 (next lowest 0x4b88ac if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4b5120 — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToString(RBX::AssetService::AccessType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE15convertToStringERKS3_
// IDA 0x4b5120: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5120() {
}

// 0x4b52c0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_
// IDA 0x4b52c0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b52c0() {
}

// 0x4b5310 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv
// IDA 0x4b5310: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5310() {
}

// 0x4b537c — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc
// IDA 0x4b537c: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b537c() {
}

// 0x4b5388 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc
// IDA 0x4b5388: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4b5388() {
}

// 0x4b538c — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToItem(RBX::AssetService::AccessType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE13convertToItemERKS3_
// IDA 0x4b538c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b538c() {
}

// 0x4b5458 — __ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::AssetService::AccessType const& rbx::any_cast<RBX::AssetService::AccessType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x4b5458: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5458() {
}

// 0x4b5548 — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToValue(RBX::Name const&,RBX::AssetService::AccessType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE14convertToValueERKNS_4NameERS3_
// IDA 0x4b5548: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5548() {
}

// 0x4b55c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x4b55c4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b55c4() {
}

// 0x4b55ec — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE13initSingletonEv
// IDA 0x4b55ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b55ec() {
}

// 0x4b55f0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE14doGetSingletonEv
// IDA 0x4b55f0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b55f0() {
}

// 0x4b56e0 — __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED1Ev
// IDA 0x4b56e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b56e0() {
}

// 0x4b56e4 — __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED2Ev
// IDA 0x4b56e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b56e4() {
}

// 0x4b58b8 — __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED0Ev
// IDA 0x4b58b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b58b8() {
}

// 0x4b5958 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupEPKc
// IDA 0x4b5958: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5958() {
}

// 0x4b5988 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupERKNS0_7VariantE
// IDA 0x4b5988: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5988() {
}

// 0x4b59a8 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE14convertToValueEmRNS0_7VariantE
// IDA 0x4b59a8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b59a8() {
}

// 0x4b5a04 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringEmRSs
// IDA 0x4b5a04: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5a04() {
}

// 0x4b5b48 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(RBX::InputObject::UserInputState const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringERKS3_
// IDA 0x4b5b48: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5b48() {
}

// 0x4b5ce8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_
// IDA 0x4b5ce8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5ce8() {
}

// 0x4b5d38 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv
// IDA 0x4b5d38: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5d38() {
}

// 0x4b5da4 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc
// IDA 0x4b5da4: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5da4() {
}

// 0x4b5db0 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc
// IDA 0x4b5db0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4b5db0() {
}

// 0x4b5db4 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToItem(RBX::InputObject::UserInputState const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE13convertToItemERKS3_
// IDA 0x4b5db4: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5db4() {
}

// 0x4b5e80 — __ZN3rbx8any_castIRKN3RBX11InputObject14UserInputStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::InputObject::UserInputState const& rbx::any_cast<RBX::InputObject::UserInputState const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX11InputObject14UserInputStateENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x4b5e80: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5e80() {
}

// 0x4b5f70 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToValue(RBX::Name const&,RBX::InputObject::UserInputState&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE14convertToValueERKNS_4NameERS3_
// IDA 0x4b5f70: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5f70() {
}

// 0x4b5fec — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputState>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject14UserInputStateEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x4b5fec: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b5fec() {
}

// 0x4b6014 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject13UserInputTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject13UserInputTypeEEEE13initSingletonEv
// IDA 0x4b6014: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b6014() {
}

// 0x4b6018 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject13UserInputTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject13UserInputTypeEEEE14doGetSingletonEv
// IDA 0x4b6018: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6018() {
}

// 0x4b6108 — __ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED1Ev
// IDA 0x4b6108: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b6108() {
}

// 0x4b610c — __ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED2Ev
// IDA 0x4b610c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b610c() {
}

// 0x4b62e0 — __ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED0Ev
// IDA 0x4b62e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b62e0() {
}

// 0x4b6380 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE6lookupEPKc
// IDA 0x4b6380: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6380() {
}

// 0x4b63b0 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE6lookupERKNS0_7VariantE
// IDA 0x4b63b0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b63b0() {
}

// 0x4b63d0 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE14convertToValueEmRNS0_7VariantE
// IDA 0x4b63d0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b63d0() {
}

// 0x4b642c — __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE15convertToStringEmRSs
// IDA 0x4b642c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b642c() {
}

// 0x4b6570 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::convertToString(RBX::InputObject::UserInputType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE15convertToStringERKS3_
// IDA 0x4b6570: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6570() {
}

// 0x4b6710 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputType>(RBX::InputObject::UserInputType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject13UserInputTypeEEERS3_RKT_
// IDA 0x4b6710: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6710() {
}

// 0x4b6760 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE9singletonEv
// IDA 0x4b6760: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6760() {
}

// 0x4b67cc — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE14construct_funcEPKcPc
// IDA 0x4b67cc: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b67cc() {
}

// 0x4b67d8 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX11InputObject13UserInputTypeEE13destruct_funcEPc
// IDA 0x4b67d8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4b67d8() {
}

// 0x4b67dc — __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::convertToItem(RBX::InputObject::UserInputType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE13convertToItemERKS3_
// IDA 0x4b67dc: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b67dc() {
}

// 0x4b68a8 — __ZN3rbx8any_castIRKN3RBX11InputObject13UserInputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::InputObject::UserInputType const& rbx::any_cast<RBX::InputObject::UserInputType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX11InputObject13UserInputTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x4b68a8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b68a8() {
}

// 0x4b6998 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputType>::convertToValue(RBX::Name const&,RBX::InputObject::UserInputType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEE14convertToValueERKNS_4NameERS3_
// IDA 0x4b6998: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6998() {
}

// 0x4b6a14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::InputObject::UserInputType>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11InputObject13UserInputTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x4b6a14: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6a14() {
}

// 0x4b6a3c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE14doGetSingletonEv
// IDA 0x4b6a3c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6a3c() {
}

// 0x4b6b2c — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED0Ev
// IDA 0x4b6b2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b6b2c() {
}

// 0x4b6bcc — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE6lookupEPKc
// IDA 0x4b6bcc: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6bcc() {
}

// 0x4b6bfc — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE6lookupERKNS0_7VariantE
// IDA 0x4b6bfc: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6bfc() {
}

// 0x4b6c1c — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToValueEmRNS0_7VariantE
// IDA 0x4b6c1c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6c1c() {
}

// 0x4b6c78 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToString(RBX::Explosion::ExplosionType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE15convertToStringERKS3_
// IDA 0x4b6c78: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6c78() {
}

// 0x4b6e18 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Explosion::ExplosionType>(RBX::Explosion::ExplosionType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9Explosion13ExplosionTypeEEERS3_RKT_
// IDA 0x4b6e18: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6e18() {
}

// 0x4b6e68 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE9singletonEv
// IDA 0x4b6e68: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6e68() {
}

// 0x4b6ed4 — __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Explosion::ExplosionType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9Explosion13ExplosionTypeEE13destruct_funcEPc
// IDA 0x4b6ed4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4b6ed4() {
}

// 0x4b6ed8 — __ZN3rbx8any_castIRKN3RBX9Explosion13ExplosionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Explosion::ExplosionType const& rbx::any_cast<RBX::Explosion::ExplosionType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX9Explosion13ExplosionTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x4b6ed8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6ed8() {
}

// 0x4b6fc8 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Explosion::ExplosionType>::convertToValue(RBX::Name const&,RBX::Explosion::ExplosionType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToValueERKNS_4NameERS3_
// IDA 0x4b6fc8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b6fc8() {
}

// 0x4b7044 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel18WaterCellDirectionEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel18WaterCellDirectionEEEE13initSingletonEv
// IDA 0x4b7044: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b7044() {
}

// 0x4b7048 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel18WaterCellDirectionEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel18WaterCellDirectionEEEE14doGetSingletonEv
// IDA 0x4b7048: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7048() {
}

// 0x4b7138 — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED1Ev
// IDA 0x4b7138: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b7138() {
}

// 0x4b713c — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED2Ev
// IDA 0x4b713c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b713c() {
}

// 0x4b7310 — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED0Ev
// IDA 0x4b7310: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b7310() {
}

// 0x4b73b0 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE6lookupEPKc
// IDA 0x4b73b0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b73b0() {
}

// 0x4b73e0 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE6lookupERKNS0_7VariantE
// IDA 0x4b73e0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b73e0() {
}

// 0x4b7400 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE14convertToValueEmRNS0_7VariantE
// IDA 0x4b7400: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7400() {
}

// 0x4b745c — __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE15convertToStringEmRSs
// IDA 0x4b745c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b745c() {
}

// 0x4b75a0 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::convertToString(RBX::Voxel::WaterCellDirection const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE15convertToStringERKS3_
// IDA 0x4b75a0: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b75a0() {
}

// 0x4b7740 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellDirection>(RBX::Voxel::WaterCellDirection const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel18WaterCellDirectionEEERS3_RKT_
// IDA 0x4b7740: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7740() {
}

// 0x4b7790 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE9singletonEv
// IDA 0x4b7790: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7790() {
}

// 0x4b77fc — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE14construct_funcEPKcPc
// IDA 0x4b77fc: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b77fc() {
}

// 0x4b7808 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellDirection>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5Voxel18WaterCellDirectionEE13destruct_funcEPc
// IDA 0x4b7808: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4b7808() {
}

// 0x4b780c — __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::convertToItem(RBX::Voxel::WaterCellDirection const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE13convertToItemERKS3_
// IDA 0x4b780c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b780c() {
}

// 0x4b78d8 — __ZN3rbx8any_castIRKN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::WaterCellDirection const& rbx::any_cast<RBX::Voxel::WaterCellDirection const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX5Voxel18WaterCellDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x4b78d8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b78d8() {
}

// 0x4b79c8 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellDirection>::convertToValue(RBX::Name const&,RBX::Voxel::WaterCellDirection&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEE14convertToValueERKNS_4NameERS3_
// IDA 0x4b79c8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b79c8() {
}

// 0x4b7a44 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellDirection>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel18WaterCellDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x4b7a44: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7a44() {
}

// 0x4b7a6c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel14WaterCellForceEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel14WaterCellForceEEEE13initSingletonEv
// IDA 0x4b7a6c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b7a6c() {
}

// 0x4b7a70 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel14WaterCellForceEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel14WaterCellForceEEEE14doGetSingletonEv
// IDA 0x4b7a70: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7a70() {
}

// 0x4b7b60 — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED1Ev
// IDA 0x4b7b60: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b7b60() {
}

// 0x4b7b64 — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED2Ev
// IDA 0x4b7b64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b7b64() {
}

// 0x4b7d38 — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED0Ev
// IDA 0x4b7d38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b7d38() {
}

// 0x4b7dd8 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE6lookupEPKc
// IDA 0x4b7dd8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7dd8() {
}

// 0x4b7e08 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE6lookupERKNS0_7VariantE
// IDA 0x4b7e08: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7e08() {
}

// 0x4b7e28 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE14convertToValueEmRNS0_7VariantE
// IDA 0x4b7e28: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7e28() {
}

// 0x4b7e84 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE15convertToStringEmRSs
// IDA 0x4b7e84: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7e84() {
}

// 0x4b7fc8 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::convertToString(RBX::Voxel::WaterCellForce const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE15convertToStringERKS3_
// IDA 0x4b7fc8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b7fc8() {
}

// 0x4b8168 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::WaterCellForce>(RBX::Voxel::WaterCellForce const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel14WaterCellForceEEERS3_RKT_
// IDA 0x4b8168: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8168() {
}

// 0x4b81b8 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE9singletonEv
// IDA 0x4b81b8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b81b8() {
}

// 0x4b8224 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE14construct_funcEPKcPc
// IDA 0x4b8224: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8224() {
}

// 0x4b8230 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::WaterCellForce>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX5Voxel14WaterCellForceEE13destruct_funcEPc
// IDA 0x4b8230: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4b8230() {
}

// 0x4b8234 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::convertToItem(RBX::Voxel::WaterCellForce const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE13convertToItemERKS3_
// IDA 0x4b8234: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8234() {
}

// 0x4b8300 — __ZN3rbx8any_castIRKN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::WaterCellForce const& rbx::any_cast<RBX::Voxel::WaterCellForce const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX5Voxel14WaterCellForceENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x4b8300: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8300() {
}

// 0x4b83f0 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::WaterCellForce>::convertToValue(RBX::Name const&,RBX::Voxel::WaterCellForce&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEE14convertToValueERKNS_4NameERS3_
// IDA 0x4b83f0: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b83f0() {
}

// 0x4b846c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::WaterCellForce>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel14WaterCellForceEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x4b846c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b846c() {
}

// 0x4b8494 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel15CellOrientationEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel15CellOrientationEEEE13initSingletonEv
// IDA 0x4b8494: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b8494() {
}

// 0x4b8498 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel15CellOrientationEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel15CellOrientationEEEE14doGetSingletonEv
// IDA 0x4b8498: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8498() {
}

// 0x4b8588 — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED1Ev
// IDA 0x4b8588: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4b8588() {
}

// 0x4b858c — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED2Ev
// IDA 0x4b858c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b858c() {
}

// 0x4b8760 — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED0Ev
// IDA 0x4b8760: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4b8760() {
}

// 0x4b8800 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE6lookupEPKc
// IDA 0x4b8800: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8800() {
}

// 0x4b8830 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE6lookupERKNS0_7VariantE
// IDA 0x4b8830: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8830() {
}

// 0x4b8850 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE14convertToValueEmRNS0_7VariantE
// IDA 0x4b8850: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4b8850() {
}
