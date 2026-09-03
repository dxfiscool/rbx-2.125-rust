//! rendering shard 354 — 120 stubs 0x4c6970..0x4cad30 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 38540->38660 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4c6970 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()
// IDA 0x4c6970: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c6970() {
}

// 0x4c6974 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()
// IDA 0x4c6974: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c6974() {
}

// 0x4c6b48 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()
// IDA 0x4c6b48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c6b48() {
}

// 0x4c6be8 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(char const*)const
// IDA 0x4c6be8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c6be8() {
}

// 0x4c6c18 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c6c18: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c6c18() {
}

// 0x4c6c38 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c6c38: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c6c38() {
}

// 0x4c6c94 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(unsigned long,std::string &)const
// IDA 0x4c6c94: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c6c94() {
}

// 0x4c6dd8 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const
// IDA 0x4c6dd8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c6dd8() {
}

// 0x4c6f78 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ExtrudedPartInstance16VisualTrussStyleEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ExtrudedPartInstance::VisualTrussStyle>(RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ExtrudedPartInstance::VisualTrussStyle>(RBX::ExtrudedPartInstance::VisualTrussStyle const&)
// IDA 0x4c6f78: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c6f78() {
}

// 0x4c6fc8 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::singleton(void)
// IDA 0x4c6fc8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c6fc8() {
}

// 0x4c7034 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::construct_func(char const*,char *)
// IDA 0x4c7034: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7034() {
}

// 0x4c7040 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::destruct_func(char *)
// IDA 0x4c7040: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c7040() {
}

// 0x4c7044 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToItem(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToItem(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const
// IDA 0x4c7044: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7044() {
}

// 0x4c7110 — __ZN3rbx8any_castIRKN3RBX20ExtrudedPartInstance16VisualTrussStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::ExtrudedPartInstance::VisualTrussStyle const& rbx::any_cast<RBX::ExtrudedPartInstance::VisualTrussStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::ExtrudedPartInstance::VisualTrussStyle const& rbx::any_cast<RBX::ExtrudedPartInstance::VisualTrussStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c7110: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7110() {
}

// 0x4c7200 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(RBX::Name const&,RBX::ExtrudedPartInstance::VisualTrussStyle&)const")]
// was: RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(RBX::Name const&,RBX::ExtrudedPartInstance::VisualTrussStyle&)const
// IDA 0x4c7200: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7200() {
}

// 0x4c727c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>> *)
// IDA 0x4c727c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c727c() {
}

// 0x4c72a4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::initSingleton(void)
// IDA 0x4c72a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c72a4() {
}

// 0x4c72a8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::doGetSingleton(void)
// IDA 0x4c72a8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c72a8() {
}

// 0x4c7398 — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()
// IDA 0x4c7398: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c7398() {
}

// 0x4c739c — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()
// IDA 0x4c739c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c739c() {
}

// 0x4c7570 — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()
// IDA 0x4c7570: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c7570() {
}

// 0x4c7610 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(char const*)const
// IDA 0x4c7610: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7610() {
}

// 0x4c7640 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c7640: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7640() {
}

// 0x4c7660 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c7660: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7660() {
}

// 0x4c76bc — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(unsigned long,std::string &)const
// IDA 0x4c76bc: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c76bc() {
}

// 0x4c7800 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(RBX::PersonalServerService::PrivilegeType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(RBX::PersonalServerService::PrivilegeType const&)const
// IDA 0x4c7800: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7800() {
}

// 0x4c79a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_21PersonalServerService13PrivilegeTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PersonalServerService::PrivilegeType>(RBX::PersonalServerService::PrivilegeType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PersonalServerService::PrivilegeType>(RBX::PersonalServerService::PrivilegeType const&)
// IDA 0x4c79a0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c79a0() {
}

// 0x4c79f0 — __ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::singleton(void)
// IDA 0x4c79f0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c79f0() {
}

// 0x4c7a5c — __ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::construct_func(char const*,char *)
// IDA 0x4c7a5c: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7a5c() {
}

// 0x4c7a68 — __ZN3rbx14implementation12typed_holderIN3RBX21PersonalServerService13PrivilegeTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::PersonalServerService::PrivilegeType>::destruct_func(char *)
// IDA 0x4c7a68: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c7a68() {
}

// 0x4c7a6c — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToItem(RBX::PersonalServerService::PrivilegeType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToItem(RBX::PersonalServerService::PrivilegeType const&)const
// IDA 0x4c7a6c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7a6c() {
}

// 0x4c7b38 — __ZN3rbx8any_castIRKN3RBX21PersonalServerService13PrivilegeTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::PersonalServerService::PrivilegeType const& rbx::any_cast<RBX::PersonalServerService::PrivilegeType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::PersonalServerService::PrivilegeType const& rbx::any_cast<RBX::PersonalServerService::PrivilegeType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c7b38: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7b38() {
}

// 0x4c7c28 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(RBX::Name const&,RBX::PersonalServerService::PrivilegeType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(RBX::Name const&,RBX::PersonalServerService::PrivilegeType&)const
// IDA 0x4c7c28: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7c28() {
}

// 0x4c7ca4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_21PersonalServerService13PrivilegeTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PersonalServerService::PrivilegeType>> *)
// IDA 0x4c7ca4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7ca4() {
}

// 0x4c7ccc — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE13initSingletonEv
// type: int()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SocialService::StuffType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SocialService::StuffType> const>::initSingleton(void)
// IDA 0x4c7ccc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c7ccc() {
}

// 0x4c7cd0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SocialService::StuffType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::SocialService::StuffType> const>::doGetSingleton(void)
// IDA 0x4c7cd0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c7cd0() {
}

// 0x4c7dc0 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::~EnumDesc()
// IDA 0x4c7dc0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c7dc0() {
}

// 0x4c7dc4 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::~EnumDesc()
// IDA 0x4c7dc4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c7dc4() {
}

// 0x4c7f98 — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::~EnumDesc()
// IDA 0x4c7f98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c7f98() {
}

// 0x4c8038 — __ZNK3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::lookup(char const*)const
// IDA 0x4c8038: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8038() {
}

// 0x4c8068 — __ZNK3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c8068: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8068() {
}

// 0x4c8088 — __ZNK3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c8088: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8088() {
}

// 0x4c80e4 — __ZNK3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToString(unsigned long,std::string &)const
// IDA 0x4c80e4: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c80e4() {
}

// 0x4c8228 — __ZNK3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToString(RBX::SocialService::StuffType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToString(RBX::SocialService::StuffType const&)const
// IDA 0x4c8228: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8228() {
}

// 0x4c83c8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13SocialService9StuffTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SocialService::StuffType>(RBX::SocialService::StuffType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::SocialService::StuffType>(RBX::SocialService::StuffType const&)
// IDA 0x4c83c8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c83c8() {
}

// 0x4c8418 — __ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::SocialService::StuffType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::SocialService::StuffType>::singleton(void)
// IDA 0x4c8418: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8418() {
}

// 0x4c8484 — __ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::SocialService::StuffType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::SocialService::StuffType>::construct_func(char const*,char *)
// IDA 0x4c8484: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8484() {
}

// 0x4c8490 — __ZN3rbx14implementation12typed_holderIN3RBX13SocialService9StuffTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::SocialService::StuffType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::SocialService::StuffType>::destruct_func(char *)
// IDA 0x4c8490: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c8490() {
}

// 0x4c8494 — __ZNK3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToItem(RBX::SocialService::StuffType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToItem(RBX::SocialService::StuffType const&)const
// IDA 0x4c8494: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8494() {
}

// 0x4c8560 — __ZN3rbx8any_castIRKN3RBX13SocialService9StuffTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::SocialService::StuffType const& rbx::any_cast<RBX::SocialService::StuffType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::SocialService::StuffType const& rbx::any_cast<RBX::SocialService::StuffType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c8560: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8560() {
}

// 0x4c8650 — __ZNK3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToValue(RBX::Name const&,RBX::SocialService::StuffType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::SocialService::StuffType>::convertToValue(RBX::Name const&,RBX::SocialService::StuffType&)const
// IDA 0x4c8650: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8650() {
}

// 0x4c86cc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>> *)
// IDA 0x4c86cc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c86cc() {
}

// 0x4c86f4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16KeyframeSequence8PriorityEEEE13initSingletonEv
// type: int()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority> const>::initSingleton(void)
// IDA 0x4c86f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c86f4() {
}

// 0x4c86f8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16KeyframeSequence8PriorityEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority> const>::doGetSingleton(void)
// IDA 0x4c86f8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c86f8() {
}

// 0x4c87e8 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::~EnumDesc()
// IDA 0x4c87e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c87e8() {
}

// 0x4c87ec — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::~EnumDesc()
// IDA 0x4c87ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c87ec() {
}

// 0x4c89c0 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::~EnumDesc()
// IDA 0x4c89c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c89c0() {
}

// 0x4c8a60 — __ZNK3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::lookup(char const*)const
// IDA 0x4c8a60: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8a60() {
}

// 0x4c8a90 — __ZNK3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c8a90: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8a90() {
}

// 0x4c8ab0 — __ZNK3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c8ab0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8ab0() {
}

// 0x4c8b0c — __ZNK3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToString(unsigned long,std::string &)const
// IDA 0x4c8b0c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8b0c() {
}

// 0x4c8c50 — __ZNK3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToString(RBX::KeyframeSequence::Priority const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToString(RBX::KeyframeSequence::Priority const&)const
// IDA 0x4c8c50: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8c50() {
}

// 0x4c8df0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_16KeyframeSequence8PriorityEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeyframeSequence::Priority>(RBX::KeyframeSequence::Priority const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeyframeSequence::Priority>(RBX::KeyframeSequence::Priority const&)
// IDA 0x4c8df0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8df0() {
}

// 0x4c8e40 — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::singleton(void)
// IDA 0x4c8e40: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8e40() {
}

// 0x4c8eac — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::construct_func(char const*,char *)
// IDA 0x4c8eac: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8eac() {
}

// 0x4c8eb8 — __ZN3rbx14implementation12typed_holderIN3RBX16KeyframeSequence8PriorityEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::KeyframeSequence::Priority>::destruct_func(char *)
// IDA 0x4c8eb8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c8eb8() {
}

// 0x4c8ebc — __ZNK3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToItem(RBX::KeyframeSequence::Priority const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToItem(RBX::KeyframeSequence::Priority const&)const
// IDA 0x4c8ebc: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8ebc() {
}

// 0x4c8f88 — __ZN3rbx8any_castIRKN3RBX16KeyframeSequence8PriorityENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::KeyframeSequence::Priority const& rbx::any_cast<RBX::KeyframeSequence::Priority const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::KeyframeSequence::Priority const& rbx::any_cast<RBX::KeyframeSequence::Priority const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c8f88: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c8f88() {
}

// 0x4c9078 — __ZNK3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToValue(RBX::Name const&,RBX::KeyframeSequence::Priority&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeyframeSequence::Priority>::convertToValue(RBX::Name const&,RBX::KeyframeSequence::Priority&)const
// IDA 0x4c9078: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9078() {
}

// 0x4c90f4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16KeyframeSequence8PriorityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeyframeSequence::Priority>> *)
// IDA 0x4c90f4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c90f4() {
}

// 0x4c911c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType> const>::initSingleton(void)
// IDA 0x4c911c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c911c() {
}

// 0x4c9120 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType> const>::doGetSingleton(void)
// IDA 0x4c9120: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9120() {
}

// 0x4c9210 — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::~EnumDesc()
// IDA 0x4c9210: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c9210() {
}

// 0x4c9214 — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::~EnumDesc()
// IDA 0x4c9214: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c9214() {
}

// 0x4c93e8 — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::~EnumDesc()
// IDA 0x4c93e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c93e8() {
}

// 0x4c9488 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::lookup(char const*)const
// IDA 0x4c9488: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9488() {
}

// 0x4c94b8 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c94b8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c94b8() {
}

// 0x4c94d8 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c94d8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c94d8() {
}

// 0x4c9534 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToString(unsigned long,std::string &)const
// IDA 0x4c9534: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9534() {
}

// 0x4c9678 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToString(RBX::BasicPartInstance::LegacyPartType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToString(RBX::BasicPartInstance::LegacyPartType const&)const
// IDA 0x4c9678: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9678() {
}

// 0x4c9818 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17BasicPartInstance14LegacyPartTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BasicPartInstance::LegacyPartType>(RBX::BasicPartInstance::LegacyPartType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::BasicPartInstance::LegacyPartType>(RBX::BasicPartInstance::LegacyPartType const&)
// IDA 0x4c9818: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9818() {
}

// 0x4c9868 — __ZN3rbx14implementation12typed_holderIN3RBX17BasicPartInstance14LegacyPartTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::singleton(void)
// IDA 0x4c9868: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9868() {
}

// 0x4c98d4 — __ZN3rbx14implementation12typed_holderIN3RBX17BasicPartInstance14LegacyPartTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::construct_func(char const*,char *)
// IDA 0x4c98d4: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c98d4() {
}

// 0x4c98e0 — __ZN3rbx14implementation12typed_holderIN3RBX17BasicPartInstance14LegacyPartTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::BasicPartInstance::LegacyPartType>::destruct_func(char *)
// IDA 0x4c98e0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4c98e0() {
}

// 0x4c98e4 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToItem(RBX::BasicPartInstance::LegacyPartType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToItem(RBX::BasicPartInstance::LegacyPartType const&)const
// IDA 0x4c98e4: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c98e4() {
}

// 0x4c99b0 — __ZN3rbx8any_castIRKN3RBX17BasicPartInstance14LegacyPartTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::BasicPartInstance::LegacyPartType const& rbx::any_cast<RBX::BasicPartInstance::LegacyPartType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::BasicPartInstance::LegacyPartType const& rbx::any_cast<RBX::BasicPartInstance::LegacyPartType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4c99b0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c99b0() {
}

// 0x4c9aa0 — __ZNK3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToValue(RBX::Name const&,RBX::BasicPartInstance::LegacyPartType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::BasicPartInstance::LegacyPartType>::convertToValue(RBX::Name const&,RBX::BasicPartInstance::LegacyPartType&)const
// IDA 0x4c9aa0: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9aa0() {
}

// 0x4c9b1c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17BasicPartInstance14LegacyPartTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::BasicPartInstance::LegacyPartType>> *)
// IDA 0x4c9b1c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9b1c() {
}

// 0x4c9b44 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid13NameOcclusionEEEE13initSingletonEv
// type: int()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion> const>::initSingleton(void)
// IDA 0x4c9b44: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c9b44() {
}

// 0x4c9b48 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid13NameOcclusionEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion> const>::doGetSingleton(void)
// IDA 0x4c9b48: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9b48() {
}

// 0x4c9c38 — __ZN3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::~EnumDesc()
// IDA 0x4c9c38: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4c9c38() {
}

// 0x4c9c3c — __ZN3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::~EnumDesc()
// IDA 0x4c9c3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c9c3c() {
}

// 0x4c9e10 — __ZN3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::~EnumDesc()
// IDA 0x4c9e10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4c9e10() {
}

// 0x4c9eb0 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::lookup(char const*)const
// IDA 0x4c9eb0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9eb0() {
}

// 0x4c9ee0 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4c9ee0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9ee0() {
}

// 0x4c9f00 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4c9f00: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9f00() {
}

// 0x4c9f5c — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToString(unsigned long,std::string &)const
// IDA 0x4c9f5c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4c9f5c() {
}

// 0x4ca0a0 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToString(RBX::Humanoid::NameOcclusion const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToString(RBX::Humanoid::NameOcclusion const&)const
// IDA 0x4ca0a0: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca0a0() {
}

// 0x4ca240 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid13NameOcclusionEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::NameOcclusion>(RBX::Humanoid::NameOcclusion const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::NameOcclusion>(RBX::Humanoid::NameOcclusion const&)
// IDA 0x4ca240: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca240() {
}

// 0x4ca290 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::singleton(void)
// IDA 0x4ca290: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca290() {
}

// 0x4ca2fc — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::construct_func(char const*,char *)
// IDA 0x4ca2fc: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca2fc() {
}

// 0x4ca308 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid13NameOcclusionEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Humanoid::NameOcclusion>::destruct_func(char *)
// IDA 0x4ca308: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ca308() {
}

// 0x4ca30c — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToItem(RBX::Humanoid::NameOcclusion const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToItem(RBX::Humanoid::NameOcclusion const&)const
// IDA 0x4ca30c: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca30c() {
}

// 0x4ca3d8 — __ZN3rbx8any_castIRKN3RBX8Humanoid13NameOcclusionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Humanoid::NameOcclusion const& rbx::any_cast<RBX::Humanoid::NameOcclusion const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Humanoid::NameOcclusion const& rbx::any_cast<RBX::Humanoid::NameOcclusion const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4ca3d8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca3d8() {
}

// 0x4ca4c8 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToValue(RBX::Name const&,RBX::Humanoid::NameOcclusion&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::NameOcclusion>::convertToValue(RBX::Name const&,RBX::Humanoid::NameOcclusion&)const
// IDA 0x4ca4c8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca4c8() {
}

// 0x4ca544 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>> *)
// IDA 0x4ca544: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca544() {
}

// 0x4ca56c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid6StatusEEEE13initSingletonEv
// type: int()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::Status> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::Status> const>::initSingleton(void)
// IDA 0x4ca56c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ca56c() {
}

// 0x4ca570 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid6StatusEEEE14doGetSingletonEv
// type: _UNKNOWN **()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::Status> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Humanoid::Status> const>::doGetSingleton(void)
// IDA 0x4ca570: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca570() {
}

// 0x4ca660 — __ZN3RBX10Reflection8EnumDescINS_8Humanoid6StatusEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::~EnumDesc()
// IDA 0x4ca660: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ca660() {
}

// 0x4ca664 — __ZN3RBX10Reflection8EnumDescINS_8Humanoid6StatusEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::~EnumDesc()
// IDA 0x4ca664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ca664() {
}

// 0x4ca838 — __ZN3RBX10Reflection8EnumDescINS_8Humanoid6StatusEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::~EnumDesc()
// IDA 0x4ca838: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ca838() {
}

// 0x4ca8d8 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid6StatusEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::lookup(char const*)const
// IDA 0x4ca8d8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca8d8() {
}

// 0x4ca908 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid6StatusEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4ca908: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca908() {
}

// 0x4ca928 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid6StatusEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4ca928: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca928() {
}

// 0x4ca984 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid6StatusEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToString(unsigned long,std::string &)const
// IDA 0x4ca984: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ca984() {
}

// 0x4caac8 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid6StatusEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToString(RBX::Humanoid::Status const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToString(RBX::Humanoid::Status const&)const
// IDA 0x4caac8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4caac8() {
}

// 0x4cac68 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_8Humanoid6StatusEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::Status>(RBX::Humanoid::Status const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Humanoid::Status>(RBX::Humanoid::Status const&)
// IDA 0x4cac68: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cac68() {
}

// 0x4cacb8 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Humanoid::Status>::singleton(void)
// IDA 0x4cacb8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cacb8() {
}

// 0x4cad24 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Humanoid::Status>::construct_func(char const*,char *)
// IDA 0x4cad24: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cad24() {
}

// 0x4cad30 — __ZN3rbx14implementation12typed_holderIN3RBX8Humanoid6StatusEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Humanoid::Status>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Humanoid::Status>::destruct_func(char *)
// IDA 0x4cad30: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4cad30() {
}