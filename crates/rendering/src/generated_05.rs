//! rendering shard b — next 100 Gfx|G3D stubs (EA-sorted)
//! Filter: Gfx|G3D (4083 total, 50 prior stubbed, 100 this batch, 3933 remaining)
//! This shard: 0xb740..0x2a4338
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xb740 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")]
// was: std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)
pub fn stub_b740() -> ! {
    todo!("0xb740 std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::push_back(G3D::Vector2int16 const&)")
}

// 0xf704 — __ZNSt6vectorIN3G3D12Vector2int16ESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
#[doc(alias = "std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")]
// was: std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)
pub fn stub_f704() -> ! {
    todo!("0xf704 std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_insert_aux(__gnu_cxx::__normal_iterator<G3D::Vector2int16*,std::vector<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>>,G3D::Vector2int16 const&)")
}

// 0xf7e8 — __ZNSt12_Vector_baseIN3G3D12Vector2int16ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)
pub fn stub_f7e8() -> ! {
    todo!("0xf7e8 std::_Vector_base<G3D::Vector2int16,std::allocator<G3D::Vector2int16>>::_M_allocate(unsigned long)")
}

// 0xf800 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector2int16ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")]
// was: G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)
pub fn stub_f800() -> ! {
    todo!("0xf800 G3D::Vector2int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector2int16 *,G3D::Vector2int16 *>(G3D::Vector2int16 *,G3D::Vector2int16 *,G3D::Vector2int16 *)")
}

// 0x25b4e0 — __ZN3RBX5Light8setColorEN3G3D6Color3E
#[doc(alias = "RBX::Light::setColor(G3D::Color3)")]
// was: RBX::Light::setColor(G3D::Color3)
pub fn stub_25b4e0() -> ! {
    todo!("0x25b4e0 RBX::Light::setColor(G3D::Color3)")
}

// 0x25c100 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()
pub fn stub_25c100() -> ! {
    todo!("0x25c100 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")
}

// 0x25ed10 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
pub fn stub_25ed10() -> ! {
    todo!("0x25ed10 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::PropDescriptor<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>(char const*,char const*,G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x25ee24 — __ZN3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()
pub fn stub_25ee24() -> ! {
    todo!("0x25ee24 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::~PropDescriptor()")
}

// 0x25ee50 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isReadOnly(void)const
pub fn stub_25ee50() -> ! {
    todo!("0x25ee50 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isReadOnly(void)const")
}

// 0x25ee54 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isWriteOnly(void)const
pub fn stub_25ee54() -> ! {
    todo!("0x25ee54 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::isWriteOnly(void)const")
}

// 0x25ee58 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const
pub fn stub_25ee58() -> ! {
    todo!("0x25ee58 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x25ee80 — __ZNK3RBX10Reflection14PropDescriptorINS_5LightEN3G3D6Color3EE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const
pub fn stub_25ee80() -> ! {
    todo!("0x25ee80 RBX::Reflection::PropDescriptor<RBX::Light,G3D::Color3>::GetSetImpl<G3D::Color3 (RBX::Light::*)(void)const,void (RBX::Light::*)(G3D::Color3)>::setValue(RBX::Reflection::DescribedBase *,G3D::Color3 const&)const")
}

// 0x26b4ac — __ZNK3RBX3Lua12LuaArguments15getVector3int16EiRN3G3D12Vector3int16E
#[doc(alias = "RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const")]
// was: RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const
pub fn stub_26b4ac() -> ! {
    todo!("0x26b4ac RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const")
}

// 0x26b504 — __ZNK3RBX3Lua12LuaArguments10getVector3EiRN3G3D7Vector3E
#[doc(alias = "RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const")]
// was: RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const
pub fn stub_26b504() -> ! {
    todo!("0x26b504 RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const")
}

// 0x26c140 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)
pub fn stub_26c140() -> ! {
    todo!("0x26c140 bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)")
}

// 0x26c230 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)
pub fn stub_26c230() -> ! {
    todo!("0x26c230 bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)")
}

// 0x26c92c — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
pub fn stub_26c92c() -> ! {
    todo!("0x26c92c bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26caa0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
pub fn stub_26caa0() -> ! {
    todo!("0x26caa0 bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26cb1c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
pub fn stub_26cb1c() -> ! {
    todo!("0x26cb1c bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26cb98 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
pub fn stub_26cb98() -> ! {
    todo!("0x26cb98 bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26cc14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
pub fn stub_26cc14() -> ! {
    todo!("0x26cc14 bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26cd0c — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8getValueINS_10Reflection7VariantEEEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")]
// was: bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)
pub fn stub_26cd0c() -> ! {
    todo!("0x26cd0c bool RBX::Lua::Bridge<G3D::Color3,true>::getValue<RBX::Reflection::Variant>(lua_State *,unsigned int,RBX::Reflection::Variant &)")
}

// 0x26e8d0 — __ZN3rbx8any_castIRKN3G3D12Vector2int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_26e8d0() -> ! {
    todo!("0x26e8d0 G3D::Vector2int16 const& rbx::any_cast<G3D::Vector2int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26e9c0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")]
// was: G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)
pub fn stub_26e9c0() -> ! {
    todo!("0x26e9c0 G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")
}

// 0x26ea00 — __ZN3rbx8any_castIRKN3G3D12Vector3int16EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_26ea00() -> ! {
    todo!("0x26ea00 G3D::Vector3int16 const& rbx::any_cast<G3D::Vector3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0x26eaf0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")]
// was: G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)
pub fn stub_26eaf0() -> ! {
    todo!("0x26eaf0 G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")
}

// 0x26f7b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector2EEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)
pub fn stub_26f7b0() -> ! {
    todo!("0x26f7b0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector2>(G3D::Vector2 const&)")
}

// 0x26f808 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)
pub fn stub_26f808() -> ! {
    todo!("0x26f808 rbx::implementation::typed_holder<G3D::Vector2>::singleton(void)")
}

// 0x26f878 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D7Vector3EEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)
pub fn stub_26f878() -> ! {
    todo!("0x26f878 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3>(G3D::Vector3 const&)")
}

// 0x26f8d8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN3G3D12Vector3int16EEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)
pub fn stub_26f8d8() -> ! {
    todo!("0x26f8d8 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<G3D::Vector3int16>(G3D::Vector3int16 const&)")
}

// 0x26f930 — __ZN3rbx14implementation12typed_holderIN3G3D12Vector3int16EE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)
pub fn stub_26f930() -> ! {
    todo!("0x26f930 rbx::implementation::typed_holder<G3D::Vector3int16>::singleton(void)")
}

// 0x2705d0 — __ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E
#[doc(alias = "RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")]
// was: RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)
pub fn stub_2705d0() -> ! {
    todo!("0x2705d0 RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")
}

// 0x2705ec — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)
pub fn stub_2705ec() -> ! {
    todo!("0x2705ec RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")
}

// 0x270724 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)
pub fn stub_270724() -> ! {
    todo!("0x270724 RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")
}

// 0x271954 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)
pub fn stub_271954() -> ! {
    todo!("0x271954 RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")
}

// 0x271e14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)
pub fn stub_271e14() -> ! {
    todo!("0x271e14 RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")
}

// 0x272268 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)
pub fn stub_272268() -> ! {
    todo!("0x272268 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")
}

// 0x2723d0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)
pub fn stub_2723d0() -> ! {
    todo!("0x2723d0 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")
}

// 0x272804 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)
pub fn stub_272804() -> ! {
    todo!("0x272804 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")
}

// 0x272940 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)
pub fn stub_272940() -> ! {
    todo!("0x272940 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(G3D::Vector2int16&,char const*,lua_State *)")
}

// 0x272d70 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)
pub fn stub_272d70() -> ! {
    todo!("0x272d70 RBX::Lua::Bridge<G3D::Vector2,true>::on_index(G3D::Vector2 const&,char const*,lua_State *)")
}

// 0x272fe4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)
pub fn stub_272fe4() -> ! {
    todo!("0x272fe4 RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(G3D::Vector2&,char const*,lua_State *)")
}

// 0x2749f0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexERKS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(G3D::CoordinateFrame const&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(G3D::CoordinateFrame const&,char const*,lua_State *)
pub fn stub_2749f0() -> ! {
    todo!("0x2749f0 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(G3D::CoordinateFrame const&,char const*,lua_State *)")
}

// 0x274da0 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexERS3_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(G3D::CoordinateFrame&,char const*,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(G3D::CoordinateFrame&,char const*,lua_State *)
pub fn stub_274da0() -> ! {
    todo!("0x274da0 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(G3D::CoordinateFrame&,char const*,lua_State *)")
}

// 0x276858 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub fn stub_276858() -> ! {
    todo!("0x276858 RBX::Lua::Bridge<G3D::Vector3int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")
}

// 0x276a48 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub fn stub_276a48() -> ! {
    todo!("0x276a48 RBX::Lua::Bridge<G3D::Vector2int16,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")
}

// 0x276c38 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub fn stub_276c38() -> ! {
    todo!("0x276c38 RBX::Lua::Bridge<G3D::Vector3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")
}

// 0x276e28 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub fn stub_276e28() -> ! {
    todo!("0x276e28 RBX::Lua::Bridge<G3D::Vector2,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")
}

// 0x277018 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub fn stub_277018() -> ! {
    todo!("0x277018 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")
}

// 0x2774ac — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3)")]
// was: G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3)
pub fn stub_2774ac() -> ! {
    todo!("0x2774ac G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<G3D::Color3>(lua_State *,G3D::Color3)")
}

// 0x2774f4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *)")]
// was: G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *)
pub fn stub_2774f4() -> ! {
    todo!("0x2774f4 G3D::Color3* RBX::Lua::Bridge<G3D::Color3,true>::pushNewObject<float *>(lua_State *,float *)")
}

// 0x27759c — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3)")]
// was: G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3)
pub fn stub_27759c() -> ! {
    todo!("0x27759c G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<G3D::Vector3>(lua_State *,G3D::Vector3)")
}

// 0x2775ec — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *)")]
// was: G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *)
pub fn stub_2775ec() -> ! {
    todo!("0x2775ec G3D::Vector3* RBX::Lua::Bridge<G3D::Vector3,true>::pushNewObject<float *>(lua_State *,float *)")
}

// 0x277634 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *)")]
// was: G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *)
pub fn stub_277634() -> ! {
    todo!("0x277634 G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<int *>(lua_State *,int *)")
}

// 0x27767c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &)
pub fn stub_27767c() -> ! {
    todo!("0x27767c bool RBX::Lua::Bridge<G3D::Vector2int16,true>::getValue<G3D::Vector2int16>(lua_State *,unsigned int,G3D::Vector2int16 &)")
}

// 0x2776ec — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIPiEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *)")]
// was: G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *)
pub fn stub_2776ec() -> ! {
    todo!("0x2776ec G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<int *>(lua_State *,int *)")
}

// 0x277730 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &)")]
// was: bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &)
pub fn stub_277730() -> ! {
    todo!("0x277730 bool RBX::Lua::Bridge<G3D::Vector2,true>::getValue<G3D::Vector2>(lua_State *,unsigned int,G3D::Vector2 &)")
}

// 0x2777a8 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIPfEEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *)")]
// was: G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *)
pub fn stub_2777a8() -> ! {
    todo!("0x2777a8 G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<float *>(lua_State *,float *)")
}

// 0x2777ec — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2)")]
// was: G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2)
pub fn stub_2777ec() -> ! {
    todo!("0x2777ec G3D::Vector2* RBX::Lua::Bridge<G3D::Vector2,true>::pushNewObject<G3D::Vector2>(lua_State *,G3D::Vector2)")
}

// 0x277894 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
#[doc(alias = "G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame)")]
// was: G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame)
pub fn stub_277894() -> ! {
    todo!("0x277894 G3D::CoordinateFrame* RBX::Lua::Bridge<G3D::CoordinateFrame,true>::pushNewObject<G3D::CoordinateFrame>(lua_State *,G3D::CoordinateFrame)")
}

// 0x2778e4 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8getValueIS3_EEbP9lua_StatejRT_
#[doc(alias = "bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &)")]
// was: bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &)
pub fn stub_2778e4() -> ! {
    todo!("0x2778e4 bool RBX::Lua::Bridge<G3D::CoordinateFrame,true>::getValue<G3D::CoordinateFrame>(lua_State *,unsigned int,G3D::CoordinateFrame &)")
}

// 0x27797c — __ZN3G3D7Matrix313fromAxisAngleERKNS_7Vector3Ef
#[doc(alias = "G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&,float)")]
// was: G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&,float)
pub fn stub_27797c() -> ! {
    todo!("0x27797c G3D::Matrix3::fromAxisAngle(G3D::Vector3 const&,float)")
}

// 0x277c90 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_gc(lua_State *)
pub fn stub_277c90() -> ! {
    todo!("0x277c90 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_gc(lua_State *)")
}

// 0x277cac — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_eq(lua_State *)
pub fn stub_277cac() -> ! {
    todo!("0x277cac RBX::Lua::Bridge<G3D::Vector3int16,true>::on_eq(lua_State *)")
}

// 0x277cf8 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(lua_State *)
pub fn stub_277cf8() -> ! {
    todo!("0x277cf8 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(lua_State *)")
}

// 0x277d1c — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_gc(lua_State *)
pub fn stub_277d1c() -> ! {
    todo!("0x277d1c RBX::Lua::Bridge<G3D::Vector2int16,true>::on_gc(lua_State *)")
}

// 0x277d38 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_eq(lua_State *)
pub fn stub_277d38() -> ! {
    todo!("0x277d38 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_eq(lua_State *)")
}

// 0x277d74 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(lua_State *)
pub fn stub_277d74() -> ! {
    todo!("0x277d74 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(lua_State *)")
}

// 0x277d98 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_gc(lua_State *)
pub fn stub_277d98() -> ! {
    todo!("0x277d98 RBX::Lua::Bridge<G3D::Vector3,true>::on_gc(lua_State *)")
}

// 0x277db4 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_eq(lua_State *)
pub fn stub_277db4() -> ! {
    todo!("0x277db4 RBX::Lua::Bridge<G3D::Vector3,true>::on_eq(lua_State *)")
}

// 0x277e20 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(lua_State *)
pub fn stub_277e20() -> ! {
    todo!("0x277e20 RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(lua_State *)")
}

// 0x277e44 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_gc(lua_State *)
pub fn stub_277e44() -> ! {
    todo!("0x277e44 RBX::Lua::Bridge<G3D::Vector2,true>::on_gc(lua_State *)")
}

// 0x277e60 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_eq(lua_State *)
pub fn stub_277e60() -> ! {
    todo!("0x277e60 RBX::Lua::Bridge<G3D::Vector2,true>::on_eq(lua_State *)")
}

// 0x277eb8 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(lua_State *)
pub fn stub_277eb8() -> ! {
    todo!("0x277eb8 RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(lua_State *)")
}

// 0x277edc — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_gc(lua_State *)
pub fn stub_277edc() -> ! {
    todo!("0x277edc RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_gc(lua_State *)")
}

// 0x277ef8 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_eq(lua_State *)
pub fn stub_277ef8() -> ! {
    todo!("0x277ef8 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_eq(lua_State *)")
}

// 0x277f70 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(lua_State *)
pub fn stub_277f70() -> ! {
    todo!("0x277f70 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(lua_State *)")
}

// 0x278084 — __ZNK3RBX6RbxRay8distanceERKN3G3D7Vector3E
#[doc(alias = "RBX::RbxRay::distance(G3D::Vector3 const&)const")]
// was: RBX::RbxRay::distance(G3D::Vector3 const&)const
pub fn stub_278084() -> ! {
    todo!("0x278084 RBX::RbxRay::distance(G3D::Vector3 const&)const")
}

// 0x2780dc — __ZNK3RBX6RbxRay12closestPointERKN3G3D7Vector3E
#[doc(alias = "RBX::RbxRay::closestPoint(G3D::Vector3 const&)const")]
// was: RBX::RbxRay::closestPoint(G3D::Vector3 const&)const
pub fn stub_2780dc() -> ! {
    todo!("0x2780dc RBX::RbxRay::closestPoint(G3D::Vector3 const&)const")
}

// 0x278574 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *)
pub fn stub_278574() -> ! {
    todo!("0x278574 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_tostring(G3D::Vector3int16 const&,lua_State *)")
}

// 0x278698 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *)
pub fn stub_278698() -> ! {
    todo!("0x278698 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_tostring(G3D::Vector2int16 const&,lua_State *)")
}

// 0x2787bc — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *)
pub fn stub_2787bc() -> ! {
    todo!("0x2787bc RBX::Lua::Bridge<G3D::Vector3,true>::on_tostring(G3D::Vector3 const&,lua_State *)")
}

// 0x2788e0 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)
pub fn stub_2788e0() -> ! {
    todo!("0x2788e0 RBX::Lua::Bridge<G3D::Vector2,true>::on_tostring(G3D::Vector2 const&,lua_State *)")
}

// 0x278b28 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_tostringERKS3_P9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)
pub fn stub_278b28() -> ! {
    todo!("0x278b28 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_tostring(G3D::CoordinateFrame const&,lua_State *)")
}

// 0x279e44 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE13registerClassEP9lua_StatePFiS6_ES8_
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))
pub fn stub_279e44() -> ! {
    todo!("0x279e44 RBX::Lua::Bridge<G3D::Color3,true>::registerClass(lua_State *,int (*)(lua_State *),int (*)(lua_State *))")
}

// 0x279f58 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_gcEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)
pub fn stub_279f58() -> ! {
    todo!("0x279f58 RBX::Lua::Bridge<G3D::Color3,true>::on_gc(lua_State *)")
}

// 0x279f74 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE5on_eqEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)
pub fn stub_279f74() -> ! {
    todo!("0x279f74 RBX::Lua::Bridge<G3D::Color3,true>::on_eq(lua_State *)")
}

// 0x279fe4 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_tostringEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)
pub fn stub_279fe4() -> ! {
    todo!("0x279fe4 RBX::Lua::Bridge<G3D::Color3,true>::on_tostring(lua_State *)")
}

// 0x2a3fc4 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(lua_State *)
pub fn stub_2a3fc4() -> ! {
    todo!("0x2a3fc4 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(lua_State *)")
}

// 0x2a3ff8 — __ZN3RBX3Lua6BridgeIN3G3D15CoordinateFrameELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(lua_State *)
pub fn stub_2a3ff8() -> ! {
    todo!("0x2a3ff8 RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(lua_State *)")
}

// 0x2a40fc — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(lua_State *)
pub fn stub_2a40fc() -> ! {
    todo!("0x2a40fc RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(lua_State *)")
}

// 0x2a4130 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(lua_State *)
pub fn stub_2a4130() -> ! {
    todo!("0x2a4130 RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(lua_State *)")
}

// 0x2a4164 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(lua_State *)
pub fn stub_2a4164() -> ! {
    todo!("0x2a4164 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(lua_State *)")
}

// 0x2a4198 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(lua_State *)
pub fn stub_2a4198() -> ! {
    todo!("0x2a4198 RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(lua_State *)")
}

// 0x2a41cc — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_index(lua_State *)
pub fn stub_2a41cc() -> ! {
    todo!("0x2a41cc RBX::Lua::Bridge<G3D::Vector3,true>::on_index(lua_State *)")
}

// 0x2a4200 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(lua_State *)
pub fn stub_2a4200() -> ! {
    todo!("0x2a4200 RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(lua_State *)")
}

// 0x2a4234 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_index(lua_State *)
pub fn stub_2a4234() -> ! {
    todo!("0x2a4234 RBX::Lua::Bridge<G3D::Vector2,true>::on_index(lua_State *)")
}

// 0x2a4268 — __ZN3RBX3Lua6BridgeIN3G3D7Vector2ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(lua_State *)
pub fn stub_2a4268() -> ! {
    todo!("0x2a4268 RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(lua_State *)")
}

// 0x2a4304 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_index(lua_State *)
pub fn stub_2a4304() -> ! {
    todo!("0x2a4304 RBX::Lua::Bridge<G3D::Color3,true>::on_index(lua_State *)")
}

// 0x2a4338 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(lua_State *)")]
// was: RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(lua_State *)
pub fn stub_2a4338() -> ! {
    todo!("0x2a4338 RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(lua_State *)")
}
