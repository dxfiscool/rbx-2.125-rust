//! rendering shard b — next 100 Gfx|G3D stubs (EA-sorted)
//! Filter: Gfx|G3D (4083 total, 50 prior stubbed, 100 this batch, 3933 remaining)
//! This shard: 0xb740..0x2a4338
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
// was: std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)
// IDA 0xb740: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_b740() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
// was: std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)
// IDA 0xf704: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f704() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)
// IDA 0xf7e8: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_f7e8() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
// was: G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)
// IDA 0xf800: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f800() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x25b4e0 — __ZN3RBX5Light8setColorEN3G3D6Color3E
#[doc(alias = "RBX::Light::setColor(G3D::Color3)")]
// was: RBX::Light::setColor(G3D::Color3)
// IDA 0x25b4e0: 30 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_25b4e0() {
}

// 0x25c100 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()
// IDA 0x25c100: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_25c100() {
}

// 0x25ed10 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x25ed10: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_25ed10() {
}

// 0x25ee24 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()
// IDA 0x25ee24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_25ee24() {
}

// 0x25ee50 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isReadOnly(void)const
// IDA 0x25ee50: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_25ee50() {
}

// 0x25ee54 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isWriteOnly(void)const
// IDA 0x25ee54: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_25ee54() {
}

// 0x25ee58 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x25ee58: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_25ee58() {
}

// 0x25ee80 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
// IDA 0x25ee80: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_25ee80() {
}

// 0x26b4ac — __ZNK3RBX3Lua12LuaArguments15getVector3int16EiRN3G3D12Vector3int16E
#[doc(alias = "RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const")]
// was: RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const
// IDA 0x26b4ac: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26b4ac() {
}

// 0x26b504 — __ZNK3RBX3Lua12LuaArguments10getVector3EiRN3G3D7Vector3E
#[doc(alias = "RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const")]
// was: RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const
// IDA 0x26b504: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26b504() {
}

// 0x26c140 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)
// IDA 0x26c140: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26c140() {
}

// 0x26c230 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)
// IDA 0x26c230: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26c230() {
}

// 0x26c92c — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// IDA 0x26c92c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26c92c() {
}

// 0x26caa0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// IDA 0x26caa0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26caa0() {
}

// 0x26cb1c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// IDA 0x26cb1c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26cb1c() {
}

// 0x26cb98 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// IDA 0x26cb98: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26cb98() {
}

// 0x26cc14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// IDA 0x26cc14: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26cc14() {
}

// 0x26cd0c — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
// IDA 0x26cd0c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26cd0c() {
}

// 0x26e8d0 — __ZN3rbx8any_castIRKN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26e8d0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e8d0() {
}

// 0x26e9c0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")]
// was: G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)
// IDA 0x26e9c0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26e9c0() {
}

// 0x26ea00 — __ZN3rbx8any_castIRKN3G3D12Vector3int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0x26ea00: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26ea00() {
}

// 0x26eaf0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")]
// was: G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)
// IDA 0x26eaf0: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26eaf0() {
}

// 0x26f7b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector2EEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)
// IDA 0x26f7b0: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26f7b0() {
}

// 0x26f808 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)
// IDA 0x26f808: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26f808() {
}

// 0x26f878 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector3EEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)
// IDA 0x26f878: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26f878() {
}

// 0x26f8d8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector3int16EEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)
// IDA 0x26f8d8: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26f8d8() {
}

// 0x26f930 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)
// IDA 0x26f930: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_26f930() {
}

// 0x2705d0 — __ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E
#[doc(alias = "RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")]
// was: RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)
// IDA 0x2705d0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2705d0() {
}

// 0x2705ec — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)
// IDA 0x2705ec: 97 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2705ec() {
}

// 0x270724 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)
// IDA 0x270724: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_270724() {
}

// 0x271954 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)
// IDA 0x271954: 231 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_271954() {
}

// 0x271e14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)
// IDA 0x271e14: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_271e14() {
}

// 0x272268 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)
// IDA 0x272268: 113 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_272268() {
}

// 0x2723d0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)
// IDA 0x2723d0: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2723d0() {
}

// 0x272804 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)
// IDA 0x272804: 98 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_272804() {
}

// 0x272940 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)
// IDA 0x272940: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_272940() {
}

// 0x272d70 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)
// IDA 0x272d70: 158 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_272d70() {
}

// 0x272fe4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)
// IDA 0x272fe4: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_272fe4() {
}

// 0x2749f0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(G3D::CoordinateFrame const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(G3D::CoordinateFrame const&,char const*,lua_State *)
// IDA 0x2749f0: 291 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2749f0() {
}

// 0x274da0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(G3D::CoordinateFrame&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(G3D::CoordinateFrame&,char const*,lua_State *)
// IDA 0x274da0: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_274da0() {
}

// 0x276858 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// IDA 0x276858: 160 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_276858() {
}

// 0x276a48 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// IDA 0x276a48: 160 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_276a48() {
}

// 0x276c38 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// IDA 0x276c38: 160 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_276c38() {
}

// 0x276e28 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// IDA 0x276e28: 160 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_276e28() {
}

// 0x277018 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// IDA 0x277018: 146 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277018() {
}

// 0x2774ac — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3)")]
// was: G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3)
// IDA 0x2774ac: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2774ac() {
}

// 0x2774f4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *)")]
// was: G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *)
// IDA 0x2774f4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2774f4() {
}

// 0x27759c — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3)")]
// was: G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3)
// IDA 0x27759c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_27759c() {
}

// 0x2775ec — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *)")]
// was: G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *)
// IDA 0x2775ec: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2775ec() {
}

// 0x277634 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *)")]
// was: G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *)
// IDA 0x277634: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277634() {
}

// 0x27767c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &)
// IDA 0x27767c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_27767c() {
}

// 0x2776ec — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *)")]
// was: G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *)
// IDA 0x2776ec: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2776ec() {
}

// 0x277730 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &)
// IDA 0x277730: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277730() {
}

// 0x2777a8 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *)")]
// was: G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *)
// IDA 0x2777a8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2777a8() {
}

// 0x2777ec — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2)")]
// was: G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2)
// IDA 0x2777ec: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2777ec() {
}

// 0x277894 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame)")]
// was: G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame)
// IDA 0x277894: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277894() {
}

// 0x2778e4 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &)")]
// was: bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &)
// IDA 0x2778e4: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2778e4() {
}

// 0x27797c — __ZN3G3D7Matrix313fromAxisAngleERKNS_7Vector3Ef
#[doc(alias = "G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&,float)")]
// was: G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&,float)
// IDA 0x27797c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_27797c() {
}

// 0x277c90 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_gc(lua_State *)
// IDA 0x277c90: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277c90() {
}

// 0x277cac — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_eq(lua_State *)
// IDA 0x277cac: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277cac() {
}

// 0x277cf8 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(lua_State *)
// IDA 0x277cf8: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277cf8() {
}

// 0x277d1c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_gc(lua_State *)
// IDA 0x277d1c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277d1c() {
}

// 0x277d38 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_eq(lua_State *)
// IDA 0x277d38: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277d38() {
}

// 0x277d74 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(lua_State *)
// IDA 0x277d74: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277d74() {
}

// 0x277d98 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_gc(lua_State *)
// IDA 0x277d98: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277d98() {
}

// 0x277db4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_eq(lua_State *)
// IDA 0x277db4: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277db4() {
}

// 0x277e20 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(lua_State *)
// IDA 0x277e20: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277e20() {
}

// 0x277e44 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_gc(lua_State *)
// IDA 0x277e44: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277e44() {
}

// 0x277e60 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_eq(lua_State *)
// IDA 0x277e60: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277e60() {
}

// 0x277eb8 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(lua_State *)
// IDA 0x277eb8: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277eb8() {
}

// 0x277edc — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_gc(lua_State *)
// IDA 0x277edc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277edc() {
}

// 0x277ef8 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_eq(lua_State *)
// IDA 0x277ef8: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277ef8() {
}

// 0x277f70 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(lua_State *)
// IDA 0x277f70: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_277f70() {
}

// 0x278084 — __ZNK3RBX6RbxRay8distanceERKN3G3D7Vector3E
#[doc(alias = "RBX::RbxRay::distance(G3D::Vector3 const&)const")]
// was: RBX::RbxRay::distance(G3D::Vector3 const&)const
// IDA 0x278084: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_278084() {
}

// 0x2780dc — __ZNK3RBX6RbxRay12closestPointERKN3G3D7Vector3E
#[doc(alias = "RBX::RbxRay::closestPoint(G3D::Vector3 const&)const")]
// was: RBX::RbxRay::closestPoint(G3D::Vector3 const&)const
// IDA 0x2780dc: 37 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2780dc() {
}

// 0x278574 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *)
// IDA 0x278574: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_278574() {
}

// 0x278698 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *)
// IDA 0x278698: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_278698() {
}

// 0x2787bc — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *)
// IDA 0x2787bc: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2787bc() {
}

// 0x2788e0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)
// IDA 0x2788e0: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2788e0() {
}

// 0x278b28 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)
// IDA 0x278b28: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_278b28() {
}

// 0x279e44 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
// IDA 0x279e44: 91 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_279e44() {
}

// 0x279f58 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)
// IDA 0x279f58: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_279f58() {
}

// 0x279f74 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)
// IDA 0x279f74: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_279f74() {
}

// 0x279fe4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)
// IDA 0x279fe4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_279fe4() {
}

// 0x2a3fc4 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(lua_State *)
// IDA 0x2a3fc4: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a3fc4() {
}

// 0x2a3ff8 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(lua_State *)
// IDA 0x2a3ff8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a3ff8() {
}

// 0x2a40fc — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(lua_State *)
// IDA 0x2a40fc: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a40fc() {
}

// 0x2a4130 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(lua_State *)
// IDA 0x2a4130: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4130() {
}

// 0x2a4164 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(lua_State *)
// IDA 0x2a4164: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4164() {
}

// 0x2a4198 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(lua_State *)
// IDA 0x2a4198: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4198() {
}

// 0x2a41cc — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_index(lua_State *)
// IDA 0x2a41cc: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a41cc() {
}

// 0x2a4200 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(lua_State *)
// IDA 0x2a4200: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4200() {
}

// 0x2a4234 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_index(lua_State *)
// IDA 0x2a4234: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4234() {
}

// 0x2a4268 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(lua_State *)
// IDA 0x2a4268: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4268() {
}

// 0x2a4304 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_index(lua_State *)
// IDA 0x2a4304: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4304() {
}

// 0x2a4338 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(lua_State *)
// IDA 0x2a4338: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4338() {
}
