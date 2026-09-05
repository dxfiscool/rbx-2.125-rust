// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler
// Filter: Script|Lua|Yield|CodeGen|Luau (4818 filtered, all already stubbed in crates/script/src) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4c3e58..0x4c76bc | EA-sorted asc distinct not yet in script (remaining 56950->56850, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x4c3e58 — __ZN3rbx8any_castIRKN3RBX7Handles11VisualStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::Handles::VisualStyle const& rbx::any_cast<RBX::Handles::VisualStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::Handles::VisualStyle const& rbx::any_cast<RBX::Handles::VisualStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX7Handles11VisualStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c3e58(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4c3f48 — __ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToValue(RBX::Name const&,RBX::Handles::VisualStyle&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToValue(RBX::Name const&,RBX::Handles::VisualStyle&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4c3f48(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Handles::VisualStyle>::convertToValue(RBX::Name const&, RBX~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c3fc4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Handles::VisualStyle>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Handles11VisualStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c3fc4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4c3fec — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE13initSingletonEv")]
pub fn stub_0x4c3fec(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c3ff0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE14doGetSingletonEv")]
pub fn stub_0x4c3ff0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType> ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c40e0 — __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED1Ev — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED1Ev")]
pub fn stub_0x4c40e0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c40e4 — __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED2Ev — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc() [0x4c40e4]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED2Ev")]
pub fn stub_0x4c40e4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c42b8 — __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED0Ev — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::~EnumDesc() [0x4c42b8]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED0Ev")]
pub fn stub_0x4c42b8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c4358 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupEPKc")]
pub fn stub_0x4c4358(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4388 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4c4388(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::lookup(RBX::Reflection::Va~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c43a8 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4c43a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(unsigned lo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4404 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringEmRSs")]
pub fn stub_0x4c4404(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(unsigned l~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4548 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(RBX::FriendService::FriendEventType const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(RBX::FriendService::FriendEventType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE15convertToStringERKS3_")]
pub fn stub_0x4c4548(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToString(RBX::Frien~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c46e8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendEventType>(RBX::FriendService::FriendEventType const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendEventType>(RBX::FriendService::FriendEventType const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService15FriendEventTypeEEERS3_RKT_")]
pub fn stub_0x4c46e8() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4c4738 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE9singletonEv")]
pub fn stub_0x4c4738(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c47a4 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToItem(RBX::FriendService::FriendEventType const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToItem(RBX::FriendService::FriendEventType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE13convertToItemERKS3_")]
pub fn stub_0x4c47a4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToItem(RBX::FriendS~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4870 — __ZN3rbx8any_castIRKN3RBX13FriendService15FriendEventTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::FriendService::FriendEventType const& rbx::any_cast<RBX::FriendService::FriendEventType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::FriendService::FriendEventType const& rbx::any_cast<RBX::FriendService::FriendEventType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13FriendService15FriendEventTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c4870(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4c4960 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(RBX::Name const&,RBX::FriendService::FriendEventType&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(RBX::Name const&,RBX::FriendService::FriendEventType&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4c4960(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendEventType>::convertToValue(RBX::Name c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c49dc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendEventType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService15FriendEventTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c49dc(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4c4a04 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE13initSingletonEv")]
pub fn stub_0x4c4a04(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4a08 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE14doGetSingletonEv")]
pub fn stub_0x4c4a08(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4af8 — __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED1Ev — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED1Ev")]
pub fn stub_0x4c4af8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c4afc — __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED2Ev — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc() [0x4c4afc]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED2Ev")]
pub fn stub_0x4c4afc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c4cd0 — __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED0Ev — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::~EnumDesc() [0x4c4cd0]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED0Ev")]
pub fn stub_0x4c4cd0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c4d70 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupEPKc")]
pub fn stub_0x4c4d70(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4da0 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4c4da0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::lookup(RBX::Reflection::Varia~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4dc0 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4c4dc0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(unsigned long,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4e1c — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringEmRSs")]
pub fn stub_0x4c4e1c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(unsigned long~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c4f60 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(RBX::FriendService::FriendStatus const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(RBX::FriendService::FriendStatus const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE15convertToStringERKS3_")]
pub fn stub_0x4c4f60(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToString(RBX::FriendSe~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5100 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendStatus>(RBX::FriendService::FriendStatus const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::FriendService::FriendStatus>(RBX::FriendService::FriendStatus const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13FriendService12FriendStatusEEERS3_RKT_")]
pub fn stub_0x4c5100() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4c5150 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE9singletonEv — rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE9singletonEv")]
pub fn stub_0x4c5150(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c51bc — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE14construct_funcEPKcPc")]
pub fn stub_0x4c51bc(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::construct_func(char c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c51c8 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13FriendService12FriendStatusEE13destruct_funcEPc")]
pub fn stub_0x4c51c8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::FriendService::FriendStatus>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c51cc — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToItem(RBX::FriendService::FriendStatus const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToItem(RBX::FriendService::FriendStatus const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE13convertToItemERKS3_")]
pub fn stub_0x4c51cc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToItem(RBX::FriendServ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5298 — __ZN3rbx8any_castIRKN3RBX13FriendService12FriendStatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::FriendService::FriendStatus const& rbx::any_cast<RBX::FriendService::FriendStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::FriendService::FriendStatus const& rbx::any_cast<RBX::FriendService::FriendStatus const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13FriendService12FriendStatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c5298(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4c5388 — __ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(RBX::Name const&,RBX::FriendService::FriendStatus&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(RBX::Name const&,RBX::FriendService::FriendStatus&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4c5388(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::FriendService::FriendStatus>::convertToValue(RBX::Name cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5404 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::FriendService::FriendStatus>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13FriendService12FriendStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c5404(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4c542c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE13initSingletonEv")]
pub fn stub_0x4c542c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5430 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE14doGetSingletonEv")]
pub fn stub_0x4c5430(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum> c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5520 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED1Ev — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED1Ev")]
pub fn stub_0x4c5520(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c5524 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED2Ev — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc() [0x4c5524]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED2Ev")]
pub fn stub_0x4c5524(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c56f8 — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED0Ev — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::~EnumDesc() [0x4c56f8]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED0Ev")]
pub fn stub_0x4c56f8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c5798 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupEPKc")]
pub fn stub_0x4c5798(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c57c8 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4c57c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::lookup(RBX::Reflection::Var~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c57e8 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4c57e8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(unsigned lon~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5844 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringEmRSs")]
pub fn stub_0x4c5844(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(unsigned lo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5988 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(RBX::PyramidInstance::NumSidesEnum const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(RBX::PyramidInstance::NumSidesEnum const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE15convertToStringERKS3_")]
pub fn stub_0x4c5988(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToString(RBX::Pyrami~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5b28 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15PyramidInstance12NumSidesEnumEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PyramidInstance::NumSidesEnum>(RBX::PyramidInstance::NumSidesEnum const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PyramidInstance::NumSidesEnum>(RBX::PyramidInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_15PyramidInstance12NumSidesEnumEEERS3_RKT_")]
pub fn stub_0x4c5b28() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4c5b78 — __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE9singletonEv — rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE9singletonEv")]
pub fn stub_0x4c5b78(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5be4 — __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE14construct_funcEPKcPc")]
pub fn stub_0x4c5be4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::construct_func(char~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5bf0 — __ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX15PyramidInstance12NumSidesEnumEE13destruct_funcEPc")]
pub fn stub_0x4c5bf0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PyramidInstance::NumSidesEnum>::destruct_func(char*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5bf4 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToItem(RBX::PyramidInstance::NumSidesEnum const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToItem(RBX::PyramidInstance::NumSidesEnum const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE13convertToItemERKS3_")]
pub fn stub_0x4c5bf4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToItem(RBX::PyramidI~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5cc0 — __ZN3rbx8any_castIRKN3RBX15PyramidInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::PyramidInstance::NumSidesEnum const& rbx::any_cast<RBX::PyramidInstance::NumSidesEnum const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::PyramidInstance::NumSidesEnum const& rbx::any_cast<RBX::PyramidInstance::NumSidesEnum const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX15PyramidInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c5cc0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4c5db0 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(RBX::Name const&,RBX::PyramidInstance::NumSidesEnum&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(RBX::Name const&,RBX::PyramidInstance::NumSidesEnum&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4c5db0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToValue(RBX::Name co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5e2c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PyramidInstance::NumSidesEnum>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15PyramidInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c5e2c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4c5e54 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE13initSingletonEv")]
pub fn stub_0x4c5e54(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5e58 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE14doGetSingletonEv")]
pub fn stub_0x4c5e58(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c5f48 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED1Ev — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED1Ev")]
pub fn stub_0x4c5f48(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c5f4c — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED2Ev — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc() [0x4c5f4c]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED2Ev")]
pub fn stub_0x4c5f4c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c6120 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED0Ev — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::~EnumDesc() [0x4c6120]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED0Ev")]
pub fn stub_0x4c6120(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c61c0 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupEPKc")]
pub fn stub_0x4c61c0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c61f0 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4c61f0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::lookup(RBX::Reflection::Varia~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6210 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4c6210(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(unsigned long,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c626c — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringEmRSs")]
pub fn stub_0x4c626c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(unsigned long~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c63b0 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(RBX::PrismInstance::NumSidesEnum const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(RBX::PrismInstance::NumSidesEnum const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE15convertToStringERKS3_")]
pub fn stub_0x4c63b0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToString(RBX::PrismIns~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6550 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13PrismInstance12NumSidesEnumEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PrismInstance::NumSidesEnum>(RBX::PrismInstance::NumSidesEnum const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::PrismInstance::NumSidesEnum>(RBX::PrismInstance::NumSidesEnum const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13PrismInstance12NumSidesEnumEEERS3_RKT_")]
pub fn stub_0x4c6550() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4c65a0 — __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE9singletonEv — rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE9singletonEv")]
pub fn stub_0x4c65a0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c660c — __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE14construct_funcEPKcPc")]
pub fn stub_0x4c660c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::construct_func(char c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6618 — __ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13PrismInstance12NumSidesEnumEE13destruct_funcEPc")]
pub fn stub_0x4c6618(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::PrismInstance::NumSidesEnum>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c661c — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToItem(RBX::PrismInstance::NumSidesEnum const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToItem(RBX::PrismInstance::NumSidesEnum const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE13convertToItemERKS3_")]
pub fn stub_0x4c661c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToItem(RBX::PrismInsta~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c66e8 — __ZN3rbx8any_castIRKN3RBX13PrismInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::PrismInstance::NumSidesEnum const& rbx::any_cast<RBX::PrismInstance::NumSidesEnum const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::PrismInstance::NumSidesEnum const& rbx::any_cast<RBX::PrismInstance::NumSidesEnum const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13PrismInstance12NumSidesEnumENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c66e8(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4c67d8 — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(RBX::Name const&,RBX::PrismInstance::NumSidesEnum&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(RBX::Name const&,RBX::PrismInstance::NumSidesEnum&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4c67d8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToValue(RBX::Name cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6854 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>,std::_Select1st<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::PrismInstance::NumSidesEnum>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13PrismInstance12NumSidesEnumEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c6854(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4c687c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE13initSingletonEv")]
pub fn stub_0x4c687c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrus~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6880 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE14doGetSingletonEv")]
pub fn stub_0x4c6880(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrus~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6970 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED1Ev — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED1Ev")]
pub fn stub_0x4c6970(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c6974 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED2Ev — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc() [0x4c6974]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED2Ev")]
pub fn stub_0x4c6974(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c6b48 — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED0Ev — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::~EnumDesc() [0x4c6b48]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED0Ev")]
pub fn stub_0x4c6b48(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c6be8 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupEPKc")]
pub fn stub_0x4c6be8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(char const*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6c18 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4c6c18(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::lookup(RBX::Reflec~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6c38 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4c6c38(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(uns~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6c94 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringEmRSs")]
pub fn stub_0x4c6c94(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(un~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6dd8 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE15convertToStringERKS3_")]
pub fn stub_0x4c6dd8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToString(RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c6f78 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ExtrudedPartInstance16VisualTrussStyleEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ExtrudedPartInstance::VisualTrussStyle>(RBX::ExtrudedPartInstance::VisualTrussStyle const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ExtrudedPartInstance::VisualTrussStyle>(RBX::ExtrudedPartInstance::VisualTrussStyle const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_20ExtrudedPartInstance16VisualTrussStyleEEERS3_RKT_")]
pub fn stub_0x4c6f78() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4c6fc8 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE9singletonEv — rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE9singletonEv")]
pub fn stub_0x4c6fc8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::singleton(~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c7034 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE14construct_funcEPKcPc")]
pub fn stub_0x4c7034(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::construct_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c7040 — __ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX20ExtrudedPartInstance16VisualTrussStyleEE13destruct_funcEPc")]
pub fn stub_0x4c7040(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::ExtrudedPartInstance::VisualTrussStyle>::destruct_f~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c7044 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToItem(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToItem(RBX::ExtrudedPartInstance::VisualTrussStyle const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE13convertToItemERKS3_")]
pub fn stub_0x4c7044(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToItem(RBX:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c7110 — __ZN3rbx8any_castIRKN3RBX20ExtrudedPartInstance16VisualTrussStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::ExtrudedPartInstance::VisualTrussStyle const& rbx::any_cast<RBX::ExtrudedPartInstance::VisualTrussStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::ExtrudedPartInstance::VisualTrussStyle const& rbx::any_cast<RBX::ExtrudedPartInstance::VisualTrussStyle const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX20ExtrudedPartInstance16VisualTrussStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4c7110(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4c7200 — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(RBX::Name const&,RBX::ExtrudedPartInstance::VisualTrussStyle&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(RBX::Name const&,RBX::ExtrudedPartInstance::VisualTrussStyle&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4c7200(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::ExtrudedPartInstance::VisualTrussStyle>::convertToValue(RBX~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c727c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ExtrudedPartInstance::VisualTrussStyle>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ExtrudedPartInstance16VisualTrussStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4c727c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4c72a4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE13initSingletonEv")]
pub fn stub_0x4c72a4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::Privilege~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c72a8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::doGetSingleton(void)
// type: void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE14doGetSingletonEv")]
pub fn stub_0x4c72a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::PersonalServerService::Privilege~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c7398 — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED1Ev — RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED1Ev")]
pub fn stub_0x4c7398(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c739c — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED2Ev — RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc() [0x4c739c]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED2Ev")]
pub fn stub_0x4c739c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c7570 — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED0Ev — RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc()
// type: void __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::~EnumDesc() [0x4c7570]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED0Ev")]
pub fn stub_0x4c7570(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4c7610 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(char const*)const
// type: int __fastcall(int, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupEPKc")]
pub fn stub_0x4c7610(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(char const*) ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c7640 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4c7640(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::lookup(RBX::Reflecti~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c7660 — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4c7660(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToValue(unsig~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4c76bc — __ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEE15convertToStringEmRSs")]
pub fn stub_0x4c76bc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::PersonalServerService::PrivilegeType>::convertToString(unsi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}
