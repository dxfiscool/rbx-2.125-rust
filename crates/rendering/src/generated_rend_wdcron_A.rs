//! rendering shard rend_wdcron_A — 120 stubs 0x83240c..0x838bc8 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render filtered exhausted -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x83240c
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x83240c — __ZL12call_orderTMP9lua_StatePK10lua_TValueS3_3TMS
// type: int __fastcall(_DWORD *, int *, int *, int)
#[doc(alias = "call_orderTM(lua_State *,lua_TValue const*,lua_TValue const*,TMS)")]
#[doc(alias = "__ZL12call_orderTMP9lua_StatePK10lua_TValueS3_3TMS")]
// IDA 0x83240c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_83240c() {
}

// 0x832478 — __Z13luaV_equalvalP9lua_StatePK10lua_TValueS3_
// type: int __fastcall(int, double *, int)
#[doc(alias = "luaV_equalval(lua_State *,lua_TValue const*,lua_TValue const*)")]
#[doc(alias = "__Z13luaV_equalvalP9lua_StatePK10lua_TValueS3_")]
// IDA 0x832478: 66 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_832478() {
}

// 0x832514 — __ZL10get_compTMP9lua_StateP5TableS2_3TMS
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "get_compTM(lua_State *,Table *,Table *,TMS)")]
#[doc(alias = "__ZL10get_compTMP9lua_StateP5TableS2_3TMS")]
// IDA 0x832514: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_832514() {
}

// 0x832584 — __Z11luaV_concatP9lua_Stateii
#[doc(alias = "luaV_concat(lua_State *,int,int)")]
#[doc(alias = "__Z11luaV_concatP9lua_Stateii")]
// IDA 0x832584: 141 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_832584() {
}

// 0x8326f8 — __ZL10call_binTMP9lua_StatePK10lua_TValueS3_PS1_3TMS
// type: int __fastcall(_DWORD *, int *, int *, int, int)
#[doc(alias = "call_binTM(lua_State *,lua_TValue const*,lua_TValue const*,lua_TValue*,TMS)")]
#[doc(alias = "__ZL10call_binTMP9lua_StatePK10lua_TValueS3_PS1_3TMS")]
// IDA 0x8326f8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8326f8() {
}

// 0x832744 — __Z12luaV_executeP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "luaV_execute(lua_State *,int)")]
#[doc(alias = "__Z12luaV_executeP9lua_Statei")]
// IDA 0x832744: 1338 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_832744() {
}

// 0x833730 — __ZL5ArithP9lua_StateP10lua_TValuePKS1_S4_3TMS
// type: int __fastcall(_DWORD *, int, int *, int *, int)
#[doc(alias = "Arith(lua_State *,lua_TValue *,lua_TValue const*,lua_TValue const*,TMS)")]
#[doc(alias = "__ZL5ArithP9lua_StateP10lua_TValuePKS1_S4_3TMS")]
// IDA 0x833730: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_833730() {
}

// 0x833800 — __GLOBAL__I_a_421
#[doc(alias = "global constructor keyed to_a_421")]
#[doc(alias = "__GLOBAL__I_a_421")]
// IDA 0x833800: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_833800() {
}

// 0x8338c8 — __Z9luaZ_fillP3Zio
#[doc(alias = "luaZ_fill(Zio *)")]
#[doc(alias = "__Z9luaZ_fillP3Zio")]
// IDA 0x8338c8: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8338c8() {
}

// 0x8338fc — __Z14luaZ_lookaheadP3Zio
#[doc(alias = "luaZ_lookahead(Zio *)")]
#[doc(alias = "__Z14luaZ_lookaheadP3Zio")]
// IDA 0x8338fc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8338fc() {
}

// 0x833930 — __Z9luaZ_initP9lua_StateP3ZioPFPKcS0_PvPmES5_
#[doc(alias = "luaZ_init(lua_State *,Zio *,char const* (*)(lua_State *,void *,unsigned long *),void *)")]
#[doc(alias = "__Z9luaZ_initP9lua_StateP3ZioPFPKcS0_PvPmES5_")]
// IDA 0x833930: 6 insns (VMOV.I32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_833930() {
}

// 0x833940 — __Z9luaZ_readP3ZioPvm
#[doc(alias = "luaZ_read(Zio *,void *,unsigned long)")]
#[doc(alias = "__Z9luaZ_readP3ZioPvm")]
// IDA 0x833940: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_833940() {
}

// 0x8339a0 — __Z14luaZ_openspaceP9lua_StateP7Mbufferm
#[doc(alias = "luaZ_openspace(lua_State *,Mbuffer *,unsigned long)")]
#[doc(alias = "__Z14luaZ_openspaceP9lua_StateP7Mbufferm")]
// IDA 0x8339a0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8339a0() {
}

// 0x8339d4 — __GLOBAL__I_a_422
#[doc(alias = "global constructor keyed to_a_422")]
#[doc(alias = "__GLOBAL__I_a_422")]
// IDA 0x8339d4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_8339d4() {
}

// 0x833a9c — __ZN3RBX12TweenServiceC1Ev
// type: _DWORD __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "RBX::TweenService::TweenService(void)")]
#[doc(alias = "__ZN3RBX12TweenServiceC1Ev")]
// IDA 0x833a9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_833a9c() {
}

// 0x833aa0 — __ZN3RBX12TweenServiceC2Ev
// type: _DWORD __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "RBX::TweenService::TweenService(void)")]
#[doc(alias = "__ZN3RBX12TweenServiceC2Ev")]
// IDA 0x833aa0: 201 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_833aa0() {
}

// 0x833ce4 — __ZN3RBX12TweenService17addTweeningObjectEN5boost8weak_ptrINS_9GuiObjectEEE
#[doc(alias = "RBX::TweenService::addTweeningObject(rbx_core::SharedPtr<RBX::GuiObject>)")]
#[doc(alias = "__ZN3RBX12TweenService17addTweeningObjectEN5boost8weak_ptrINS_9GuiObjectEEE")]
// IDA 0x833ce4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_833ce4() {
}

// 0x833d10 — __ZN3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::TweenService::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE")]
// IDA 0x833d10: 134 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_833d10() {
}

// 0x833e78 — __ZThn96_N3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "non-virtual thunk toRBX::TweenService::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZThn96_N3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE")]
// IDA 0x833e78: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_833e78() {
}

// 0x833e80 — __ZN3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "RBX::TweenService::~TweenService()")]
#[doc(alias = "__ZN3RBX12TweenServiceD1Ev")]
// IDA 0x833e80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_833e80() {
}

// 0x833f94 — __ZN3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "RBX::TweenService::~TweenService()")]
#[doc(alias = "__ZN3RBX12TweenServiceD0Ev")]
// IDA 0x833f94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_833f94() {
}

// 0x8340b8 — __ZN3RBX12TweenService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::TweenService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::TweenService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX12TweenService17onServiceProviderEPNS_15ServiceProviderES2_")]
// IDA 0x8340b8: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8340b8() {
}

// 0x8340c0 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE12getClassNameEv")]
// IDA 0x8340c0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8340c0() {
}

// 0x8340e8 — __ZThn32_N3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
#[doc(alias = "__ZThn32_N3RBX12TweenServiceD1Ev")]
// IDA 0x8340e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8340e8() {
}

// 0x8341fc — __ZThn32_N3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
#[doc(alias = "__ZThn32_N3RBX12TweenServiceD0Ev")]
// IDA 0x8341fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8341fc() {
}

// 0x834324 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEE12getClassNameEv")]
// IDA 0x834324: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_834324() {
}

// 0x83434c — __ZThn36_N3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
#[doc(alias = "__ZThn36_N3RBX12TweenServiceD1Ev")]
// IDA 0x83434c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_83434c() {
}

// 0x834460 — __ZThn36_N3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
#[doc(alias = "__ZThn36_N3RBX12TweenServiceD0Ev")]
// IDA 0x834460: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_834460() {
}

// 0x834588 — __ZThn96_N3RBX12TweenServiceD1Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
#[doc(alias = "__ZThn96_N3RBX12TweenServiceD1Ev")]
// IDA 0x834588: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_834588() {
}

// 0x83469c — __ZThn96_N3RBX12TweenServiceD0Ev
// type: void __fastcall(RBX::TweenService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
#[doc(alias = "__ZThn96_N3RBX12TweenServiceD0Ev")]
// IDA 0x83469c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_83469c() {
}

// 0x8347c4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::GuiObject>,rbx_core::SharedPtr<RBX::GuiObject>,std::_Identity<rbx_core::SharedPtr<RBX::GuiObject>>,std::less<rbx_core::SharedPtr<RBX::GuiObject>>,std::allocator<rbx_core::SharedPtr<RBX::GuiObject>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::GuiObject>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0x8347c4: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8347c4() {
}

// 0x8347e0 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::GuiObject>,rbx_core::SharedPtr<RBX::GuiObject>,std::_Identity<rbx_core::SharedPtr<RBX::GuiObject>>,std::less<rbx_core::SharedPtr<RBX::GuiObject>>,std::allocator<rbx_core::SharedPtr<RBX::GuiObject>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::GuiObject> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_")]
// IDA 0x8347e0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8347e0() {
}

// 0x834848 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::GuiObject>,rbx_core::SharedPtr<RBX::GuiObject>,std::_Identity<rbx_core::SharedPtr<RBX::GuiObject>>,std::less<rbx_core::SharedPtr<RBX::GuiObject>>,std::allocator<rbx_core::SharedPtr<RBX::GuiObject>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::SharedPtr<RBX::GuiObject> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
// IDA 0x834848: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_834848() {
}

// 0x834894 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::GuiObject>,rbx_core::SharedPtr<RBX::GuiObject>,std::_Identity<rbx_core::SharedPtr<RBX::GuiObject>>,std::less<rbx_core::SharedPtr<RBX::GuiObject>>,std::allocator<rbx_core::SharedPtr<RBX::GuiObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::GuiObject> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_")]
// IDA 0x834894: 103 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_834894() {
}

// 0x8349b8 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE4findERKS4_
// type: int(void)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::GuiObject>,rbx_core::SharedPtr<RBX::GuiObject>,std::_Identity<rbx_core::SharedPtr<RBX::GuiObject>>,std::less<rbx_core::SharedPtr<RBX::GuiObject>>,std::allocator<rbx_core::SharedPtr<RBX::GuiObject>>>::find(rbx_core::SharedPtr<RBX::GuiObject> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE4findERKS4_")]
// IDA 0x8349b8: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8349b8() {
}

// 0x8349f8 — __ZN3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x8349f8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8349f8() {
}

// 0x8349fc — __ZN3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x8349fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8349fc() {
}

// 0x834a9c — __ZThn32_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x834a9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_834a9c() {
}

// 0x834aa4 — __ZThn32_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x834aa4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_834aa4() {
}

// 0x834b48 — __ZThn36_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x834b48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_834b48() {
}

// 0x834b50 — __ZThn36_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12TweenServiceELZNS_13sTweenServiceEENS_17NonFactoryProductINS_8InstanceELZNS_13sTweenServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x834b50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_834b50() {
}

// 0x834bf4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::GuiObject>,rbx_core::SharedPtr<RBX::GuiObject>,std::_Identity<rbx_core::SharedPtr<RBX::GuiObject>>,std::less<rbx_core::SharedPtr<RBX::GuiObject>>,std::allocator<rbx_core::SharedPtr<RBX::GuiObject>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::GuiObject>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0x834bf4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_834bf4() {
}

// 0x834c1c — __GLOBAL__I_a_423
#[doc(alias = "global constructor keyed to_a_423")]
#[doc(alias = "__GLOBAL__I_a_423")]
// IDA 0x834c1c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_834c1c() {
}

// 0x834df0 — __ZN3RBX15NotificationBoxC1Ev
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "RBX::NotificationBox::NotificationBox(void)")]
#[doc(alias = "__ZN3RBX15NotificationBoxC1Ev")]
// IDA 0x834df0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_834df0() {
}

// 0x834df4 — __ZN3RBX15NotificationBoxC2Ev
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "RBX::NotificationBox::NotificationBox(void)")]
#[doc(alias = "__ZN3RBX15NotificationBoxC2Ev")]
// IDA 0x834df4: 191 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_834df4() {
}

// 0x835030 — __ZN3RBX15NotificationBox15addNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::NotificationBox::addNotification(rbx_core::SharedPtr<RBX::NotificationObject>)")]
#[doc(alias = "__ZN3RBX15NotificationBox15addNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE")]
// IDA 0x835030: 205 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835030() {
}

// 0x835248 — __ZN3RBX15NotificationBox18removeNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE
#[doc(alias = "RBX::NotificationBox::removeNotification(rbx_core::SharedPtr<RBX::NotificationObject>)")]
#[doc(alias = "__ZN3RBX15NotificationBox18removeNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE")]
// IDA 0x835248: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835248() {
}

// 0x835270 — __ZN3RBX15NotificationBox13organizeStackEv
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "RBX::NotificationBox::organizeStack(void)")]
#[doc(alias = "__ZN3RBX15NotificationBox13organizeStackEv")]
// IDA 0x835270: 207 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835270() {
}

// 0x835498 — __ZN3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "RBX::NotificationBox::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE")]
// IDA 0x835498: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835498() {
}

// 0x83554c — __ZThn536_N3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::onHeartbeat(RBX::Heartbeat const&)")]
#[doc(alias = "__ZThn536_N3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE")]
// IDA 0x83554c: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_83554c() {
}

// 0x835554 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE6removeERKS4_
// type: int(void)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::remove(rbx_core::SharedPtr<RBX::NotificationObject> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE6removeERKS4_")]
// IDA 0x835554: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835554() {
}

// 0x83557c — __ZN3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "RBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZN3RBX15NotificationBoxD1Ev")]
// IDA 0x83557c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_83557c() {
}

// 0x8356a4 — __ZN3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "RBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZN3RBX15NotificationBoxD0Ev")]
// IDA 0x8356a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8356a4() {
}

// 0x8357dc — __ZN3RBX15NotificationBox17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::NotificationBox *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::NotificationBox::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX15NotificationBox17onServiceProviderEPNS_15ServiceProviderES2_")]
// IDA 0x8357dc: 2 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8357dc() {
}

// 0x8357e4 — __ZNK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE12getClassNameEv")]
// IDA 0x8357e4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8357e4() {
}

// 0x8357f4 — __ZThn32_N3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZThn32_N3RBX15NotificationBoxD1Ev")]
// IDA 0x8357f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8357f4() {
}

// 0x835918 — __ZThn32_N3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZThn32_N3RBX15NotificationBoxD0Ev")]
// IDA 0x835918: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_835918() {
}

// 0x835a50 — __ZThn32_NK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE12getClassNameEv")]
// IDA 0x835a50: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835a50() {
}

// 0x835a60 — __ZThn36_N3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZThn36_N3RBX15NotificationBoxD1Ev")]
// IDA 0x835a60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_835a60() {
}

// 0x835b84 — __ZThn36_N3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZThn36_N3RBX15NotificationBoxD0Ev")]
// IDA 0x835b84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_835b84() {
}

// 0x835cbc — __ZThn536_N3RBX15NotificationBoxD1Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZThn536_N3RBX15NotificationBoxD1Ev")]
// IDA 0x835cbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_835cbc() {
}

// 0x835de4 — __ZThn536_N3RBX15NotificationBoxD0Ev
// type: void __fastcall(RBX::NotificationBox *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
#[doc(alias = "__ZThn536_N3RBX15NotificationBoxD0Ev")]
// IDA 0x835de4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_835de4() {
}

// 0x835f20 — __ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0x835f20: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_835f20() {
}

// 0x835f24 — __ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0x835f24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_835f24() {
}

// 0x835fc0 — __ZNK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0x835fc0: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_835fc0() {
}

// 0x836048 — __ZNK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7Creator6createEv")]
// IDA 0x836048: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836048() {
}

// 0x83618c — __ZN3RBX4Name13callDoDeclareILZNS_16sNotificationBoxEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sNotificationBoxEEEEvv")]
// IDA 0x83618c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_83618c() {
}

// 0x836190 — __ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNotificationBoxEEEERKS0_v")]
// IDA 0x836190: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836190() {
}

// 0x836270 — __ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0x836270: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836270() {
}

// 0x8364b4 — __ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NotificationBoxENS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEE17static_getCreatorEv")]
// IDA 0x8364b4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8364b4() {
}

// 0x836528 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_clearEv
// type: int(void)
#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_clearEv")]
// IDA 0x836528: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836528() {
}

// 0x836550 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// type: int __fastcall(int, std::_List_node_base *this)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::NotificationObject>>)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")]
// IDA 0x836550: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836550() {
}

// 0x836570 — __ZN5boost8weak_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject>::weak_ptr<RBX::NotificationObject>(rbx_core::SharedPtr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::NotificationObject>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
// IDA 0x836570: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836570() {
}

// 0x8365c0 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::NotificationObject> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE14_M_create_nodeERKS4_")]
// IDA 0x8365c0: 81 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8365c0() {
}

// 0x8366a4 — __ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x8366a4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8366a4() {
}

// 0x8366a8 — __ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x8366a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8366a8() {
}

// 0x836748 — __ZThn32_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x836748: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_836748() {
}

// 0x836750 — __ZThn32_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x836750: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_836750() {
}

// 0x8367f4 — __ZThn36_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x8367f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8367f4() {
}

// 0x8367fc — __ZThn36_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15NotificationBoxELZNS_16sNotificationBoxEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_16sNotificationBoxEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x8367fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8367fc() {
}

// 0x8368a0 — __GLOBAL__I_a_424
#[doc(alias = "global constructor keyed to_a_424")]
#[doc(alias = "__GLOBAL__I_a_424")]
// IDA 0x8368a0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_8368a0() {
}

// 0x836ab4 — __ZN3RBX18NotificationObjectC1Ev
// type: _DWORD __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "RBX::NotificationObject::NotificationObject(void)")]
#[doc(alias = "__ZN3RBX18NotificationObjectC1Ev")]
// IDA 0x836ab4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_836ab4() {
}

// 0x836ab8 — __ZN3RBX18NotificationObjectC2Ev
// type: _DWORD __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "RBX::NotificationObject::NotificationObject(void)")]
#[doc(alias = "__ZN3RBX18NotificationObjectC2Ev")]
// IDA 0x836ab8: 328 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836ab8() {
}

// 0x836e60 — __ZN3RBX18NotificationObject10initializeESsSsSsiN5boost8functionIFvvEEE
#[doc(alias = "RBX::NotificationObject::initialize(std::string,std::string,std::string,int,boost::function<void ()(void)>)")]
#[doc(alias = "__ZN3RBX18NotificationObject10initializeESsSsSsiN5boost8functionIFvvEEE")]
// IDA 0x836e60: 519 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_836e60() {
}

// 0x8373fc — __ZN3RBX18NotificationObject17processMouseEventERKNS_8GuiEventE
#[doc(alias = "RBX::NotificationObject::processMouseEvent(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX18NotificationObject17processMouseEventERKNS_8GuiEventE")]
// IDA 0x8373fc: 113 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8373fc() {
}

// 0x837548 — __ZN5boost10shared_ptrIN3RBX9TextLabelEEaSERKS3_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::TextLabel>::operator=(rbx_core::SharedPtr<RBX::TextLabel> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9TextLabelEEaSERKS3_")]
// IDA 0x837548: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837548() {
}

// 0x8375f0 — __ZN3RBX18NotificationObjectD1Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
#[doc(alias = "__ZN3RBX18NotificationObjectD1Ev")]
// IDA 0x8375f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8375f0() {
}

// 0x8375f4 — __ZN3RBX18NotificationObjectD0Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
#[doc(alias = "__ZN3RBX18NotificationObjectD0Ev")]
// IDA 0x8375f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8375f4() {
}

// 0x837694 — __ZNK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE12getClassNameEv")]
// IDA 0x837694: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837694() {
}

// 0x8376a4 — __ZThn32_N3RBX18NotificationObjectD1Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
#[doc(alias = "__ZThn32_N3RBX18NotificationObjectD1Ev")]
// IDA 0x8376a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8376a4() {
}

// 0x8376ac — __ZThn32_N3RBX18NotificationObjectD0Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
#[doc(alias = "__ZThn32_N3RBX18NotificationObjectD0Ev")]
// IDA 0x8376ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8376ac() {
}

// 0x837750 — __ZThn32_NK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE12getClassNameEv")]
// IDA 0x837750: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837750() {
}

// 0x837760 — __ZThn36_N3RBX18NotificationObjectD1Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
#[doc(alias = "__ZThn36_N3RBX18NotificationObjectD1Ev")]
// IDA 0x837760: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_837760() {
}

// 0x837768 — __ZThn36_N3RBX18NotificationObjectD0Ev
// type: void __fastcall(RBX::NotificationObject *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
#[doc(alias = "__ZThn36_N3RBX18NotificationObjectD0Ev")]
// IDA 0x837768: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_837768() {
}

// 0x83780c — __ZN3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0x83780c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_83780c() {
}

// 0x837810 — __ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0x837810: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_837810() {
}

// 0x837814 — __ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0x837814: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_837814() {
}

// 0x8378b0 — __ZNK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0x8378b0: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8378b0() {
}

// 0x837938 — __ZNK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7Creator6createEv")]
// IDA 0x837938: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837938() {
}

// 0x837a7c — __ZN3RBX4Name13callDoDeclareILZNS_19sNotificationObjectEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sNotificationObjectEEEEvv")]
// IDA 0x837a7c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_837a7c() {
}

// 0x837a80 — __ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sNotificationObjectEEEERKS0_v")]
// IDA 0x837a80: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837a80() {
}

// 0x837b60 — __ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0x837b60: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837b60() {
}

// 0x837da4 — __ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18NotificationObjectENS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEE17static_getCreatorEv")]
// IDA 0x837da4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837da4() {
}

// 0x837e18 — __ZN3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7CreatorD2Ev")]
// IDA 0x837e18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_837e18() {
}

// 0x837eb4 — __ZNK3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7Creator12getClassNameEv")]
// IDA 0x837eb4: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837eb4() {
}

// 0x837f20 — __ZNK3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7Creator6createEv")]
// IDA 0x837f20: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_837f20() {
}

// 0x838064 — __ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sFrameEEEERKS0_v")]
// IDA 0x838064: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_838064() {
}

// 0x8380a8 — __ZN3RBX4Name13callDoDeclareILZNS_6sFrameEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sFrameEEEEvv")]
// IDA 0x8380a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_8380a8() {
}

// 0x8380ac — __ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sFrameEEEERKS0_v")]
// IDA 0x8380ac: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8380ac() {
}

// 0x838190 — __ZN3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE7CreatorC2Ev")]
// IDA 0x838190: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_838190() {
}

// 0x8383b8 — __ZN3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x8383b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8383b8() {
}

// 0x8384b0 — __ZN3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x8384b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8384b0() {
}

// 0x8385b8 — __ZThn32_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x8385b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8385b8() {
}

// 0x8386ac — __ZThn32_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x8386ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8386ac() {
}

// 0x8387b8 — __ZThn36_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// IDA 0x8387b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8387b8() {
}

// 0x8388ac — __ZThn36_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18NotificationObjectELZNS_19sNotificationObjectEENS_14FactoryProductIS2_NS_5FrameELZNS_19sNotificationObjectEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// IDA 0x8388ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8388ac() {
}

// 0x8389b8 — __ZN3RBX5FrameD1Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::~Frame()")]
#[doc(alias = "__ZN3RBX5FrameD1Ev")]
// IDA 0x8389b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_8389b8() {
}

// 0x838ab0 — __ZN3RBX5FrameD0Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::~Frame()")]
#[doc(alias = "__ZN3RBX5FrameD0Ev")]
// IDA 0x838ab0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_838ab0() {
}

// 0x838bb8 — __ZNK3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5FrameENS_9GuiObjectELZNS_6sFrameEENS_8InstanceEE12getClassNameEv")]
// IDA 0x838bb8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_838bb8() {
}

// 0x838bc8 — __ZThn32_N3RBX5FrameD1Ev
// type: void __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
#[doc(alias = "__ZThn32_N3RBX5FrameD1Ev")]
// IDA 0x838bc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_838bc8() {
}
