// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel (strict 9903) EA-sorted asc next 150 not yet in datamodel — fallback to global gap filler EA-sorted asc (strict filter 10215 including Workspace already complete, 0 remaining; using lowest uncovered EA asc)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 150 stubs | range 0x26dc28..0x273c2c | datamodel before 30514, after 30664, global 85545 all covered, fallback to datamodel gap EA-sorted asc, bg worker datamodel-bg1
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x26dc28 — __ZNK3RBX3Lua12LuaArguments4sizeEv
// type: int __fastcall(RBX::Lua::LuaArguments *this)
#[doc(alias = "RBX::Lua::LuaArguments::size(void)const")]
pub fn stub_26dc28() -> ! {
    todo!("0x26dc28 RBX::Lua::LuaArguments::size(void)const")
}

// 0x26dc38 — __ZNK3RBX3Lua12LuaArguments10getVariantEiRNS_10Reflection7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Lua::LuaArguments::getVariant(int,RBX::Reflection::Variant &)const")]
pub fn stub_26dc38() -> ! {
    todo!("0x26dc38 RBX::Lua::LuaArguments::getVariant(int,RBX::Reflection::Variant &)const")
}

// 0x26dca8 — __ZNK3RBX3Lua12LuaArguments7getLongEiRl
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, int *)
#[doc(alias = "RBX::Lua::LuaArguments::getLong(int,long &)const")]
pub fn stub_26dca8() -> ! {
    todo!("0x26dca8 RBX::Lua::LuaArguments::getLong(int,long &)const")
}

// 0x26ddb4 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEE
// type: int __fastcall(_DWORD *, _DWORD *)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
pub fn stub_26ddb4() -> ! {
    todo!("0x26ddb4 RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")
}

// 0x26dddc — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEE
// type: int __fastcall(int *, int *)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_26dddc() -> ! {
    todo!("0x26dddc RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0x26dea0 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS2_9unordered13unordered_mapISsNS_10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS7_EEEEEE
// type: int __fastcall(int *, _DWORD *)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_26dea0() -> ! {
    todo!("0x26dea0 RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0x26df2c — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// type: int __fastcall(int *, char ******)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<RBX::Reflection::Tuple const>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_26df2c() -> ! {
    todo!("0x26df2c RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")
}

// 0x26df60 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEE
// type: int __fastcall(int *, const shared_count *)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
pub fn stub_26df60() -> ! {
    todo!("0x26df60 RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")
}

// 0x26e030 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEE
// type: int __fastcall(int *, const shared_count *)
// was: RBX::Lua::ArgumentPusher::operator()(boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
pub fn stub_26e030() -> ! {
    todo!("0x26e030 RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")
}

// 0x26e100 — __ZN3RBX3Lua14ArgumentPusherclINS_6CellIDEEEiRKT_PN5boost10disable_ifINS7_13is_arithmeticIS4_EEvE4typeE
// type: int __fastcall(int *, int)
#[doc(alias = "int RBX::Lua::ArgumentPusher::operator()<RBX::CellID>(RBX::CellID const&,boost::disable_if<boost::is_arithmetic<RBX::CellID>,void>::type *)")]
pub fn stub_26e100() -> ! {
    todo!("0x26e100 int RBX::Lua::ArgumentPusher::operator()<RBX::CellID>(RBX::CellID const&,boost::disable_if<boost::is_arithmetic<RBX::CellID>,void>::type *)")
}

// 0x26e1d8 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "RBX::InputObject* RBX::Lua::Bridge<RBX::InputObject,true>::pushNewObject<RBX::InputObject>(lua_State *,RBX::InputObject)")]
pub fn stub_26e1d8() -> ! {
    todo!("0x26e1d8 RBX::InputObject* RBX::Lua::Bridge<RBX::InputObject,true>::pushNewObject<RBX::InputObject>(lua_State *,RBX::InputObject)")
}

// 0x26e228 — __ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26e228() -> ! {
    todo!("0x26e228 RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e318 — __ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26e318() -> ! {
    todo!("0x26e318 RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e408 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::CellID* RBX::Lua::Bridge<RBX::CellID,true>::pushNewObject<RBX::CellID>(lua_State *,RBX::CellID)")]
pub fn stub_26e408() -> ! {
    todo!("0x26e408 RBX::CellID* RBX::Lua::Bridge<RBX::CellID,true>::pushNewObject<RBX::CellID>(lua_State *,RBX::CellID)")
}

// 0x26e464 — __ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26e464() -> ! {
    todo!("0x26e464 RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e554 — __ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26e554() -> ! {
    todo!("0x26e554 RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e648 — __ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26e648() -> ! {
    todo!("0x26e648 RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e738 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, __int64 *)
#[doc(alias = "RBX::Region3int16* RBX::Lua::Bridge<RBX::Region3int16,true>::pushNewObject<RBX::Region3int16>(lua_State *,RBX::Region3int16)")]
pub fn stub_26e738() -> ! {
    todo!("0x26e738 RBX::Region3int16* RBX::Lua::Bridge<RBX::Region3int16,true>::pushNewObject<RBX::Region3int16>(lua_State *,RBX::Region3int16)")
}

// 0x26e780 — __ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26e780() -> ! {
    todo!("0x26e780 RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e870 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: G3D::Matrix3 *__fastcall(int, int)
#[doc(alias = "RBX::Region3* RBX::Lua::Bridge<RBX::Region3,true>::pushNewObject<RBX::Region3>(lua_State *,RBX::Region3)")]
pub fn stub_26e870() -> ! {
    todo!("0x26e870 RBX::Region3* RBX::Lua::Bridge<RBX::Region3,true>::pushNewObject<RBX::Region3>(lua_State *,RBX::Region3)")
}

// 0x26e8d0 — __ZN3rbx8any_castIRKN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26e8d0() -> ! {
    todo!("0x26e8d0 G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e9c0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")]
pub fn stub_26e9c0() -> ! {
    todo!("0x26e9c0 G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")
}

// 0x26ea00 — __ZN3rbx8any_castIRKN3G3D12Vector3int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26ea00() -> ! {
    todo!("0x26ea00 G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26eaf0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: int __fastcall(int, int, __int16)
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")]
pub fn stub_26eaf0() -> ! {
    todo!("0x26eaf0 G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")
}

// 0x26eb44 — __ZN3rbx8any_castIRKN5boost10shared_ptrINS1_8functionIFvNS2_IKN3RBX10Reflection5TupleEEENS3_IFvPNS4_3Lua12IAsyncResultEEEEEEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<boost::shared_ptr<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26eb44() -> ! {
    todo!("0x26eb44 rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26ec34 — __ZN3rbx8any_castIRKN5boost10shared_ptrINS1_8functionIFNS2_IKN3RBX10Reflection5TupleEEES8_EEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>> const& rbx::any_cast<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26ec34() -> ! {
    todo!("0x26ec34 rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const& rbx::any_cast<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26ed24 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKN3RBX10Reflection5TupleEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: boost::shared_ptr<RBX::Reflection::Tuple const> const& rbx::any_cast<boost::shared_ptr<RBX::Reflection::Tuple const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26ed24() -> ! {
    todo!("0x26ed24 rbx_core::SharedPtr<RBX::Reflection::Tuple const> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26f0e4 — __ZN3rbx8any_castIRKN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
// was: boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26f0e4() -> ! {
    todo!("0x26f0e4 rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const& rbx::any_cast<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26f1d4 — __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKNS_10Reflection7VariantESt6vectorIS6_SaIS6_EEEEEEiT_SD_P9lua_State
// type: int __fastcall(char ****, char ****, int)
#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,lua_State *)")]
pub fn stub_26f1d4() -> ! {
    todo!("0x26f1d4 int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,lua_State *)")
}

// 0x26f280 — __ZN3rbx8any_castIRKN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: _DWORD **__fastcall(_DWORD **)
#[doc(alias = "RBX::Lua::WeakFunctionRef const& rbx::any_cast<RBX::Lua::WeakFunctionRef const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26f280() -> ! {
    todo!("0x26f280 RBX::Lua::WeakFunctionRef const& rbx::any_cast<RBX::Lua::WeakFunctionRef const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26f368 — __ZN3RBX10Reflection14EnumDescriptor16lookupDescriptorERKSt9type_info
// type: int __fastcall(RBX::Reflection::EnumDescriptor *this, const std::type_info *)
#[doc(alias = "RBX::Reflection::EnumDescriptor::lookupDescriptor(std::type_info const&)")]
pub fn stub_26f368() -> ! {
    todo!("0x26f368 RBX::Reflection::EnumDescriptor::lookupDescriptor(std::type_info const&)")
}

// 0x26f3a0 — __ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::ProtectedString const& rbx::any_cast<RBX::ProtectedString const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26f3a0() -> ! {
    todo!("0x26f3a0 RBX::ProtectedString const& rbx::any_cast<RBX::ProtectedString const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26f490 — __ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: _DWORD **__fastcall(_DWORD **)
#[doc(alias = "long const& rbx::any_cast<long const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_26f490() -> ! {
    todo!("0x26f490 long const& rbx::any_cast<long const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26f578 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject>(RBX::InputObject const&)")]
pub fn stub_26f578() -> ! {
    todo!("0x26f578 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject>(RBX::InputObject const&)")
}

// 0x26f5e0 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject>::construct_func(char const*,char *)")]
pub fn stub_26f5e0() -> ! {
    todo!("0x26f5e0 rbx::implementation::typed_holder<RBX::InputObject>::construct_func(char const*,char *)")
}

// 0x26f600 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CellID>(RBX::CellID const&)")]
pub fn stub_26f600() -> ! {
    todo!("0x26f600 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CellID>(RBX::CellID const&)")
}

// 0x26f680 — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv
// type: _DWORD *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::singleton(void)")]
pub fn stub_26f680() -> ! {
    todo!("0x26f680 rbx::implementation::typed_holder<RBX::CellID>::singleton(void)")
}

// 0x26f6ec — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE14construct_funcEPKcPc
// type: int __fastcall(int result, int)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::construct_func(char const*,char *)")]
pub fn stub_26f6ec() -> ! {
    todo!("0x26f6ec rbx::implementation::typed_holder<RBX::CellID>::construct_func(char const*,char *)")
}

// 0x26f718 — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE13destruct_funcEPc
// type: void __fastcall(RBX::CellID *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::destruct_func(char *)")]
pub fn stub_26f718() -> ! {
    todo!("0x26f718 rbx::implementation::typed_holder<RBX::CellID>::destruct_func(char *)")
}

// 0x26f720 — __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim>::construct_func(char const*,char *)")]
pub fn stub_26f720() -> ! {
    todo!("0x26f720 rbx::implementation::typed_holder<RBX::UDim>::construct_func(char const*,char *)")
}

// 0x26f730 — __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim>::destruct_func(char *)")]
pub fn stub_26f730() -> ! {
    todo!("0x26f730 rbx::implementation::typed_holder<RBX::UDim>::destruct_func(char *)")
}

// 0x26f738 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv
// type: int *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::singleton(void)")]
pub fn stub_26f738() -> ! {
    todo!("0x26f738 rbx::implementation::typed_holder<RBX::RbxRay>::singleton(void)")
}

// 0x26f7a8 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE13destruct_funcEPc
// type: int __fastcall(int (__fastcall ***)(_DWORD))
#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::destruct_func(char *)")]
pub fn stub_26f7a8() -> ! {
    todo!("0x26f7a8 rbx::implementation::typed_holder<RBX::RbxRay>::destruct_func(char *)")
}

// 0x26f7b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector2EEERS3_RKT_
// type: int **__fastcall(int **, int **)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)")]
pub fn stub_26f7b0() -> ! {
    todo!("0x26f7b0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)")
}

// 0x26f808 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE9singletonEv
// type: int *()
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)")]
pub fn stub_26f808() -> ! {
    todo!("0x26f808 rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)")
}

// 0x26f878 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector3EEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)")]
pub fn stub_26f878() -> ! {
    todo!("0x26f878 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)")
}

// 0x26f8d8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector3int16EEERS3_RKT_
// type: int __fastcall(int, int *)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)")]
pub fn stub_26f8d8() -> ! {
    todo!("0x26f8d8 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)")
}

// 0x26f930 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE9singletonEv
// type: int *()
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)")]
pub fn stub_26f930() -> ! {
    todo!("0x26f930 rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)")
}

// 0x26f9a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_
// type: int __fastcall(int, __int64 *)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3int16>(RBX::Region3int16 const&)")]
pub fn stub_26f9a0() -> ! {
    todo!("0x26f9a0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3int16>(RBX::Region3int16 const&)")
}

// 0x26fa00 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv
// type: int *()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::singleton(void)")]
pub fn stub_26fa00() -> ! {
    todo!("0x26fa00 rbx::implementation::typed_holder<RBX::Region3int16>::singleton(void)")
}

// 0x26fa70 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE13destruct_funcEPc
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::destruct_func(char *)")]
pub fn stub_26fa70() -> ! {
    todo!("0x26fa70 rbx::implementation::typed_holder<RBX::Region3int16>::destruct_func(char *)")
}

// 0x26faf8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_3Lua15WeakFunctionRefEEERS3_RKT_
// type: int **__fastcall(int **, const RBX::Lua::WeakFunctionRef *)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Lua::WeakFunctionRef>(RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_26faf8() -> ! {
    todo!("0x26faf8 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Lua::WeakFunctionRef>(RBX::Lua::WeakFunctionRef const&)")
}

// 0x26fb50 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE14construct_funcEPKcPc
// type: const RBX::Lua::WeakFunctionRef *__fastcall(const RBX::Lua::WeakFunctionRef *result, RBX::Lua::WeakFunctionRef *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::construct_func(char const*,char *)")]
pub fn stub_26fb50() -> ! {
    todo!("0x26fb50 rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::construct_func(char const*,char *)")
}

// 0x26fb60 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE13destruct_funcEPc
// type: int __fastcall(int)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::destruct_func(char *)")]
pub fn stub_26fb60() -> ! {
    todo!("0x26fb60 rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::destruct_func(char *)")
}

// 0x26fb68 — __ZN5boost11get_deleterIN3rbx6detail13sp_ms_deleterIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEESA_EEPT_RKNS_10shared_ptrIT0_EE
// type: int __fastcall(int)
// was: rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)
#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
pub fn stub_26fb68() -> ! {
    todo!("0x26fb68 rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")
}

// 0x26fbc4 — __ZN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS4_EEEC2IS7_N3rbx6detail13sp_ms_deleterIS7_EEEEPT_T0_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int *, int, int, int)
// was: boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)
#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
pub fn stub_26fbc4() -> ! {
    todo!("0x26fbc4 rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")
}

// 0x26fccc — __ZN5boost6detail12shared_countC2IPKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEN3rbx6detail13sp_ms_deleterIS9_EEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
pub fn stub_26fccc() -> ! {
    todo!("0x26fccc boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")
}

// 0x26fdd0 — __ZN5boost6detail18sp_counted_impl_pdIPKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS8_EEED1Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~sp_counted_impl_pd()")]
pub fn stub_26fdd0() -> ! {
    todo!("0x26fdd0 boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~sp_counted_impl_pd()")
}

// 0x26fdfc — __ZN5boost6detail18sp_counted_impl_pdIPKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS8_EEED0Ev
// type: void __fastcall(_BYTE *, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~sp_counted_impl_pd()")]
pub fn stub_26fdfc() -> ! {
    todo!("0x26fdfc boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~sp_counted_impl_pd()")
}

// 0x26feb4 — __ZN5boost6detail18sp_counted_impl_pdIPKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS8_EEE7disposeEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::dispose(void)")]
pub fn stub_26feb4() -> ! {
    todo!("0x26feb4 boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::dispose(void)")
}

// 0x26fed4 — __ZN5boost6detail18sp_counted_impl_pdIPKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS8_EEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::get_deleter(std::type_info const&)")]
pub fn stub_26fed4() -> ! {
    todo!("0x26fed4 boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::get_deleter(std::type_info const&)")
}

// 0x26feec — __ZN5boost6detail18sp_counted_impl_pdIPKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS8_EEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::get_untyped_deleter(void)")]
pub fn stub_26feec() -> ! {
    todo!("0x26feec boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::get_untyped_deleter(void)")
}

// 0x26fef0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS5_9unordered13unordered_mapISsNS1_10Reflection7VariantENS5_4hashISsEESt8equal_toISsESaISt4pairIKSsSA_EEEEEEEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_26fef0() -> ! {
    todo!("0x26fef0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")
}

// 0x26ff58 — __ZN5boost10shared_ptrIKNS_9unordered13unordered_mapISsN3RBX10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsS5_EEEEEaSERKSG_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>::operator=(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)
#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>::operator=(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")]
pub fn stub_26ff58() -> ! {
    todo!("0x26ff58 rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>::operator=(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> const&)")
}

// 0x270008 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE8getValueIS6_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::getValue<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,unsigned int,RBX::Reflection::EnumDescriptor::Item const* &)")]
pub fn stub_270008() -> ! {
    todo!("0x270008 bool RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::getValue<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,unsigned int,RBX::Reflection::EnumDescriptor::Item const* &)")
}

// 0x270078 — __GLOBAL__I_a_63
#[doc(alias = "global constructor keyed to_a_63")]
pub fn stub_270078() -> ! {
    todo!("0x270078 global constructor keyed to_a_63")
}

// 0x270210 — __ZN3RBX3Lua17safe_lua_tostringEP9lua_Statei
// type: const char *__fastcall(int, int)
#[doc(alias = "RBX::Lua::safe_lua_tostring(lua_State *,int)")]
pub fn stub_270210() -> ! {
    todo!("0x270210 RBX::Lua::safe_lua_tostring(lua_State *,int)")
}

// 0x270230 — __ZN3RBX3Lua22throwable_lua_tostringEP9lua_Statei
// type: const char *__fastcall(int, int)
#[doc(alias = "RBX::Lua::throwable_lua_tostring(lua_State *,int)")]
pub fn stub_270230() -> ! {
    todo!("0x270230 RBX::Lua::throwable_lua_tostring(lua_State *,int)")
}

// 0x270448 — __ZN3RBX3Lua11lua_tofloatEP9lua_Statei
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::lua_tofloat(lua_State *,int)")]
pub fn stub_270448() -> ! {
    todo!("0x270448 RBX::Lua::lua_tofloat(lua_State *,int)")
}

// 0x2704e0 — __ZN3RBX3Lua12Color3Bridge9newColor3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Color3Bridge::newColor3(lua_State *)")]
pub fn stub_2704e0() -> ! {
    todo!("0x2704e0 RBX::Lua::Color3Bridge::newColor3(lua_State *)")
}

// 0x270594 — __ZN3RBX3Lua12Color3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Color3Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_270594() -> ! {
    todo!("0x270594 RBX::Lua::Color3Bridge::registerClassLibrary(lua_State *)")
}

// 0x2705d0 — __ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")]
pub fn stub_2705d0() -> ! {
    todo!("0x2705d0 RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")
}

// 0x2705ec — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(float *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")]
pub fn stub_2705ec() -> ! {
    todo!("0x2705ec RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")
}

// 0x270724 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")]
pub fn stub_270724() -> ! {
    todo!("0x270724 RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")
}

// 0x2707dc — __ZN3RBX3Lua12RbxRayBridge9newRbxRayEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::RbxRayBridge::newRbxRay(lua_State *)")]
pub fn stub_2707dc() -> ! {
    todo!("0x2707dc RBX::Lua::RbxRayBridge::newRbxRay(lua_State *)")
}

// 0x2708b0 — __ZN3RBX3Lua12RbxRayBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::RbxRayBridge::registerClassLibrary(lua_State *)")]
pub fn stub_2708b0() -> ! {
    todo!("0x2708b0 RBX::Lua::RbxRayBridge::registerClassLibrary(lua_State *)")
}

// 0x2708ec — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_index(RBX::RbxRay const&,char const*,lua_State *)")]
pub fn stub_2708ec() -> ! {
    todo!("0x2708ec RBX::Lua::Bridge<RBX::RbxRay,true>::on_index(RBX::RbxRay const&,char const*,lua_State *)")
}

// 0x270afc — __ZN3RBX3LuaL19closestPointVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::closestPointVector3(lua_State *)")]
pub fn stub_270afc() -> ! {
    todo!("0x270afc RBX::Lua::closestPointVector3(lua_State *)")
}

// 0x270b48 — __ZN3RBX3LuaL15distanceVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::distanceVector3(lua_State *)")]
pub fn stub_270b48() -> ! {
    todo!("0x270b48 RBX::Lua::distanceVector3(lua_State *)")
}

// 0x270b98 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_newindex(RBX::RbxRay&,char const*,lua_State *)")]
pub fn stub_270b98() -> ! {
    todo!("0x270b98 RBX::Lua::Bridge<RBX::RbxRay,true>::on_newindex(RBX::RbxRay&,char const*,lua_State *)")
}

// 0x270c50 — __ZN3RBX3Lua13Region3Bridge10newRegion3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3Bridge::newRegion3(lua_State *)")]
pub fn stub_270c50() -> ! {
    todo!("0x270c50 RBX::Lua::Region3Bridge::newRegion3(lua_State *)")
}

// 0x270d50 — __ZN3RBX3Lua13Region3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_270d50() -> ! {
    todo!("0x270d50 RBX::Lua::Region3Bridge::registerClassLibrary(lua_State *)")
}

// 0x270d8c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_index(RBX::Region3 const&,char const*,lua_State *)")]
pub fn stub_270d8c() -> ! {
    todo!("0x270d8c RBX::Lua::Bridge<RBX::Region3,true>::on_index(RBX::Region3 const&,char const*,lua_State *)")
}

// 0x270ec8 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_newindex(RBX::Region3&,char const*,lua_State *)")]
pub fn stub_270ec8() -> ! {
    todo!("0x270ec8 RBX::Lua::Bridge<RBX::Region3,true>::on_newindex(RBX::Region3&,char const*,lua_State *)")
}

// 0x270f80 — __ZN3RBX3Lua18Region3int16Bridge15newRegion3int16EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3int16Bridge::newRegion3int16(lua_State *)")]
pub fn stub_270f80() -> ! {
    todo!("0x270f80 RBX::Lua::Region3int16Bridge::newRegion3int16(lua_State *)")
}

// 0x271064 — __ZN3RBX3Lua18Region3int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3int16Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_271064() -> ! {
    todo!("0x271064 RBX::Lua::Region3int16Bridge::registerClassLibrary(lua_State *)")
}

// 0x2710a0 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_index(RBX::Region3int16 const&,char const*,lua_State *)")]
pub fn stub_2710a0() -> ! {
    todo!("0x2710a0 RBX::Lua::Bridge<RBX::Region3int16,true>::on_index(RBX::Region3int16 const&,char const*,lua_State *)")
}

// 0x2711d4 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_newindex(RBX::Region3int16&,char const*,lua_State *)")]
pub fn stub_2711d4() -> ! {
    todo!("0x2711d4 RBX::Lua::Bridge<RBX::Region3int16,true>::on_newindex(RBX::Region3int16&,char const*,lua_State *)")
}

// 0x27128c — __ZN3RBX3Lua13Vector3Bridge10newVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3(lua_State *)")]
pub fn stub_27128c() -> ! {
    todo!("0x27128c RBX::Lua::Vector3Bridge::newVector3(lua_State *)")
}

// 0x271340 — __ZN3RBX3Lua13Vector3Bridge22newVector3FromNormalIdEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3FromNormalId(lua_State *)")]
pub fn stub_271340() -> ! {
    todo!("0x271340 RBX::Lua::Vector3Bridge::newVector3FromNormalId(lua_State *)")
}

// 0x2714a0 — __ZN3RBX3Lua13Vector3Bridge18newVector3FromAxisEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3FromAxis(lua_State *)")]
pub fn stub_2714a0() -> ! {
    todo!("0x2714a0 RBX::Lua::Vector3Bridge::newVector3FromAxis(lua_State *)")
}

// 0x271604 — __ZN3RBX3Lua13Vector3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_271604() -> ! {
    todo!("0x271604 RBX::Lua::Vector3Bridge::registerClassLibrary(lua_State *)")
}

// 0x271640 — __ZN3RBX3Lua13Vector3Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_add(lua_State *)")]
pub fn stub_271640() -> ! {
    todo!("0x271640 RBX::Lua::Vector3Bridge::on_add(lua_State *)")
}

// 0x2716a0 — __ZN3RBX3Lua13Vector3Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_sub(lua_State *)")]
pub fn stub_2716a0() -> ! {
    todo!("0x2716a0 RBX::Lua::Vector3Bridge::on_sub(lua_State *)")
}

// 0x271700 — __ZN3RBX3Lua13Vector3Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_mul(lua_State *)")]
pub fn stub_271700() -> ! {
    todo!("0x271700 RBX::Lua::Vector3Bridge::on_mul(lua_State *)")
}

// 0x271804 — __ZN3RBX3Lua13Vector3Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_div(lua_State *)")]
pub fn stub_271804() -> ! {
    todo!("0x271804 RBX::Lua::Vector3Bridge::on_div(lua_State *)")
}

// 0x27191c — __ZN3RBX3Lua13Vector3Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_unm(lua_State *)")]
pub fn stub_27191c() -> ! {
    todo!("0x27191c RBX::Lua::Vector3Bridge::on_unm(lua_State *)")
}

// 0x271954 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")]
pub fn stub_271954() -> ! {
    todo!("0x271954 RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")
}

// 0x271c4c — __ZN3RBX3LuaL11lerpVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::lerpVector3(lua_State *)")]
pub fn stub_271c4c() -> ! {
    todo!("0x271c4c RBX::Lua::lerpVector3(lua_State *)")
}

// 0x271cd0 — __ZN3RBX3LuaL12crossVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::crossVector3(lua_State *)")]
pub fn stub_271cd0() -> ! {
    todo!("0x271cd0 RBX::Lua::crossVector3(lua_State *)")
}

// 0x271d48 — __ZN3RBX3LuaL10dotVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::dotVector3(lua_State *)")]
pub fn stub_271d48() -> ! {
    todo!("0x271d48 RBX::Lua::dotVector3(lua_State *)")
}

// 0x271dac — __ZN3RBX3LuaL14isCloseVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::isCloseVector3(lua_State *)")]
pub fn stub_271dac() -> ! {
    todo!("0x271dac RBX::Lua::isCloseVector3(lua_State *)")
}

// 0x271e14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")]
pub fn stub_271e14() -> ! {
    todo!("0x271e14 RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")
}

// 0x271ecc — __ZN3RBX3Lua18Vector3int16Bridge15newVector3int16EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::newVector3int16(lua_State *)")]
pub fn stub_271ecc() -> ! {
    todo!("0x271ecc RBX::Lua::Vector3int16Bridge::newVector3int16(lua_State *)")
}

// 0x271f84 — __ZN3RBX3Lua18Vector3int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_271f84() -> ! {
    todo!("0x271f84 RBX::Lua::Vector3int16Bridge::registerClassLibrary(lua_State *)")
}

// 0x271fc0 — __ZN3RBX3Lua18Vector3int16Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_add(lua_State *)")]
pub fn stub_271fc0() -> ! {
    todo!("0x271fc0 RBX::Lua::Vector3int16Bridge::on_add(lua_State *)")
}

// 0x272008 — __ZN3RBX3Lua18Vector3int16Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_sub(lua_State *)")]
pub fn stub_272008() -> ! {
    todo!("0x272008 RBX::Lua::Vector3int16Bridge::on_sub(lua_State *)")
}

// 0x272050 — __ZN3RBX3Lua18Vector3int16Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_mul(lua_State *)")]
pub fn stub_272050() -> ! {
    todo!("0x272050 RBX::Lua::Vector3int16Bridge::on_mul(lua_State *)")
}

// 0x272108 — __ZN3RBX3Lua18Vector3int16Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_div(lua_State *)")]
pub fn stub_272108() -> ! {
    todo!("0x272108 RBX::Lua::Vector3int16Bridge::on_div(lua_State *)")
}

// 0x272230 — __ZN3RBX3Lua18Vector3int16Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_unm(lua_State *)")]
pub fn stub_272230() -> ! {
    todo!("0x272230 RBX::Lua::Vector3int16Bridge::on_unm(lua_State *)")
}

// 0x272268 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")]
pub fn stub_272268() -> ! {
    todo!("0x272268 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")
}

// 0x2723d0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")]
pub fn stub_2723d0() -> ! {
    todo!("0x2723d0 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")
}

// 0x272488 — __ZN3RBX3Lua18Vector2int16Bridge15newVector2int16EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::newVector2int16(lua_State *)")]
pub fn stub_272488() -> ! {
    todo!("0x272488 RBX::Lua::Vector2int16Bridge::newVector2int16(lua_State *)")
}

// 0x272540 — __ZN3RBX3Lua18Vector2int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_272540() -> ! {
    todo!("0x272540 RBX::Lua::Vector2int16Bridge::registerClassLibrary(lua_State *)")
}

// 0x27257c — __ZN3RBX3Lua18Vector2int16Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_add(lua_State *)")]
pub fn stub_27257c() -> ! {
    todo!("0x27257c RBX::Lua::Vector2int16Bridge::on_add(lua_State *)")
}

// 0x2725bc — __ZN3RBX3Lua18Vector2int16Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_sub(lua_State *)")]
pub fn stub_2725bc() -> ! {
    todo!("0x2725bc RBX::Lua::Vector2int16Bridge::on_sub(lua_State *)")
}

// 0x2725fc — __ZN3RBX3Lua18Vector2int16Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_mul(lua_State *)")]
pub fn stub_2725fc() -> ! {
    todo!("0x2725fc RBX::Lua::Vector2int16Bridge::on_mul(lua_State *)")
}

// 0x2726f8 — __ZN3RBX3Lua18Vector2int16Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_div(lua_State *)")]
pub fn stub_2726f8() -> ! {
    todo!("0x2726f8 RBX::Lua::Vector2int16Bridge::on_div(lua_State *)")
}

// 0x2727d4 — __ZN3RBX3Lua18Vector2int16Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_unm(lua_State *)")]
pub fn stub_2727d4() -> ! {
    todo!("0x2727d4 RBX::Lua::Vector2int16Bridge::on_unm(lua_State *)")
}

// 0x272804 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")]
pub fn stub_272804() -> ! {
    todo!("0x272804 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")
}

// 0x272940 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")]
pub fn stub_272940() -> ! {
    todo!("0x272940 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")
}

// 0x2729f8 — __ZN3RBX3Lua13Vector2Bridge10newVector2EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::newVector2(lua_State *)")]
pub fn stub_2729f8() -> ! {
    todo!("0x2729f8 RBX::Lua::Vector2Bridge::newVector2(lua_State *)")
}

// 0x272aac — __ZN3RBX3Lua13Vector2Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_272aac() -> ! {
    todo!("0x272aac RBX::Lua::Vector2Bridge::registerClassLibrary(lua_State *)")
}

// 0x272ae8 — __ZN3RBX3Lua13Vector2Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_add(lua_State *)")]
pub fn stub_272ae8() -> ! {
    todo!("0x272ae8 RBX::Lua::Vector2Bridge::on_add(lua_State *)")
}

// 0x272b40 — __ZN3RBX3Lua13Vector2Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_sub(lua_State *)")]
pub fn stub_272b40() -> ! {
    todo!("0x272b40 RBX::Lua::Vector2Bridge::on_sub(lua_State *)")
}

// 0x272b98 — __ZN3RBX3Lua13Vector2Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_mul(lua_State *)")]
pub fn stub_272b98() -> ! {
    todo!("0x272b98 RBX::Lua::Vector2Bridge::on_mul(lua_State *)")
}

// 0x272c6c — __ZN3RBX3Lua13Vector2Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_div(lua_State *)")]
pub fn stub_272c6c() -> ! {
    todo!("0x272c6c RBX::Lua::Vector2Bridge::on_div(lua_State *)")
}

// 0x272d28 — __ZN3RBX3Lua13Vector2Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2Bridge::on_unm(lua_State *)")]
pub fn stub_272d28() -> ! {
    todo!("0x272d28 RBX::Lua::Vector2Bridge::on_unm(lua_State *)")
}

// 0x272d70 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")]
pub fn stub_272d70() -> ! {
    todo!("0x272d70 RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")
}

// 0x272f6c — __ZN3RBX3LuaL11lerpVector2EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::lerpVector2(lua_State *)")]
pub fn stub_272f6c() -> ! {
    todo!("0x272f6c RBX::Lua::lerpVector2(lua_State *)")
}

// 0x272fe4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")]
pub fn stub_272fe4() -> ! {
    todo!("0x272fe4 RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")
}

// 0x27309c — __ZN3RBX3Lua16BrickColorBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::registerClassLibrary(lua_State *)")]
pub fn stub_27309c() -> ! {
    todo!("0x27309c RBX::Lua::BrickColorBridge::registerClassLibrary(lua_State *)")
}

// 0x2730d8 — __ZN3RBX3Lua16BrickColorBridge13newBrickColorEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::newBrickColor(lua_State *)")]
pub fn stub_2730d8() -> ! {
    todo!("0x2730d8 RBX::Lua::BrickColorBridge::newBrickColor(lua_State *)")
}

// 0x2731f0 — __ZN3RBX3Lua16BrickColorBridge16randomBrickColorEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::randomBrickColor(lua_State *)")]
pub fn stub_2731f0() -> ! {
    todo!("0x2731f0 RBX::Lua::BrickColorBridge::randomBrickColor(lua_State *)")
}

// 0x27320c — __ZN3RBX3Lua16BrickColorBridge17paletteBrickColorEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::BrickColorBridge::paletteBrickColor(lua_State *)")]
pub fn stub_27320c() -> ! {
    todo!("0x27320c RBX::Lua::BrickColorBridge::paletteBrickColor(lua_State *)")
}

// 0x273330 — __ZN3RBX3LuaL9pushWhiteEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushWhite(lua_State *)")]
pub fn stub_273330() -> ! {
    todo!("0x273330 RBX::Lua::pushWhite(lua_State *)")
}

// 0x273340 — __ZN3RBX3LuaL8pushGrayEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushGray(lua_State *)")]
pub fn stub_273340() -> ! {
    todo!("0x273340 RBX::Lua::pushGray(lua_State *)")
}

// 0x273350 — __ZN3RBX3LuaL12pushDarkGrayEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushDarkGray(lua_State *)")]
pub fn stub_273350() -> ! {
    todo!("0x273350 RBX::Lua::pushDarkGray(lua_State *)")
}

// 0x273360 — __ZN3RBX3LuaL9pushBlackEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushBlack(lua_State *)")]
pub fn stub_273360() -> ! {
    todo!("0x273360 RBX::Lua::pushBlack(lua_State *)")
}

// 0x273370 — __ZN3RBX3LuaL7pushRedEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushRed(lua_State *)")]
pub fn stub_273370() -> ! {
    todo!("0x273370 RBX::Lua::pushRed(lua_State *)")
}

// 0x273380 — __ZN3RBX3LuaL10pushYellowEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushYellow(lua_State *)")]
pub fn stub_273380() -> ! {
    todo!("0x273380 RBX::Lua::pushYellow(lua_State *)")
}

// 0x273390 — __ZN3RBX3LuaL9pushGreenEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushGreen(lua_State *)")]
pub fn stub_273390() -> ! {
    todo!("0x273390 RBX::Lua::pushGreen(lua_State *)")
}

// 0x2733a0 — __ZN3RBX3LuaL8pushBlueEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::pushBlue(lua_State *)")]
pub fn stub_2733a0() -> ! {
    todo!("0x2733a0 RBX::Lua::pushBlue(lua_State *)")
}

// 0x2733b0 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(RBX::BrickColor *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_index(RBX::BrickColor const&,char const*,lua_State *)")]
pub fn stub_2733b0() -> ! {
    todo!("0x2733b0 RBX::Lua::Bridge<RBX::BrickColor,true>::on_index(RBX::BrickColor const&,char const*,lua_State *)")
}

// 0x2735bc — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_newindex(RBX::BrickColor&,char const*,lua_State *)")]
pub fn stub_2735bc() -> ! {
    todo!("0x2735bc RBX::Lua::Bridge<RBX::BrickColor,true>::on_newindex(RBX::BrickColor&,char const*,lua_State *)")
}

// 0x273674 — __ZN3RBX3Lua21CoordinateFrameBridge18newCoordinateFrameEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::newCoordinateFrame(lua_State *)")]
pub fn stub_273674() -> ! {
    todo!("0x273674 RBX::Lua::CoordinateFrameBridge::newCoordinateFrame(lua_State *)")
}

// 0x27399c — __ZN3RBX3Lua21CoordinateFrameBridge18fromEulerAnglesXYZEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::fromEulerAnglesXYZ(lua_State *)")]
pub fn stub_27399c() -> ! {
    todo!("0x27399c RBX::Lua::CoordinateFrameBridge::fromEulerAnglesXYZ(lua_State *)")
}

// 0x273ad8 — __ZN3RBX3Lua21CoordinateFrameBridge13fromAxisAngleEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::fromAxisAngle(lua_State *)")]
pub fn stub_273ad8() -> ! {
    todo!("0x273ad8 RBX::Lua::CoordinateFrameBridge::fromAxisAngle(lua_State *)")
}

// 0x273bf0 — __ZN3RBX3Lua21CoordinateFrameBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::registerClassLibrary(lua_State *)")]
pub fn stub_273bf0() -> ! {
    todo!("0x273bf0 RBX::Lua::CoordinateFrameBridge::registerClassLibrary(lua_State *)")
}

// 0x273c2c — __ZN3RBX3Lua21CoordinateFrameBridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::CoordinateFrameBridge::on_add(lua_State *)")]
pub fn stub_273c2c() -> ! {
    todo!("0x273c2c RBX::Lua::CoordinateFrameBridge::on_add(lua_State *)")
}
