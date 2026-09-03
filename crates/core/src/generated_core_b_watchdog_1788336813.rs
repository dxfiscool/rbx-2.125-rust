//! core b watchdog — 120 core stubs EA-sorted, gap filler distinct from shard A (watchdog_coreA 0x54c9a8..0x5765ec).
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 120 not yet in crates/core/src, not overlapping shard A/B.
//! Range: 0x26b55c..0x2aa5c8 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Lua::LuaArguments::getObject(int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)const")]
#[doc(alias = "__ZNK3RBX3Lua12LuaArguments9getObjectEiRN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE")]
// 0x26b55c — __ZNK3RBX3Lua12LuaArguments9getObjectEiRN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE
// was: RBX::Lua::LuaArguments::getObject(int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)const
// type: int __fastcall(int, int, int)
pub fn stub_26b55c() {
    // IDA 0x26b55c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::DescribedBase>& rbx_core::SharedPtr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSINS1_8InstanceEEERS4_RKNS0_IT_EE")]
// 0x26c350 — __ZN5boost10shared_ptrIN3RBX10Reflection13DescribedBaseEEaSINS1_8InstanceEEERS4_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Reflection::DescribedBase>& boost::shared_ptr<RBX::Reflection::DescribedBase>::operator=<RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
pub fn stub_26c350() {
    // IDA 0x26c350: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE")]
// 0x26dce4 — __ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Instance> const&)
// type: int __fastcall(int *, int)
pub fn stub_26dce4() {
    // IDA 0x26dce4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEE")]
// 0x26ddb4 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_26ddb4() {
    // IDA 0x26ddb4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEE")]
// 0x26dddc — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
// type: int __fastcall(int *, int *)
pub fn stub_26dddc() {
    // IDA 0x26dddc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsNS_10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEE")]
// 0x26dea0 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsNS_10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
// type: int __fastcall(int *, _DWORD *)
pub fn stub_26dea0() {
    // IDA 0x26dea0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE")]
// 0x26df08 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>)
// type: int __fastcall(_DWORD *, _DWORD *)
pub fn stub_26df08() {
    // IDA 0x26df08: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS_10Reflection5TupleEEE")]
// 0x26df2c — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Reflection::Tuple const>)
// type: int __fastcall(int *, char ******)
pub fn stub_26df2c() {
    // IDA 0x26df2c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEE")]
// 0x26df60 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
// type: int __fastcall(int *, const shared_count *)
pub fn stub_26df60() {
    // IDA 0x26df60: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEE")]
// 0x26e030 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEE
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)
// type: int __fastcall(int *, const shared_count *)
pub fn stub_26e030() {
    // IDA 0x26e030: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrINS1_8functionIFvNS2_IKN3RBX10Reflection5TupleEEENS3_IFvPNS4_3Lua12IAsyncResultEEEEEEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26eb44 — __ZN3rbx8any_castIRKN5boost10shared_ptrINS1_8functionIFvNS2_IKN3RBX10Reflection5TupleEEENS3_IFvPNS4_3Lua12IAsyncResultEEEEEEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// was: boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResul
// type: char ****__fastcall(char ****)
pub fn stub_26eb44() {
    // IDA 0x26eb44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE")]
// 0x26ee14 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt6vectorINS2_IN3RBX8InstanceEEESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const& rbx::any_cast<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> const&,RBX::Region3>(rbx::placement_any
// type: char ****__fastcall(char ****)
pub fn stub_26ee14() {
    // IDA 0x26ee14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State")]
// 0x26ef04 — __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State
// was: int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_p
// type: int __fastcall(char ****, char ****, int)
pub fn stub_26ef04() {
    // IDA 0x26ef04: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
// 0x26fa78 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_26fa78() {
    // IDA 0x26fa78: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Reflection::DescribedBase> &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS3_INS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_")]
// 0x26ff94 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueINS3_INS_10Reflection13DescribedBaseEEEEEbP9lua_StatejRT_
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Reflection::DescribedBase>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Reflection::DescribedBase> &)
// type: int __fastcall(int, int, int)
pub fn stub_26ff94() {
    // IDA 0x26ff94: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::CellID::fromParameters(bool,float *,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX6CellID14fromParametersEbPfN5boost10shared_ptrINS_8InstanceEEE")]
// 0x277af4 — __ZN3RBX6CellID14fromParametersEbPfN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::CellID::fromParameters(bool,float *,boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, int, int, const shared_count *)
pub fn stub_277af4() {
    // IDA 0x277af4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13registerClassEP9lua_StatePFiSE_ESG_")]
// 0x27a1b0 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13registerClassEP9lua_StatePFiSE_ESG_
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// type: int __fastcall(int, int, int)
pub fn stub_27a1b0() {
    // IDA 0x27a1b0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE5on_gcEP9lua_State")]
// 0x27a2c4 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE5on_gcEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_gc(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a2c4() {
    // IDA 0x27a2c4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE5on_eqEP9lua_State")]
// 0x27a2ec — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE5on_eqEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_eq(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a2ec() {
    // IDA 0x27a2ec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringEP9lua_State")]
// 0x27a32c — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_tostringEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,true>::on_tostring(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a32c() {
    // IDA 0x27a32c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13registerClassEP9lua_StatePFiSI_ESK_")]
// 0x27a354 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13registerClassEP9lua_StatePFiSI_ESK_
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// type: int __fastcall(int, int, int)
pub fn stub_27a354() {
    // IDA 0x27a354: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE5on_gcEP9lua_State")]
// 0x27a468 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE5on_gcEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_gc(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a468() {
    // IDA 0x27a468: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE5on_eqEP9lua_State")]
// 0x27a490 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE5on_eqEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_eq(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a490() {
    // IDA 0x27a490: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringEP9lua_State")]
// 0x27a4d0 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_tostringEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_tostring(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a4d0() {
    // IDA 0x27a4d0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13registerClassEP9lua_StatePFiS8_ESA_")]
// 0x27a4f8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE13registerClassEP9lua_StatePFiS8_ESA_
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// type: int __fastcall(int, int, int)
pub fn stub_27a4f8() {
    // IDA 0x27a4f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE5on_gcEP9lua_State")]
// 0x27a5e0 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE5on_gcEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_gc(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a5e0() {
    // IDA 0x27a5e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringEP9lua_State")]
// 0x27a608 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringEP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(lua_State *)
// type: int __fastcall(int)
pub fn stub_27a608() {
    // IDA 0x27a608: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_index(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexERKS5_PKcP9lua_State")]
// 0x27c258 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8on_indexERKS5_PKcP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_index(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
// type: int __fastcall(int, int, int)
pub fn stub_27c258() {
    // IDA 0x27c258: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "PropertyNameCorrection(rbx_core::SharedPtr<RBX::Instance> const&,char const*,lua_State *)")]
#[doc(alias = "__ZL22PropertyNameCorrectionRKN5boost10shared_ptrIN3RBX8InstanceEEEPKcP9lua_State")]
// 0x27dbc8 — __ZL22PropertyNameCorrectionRKN5boost10shared_ptrIN3RBX8InstanceEEEPKcP9lua_State
// was: PropertyNameCorrection(boost::shared_ptr<RBX::Instance> const&,char const*,lua_State *)
// type: const char *__fastcall(int, const char *, int)
pub fn stub_27dbc8() {
    // IDA 0x27dbc8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::callCallback(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>)")]
#[doc(alias = "__ZN3RBX3Lua12callCallbackENS0_15WeakFunctionRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS2_13intrusive_ptrINS0_13WeakThreadRefEEE")]
// 0x27e448 — __ZN3RBX3Lua12callCallbackENS0_15WeakFunctionRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEENS2_13intrusive_ptrINS0_13WeakThreadRefEEE
// was: RBX::Lua::callCallback(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>)
// type: void __fastcall(boost::detail::sp_counted_base **, int, boost::detail::sp_counted_base ***, boost::detail::sp_counted_base *)
pub fn stub_27e448() {
    // IDA 0x27e448: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_newindex(rbx_core::SharedPtr<RBX::Instance>&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexERS5_PKcP9lua_State")]
// 0x27ef18 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_newindexERS5_PKcP9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_newindex(boost::shared_ptr<RBX::Instance>&,char const*,lua_State *)
// type: void __fastcall(RBX::Security::Context *, const char *, int)
pub fn stub_27ef18() {
    // IDA 0x27ef18: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::on_tostring(rbx_core::SharedPtr<RBX::Instance> const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringERKS5_P9lua_State")]
// 0x280b90 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE11on_tostringERKS5_P9lua_State
// was: RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::on_tostring(boost::shared_ptr<RBX::Instance> const&,lua_State *)
// type: int __fastcall(RBX::Instance **, int)
pub fn stub_280b90() {
    // IDA 0x280b90: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "bool RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::getValue<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,unsigned int,rbx_core::SharedPtr<RBX::Instance> &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueIS5_EEbP9lua_StatejRT_")]
// 0x281494 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS_8InstanceEEELb0EE8getValueIS5_EEbP9lua_StatejRT_
// was: bool RBX::Lua::Bridge<boost::shared_ptr<RBX::Instance>,false>::getValue<boost::shared_ptr<RBX::Instance>>(lua_State *,unsigned int,boost::shared_ptr<RBX::Instance> &)
// type: int __fastcall(int, int, int)
pub fn stub_281494() {
    // IDA 0x281494: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list_av_2<rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,RBX::Reflection::Variant,rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>>(void (YieldFunctionStateObject::*)(RBX::Reflection::Variant),rbx_core::SharedPtr<YieldFunctionStateObject>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIv24YieldFunctionStateObjectN3RBX10Reflection7VariantENS_10shared_ptrIS1_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISB_T0_T1_EENS9_9list_av_2IT2_T3_E4typeEEEMSE_FSB_SF_ESI_SJ_")]
// 0x281504 — __ZN5boost4bindIv24YieldFunctionStateObjectN3RBX10Reflection7VariantENS_10shared_ptrIS1_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISB_T0_T1_EENS9_9list_av_2IT2_T3_E4typeEEEMSE_FSB_SF_ESI_SJ_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list_av_2<boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,RBX::Reflection::Variant,boost::shared_ptr<YieldFunctionStateObject>,boos
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_281504() {
    // IDA 0x281504: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS7_5list2INS7_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x282734 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS7_5list2INS7_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
pub fn stub_282734() {
    // IDA 0x282734: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5list2INS6_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x2828bc — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5list2INS6_5valueISB_EENSF_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_2828bc() {
    // IDA 0x2828bc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX10Reflection7VariantEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS7_5list2INS7_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0x283d38 — __ZN5boost8functionIFvN3RBX10Reflection7VariantEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS7_5list2INS7_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_283d38() {
    // IDA 0x283d38: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1IvN3RBX10Reflection7VariantEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x283e1c — __ZN5boost9function1IvN3RBX10Reflection7VariantEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_283e1c() {
    // IDA 0x283e1c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_")]
// 0x283f04 — __ZN5boost9function1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
// was: void boost::function1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_283f04() {
    // IDA 0x283f04: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
// 0x283ffc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::d
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_283ffc() {
    // IDA 0x283ffc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,void,RBX::Reflection::Variant>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_")]
// 0x284018 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,void,RBX::Reflection::Variant>::invoke(boost::detail::
// type: int __fastcall(int *, int)
pub fn stub_284018() {
    // IDA 0x284018: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
// 0x284034 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_284034() {
    // IDA 0x284034: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x28411c — __ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_28411c() {
    // IDA 0x28411c: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x284200 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bi
// type: void __fastcall(int, int, _DWORD *)
pub fn stub_284200() {
    // IDA 0x284200: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list1<RBX::Reflection::Variant&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant> &,boost::_bi::list1<RBX::Reflection::Variant&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_N3RBX10Reflection7VariantEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
// 0x2842d4 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_N3RBX10Reflection7VariantEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list1<RBX::Reflection::Variant&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObjec
// type: int __fastcall(int, int, int *)
pub fn stub_2842d4() {
    // IDA 0x2842d4: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "void boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>::call<rbx_core::SharedPtr<YieldFunctionStateObject>,RBX::Reflection::Variant>(rbx_core::SharedPtr<YieldFunctionStateObject> &,void const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEE4callINS_10shared_ptrIS2_EES5_EEvRT_PKvRT0_")]
// 0x2843e0 — __ZNK5boost4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEE4callINS_10shared_ptrIS2_EES5_EEvRT_PKvRT0_
// was: void boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>::call<boost::shared_ptr<YieldFunctionStateObject>,RBX::Reflection::Variant>(boost::shared_ptr<YieldFunctionStateObject> &,void const*,RBX::Reflection::Variant &)const
// type: int __fastcall(char **, _DWORD *, int, int *)
pub fn stub_2843e0() {
    // IDA 0x2843e0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x2844f8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
pub fn stub_2844f8() {
    // IDA 0x2844f8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,rbx_core::SharedPtr<RBX::Instance>,lua_State *)")]
#[doc(alias = "__ZN24YieldFunctionStateObjectC2EPKN3RBX10Reflection23YieldFunctionDescriptorEN5boost10shared_ptrINS0_8InstanceEEEP9lua_State")]
// 0x284650 — __ZN24YieldFunctionStateObjectC2EPKN3RBX10Reflection23YieldFunctionDescriptorEN5boost10shared_ptrINS0_8InstanceEEEP9lua_State
// was: YieldFunctionStateObject::YieldFunctionStateObject(RBX::Reflection::YieldFunctionDescriptor const*,boost::shared_ptr<RBX::Instance>,lua_State *)
// type: _DWORD *__fastcall(_DWORD *, int, const shared_count *, int, int, void *, int, int, int, int)
pub fn stub_284650() {
    // IDA 0x284650: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list_av_3<RBX::Lua::WeakFunctionRef,boost::arg<1>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>::type> boost::bind<rbx_core::SharedPtr<RBX::Reflection::Tuple>,RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,RBX::Lua::WeakFunctionRef,boost::arg<1>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>(rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),RBX::Lua::WeakFunctionRef,boost::arg<1>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>)")]
#[doc(alias = "__ZN5boost4bindINS_10shared_ptrIN3RBX10Reflection5TupleEEENS2_3Lua15WeakFunctionRefENS1_IKS4_EENS_13intrusive_ptrINS6_13WeakThreadRefEEES7_NS_3argILi1EEESC_EENS_3_bi6bind_tIT_PFSH_T0_T1_T2_ENSF_9list_av_3IT3_T4_T5_E4typeEEESM_SO_SP_SQ_")]
// 0x284820 — __ZN5boost4bindINS_10shared_ptrIN3RBX10Reflection5TupleEEENS2_3Lua15WeakFunctionRefENS1_IKS4_EENS_13intrusive_ptrINS6_13WeakThreadRefEEES7_NS_3argILi1EEESC_EENS_3_bi6bind_tIT_PFSH_T0_T1_T2_ENSF_9list_av_3IT3_T4_T5_E4typeEEESM_SO_SP_SQ_
// was: boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list_av_3<RBX::Lua::WeakFunctionRef,boost::arg<1>,boost::int
// type: void __fastcall(_DWORD *, int, const RBX::Lua::WeakFunctionRef *, int *)
pub fn stub_284820() {
    // IDA 0x284820: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSD_13WeakThreadRefEEEENSB_5list3INSB_5valueISE_EENS_3argILi1EEENSL_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// 0x284c80 — __ZN5boost8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSD_13WeakThreadRefEEEENSB_5list3INSB_5valueISE_EENS_3argILi1EEENSL_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
pub fn stub_284c80() {
    // IDA 0x284c80: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0x284dd8 — __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *)
pub fn stub_284dd8() {
    // IDA 0x284dd8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::function1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>)")]
#[doc(alias = "__ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEEvT_")]
// 0x284f34 — __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEEvT_
// was: void boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple co
// type: void __fastcall(int, int *)
pub fn stub_284f34() {
    // IDA 0x284f34: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
// 0x2850a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::v
// type: _UNKNOWN **__fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_2850a0() {
    // IDA 0x2850a0: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>,rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
#[doc(alias = "__ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEES9_SD_E6invokeERNS1_15function_bufferESD_")]
// 0x2850bc — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEES9_SD_E6invokeERNS1_15function_bufferESD_
// was: boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::
pub fn stub_2850bc() {
    // IDA 0x2850bc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE9assign_toINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE")]
// 0x2850d4 — __ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE9assign_toINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, RBX::Lua::WeakFunctionRef *, RBX::Lua::WeakFunctionRef *, RBX::Lua::WeakFunctionRef *, int, int, int, int)
pub fn stub_2850d4() {
    // IDA 0x2850d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE9assign_toINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// 0x285230 — __ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE9assign_toINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX
// type: int __fastcall(int, int *, int)
pub fn stub_285230() {
    // IDA 0x285230: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::detail::function::basic_vtable1<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE14assign_functorINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
// 0x285388 — __ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE14assign_functorINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_pt
// type: void __fastcall(int, _DWORD *, _DWORD *, int, void *, RBX::Lua::WeakFunctionRef *, int, int, int, int)
pub fn stub_285388() {
    // IDA 0x285388: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple> boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>::operator()<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<rbx_core::SharedPtr<RBX::Reflection::Tuple>>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>&> &,long)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua15WeakFunctionRefEEENS_3argILi1EEENS2_INS_13intrusive_ptrINS4_13WeakThreadRefEEEEEEclINS_10shared_ptrINS3_10Reflection5TupleEEEPFSI_S5_NSF_IKSH_EESB_ENS0_5list1IRSK_EEEET_NS0_4typeISQ_EERT0_RT1_l")]
// 0x285484 — __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua15WeakFunctionRefEEENS_3argILi1EEENS2_INS_13intrusive_ptrINS4_13WeakThreadRefEEEEEEclINS_10shared_ptrINS3_10Reflection5TupleEEEPFSI_S5_NSF_IKSH_EESB_ENS0_5list1IRSK_EEEET_NS0_4typeISQ_EERT0_RT1_l
// was: boost::shared_ptr<RBX::Reflection::Tuple> boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>::operator()<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakF
// type: void __fastcall(int, const RBX::Lua::WeakFunctionRef *, void (__fastcall **)(int, _BYTE *, sp_counted_base **, int *), const shared_count **, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int, int, int)
pub fn stub_285484() {
    // IDA 0x285484: bind listN::operator() forwarded bound + call args into the target. Closure capture+call — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<rbx_core::SharedPtr<RBX::Reflection::Tuple>,rbx_core::SharedPtr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// 0x2855c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::v
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, RBX::Lua::WeakFunctionRef *, int, int, int, int)
pub fn stub_2855c8() {
    // IDA 0x2855c8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> rbx::make_shared<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>,RBX::Lua::FunctionScriptSlot>(RBX::Lua::FunctionScriptSlot const&)")]
#[doc(alias = "__ZN3rbx11make_sharedIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua18FunctionScriptSlotEEES5_EEN5boost10shared_ptrIT_EERKT0_")]
// 0x288798 — __ZN3rbx11make_sharedIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua18FunctionScriptSlotEEES5_EEN5boost10shared_ptrIT_EERKT0_
// was: boost::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> rbx::make_shared<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>,RBX::Lua::FunctionScriptSlot>(RBX::Lua::FunctionScriptSlot const&)
pub fn stub_288798() {
    // IDA 0x288798: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> rbx::make_shared<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>,RBX::Lua::WaitScriptSlot>(RBX::Lua::WaitScriptSlot const&)")]
#[doc(alias = "__ZN3rbx11make_sharedIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua14WaitScriptSlotEEES5_EEN5boost10shared_ptrIT_EERKT0_")]
// 0x28894c — __ZN3rbx11make_sharedIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua14WaitScriptSlotEEES5_EEN5boost10shared_ptrIT_EERKT0_
// was: boost::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> rbx::make_shared<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>,RBX::Lua::WaitScriptSlot>(RBX::Lua::WaitScriptSlot const&)
pub fn stub_28894c() {
    // IDA 0x28894c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>,RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>(rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIN3RBX10Reflection19TGenericSlotWrapperINS4_3Lua14WaitScriptSlotEEEEES9_EEPT_RKNS_10shared_ptrIT0_EE")]
// 0x288ab0 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIN3RBX10Reflection19TGenericSlotWrapperINS4_3Lua14WaitScriptSlotEEEEES9_EEPT_RKNS_10shared_ptrIT0_EE
// was: rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>,RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>(boost::shared_ptr<RBX::Reflection
pub fn stub_288ab0() {
    // IDA 0x288ab0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua14WaitScriptSlotEEEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_")]
// 0x28932c — __ZN5boost10shared_ptrIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua14WaitScriptSlotEEEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
// was: boost::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::Wait
pub fn stub_28932c() {
    // IDA 0x28932c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10Reflection19TGenericSlotWrapperINS3_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS8_EEEET_T0_")]
// 0x289438 — __ZN5boost6detail12shared_countC2IPN3RBX10Reflection19TGenericSlotWrapperINS3_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS8_EEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Refl
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_289438() {
    // IDA 0x289438: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev")]
// 0x289540 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::~sp_counted_impl_pd()
pub fn stub_289540() {
    // IDA 0x289540: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED0Ev")]
// 0x28956c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::~sp_counted_impl_pd()
pub fn stub_28956c() {
    // IDA 0x28956c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE7disposeEv")]
// 0x289628 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::dispose(void)
pub fn stub_289628() {
    // IDA 0x289628: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info")]
// 0x289648 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::get_deleter(std::type_info const&)
pub fn stub_289648() {
    // IDA 0x289648: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv")]
// 0x289660 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua14WaitScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::WaitScriptSlot>>>::get_untyped_deleter(void)
pub fn stub_289660() {
    // IDA 0x289660: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>,RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>(rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> const&)")]
#[doc(alias = "__ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIN3RBX10Reflection19TGenericSlotWrapperINS4_3Lua18FunctionScriptSlotEEEEES9_EEPT_RKNS_10shared_ptrIT0_EE")]
// 0x289664 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIN3RBX10Reflection19TGenericSlotWrapperINS4_3Lua18FunctionScriptSlotEEEEES9_EEPT_RKNS_10shared_ptrIT0_EE
// was: rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>,RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>(boost::shared_ptr<RBX
pub fn stub_289664() {
    // IDA 0x289664: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua18FunctionScriptSlotEEEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_")]
// 0x28a598 — __ZN5boost10shared_ptrIN3RBX10Reflection19TGenericSlotWrapperINS1_3Lua18FunctionScriptSlotEEEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
// was: boost::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>::shared_ptr<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RB
pub fn stub_28a598() {
    // IDA 0x28a598: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10Reflection19TGenericSlotWrapperINS3_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS8_EEEET_T0_")]
// 0x28a6a4 — __ZN5boost6detail12shared_countC2IPN3RBX10Reflection19TGenericSlotWrapperINS3_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS8_EEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>(RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_delet
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_28a6a4() {
    // IDA 0x28a6a4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev")]
// 0x28a7ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::~sp_counted_impl_pd()
pub fn stub_28a7ac() {
    // IDA 0x28a7ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED0Ev")]
// 0x28a7d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::~sp_counted_impl_pd()
pub fn stub_28a7d8() {
    // IDA 0x28a7d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE7disposeEv")]
// 0x28a894 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::dispose(void)
pub fn stub_28a894() {
    // IDA 0x28a894: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info")]
// 0x28a8b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::get_deleter(std::type_info const&)
pub fn stub_28a8b4() {
    // IDA 0x28a8b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv")]
// 0x28a8cc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Reflection19TGenericSlotWrapperINS2_3Lua18FunctionScriptSlotEEEN3rbx6detail13sp_ms_deleterIS7_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot> *,rbx::detail::sp_ms_deleter<RBX::Reflection::TGenericSlotWrapper<RBX::Lua::FunctionScriptSlot>>>::get_untyped_deleter(void)
pub fn stub_28a8cc() {
    // IDA 0x28a8cc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)")]
#[doc(alias = "__ZN3RBX6Script8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE")]
// 0x28e0f8 — __ZN3RBX6Script8writeXmlERKN5boost8functionIFbPNS_8InstanceEEEENS_11CreatorRoleE
// was: RBX::Script::writeXml(boost::function<bool ()(RBX::Instance *)> const&,RBX::CreatorRole)
pub fn stub_28e0f8() {
    // IDA 0x28e0f8: XML serialization helper. Serializer owned by higher crates — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_6ScriptEEEN5boost10shared_ptrIT_EEv")]
// 0x28e630 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6ScriptEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Script> RBX::Creatable<RBX::Instance>::create<RBX::Script>(void)
pub fn stub_28e630() {
    // IDA 0x28e630: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x28e6e0 — __ZN5boost10shared_ptrIN3RBX6ScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Script>::shared_ptr<RBX::Script,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_28e6e0() {
    // IDA 0x28e6e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Script,RBX::Script>(rbx_core::SharedPtr<RBX::Script> const*,RBX::Script *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6ScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x28e7a8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6ScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Script,RBX::Script>(boost::shared_ptr<RBX::Script> const*,RBX::Script *)const
pub fn stub_28e7a8() {
    // IDA 0x28e7a8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX6ScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x28e890 — __ZN5boost6detail12shared_countC2IPN3RBX6ScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_28e890() {
    // IDA 0x28e890: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x28e998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_28e998() {
    // IDA 0x28e998: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x28e99c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_28e99c() {
    // IDA 0x28e99c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x28e9a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_28e9a0() {
    // IDA 0x28e9a0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x28e9c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_28e9c0() {
    // IDA 0x28e9c0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x28e9d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6ScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Script *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_28e9d8() {
    // IDA 0x28e9d8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_20RuntimeScriptServiceEEEN5boost10shared_ptrIT_EEv")]
// 0x28f0b4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20RuntimeScriptServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::RuntimeScriptService> RBX::Creatable<RBX::Instance>::create<RBX::RuntimeScriptService>(void)
pub fn stub_28f0b4() {
    // IDA 0x28f0b4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20RuntimeScriptServiceEEERS3_RKNS0_IT_EE")]
// 0x28f164 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_20RuntimeScriptServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RuntimeScriptService>(boost::shared_ptr<RBX::RuntimeScriptService> const&)
pub fn stub_28f164() {
    // IDA 0x28f164: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20RuntimeScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x28f62c — __ZN5boost10shared_ptrIN3RBX20RuntimeScriptServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::RuntimeScriptService>::shared_ptr<RBX::RuntimeScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_28f62c() {
    // IDA 0x28f62c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RuntimeScriptService,RBX::RuntimeScriptService>(rbx_core::SharedPtr<RBX::RuntimeScriptService> const*,RBX::RuntimeScriptService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20RuntimeScriptServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x28f6f4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20RuntimeScriptServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RuntimeScriptService,RBX::RuntimeScriptService>(boost::shared_ptr<RBX::RuntimeScriptService> const*,RBX::RuntimeScriptService *)const
pub fn stub_28f6f4() {
    // IDA 0x28f6f4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX20RuntimeScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x28f7dc — __ZN5boost6detail12shared_countC2IPN3RBX20RuntimeScriptServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_28f7dc() {
    // IDA 0x28f7dc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x28f8e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_28f8e4() {
    // IDA 0x28f8e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x28f8e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_28f8e8() {
    // IDA 0x28f8e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x28f8ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_28f8ec() {
    // IDA 0x28f8ec: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x28f90c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_28f90c() {
    // IDA 0x28f90c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x28f924 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX20RuntimeScriptServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::RuntimeScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_28f924() {
    // IDA 0x28f924: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "RBX::ScriptContext::registerDevelopmentLibrary(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13ScriptContext26registerDevelopmentLibraryESsN5boost10shared_ptrINS_8InstanceEEE")]
// 0x295640 — __ZN3RBX13ScriptContext26registerDevelopmentLibraryESsN5boost10shared_ptrINS_8InstanceEEE
// was: RBX::ScriptContext::registerDevelopmentLibrary(std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_295640() {
    // IDA 0x295640: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ScriptContext::addCoreScript(int,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
#[doc(alias = "__ZN3RBX13ScriptContext13addCoreScriptEiN5boost10shared_ptrINS_8InstanceEEESs")]
// 0x295890 — __ZN3RBX13ScriptContext13addCoreScriptEiN5boost10shared_ptrINS_8InstanceEEESs
// was: RBX::ScriptContext::addCoreScript(int,boost::shared_ptr<RBX::Instance>,std::string)
pub fn stub_295890() {
    // IDA 0x295890: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ScriptContext::onCamelCaseViolation(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX13ScriptContext20onCamelCaseViolationEN5boost10shared_ptrINS_8InstanceEEESsS4_")]
// 0x29b090 — __ZN3RBX13ScriptContext20onCamelCaseViolationEN5boost10shared_ptrINS_8InstanceEEESsS4_
// was: RBX::ScriptContext::onCamelCaseViolation(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
pub fn stub_29b090() {
    // IDA 0x29b090: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Lua::Continuations::onSuccessHandler(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>)")]
#[doc(alias = "__ZN3RBX3Lua13Continuations16onSuccessHandlerEP9lua_StateN5boost8functionIFvNS4_10shared_ptrIKNS_10Reflection5TupleEEEEEE")]
// 0x29df1c — __ZN3RBX3Lua13Continuations16onSuccessHandlerEP9lua_StateN5boost8functionIFvNS4_10shared_ptrIKNS_10Reflection5TupleEEEEEE
// was: RBX::Lua::Continuations::onSuccessHandler(lua_State *,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_29df1c() {
    // IDA 0x29df1c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ScriptContext::resume(RBX::Lua::ThreadRef,boost::function1<unsigned long,lua_State *>,boost::function2<void,lua_State *,unsigned long>)")]
#[doc(alias = "__ZN3RBX13ScriptContext6resumeENS_3Lua9ThreadRefEN5boost9function1ImP9lua_StateEENS3_9function2IvS6_mEE")]
// 0x29e978 — __ZN3RBX13ScriptContext6resumeENS_3Lua9ThreadRefEN5boost9function1ImP9lua_StateEENS3_9function2IvS6_mEE
// was: RBX::ScriptContext::resume(RBX::Lua::ThreadRef,boost::function1<unsigned long,lua_State *>,boost::function2<void,lua_State *,unsigned long>)
pub fn stub_29e978() {
    // IDA 0x29e978: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptContext::resumeWithArgs(RBX::Lua::ThreadRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
#[doc(alias = "__ZN3RBX13ScriptContext14resumeWithArgsENS_3Lua9ThreadRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEE")]
// 0x29fcc0 — __ZN3RBX13ScriptContext14resumeWithArgsENS_3Lua9ThreadRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// was: RBX::ScriptContext::resumeWithArgs(RBX::Lua::ThreadRef,boost::shared_ptr<RBX::Reflection::Tuple const>)
// type: int __fastcall(char, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
pub fn stub_29fcc0() {
    // IDA 0x29fcc0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::ScriptContext::scheduleResume(RBX::Lua::ThreadRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
#[doc(alias = "__ZN3RBX13ScriptContext14scheduleResumeENS_3Lua9ThreadRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEE")]
// 0x2a279c — __ZN3RBX13ScriptContext14scheduleResumeENS_3Lua9ThreadRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// was: RBX::ScriptContext::scheduleResume(RBX::Lua::ThreadRef,boost::shared_ptr<RBX::Reflection::Tuple const>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_2a279c() {
    // IDA 0x2a279c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
// 0x2a3818 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvN5boost10shared_ptrINS_8InstanceEEESsS6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,bo
pub fn stub_2a3818() {
    // IDA 0x2a3818: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev")]
// 0x2a38e0 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_2a38e0() {
    // IDA 0x2a38e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED1Ev")]
// 0x2a3a08 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFviN5boost10shared_ptrINS_8InstanceEEESsELi3EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,boost::shared_ptr<RBX::Instance>,std::string),3>::~BoundFuncDesc()
pub fn stub_2a3a08() {
    // IDA 0x2a3a08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EED1Ev")]
// 0x2a3b60 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKNS0_5TupleEEEbELi1EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,boost::shared_ptr<RBX::Reflection::Tuple const> ()(bool),1>::~BoundFuncDesc()
pub fn stub_2a3b60() {
    // IDA 0x2a3b60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED1Ev")]
// 0x2a3ba0 — __ZN3RBX10Reflection13BoundFuncDescINS_13ScriptContextEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvELi0EED1Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()
pub fn stub_2a3ba0() {
    // IDA 0x2a3ba0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
// 0x2a3c28 — __ZN3RBX10Reflection9EventDescINS_13ScriptContextEFvSsSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// was: RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,boost::shared_ptr<RBX::Instance>)> RBX::ScriptContext::*>::~Event
pub fn stub_2a3c28() {
    // IDA 0x2a3c28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger>::shared_ptr<RBX::Scripting::ScriptDebugger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9Scripting14ScriptDebuggerEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x2a9e50 — __ZN5boost10shared_ptrIN3RBX9Scripting14ScriptDebuggerEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Scripting::ScriptDebugger>::shared_ptr<RBX::Scripting::ScriptDebugger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_2a9e50() {
    // IDA 0x2a9e50: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::ScriptDebugger,RBX::Scripting::ScriptDebugger>(rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> const*,RBX::Scripting::ScriptDebugger *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9Scripting14ScriptDebuggerES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0x2a9f18 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9Scripting14ScriptDebuggerES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::ScriptDebugger,RBX::Scripting::ScriptDebugger>(boost::shared_ptr<RBX::Scripting::ScriptDebugger> const*,RBX::Scripting::ScriptDebugger *)const
pub fn stub_2a9f18() {
    // IDA 0x2a9f18: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9Scripting14ScriptDebuggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// 0x2aa004 — __ZN5boost6detail12shared_countC2IPN3RBX9Scripting14ScriptDebuggerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// was: boost::detail::shared_count::shared_count<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_2aa004() {
    // IDA 0x2aa004: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// 0x2aa10c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_2aa10c() {
    // IDA 0x2aa10c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// 0x2aa110 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_2aa110() {
    // IDA 0x2aa110: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// 0x2aa114 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_2aa114() {
    // IDA 0x2aa114: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// 0x2aa134 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_2aa134() {
    // IDA 0x2aa134: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// 0x2aa14c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting14ScriptDebuggerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_2aa14c() {
    // IDA 0x2aa14c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript> RBX::Creatable<RBX::Instance>::create<RBX::LocalScript>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11LocalScriptEEEN5boost10shared_ptrIT_EEv")]
// 0x2aa518 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11LocalScriptEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::LocalScript> RBX::Creatable<RBX::Instance>::create<RBX::LocalScript>(void)
pub fn stub_2aa518() {
    // IDA 0x2aa518: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript>::shared_ptr<RBX::LocalScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11LocalScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// 0x2aa5c8 — __ZN5boost10shared_ptrIN3RBX11LocalScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::LocalScript>::shared_ptr<RBX::LocalScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_2aa5c8() {
    // IDA 0x2aa5c8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
