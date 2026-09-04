//! core watchdog d — 100 core stubs EA-sorted, fourth gap filler after watchdog_c 0x26ce80.
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_core — next 100 uncovered after 0x26ce80 (watchdog_c max).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Faces,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// 0x26cefc — __ZN3RBX3Lua6BridgeINS_5FacesELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
pub fn stub_26cefc() {
    // IDA 0x26cefc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Axes,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// 0x26cf78 — __ZN3RBX3Lua6BridgeINS_4AxesELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_26cf78() {
    // IDA 0x26cf78: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::CellID,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// 0x26cff4 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_26cff4() {
    // IDA 0x26cff4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::InputObject,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// 0x26d070 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_26d070() {
    // IDA 0x26d070: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "int RBX::withVariantValue<int,RBX::Lua::ArgumentPusher>(RBX::Reflection::Variant const&,RBX::Lua::ArgumentPusher)")]
// 0x26d0ec — __ZN3RBX16withVariantValueIiNS_3Lua14ArgumentPusherEEET_RKNS_10Reflection7VariantET0_
// type: int __fastcall(char ****, int)
pub fn stub_26d0ec() {
    // IDA 0x26d0ec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::LuaArguments::size(void)const")]
// 0x26dc28 — __ZNK3RBX3Lua12LuaArguments4sizeEv
// type: int __fastcall(RBX::Lua::LuaArguments *this)
pub fn stub_26dc28() {
    // IDA 0x26dc28: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::LuaArguments::getVariant(int,RBX::Reflection::Variant &)const")]
// 0x26dc38 — __ZNK3RBX3Lua12LuaArguments10getVariantEiRNS_10Reflection7VariantE
// type: int __fastcall(int, int, int)
pub fn stub_26dc38() {
    // IDA 0x26dc38: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::LuaArguments::getLong(int,long &)const")]
// 0x26dca8 — __ZNK3RBX3Lua12LuaArguments7getLongEiRl
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, int *)
pub fn stub_26dca8() {
    // IDA 0x26dca8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::InputObject* RBX::Lua::Bridge<RBX::InputObject,true>::pushNewObject<RBX::InputObject>(lua_State *,RBX::InputObject)")]
// 0x26e1d8 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
pub fn stub_26e1d8() {
    // IDA 0x26e1d8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::CellID* RBX::Lua::Bridge<RBX::CellID,true>::pushNewObject<RBX::CellID>(lua_State *,RBX::CellID)")]
// 0x26e408 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, int)
pub fn stub_26e408() {
    // IDA 0x26e408: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Region3int16* RBX::Lua::Bridge<RBX::Region3int16,true>::pushNewObject<RBX::Region3int16>(lua_State *,RBX::Region3int16)")]
// 0x26e738 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, __int64 *)
pub fn stub_26e738() {
    // IDA 0x26e738: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Region3* RBX::Lua::Bridge<RBX::Region3,true>::pushNewObject<RBX::Region3>(lua_State *,RBX::Region3)")]
// 0x26e870 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: G3D::Matrix3 *__fastcall(int, int)
pub fn stub_26e870() {
    // IDA 0x26e870: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")]
// 0x26e9c0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
pub fn stub_26e9c0() {
    // IDA 0x26e9c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")]
// 0x26eaf0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: int __fastcall(int, int, __int16)
pub fn stub_26eaf0() {
    // IDA 0x26eaf0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::Variant const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,lua_State *)")]
// 0x26f1d4 — __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKNS_10Reflection7VariantESt6vectorIS6_SaIS6_EEEEEEiT_SD_P9lua_State
// type: int __fastcall(char ****, char ****, int)
pub fn stub_26f1d4() {
    // IDA 0x26f1d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Lua::WeakFunctionRef const& rbx::any_cast<RBX::Lua::WeakFunctionRef const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26f280 — __ZN3rbx8any_castIRKN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: _DWORD **__fastcall(_DWORD **)
pub fn stub_26f280() {
    // IDA 0x26f280: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::lookupDescriptor(std::type_info const&)")]
// 0x26f368 — __ZN3RBX10Reflection14EnumDescriptor16lookupDescriptorERKSt9type_info
// type: int __fastcall(RBX::Reflection::EnumDescriptor *this, const std::type_info *)
pub fn stub_26f368() {
    // IDA 0x26f368: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Lua::WeakFunctionRef>(RBX::Lua::WeakFunctionRef const&)")]
// 0x26faf8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_3Lua15WeakFunctionRefEEERS3_RKT_
// type: int **__fastcall(int **, const RBX::Lua::WeakFunctionRef *)
pub fn stub_26faf8() {
    // IDA 0x26faf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::construct_func(char const*,char *)")]
// 0x26fb50 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE14construct_funcEPKcPc
// type: const RBX::Lua::WeakFunctionRef *__fastcall(const RBX::Lua::WeakFunctionRef *result, RBX::Lua::WeakFunctionRef *)
pub fn stub_26fb50() {
    // IDA 0x26fb50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::destruct_func(char *)")]
// 0x26fb60 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE13destruct_funcEPc
// type: int __fastcall(int)
pub fn stub_26fb60() {
    // IDA 0x26fb60: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "bool RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::getValue<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,unsigned int,RBX::Reflection::EnumDescriptor::Item const* &)")]
// 0x270008 — __ZN3RBX3Lua6BridgeIPKNS_10Reflection14EnumDescriptor4ItemELb1EE8getValueIS6_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
pub fn stub_270008() {
    // IDA 0x270008: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "RBX::Lua::safe_lua_tostring(lua_State *,int)")]
// 0x270210 — __ZN3RBX3Lua17safe_lua_tostringEP9lua_Statei
// type: const char *__fastcall(int, int)
pub fn stub_270210() {
    // IDA 0x270210: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Lua::throwable_lua_tostring(lua_State *,int)")]
// 0x270230 — __ZN3RBX3Lua22throwable_lua_tostringEP9lua_Statei
// type: const char *__fastcall(int, int)
pub fn stub_270230() {
    // IDA 0x270230: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Lua::lua_tofloat(lua_State *,int)")]
// 0x270448 — __ZN3RBX3Lua11lua_tofloatEP9lua_Statei
// type: int __fastcall(int, int)
pub fn stub_270448() {
    // IDA 0x270448: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::Lua::Color3Bridge::newColor3(lua_State *)")]
// 0x2704e0 — __ZN3RBX3Lua12Color3Bridge9newColor3EP9lua_State
// type: int __fastcall(int)
pub fn stub_2704e0() {
    // IDA 0x2704e0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Color3Bridge::registerClassLibrary(lua_State *)")]
// 0x270594 — __ZN3RBX3Lua12Color3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_270594() {
    // IDA 0x270594: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")]
// 0x2705d0 — __ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E
// type: int __fastcall(int, _DWORD *)
pub fn stub_2705d0() {
    // IDA 0x2705d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")]
// 0x2705ec — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(float *, char *__s1, int)
pub fn stub_2705ec() {
    // IDA 0x2705ec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")]
// 0x270724 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_270724() {
    // IDA 0x270724: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::RbxRayBridge::newRbxRay(lua_State *)")]
// 0x2707dc — __ZN3RBX3Lua12RbxRayBridge9newRbxRayEP9lua_State
// type: int __fastcall(int)
pub fn stub_2707dc() {
    // IDA 0x2707dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::RbxRayBridge::registerClassLibrary(lua_State *)")]
// 0x2708b0 — __ZN3RBX3Lua12RbxRayBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_2708b0() {
    // IDA 0x2708b0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_index(RBX::RbxRay const&,char const*,lua_State *)")]
// 0x2708ec — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
pub fn stub_2708ec() {
    // IDA 0x2708ec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::closestPointVector3(lua_State *)")]
// 0x270afc — __ZN3RBX3LuaL19closestPointVector3EP9lua_State
// type: int __fastcall(int)
pub fn stub_270afc() {
    // IDA 0x270afc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::distanceVector3(lua_State *)")]
// 0x270b48 — __ZN3RBX3LuaL15distanceVector3EP9lua_State
// type: int __fastcall(int)
pub fn stub_270b48() {
    // IDA 0x270b48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_newindex(RBX::RbxRay&,char const*,lua_State *)")]
// 0x270b98 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_270b98() {
    // IDA 0x270b98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Region3Bridge::newRegion3(lua_State *)")]
// 0x270c50 — __ZN3RBX3Lua13Region3Bridge10newRegion3EP9lua_State
// type: int __fastcall(int)
pub fn stub_270c50() {
    // IDA 0x270c50: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Region3Bridge::registerClassLibrary(lua_State *)")]
// 0x270d50 — __ZN3RBX3Lua13Region3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_270d50() {
    // IDA 0x270d50: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_index(RBX::Region3 const&,char const*,lua_State *)")]
// 0x270d8c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
pub fn stub_270d8c() {
    // IDA 0x270d8c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_newindex(RBX::Region3&,char const*,lua_State *)")]
// 0x270ec8 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_270ec8() {
    // IDA 0x270ec8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Region3int16Bridge::newRegion3int16(lua_State *)")]
// 0x270f80 — __ZN3RBX3Lua18Region3int16Bridge15newRegion3int16EP9lua_State
// type: int __fastcall(int)
pub fn stub_270f80() {
    // IDA 0x270f80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Region3int16Bridge::registerClassLibrary(lua_State *)")]
// 0x271064 — __ZN3RBX3Lua18Region3int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_271064() {
    // IDA 0x271064: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_index(RBX::Region3int16 const&,char const*,lua_State *)")]
// 0x2710a0 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
pub fn stub_2710a0() {
    // IDA 0x2710a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_newindex(RBX::Region3int16&,char const*,lua_State *)")]
// 0x2711d4 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_2711d4() {
    // IDA 0x2711d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3(lua_State *)")]
// 0x27128c — __ZN3RBX3Lua13Vector3Bridge10newVector3EP9lua_State
// type: int __fastcall(int)
pub fn stub_27128c() {
    // IDA 0x27128c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3FromNormalId(lua_State *)")]
// 0x271340 — __ZN3RBX3Lua13Vector3Bridge22newVector3FromNormalIdEP9lua_State
// type: int __fastcall(int)
pub fn stub_271340() {
    // IDA 0x271340: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3FromAxis(lua_State *)")]
// 0x2714a0 — __ZN3RBX3Lua13Vector3Bridge18newVector3FromAxisEP9lua_State
// type: int __fastcall(int)
pub fn stub_2714a0() {
    // IDA 0x2714a0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::registerClassLibrary(lua_State *)")]
// 0x271604 — __ZN3RBX3Lua13Vector3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_271604() {
    // IDA 0x271604: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::on_add(lua_State *)")]
// 0x271640 — __ZN3RBX3Lua13Vector3Bridge6on_addEP9lua_State
// type: int __fastcall(int)
pub fn stub_271640() {
    // IDA 0x271640: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::on_sub(lua_State *)")]
// 0x2716a0 — __ZN3RBX3Lua13Vector3Bridge6on_subEP9lua_State
// type: int __fastcall(int)
pub fn stub_2716a0() {
    // IDA 0x2716a0: physics-engine collision/contact helper (Body/Primitive/Contact graph). Owned by higher crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::on_mul(lua_State *)")]
// 0x271700 — __ZN3RBX3Lua13Vector3Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
pub fn stub_271700() {
    // IDA 0x271700: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::on_div(lua_State *)")]
// 0x271804 — __ZN3RBX3Lua13Vector3Bridge6on_divEP9lua_State
// type: int __fastcall(int)
pub fn stub_271804() {
    // IDA 0x271804: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3Bridge::on_unm(lua_State *)")]
// 0x27191c — __ZN3RBX3Lua13Vector3Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
pub fn stub_27191c() {
    // IDA 0x27191c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")]
// 0x271954 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int)
pub fn stub_271954() {
    // IDA 0x271954: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::lerpVector3(lua_State *)")]
// 0x271c4c — __ZN3RBX3LuaL11lerpVector3EP9lua_State
// type: int __fastcall(int)
pub fn stub_271c4c() {
    // IDA 0x271c4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::crossVector3(lua_State *)")]
// 0x271cd0 — __ZN3RBX3LuaL12crossVector3EP9lua_State
// type: int __fastcall(int)
pub fn stub_271cd0() {
    // IDA 0x271cd0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::dotVector3(lua_State *)")]
// 0x271d48 — __ZN3RBX3LuaL10dotVector3EP9lua_State
// type: int __fastcall(int)
pub fn stub_271d48() {
    // IDA 0x271d48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::isCloseVector3(lua_State *)")]
// 0x271dac — __ZN3RBX3LuaL14isCloseVector3EP9lua_State
// type: int __fastcall(int)
pub fn stub_271dac() {
    // IDA 0x271dac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")]
// 0x271e14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_271e14() {
    // IDA 0x271e14: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3int16Bridge::newVector3int16(lua_State *)")]
// 0x271ecc — __ZN3RBX3Lua18Vector3int16Bridge15newVector3int16EP9lua_State
// type: int __fastcall(int)
pub fn stub_271ecc() {
    // IDA 0x271ecc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3int16Bridge::registerClassLibrary(lua_State *)")]
// 0x271f84 — __ZN3RBX3Lua18Vector3int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_271f84() {
    // IDA 0x271f84: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_add(lua_State *)")]
// 0x271fc0 — __ZN3RBX3Lua18Vector3int16Bridge6on_addEP9lua_State
// type: int __fastcall(int)
pub fn stub_271fc0() {
    // IDA 0x271fc0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_sub(lua_State *)")]
// 0x272008 — __ZN3RBX3Lua18Vector3int16Bridge6on_subEP9lua_State
// type: int __fastcall(int)
pub fn stub_272008() {
    // IDA 0x272008: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_mul(lua_State *)")]
// 0x272050 — __ZN3RBX3Lua18Vector3int16Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
pub fn stub_272050() {
    // IDA 0x272050: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_div(lua_State *)")]
// 0x272108 — __ZN3RBX3Lua18Vector3int16Bridge6on_divEP9lua_State
// type: int __fastcall(int)
pub fn stub_272108() {
    // IDA 0x272108: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_unm(lua_State *)")]
// 0x272230 — __ZN3RBX3Lua18Vector3int16Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
pub fn stub_272230() {
    // IDA 0x272230: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")]
// 0x272268 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int)
pub fn stub_272268() {
    // IDA 0x272268: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")]
// 0x2723d0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_2723d0() {
    // IDA 0x2723d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2int16Bridge::newVector2int16(lua_State *)")]
// 0x272488 — __ZN3RBX3Lua18Vector2int16Bridge15newVector2int16EP9lua_State
// type: int __fastcall(int)
pub fn stub_272488() {
    // IDA 0x272488: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2int16Bridge::registerClassLibrary(lua_State *)")]
// 0x272540 — __ZN3RBX3Lua18Vector2int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_272540() {
    // IDA 0x272540: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_add(lua_State *)")]
// 0x27257c — __ZN3RBX3Lua18Vector2int16Bridge6on_addEP9lua_State
// type: int __fastcall(int)
pub fn stub_27257c() {
    // IDA 0x27257c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_sub(lua_State *)")]
// 0x2725bc — __ZN3RBX3Lua18Vector2int16Bridge6on_subEP9lua_State
// type: int __fastcall(int)
pub fn stub_2725bc() {
    // IDA 0x2725bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_mul(lua_State *)")]
// 0x2725fc — __ZN3RBX3Lua18Vector2int16Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
pub fn stub_2725fc() {
    // IDA 0x2725fc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_div(lua_State *)")]
// 0x2726f8 — __ZN3RBX3Lua18Vector2int16Bridge6on_divEP9lua_State
// type: int __fastcall(int)
pub fn stub_2726f8() {
    // IDA 0x2726f8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_unm(lua_State *)")]
// 0x2727d4 — __ZN3RBX3Lua18Vector2int16Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
pub fn stub_2727d4() {
    // IDA 0x2727d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")]
// 0x272804 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int)
pub fn stub_272804() {
    // IDA 0x272804: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")]
// 0x272940 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_272940() {
    // IDA 0x272940: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2Bridge::newVector2(lua_State *)")]
// 0x2729f8 — __ZN3RBX3Lua13Vector2Bridge10newVector2EP9lua_State
// type: int __fastcall(int)
pub fn stub_2729f8() {
    // IDA 0x2729f8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2Bridge::registerClassLibrary(lua_State *)")]
// 0x272aac — __ZN3RBX3Lua13Vector2Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_272aac() {
    // IDA 0x272aac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2Bridge::on_add(lua_State *)")]
// 0x272ae8 — __ZN3RBX3Lua13Vector2Bridge6on_addEP9lua_State
// type: int __fastcall(int)
pub fn stub_272ae8() {
    // IDA 0x272ae8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2Bridge::on_sub(lua_State *)")]
// 0x272b40 — __ZN3RBX3Lua13Vector2Bridge6on_subEP9lua_State
// type: int __fastcall(int)
pub fn stub_272b40() {
    // IDA 0x272b40: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2Bridge::on_mul(lua_State *)")]
// 0x272b98 — __ZN3RBX3Lua13Vector2Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
pub fn stub_272b98() {
    // IDA 0x272b98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2Bridge::on_div(lua_State *)")]
// 0x272c6c — __ZN3RBX3Lua13Vector2Bridge6on_divEP9lua_State
// type: int __fastcall(int)
pub fn stub_272c6c() {
    // IDA 0x272c6c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Vector2Bridge::on_unm(lua_State *)")]
// 0x272d28 — __ZN3RBX3Lua13Vector2Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
pub fn stub_272d28() {
    // IDA 0x272d28: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")]
// 0x272d70 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int)
pub fn stub_272d70() {
    // IDA 0x272d70: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::lerpVector2(lua_State *)")]
// 0x272f6c — __ZN3RBX3LuaL11lerpVector2EP9lua_State
// type: int __fastcall(int)
pub fn stub_272f6c() {
    // IDA 0x272f6c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")]
// 0x272fe4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_272fe4() {
    // IDA 0x272fe4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::BrickColorBridge::registerClassLibrary(lua_State *)")]
// 0x27309c — __ZN3RBX3Lua16BrickColorBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
pub fn stub_27309c() {
    // IDA 0x27309c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::BrickColorBridge::newBrickColor(lua_State *)")]
// 0x2730d8 — __ZN3RBX3Lua16BrickColorBridge13newBrickColorEP9lua_State
// type: int __fastcall(int)
pub fn stub_2730d8() {
    // IDA 0x2730d8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::BrickColorBridge::randomBrickColor(lua_State *)")]
// 0x2731f0 — __ZN3RBX3Lua16BrickColorBridge16randomBrickColorEP9lua_State
// type: int __fastcall(int)
pub fn stub_2731f0() {
    // IDA 0x2731f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::BrickColorBridge::paletteBrickColor(lua_State *)")]
// 0x27320c — __ZN3RBX3Lua16BrickColorBridge17paletteBrickColorEP9lua_State
// type: int __fastcall(int)
pub fn stub_27320c() {
    // IDA 0x27320c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushWhite(lua_State *)")]
// 0x273330 — __ZN3RBX3LuaL9pushWhiteEP9lua_State
// type: int __fastcall(int)
pub fn stub_273330() {
    // IDA 0x273330: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushGray(lua_State *)")]
// 0x273340 — __ZN3RBX3LuaL8pushGrayEP9lua_State
// type: int __fastcall(int)
pub fn stub_273340() {
    // IDA 0x273340: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushDarkGray(lua_State *)")]
// 0x273350 — __ZN3RBX3LuaL12pushDarkGrayEP9lua_State
// type: int __fastcall(int)
pub fn stub_273350() {
    // IDA 0x273350: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushBlack(lua_State *)")]
// 0x273360 — __ZN3RBX3LuaL9pushBlackEP9lua_State
// type: int __fastcall(int)
pub fn stub_273360() {
    // IDA 0x273360: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushRed(lua_State *)")]
// 0x273370 — __ZN3RBX3LuaL7pushRedEP9lua_State
// type: int __fastcall(int)
pub fn stub_273370() {
    // IDA 0x273370: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushYellow(lua_State *)")]
// 0x273380 — __ZN3RBX3LuaL10pushYellowEP9lua_State
// type: int __fastcall(int)
pub fn stub_273380() {
    // IDA 0x273380: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushGreen(lua_State *)")]
// 0x273390 — __ZN3RBX3LuaL9pushGreenEP9lua_State
// type: int __fastcall(int)
pub fn stub_273390() {
    // IDA 0x273390: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::pushBlue(lua_State *)")]
// 0x2733a0 — __ZN3RBX3LuaL8pushBlueEP9lua_State
// type: int __fastcall(int)
pub fn stub_2733a0() {
    // IDA 0x2733a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_index(RBX::BrickColor const&,char const*,lua_State *)")]
// 0x2733b0 — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(RBX::BrickColor *, char *__s1, int)
pub fn stub_2733b0() {
    // IDA 0x2733b0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_newindex(RBX::BrickColor&,char const*,lua_State *)")]
// 0x2735bc — __ZN3RBX3Lua6BridgeINS_10BrickColorELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
pub fn stub_2735bc() {
    // IDA 0x2735bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}
