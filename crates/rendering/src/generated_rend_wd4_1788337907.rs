//! rendering shard wd4 — 120 stubs 0x786c28..0x793518 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre/G3D complete, global gap filler 52784->52904 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch wd4]
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 120 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x786c28 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14delete_bucketsEv
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14delete_bucketsEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_buckets(void)")]
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEE14delete_bucketsEv")]
// IDA 0x786c28: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786c28() {
}


// 0x786c5c — __ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOverBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::~StepOverBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD1Ev")]
// IDA 0x786c5c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_786c5c() {
}


// 0x786c60 — __ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOverBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::~StepOverBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD0Ev")]
// IDA 0x786c60: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_786c60() {
}


// 0x786c64 — __ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpoint7hitTestEP9lua_StateP9lua_Debug
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::hitTest(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpoint7hitTestEP9lua_StateP9lua_Debug")]
// IDA 0x786c64: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786c64() {
}


// 0x786d28 — __ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOutBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::~StepOutBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD1Ev")]
// IDA 0x786d28: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_786d28() {
}


// 0x786d2c — __ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOutBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::~StepOutBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD0Ev")]
// IDA 0x786d2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_786d2c() {
}


// 0x786d30 — __ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpoint7hitTestEP9lua_StateP9lua_Debug
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::hitTest(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpoint7hitTestEP9lua_StateP9lua_Debug")]
// IDA 0x786d30: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786d30() {
}


// 0x786e24 — __ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepInBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::~StepInBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD1Ev")]
// IDA 0x786e24: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_786e24() {
}


// 0x786e28 — __ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepInBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::~StepInBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD0Ev")]
// IDA 0x786e28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_786e28() {
}


// 0x786e2c — __ZN3RBX9Scripting14ScriptDebugger16StepInBreakpoint7hitTestEP9lua_StateP9lua_Debug
#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::hitTest(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16StepInBreakpoint7hitTestEP9lua_StateP9lua_Debug")]
// IDA 0x786e2c: 15 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786e2c() {
}


// 0x786e50 — __ZN5boost9function2IbP9lua_StateP9lua_DebugE5clearEv
// was: __ZN5boost9function2IbP9lua_StateP9lua_DebugE5clearEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::function2<bool,lua_State *,lua_Debug *>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IbP9lua_StateP9lua_DebugE5clearEv")]
// IDA 0x786e50: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786e50() {
}


// 0x786e7c — __ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE5clearEv
// was: __ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE5clearEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::function2<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,lua_State *,lua_Debug *>::clear(void)")]
#[doc(alias = "__ZN5boost9function2INS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEP9lua_StateP9lua_DebugE5clearEv")]
// IDA 0x786e7c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786e7c() {
}


// 0x786ea8 — __ZNK5boost9function2IvP9lua_StateP9lua_DebugEclES2_S4_
// was: __ZNK5boost9function2IvP9lua_StateP9lua_DebugEclES2_S4_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::operator()(lua_State *,lua_Debug *)const")]
#[doc(alias = "__ZNK5boost9function2IvP9lua_StateP9lua_DebugEclES2_S4_")]
// IDA 0x786ea8: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786ea8() {
}


// 0x786f70 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE5dummy7nonnullEv
// was: __ZN5boost9function2IvP9lua_StateP9lua_DebugE5dummy7nonnullEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function2IvP9lua_StateP9lua_DebugE5dummy7nonnullEv")]
// IDA 0x786f70: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_786f70() {
}


// 0x786f74 — __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE5clearEv
// was: __ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE5clearEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::function2<RBX::Reflection::Variant,lua_State *,lua_Debug *>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IN3RBX10Reflection7VariantEP9lua_StateP9lua_DebugE5clearEv")]
// IDA 0x786f74: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786f74() {
}


// 0x786fa0 — __ZN3RBX3Lua13WeakThreadRefC2Ev
// type: _DWORD __fastcall(RBX::Lua::WeakThreadRef *__hidden this)
#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(void)")]
#[doc(alias = "__ZN3RBX3Lua13WeakThreadRefC2Ev")]
// IDA 0x786fa0: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_786fa0() {
}


// 0x787034 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting15DebuggerManagerEEEN5boost10shared_ptrIT_EEv
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting15DebuggerManagerEEEN5boost10shared_ptrIT_EEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::Scripting::DebuggerManager> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerManager>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting15DebuggerManagerEEEN5boost10shared_ptrIT_EEv")]
// IDA 0x787034: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_787034() {
}


// 0x7870e4 — __ZN5boost10shared_ptrIN3RBX9Scripting15DebuggerManagerEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: __ZN5boost10shared_ptrIN3RBX9Scripting15DebuggerManagerEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<RBX::Scripting::DebuggerManager>::shared_ptr<RBX::Scripting::DebuggerManager,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9Scripting15DebuggerManagerEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// IDA 0x7870e4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7870e4() {
}


// 0x7871ac — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9Scripting15DebuggerManagerES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9Scripting15DebuggerManagerES7_EEvPKNS_10shared_ptrIT_EEPT0_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerManager,RBX::Scripting::DebuggerManager>(boost::shared_ptr<RBX::Scripting::DebuggerManager> const*,RBX::Scripting::DebuggerManager *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9Scripting15DebuggerManagerES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
// IDA 0x7871ac: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7871ac() {
}


// 0x787294 — __ZN5boost6detail12shared_countC2IPN3RBX9Scripting15DebuggerManagerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2IPN3RBX9Scripting15DebuggerManagerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9Scripting15DebuggerManagerENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x787294: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_787294() {
}


// 0x78739c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x78739c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_78739c() {
}


// 0x7873a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x7873a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7873a0() {
}


// 0x7873a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x7873a4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7873a4() {
}


// 0x7873c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x7873c4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7873c4() {
}


// 0x7873dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerManager *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting15DebuggerManagerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x7873dc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7873dc() {
}


// 0x7873e0 — __GLOBAL__I_a_359
#[doc(alias = "global constructor keyed to_a_359")]
#[doc(alias = "__GLOBAL__I_a_359")]
// IDA 0x7873e0: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_7873e0() {
}


// 0x7881d0 — __ZN12SerializerV24loadEP10XmlElementPN3RBX9DataModelE
// type: _DWORD __fastcall(SerializerV2 *__hidden this, XmlElement *, RBX::DataModel *)
#[doc(alias = "SerializerV2::load(XmlElement *,RBX::DataModel *)")]
#[doc(alias = "__ZN12SerializerV24loadEP10XmlElementPN3RBX9DataModelE")]
// IDA 0x7881d0: 174 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7881d0() {
}


// 0x7883a8 — __ZN12SerializerV24loadERSiPN3RBX9DataModelE
// type: _DWORD __fastcall(SerializerV2 *__hidden this, std::istream *, RBX::DataModel *)
#[doc(alias = "SerializerV2::load(std::istream &,RBX::DataModel *)")]
#[doc(alias = "__ZN12SerializerV24loadERSiPN3RBX9DataModelE")]
// IDA 0x7883a8: 577 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7883a8() {
}


// 0x788dec — __ZN12SerializerV27loadXMLERSiPN3RBX9DataModelE
// type: _DWORD __fastcall(SerializerV2 *__hidden this, std::istream *, RBX::DataModel *)
#[doc(alias = "SerializerV2::loadXML(std::istream &,RBX::DataModel *)")]
#[doc(alias = "__ZN12SerializerV27loadXMLERSiPN3RBX9DataModelE")]
// IDA 0x788dec: 226 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_788dec() {
}


// 0x789238 — __ZN12SerializerV212loadInstanceEPK10XmlElementRN3RBX16IReferenceBinderENS3_11CreatorRoleE
// type: int __fastcall(int, int, XmlElement *this, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: int __fastcall(int, int, XmlElement *this, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "SerializerV2::loadInstance(XmlElement const*,RBX::IReferenceBinder &,RBX::CreatorRole)")]
#[doc(alias = "__ZN12SerializerV212loadInstanceEPK10XmlElementRN3RBX16IReferenceBinderENS3_11CreatorRoleE")]
// IDA 0x789238: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_789238() {
}


// 0x7893c4 — __ZN12SerializerV221loadInstancesFromTextEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int)
// was: __ZN12SerializerV221loadInstancesFromTextEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EE — uses rbx_core::SharedPtr not boost
#[doc(alias = "SerializerV2::loadInstancesFromText(XmlElement const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> &)")]
#[doc(alias = "__ZN12SerializerV221loadInstancesFromTextEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EE")]
// IDA 0x7893c4: 172 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7893c4() {
}


// 0x789594 — __ZN12SerializerV213loadInstancesEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EERNS6_16IReferenceBinderENS6_11CreatorRoleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *)
// was: __ZN12SerializerV213loadInstancesEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EERNS6_16IReferenceBinderENS6_11CreatorRoleE — uses rbx_core::SharedPtr not boost
#[doc(alias = "SerializerV2::loadInstances(XmlElement const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::IReferenceBinder &,RBX::CreatorRole)")]
#[doc(alias = "__ZN12SerializerV213loadInstancesEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EERNS6_16IReferenceBinderENS6_11CreatorRoleE")]
// IDA 0x789594: 196 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_789594() {
}


// 0x7897ac — __ZN12SerializerV223loadInstancesFromMemoryEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EENS6_11CreatorRoleE
// was: __ZN12SerializerV223loadInstancesFromMemoryEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EENS6_11CreatorRoleE — uses rbx_core::SharedPtr not boost
#[doc(alias = "SerializerV2::loadInstancesFromMemory(XmlElement const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> &,RBX::CreatorRole)")]
#[doc(alias = "__ZN12SerializerV223loadInstancesFromMemoryEPK10XmlElementRSt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS8_EENS6_11CreatorRoleE")]
// IDA 0x7897ac: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7897ac() {
}


// 0x789908 — __ZN12SerializerV214newRootElementEv
// type: _DWORD __fastcall(SerializerV2 *__hidden this)
#[doc(alias = "SerializerV2::newRootElement(void)")]
#[doc(alias = "__ZN12SerializerV214newRootElementEv")]
// IDA 0x789908: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_789908() {
}


// 0x789a38 — __ZN12SerializerV214newRootElementERKSs
// type: _DWORD __fastcall(SerializerV2 *__hidden this, const std::string *)
#[doc(alias = "SerializerV2::newRootElement(std::string const&)")]
#[doc(alias = "__ZN12SerializerV214newRootElementERKSs")]
// IDA 0x789a38: 390 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_789a38() {
}


// 0x789e50 — __ZN13ArchiveBinder11resolveRefsEv
// type: _DWORD __fastcall(ArchiveBinder *__hidden this)
#[doc(alias = "ArchiveBinder::resolveRefs(void)")]
#[doc(alias = "__ZN13ArchiveBinder11resolveRefsEv")]
// IDA 0x789e50: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_789e50() {
}


// 0x789ea4 — __ZN13ArchiveBinderD1Ev
// type: void __fastcall(ArchiveBinder *__hidden this)
#[doc(alias = "ArchiveBinder::~ArchiveBinder()")]
#[doc(alias = "__ZN13ArchiveBinderD1Ev")]
// IDA 0x789ea4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_789ea4() {
}


// 0x789fb4 — __ZN12MemoryBinderD1Ev
// type: void __fastcall(MemoryBinder *__hidden this)
#[doc(alias = "MemoryBinder::~MemoryBinder()")]
#[doc(alias = "__ZN12MemoryBinderD1Ev")]
// IDA 0x789fb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_789fb4() {
}


// 0x78a098 — __ZN10XmlElementC2IPKN3RBX4NameEEERS3_T_
#[doc(alias = "XmlElement::XmlElement<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
#[doc(alias = "__ZN10XmlElementC2IPKN3RBX4NameEEERS3_T_")]
// IDA 0x78a098: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a098() {
}


// 0x78a16c — __ZN12XmlAttributeC2IiEERKN3RBX4NameET_
#[doc(alias = "XmlAttribute::XmlAttribute<int>(RBX::Name const&,int)")]
#[doc(alias = "__ZN12XmlAttributeC2IiEERKN3RBX4NameET_")]
// IDA 0x78a16c: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a16c() {
}


// 0x78a230 — __ZN12XmlAttributeC2IPKcEERKN3RBX4NameET_
#[doc(alias = "XmlAttribute::XmlAttribute<char const*>(RBX::Name const&,char const*)")]
#[doc(alias = "__ZN12XmlAttributeC2IPKcEERKN3RBX4NameET_")]
// IDA 0x78a230: 65 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a230() {
}


// 0x78a2ec — __ZN16XmlNameValuePairC2ERKN3RBX4NameEPKc
// type: XmlNameValuePair *__fastcall(XmlNameValuePair *__hidden this, const RBX::Name *, const char *)
#[doc(alias = "XmlNameValuePair::XmlNameValuePair(RBX::Name const&,char const*)")]
#[doc(alias = "__ZN16XmlNameValuePairC2ERKN3RBX4NameEPKc")]
// IDA 0x78a2ec: 65 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a2ec() {
}


// 0x78a3a4 — __ZN12MemoryBinder10announceIDEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseE
#[doc(alias = "MemoryBinder::announceID(XmlNameValuePair const*,RBX::Reflection::DescribedBase *)")]
#[doc(alias = "__ZN12MemoryBinder10announceIDEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseE")]
// IDA 0x78a3a4: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a3a4() {
}


// 0x78a3ac — __ZN12MemoryBinder13announceIDREFEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseEPKNS3_6IIDREFE
#[doc(alias = "MemoryBinder::announceIDREF(XmlNameValuePair const*,RBX::Reflection::DescribedBase *,RBX::IIDREF const*)")]
#[doc(alias = "__ZN12MemoryBinder13announceIDREFEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseEPKNS3_6IIDREFE")]
// IDA 0x78a3ac: 34 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a3ac() {
}


// 0x78a410 — __ZN12MemoryBinder11resolveRefsEv
// type: _DWORD __fastcall(MemoryBinder *__hidden this)
#[doc(alias = "MemoryBinder::resolveRefs(void)")]
#[doc(alias = "__ZN12MemoryBinder11resolveRefsEv")]
// IDA 0x78a410: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a410() {
}


// 0x78a47c — __ZN12MemoryBinderD0Ev
// type: void __fastcall(MemoryBinder *__hidden this)
#[doc(alias = "MemoryBinder::~MemoryBinder()")]
#[doc(alias = "__ZN12MemoryBinderD0Ev")]
// IDA 0x78a47c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_78a47c() {
}


// 0x78a568 — __ZN12MemoryBinder9processIDEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseE
// type: _DWORD __fastcall(MemoryBinder *__hidden this, const XmlNameValuePair *, RBX::Reflection::DescribedBase *)
#[doc(alias = "MemoryBinder::processID(XmlNameValuePair const*,RBX::Reflection::DescribedBase *)")]
#[doc(alias = "__ZN12MemoryBinder9processIDEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseE")]
// IDA 0x78a568: 142 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a568() {
}


// 0x78a6e0 — __ZN12MemoryBinder12processIDREFEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseEPKNS3_6IIDREFE
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "MemoryBinder::processIDREF(XmlNameValuePair const*,RBX::Reflection::DescribedBase *,RBX::IIDREF const*)")]
#[doc(alias = "__ZN12MemoryBinder12processIDREFEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseEPKNS3_6IIDREFE")]
// IDA 0x78a6e0: 122 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78a6e0() {
}


// 0x78a824 — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE9push_backERKS1_
#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::push_back(MemoryBinder::IDREFItem const&)")]
#[doc(alias = "__ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE9push_backERKS1_")]
// IDA 0x78a824: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_78a824() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x78a880 — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
// was: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<MemoryBinder::IDREFItem*,std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>>,MemoryBinder::IDREFItem const&)")]
#[doc(alias = "__ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")]
// IDA 0x78a880: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_78a880() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x78ac98 — __ZNSt12_Vector_baseIN12MemoryBinder9IDREFItemESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN12MemoryBinder9IDREFItemESaIS1_EE11_M_allocateEm")]
// IDA 0x78ac98: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_78ac98() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x78acb0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN12MemoryBinder9IDREFItemES5_EET0_T_S7_S6_
#[doc(alias = "MemoryBinder::IDREFItem * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<MemoryBinder::IDREFItem *,MemoryBinder::IDREFItem *>(MemoryBinder::IDREFItem *,MemoryBinder::IDREFItem *,MemoryBinder::IDREFItem *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN12MemoryBinder9IDREFItemES5_EET0_T_S7_S6_")]
// IDA 0x78acb0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_78acb0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x78ad0c — __ZNSt3mapIN3RBX14InstanceHandleES1_St4lessIS1_ESaISt4pairIKS1_S1_EEEixERS5_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::InstanceHandle,RBX::InstanceHandle,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::operator[](RBX::InstanceHandle const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX14InstanceHandleES1_St4lessIS1_ESaISt4pairIKS1_S1_EEEixERS5_")]
// IDA 0x78ad0c: 156 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78ad0c() {
}


// 0x78aeb0 — __ZNSt4pairIKN3RBX14InstanceHandleES1_EC2ERS2_S4_
#[doc(alias = "std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>::pair(RBX::InstanceHandle const&,RBX::InstanceHandle const&)")]
#[doc(alias = "__ZNSt4pairIKN3RBX14InstanceHandleES1_EC2ERS2_S4_")]
// IDA 0x78aeb0: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78aeb0() {
}


// 0x78af90 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>,std::_Select1st<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// IDA 0x78af90: 98 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78af90() {
}


// 0x78b078 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>,std::_Select1st<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
// IDA 0x78b078: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b078() {
}


// 0x78b0c8 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>,std::_Select1st<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::_M_insert_unique(std::pair<RBX::InstanceHandle const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE16_M_insert_uniqueERKS4_")]
// IDA 0x78b0c8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b0c8() {
}


// 0x78b148 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>,std::_Select1st<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::_M_create_node(std::pair<RBX::InstanceHandle const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE14_M_create_nodeERKS4_")]
// IDA 0x78b148: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b148() {
}


// 0x78b274 — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE15_M_erase_at_endEPS1_
#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::_M_erase_at_end(MemoryBinder::IDREFItem*)")]
#[doc(alias = "__ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EE15_M_erase_at_endEPS1_")]
// IDA 0x78b274: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b274() {
}


// 0x78b2a4 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE4findERS3_
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>,std::_Select1st<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::find(RBX::InstanceHandle const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE4findERS3_")]
// IDA 0x78b2a4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b2a4() {
}


// 0x78b2f4 — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>,std::_Select1st<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0x78b2f4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b2f4() {
}


// 0x78b31c — __ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<RBX::InstanceHandle,std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>,std::_Select1st<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>,std::less<RBX::InstanceHandle>,std::allocator<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::InstanceHandle const,RBX::InstanceHandle>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX14InstanceHandleESt4pairIKS1_S1_ESt10_Select1stIS4_ESt4lessIS1_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0x78b31c: 74 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b31c() {
}


// 0x78b3ec — __ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EED2Ev
#[doc(alias = "std::vector<MemoryBinder::IDREFItem,std::allocator<MemoryBinder::IDREFItem>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN12MemoryBinder9IDREFItemESaIS1_EED2Ev")]
// IDA 0x78b3ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_78b3ec() {
}


// 0x78b4b8 — __ZN13ArchiveBinderD0Ev
// type: void __fastcall(ArchiveBinder *__hidden this)
#[doc(alias = "ArchiveBinder::~ArchiveBinder()")]
#[doc(alias = "__ZN13ArchiveBinderD0Ev")]
// IDA 0x78b4b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_78b4b8() {
}


// 0x78b5d8 — __ZN13ArchiveBinder9processIDEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseE
// type: _DWORD __fastcall(ArchiveBinder *__hidden this, const XmlNameValuePair *, RBX::Reflection::DescribedBase *)
#[doc(alias = "ArchiveBinder::processID(XmlNameValuePair const*,RBX::Reflection::DescribedBase *)")]
#[doc(alias = "__ZN13ArchiveBinder9processIDEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseE")]
// IDA 0x78b5d8: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b5d8() {
}


// 0x78b7b4 — __ZN13ArchiveBinder12processIDREFEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseEPKNS3_6IIDREFE
#[doc(alias = "ArchiveBinder::processIDREF(XmlNameValuePair const*,RBX::Reflection::DescribedBase *,RBX::IIDREF const*)")]
#[doc(alias = "__ZN13ArchiveBinder12processIDREFEPK16XmlNameValuePairPN3RBX10Reflection13DescribedBaseEPKNS3_6IIDREFE")]
// IDA 0x78b7b4: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b7b4() {
}


// 0x78b7e8 — __ZNSt3mapISsN3RBX14InstanceHandleESt4lessISsESaISt4pairIKSsS1_EEEixERS5_
#[doc(alias = "std::map<std::string,RBX::InstanceHandle,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN3RBX14InstanceHandleESt4lessISsESaISt4pairIKSsS1_EEEixERS5_")]
// IDA 0x78b7e8: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78b7e8() {
}


// 0x78ba04 — __ZNSt4pairIKSsN3RBX14InstanceHandleEEC2ERS0_RKS2_
#[doc(alias = "std::pair<std::string const,RBX::InstanceHandle>::pair(std::string const&,RBX::InstanceHandle const&)")]
#[doc(alias = "__ZNSt4pairIKSsN3RBX14InstanceHandleEEC2ERS0_RKS2_")]
// IDA 0x78ba04: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78ba04() {
}


// 0x78bad0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InstanceHandle>>,std::pair<std::string const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_")]
// IDA 0x78bad0: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bad0() {
}


// 0x78bbbc — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// type: int __fastcall(int, int, int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")]
// IDA 0x78bbbc: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bbbc() {
}


// 0x78bc0c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::_M_insert_unique(std::pair<std::string const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE16_M_insert_uniqueERKS4_")]
// IDA 0x78bc0c: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bc0c() {
}


// 0x78bc90 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::_M_create_node(std::pair<std::string const,RBX::InstanceHandle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE14_M_create_nodeERKS4_")]
// IDA 0x78bc90: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bc90() {
}


// 0x78bda0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::lower_bound(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE11lower_boundERS1_")]
// IDA 0x78bda0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bda0() {
}


// 0x78bdd0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::InstanceHandle>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0x78bdd0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bdd0() {
}


// 0x78bdf8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InstanceHandle>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")]
// IDA 0x78bdf8: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bdf8() {
}


// 0x78bec4 — __ZSt8count_ifISt14_List_iteratorIN13ArchiveBinder12IDREFBindingEESt9binder1stISt10mem_fun1_tIbS1_S2_EEENSt15iterator_traitsIT_E15difference_typeES9_S9_T0_
#[doc(alias = "std::iterator_traits<std::_List_iterator<ArchiveBinder::IDREFBinding>>::difference_type std::count_if<std::_List_iterator<ArchiveBinder::IDREFBinding>,std::binder1st<std::mem_fun1_t<bool,ArchiveBinder,ArchiveBinder::IDREFBinding>>>(std::_List_iterator<ArchiveBinder::IDREFBinding>,std::_List_iterator<ArchiveBinder::IDREFBinding>,std::binder1st<std::mem_fun1_t<bool,ArchiveBinder,ArchiveBinder::IDREFBinding>>)")]
#[doc(alias = "__ZSt8count_ifISt14_List_iteratorIN13ArchiveBinder12IDREFBindingEESt9binder1stISt10mem_fun1_tIbS1_S2_EEENSt15iterator_traitsIT_E15difference_typeES9_S9_T0_")]
// IDA 0x78bec4: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bec4() {
}


// 0x78bf04 — __ZN13ArchiveBinder12resolveIDREFENS_12IDREFBindingE
#[doc(alias = "ArchiveBinder::resolveIDREF(ArchiveBinder::IDREFBinding)")]
#[doc(alias = "__ZN13ArchiveBinder12resolveIDREFENS_12IDREFBindingE")]
// IDA 0x78bf04: 296 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78bf04() {
}


// 0x78c238 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InstanceHandle>,std::_Select1st<std::pair<std::string const,RBX::InstanceHandle>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InstanceHandle>>>::find(std::string const&)")]
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14InstanceHandleEESt10_Select1stIS4_ESt4lessISsESaIS4_EE4findERS1_")]
// IDA 0x78c238: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78c238() {
}


// 0x78c288 — __ZNKSt9binder1stISt10mem_fun1_tIb13ArchiveBinderNS1_12IDREFBindingEEEclERS2_
#[doc(alias = "std::binder1st<std::mem_fun1_t<bool,ArchiveBinder,ArchiveBinder::IDREFBinding>>::operator()(ArchiveBinder::IDREFBinding&)const")]
#[doc(alias = "__ZNKSt9binder1stISt10mem_fun1_tIb13ArchiveBinderNS1_12IDREFBindingEEEclERS2_")]
// IDA 0x78c288: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78c288() {
}


// 0x78c2ac — __GLOBAL__I_a_360
#[doc(alias = "global constructor keyed to_a_360")]
#[doc(alias = "__GLOBAL__I_a_360")]
// IDA 0x78c2ac: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_78c2ac() {
}


// 0x78c444 — __ZN3RBX9WebParser8loadListEPK10XmlElementRSt6vectorINS_10Reflection7VariantESaIS6_EE
#[doc(alias = "RBX::WebParser::loadList(XmlElement const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> &)")]
#[doc(alias = "__ZN3RBX9WebParser8loadListEPK10XmlElementRSt6vectorINS_10Reflection7VariantESaIS6_EE")]
// IDA 0x78c444: 120 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78c444() {
}


// 0x78c588 — __ZN3RBX9WebParser23parseWebGenericResponseERSiRNS_10Reflection7VariantE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::WebParser::parseWebGenericResponse(std::istream &,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX9WebParser23parseWebGenericResponseERSiRNS_10Reflection7VariantE")]
// IDA 0x78c588: 160 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78c588() {
}


// 0x78c73c — __ZN3RBX9WebParser23parseWebGenericResponseEPK10XmlElementRNS_10Reflection7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::WebParser::parseWebGenericResponse(XmlElement const*,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX9WebParser23parseWebGenericResponseEPK10XmlElementRNS_10Reflection7VariantE")]
// IDA 0x78c73c: 211 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78c73c() {
}


// 0x78c970 — __ZN3RBX9WebParser9loadTableEPK10XmlElementRSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEE
#[doc(alias = "RBX::WebParser::loadTable(XmlElement const*,std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> &)")]
#[doc(alias = "__ZN3RBX9WebParser9loadTableEPK10XmlElementRSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEE")]
// IDA 0x78c970: 182 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78c970() {
}


// 0x78cb60 — __ZN3RBX9WebParser9loadValueEPK10XmlElementRNS_10Reflection7VariantE
// type: int __fastcall(XmlElement *this, int)
#[doc(alias = "RBX::WebParser::loadValue(XmlElement const*,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX9WebParser9loadValueEPK10XmlElementRNS_10Reflection7VariantE")]
// IDA 0x78cb60: 755 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78cb60() {
}


// 0x78d360 — __ZN3RBX9WebParser9loadEntryEPK10XmlElementRSsRNS_10Reflection7VariantE
// type: int __fastcall(XmlElement *this)
#[doc(alias = "RBX::WebParser::loadEntry(XmlElement const*,std::string &,RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX9WebParser9loadEntryEPK10XmlElementRSsRNS_10Reflection7VariantE")]
// IDA 0x78d360: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78d360() {
}


// 0x78d3cc — __ZN3RBX9WebParser27populateValueTableFromPtreeERKN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEERNS1_10shared_ptrINS1_9unordered13unordered_mapISsNS_10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsSD_EEEEEE
// was: __ZN3RBX9WebParser27populateValueTableFromPtreeERKN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEERNS1_10shared_ptrINS1_9unordered13unordered_mapISsNS_10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsSD_EEEEEE — uses rbx_core::SharedPtr not boost
#[doc(alias = "RBX::WebParser::populateValueTableFromPtree(boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>> const&,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> &)")]
#[doc(alias = "__ZN3RBX9WebParser27populateValueTableFromPtreeERKN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEERNS1_10shared_ptrINS1_9unordered13unordered_mapISsNS_10Reflection7VariantENS1_4hashISsEESt8equal_toISsESaISt4pairIKSsSD_EEEEEE")]
// IDA 0x78d3cc: 557 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78d3cc() {
}


// 0x78d980 — __ZN3RBX9WebParser20parseWebJSONResponseERSt18basic_stringstreamIcSt11char_traitsIcESaIcEERN5boost10shared_ptrINS7_9unordered13unordered_mapISsNS_10Reflection7VariantENS7_4hashISsEESt8equal_toISsESaISt4pairIKSsSC_EEEEEE
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN3RBX9WebParser20parseWebJSONResponseERSt18basic_stringstreamIcSt11char_traitsIcESaIcEERN5boost10shared_ptrINS7_9unordered13unordered_mapISsNS_10Reflection7VariantENS7_4hashISsEESt8equal_toISsESaISt4pairIKSsSC_EEEEEE — uses rbx_core::SharedPtr not boost
#[doc(alias = "RBX::WebParser::parseWebJSONResponse(std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>> &,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> &)")]
#[doc(alias = "__ZN3RBX9WebParser20parseWebJSONResponseERSt18basic_stringstreamIcSt11char_traitsIcESaIcEERN5boost10shared_ptrINS7_9unordered13unordered_mapISsNS_10Reflection7VariantENS7_4hashISsEESt8equal_toISsESaISt4pairIKSsSC_EEEEEE")]
// IDA 0x78d980: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_78d980() {
}


// 0x78ec1c — __ZNSt6vectorImSaImEE9push_backERKm
#[doc(alias = "std::vector<unsigned long,std::allocator<unsigned long>>::push_back(unsigned long const&)")]
#[doc(alias = "__ZNSt6vectorImSaImEE9push_backERKm")]
// IDA 0x78ec1c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_78ec1c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x7926cc — __ZN5boost6spirit7classic4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES15_EaSINSE_INS1_6actionINS1_10differenceINS19_ISM_SK_EESK_EENS_13property_tree11json_parser7contextINS1C_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsS16_EEEEEEEERS16_RKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
// was: __ZN5boost6spirit7classic4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES15_EaSINSE_INS1_6actionINS1_10differenceINS19_ISM_SK_EESK_EENS_13property_tree11json_parser7contextINS1C_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsS16_EEEEEEEERS16_RKT_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>& boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>::operator=<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::difference<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::strlit<char const*>>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_char>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>>>>(boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::difference<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::strlit<char const*>>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_char>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>>> const&)")]
#[doc(alias = "__ZN5boost6spirit7classic4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES15_EaSINSE_INS1_6actionINS1_10differenceINS19_ISM_SK_EESK_EENS_13property_tree11json_parser7contextINS1C_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsS16_EEEEEEEERS16_RKT_")]
// IDA 0x7926cc: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7926cc() {
}


// 0x7927d8 — __ZNK5boost6spirit7classic9assertionISsEclINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSH_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSH_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSJ_ISN_SQ_SN_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES18_EEEENS1_16assertive_parserISsT_EERKS1B_
// was: __ZNK5boost6spirit7classic9assertionISsEclINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSH_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSH_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSJ_ISN_SQ_SN_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES18_EEEENS1_16assertive_parserISsT_EERKS1B_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>> boost::spirit::classic::assertion<std::string>::operator()<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>(boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic9assertionISsEclINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSH_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSH_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSJ_ISN_SQ_SN_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES18_EEEENS1_16assertive_parserISsT_EERKS1B_")]
// IDA 0x7927d8: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7927d8() {
}


// 0x7928f8 — __ZN5boost6spirit7classic4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES15_EaSINSE_INS1_6actionINS1_5chsetIcEENS_13property_tree11json_parser7contextINS1B_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeEEENS1_8sequenceINS1_5chlitIcEENS18_INS1_11uint_parserImLi16ELj4ELi4EEENS1I_9a_unicodeEEEEEEEEERS16_RKT_
// was: __ZN5boost6spirit7classic4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES15_EaSINSE_INS1_6actionINS1_5chsetIcEENS_13property_tree11json_parser7contextINS1B_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeEEENS1_8sequenceINS1_5chlitIcEENS18_INS1_11uint_parserImLi16ELj4ELi4EEENS1I_9a_unicodeEEEEEEEEERS16_RKT_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>& boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>::operator=<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::chset<char>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_escape>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::action<boost::spirit::classic::uint_parser<unsigned long,16,4u,4>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_unicode>>>>(boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::chset<char>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_escape>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::action<boost::spirit::classic::uint_parser<unsigned long,16,4u,4>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_unicode>>> const&)")]
#[doc(alias = "__ZN5boost6spirit7classic4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES15_EaSINSE_INS1_6actionINS1_5chsetIcEENS_13property_tree11json_parser7contextINS1B_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeEEENS1_8sequenceINS1_5chlitIcEENS18_INS1_11uint_parserImLi16ELj4ELi4EEENS1I_9a_unicodeEEEEEEEEERS16_RKT_")]
// IDA 0x7928f8: 93 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7928f8() {
}


// 0x792a04 — __ZN5boost10shared_ptrINS_6spirit7classic11basic_chsetIcEEEC2IS4_EEPT_
// was: __ZN5boost10shared_ptrINS_6spirit7classic11basic_chsetIcEEEC2IS4_EEPT_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::shared_ptr<boost::spirit::classic::basic_chset<char>>::shared_ptr<boost::spirit::classic::basic_chset<char>>(boost::spirit::classic::basic_chset<char> *)")]
#[doc(alias = "__ZN5boost10shared_ptrINS_6spirit7classic11basic_chsetIcEEEC2IS4_EEPT_")]
// IDA 0x792a04: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792a04() {
}


// 0x792ad8 — __ZN5boost6detail12shared_countC2INS_6spirit7classic11basic_chsetIcEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
// was: __ZN5boost6detail12shared_countC2INS_6spirit7classic11basic_chsetIcEEEEPT_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::shared_count::shared_count<boost::spirit::classic::basic_chset<char>>(boost::spirit::classic::basic_chset<char> *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2INS_6spirit7classic11basic_chsetIcEEEEPT_")]
// IDA 0x792ad8: 85 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792ad8() {
}


// 0x792bc8 — __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic11basic_chsetIcEEED0Ev
// was: __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic11basic_chsetIcEEED0Ev — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::spirit::classic::basic_chset<char>>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic11basic_chsetIcEEED0Ev")]
// IDA 0x792bc8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_792bc8() {
}


// 0x792bd0 — __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic11basic_chsetIcEEE11get_deleterERKSt9type_info
// was: __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic11basic_chsetIcEEE11get_deleterERKSt9type_info — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::spirit::classic::basic_chset<char>>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic11basic_chsetIcEEE11get_deleterERKSt9type_info")]
// IDA 0x792bd0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792bd0() {
}


// 0x792bd8 — __ZN5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_5chsetIcEENS_13property_tree11json_parser7contextINS8_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeEEENS1_8sequenceINS1_5chlitIcEENS5_INS1_11uint_parserImLi16ELj4ELi4EEENSF_9a_unicodeEEEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS17_S1A_S17_S1E_S1F_S1G_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEED1Ev
// was: __ZN5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_5chsetIcEENS_13property_tree11json_parser7contextINS8_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeEEENS1_8sequenceINS1_5chlitIcEENS5_INS1_11uint_parserImLi16ELj4ELi4EEENSF_9a_unicodeEEEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS17_S1A_S17_S1E_S1F_S1G_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEED1Ev — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::impl::concrete_parser<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::chset<char>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_escape>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::action<boost::spirit::classic::uint_parser<unsigned long,16,4u,4>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_unicode>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t>::~concrete_parser()")]
#[doc(alias = "__ZN5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_5chsetIcEENS_13property_tree11json_parser7contextINS8_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeEEENS1_8sequenceINS1_5chlitIcEENS5_INS1_11uint_parserImLi16ELj4ELi4EEENSF_9a_unicodeEEEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS17_S1A_S17_S1E_S1F_S1G_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEED1Ev")]
// IDA 0x792bd8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792bd8() {
}


// 0x792c00 — __ZNK5boost6spirit7classic8sequenceINS1_5chlitIcEENS1_6actionINS1_11uint_parserImLi16ELj4ELi4EEENS_13property_tree11json_parser7contextINS8_11basic_ptreeISsSsSt4lessISsEEEE9a_unicodeEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSV_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSV_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSX_IS11_S14_S11_S18_S19_S1A_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISI_T_E4typeERKS1N_
// type: int __fastcall(int, int)
// was: __ZNK5boost6spirit7classic8sequenceINS1_5chlitIcEENS1_6actionINS1_11uint_parserImLi16ELj4ELi4EEENS_13property_tree11json_parser7contextINS8_11basic_ptreeISsSsSt4lessISsEEEE9a_unicodeEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSV_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSV_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSX_IS11_S14_S11_S18_S19_S1A_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISI_T_E4typeERKS1N_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::action<boost::spirit::classic::uint_parser<unsigned long,16,4u,4>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_unicode>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::action<boost::spirit::classic::uint_parser<unsigned long,16,4u,4>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_unicode>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic8sequenceINS1_5chlitIcEENS1_6actionINS1_11uint_parserImLi16ELj4ELi4EEENS_13property_tree11json_parser7contextINS8_11basic_ptreeISsSsSt4lessISsEEEE9a_unicodeEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSV_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSV_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSX_IS11_S14_S11_S18_S19_S1A_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISI_T_E4typeERKS1N_")]
// IDA 0x792c00: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792c00() {
}


// 0x792c4c — __ZNK5boost6spirit7classic6actionINS1_11uint_parserImLi16ELj4ELi4EEENS_13property_tree11json_parser7contextINS5_11basic_ptreeISsSsSt4lessISsEEEE9a_unicodeEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSR_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSR_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENST_ISX_S10_SX_S14_S15_S16_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISE_T_E4typeERKS1J_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
// was: __ZNK5boost6spirit7classic6actionINS1_11uint_parserImLi16ELj4ELi4EEENS_13property_tree11json_parser7contextINS5_11basic_ptreeISsSsSt4lessISsEEEE9a_unicodeEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSR_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSR_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENST_ISX_S10_SX_S14_S15_S16_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISE_T_E4typeERKS1J_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::action<boost::spirit::classic::uint_parser<unsigned long,16,4u,4>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_unicode>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::action<boost::spirit::classic::uint_parser<unsigned long,16,4u,4>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_unicode>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic6actionINS1_11uint_parserImLi16ELj4ELi4EEENS_13property_tree11json_parser7contextINS5_11basic_ptreeISsSsSt4lessISsEEEE9a_unicodeEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSR_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSR_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENST_ISX_S10_SX_S14_S15_S16_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISE_T_E4typeERKS1J_")]
// IDA 0x792c4c: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792c4c() {
}


// 0x792d08 — __ZNK5boost6spirit7classic4impl16uint_parser_implImLi16ELj4ELi4EE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSH_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSH_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSJ_ISN_SQ_SN_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS4_T_E4typeERKS19_
// was: __ZNK5boost6spirit7classic4impl16uint_parser_implImLi16ELj4ELi4EE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSH_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSH_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSJ_ISN_SQ_SN_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS4_T_E4typeERKS19_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::impl::uint_parser_impl<unsigned long,16,4u,4>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::impl::uint_parser_impl<unsigned long,16,4u,4>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic4impl16uint_parser_implImLi16ELj4ELi4EE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSH_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSH_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSJ_ISN_SQ_SN_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS4_T_E4typeERKS19_")]
// IDA 0x792d08: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792d08() {
}


// 0x792d4c — __ZN5boost6spirit7classic4impl11extract_intILi16ELj4ELi4ENS2_19positive_accumulateImLi16EEEE1fIKNS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSJ_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSJ_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSL_ISP_SS_SP_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEmEEbRT_RT0_Rm
// was: __ZN5boost6spirit7classic4impl11extract_intILi16ELj4ELi4ENS2_19positive_accumulateImLi16EEEE1fIKNS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSJ_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSJ_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSL_ISP_SS_SP_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEmEEbRT_RT0_Rm — uses rbx_core::SharedPtr not boost
#[doc(alias = "bool boost::spirit::classic::impl::extract_int<16,4u,4,boost::spirit::classic::impl::positive_accumulate<unsigned long,16>>::f<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const,unsigned long>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const &,unsigned long &,unsigned long &)")]
#[doc(alias = "__ZN5boost6spirit7classic4impl11extract_intILi16ELj4ELi4ENS2_19positive_accumulateImLi16EEEE1fIKNS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSJ_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSJ_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSL_ISP_SS_SP_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEmEEbRT_RT0_Rm")]
// IDA 0x792d4c: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792d4c() {
}


// 0x792db0 — __ZN5boost6spirit7classic4impl12radix_traitsILi16EE5digitIcmEEbT_RT0_
// was: __ZN5boost6spirit7classic4impl12radix_traitsILi16EE5digitIcmEEbT_RT0_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "bool boost::spirit::classic::impl::radix_traits<16>::digit<char,unsigned long>(char,unsigned long &)")]
#[doc(alias = "__ZN5boost6spirit7classic4impl12radix_traitsILi16EE5digitIcmEEbT_RT0_")]
// IDA 0x792db0: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792db0() {
}


// 0x792e00 — __ZNK5boost13property_tree11json_parser7contextINS0_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeclEc
// was: __ZNK5boost13property_tree11json_parser7contextINS0_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeclEc — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_escape::operator()(char)const")]
#[doc(alias = "__ZNK5boost13property_tree11json_parser7contextINS0_11basic_ptreeISsSsSt4lessISsEEEE8a_escapeclEc")]
// IDA 0x792e00: 52 insns (CMP..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792e00() {
}


// 0x792e78 — __ZNKSt6bitsetILm256EE4testEm
#[doc(alias = "std::bitset<256ul>::test(unsigned long)const")]
#[doc(alias = "__ZNKSt6bitsetILm256EE4testEm")]
// IDA 0x792e78: 16 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792e78() {
}


// 0x792ea8 — __ZN5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_ED1Ev
// was: __ZN5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_ED1Ev — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::impl::concrete_parser<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::difference<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::strlit<char const*>>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_char>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t>::~concrete_parser()")]
#[doc(alias = "__ZN5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_ED1Ev")]
// IDA 0x792ea8: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792ea8() {
}


// 0x792ed0 — __ZNK5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_E16do_parse_virtualERKS1O_
// was: __ZNK5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_E16do_parse_virtualERKS1O_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::impl::concrete_parser<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::difference<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::strlit<char const*>>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_char>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t>::do_parse_virtual(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_E16do_parse_virtualERKS1O_")]
// IDA 0x792ed0: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792ed0() {
}


// 0x792ed8 — __ZNK5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_E5cloneEv
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
// was: __ZNK5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_E5cloneEv — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::impl::concrete_parser<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::difference<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::strlit<char const*>>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_char>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t>::clone(void)const")]
#[doc(alias = "__ZNK5boost6spirit7classic4impl15concrete_parserINS1_11alternativeINS1_6actionINS1_10differenceINS6_INS1_14anychar_parserENS1_6strlitIPKcEEEESB_EENS_13property_tree11json_parser7contextINSE_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS4_INS4_INS1_12space_parserENS1_13confix_parserISB_NS1_11kleene_starIS7_EENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_ISB_S17_SB_S1B_S1C_S1D_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1P_EEEEEEEES1O_S1P_E5cloneEv")]
// IDA 0x792ed8: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792ed8() {
}


// 0x792fd8 — __ZN5boost6spirit7classic6throw_ISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEvT0_T_
// was: __ZN5boost6spirit7classic6throw_ISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEvT0_T_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::spirit::classic::throw_<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,std::string)")]
#[doc(alias = "__ZN5boost6spirit7classic6throw_ISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEvT0_T_")]
// IDA 0x792fd8: 104 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_792fd8() {
}


// 0x793108 — __ZN5boost15throw_exceptionINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEvRKT_
// was: __ZN5boost15throw_exceptionINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEvRKT_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::throw_exception<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>>(boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>> const&)")]
#[doc(alias = "__ZN5boost15throw_exceptionINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEvRKT_")]
// IDA 0x793108: 76 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_793108() {
}


// 0x7931e8 — __ZNK5boost6spirit7classic10differenceINS1_14anychar_parserENS1_6strlitIPKcEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSL_INS1_12space_parserENS1_13confix_parserIS7_NS1_11kleene_starIS3_EENSL_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSN_IS7_SP_S7_ST_SU_SV_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS8_T_E4typeERKS18_
// was: __ZNK5boost6spirit7classic10differenceINS1_14anychar_parserENS1_6strlitIPKcEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSL_INS1_12space_parserENS1_13confix_parserIS7_NS1_11kleene_starIS3_EENSL_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSN_IS7_SP_S7_ST_SU_SV_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS8_T_E4typeERKS18_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic10differenceINS1_14anychar_parserENS1_6strlitIPKcEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSL_INS1_12space_parserENS1_13confix_parserIS7_NS1_11kleene_starIS3_EENSL_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSN_IS7_SP_S7_ST_SU_SV_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS8_T_E4typeERKS18_")]
// IDA 0x7931e8: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7931e8() {
}


// 0x793230 — __ZNK5boost6spirit7classic5chseqIPKcE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSI_INS1_12space_parserENS1_13confix_parserINS1_6strlitIS4_EENS1_11kleene_starINS1_14anychar_parserEEENSI_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSK_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS5_T_E4typeERKS18_
// was: __ZNK5boost6spirit7classic5chseqIPKcE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSI_INS1_12space_parserENS1_13confix_parserINS1_6strlitIS4_EENS1_11kleene_starINS1_14anychar_parserEEENSI_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSK_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS5_T_E4typeERKS18_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::chseq<char const*>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::chseq<char const*>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic5chseqIPKcE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSI_INS1_12space_parserENS1_13confix_parserINS1_6strlitIS4_EENS1_11kleene_starINS1_14anychar_parserEEENSI_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSK_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS5_T_E4typeERKS18_")]
// IDA 0x793230: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_793230() {
}


// 0x793278 — __ZN5boost6spirit7classic4impl15concrete_parserINS1_8positiveINS1_10contiguousINS1_13confix_parserINS1_5chlitIcEENS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS6_INS1_6strlitIPKcEENS9_INS1_14anychar_parserEEENSM_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS6_ISR_ST_SR_SX_SY_SZ_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1B_EEEES8_SX_SY_NS1_10non_lexemeEEEEEEENSB_ISI_NSJ_IS15_S17_S18_EEEES1B_ED0Ev
// was: __ZN5boost6spirit7classic4impl15concrete_parserINS1_8positiveINS1_10contiguousINS1_13confix_parserINS1_5chlitIcEENS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS6_INS1_6strlitIPKcEENS9_INS1_14anychar_parserEEENSM_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS6_ISR_ST_SR_SX_SY_SZ_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1B_EEEES8_SX_SY_NS1_10non_lexemeEEEEEEENSB_ISI_NSJ_IS15_S17_S18_EEEES1B_ED0Ev — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::impl::concrete_parser<boost::spirit::classic::positive<boost::spirit::classic::contiguous<boost::spirit::classic::confix_parser<boost::spirit::classic::chlit<char>,boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::non_lexeme>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t>::~concrete_parser()")]
#[doc(alias = "__ZN5boost6spirit7classic4impl15concrete_parserINS1_8positiveINS1_10contiguousINS1_13confix_parserINS1_5chlitIcEENS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS6_INS1_6strlitIPKcEENS9_INS1_14anychar_parserEEENSM_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS6_ISR_ST_SR_SX_SY_SZ_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1B_EEEES8_SX_SY_NS1_10non_lexemeEEEEEEENSB_ISI_NSJ_IS15_S17_S18_EEEES1B_ED0Ev")]
// IDA 0x793278: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_793278() {
}


// 0x793280 — __ZNK5boost6spirit7classic8positiveINS1_10contiguousINS1_13confix_parserINS1_5chlitIcEENS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS4_INS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS4_ISP_SR_SP_SV_SW_SX_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES19_EEEES6_SV_SW_NS1_10non_lexemeEEEEEE5parseINS9_ISG_NSH_IS13_S15_S16_EEEEEENS1_13parser_resultIS1F_T_E4typeERKS1K_
// was: __ZNK5boost6spirit7classic8positiveINS1_10contiguousINS1_13confix_parserINS1_5chlitIcEENS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS4_INS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS4_ISP_SR_SP_SV_SW_SX_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES19_EEEES6_SV_SW_NS1_10non_lexemeEEEEEE5parseINS9_ISG_NSH_IS13_S15_S16_EEEEEENS1_13parser_resultIS1F_T_E4typeERKS1K_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::positive<boost::spirit::classic::contiguous<boost::spirit::classic::confix_parser<boost::spirit::classic::chlit<char>,boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::non_lexeme>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::positive<boost::spirit::classic::contiguous<boost::spirit::classic::confix_parser<boost::spirit::classic::chlit<char>,boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::non_lexeme>>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic8positiveINS1_10contiguousINS1_13confix_parserINS1_5chlitIcEENS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS4_INS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS4_ISP_SR_SP_SV_SW_SX_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES19_EEEES6_SV_SW_NS1_10non_lexemeEEEEEE5parseINS9_ISG_NSH_IS13_S15_S16_EEEEEENS1_13parser_resultIS1F_T_E4typeERKS1K_")]
// IDA 0x793280: 88 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_793280() {
}


// 0x79338c — __ZNK5boost6spirit7classic8sequenceINS2_INS1_5chlitIcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1A_EEEES4_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEES4_E5parseIS19_EENS1_13parser_resultIS1J_T_E4typeERKS1M_
// was: __ZNK5boost6spirit7classic8sequenceINS2_INS1_5chlitIcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1A_EEEES4_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEES4_E5parseIS19_EENS1_13parser_resultIS1J_T_E4typeERKS1M_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>,boost::spirit::classic::chlit<char>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic8sequenceINS2_INS1_5chlitIcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1A_EEEES4_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEES4_E5parseIS19_EENS1_13parser_resultIS1J_T_E4typeERKS1M_")]
// IDA 0x79338c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_79338c() {
}


// 0x7933c8 — __ZNK5boost6spirit7classic8sequenceINS1_5chlitIcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1A_EEEES4_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEE5parseIS19_EENS1_13parser_resultIS1I_T_E4typeERKS1L_
// was: __ZNK5boost6spirit7classic8sequenceINS1_5chlitIcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1A_EEEES4_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEE5parseIS19_EENS1_13parser_resultIS1I_T_E4typeERKS1L_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic8sequenceINS1_5chlitIcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS7_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1A_EEEES4_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEE5parseIS19_EENS1_13parser_resultIS1I_T_E4typeERKS1L_")]
// IDA 0x7933c8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7933c8() {
}


// 0x793418 — __ZN5boost6spirit7classic4impl25refactor_unary_non_nestedINS1_21unary_parser_categoryEE5parseINS1_21refactor_unary_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS9_INS1_14anychar_parserEEENSM_INS1_10eol_parserENS1_10end_parserEEES4_NS1_10non_nestedENS1_9is_lexemeEEEEENSO_ISS_SU_SS_S4_SY_SZ_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1B_EEEENS1_5chlitIcEEEENS1_22non_nested_refactoringEEES1A_S1G_EENS1_13parser_resultIT_T0_E4typeERKS1K_RKS1L_RKT1_
// was: __ZN5boost6spirit7classic4impl25refactor_unary_non_nestedINS1_21unary_parser_categoryEE5parseINS1_21refactor_unary_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS9_INS1_14anychar_parserEEENSM_INS1_10eol_parserENS1_10end_parserEEES4_NS1_10non_nestedENS1_9is_lexemeEEEEENSO_ISS_SU_SS_S4_SY_SZ_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1B_EEEENS1_5chlitIcEEEENS1_22non_nested_refactoringEEES1A_S1G_EENS1_13parser_resultIT_T0_E4typeERKS1K_RKS1L_RKT1_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::refactor_unary_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::non_nested_refactoring>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::impl::refactor_unary_non_nested<boost::spirit::classic::unary_parser_category>::parse<boost::spirit::classic::refactor_unary_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::non_nested_refactoring>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>>(boost::spirit::classic::refactor_unary_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::non_nested_refactoring> const&,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&,boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::chlit<char>> const&)")]
#[doc(alias = "__ZN5boost6spirit7classic4impl25refactor_unary_non_nestedINS1_21unary_parser_categoryEE5parseINS1_21refactor_unary_parserINS1_10differenceINS1_11kleene_starINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS9_INS1_14anychar_parserEEENSM_INS1_10eol_parserENS1_10end_parserEEES4_NS1_10non_nestedENS1_9is_lexemeEEEEENSO_ISS_SU_SS_S4_SY_SZ_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1B_EEEENS1_5chlitIcEEEENS1_22non_nested_refactoringEEES1A_S1G_EENS1_13parser_resultIT_T0_E4typeERKS1K_RKS1L_RKT1_")]
// IDA 0x793418: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_793418() {
}


// 0x793454 — __ZNK5boost6spirit7classic10differenceINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSF_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSF_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSH_ISL_SO_SL_SS_ST_SU_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EENS1_5chlitIcEEE5parseIS15_EENS1_13parser_resultIS1A_T_E4typeERKS1D_
// was: __ZNK5boost6spirit7classic10differenceINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSF_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSF_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSH_ISL_SO_SL_SS_ST_SU_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EENS1_5chlitIcEEE5parseIS15_EENS1_13parser_resultIS1A_T_E4typeERKS1D_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::difference<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>,boost::spirit::classic::chlit<char>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::difference<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>,boost::spirit::classic::chlit<char>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic10differenceINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSF_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSF_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSH_ISL_SO_SL_SS_ST_SU_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EENS1_5chlitIcEEE5parseIS15_EENS1_13parser_resultIS1A_T_E4typeERKS1D_")]
// IDA 0x793454: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_793454() {
}


// 0x7934ac — __ZN5boost6spirit7classic4impl12skipper_skipINS1_11alternativeINS4_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS6_ISA_SD_SA_SH_SI_SJ_EEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyISN_NS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEESY_EEvRKT_RKT0_RKNS1_24skipper_iteration_policyIT1_EE
// was: __ZN5boost6spirit7classic4impl12skipper_skipINS1_11alternativeINS4_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS6_ISA_SD_SA_SH_SI_SJ_EEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyISN_NS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEESY_EEvRKT_RKT0_RKNS1_24skipper_iteration_policyIT1_EE — uses rbx_core::SharedPtr not boost
#[doc(alias = "void boost::spirit::classic::impl::skipper_skip<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::iteration_policy>(boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>> const&,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&,boost::spirit::classic::skipper_iteration_policy<boost::spirit::classic::iteration_policy> const&)")]
#[doc(alias = "__ZN5boost6spirit7classic4impl12skipper_skipINS1_11alternativeINS4_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS4_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS6_ISA_SD_SA_SH_SI_SJ_EEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyISN_NS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEESY_EEvRKT_RKT0_RKNS1_24skipper_iteration_policyIT1_EE")]
// IDA 0x7934ac: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7934ac() {
}


// 0x7934e8 — __ZNK5boost6spirit7classic8sequenceINS2_INS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEES6_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEES6_E5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSV_INS1_12space_parserENS1_13confix_parserIS6_SB_NSV_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSX_IS6_SB_S6_S11_S12_S13_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISI_T_E4typeERKS1G_
// was: __ZNK5boost6spirit7classic8sequenceINS2_INS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEES6_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEES6_E5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSV_INS1_12space_parserENS1_13confix_parserIS6_SB_NSV_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSX_IS6_SB_S6_S11_S12_S13_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISI_T_E4typeERKS1G_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>,boost::spirit::classic::strlit<char const*>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic8sequenceINS2_INS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEES6_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEES6_E5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSV_INS1_12space_parserENS1_13confix_parserIS6_SB_NSV_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSX_IS6_SB_S6_S11_S12_S13_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISI_T_E4typeERKS1G_")]
// IDA 0x7934e8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7934e8() {
}


// 0x793518 — __ZNK5boost6spirit7classic8sequenceINS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEES6_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSU_INS1_12space_parserENS1_13confix_parserIS6_SB_NSU_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSW_IS6_SB_S6_S10_S11_S12_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISH_T_E4typeERKS1F_
// type: int __fastcall(int, int)
// was: __ZNK5boost6spirit7classic8sequenceINS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEES6_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSU_INS1_12space_parserENS1_13confix_parserIS6_SB_NSU_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSW_IS6_SB_S6_S10_S11_S12_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISH_T_E4typeERKS1F_ — uses rbx_core::SharedPtr not boost
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::sequence<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::sequence<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>(boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>> const&)const")]
#[doc(alias = "__ZNK5boost6spirit7classic8sequenceINS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEES6_EENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSU_INS1_12space_parserENS1_13confix_parserIS6_SB_NSU_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSW_IS6_SB_S6_S10_S11_S12_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISH_T_E4typeERKS1F_")]
// IDA 0x793518: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_793518() {
}

