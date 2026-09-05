// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler
// Filter: Script|Lua|Yield|CodeGen|Luau (4818 filtered, all already stubbed in crates/script/src) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4b9680..0x4bce78 | EA-sorted asc distinct not yet in script (remaining 57250->57150, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x4b9680 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5Voxel9CellBlockEE13destruct_funcEPc")]
pub fn stub_0x4b9680(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Voxel::CellBlock>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b9684 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToItem(RBX::Voxel::CellBlock const&)const
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToItem(RBX::Voxel::CellBlock const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE13convertToItemERKS3_")]
pub fn stub_0x4b9684(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToItem(RBX::Voxel::CellBlock cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b9750 — __ZN3rbx8any_castIRKN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::Voxel::CellBlock const& rbx::any_cast<RBX::Voxel::CellBlock const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX5Voxel9CellBlockENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4b9750(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4b9840 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(RBX::Name const&,RBX::Voxel::CellBlock&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(RBX::Name const&,RBX::Voxel::CellBlock&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4b9840(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellBlock>::convertToValue(RBX::Name const&, RBX::Vo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b98bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellBlock>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel9CellBlockEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4b98bc(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4b98e4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE13initSingletonEv")]
pub fn stub_0x4b98e4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::ini~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b98e8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE14doGetSingletonEv")]
pub fn stub_0x4b98e8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial> const>::doG~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b99d8 — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED1Ev — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED1Ev")]
pub fn stub_0x4b99d8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b99dc — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED2Ev — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc() [0x4b99dc]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED2Ev")]
pub fn stub_0x4b99dc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b9bb0 — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED0Ev — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::~EnumDesc() [0x4b9bb0]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED0Ev")]
pub fn stub_0x4b9bb0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b9c50 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE6lookupEPKc")]
pub fn stub_0x4b9c50(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b9c80 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4b9c80(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::lookup(RBX::Reflection::Variant const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b9ca0 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4b9ca0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(unsigned long, RBX::Re~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b9cfc — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE15convertToStringEmRSs")]
pub fn stub_0x4b9cfc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(unsigned long, std::s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b9e40 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(RBX::Voxel::CellMaterial const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(RBX::Voxel::CellMaterial const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE15convertToStringERKS3_")]
pub fn stub_0x4b9e40(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToString(RBX::Voxel::CellMater~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b9fe0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Voxel::CellMaterial>(RBX::Voxel::CellMaterial const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Voxel12CellMaterialEEERS3_RKT_")]
pub fn stub_0x4b9fe0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4ba030 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE9singletonEv")]
pub fn stub_0x4ba030(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba09c — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE14construct_funcEPKcPc")]
pub fn stub_0x4ba09c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::construct_func(char const*, c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba0a8 — __ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5Voxel12CellMaterialEE13destruct_funcEPc")]
pub fn stub_0x4ba0a8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Voxel::CellMaterial>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba0ac — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToItem(RBX::Voxel::CellMaterial const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToItem(RBX::Voxel::CellMaterial const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE13convertToItemERKS3_")]
pub fn stub_0x4ba0ac(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToItem(RBX::Voxel::CellMateria~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba178 — __ZN3rbx8any_castIRKN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::Voxel::CellMaterial const& rbx::any_cast<RBX::Voxel::CellMaterial const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX5Voxel12CellMaterialENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4ba178(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4ba268 — __ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(RBX::Name const&,RBX::Voxel::CellMaterial&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(RBX::Name const&,RBX::Voxel::CellMaterial&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4ba268(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Voxel::CellMaterial>::convertToValue(RBX::Name const&, RBX:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba2e4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Voxel::CellMaterial>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Voxel12CellMaterialEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4ba2e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4ba30c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE13initSingletonEv")]
pub fn stub_0x4ba30c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba310 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE14doGetSingletonEv")]
pub fn stub_0x4ba310(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone> const>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba400 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED1Ev — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED1Ev")]
pub fn stub_0x4ba400(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4ba404 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED2Ev — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc() [0x4ba404]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED2Ev")]
pub fn stub_0x4ba404(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4ba5d8 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED0Ev — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::~EnumDesc() [0x4ba5d8]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED0Ev")]
pub fn stub_0x4ba5d8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4ba678 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE6lookupEPKc")]
pub fn stub_0x4ba678(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba6a8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4ba6a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::lookup(RBX::Reflection::Variant co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba6c8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4ba6c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(unsigned long, RBX:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba724 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE15convertToStringEmRSs")]
pub fn stub_0x4ba724(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(unsigned long, std~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4ba868 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(RBX::DialogRoot::DialogTone const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(RBX::DialogRoot::DialogTone const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE15convertToStringERKS3_")]
pub fn stub_0x4ba868(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToString(RBX::DialogRoot::D~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4baa08 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogTone>(RBX::DialogRoot::DialogTone const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot10DialogToneEEERS3_RKT_")]
pub fn stub_0x4baa08() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4baa58 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE9singletonEv")]
pub fn stub_0x4baa58(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4baac4 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE14construct_funcEPKcPc")]
pub fn stub_0x4baac4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::construct_func(char const*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4baad0 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot10DialogToneEE13destruct_funcEPc")]
pub fn stub_0x4baad0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DialogRoot::DialogTone>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4baad4 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToItem(RBX::DialogRoot::DialogTone const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToItem(RBX::DialogRoot::DialogTone const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE13convertToItemERKS3_")]
pub fn stub_0x4baad4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToItem(RBX::DialogRoot::Dia~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4baba0 — __ZN3rbx8any_castIRKN3RBX10DialogRoot10DialogToneENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::DialogRoot::DialogTone const& rbx::any_cast<RBX::DialogRoot::DialogTone const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX10DialogRoot10DialogToneENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4baba0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4bac90 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogTone&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogTone&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4bac90(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToValue(RBX::Name const&, R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bad0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot10DialogToneEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4bad0c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4bad34 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE13initSingletonEv")]
pub fn stub_0x4bad34(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bad38 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE14doGetSingletonEv")]
pub fn stub_0x4bad38(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose> const~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bae28 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED1Ev — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED1Ev")]
pub fn stub_0x4bae28(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bae2c — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED2Ev — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc() [0x4bae2c]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED2Ev")]
pub fn stub_0x4bae2c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bb000 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED0Ev — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::~EnumDesc() [0x4bb000]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED0Ev")]
pub fn stub_0x4bb000(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bb0a0 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(char const*)const
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE6lookupEPKc")]
pub fn stub_0x4bb0a0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb0d0 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4bb0d0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::lookup(RBX::Reflection::Variant~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb0f0 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4bb0f0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(unsigned long, R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb14c — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE15convertToStringEmRSs")]
pub fn stub_0x4bb14c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(unsigned long, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb290 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(RBX::DialogRoot::DialogPurpose const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(RBX::DialogRoot::DialogPurpose const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE15convertToStringERKS3_")]
pub fn stub_0x4bb290(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToString(RBX::DialogRoot~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb430 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::DialogRoot::DialogPurpose>(RBX::DialogRoot::DialogPurpose const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10DialogRoot13DialogPurposeEEERS3_RKT_")]
pub fn stub_0x4bb430() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4bb480 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE9singletonEv")]
pub fn stub_0x4bb480(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb4ec — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE14construct_funcEPKcPc")]
pub fn stub_0x4bb4ec(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::construct_func(char con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb4f8 — __ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX10DialogRoot13DialogPurposeEE13destruct_funcEPc")]
pub fn stub_0x4bb4f8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::DialogRoot::DialogPurpose>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb4fc — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToItem(RBX::DialogRoot::DialogPurpose const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToItem(RBX::DialogRoot::DialogPurpose const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE13convertToItemERKS3_")]
pub fn stub_0x4bb4fc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToItem(RBX::DialogRoot::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb5c8 — __ZN3rbx8any_castIRKN3RBX10DialogRoot13DialogPurposeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::DialogRoot::DialogPurpose const& rbx::any_cast<RBX::DialogRoot::DialogPurpose const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX10DialogRoot13DialogPurposeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4bb5c8(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4bb6b8 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogPurpose&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(RBX::Name const&,RBX::DialogRoot::DialogPurpose&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4bb6b8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToValue(RBX::Name const&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb734 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogPurpose>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10DialogRoot13DialogPurposeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4bb734(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4bb75c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE13initSingletonEv")]
pub fn stub_0x4bb75c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::initSi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb760 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE14doGetSingletonEv")]
pub fn stub_0x4bb760(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GuiButton::Style> const>::doGetS~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bb850 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED1Ev — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED1Ev")]
pub fn stub_0x4bb850(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bb854 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED2Ev — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc() [0x4bb854]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED2Ev")]
pub fn stub_0x4bb854(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bba28 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED0Ev — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::~EnumDesc() [0x4bba28]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED0Ev")]
pub fn stub_0x4bba28(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bbac8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupEPKc")]
pub fn stub_0x4bbac8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbaf8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4bbaf8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::lookup(RBX::Reflection::Variant const&) ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbb18 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4bbb18(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(unsigned long, RBX::Refle~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbb74 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringEmRSs")]
pub fn stub_0x4bbb74(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(unsigned long, std::stri~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbcb8 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(RBX::GuiButton::Style const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(RBX::GuiButton::Style const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE15convertToStringERKS3_")]
pub fn stub_0x4bbcb8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToString(RBX::GuiButton::Style co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbe58 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiButton::Style>(RBX::GuiButton::Style const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9GuiButton5StyleEEERS3_RKT_")]
pub fn stub_0x4bbe58() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4bbea8 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE9singletonEv — rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE9singletonEv")]
pub fn stub_0x4bbea8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiButton::Style>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbf14 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE14construct_funcEPKcPc")]
pub fn stub_0x4bbf14(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiButton::Style>::construct_func(char const*, char~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbf20 — __ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9GuiButton5StyleEE13destruct_funcEPc")]
pub fn stub_0x4bbf20(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::GuiButton::Style>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbf24 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToItem(RBX::GuiButton::Style const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToItem(RBX::GuiButton::Style const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE13convertToItemERKS3_")]
pub fn stub_0x4bbf24(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToItem(RBX::GuiButton::Style cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bbff0 — __ZN3rbx8any_castIRKN3RBX9GuiButton5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::GuiButton::Style const& rbx::any_cast<RBX::GuiButton::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9GuiButton5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4bbff0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4bc0e0 — __ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(RBX::Name const&,RBX::GuiButton::Style&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(RBX::Name const&,RBX::GuiButton::Style&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9GuiButton5StyleEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4bc0e0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::GuiButton::Style>::convertToValue(RBX::Name const&, RBX::Gu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc15c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiButton::Style>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4bc15c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4bc184 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE13initSingletonEv")]
pub fn stub_0x4bc184(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::initSingle~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc188 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE14doGetSingletonEv")]
pub fn stub_0x4bc188(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Frame::Style> const>::doGetSingl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc278 — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED1Ev — RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED1Ev")]
pub fn stub_0x4bc278(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bc27c — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED2Ev — RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc() [0x4bc27c]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED2Ev")]
pub fn stub_0x4bc27c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bc450 — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED0Ev — RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::~EnumDesc() [0x4bc450]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED0Ev")]
pub fn stub_0x4bc450(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bc4f0 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupEPKc")]
pub fn stub_0x4bc4f0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc520 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4bc520(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::lookup(RBX::Reflection::Variant const&) cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc540 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4bc540(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(unsigned long, RBX::Reflectio~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc59c — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringEmRSs")]
pub fn stub_0x4bc59c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(unsigned long, std::string&)~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc6e0 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(RBX::Frame::Style const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(RBX::Frame::Style const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE15convertToStringERKS3_")]
pub fn stub_0x4bc6e0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToString(RBX::Frame::Style const&) co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc880 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Frame::Style>(RBX::Frame::Style const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Frame::Style>(RBX::Frame::Style const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_5Frame5StyleEEERS3_RKT_")]
pub fn stub_0x4bc880() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4bc8d0 — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv — rbx::implementation::typed_holder<RBX::Frame::Style>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE9singletonEv")]
pub fn stub_0x4bc8d0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Frame::Style>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc93c — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::Frame::Style>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE14construct_funcEPKcPc")]
pub fn stub_0x4bc93c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Frame::Style>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc948 — __ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::Frame::Style>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Frame::Style>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX5Frame5StyleEE13destruct_funcEPc")]
pub fn stub_0x4bc948(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::Frame::Style>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bc94c — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToItem(RBX::Frame::Style const&)const
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToItem(RBX::Frame::Style const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE13convertToItemERKS3_")]
pub fn stub_0x4bc94c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToItem(RBX::Frame::Style const&) cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bca18 — __ZN3rbx8any_castIRKN3RBX5Frame5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Frame::Style const& rbx::any_cast<RBX::Frame::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::Frame::Style const& rbx::any_cast<RBX::Frame::Style const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX5Frame5StyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4bca18(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4bcb08 — __ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(RBX::Name const&,RBX::Frame::Style&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(RBX::Name const&,RBX::Frame::Style&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_5Frame5StyleEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4bcb08(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Frame::Style>::convertToValue(RBX::Name const&, RBX::Frame:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bcb84 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Frame::Style>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Frame::Style>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4bcb84(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4bcbac — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE13initSingletonEv")]
pub fn stub_0x4bcbac(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQuality~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bcbb0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv")]
pub fn stub_0x4bcbb0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQuality~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4bcca0 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED1Ev — RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED1Ev")]
pub fn stub_0x4bcca0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bcca4 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev — RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc() [0x4bcca4]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev")]
pub fn stub_0x4bcca4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4bce78 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED0Ev — RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc() [0x4bce78]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED0Ev")]
pub fn stub_0x4bce78(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}
