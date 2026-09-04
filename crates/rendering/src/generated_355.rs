//! rendering shard 355 — 100 stubs 0x4cad34..0x4ceff0 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 38660->38760 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x4cad34 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid6StatusEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToItem(RBX::Humanoid::Status const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToItem(RBX::Humanoid::Status const&)const
// IDA 0x4cad34: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cad34() {
}

// 0x4cae00 — __ZN3rbx8any_castIRKN3RBX8Humanoid6StatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Humanoid::Status const& rbx::any_cast<RBX::Humanoid::Status const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Humanoid::Status const& rbx::any_cast<RBX::Humanoid::Status const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4cae00: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cae00() {
}

// 0x4caef0 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid6StatusEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToValue(RBX::Name const&,RBX::Humanoid::Status&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Humanoid::Status>::convertToValue(RBX::Name const&,RBX::Humanoid::Status&)const
// IDA 0x4caef0: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4caef0() {
}

// 0x4caf6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::Status>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Humanoid::Status>> *)
// IDA 0x4caf6c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4caf6c() {
}

// 0x4cb9bc — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Legacy17SurfaceConstraintEEEE13initSingletonEv
// type: int()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint> const>::initSingleton(void)
// IDA 0x4cb9bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cb9bc() {
}

// 0x4cb9c0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_6Legacy17SurfaceConstraintEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint> const>::doGetSingleton(void)
// IDA 0x4cb9c0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cb9c0() {
}

// 0x4cbab0 — __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::~EnumDesc()
// IDA 0x4cbab0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cbab0() {
}

// 0x4cbab4 — __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::~EnumDesc()
// IDA 0x4cbab4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cbab4() {
}

// 0x4cbc88 — __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::~EnumDesc()
// IDA 0x4cbc88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cbc88() {
}

// 0x4cbd28 — __ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::lookup(char const*)const
// IDA 0x4cbd28: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cbd28() {
}

// 0x4cbd58 — __ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4cbd58: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cbd58() {
}

// 0x4cbd78 — __ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4cbd78: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cbd78() {
}

// 0x4cbdd4 — __ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToString(unsigned long,std::string &)const
// IDA 0x4cbdd4: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cbdd4() {
}

// 0x4cbf18 — __ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE15convertToStringERKS3_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToString(RBX::Legacy::SurfaceConstraint const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToString(RBX::Legacy::SurfaceConstraint const&)const
// IDA 0x4cbf18: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cbf18() {
}

// 0x4cc0b8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6Legacy17SurfaceConstraintEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Legacy::SurfaceConstraint>(RBX::Legacy::SurfaceConstraint const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Legacy::SurfaceConstraint>(RBX::Legacy::SurfaceConstraint const&)
// IDA 0x4cc0b8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc0b8() {
}

// 0x4cc108 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::singleton(void)
// IDA 0x4cc108: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc108() {
}

// 0x4cc174 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::construct_func(char const*,char *)
// IDA 0x4cc174: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc174() {
}

// 0x4cc180 — __ZN3rbx14implementation12typed_holderIN3RBX6Legacy17SurfaceConstraintEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Legacy::SurfaceConstraint>::destruct_func(char *)
// IDA 0x4cc180: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4cc180() {
}

// 0x4cc184 — __ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToItem(RBX::Legacy::SurfaceConstraint const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToItem(RBX::Legacy::SurfaceConstraint const&)const
// IDA 0x4cc184: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc184() {
}

// 0x4cc250 — __ZN3rbx8any_castIRKN3RBX6Legacy17SurfaceConstraintENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Legacy::SurfaceConstraint const& rbx::any_cast<RBX::Legacy::SurfaceConstraint const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Legacy::SurfaceConstraint const& rbx::any_cast<RBX::Legacy::SurfaceConstraint const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4cc250: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc250() {
}

// 0x4cc340 — __ZNK3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE14convertToValueERKNS_4NameERS3_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToValue(RBX::Name const&,RBX::Legacy::SurfaceConstraint&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::convertToValue(RBX::Name const&,RBX::Legacy::SurfaceConstraint&)const
// IDA 0x4cc340: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc340() {
}

// 0x4cc3bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>> *)
// IDA 0x4cc3bc: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc3bc() {
}

// 0x4cc3e4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17KeywordFilterTypeEEEE13initSingletonEv
// type: int()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeywordFilterType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeywordFilterType> const>::initSingleton(void)
// IDA 0x4cc3e4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cc3e4() {
}

// 0x4cc3e8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17KeywordFilterTypeEEEE14doGetSingletonEv
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeywordFilterType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::KeywordFilterType> const>::doGetSingleton(void)
// IDA 0x4cc3e8: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc3e8() {
}

// 0x4cc4d8 — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::~EnumDesc()
// IDA 0x4cc4d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cc4d8() {
}

// 0x4cc4dc — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::~EnumDesc()
// IDA 0x4cc4dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cc4dc() {
}

// 0x4cc6b0 — __ZN3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEED0Ev
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::~EnumDesc()
// IDA 0x4cc6b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cc6b0() {
}

// 0x4cc750 — __ZNK3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::lookup(char const*)const
// IDA 0x4cc750: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc750() {
}

// 0x4cc780 — __ZNK3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE6lookupERKNS0_7VariantE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4cc780: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc780() {
}

// 0x4cc7a0 — __ZNK3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4cc7a0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc7a0() {
}

// 0x4cc7fc — __ZNK3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToString(unsigned long,std::string &)const
// IDA 0x4cc7fc: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc7fc() {
}

// 0x4cc940 — __ZNK3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE15convertToStringERKS2_
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToString(RBX::KeywordFilterType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToString(RBX::KeywordFilterType const&)const
// IDA 0x4cc940: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cc940() {
}

// 0x4ccae0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17KeywordFilterTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeywordFilterType>(RBX::KeywordFilterType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::KeywordFilterType>(RBX::KeywordFilterType const&)
// IDA 0x4ccae0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ccae0() {
}

// 0x4ccb30 — __ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::KeywordFilterType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::KeywordFilterType>::singleton(void)
// IDA 0x4ccb30: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ccb30() {
}

// 0x4ccb9c — __ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::KeywordFilterType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::KeywordFilterType>::construct_func(char const*,char *)
// IDA 0x4ccb9c: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ccb9c() {
}

// 0x4ccba8 — __ZN3rbx14implementation12typed_holderIN3RBX17KeywordFilterTypeEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::KeywordFilterType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::KeywordFilterType>::destruct_func(char *)
// IDA 0x4ccba8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4ccba8() {
}

// 0x4ccbac — __ZNK3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE13convertToItemERKS2_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToItem(RBX::KeywordFilterType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToItem(RBX::KeywordFilterType const&)const
// IDA 0x4ccbac: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ccbac() {
}

// 0x4ccc78 — __ZN3rbx8any_castIRKN3RBX17KeywordFilterTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::KeywordFilterType const& rbx::any_cast<RBX::KeywordFilterType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::KeywordFilterType const& rbx::any_cast<RBX::KeywordFilterType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4ccc78: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ccc78() {
}

// 0x4ccd68 — __ZNK3RBX10Reflection8EnumDescINS_17KeywordFilterTypeEE14convertToValueERKNS_4NameERS2_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToValue(RBX::Name const&,RBX::KeywordFilterType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::KeywordFilterType>::convertToValue(RBX::Name const&,RBX::KeywordFilterType&)const
// IDA 0x4ccd68: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ccd68() {
}

// 0x4ccde4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17KeywordFilterTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeywordFilterType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::KeywordFilterType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::KeywordFilterType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::KeywordFilterType>> *)
// IDA 0x4ccde4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ccde4() {
}

// 0x4cce0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Joint9JointTypeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Joint::JointType> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Joint::JointType> const>::initSingleton(void)
// IDA 0x4cce0c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cce0c() {
}

// 0x4cce10 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Joint9JointTypeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Joint::JointType> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Joint::JointType> const>::doGetSingleton(void)
// IDA 0x4cce10: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cce10() {
}

// 0x4ccf00 — __ZN3RBX10Reflection8EnumDescINS_5Joint9JointTypeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::~EnumDesc()
// IDA 0x4ccf00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ccf00() {
}

// 0x4ccf04 — __ZN3RBX10Reflection8EnumDescINS_5Joint9JointTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::~EnumDesc()
// IDA 0x4ccf04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ccf04() {
}

// 0x4cd0d8 — __ZN3RBX10Reflection8EnumDescINS_5Joint9JointTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::~EnumDesc()
// IDA 0x4cd0d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cd0d8() {
}

// 0x4cd178 — __ZNK3RBX10Reflection8EnumDescINS_5Joint9JointTypeEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::lookup(char const*)const
// IDA 0x4cd178: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd178() {
}

// 0x4cd1a8 — __ZNK3RBX10Reflection8EnumDescINS_5Joint9JointTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4cd1a8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd1a8() {
}

// 0x4cd1c8 — __ZNK3RBX10Reflection8EnumDescINS_5Joint9JointTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4cd1c8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd1c8() {
}

// 0x4cd224 — __ZNK3RBX10Reflection8EnumDescINS_5Joint9JointTypeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToString(unsigned long,std::string &)const
// IDA 0x4cd224: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd224() {
}

// 0x4cd368 — __ZNK3RBX10Reflection8EnumDescINS_5Joint9JointTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToString(RBX::Joint::JointType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToString(RBX::Joint::JointType const&)const
// IDA 0x4cd368: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd368() {
}

// 0x4cd508 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Joint9JointTypeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Joint::JointType>(RBX::Joint::JointType const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Joint::JointType>(RBX::Joint::JointType const&)
// IDA 0x4cd508: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd508() {
}

// 0x4cd558 — __ZN3rbx14implementation12typed_holderIN3RBX5Joint9JointTypeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Joint::JointType>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Joint::JointType>::singleton(void)
// IDA 0x4cd558: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd558() {
}

// 0x4cd5c4 — __ZN3rbx14implementation12typed_holderIN3RBX5Joint9JointTypeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Joint::JointType>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Joint::JointType>::construct_func(char const*,char *)
// IDA 0x4cd5c4: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd5c4() {
}

// 0x4cd5d0 — __ZN3rbx14implementation12typed_holderIN3RBX5Joint9JointTypeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Joint::JointType>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Joint::JointType>::destruct_func(char *)
// IDA 0x4cd5d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4cd5d0() {
}

// 0x4cd5d4 — __ZNK3RBX10Reflection8EnumDescINS_5Joint9JointTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToItem(RBX::Joint::JointType const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToItem(RBX::Joint::JointType const&)const
// IDA 0x4cd5d4: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd5d4() {
}

// 0x4cd6a0 — __ZN3rbx8any_castIRKN3RBX5Joint9JointTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Joint::JointType const& rbx::any_cast<RBX::Joint::JointType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Joint::JointType const& rbx::any_cast<RBX::Joint::JointType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4cd6a0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd6a0() {
}

// 0x4cd790 — __ZNK3RBX10Reflection8EnumDescINS_5Joint9JointTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToValue(RBX::Name const&,RBX::Joint::JointType&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Joint::JointType>::convertToValue(RBX::Name const&,RBX::Joint::JointType&)const
// IDA 0x4cd790: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd790() {
}

// 0x4cd80c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Joint9JointTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Joint::JointType>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Joint::JointType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Joint::JointType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Joint::JointType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Joint::JointType>> *)
// IDA 0x4cd80c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd80c() {
}

// 0x4cd834 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Feature9TopBottomEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::TopBottom> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::TopBottom> const>::initSingleton(void)
// IDA 0x4cd834: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cd834() {
}

// 0x4cd838 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Feature9TopBottomEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::TopBottom> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::TopBottom> const>::doGetSingleton(void)
// IDA 0x4cd838: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cd838() {
}

// 0x4cd928 — __ZN3RBX10Reflection8EnumDescINS_7Feature9TopBottomEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::~EnumDesc()
// IDA 0x4cd928: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cd928() {
}

// 0x4cd92c — __ZN3RBX10Reflection8EnumDescINS_7Feature9TopBottomEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::~EnumDesc()
// IDA 0x4cd92c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cd92c() {
}

// 0x4cdb00 — __ZN3RBX10Reflection8EnumDescINS_7Feature9TopBottomEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::~EnumDesc()
// IDA 0x4cdb00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cdb00() {
}

// 0x4cdba0 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::lookup(char const*)const
// IDA 0x4cdba0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdba0() {
}

// 0x4cdbd0 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4cdbd0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdbd0() {
}

// 0x4cdbf0 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4cdbf0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdbf0() {
}

// 0x4cdc4c — __ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToString(unsigned long,std::string &)const
// IDA 0x4cdc4c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdc4c() {
}

// 0x4cdd90 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToString(RBX::Feature::TopBottom const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToString(RBX::Feature::TopBottom const&)const
// IDA 0x4cdd90: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdd90() {
}

// 0x4cdf30 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9TopBottomEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::TopBottom>(RBX::Feature::TopBottom const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::TopBottom>(RBX::Feature::TopBottom const&)
// IDA 0x4cdf30: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdf30() {
}

// 0x4cdf80 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Feature::TopBottom>::singleton(void)
// IDA 0x4cdf80: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdf80() {
}

// 0x4cdfec — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Feature::TopBottom>::construct_func(char const*,char *)
// IDA 0x4cdfec: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdfec() {
}

// 0x4cdff8 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9TopBottomEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::TopBottom>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Feature::TopBottom>::destruct_func(char *)
// IDA 0x4cdff8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4cdff8() {
}

// 0x4cdffc — __ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToItem(RBX::Feature::TopBottom const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToItem(RBX::Feature::TopBottom const&)const
// IDA 0x4cdffc: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cdffc() {
}

// 0x4ce0c8 — __ZN3rbx8any_castIRKN3RBX7Feature9TopBottomENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Feature::TopBottom const& rbx::any_cast<RBX::Feature::TopBottom const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Feature::TopBottom const& rbx::any_cast<RBX::Feature::TopBottom const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4ce0c8: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce0c8() {
}

// 0x4ce1b8 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9TopBottomEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToValue(RBX::Name const&,RBX::Feature::TopBottom&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::TopBottom>::convertToValue(RBX::Name const&,RBX::Feature::TopBottom&)const
// IDA 0x4ce1b8: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce1b8() {
}

// 0x4ce234 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9TopBottomEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>> *)
// IDA 0x4ce234: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce234() {
}

// 0x4ce25c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Feature9LeftRightEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::LeftRight> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::LeftRight> const>::initSingleton(void)
// IDA 0x4ce25c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ce25c() {
}

// 0x4ce260 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Feature9LeftRightEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::LeftRight> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::LeftRight> const>::doGetSingleton(void)
// IDA 0x4ce260: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce260() {
}

// 0x4ce350 — __ZN3RBX10Reflection8EnumDescINS_7Feature9LeftRightEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::~EnumDesc()
// IDA 0x4ce350: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ce350() {
}

// 0x4ce354 — __ZN3RBX10Reflection8EnumDescINS_7Feature9LeftRightEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::~EnumDesc()
// IDA 0x4ce354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ce354() {
}

// 0x4ce528 — __ZN3RBX10Reflection8EnumDescINS_7Feature9LeftRightEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::~EnumDesc()
// IDA 0x4ce528: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ce528() {
}

// 0x4ce5c8 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::lookup(char const*)const
// IDA 0x4ce5c8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce5c8() {
}

// 0x4ce5f8 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::lookup(RBX::Reflection::Variant const&)const
// IDA 0x4ce5f8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce5f8() {
}

// 0x4ce618 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// IDA 0x4ce618: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce618() {
}

// 0x4ce674 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToString(unsigned long,std::string &)const
// IDA 0x4ce674: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce674() {
}

// 0x4ce7b8 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToString(RBX::Feature::LeftRight const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToString(RBX::Feature::LeftRight const&)const
// IDA 0x4ce7b8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce7b8() {
}

// 0x4ce958 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_7Feature9LeftRightEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::LeftRight>(RBX::Feature::LeftRight const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Feature::LeftRight>(RBX::Feature::LeftRight const&)
// IDA 0x4ce958: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce958() {
}

// 0x4ce9a8 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Feature::LeftRight>::singleton(void)
// IDA 0x4ce9a8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ce9a8() {
}

// 0x4cea14 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Feature::LeftRight>::construct_func(char const*,char *)
// IDA 0x4cea14: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cea14() {
}

// 0x4cea20 — __ZN3rbx14implementation12typed_holderIN3RBX7Feature9LeftRightEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Feature::LeftRight>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Feature::LeftRight>::destruct_func(char *)
// IDA 0x4cea20: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4cea20() {
}

// 0x4cea24 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToItem(RBX::Feature::LeftRight const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToItem(RBX::Feature::LeftRight const&)const
// IDA 0x4cea24: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cea24() {
}

// 0x4ceaf0 — __ZN3rbx8any_castIRKN3RBX7Feature9LeftRightENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Feature::LeftRight const& rbx::any_cast<RBX::Feature::LeftRight const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Feature::LeftRight const& rbx::any_cast<RBX::Feature::LeftRight const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x4ceaf0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ceaf0() {
}

// 0x4cebe0 — __ZNK3RBX10Reflection8EnumDescINS_7Feature9LeftRightEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToValue(RBX::Name const&,RBX::Feature::LeftRight&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::LeftRight>::convertToValue(RBX::Name const&,RBX::Feature::LeftRight&)const
// IDA 0x4cebe0: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cebe0() {
}

// 0x4cec5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Feature9LeftRightEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>> *)
// IDA 0x4cec5c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cec5c() {
}

// 0x4cec84 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Feature5InOutEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::InOut> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::InOut> const>::initSingleton(void)
// IDA 0x4cec84: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4cec84() {
}

// 0x4cec88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Feature5InOutEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::InOut> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Feature::InOut> const>::doGetSingleton(void)
// IDA 0x4cec88: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4cec88() {
}

// 0x4ced78 — __ZN3RBX10Reflection8EnumDescINS_7Feature5InOutEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::InOut>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::InOut>::~EnumDesc()
// IDA 0x4ced78: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4ced78() {
}

// 0x4ced7c — __ZN3RBX10Reflection8EnumDescINS_7Feature5InOutEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::InOut>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::InOut>::~EnumDesc()
// IDA 0x4ced7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4ced7c() {
}

// 0x4cef50 — __ZN3RBX10Reflection8EnumDescINS_7Feature5InOutEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::InOut>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::InOut>::~EnumDesc()
// IDA 0x4cef50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4cef50() {
}

// 0x4ceff0 — __ZNK3RBX10Reflection8EnumDescINS_7Feature5InOutEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Feature::InOut>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Feature::InOut>::lookup(char const*)const
// IDA 0x4ceff0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4ceff0() {
}