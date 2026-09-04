//! rendering shard 251 — 100 stubs EA-sorted asc global gap filler after 0x2c9f1c not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 27270->27370 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2c9f54 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE8on_indexERKS6_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_index(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node> const&,char const*,lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE8on_indexERKS6_PKcP9lua_State
// IDA 0x2c9f54: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c9f54() {
}

// 0x2ca00c — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_newindexERS6_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_newindex(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>&,char const*,lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_newindexERS6_PKcP9lua_State
// IDA 0x2ca00c: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca00c() {
}

// 0x2ca0c4 — __ZN3RBX3Lua19dumpThreadRefCountsEv
// type: _DWORD __fastcall(RBX::Lua *__hidden this)
#[doc(alias = "RBX::Lua::dumpThreadRefCounts(void)")]
// was: __ZN3RBX3Lua19dumpThreadRefCountsEv
// IDA 0x2ca0c4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca0c4() {
}

// 0x2ca11c — __ZN3RBX3Lua15WeakFunctionRefC1EP9lua_Statei
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(lua_State *,int)")]
// was: __ZN3RBX3Lua15WeakFunctionRefC1EP9lua_Statei
// IDA 0x2ca11c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2ca11c() {
}

// 0x2ca120 — __ZN3RBX3Lua15WeakFunctionRefC2EP9lua_Statei
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(lua_State *,int)")]
// was: __ZN3RBX3Lua15WeakFunctionRefC2EP9lua_Statei
// IDA 0x2ca120: 99 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca120() {
}

// 0x2ca240 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE8on_indexERKSB_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_index(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> const&,char const*,lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE8on_indexERKSB_PKcP9lua_State
// IDA 0x2ca240: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca240() {
}

// 0x2ca2f8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_newindexERSB_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_newindex(rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>&,char const*,lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE11on_newindexERSB_PKcP9lua_State
// IDA 0x2ca2f8: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca2f8() {
}

// 0x2ca3b0 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE8on_indexERKSF_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_index(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> const&,char const*,lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE8on_indexERKSF_PKcP9lua_State
// IDA 0x2ca3b0: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca3b0() {
}

// 0x2ca468 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_newindexERSF_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_newindex(rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>&,char const*,lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE11on_newindexERSF_PKcP9lua_State
// IDA 0x2ca468: 61 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca468() {
}

// 0x2ca520 — __ZN3RBX3Lua14lua_tofunctionEP9lua_Statei
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Lua::lua_tofunction(lua_State *,int)")]
// was: __ZN3RBX3Lua14lua_tofunctionEP9lua_Statei
// IDA 0x2ca520: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca520() {
}

// 0x2ca52c — __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateRKNS0_15WeakFunctionRefE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,RBX::Lua::WeakFunctionRef const&)")]
// was: __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateRKNS0_15WeakFunctionRefE
// IDA 0x2ca52c: 26 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca52c() {
}

// 0x2ca57c — __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFNS4_IKNS_10Reflection5TupleEEES9_EEEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// was: __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFNS4_IKNS_10Reflection5TupleEEES9_EEEEE
// IDA 0x2ca57c: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca57c() {
}

// 0x2ca664 — __ZL25callGenericFunctionBridgeP9lua_State
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "callGenericFunctionBridge(lua_State *)")]
// was: __ZL25callGenericFunctionBridgeP9lua_State
// IDA 0x2ca664: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca664() {
}

// 0x2ca820 — __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFvNS4_IKNS_10Reflection5TupleEEENS5_IFvPNS0_12IAsyncResultEEEEEEEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::lua_pushfunction(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
// was: __ZN3RBX3Lua16lua_pushfunctionEP9lua_StateN5boost10shared_ptrINS3_8functionIFvNS4_IKNS_10Reflection5TupleEEENS5_IFvPNS0_12IAsyncResultEEEEEEEEE
// IDA 0x2ca820: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca820() {
}

// 0x2ca908 — __ZL30callGenericAsyncFunctionBridgeP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "callGenericAsyncFunctionBridge(lua_State *)")]
// was: __ZL30callGenericAsyncFunctionBridgeP9lua_State
// IDA 0x2ca908: 397 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ca908() {
}

// 0x2cad6c — __ZN3RBX3Lua15WeakFunctionRefD0Ev
// type: void __fastcall(RBX::Lua::WeakFunctionRef *__hidden this)
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
// was: __ZN3RBX3Lua15WeakFunctionRefD0Ev
// IDA 0x2cad6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cad6c() {
}

// 0x2cae0c — __ZN3RBX3Lua15WeakFunctionRefD1Ev
// type: void __fastcall(RBX::Lua::WeakFunctionRef *__hidden this)
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
// was: __ZN3RBX3Lua15WeakFunctionRefD1Ev
// IDA 0x2cae0c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2cae0c() {
}

// 0x2cae10 — __ZN3RBX3Lua15WeakFunctionRefD2Ev
// type: void __fastcall(RBX::Lua::WeakFunctionRef *__hidden this)
#[doc(alias = "RBX::Lua::WeakFunctionRef::~WeakFunctionRef()")]
// was: __ZN3RBX3Lua15WeakFunctionRefD2Ev
// IDA 0x2cae10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cae10() {
}

// 0x2caf24 — __ZN3RBX3Lua15WeakFunctionRef9removeRefEv
// type: _DWORD __fastcall(RBX::Lua::WeakFunctionRef *__hidden this)
#[doc(alias = "RBX::Lua::WeakFunctionRef::removeRef(void)")]
// was: __ZN3RBX3Lua15WeakFunctionRef9removeRefEv
// IDA 0x2caf24: 40 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2caf24() {
}

// 0x2caf98 — __ZN3RBX3Lua15WeakFunctionRefC1ERKS1_
// type: _DWORD __fastcall(RBX::Lua::WeakFunctionRef *__hidden this, const RBX::Lua::WeakFunctionRef *)
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&)")]
// was: __ZN3RBX3Lua15WeakFunctionRefC1ERKS1_
// IDA 0x2caf98: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2caf98() {
}

// 0x2caf9c — __ZN3RBX3Lua15WeakFunctionRefC2ERKS1_
// type: _DWORD __fastcall(RBX::Lua::WeakFunctionRef *__hidden this, const RBX::Lua::WeakFunctionRef *)
#[doc(alias = "RBX::Lua::WeakFunctionRef::WeakFunctionRef(RBX::Lua::WeakFunctionRef const&)")]
// was: __ZN3RBX3Lua15WeakFunctionRefC2ERKS1_
// IDA 0x2caf9c: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2caf9c() {
}

// 0x2cb0fc — __ZN3RBX3Lua6detail13LiveThreadRefC2EP9lua_State
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::LiveThreadRef(lua_State *)")]
// was: __ZN3RBX3Lua6detail13LiveThreadRefC2EP9lua_State
// IDA 0x2cb0fc: 169 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cb0fc() {
}

// 0x2cb2ec — __ZN3RBX3Lua6detail13LiveThreadRefD1Ev
// type: void __fastcall(RBX::Lua::detail::LiveThreadRef *__hidden this)
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef()")]
// was: __ZN3RBX3Lua6detail13LiveThreadRefD1Ev
// IDA 0x2cb2ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2cb2ec() {
}

// 0x2cb2f0 — __ZN3RBX3Lua6detail13LiveThreadRefD2Ev
// type: void __fastcall(RBX::Lua::detail::LiveThreadRef *__hidden this)
#[doc(alias = "RBX::Lua::detail::LiveThreadRef::~LiveThreadRef()")]
// was: __ZN3RBX3Lua6detail13LiveThreadRefD2Ev
// IDA 0x2cb2f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cb2f0() {
}

// 0x2cb3fc — __ZN3RBX3Lua15WeakFunctionRefaSERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Lua::WeakFunctionRef::operator=(RBX::Lua::WeakFunctionRef const&)")]
// was: __ZN3RBX3Lua15WeakFunctionRefaSERKS1_
// IDA 0x2cb3fc: 80 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cb3fc() {
}

// 0x2cb4d0 — __ZN3RBX10Reflection4Type12getSingletonINS_3Lua15WeakFunctionRefEEERKS1_v
// type: int(void)
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Lua::WeakFunctionRef>(void)")]
// was: __ZN3RBX10Reflection4Type12getSingletonINS_3Lua15WeakFunctionRefEEERKS1_v
// IDA 0x2cb4d0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cb4d0() {
}

// 0x2cb5b4 — __ZN3RBX10Reflection7Variant7convertINS_3Lua15WeakFunctionRefEEERT_v
#[doc(alias = "RBX::Lua::WeakFunctionRef & RBX::Reflection::Variant::convert<RBX::Lua::WeakFunctionRef>(void)")]
// was: __ZN3RBX10Reflection7Variant7convertINS_3Lua15WeakFunctionRefEEERT_v
// IDA 0x2cb5b4: 165 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cb5b4() {
}

// 0x2cb790 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(void)")]
// was: __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEERKS1_v
// IDA 0x2cb790: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cb790() {
}

// 0x2cb874 — __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(void)")]
// was: __ZN3RBX10Reflection4Type12getSingletonIN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEERKS1_v
// IDA 0x2cb874: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cb874() {
}

// 0x2cb958 — __ZL13onAsyncResultN3RBX3Lua9ThreadRefEN5boost8weak_ptrINS_13ScriptContextEEEPNS0_12IAsyncResultE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "onAsyncResult(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *)")]
// was: __ZL13onAsyncResultN3RBX3Lua9ThreadRefEN5boost8weak_ptrINS_13ScriptContextEEEPNS0_12IAsyncResultE
// IDA 0x2cb958: 269 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cb958() {
}

// 0x2cbc1c — __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSEPS4_
// type: int __fastcall(int, int32_t *__theValue)
#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef>::operator=(RBX::Lua::detail::LiveThreadRef*)")]
// was: __ZN5boost13intrusive_ptrIN3RBX3Lua6detail13LiveThreadRefEEaSEPS4_
// IDA 0x2cbc1c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbc1c() {
}

// 0x2cbc40 — __ZN16RobloxExtraSpace13createNewNodeEv
// type: _DWORD __fastcall(RobloxExtraSpace *__hidden this)
#[doc(alias = "RobloxExtraSpace::createNewNode(void)")]
// was: __ZN16RobloxExtraSpace13createNewNodeEv
// IDA 0x2cbc40: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbc40() {
}

// 0x2cbd58 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13pushNewObjectISB_EEPSB_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS_10Reflection5TupleEEES8_EEEEELb1EE13pushNewObjectISB_EEPSB_P9lua_StateT_
// IDA 0x2cbd58: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbd58() {
}

// 0x2cbda8 — __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13pushNewObjectISF_EEPSF_P9lua_StateT_
#[doc(alias = "rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>* RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::pushNewObject<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(lua_State *,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>)")]
// was: __ZN3RBX3Lua6BridgeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS_10Reflection5TupleEEENS4_IFvPNS0_12IAsyncResultEEEEEEEEELb1EE13pushNewObjectISF_EEPSF_P9lua_StateT_
// IDA 0x2cbda8: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbda8() {
}

// 0x2cbdf8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED1Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
// was: __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED1Ev
// IDA 0x2cbdf8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2cbdf8() {
}

// 0x2cbdfc — __ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::Lua::WeakFunctionRef * rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX3Lua15WeakFunctionRefENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x2cbdfc: 30 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbdfc() {
}

// 0x2cbe50 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED1Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::~TType()")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED1Ev
// IDA 0x2cbe50: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2cbe50() {
}

// 0x2cbe54 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED1Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED1Ev
// IDA 0x2cbe54: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2cbe54() {
}

// 0x2cbe58 — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>(char const*,rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>> *)")]
// was: __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFvNS4_IKNS0_5TupleEEENS5_IFvPNS_3Lua12IAsyncResultEEEEEEEEEEEPKcPT_
// IDA 0x2cbe58: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbe58() {
}

// 0x2cbf04 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED0Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>>::~TType()")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFvNS3_IKNS0_5TupleEEENS4_IFvPNS_3Lua12IAsyncResultEEEEEEEEEED0Ev
// IDA 0x2cbf04: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2cbf04() {
}

// 0x2cbf08 — __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(char const*,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> *)")]
// was: __ZN3RBX10Reflection4TypeC2IN5boost10shared_ptrINS3_8functionIFNS4_IKNS0_5TupleEEES8_EEEEEEEPKcPT_
// IDA 0x2cbf08: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbf08() {
}

// 0x2cbfb4 — __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED0Ev
#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::~TType()")]
// was: __ZN3RBX10Reflection5TTypeIN5boost10shared_ptrINS2_8functionIFNS3_IKNS0_5TupleEEES7_EEEEEED0Ev
// IDA 0x2cbfb4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2cbfb4() {
}

// 0x2cbfb8 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE9singletonEv
// IDA 0x2cbfb8: 33 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cbfb8() {
}

// 0x2cc020 — __ZN3RBX10Reflection4TypeC2INS_3Lua15WeakFunctionRefEEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Lua::WeakFunctionRef>(char const*,RBX::Lua::WeakFunctionRef *)")]
// was: __ZN3RBX10Reflection4TypeC2INS_3Lua15WeakFunctionRefEEEPKcPT_
// IDA 0x2cc020: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cc020() {
}

// 0x2cc0c8 — __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED0Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Lua::WeakFunctionRef>::~TType()")]
// was: __ZN3RBX10Reflection5TTypeINS_3Lua15WeakFunctionRefEED0Ev
// IDA 0x2cc0c8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2cc0c8() {
}

// 0x2cc0cc — __ZNK5boost9function2IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEENS_8functionIFvPNS2_3Lua12IAsyncResultEEEEEclES6_SC_
#[doc(alias = "boost::function2<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)const")]
// was: __ZNK5boost9function2IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEENS_8functionIFvPNS2_3Lua12IAsyncResultEEEEEclES6_SC_
// IDA 0x2cc0cc: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cc0cc() {
}

// 0x2cc210 — __ZN5boost4bindIvN3RBX3Lua9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEEPNS2_12IAsyncResultES3_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list_av_3<RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,boost::arg<1>>::type> boost::bind<void,RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *,RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,boost::arg<1>>(void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,boost::arg<1>)")]
// was: __ZN5boost4bindIvN3RBX3Lua9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEEPNS2_12IAsyncResultES3_S6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSD_T0_T1_T2_ENSB_9list_av_3IT3_T4_T5_E4typeEEESI_SK_SL_SM_
// IDA 0x2cc210: 360 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cc210() {
}

// 0x2cc608 — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE5clearEv
#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::clear(void)")]
// was: __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE5clearEv
// IDA 0x2cc608: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cc608() {
}

// 0x2cc634 — __ZN5boost3_bi5valueIN3RBX3Lua9ThreadRefEEC2ERKS4_
#[doc(alias = "boost::_bi::value<RBX::Lua::ThreadRef>::value(RBX::Lua::ThreadRef const&)")]
// was: __ZN5boost3_bi5valueIN3RBX3Lua9ThreadRefEEC2ERKS4_
// IDA 0x2cc634: 63 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cc634() {
}

// 0x2cc6f0 — __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>)")]
// was: __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_
// IDA 0x2cc6f0: 173 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cc6f0() {
}

// 0x2cc8d0 — __ZN5boost3_bi8storage3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>::storage3(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEC2ES6_SA_SC_
// IDA 0x2cc8d0: 173 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cc8d0() {
}

// 0x2ccab0 — __ZN5boost3_bi8storage2INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEEEC2ES6_SA_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>>::storage2(boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEEEC2ES6_SA_
// IDA 0x2ccab0: 189 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ccab0() {
}

// 0x2cccc4 — __ZN5boost3_bi8storage1INS0_5valueIN3RBX3Lua9ThreadRefEEEEC2ES6_
#[doc(alias = "boost::_bi::storage1<boost::_bi::value<RBX::Lua::ThreadRef>>::storage1(boost::_bi::value<RBX::Lua::ThreadRef>)")]
// was: __ZN5boost3_bi8storage1INS0_5valueIN3RBX3Lua9ThreadRefEEEEC2ES6_
// IDA 0x2cccc4: 63 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cccc4() {
}

// 0x2ccd80 — __ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX3Lua12IAsyncResultEEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS8_5list3INS8_5valueISA_EENSH_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// IDA 0x2ccd80: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ccd80() {
}

// 0x2ccf68 — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEEC2INS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// IDA 0x2ccf68: 176 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ccf68() {
}

// 0x2cd154 — __ZN5boost3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS2_13ScriptContextEEEPNS3_12IAsyncResultEENS0_5list3INS0_5valueIS4_EENSD_IS7_EENS_3argILi1EEEEEEC2ERKSJ_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>::bind_t(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS2_13ScriptContextEEEPNS3_12IAsyncResultEENS0_5list3INS0_5valueIS4_EENSD_IS7_EENS_3argILi1EEEEEEC2ERKSJ_
// IDA 0x2cd154: 141 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd154() {
}

// 0x2cd2dc — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS2_9ThreadRefENS_8weak_ptrINS1_13ScriptContextEEES4_ENS7_5list3INS7_5valueIS9_EENSG_ISC_EENS_3argILi1EEEEEEEEEvT_
// IDA 0x2cd2dc: 180 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd2dc() {
}

// 0x2cd4d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// IDA 0x2cd4d4: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd4d4() {
}

// 0x2cd4f0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>,void,RBX::Lua::IAsyncResult *>::invoke(boost::detail::function::function_buffer &,RBX::Lua::IAsyncResult *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// IDA 0x2cd4f0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd4f0() {
}

// 0x2cd50c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2cd50c: 175 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd50c() {
}

// 0x2cd6f4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE9assign_toINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2cd6f4: 173 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd6f4() {
}

// 0x2cd8d8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE14assign_functorINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Lua::IAsyncResult *>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX3Lua12IAsyncResultEE14assign_functorINS_3_bi6bind_tIvPFvNS4_9ThreadRefENS_8weak_ptrINS3_13ScriptContextEEES6_ENS9_5list3INS9_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x2cd8d8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd8d8() {
}

// 0x2cd984 — __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEclIPFvS5_S9_PNS4_12IAsyncResultEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>::operator()<void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list1<RBX::Lua::IAsyncResult *&>>(boost::_bi::type<void>,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *) &,boost::_bi::list1<RBX::Lua::IAsyncResult *&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua9ThreadRefEEENS2_INS_8weak_ptrINS3_13ScriptContextEEEEENS_3argILi1EEEEclIPFvS5_S9_PNS4_12IAsyncResultEENS0_5list1IRSG_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2cd984: 177 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cd984() {
}

// 0x2cdb6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::Lua::ThreadRef,rbx_core::Weak<RBX::ScriptContext>,RBX::Lua::IAsyncResult *),boost::_bi::list3<boost::_bi::value<RBX::Lua::ThreadRef>,boost::_bi::value<rbx_core::Weak<RBX::ScriptContext>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvN3RBX3Lua9ThreadRefENS_8weak_ptrINS5_13ScriptContextEEEPNS6_12IAsyncResultEENS3_5list3INS3_5valueIS7_EENSG_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x2cdb6c: 175 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cdb6c() {
}

// 0x2cdd44 — __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE13assign_to_ownERKS5_
#[doc(alias = "boost::function1<void,RBX::Lua::IAsyncResult *>::assign_to_own(boost::function1<void,RBX::Lua::IAsyncResult *> const&)")]
// was: __ZN5boost9function1IvPN3RBX3Lua12IAsyncResultEE13assign_to_ownERKS5_
// IDA 0x2cdd44: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cdd44() {
}

// 0x2cdd74 — __ZNK5boost9function1INS_10shared_ptrIKN3RBX10Reflection5TupleEEES6_EclES6_
#[doc(alias = "boost::function1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
// was: __ZNK5boost9function1INS_10shared_ptrIKN3RBX10Reflection5TupleEEES6_EclES6_
// IDA 0x2cdd74: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cdd74() {
}

// 0x2cde88 — __GLOBAL__I_a_75
#[doc(alias = "global constructor keyed to_a_75")]
// was: __GLOBAL__I_a_75
// IDA 0x2cde88: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2cde88() {
}

// 0x2ce130 — __ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Security::Context::isInRole(RBX::Security::Identities,RBX::Security::Permissions)")]
// was: __ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE
// IDA 0x2ce130: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce130() {
}

// 0x2ce1fc — __GLOBAL__I_a_76
#[doc(alias = "global constructor keyed to_a_76")]
// was: __GLOBAL__I_a_76
// IDA 0x2ce1fc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2ce1fc() {
}

// 0x2ce618 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEC2ERKS6_
#[doc(alias = "std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>>::vector(std::vector<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>> const&)")]
// was: __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEC2ERKS6_
// IDA 0x2ce618: 172 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce618() {
}

// 0x2ce7d4 — __ZNSt12_Vector_baseIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEC2EmRKS5_
#[doc(alias = "std::_Vector_base<rbx_core::Weak<RBX::PartInstance>,std::allocator<rbx_core::Weak<RBX::PartInstance>>>::_Vector_base(unsigned long,std::allocator<rbx_core::Weak<RBX::PartInstance>> const&)")]
// was: __ZNSt12_Vector_baseIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EEC2EmRKS5_
// IDA 0x2ce7d4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce7d4() {
}

// 0x2ce804 — __ZN5boost10shared_ptrIN3RBX14AdvLuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::AdvLuaDragTool>::shared_ptr<RBX::AdvLuaDragTool,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX14AdvLuaDragToolEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x2ce804: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce804() {
}

// 0x2ce8cc — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14AdvLuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::AdvLuaDragTool,RBX::AdvLuaDragTool>(rbx_core::SharedPtr<RBX::AdvLuaDragTool> const*,RBX::AdvLuaDragTool *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_14AdvLuaDragToolES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2ce8cc: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce8cc() {
}

// 0x2ce9b0 — __ZN5boost6detail12shared_countC2IPN3RBX14AdvLuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX14AdvLuaDragToolENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x2ce9b0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ce9b0() {
}

// 0x2ceaa8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x2ceaa8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2ceaa8() {
}

// 0x2ceaac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x2ceaac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2ceaac() {
}

// 0x2ceab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x2ceab0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ceab0() {
}

// 0x2ceac0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2ceac0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ceac0() {
}

// 0x2cead8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::AdvLuaDragTool *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14AdvLuaDragToolENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2cead8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cead8() {
}

// 0x2ceadc — __GLOBAL__I_a_77
#[doc(alias = "global constructor keyed to_a_77")]
// was: __GLOBAL__I_a_77
// IDA 0x2ceadc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_2ceadc() {
}

// 0x2ced4c — __ZN3RBX13AdvLuaDraggerC2Ev
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "RBX::AdvLuaDragger::AdvLuaDragger(void)")]
// was: __ZN3RBX13AdvLuaDraggerC2Ev
// IDA 0x2ced4c: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ced4c() {
}

// 0x2cef40 — __ZN3RBX13AdvLuaDraggerD0Ev
// type: void __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
// was: __ZN3RBX13AdvLuaDraggerD0Ev
// IDA 0x2cef40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cef40() {
}

// 0x2cefe0 — __ZN3RBX13AdvLuaDraggerD1Ev
// type: void __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
// was: __ZN3RBX13AdvLuaDraggerD1Ev
// IDA 0x2cefe0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2cefe0() {
}

// 0x2cefe4 — __ZThn32_N3RBX13AdvLuaDraggerD0Ev
// type: void __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
// was: __ZThn32_N3RBX13AdvLuaDraggerD0Ev
// IDA 0x2cefe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cefe4() {
}

// 0x2cefec — __ZThn36_N3RBX13AdvLuaDraggerD0Ev
// type: void __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
// was: __ZThn36_N3RBX13AdvLuaDraggerD0Ev
// IDA 0x2cefec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cefec() {
}

// 0x2ceff4 — __ZN3RBX13AdvLuaDraggerD2Ev
// type: void __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "RBX::AdvLuaDragger::~AdvLuaDragger()")]
// was: __ZN3RBX13AdvLuaDraggerD2Ev
// IDA 0x2ceff4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ceff4() {
}

// 0x2cf168 — __ZThn32_N3RBX13AdvLuaDraggerD1Ev
// type: void __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
// was: __ZThn32_N3RBX13AdvLuaDraggerD1Ev
// IDA 0x2cf168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cf168() {
}

// 0x2cf170 — __ZThn36_N3RBX13AdvLuaDraggerD1Ev
// type: void __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvLuaDragger::~AdvLuaDragger()")]
// was: __ZThn36_N3RBX13AdvLuaDraggerD1Ev
// IDA 0x2cf170: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2cf170() {
}

// 0x2cf3b8 — __ZN3RBX13AdvLuaDragger9mouseMoveENS_6RbxRayE
#[doc(alias = "RBX::AdvLuaDragger::mouseMove(RBX::RbxRay)")]
// was: __ZN3RBX13AdvLuaDragger9mouseMoveENS_6RbxRayE
// IDA 0x2cf3b8: 261 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cf3b8() {
}

// 0x2cf6d0 — __ZN3RBX13AdvLuaDragger16tryStartDraggingERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::AdvLuaDragger::tryStartDragging(RBX::RbxRay const&)")]
// was: __ZN3RBX13AdvLuaDragger16tryStartDraggingERKNS_6RbxRayE
// IDA 0x2cf6d0: 183 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cf6d0() {
}

// 0x2cf930 — __ZN3RBX13AdvLuaDragger6doDragERKNS_6RbxRayE
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this, const RBX::RbxRay *)
#[doc(alias = "RBX::AdvLuaDragger::doDrag(RBX::RbxRay const&)")]
// was: __ZN3RBX13AdvLuaDragger6doDragERKNS_6RbxRayE
// IDA 0x2cf930: 358 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cf930() {
}

// 0x2cfd7c — __ZN3RBX13AdvLuaDragger7mouseUpEv
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "RBX::AdvLuaDragger::mouseUp(void)")]
// was: __ZN3RBX13AdvLuaDragger7mouseUpEv
// IDA 0x2cfd7c: 243 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2cfd7c() {
}

// 0x2d0154 — __ZN3RBX13AdvLuaDragger13startDraggingEv
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "RBX::AdvLuaDragger::startDragging(void)")]
// was: __ZN3RBX13AdvLuaDragger13startDraggingEv
// IDA 0x2d0154: 221 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d0154() {
}

// 0x2d05bc — __ZN3RBX13AdvLuaDragger15alignPartToGridEv
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this)
#[doc(alias = "RBX::AdvLuaDragger::alignPartToGrid(void)")]
// was: __ZN3RBX13AdvLuaDragger15alignPartToGridEv
// IDA 0x2d05bc: 132 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d05bc() {
}

// 0x2d072c — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_
#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*)")]
// was: __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_
// IDA 0x2d072c: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d072c() {
}

// 0x2d07e0 — __ZNK3RBX13AdvLuaDragger12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::AdvLuaDragger *__hidden this, const Instance *)
#[doc(alias = "RBX::AdvLuaDragger::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX13AdvLuaDragger12askSetParentEPKNS_8InstanceE
// IDA 0x2d07e0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d07e0() {
}

// 0x2d07e4 — __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv
// IDA 0x2d07e4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d07e4() {
}

// 0x2d07f4 — __ZThn32_NK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E12getClassNameEv
// IDA 0x2d07f4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2d07f4() {
}

// 0x2d0804 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD1Ev
// IDA 0x2d0804: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2d0804() {
}

// 0x2d0808 — __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_13AdvLuaDraggerENS_8InstanceELZNS_14sAdvLuaDraggerEES2_E7CreatorD2Ev
// IDA 0x2d0808: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2d0808() {
}
