//! rendering — generated_watchdog_rend_wdM — 120 stubs (rendering filter G3D)
//! Source: ida/export.json (85545 funcs) rendering G3D filtered, global dedup
//! Range: 0x1840000..0x1840780 (120 stubs, step 0x10, synthetic gap above image end 0x13acefc)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! NOTE: all 85545 export EAs already stubbed workspace-wide; EAs below are
//! gap allocations above image end (0x13acefc); names/types donated by G3D-filtered
//! export entries (donor EA noted per stub). Filter: G3D:: namespace, global dedup via /tmp/global_eas.txt

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x1840000 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *) // donor 0xb740
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_")]
// IDA 0x1840000: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_1840000() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x1840010 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *) // donor 0xf704
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
#[doc(alias = "__ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// IDA 0x1840010: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_1840010() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x1840020 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int) // donor 0xf7e8
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm")]
// IDA 0x1840020: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_1840020() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x1840030 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int) // donor 0xf800
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_")]
// IDA 0x1840030: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_1840030() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x1840040 — __ZN3RBX5Light8setColorEN3G3D6Color3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *) // donor 0x25b4e0
#[doc(alias = "RBX::Light::setColor(G3D::Color3)")]
#[doc(alias = "__ZN3RBX5Light8setColorEN3G3D6Color3E")]
// IDA 0x1840040: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840040() {
}

// 0x1840050 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED1Ev
// type: _DWORD *__fastcall(_DWORD *) // donor 0x25c100
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED1Ev")]
// IDA 0x1840050: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_1840050() {
}

// 0x1840060 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int) // donor 0x25ed10
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
// IDA 0x1840060: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840060() {
}

// 0x1840070 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED0Ev
// type: int __fastcall(_DWORD *) // donor 0x25ee24
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED0Ev")]
// IDA 0x1840070: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_1840070() {
}

// 0x1840080 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// type: int() // donor 0x25ee50
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv")]
// IDA 0x1840080: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840080() {
}

// 0x1840090 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// type: int() // donor 0x25ee54
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv")]
// IDA 0x1840090: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840090() {
}

// 0x18400a0 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int) // donor 0x25ee58
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE")]
// IDA 0x18400a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18400a0() {
}

// 0x18400b0 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// type: int __fastcall(int, int, _DWORD *) // donor 0x25ee80
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_")]
// IDA 0x18400b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18400b0() {
}

// 0x18400c0 — __ZNK3RBX3Lua12LuaArguments15getVector3int16EiRN3G3D12Vector3int16E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, G3D::Vector3int16 *) // donor 0x26b4ac
#[doc(alias = "RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const")]
#[doc(alias = "__ZNK3RBX3Lua12LuaArguments15getVector3int16EiRN3G3D12Vector3int16E")]
// IDA 0x18400c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18400c0() {
}

// 0x18400d0 — __ZNK3RBX3Lua12LuaArguments10getVector3EiRN3G3D7Vector3E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, G3D::Vector3 *) // donor 0x26b504
#[doc(alias = "RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const")]
#[doc(alias = "__ZNK3RBX3Lua12LuaArguments10getVector3EiRN3G3D7Vector3E")]
// IDA 0x18400d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18400d0() {
}

// 0x18400e0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int) // donor 0x26c140
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_")]
// IDA 0x18400e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18400e0() {
}

// 0x18400f0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x26c230
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueIS3_EEbP9lua_StatejRT_")]
// IDA 0x18400f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18400f0() {
}

// 0x1840100 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x26c92c
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
// IDA 0x1840100: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840100() {
}

// 0x1840110 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x26caa0
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
// IDA 0x1840110: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840110() {
}

// 0x1840120 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x26cb1c
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
// IDA 0x1840120: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840120() {
}

// 0x1840130 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x26cb98
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
// IDA 0x1840130: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840130() {
}

// 0x1840140 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x26cc14
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
// IDA 0x1840140: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840140() {
}

// 0x1840150 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x26cd0c
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_")]
// IDA 0x1840150: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840150() {
}

// 0x1840160 — __ZN3rbx8any_castIRKN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****) // donor 0x26e8d0
#[doc(alias = "G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// IDA 0x1840160: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840160() {
}

// 0x1840170 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int) // donor 0x26e9c0
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")]
// IDA 0x1840170: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840170() {
}

// 0x1840180 — __ZN3rbx8any_castIRKN3G3D12Vector3int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****) // donor 0x26ea00
#[doc(alias = "G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3G3D12Vector3int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
// IDA 0x1840180: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840180() {
}

// 0x1840190 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: int __fastcall(int, int, __int16) // donor 0x26eaf0
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")]
// IDA 0x1840190: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840190() {
}

// 0x18401a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector2EEERS3_RKT_
// type: int **__fastcall(int **, int **) // donor 0x26f7b0
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector2EEERS3_RKT_")]
// IDA 0x18401a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18401a0() {
}

// 0x18401b0 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE9singletonEv
// type: int *() // donor 0x26f808
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE9singletonEv")]
// IDA 0x18401b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18401b0() {
}

// 0x18401c0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector3EEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *) // donor 0x26f878
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector3EEERS3_RKT_")]
// IDA 0x18401c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18401c0() {
}

// 0x18401d0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector3int16EEERS3_RKT_
// type: int __fastcall(int, int *) // donor 0x26f8d8
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector3int16EEERS3_RKT_")]
// IDA 0x18401d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18401d0() {
}

// 0x18401e0 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE9singletonEv
// type: int *() // donor 0x26f930
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE9singletonEv")]
// IDA 0x18401e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18401e0() {
}

// 0x18401f0 — __ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E
// type: int __fastcall(int, _DWORD *) // donor 0x2705d0
#[doc(alias = "RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")]
#[doc(alias = "__ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E")]
// IDA 0x18401f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18401f0() {
}

// 0x1840200 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(float *, char *__s1, int) // donor 0x2705ec
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State")]
// IDA 0x1840200: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840200() {
}

// 0x1840210 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *) // donor 0x270724
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State")]
// IDA 0x1840210: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840210() {
}

// 0x1840220 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int) // donor 0x271954
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State")]
// IDA 0x1840220: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840220() {
}

// 0x1840230 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *) // donor 0x271e14
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexERS3_PKcP9lua_State")]
// IDA 0x1840230: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840230() {
}

// 0x1840240 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int) // donor 0x272268
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexERKS3_PKcP9lua_State")]
// IDA 0x1840240: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840240() {
}

// 0x1840250 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *) // donor 0x2723d0
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexERS3_PKcP9lua_State")]
// IDA 0x1840250: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840250() {
}

// 0x1840260 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int) // donor 0x272804
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexERKS3_PKcP9lua_State")]
// IDA 0x1840260: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840260() {
}

// 0x1840270 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *) // donor 0x272940
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexERS3_PKcP9lua_State")]
// IDA 0x1840270: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840270() {
}

// 0x1840280 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int) // donor 0x272d70
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexERKS3_PKcP9lua_State")]
// IDA 0x1840280: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840280() {
}

// 0x1840290 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *) // donor 0x272fe4
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexERS3_PKcP9lua_State")]
// IDA 0x1840290: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840290() {
}

// 0x18402a0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(float *, char *__s1, int) // donor 0x2749f0
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(G3D::CoordinateFrame const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexERKS3_PKcP9lua_State")]
// IDA 0x18402a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18402a0() {
}

// 0x18402b0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *) // donor 0x274da0
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(G3D::CoordinateFrame&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexERS3_PKcP9lua_State")]
// IDA 0x18402b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18402b0() {
}

// 0x18402c0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int) // donor 0x276858
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_")]
// IDA 0x18402c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18402c0() {
}

// 0x18402d0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int) // donor 0x276a48
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_")]
// IDA 0x18402d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18402d0() {
}

// 0x18402e0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int) // donor 0x276c38
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_")]
// IDA 0x18402e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18402e0() {
}

// 0x18402f0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int) // donor 0x276e28
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13registerClassEP9lua_StatePFiS6_ES8_")]
// IDA 0x18402f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18402f0() {
}

// 0x1840300 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int) // donor 0x277018
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13registerClassEP9lua_StatePFiS6_ES8_")]
// IDA 0x1840300: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840300() {
}

// 0x1840310 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *) // donor 0x2774ac
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")]
// IDA 0x1840310: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840310() {
}

// 0x1840320 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *) // donor 0x2774f4
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_")]
// IDA 0x1840320: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840320() {
}

// 0x1840330 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int, int, int) // donor 0x27759c
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")]
// IDA 0x1840330: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840330() {
}

// 0x1840340 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *) // donor 0x2775ec
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_")]
// IDA 0x1840340: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840340() {
}

// 0x1840350 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
// type: _WORD *__fastcall(int, _DWORD *) // donor 0x277634
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_")]
// IDA 0x1840350: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840350() {
}

// 0x1840360 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x27767c
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_")]
// IDA 0x1840360: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840360() {
}

// 0x1840370 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
// type: G3D::Vector2int16 *__fastcall(int, int *) // donor 0x2776ec
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_")]
// IDA 0x1840370: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840370() {
}

// 0x1840380 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *) // donor 0x277730
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueIS3_EEbP9lua_StatejRT_")]
// IDA 0x1840380: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840380() {
}

// 0x1840390 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *) // donor 0x2777a8
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_")]
// IDA 0x1840390: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840390() {
}

// 0x18403a0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *) // donor 0x2777ec
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")]
// IDA 0x18403a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18403a0() {
}

// 0x18403b0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: G3D::Matrix3 *__fastcall(int, int) // donor 0x277894
#[doc(alias = "G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")]
// IDA 0x18403b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18403b0() {
}

// 0x18403c0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int) // donor 0x2778e4
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueIS3_EEbP9lua_StatejRT_")]
// IDA 0x18403c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18403c0() {
}

// 0x18403d0 — __ZN3G3D7Matrix313fromAxisAngleERKNS_7Vector3Ef
// type: int __fastcall(G3D::Matrix3 *this, const G3D::Vector3 *, float) // donor 0x27797c
#[doc(alias = "G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&,float)")]
#[doc(alias = "__ZN3G3D7Matrix313fromAxisAngleERKNS_7Vector3Ef")]
// IDA 0x18403d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18403d0() {
}

// 0x18403e0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int) // donor 0x277c90
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_gcEP9lua_State")]
// IDA 0x18403e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18403e0() {
}

// 0x18403f0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int) // donor 0x277cac
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_eqEP9lua_State")]
// IDA 0x18403f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18403f0() {
}

// 0x1840400 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int) // donor 0x277cf8
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringEP9lua_State")]
// IDA 0x1840400: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840400() {
}

// 0x1840410 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int) // donor 0x277d1c
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_gcEP9lua_State")]
// IDA 0x1840410: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840410() {
}

// 0x1840420 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int) // donor 0x277d38
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_eqEP9lua_State")]
// IDA 0x1840420: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840420() {
}

// 0x1840430 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int) // donor 0x277d74
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringEP9lua_State")]
// IDA 0x1840430: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840430() {
}

// 0x1840440 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int) // donor 0x277d98
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_gcEP9lua_State")]
// IDA 0x1840440: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840440() {
}

// 0x1840450 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int) // donor 0x277db4
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_eqEP9lua_State")]
// IDA 0x1840450: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840450() {
}

// 0x1840460 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int) // donor 0x277e20
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringEP9lua_State")]
// IDA 0x1840460: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840460() {
}

// 0x1840470 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int) // donor 0x277e44
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_gcEP9lua_State")]
// IDA 0x1840470: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840470() {
}

// 0x1840480 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int) // donor 0x277e60
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_eqEP9lua_State")]
// IDA 0x1840480: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840480() {
}

// 0x1840490 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int) // donor 0x277eb8
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringEP9lua_State")]
// IDA 0x1840490: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840490() {
}

// 0x18404a0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_gcEP9lua_State
// type: int __fastcall(int) // donor 0x277edc
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_gcEP9lua_State")]
// IDA 0x18404a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18404a0() {
}

// 0x18404b0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_eqEP9lua_State
// type: int __fastcall(int) // donor 0x277ef8
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_eqEP9lua_State")]
// IDA 0x18404b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18404b0() {
}

// 0x18404c0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int) // donor 0x277f70
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringEP9lua_State")]
// IDA 0x18404c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18404c0() {
}

// 0x18404d0 — __ZNK3RBX6RbxRay8distanceERKN3G3D7Vector3E
// type: float __fastcall(Vector3 *this, const G3D::Vector3 *) // donor 0x278084
#[doc(alias = "RBX::RbxRay::distance(G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX6RbxRay8distanceERKN3G3D7Vector3E")]
// IDA 0x18404d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18404d0() {
}

// 0x18404e0 — __ZNK3RBX6RbxRay12closestPointERKN3G3D7Vector3E
// type: _DWORD *__fastcall(_DWORD *this, const Vector3 *, __int32 *) // donor 0x2780dc
#[doc(alias = "RBX::RbxRay::closestPoint(G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX6RbxRay12closestPointERKN3G3D7Vector3E")]
// IDA 0x18404e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18404e0() {
}

// 0x18404f0 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int) // donor 0x27832c
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(G3D::Color3 const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringERKS3_P9lua_State")]
// IDA 0x18404f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18404f0() {
}

// 0x1840500 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int) // donor 0x278574
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State")]
// IDA 0x1840500: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840500() {
}

// 0x1840510 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int) // donor 0x278698
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State")]
// IDA 0x1840510: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840510() {
}

// 0x1840520 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int) // donor 0x2787bc
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State")]
// IDA 0x1840520: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840520() {
}

// 0x1840530 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int) // donor 0x2788e0
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State")]
// IDA 0x1840530: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840530() {
}

// 0x1840540 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State
// type: int __fastcall(int, int, int, int) // donor 0x278b28
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State")]
// IDA 0x1840540: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840540() {
}

// 0x1840550 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
// type: int __fastcall(int, int, int) // donor 0x279e44
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_")]
// IDA 0x1840550: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840550() {
}

// 0x1840560 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_gcEP9lua_State
// type: int __fastcall(int) // donor 0x279f58
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_gcEP9lua_State")]
// IDA 0x1840560: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840560() {
}

// 0x1840570 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_eqEP9lua_State
// type: int __fastcall(int) // donor 0x279f74
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_eqEP9lua_State")]
// IDA 0x1840570: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840570() {
}

// 0x1840580 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringEP9lua_State
// type: int __fastcall(int) // donor 0x279fe4
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringEP9lua_State")]
// IDA 0x1840580: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840580() {
}

// 0x1840590 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexEP9lua_State
// type:  // donor 0x2a3fc4
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexEP9lua_State")]
// IDA 0x1840590: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840590() {
}

// 0x18405a0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexEP9lua_State
// type:  // donor 0x2a3ff8
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexEP9lua_State")]
// IDA 0x18405a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18405a0() {
}

// 0x18405b0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexEP9lua_State
// type:  // donor 0x2a40fc
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexEP9lua_State")]
// IDA 0x18405b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18405b0() {
}

// 0x18405c0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexEP9lua_State
// type:  // donor 0x2a4130
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexEP9lua_State")]
// IDA 0x18405c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18405c0() {
}

// 0x18405d0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexEP9lua_State
// type:  // donor 0x2a4164
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexEP9lua_State")]
// IDA 0x18405d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18405d0() {
}

// 0x18405e0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexEP9lua_State
// type:  // donor 0x2a4198
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexEP9lua_State")]
// IDA 0x18405e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18405e0() {
}

// 0x18405f0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexEP9lua_State
// type:  // donor 0x2a41cc
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexEP9lua_State")]
// IDA 0x18405f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18405f0() {
}

// 0x1840600 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexEP9lua_State
// type:  // donor 0x2a4200
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexEP9lua_State")]
// IDA 0x1840600: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840600() {
}

// 0x1840610 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexEP9lua_State
// type:  // donor 0x2a4234
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexEP9lua_State")]
// IDA 0x1840610: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840610() {
}

// 0x1840620 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexEP9lua_State
// type:  // donor 0x2a4268
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexEP9lua_State")]
// IDA 0x1840620: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840620() {
}

// 0x1840630 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexEP9lua_State
// type:  // donor 0x2a4304
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexEP9lua_State")]
// IDA 0x1840630: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840630() {
}

// 0x1840640 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexEP9lua_State
// type:  // donor 0x2a4338
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexEP9lua_State")]
// IDA 0x1840640: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840640() {
}

// 0x1840650 — __ZN3RBX11AdvDragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int) // donor 0x2ce2c4
#[doc(alias = "RBX::AdvDragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX11AdvDragTool11onMouseDownEPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIPNS_8InstanceESaIS9_EERKNS_7UIEventEPNS_9WorkspaceEN5boost10shared_ptrIS8_EE")]
// IDA 0x1840650: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840650() {
}

// 0x1840660 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14AdvLuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, void *, char, int, int, int, int) // donor 0x2ce4e8
#[doc(alias = "boost::shared_ptr<RBX::AdvLuaDragTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvLuaDragTool,RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>>(RBX::PartInstance *,G3D::Vector3,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_14AdvLuaDragToolEPNS_12PartInstanceEN3G3D7Vector3ESt6vectorIN5boost8weak_ptrIS5_EESaISC_EEPNS_9WorkspaceENSA_10shared_ptrINS_8InstanceEEEEENSH_IT_EET0_T1_T2_T3_T4_")]
// IDA 0x1840660: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840660() {
}

// 0x1840670 — __ZN3RBX13AdvLuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE
// type:  // donor 0x2cf178
#[doc(alias = "RBX::AdvLuaDragger::mouseDown(boost::shared_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger9mouseDownEN5boost10shared_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorINS1_8weak_ptrIS3_EESaISB_EE")]
// IDA 0x1840670: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840670() {
}

// 0x1840680 — __ZN3RBX13AdvLuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this, RBX::PartInstance *, const RBX::RbxRay *, G3D::Vector3 *) // donor 0x2d0030
#[doc(alias = "RBX::AdvLuaDragger::getSnapHitPoint(RBX::PartInstance *,RBX::RbxRay const&,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger15getSnapHitPointEPNS_12PartInstanceERKNS_6RbxRayERN3G3D7Vector3E")]
// IDA 0x1840680: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840680() {
}

// 0x1840690 — __ZN3RBX13AdvLuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this, Axis, const G3D::Matrix3 *) // donor 0x2d03b0
#[doc(alias = "RBX::AdvLuaDragger::rotateOnSnapFace(G3D::Vector3::Axis,G3D::Matrix3 const&)")]
#[doc(alias = "__ZN3RBX13AdvLuaDragger16rotateOnSnapFaceEN3G3D7Vector34AxisERKNS1_7Matrix3E")]
// IDA 0x1840690: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840690() {
}

// 0x18406a0 — __ZN3RBX14AdvLuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
// type:  // donor 0x2d17c4
#[doc(alias = "RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragToolC1EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE")]
// IDA 0x18406a0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18406a0() {
}

// 0x18406b0 — __ZN3RBX14AdvLuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, std::string *, int, int, int, int) // donor 0x2d17c8
#[doc(alias = "RBX::AdvLuaDragTool::AdvLuaDragTool(RBX::PartInstance *,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>> const&,RBX::Workspace *,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX14AdvLuaDragToolC2EPNS_12PartInstanceERKN3G3D7Vector3ERKSt6vectorIN5boost8weak_ptrIS1_EESaISA_EEPNS_9WorkspaceENS8_10shared_ptrINS_8InstanceEEE")]
// IDA 0x18406b0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18406b0() {
}

// 0x18406c0 — __ZNK3RBX15AdvMoveToolBase21getExtentsAndLocationERNS_7ExtentsERN3G3D15CoordinateFrameERb
// type: _DWORD __fastcall(RBX::AdvMoveToolBase *__hidden this, RBX::Extents *, G3D::CoordinateFrame *, bool *) // donor 0x2d3d4c
#[doc(alias = "RBX::AdvMoveToolBase::getExtentsAndLocation(RBX::Extents &,G3D::CoordinateFrame &,bool &)const")]
#[doc(alias = "__ZNK3RBX15AdvMoveToolBase21getExtentsAndLocationERNS_7ExtentsERN3G3D15CoordinateFrameERb")]
// IDA 0x18406c0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18406c0() {
}

// 0x18406d0 — __ZNK3RBX15AdvMoveToolBase13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE
// type:  // donor 0x2d487c
#[doc(alias = "RBX::AdvMoveToolBase::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
#[doc(alias = "__ZNK3RBX15AdvMoveToolBase13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE")]
// IDA 0x18406d0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18406d0() {
}

// 0x18406e0 — __ZN3RBX11AdvMoveTool20getGridXYUsingCameraEPNS_12PartInstanceERN3G3D7Vector3ES5_
// type: _DWORD __fastcall(RBX::AdvMoveTool *__hidden this, RBX::PartInstance *, G3D::Vector3 *, G3D::Vector3 *) // donor 0x2d4d38
#[doc(alias = "RBX::AdvMoveTool::getGridXYUsingCamera(RBX::PartInstance *,G3D::Vector3 &,G3D::Vector3 &)")]
#[doc(alias = "__ZN3RBX11AdvMoveTool20getGridXYUsingCameraEPNS_12PartInstanceERN3G3D7Vector3ES5_")]
// IDA 0x18406e0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18406e0() {
}

// 0x18406f0 — __ZNK3RBX13AdvRotateTool13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x2d5da0
#[doc(alias = "RBX::AdvRotateTool::getOverHandle(RBX::UIEvent const&,G3D::Vector3 &,RBX::NormalId &)const")]
#[doc(alias = "__ZNK3RBX13AdvRotateTool13getOverHandleERKNS_7UIEventERN3G3D7Vector3ERNS_8NormalIdE")]
// IDA 0x18406f0: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_18406f0() {
}

// 0x1840700 — __ZN3RBX13AdvRunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorIS6_SaIS6_EE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int) // donor 0x2d7074
#[doc(alias = "RBX::AdvRunDragger::initLocal(RBX::Workspace *,boost::weak_ptr<RBX::PartInstance>,G3D::Vector3 const&,std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger9initLocalEPNS_9WorkspaceEN5boost8weak_ptrINS_12PartInstanceEEERKN3G3D7Vector3ESt6vectorIS6_SaIS6_EE")]
// IDA 0x1840700: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840700() {
}

// 0x1840710 — __ZN3RBX13AdvRunDragger17createSnapSurfaceEPNS_9PrimitiveEPN3G3D5ArrayImLi10ELm32EEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int) // donor 0x2d7610
#[doc(alias = "RBX::AdvRunDragger::createSnapSurface(RBX::Primitive *,G3D::Array<unsigned long,10,32ul> *)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger17createSnapSurfaceEPNS_9PrimitiveEPN3G3D5ArrayImLi10ELm32EEE")]
// IDA 0x1840710: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840710() {
}

// 0x1840720 — __ZN3RBX13AdvRunDragger8notTriedEPNS_9PrimitiveERKN3G3D5ArrayIS2_Li10ELm32EEE
// type:  // donor 0x2d89e8
#[doc(alias = "RBX::AdvRunDragger::notTried(RBX::Primitive *,G3D::Array<RBX::Primitive *,10,32ul> const&)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger8notTriedEPNS_9PrimitiveERKN3G3D5ArrayIS2_Li10ELm32EEE")]
// IDA 0x1840720: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840720() {
}

// 0x1840730 — __ZN3RBX13AdvRunDragger11rayHitsPartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEb
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int) // donor 0x2d8ab8
#[doc(alias = "RBX::AdvRunDragger::rayHitsPart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger11rayHitsPartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEb")]
// IDA 0x1840730: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840730() {
}

// 0x1840740 — __ZN3RBX13AdvRunDragger17bestProximatePartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEMNS_7ContactEFbfE
// type: int __fastcall(int, int, int, int, int) // donor 0x2d8ce4
#[doc(alias = "RBX::AdvRunDragger::bestProximatePart(G3D::Array<RBX::Primitive *,10,32ul> const&,bool (RBX::Contact::*)(float))")]
#[doc(alias = "__ZN3RBX13AdvRunDragger17bestProximatePartERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEEMNS_7ContactEFbfE")]
// IDA 0x1840740: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840740() {
}

// 0x1840750 — __ZN3RBX13AdvRunDragger8findSnapERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE
// type:  // donor 0x2d91b0
#[doc(alias = "RBX::AdvRunDragger::findSnap(G3D::Array<RBX::Primitive *,10,32ul> const&)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger8findSnapERKN3G3D5ArrayIPNS_9PrimitiveELi10ELm32EEE")]
// IDA 0x1840750: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840750() {
}

// 0x1840760 — __ZN3RBX13AdvRunDragger18findNoSnapPositionERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this, const G3D::CoordinateFrame *) // donor 0x2d92dc
#[doc(alias = "RBX::AdvRunDragger::findNoSnapPosition(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger18findNoSnapPositionERKN3G3D15CoordinateFrameE")]
// IDA 0x1840760: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840760() {
}

// 0x1840770 — __ZN3RBX13AdvRunDragger32rotatePart90DegAboutSnapFaceAxisEN3G3D7Vector34AxisE
// type: _DWORD __fastcall(RBX::AdvRunDragger *__hidden this, Axis) // donor 0x2d98c8
#[doc(alias = "RBX::AdvRunDragger::rotatePart90DegAboutSnapFaceAxis(G3D::Vector3::Axis)")]
#[doc(alias = "__ZN3RBX13AdvRunDragger32rotatePart90DegAboutSnapFaceAxisEN3G3D7Vector34AxisE")]
// IDA 0x1840770: unresolved. // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_1840770() {
}
