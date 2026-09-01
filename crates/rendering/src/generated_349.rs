//! rendering shard 349 — 100 stubs 0x4b88ac..0x4bc15c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 38160->38260 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 38160 before -> 38260 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 lowest remaining 0x4b88ac..0x4bc15c (next lowest 0x4bc184)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4b88ac — __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToString(unsigned long,std::string &)const
pub fn stub_4b88ac() -> ! {
    todo!("0x4b88ac RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToString(unsigned long,std::string &)const")
}

// 0x4b89f0 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToString(RBX::Voxel::CellOrientation const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToString(RBX::Voxel::CellOrientation const&)const
pub fn stub_4b89f0() -> ! {
    todo!("0x4b89f0 RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToString(RBX::Voxel::CellOrientation const&)const")
}

// 0x4b8b90 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel15CellOrientationEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)
pub fn stub_4b8b90() -> ! {
    todo!("0x4b8b90 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellOrientation>(RBX::Voxel::CellOrientation const&)")
}

// 0x4b8be0 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void)
pub fn stub_4b8be0() -> ! {
    todo!("0x4b8be0 rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::singleton(void)")
}

// 0x4b8c4c — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::construct_func(char const*,char *)
pub fn stub_4b8c4c() -> ! {
    todo!("0x4b8c4c rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::construct_func(char const*,char *)")
}

// 0x4b8c58 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel15CellOrientationEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::destruct_func(char *)
pub fn stub_4b8c58() -> ! {
    todo!("0x4b8c58 rbx::implementation::typed_holder<RBX::Voxel::CellOrientation>::destruct_func(char *)")
}

// 0x4b8c5c — __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToItem(RBX::Voxel::CellOrientation const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToItem(RBX::Voxel::CellOrientation const&)const
pub fn stub_4b8c5c() -> ! {
    todo!("0x4b8c5c RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToItem(RBX::Voxel::CellOrientation const&)const")
}

// 0x4b8d28 — __ZN3rbx8any_castIRKN3RBX5Voxel15CellOrientationENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellOrientation const& rbx::any_cast<RBX::Voxel::CellOrientation const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Voxel::CellOrientation const& rbx::any_cast<RBX::Voxel::CellOrientation const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b8d28() -> ! {
    todo!("0x4b8d28 RBX::Voxel::CellOrientation const& rbx::any_cast<RBX::Voxel::CellOrientation const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4b8e18 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToValue(RBX::Name const&,RBX::Voxel::CellOrientation&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToValue(RBX::Name const&,RBX::Voxel::CellOrientation&)const
pub fn stub_4b8e18() -> ! {
    todo!("0x4b8e18 RBX::Reflection::EnumDesc<RBX::Voxel::CellOrientation>::convertToValue(RBX::Name const&,RBX::Voxel::CellOrientation&)const")
}

// 0x4b8e94 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel15CellOrientationEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *)
pub fn stub_4b8e94() -> ! {
    todo!("0x4b8e94 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellOrientation>> *)")
}

// 0x4b8ebc — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel9CellBlockEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock> const>::initSingleton(void)
pub fn stub_4b8ebc() -> ! {
    todo!("0x4b8ebc RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock> const>::initSingleton(void)")
}

// 0x4b8ec0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel9CellBlockEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock> const>::doGetSingleton(void)
pub fn stub_4b8ec0() -> ! {
    todo!("0x4b8ec0 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock> const>::doGetSingleton(void)")
}

// 0x4b8fb0 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()
pub fn stub_4b8fb0() -> ! {
    todo!("0x4b8fb0 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()")
}

// 0x4b8fb4 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()
pub fn stub_4b8fb4() -> ! {
    todo!("0x4b8fb4 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()")
}

// 0x4b9188 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()
pub fn stub_4b9188() -> ! {
    todo!("0x4b9188 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::~EnumDesc()")
}

// 0x4b9228 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::lookup(char const*)const
pub fn stub_4b9228() -> ! {
    todo!("0x4b9228 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::lookup(char const*)const")
}

// 0x4b9258 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4b9258() -> ! {
    todo!("0x4b9258 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4b9278 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4b9278() -> ! {
    todo!("0x4b9278 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4b92d4 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToString(unsigned long,std::string &)const
pub fn stub_4b92d4() -> ! {
    todo!("0x4b92d4 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToString(unsigned long,std::string &)const")
}

// 0x4b9418 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToString(RBX::Voxel::CellBlock const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToString(RBX::Voxel::CellBlock const&)const
pub fn stub_4b9418() -> ! {
    todo!("0x4b9418 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToString(RBX::Voxel::CellBlock const&)const")
}

// 0x4b95b8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel9CellBlockEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)
pub fn stub_4b95b8() -> ! {
    todo!("0x4b95b8 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellBlock>(RBX::Voxel::CellBlock const&)")
}

// 0x4b9608 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void)
pub fn stub_4b9608() -> ! {
    todo!("0x4b9608 rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::singleton(void)")
}

// 0x4b9674 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::construct_func(char const*,char *)
pub fn stub_4b9674() -> ! {
    todo!("0x4b9674 rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::construct_func(char const*,char *)")
}

// 0x4b9680 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)
pub fn stub_4b9680() -> ! {
    todo!("0x4b9680 rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)")
}

// 0x4b9684 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE13convertToItemERKS3_
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToItem(RBX::Voxel::CellBlock const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToItem(RBX::Voxel::CellBlock const&)const
pub fn stub_4b9684() -> ! {
    todo!("0x4b9684 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToItem(RBX::Voxel::CellBlock const&)const")
}

// 0x4b9750 — __ZN3rbx8any_castIRKN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4b9750() -> ! {
    todo!("0x4b9750 RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4b9840 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(RBX::Name const&,RBX::Voxel::CellBlock&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(RBX::Name const&,RBX::Voxel::CellBlock&)const
pub fn stub_4b9840() -> ! {
    todo!("0x4b9840 RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(RBX::Name const&,RBX::Voxel::CellBlock&)const")
}

// 0x4b98bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)
pub fn stub_4b98bc() -> ! {
    todo!("0x4b98bc std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)")
}

// 0x4b98e4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::initSingleton(void)
pub fn stub_4b98e4() -> ! {
    todo!("0x4b98e4 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::initSingleton(void)")
}

// 0x4b98e8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::doGetSingleton(void)
pub fn stub_4b98e8() -> ! {
    todo!("0x4b98e8 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::doGetSingleton(void)")
}

// 0x4b99d8 — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()
pub fn stub_4b99d8() -> ! {
    todo!("0x4b99d8 RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()")
}

// 0x4b99dc — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()
pub fn stub_4b99dc() -> ! {
    todo!("0x4b99dc RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()")
}

// 0x4b9bb0 — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()
pub fn stub_4b9bb0() -> ! {
    todo!("0x4b9bb0 RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()")
}

// 0x4b9c50 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(char const*)const
pub fn stub_4b9c50() -> ! {
    todo!("0x4b9c50 RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(char const*)const")
}

// 0x4b9c80 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4b9c80() -> ! {
    todo!("0x4b9c80 RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4b9ca0 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4b9ca0() -> ! {
    todo!("0x4b9ca0 RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4b9cfc — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(unsigned long,std::string &)const
pub fn stub_4b9cfc() -> ! {
    todo!("0x4b9cfc RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(unsigned long,std::string &)const")
}

// 0x4b9e40 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(RBX::Voxel::CellMaterial const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(RBX::Voxel::CellMaterial const&)const
pub fn stub_4b9e40() -> ! {
    todo!("0x4b9e40 RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(RBX::Voxel::CellMaterial const&)const")
}

// 0x4b9fe0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)
pub fn stub_4b9fe0() -> ! {
    todo!("0x4b9fe0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)")
}

// 0x4ba030 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)
pub fn stub_4ba030() -> ! {
    todo!("0x4ba030 rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)")
}

// 0x4ba09c — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)
pub fn stub_4ba09c() -> ! {
    todo!("0x4ba09c rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)")
}

// 0x4ba0a8 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)
pub fn stub_4ba0a8() -> ! {
    todo!("0x4ba0a8 rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)")
}

// 0x4ba0ac — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToItem(RBX::Voxel::CellMaterial const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToItem(RBX::Voxel::CellMaterial const&)const
pub fn stub_4ba0ac() -> ! {
    todo!("0x4ba0ac RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToItem(RBX::Voxel::CellMaterial const&)const")
}

// 0x4ba178 — __ZN3rbx8any_castIRKN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4ba178() -> ! {
    todo!("0x4ba178 RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4ba268 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(RBX::Name const&,RBX::Voxel::CellMaterial&)const")]
// was: RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(RBX::Name const&,RBX::Voxel::CellMaterial&)const
pub fn stub_4ba268() -> ! {
    todo!("0x4ba268 RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(RBX::Name const&,RBX::Voxel::CellMaterial&)const")
}

// 0x4ba2e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)
pub fn stub_4ba2e4() -> ! {
    todo!("0x4ba2e4 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)")
}

// 0x4ba30c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::initSingleton(void)
pub fn stub_4ba30c() -> ! {
    todo!("0x4ba30c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::initSingleton(void)")
}

// 0x4ba310 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::doGetSingleton(void)
pub fn stub_4ba310() -> ! {
    todo!("0x4ba310 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::doGetSingleton(void)")
}

// 0x4ba400 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()
pub fn stub_4ba400() -> ! {
    todo!("0x4ba400 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()")
}

// 0x4ba404 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()
pub fn stub_4ba404() -> ! {
    todo!("0x4ba404 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()")
}

// 0x4ba5d8 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()
pub fn stub_4ba5d8() -> ! {
    todo!("0x4ba5d8 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()")
}

// 0x4ba678 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(char const*)const
pub fn stub_4ba678() -> ! {
    todo!("0x4ba678 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(char const*)const")
}

// 0x4ba6a8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4ba6a8() -> ! {
    todo!("0x4ba6a8 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4ba6c8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4ba6c8() -> ! {
    todo!("0x4ba6c8 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4ba724 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(unsigned long,std::string &)const
pub fn stub_4ba724() -> ! {
    todo!("0x4ba724 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(unsigned long,std::string &)const")
}

// 0x4ba868 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(RBX::DialogRoot::DialogTone const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(RBX::DialogRoot::DialogTone const&)const
pub fn stub_4ba868() -> ! {
    todo!("0x4ba868 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(RBX::DialogRoot::DialogTone const&)const")
}

// 0x4baa08 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)
pub fn stub_4baa08() -> ! {
    todo!("0x4baa08 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)")
}

// 0x4baa58 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)
pub fn stub_4baa58() -> ! {
    todo!("0x4baa58 rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)")
}

// 0x4baac4 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)
pub fn stub_4baac4() -> ! {
    todo!("0x4baac4 rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)")
}

// 0x4baad0 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)
pub fn stub_4baad0() -> ! {
    todo!("0x4baad0 rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)")
}

// 0x4baad4 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToItem(RBX::DialogRoot::DialogTone const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToItem(RBX::DialogRoot::DialogTone const&)const
pub fn stub_4baad4() -> ! {
    todo!("0x4baad4 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToItem(RBX::DialogRoot::DialogTone const&)const")
}

// 0x4baba0 — __ZN3rbx8any_castIRKN3RBX10DialogRoot10DialogToneENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4baba0() -> ! {
    todo!("0x4baba0 RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4bac90 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogTone&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogTone&)const
pub fn stub_4bac90() -> ! {
    todo!("0x4bac90 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogTone&)const")
}

// 0x4bad0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)
pub fn stub_4bad0c() -> ! {
    todo!("0x4bad0c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)")
}

// 0x4bad34 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::initSingleton(void)
pub fn stub_4bad34() -> ! {
    todo!("0x4bad34 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::initSingleton(void)")
}

// 0x4bad38 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::doGetSingleton(void)
pub fn stub_4bad38() -> ! {
    todo!("0x4bad38 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::doGetSingleton(void)")
}

// 0x4bae28 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()
pub fn stub_4bae28() -> ! {
    todo!("0x4bae28 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()")
}

// 0x4bae2c — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()
pub fn stub_4bae2c() -> ! {
    todo!("0x4bae2c RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()")
}

// 0x4bb000 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()
pub fn stub_4bb000() -> ! {
    todo!("0x4bb000 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()")
}

// 0x4bb0a0 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE6lookupEPKc
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(char const*)const
pub fn stub_4bb0a0() -> ! {
    todo!("0x4bb0a0 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(char const*)const")
}

// 0x4bb0d0 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4bb0d0() -> ! {
    todo!("0x4bb0d0 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4bb0f0 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4bb0f0() -> ! {
    todo!("0x4bb0f0 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4bb14c — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(unsigned long,std::string &)const
pub fn stub_4bb14c() -> ! {
    todo!("0x4bb14c RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(unsigned long,std::string &)const")
}

// 0x4bb290 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(RBX::DialogRoot::DialogPurpose const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(RBX::DialogRoot::DialogPurpose const&)const
pub fn stub_4bb290() -> ! {
    todo!("0x4bb290 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(RBX::DialogRoot::DialogPurpose const&)const")
}

// 0x4bb430 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)
pub fn stub_4bb430() -> ! {
    todo!("0x4bb430 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)")
}

// 0x4bb480 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)
pub fn stub_4bb480() -> ! {
    todo!("0x4bb480 rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)")
}

// 0x4bb4ec — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)
pub fn stub_4bb4ec() -> ! {
    todo!("0x4bb4ec rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)")
}

// 0x4bb4f8 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)
pub fn stub_4bb4f8() -> ! {
    todo!("0x4bb4f8 rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)")
}

// 0x4bb4fc — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToItem(RBX::DialogRoot::DialogPurpose const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToItem(RBX::DialogRoot::DialogPurpose const&)const
pub fn stub_4bb4fc() -> ! {
    todo!("0x4bb4fc RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToItem(RBX::DialogRoot::DialogPurpose const&)const")
}

// 0x4bb5c8 — __ZN3rbx8any_castIRKN3RBX10DialogRoot13DialogPurposeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bb5c8() -> ! {
    todo!("0x4bb5c8 RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4bb6b8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogPurpose&)const")]
// was: RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogPurpose&)const
pub fn stub_4bb6b8() -> ! {
    todo!("0x4bb6b8 RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogPurpose&)const")
}

// 0x4bb734 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)
pub fn stub_4bb734() -> ! {
    todo!("0x4bb734 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)")
}

// 0x4bb75c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::initSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::initSingleton(void)
pub fn stub_4bb75c() -> ! {
    todo!("0x4bb75c RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::initSingleton(void)")
}

// 0x4bb760 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::doGetSingleton(void)
pub fn stub_4bb760() -> ! {
    todo!("0x4bb760 RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::doGetSingleton(void)")
}

// 0x4bb850 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()
pub fn stub_4bb850() -> ! {
    todo!("0x4bb850 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")
}

// 0x4bb854 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()
pub fn stub_4bb854() -> ! {
    todo!("0x4bb854 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")
}

// 0x4bba28 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED0Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()
pub fn stub_4bba28() -> ! {
    todo!("0x4bba28 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")
}

// 0x4bbac8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupEPKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(char const*)const")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(char const*)const
pub fn stub_4bbac8() -> ! {
    todo!("0x4bbac8 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(char const*)const")
}

// 0x4bbaf8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(RBX::Reflection::Variant const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(RBX::Reflection::Variant const&)const
pub fn stub_4bbaf8() -> ! {
    todo!("0x4bbaf8 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(RBX::Reflection::Variant const&)const")
}

// 0x4bbb18 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
pub fn stub_4bbb18() -> ! {
    todo!("0x4bbb18 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0x4bbb74 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringEmRSs
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(unsigned long,std::string &)const")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(unsigned long,std::string &)const
pub fn stub_4bbb74() -> ! {
    todo!("0x4bbb74 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(unsigned long,std::string &)const")
}

// 0x4bbcb8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(RBX::GuiButton::Style const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(RBX::GuiButton::Style const&)const
pub fn stub_4bbcb8() -> ! {
    todo!("0x4bbcb8 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(RBX::GuiButton::Style const&)const")
}

// 0x4bbe58 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)
pub fn stub_4bbe58() -> ! {
    todo!("0x4bbe58 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)")
}

// 0x4bbea8 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)
pub fn stub_4bbea8() -> ! {
    todo!("0x4bbea8 rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)")
}

// 0x4bbf14 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE14construct_funcEPKcPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)")]
// was: rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)
pub fn stub_4bbf14() -> ! {
    todo!("0x4bbf14 rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)")
}

// 0x4bbf20 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)
pub fn stub_4bbf20() -> ! {
    todo!("0x4bbf20 rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)")
}

// 0x4bbf24 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToItem(RBX::GuiButton::Style const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToItem(RBX::GuiButton::Style const&)const
pub fn stub_4bbf24() -> ! {
    todo!("0x4bbf24 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToItem(RBX::GuiButton::Style const&)const")
}

// 0x4bbff0 — __ZN3rbx8any_castIRKN3RBX9GuiButton5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_4bbff0() -> ! {
    todo!("0x4bbff0 RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x4bc0e0 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(RBX::Name const&,RBX::GuiButton::Style&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(RBX::Name const&,RBX::GuiButton::Style&)const
pub fn stub_4bc0e0() -> ! {
    todo!("0x4bc0e0 RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(RBX::Name const&,RBX::GuiButton::Style&)const")
}

// 0x4bc15c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)
pub fn stub_4bc15c() -> ! {
    todo!("0x4bc15c std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)")
}
