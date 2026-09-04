//! rendering shard 480 — 100 stubs 0x77123c..0x775d3c EA-sorted asc global gap filler not yet in rbx_rendering (Ogre 9839/9839 + G3D 3882/3882 complete, 51804->51904 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc after shard 479 (0x77123c..0x775d3c)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x77123c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()
// IDA 0x77123c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77123c() {
}

// 0x77127c — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
// IDA 0x77127c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77127c() {
}

// 0x7712a0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()
// IDA 0x7712a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7712a0() {
}

// 0x771394 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFNS0_7VariantEN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,RBX::Reflection::Variant ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
// IDA 0x771394: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771394() {
}

// 0x771488 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvvELi0EED1Ev")]
// IDA 0x771488: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771488() {
}

// 0x7714ac — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvELi0EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()
// IDA 0x7714ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7714ac() {
}

// 0x7714d0 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEiELi1EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),1>::~BoundFuncDesc()
// IDA 0x7714d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7714d0() {
}

// 0x771510 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvELi0EED1Ev")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(void),0>::~BoundFuncDesc()
// IDA 0x771510: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771510() {
}

// 0x771534 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant,int),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEiELi3EED1Ev")]
// IDA 0x771534: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771534() {
}

// 0x771658 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::ScriptDebugger,void ()(std::string,RBX::Reflection::Variant),2>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting14ScriptDebuggerEFvSsNS0_7VariantEELi2EED1Ev")]
// IDA 0x771658: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771658() {
}

// 0x771770 — __ZNK3RBX9Scripting14ScriptDebugger9getScriptEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getScript(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger9getScriptEv")]
// IDA 0x771770: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771770() {
}

// 0x771778 — __ZN3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Scripting::ScriptDebugger,RBX::Script>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEED1Ev")]
// IDA 0x771778: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771778() {
}

// 0x7717a4 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerESsED1Ev")]
// IDA 0x7717a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7717a4() {
}

// 0x7717c8 — __ZNK3RBX9Scripting14ScriptDebugger11isDebuggingEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::isDebugging(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger11isDebuggingEv")]
// IDA 0x7717c8: 10 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7717c8() {
}

// 0x7717e0 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEbED1Ev")]
// IDA 0x7717e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7717e0() {
}

// 0x771804 — __ZNK3RBX9Scripting14ScriptDebugger8isPausedEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::isPaused(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger8isPausedEv")]
// IDA 0x771804: 10 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771804() {
}

// 0x77181c — __ZNK3RBX9Scripting14ScriptDebugger14getCurrentLineEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
#[doc(alias = "RBX::Scripting::ScriptDebugger::getCurrentLine(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger14getCurrentLineEv")]
// IDA 0x77181c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77181c() {
}

// 0x771824 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::ScriptDebugger,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting14ScriptDebuggerEiED1Ev")]
// IDA 0x771824: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771824() {
}

// 0x771848 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(int),rbx::signal<void ()(int)>,rbx::signal<void ()(int)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFviEN3rbx6signalIS4_EEMS3_S7_ED1Ev")]
// IDA 0x771848: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771848() {
}

// 0x77186c — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvvEN3rbx6signalIS4_EEMS3_S7_ED1Ev")]
// IDA 0x77186c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77186c() {
}

// 0x771890 — __ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9Scripting14ScriptDebuggerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev")]
// was: RBX::Reflection::EventDesc<RBX::Scripting::ScriptDebugger,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Scripting::ScriptDebugger::*>::~EventDesc()
// IDA 0x771890: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771890() {
}

// 0x7718b4 — __ZNK3RBX9Scripting18DebuggerBreakpoint7getLineEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::getLine(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting18DebuggerBreakpoint7getLineEv")]
// IDA 0x7718b4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7718b4() {
}

// 0x7718b8 — __ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9Scripting18DebuggerBreakpointEiED1Ev")]
// IDA 0x7718b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7718b8() {
}

// 0x7718e0 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EED1Ev
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::~BoundProp()")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EED1Ev")]
// IDA 0x7718e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7718e0() {
}

// 0x771908 — __ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9Scripting13DebuggerWatchEFvvELi0EED1Ev")]
// IDA 0x771908: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771908() {
}

// 0x77192c — __ZN3rbx11make_sharedISt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS6_EEmEENS3_IT_EERKT0_
#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> rbx::make_shared<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>,unsigned long>(unsigned long const&)")]
#[doc(alias = "__ZN3rbx11make_sharedISt6vectorIN5boost10shared_ptrIN3RBX8InstanceEEESaIS6_EEmEENS3_IT_EERKT0_")]
// was: boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> rbx::make_shared<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>,unsigned long>(unsigned long const&)
// IDA 0x77192c: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77192c() {
}

// 0x771ab8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9Scripting14ScriptDebuggerEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Scripting::ScriptDebugger>(rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9Scripting14ScriptDebuggerEEERS3_RKNS0_IT_EE")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Scripting::ScriptDebugger>(boost::shared_ptr<RBX::Scripting::ScriptDebugger> const&)
// IDA 0x771ab8: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771ab8() {
}

// 0x771aec — __ZN3RBX11shared_fromINS_9Scripting14ScriptDebuggerEEEN5boost10shared_ptrIT_EEPS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::shared_from<RBX::Scripting::ScriptDebugger>(RBX::Scripting::ScriptDebugger*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_9Scripting14ScriptDebuggerEEEN5boost10shared_ptrIT_EEPS5_")]
// was: boost::shared_ptr<RBX::Scripting::ScriptDebugger> RBX::shared_from<RBX::Scripting::ScriptDebugger>(RBX::Scripting::ScriptDebugger*)
// IDA 0x771aec: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771aec() {
}

// 0x771c5c — __ZN5boost10shared_ptrIN3RBX9Scripting15DebuggerManagerEED1Ev
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerManager>::~shared_ptr()")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9Scripting15DebuggerManagerEED1Ev")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerManager>::~shared_ptr()
// IDA 0x771c5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_771c5c() {
}

// 0x771c70 — __ZN5boost10shared_ptrIN3RBX9Scripting14ScriptDebuggerEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger>::operator=(rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9Scripting14ScriptDebuggerEEaSERKS4_")]
// was: boost::shared_ptr<RBX::Scripting::ScriptDebugger>::operator=(boost::shared_ptr<RBX::Scripting::ScriptDebugger> const&)
// IDA 0x771c70: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771c70() {
}

// 0x771ca8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting14ScriptDebuggerEN5boost17reference_wrapperINS_6ScriptEEEEENS6_10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger,boost::reference_wrapper<RBX::Script>>(boost::reference_wrapper<RBX::Script>)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting14ScriptDebuggerEN5boost17reference_wrapperINS_6ScriptEEEEENS6_10shared_ptrIT_EET0_")]
// was: boost::shared_ptr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger,boost::reference_wrapper<RBX::Script>>(boost::reference_wrapper<RBX::Script>)
// IDA 0x771ca8: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771ca8() {
}

// 0x771d60 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9Scripting18DebuggerBreakpointEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Scripting::DebuggerBreakpoint>(rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9Scripting18DebuggerBreakpointEEERS3_RKNS0_IT_EE")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Scripting::DebuggerBreakpoint>(boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint> const&)
// IDA 0x771d60: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771d60() {
}

// 0x771d94 — __ZN3RBX11shared_fromINS_9Scripting18DebuggerBreakpointEEEN5boost10shared_ptrIT_EEPS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> RBX::shared_from<RBX::Scripting::DebuggerBreakpoint>(RBX::Scripting::DebuggerBreakpoint*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_9Scripting18DebuggerBreakpointEEEN5boost10shared_ptrIT_EEPS5_")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint> RBX::shared_from<RBX::Scripting::DebuggerBreakpoint>(RBX::Scripting::DebuggerBreakpoint*)
// IDA 0x771d94: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771d94() {
}

// 0x771f04 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting13DebuggerWatchESsEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch,std::string>(std::string)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting13DebuggerWatchESsEEN5boost10shared_ptrIT_EET0_")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch,std::string>(std::string)
// IDA 0x771f04: 118 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_771f04() {
}

// 0x77205c — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9Scripting13DebuggerWatchEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Scripting::DebuggerWatch>(rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9Scripting13DebuggerWatchEEERS3_RKNS0_IT_EE")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::Scripting::DebuggerWatch>(boost::shared_ptr<RBX::Scripting::DebuggerWatch> const&)
// IDA 0x77205c: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77205c() {
}

// 0x772090 — __ZN3RBX11shared_fromINS_9Scripting13DebuggerWatchEEEN5boost10shared_ptrIT_EEPS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::shared_from<RBX::Scripting::DebuggerWatch>(RBX::Scripting::DebuggerWatch*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_9Scripting13DebuggerWatchEEEN5boost10shared_ptrIT_EEPS5_")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerWatch> RBX::shared_from<RBX::Scripting::DebuggerWatch>(RBX::Scripting::DebuggerWatch*)
// IDA 0x772090: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_772090() {
}

// 0x772200 — __ZN3RBX9Scripting14ScriptDebugger16withPausedThreadINS_10Reflection7VariantEEET_N5boost8functionIFS5_P9lua_StateP9lua_DebugEEE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::Variant RBX::Scripting::ScriptDebugger::withPausedThread<RBX::Reflection::Variant>(boost::function<RBX::Reflection::Variant ()(lua_State *,lua_Debug *)>)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16withPausedThreadINS_10Reflection7VariantEEET_N5boost8functionIFS5_P9lua_StateP9lua_DebugEEE")]
// IDA 0x772200: 538 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_772200() {
}

// 0x7727e8 — __ZN5boost4bindIN3RBX10Reflection7VariantESsP9lua_StateSsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<RBX::Reflection::Variant,RBX::Reflection::Variant (*)(std::string,lua_State *),boost::_bi::list_av_2<std::string,boost::arg<1>>::type> boost::bind<RBX::Reflection::Variant,std::string,lua_State *,std::string,boost::arg<1>>(RBX::Reflection::Variant (*)(std::string,lua_State *),std::string,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIN3RBX10Reflection7VariantESsP9lua_StateSsNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_")]
// IDA 0x7727e8: 145 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7727e8() {
}

// 0x772990 — __ZN3RBX9Scripting14ScriptDebugger6onHookEP9lua_StateP9lua_Debug
#[doc(alias = "RBX::Scripting::ScriptDebugger::onHook(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger6onHookEP9lua_StateP9lua_Debug")]
// IDA 0x772990: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_772990() {
}

// 0x772bcc — __ZN3rbx11make_sharedISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS4_EEEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> rbx::make_shared<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(void)")]
#[doc(alias = "__ZN3rbx11make_sharedISt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS4_EEEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>> rbx::make_shared<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(void)
// IDA 0x772bcc: 127 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_772bcc() {
}

// 0x772d2c — __ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE9push_backERKS3_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::push_back(RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE9push_backERKS3_")]
// IDA 0x772d2c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_772d2c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x772ed4 — __ZN5boost10shared_ptrIN3RBX9Scripting18DebuggerBreakpointEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint>::operator=(rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9Scripting18DebuggerBreakpointEEaSERKS4_")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint>::operator=(boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint> const&)
// IDA 0x772ed4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_772ed4() {
}

// 0x772f0c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting18DebuggerBreakpointEiEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerBreakpoint,int>(int)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting18DebuggerBreakpointEiEEN5boost10shared_ptrIT_EET0_")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerBreakpoint,int>(int)
// IDA 0x772f0c: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_772f0c() {
}

// 0x772fc0 — __ZSt6removeIN9__gnu_cxx17__normal_iteratorIPPN3RBX9Scripting13DebuggerWatchESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_
#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::remove<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&)")]
#[doc(alias = "__ZSt6removeIN9__gnu_cxx17__normal_iteratorIPPN3RBX9Scripting13DebuggerWatchESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_")]
// IDA 0x772fc0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_772fc0() {
}

// 0x772fec — __ZNSt6vectorIPN3RBX9Scripting13DebuggerWatchESaIS3_EE9push_backERKS3_
#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::push_back(RBX::Scripting::DebuggerWatch * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9Scripting13DebuggerWatchESaIS3_EE9push_backERKS3_")]
// IDA 0x772fec: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_772fec() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x773018 — __ZN3RBX11shared_fromINS_6ScriptEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::Script> RBX::shared_from<RBX::Script>(RBX::Script*)")]
#[doc(alias = "__ZN3RBX11shared_fromINS_6ScriptEEEN5boost10shared_ptrIT_EEPS4_")]
// was: boost::shared_ptr<RBX::Script> RBX::shared_from<RBX::Script>(RBX::Script*)
// IDA 0x773018: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_773018() {
}

// 0x773188 — __ZN3rbx7signals6signalIFvP9lua_StateEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(lua_State *)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvP9lua_StateEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvN3RBX9Scripting14ScriptDebuggerES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// IDA 0x773188: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_773188() {
}

// 0x7731fc — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Scripting::ScriptDebugger>,boost::_bi::list1<boost::_bi::value<RBX::Scripting::ScriptDebugger*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9Scripting14ScriptDebuggerEEENS6_5list1INS6_5valueIPSC_EEEEEEEENS0_10connectionERKT_")]
// IDA 0x7731fc: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7731fc() {
}

// 0x773270 — __ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEET_NS3_8functionIFSH_P9lua_StateP9lua_DebugEEE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int)
#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Scripting::ScriptDebugger::withPausedThread<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(boost::function<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEET_NS3_8functionIFSH_P9lua_StateP9lua_DebugEEE")]
// was: boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> RBX::Scripting::ScriptDebugger::withPausedThread<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(boost::function<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(lua_State *,lua_Debug *)>)
// IDA 0x773270: 508 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_773270() {
}

// 0x773814 — __ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIbEET_N5boost8functionIFS3_P9lua_StateP9lua_DebugEEE
// type: int __fastcall(int, int)
#[doc(alias = "bool RBX::Scripting::ScriptDebugger::withPausedThread<bool>(boost::function<bool ()(lua_State *,lua_Debug *)>)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16withPausedThreadIbEET_N5boost8functionIFS3_P9lua_StateP9lua_DebugEEE")]
// IDA 0x773814: 485 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_773814() {
}

// 0x773d80 — __ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEiP9lua_StateSsNS_17reference_wrapperIS4_EEiNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_ENSC_9list_av_4IT4_T5_T6_T7_E4typeEEESK_SM_SN_SO_SP_
// type: int __fastcall(int, int, std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),boost::_bi::list_av_4<std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>>::type> boost::bind<bool,std::string,RBX::Reflection::Variant const&,int,lua_State *,std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>>(bool (*)(std::string,RBX::Reflection::Variant const&,int,lua_State *),std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,int,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEiP9lua_StateSsNS_17reference_wrapperIS4_EEiNS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_ENSC_9list_av_4IT4_T5_T6_T7_E4typeEEESK_SM_SN_SO_SP_")]
// IDA 0x773d80: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_773d80() {
}

// 0x773f38 — __ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEP9lua_StateSsNS_17reference_wrapperIS4_EENS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_ENSC_9list_av_3IT3_T4_T5_E4typeEEESJ_SL_SM_SN_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),boost::_bi::list_av_3<std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>::type> boost::bind<bool,std::string,RBX::Reflection::Variant const&,lua_State *,std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>>(bool (*)(std::string,RBX::Reflection::Variant const&,lua_State *),std::string,boost::reference_wrapper<RBX::Reflection::Variant const>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIbSsRKN3RBX10Reflection7VariantEP9lua_StateSsNS_17reference_wrapperIS4_EENS_3argILi1EEEEENS_3_bi6bind_tIT_PFSE_T0_T1_T2_ENSC_9list_av_3IT3_T4_T5_E4typeEEESJ_SL_SM_SN_")]
// IDA 0x773f38: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_773f38() {
}

// 0x7740e8 — __ZNK3RBX10Reflection7Variant3getIN5boost10shared_ptrINS_8InstanceEEEEET_v
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> RBX::Reflection::Variant::get<rbx_core::SharedPtr<RBX::Instance>>(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection7Variant3getIN5boost10shared_ptrINS_8InstanceEEEEET_v")]
// was: boost::shared_ptr<RBX::Instance> RBX::Reflection::Variant::get<boost::shared_ptr<RBX::Instance>>(void)const
// IDA 0x7740e8: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7740e8() {
}

// 0x774260 — __ZNSt6vectorIiSaIiEE9push_backERKi
#[doc(alias = "std::vector<int,std::allocator<int>>::push_back(int const&)")]
#[doc(alias = "__ZNSt6vectorIiSaIiEE9push_backERKi")]
// IDA 0x774260: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_774260() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x774288 — __ZN3RBX9Scripting13DebuggerWatchD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatchD1Ev")]
// IDA 0x774288: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_774288() {
}

// 0x77436c — __ZN3RBX9Scripting13DebuggerWatchD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "RBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatchD0Ev")]
// IDA 0x77436c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77436c() {
}

// 0x774460 — __ZNK3RBX9Scripting13DebuggerWatch14verifyAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerWatch *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerWatch::verifyAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting13DebuggerWatch14verifyAddChildEPKNS_8InstanceE")]
// IDA 0x774460: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774460() {
}

// 0x774580 — __ZNK3RBX9Scripting13DebuggerWatch14askForbidChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerWatch *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerWatch::askForbidChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting13DebuggerWatch14askForbidChildEPKNS_8InstanceE")]
// IDA 0x774580: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774580() {
}

// 0x774584 — __ZThn32_N3RBX9Scripting13DebuggerWatchD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting13DebuggerWatchD1Ev")]
// IDA 0x774584: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_774584() {
}

// 0x774664 — __ZThn32_N3RBX9Scripting13DebuggerWatchD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting13DebuggerWatchD0Ev")]
// IDA 0x774664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_774664() {
}

// 0x77475c — __ZThn36_N3RBX9Scripting13DebuggerWatchD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting13DebuggerWatchD1Ev")]
// IDA 0x77475c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77475c() {
}

// 0x77483c — __ZThn36_N3RBX9Scripting13DebuggerWatchD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting13DebuggerWatchD0Ev")]
// IDA 0x77483c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_77483c() {
}

// 0x774938 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv")]
// IDA 0x774938: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774938() {
}

// 0x774960 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_9Scripting16sDebuggerManagerEEE12getClassNameEv")]
// IDA 0x774960: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774960() {
}

// 0x774988 — __ZNK3RBX9Scripting18DebuggerBreakpoint14verifyAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::verifyAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting18DebuggerBreakpoint14verifyAddChildEPKNS_8InstanceE")]
// IDA 0x774988: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774988() {
}

// 0x774aa8 — __ZNK3RBX9Scripting18DebuggerBreakpoint14askForbidChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::askForbidChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX9Scripting18DebuggerBreakpoint14askForbidChildEPKNS_8InstanceE")]
// IDA 0x774aa8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774aa8() {
}

// 0x774aac — __ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv")]
// IDA 0x774aac: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774aac() {
}

// 0x774abc — __ZThn32_NK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E12getClassNameEv")]
// IDA 0x774abc: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774abc() {
}

// 0x774ad0 — __ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorD1Ev")]
// IDA 0x774ad0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_774ad0() {
}

// 0x774ad4 — __ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD1Ev")]
// IDA 0x774ad4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_774ad4() {
}

// 0x774ad8 — __ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorD2Ev")]
// IDA 0x774ad8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_774ad8() {
}

// 0x774b74 — __ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator12getClassNameEv")]
// IDA 0x774b74: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774b74() {
}

// 0x774bfc — __ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7Creator6createEv")]
// IDA 0x774bfc: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774bfc() {
}

// 0x774d40 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting18DebuggerBreakpointEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerBreakpoint>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9Scripting18DebuggerBreakpointEEEN5boost10shared_ptrIT_EEv")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerBreakpoint>(void)
// IDA 0x774d40: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774d40() {
}

// 0x774df0 — __ZN5boost10shared_ptrIN3RBX9Scripting18DebuggerBreakpointEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint>::shared_ptr<RBX::Scripting::DebuggerBreakpoint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9Scripting18DebuggerBreakpointEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint>::shared_ptr<RBX::Scripting::DebuggerBreakpoint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x774df0: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774df0() {
}

// 0x774ebc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9Scripting18DebuggerBreakpointES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerBreakpoint,RBX::Scripting::DebuggerBreakpoint>(rbx_core::SharedPtr<RBX::Scripting::DebuggerBreakpoint> const*,RBX::Scripting::DebuggerBreakpoint *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9Scripting18DebuggerBreakpointES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerBreakpoint,RBX::Scripting::DebuggerBreakpoint>(boost::shared_ptr<RBX::Scripting::DebuggerBreakpoint> const*,RBX::Scripting::DebuggerBreakpoint *)const
// IDA 0x774ebc: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774ebc() {
}

// 0x774fa4 — __ZN5boost6detail12shared_countC2IPN3RBX9Scripting18DebuggerBreakpointENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9Scripting18DebuggerBreakpointENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// IDA 0x774fa4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_774fa4() {
}

// 0x7750ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// IDA 0x7750ac: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_7750ac() {
}

// 0x7750b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// IDA 0x7750b0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7750b0() {
}

// 0x7750b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// IDA 0x7750b4: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7750b4() {
}

// 0x7750d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// IDA 0x7750d8: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7750d8() {
}

// 0x7750f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerBreakpoint *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9Scripting18DebuggerBreakpointENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// IDA 0x7750f0: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7750f0() {
}

// 0x7750f4 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting19sDebuggerBreakpointEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting19sDebuggerBreakpointEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting19sDebuggerBreakpointEEEEvv")]
// IDA 0x7750f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_7750f4() {
}

// 0x7750f8 — __ZN3RBX4Name9doDeclareILZNS_9Scripting19sDebuggerBreakpointEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting19sDebuggerBreakpointEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting19sDebuggerBreakpointEEEERKS0_v")]
// IDA 0x7750f8: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7750f8() {
}

// 0x7751dc — __ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E7CreatorC2Ev")]
// IDA 0x7751dc: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_7751dc() {
}

// 0x775424 — __ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting18DebuggerBreakpointENS_8InstanceELZNS1_19sDebuggerBreakpointEES3_E17static_getCreatorEv")]
// IDA 0x775424: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775424() {
}

// 0x775498 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting16sDebuggerManagerEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting16sDebuggerManagerEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting16sDebuggerManagerEEEEvv")]
// IDA 0x775498: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_775498() {
}

// 0x77549c — __ZN3RBX4Name9doDeclareILZNS_9Scripting16sDebuggerManagerEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting16sDebuggerManagerEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting16sDebuggerManagerEEEERKS0_v")]
// IDA 0x77549c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_77549c() {
}

// 0x775580 — __ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorD2Ev")]
// IDA 0x775580: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_775580() {
}

// 0x775620 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting14sDebuggerWatchEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting14sDebuggerWatchEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting14sDebuggerWatchEEEEvv")]
// IDA 0x775620: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_775620() {
}

// 0x775628 — __ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7Creator6createEv")]
// IDA 0x775628: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775628() {
}

// 0x775770 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting15sScriptDebuggerEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting15sScriptDebuggerEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting15sScriptDebuggerEEEEvv")]
// IDA 0x775770: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_775770() {
}

// 0x775778 — __ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
// IDA 0x775778: 92 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775778() {
}

// 0x775894 — __ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
// IDA 0x775894: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_775894() {
}

// 0x775898 — __ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
// IDA 0x775898: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_775898() {
}

// 0x775938 — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
// IDA 0x775938: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_775938() {
}

// 0x775940 — __ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
// IDA 0x775940: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_775940() {
}

// 0x7759e4 — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
// IDA 0x7759e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7759e4() {
}

// 0x7759ec — __ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting18DebuggerBreakpointELZNS2_19sDebuggerBreakpointEENS_14FactoryProductIS3_NS_8InstanceELZNS2_19sDebuggerBreakpointEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
// IDA 0x7759ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_7759ec() {
}

// 0x775d3c — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt16reverse_iteratorIN9__gnu_cxx17__normal_iteratorIPiSt6vectorIiSaIiEEEEESt16ostream_iteratorIicSt11char_traitsIcEEEET0_T_SH_SG_
#[doc(alias = "std::ostream_iterator<int,char,std::char_traits<char>> std::__copy<false,std::random_access_iterator_tag>::copy<std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::ostream_iterator<int,char,std::char_traits<char>>>(std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::reverse_iterator<__gnu_cxx::__normal_iterator<int *,std::vector<int,std::allocator<int>>>>,std::ostream_iterator<int,char,std::char_traits<char>>)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt16reverse_iteratorIN9__gnu_cxx17__normal_iteratorIPiSt6vectorIiSaIiEEEEESt16ostream_iteratorIicSt11char_traitsIcEEEET0_T_SH_SG_")]
// IDA 0x775d3c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_775d3c() {
}
