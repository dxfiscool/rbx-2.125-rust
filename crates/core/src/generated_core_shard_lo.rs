//! core shard lo — 100 core stubs EA-sorted, next uncovered fallback after shard ln (0x647bd4..0x7f9ce8, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|FMOD|Lua (fallback 37271, 6238 uncovered -> 6138 after, 38449->38549 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::Stats::StatsService::tryToStartScript(void)")]
#[doc(alias = "__ZN3RBX5Stats12StatsService16tryToStartScriptEv")]
// 0x647bd4 — __ZN3RBX5Stats12StatsService16tryToStartScriptEv
// type: _DWORD __fastcall(RBX::Stats::StatsService *__hidden this)
pub fn stub_0x647bd4() {
    // IDA 0x647bd4: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sScriptContextEEEEvv")]
// 0x652a70 — __ZN3RBX4Name13callDoDeclareILZNS_14sScriptContextEEEEvv
pub fn stub_0x652a70() {
    // IDA 0x652a70: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX5NamedINS_18ScriptMouseCommandELZNS_17sToolMouseCommandEEE7getNameEv")]
// 0x689260 — __ZNK3RBX5NamedINS_18ScriptMouseCommandELZNS_17sToolMouseCommandEEE7getNameEv
pub fn stub_0x689260() {
    // IDA 0x689260: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv")]
// 0x6d3ca0 — __ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv
pub fn stub_0x6d3ca0() {
    // IDA 0x6d3ca0: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_v")]
// 0x705bb0 — __ZNK3RBX15ServiceProvider4findINS_13ScriptServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x705bb0() {
    // IDA 0x705bb0: script yield/resume state machine owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v")]
// 0x705dc8 — __ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v
pub fn stub_0x705dc8() {
    // IDA 0x705dc8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v")]
// 0x705e10 — __ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v
// type: int()
pub fn stub_0x705e10() {
    // IDA 0x705e10: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptServiceEEEmv")]
// 0x705ef8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ScriptServiceEEEmv
pub fn stub_0x705ef8() {
    // IDA 0x705ef8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::enableDebugging(void)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager15enableDebuggingEv")]
// 0x767b10 — __ZN3RBX9Scripting15DebuggerManager15enableDebuggingEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x767b10() {
    // IDA 0x767b10: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::resume(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger6resumeEv")]
// 0x76829c — __ZN3RBX9Scripting14ScriptDebugger6resumeEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76829c() {
    // IDA 0x76829c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOver(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8stepOverEv")]
// 0x7685c4 — __ZN3RBX9Scripting14ScriptDebugger8stepOverEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x7685c4() {
    // IDA 0x7685c4: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::stepInto(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8stepIntoEv")]
// 0x768750 — __ZN3RBX9Scripting14ScriptDebugger8stepIntoEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x768750() {
    // IDA 0x768750: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOut(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger7stepOutEv")]
// 0x7688d8 — __ZN3RBX9Scripting14ScriptDebugger7stepOutEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x7688d8() {
    // IDA 0x7688d8: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getLocals(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger9getLocalsEi")]
// 0x769338 — __ZN3RBX9Scripting14ScriptDebugger9getLocalsEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
pub fn stub_0x769338() {
    // IDA 0x769338: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getUpvalues(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger11getUpvaluesEi")]
// 0x769414 — __ZN3RBX9Scripting14ScriptDebugger11getUpvaluesEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
pub fn stub_0x769414() {
    // IDA 0x769414: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getGlobals(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger10getGlobalsEv")]
// 0x7694f0 — __ZN3RBX9Scripting14ScriptDebugger10getGlobalsEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x7694f0() {
    // IDA 0x7694f0: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getScriptPath(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger13getScriptPathEv")]
// 0x769db0 — __ZNK3RBX9Scripting14ScriptDebugger13getScriptPathEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x769db0() {
    // IDA 0x769db0: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::setScriptPath(std::string)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13setScriptPathESs")]
// 0x769f7c — __ZN3RBX9Scripting14ScriptDebugger13setScriptPathESs
pub fn stub_0x769f7c() {
    // IDA 0x769f7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::checkExpressionSyntax(void)")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatch21checkExpressionSyntaxEv")]
// 0x76a5c0 — __ZN3RBX9Scripting13DebuggerWatch21checkExpressionSyntaxEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
pub fn stub_0x76a5c0() {
    // IDA 0x76a5c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::singleton(void)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager9singletonEv")]
// 0x76a92c — __ZN3RBX9Scripting15DebuggerManager9singletonEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76a92c() {
    // IDA 0x76a92c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::DebuggerManager(void)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerC2Ev")]
// 0x76ab8c — __ZN3RBX9Scripting15DebuggerManagerC2Ev
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76ab8c() {
    // IDA 0x76ab8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerD0Ev")]
// 0x76aec4 — __ZN3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76aec4() {
    // IDA 0x76aec4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerD1Ev")]
// 0x76af64 — __ZN3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76af64() {
    // IDA 0x76af64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting15DebuggerManagerD0Ev")]
// 0x76af68 — __ZThn32_N3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76af68() {
    // IDA 0x76af68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting15DebuggerManagerD0Ev")]
// 0x76af70 — __ZThn36_N3RBX9Scripting15DebuggerManagerD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76af70() {
    // IDA 0x76af70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManagerD2Ev")]
// 0x76af78 — __ZN3RBX9Scripting15DebuggerManagerD2Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76af78() {
    // IDA 0x76af78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting15DebuggerManagerD1Ev")]
// 0x76b128 — __ZThn32_N3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76b128() {
    // IDA 0x76b128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting15DebuggerManagerD1Ev")]
// 0x76b130 — __ZThn36_N3RBX9Scripting15DebuggerManagerD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x76b130() {
    // IDA 0x76b130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager12findDebuggerEP9lua_State")]
// 0x76b13c — __ZN3RBX9Scripting15DebuggerManager12findDebuggerEP9lua_State
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x76b13c() {
    // IDA 0x76b13c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(RBX::Script *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager12findDebuggerEPNS_6ScriptE")]
// 0x76b2b0 — __ZN3RBX9Scripting15DebuggerManager12findDebuggerEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Script *)
pub fn stub_0x76b2b0() {
    // IDA 0x76b2b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::addDebugger(RBX::Script *)")]
#[doc(alias = "__ZN3RBX9Scripting15DebuggerManager11addDebuggerEPNS_6ScriptE")]
// 0x76b470 — __ZN3RBX9Scripting15DebuggerManager11addDebuggerEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this, RBX::Script *)
pub fn stub_0x76b470() {
    // IDA 0x76b470: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::ScriptDebugger(RBX::Script &)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerC2ERNS_6ScriptE")]
// 0x76b99c — __ZN3RBX9Scripting14ScriptDebuggerC2ERNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Script *)
pub fn stub_0x76b99c() {
    // IDA 0x76b99c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::setScript(RBX::Script *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger9setScriptEPNS_6ScriptE")]
// 0x76c054 — __ZN3RBX9Scripting14ScriptDebugger9setScriptEPNS_6ScriptE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Script *)
pub fn stub_0x76c054() {
    // IDA 0x76c054: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerD0Ev")]
// 0x76c3a4 — __ZN3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76c3a4() {
    // IDA 0x76c3a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerD1Ev")]
// 0x76c444 — __ZN3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76c444() {
    // IDA 0x76c444: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting14ScriptDebuggerD0Ev")]
// 0x76c448 — __ZThn32_N3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76c448() {
    // IDA 0x76c448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting14ScriptDebuggerD0Ev")]
// 0x76c450 — __ZThn36_N3RBX9Scripting14ScriptDebuggerD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76c450() {
    // IDA 0x76c450: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebuggerD2Ev")]
// 0x76c458 — __ZN3RBX9Scripting14ScriptDebuggerD2Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76c458() {
    // IDA 0x76c458: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting14ScriptDebuggerD1Ev")]
// 0x76ca0c — __ZThn32_N3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76ca0c() {
    // IDA 0x76ca0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting14ScriptDebuggerD1Ev")]
// 0x76ca14 — __ZThn36_N3RBX9Scripting14ScriptDebuggerD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76ca14() {
    // IDA 0x76ca14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::addWatch(std::string)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8addWatchESs")]
// 0x76ca1c — __ZN3RBX9Scripting14ScriptDebugger8addWatchESs
pub fn stub_0x76ca1c() {
    // IDA 0x76ca1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatchValue(RBX::Scripting::DebuggerWatch *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13getWatchValueEPNS0_13DebuggerWatchE")]
// 0x76cb6c — __ZN3RBX9Scripting14ScriptDebugger13getWatchValueEPNS0_13DebuggerWatchE
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, RBX::Scripting::DebuggerWatch *)
pub fn stub_0x76cb6c() {
    // IDA 0x76cb6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::hook(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger4hookEP9lua_StateP9lua_Debug")]
// 0x76d500 — __ZN3RBX9Scripting14ScriptDebugger4hookEP9lua_StateP9lua_Debug
pub fn stub_0x76d500() {
    // IDA 0x76d500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::debuggerBreak(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13debuggerBreakEP9lua_StateP9lua_Debug")]
// 0x76d5e0 — __ZN3RBX9Scripting14ScriptDebugger13debuggerBreakEP9lua_StateP9lua_Debug
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x76d5e0() {
    // IDA 0x76d5e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readLocals(int,lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger10readLocalsEiP9lua_State")]
// 0x76d95c — __ZN3RBX9Scripting14ScriptDebugger10readLocalsEiP9lua_State
pub fn stub_0x76d95c() {
    // IDA 0x76d95c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readGlobals(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger11readGlobalsEP9lua_State")]
// 0x76dc5c — __ZN3RBX9Scripting14ScriptDebugger11readGlobalsEP9lua_State
pub fn stub_0x76dc5c() {
    // IDA 0x76dc5c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readUpvalues(int,lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger12readUpvaluesEiP9lua_State")]
// 0x76dfcc — __ZN3RBX9Scripting14ScriptDebugger12readUpvaluesEiP9lua_State
pub fn stub_0x76dfcc() {
    // IDA 0x76dfcc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readStack(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger9readStackEP9lua_State")]
// 0x76e434 — __ZN3RBX9Scripting14ScriptDebugger9readStackEP9lua_State
pub fn stub_0x76e434() {
    // IDA 0x76e434: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::onLineHook(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger10onLineHookEP9lua_StateP9lua_Debug")]
// 0x76ea28 — __ZN3RBX9Scripting14ScriptDebugger10onLineHookEP9lua_StateP9lua_Debug
// type: int __fastcall(char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x76ea28() {
    // IDA 0x76ea28: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::findBreakpoint(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger14findBreakpointEi")]
// 0x76ecb0 — __ZN3RBX9Scripting14ScriptDebugger14findBreakpointEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
pub fn stub_0x76ecb0() {
    // IDA 0x76ecb0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::shouldBreak(RBX::Scripting::DebuggerBreakpoint *,lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger11shouldBreakEPNS0_18DebuggerBreakpointEP9lua_State")]
// 0x76ece8 — __ZN3RBX9Scripting14ScriptDebugger11shouldBreakEPNS0_18DebuggerBreakpointEP9lua_State
pub fn stub_0x76ece8() {
    // IDA 0x76ece8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::setBreakpoint(int)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger13setBreakpointEi")]
// 0x76f488 — __ZN3RBX9Scripting14ScriptDebugger13setBreakpointEi
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this, int)
pub fn stub_0x76f488() {
    // IDA 0x76f488: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::onScriptStarting(lua_State *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16onScriptStartingEP9lua_State")]
// 0x76fa0c — __ZN3RBX9Scripting14ScriptDebugger16onScriptStartingEP9lua_State
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int)
pub fn stub_0x76fa0c() {
    // IDA 0x76fa0c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::onScriptStopped(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger15onScriptStoppedEv")]
// 0x76fbc8 — __ZN3RBX9Scripting14ScriptDebugger15onScriptStoppedEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x76fbc8() {
    // IDA 0x76fbc8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getStack(void)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger8getStackEv")]
// 0x770184 — __ZN3RBX9Scripting14ScriptDebugger8getStackEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x770184() {
    // IDA 0x770184: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::DebuggerBreakpoint(void)")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointC2Ev")]
// 0x770384 — __ZN3RBX9Scripting18DebuggerBreakpointC2Ev
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x770384() {
    // IDA 0x770384: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::DebuggerBreakpoint(int)")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointC2Ei")]
// 0x7704dc — __ZN3RBX9Scripting18DebuggerBreakpointC2Ei
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this, int)
pub fn stub_0x7704dc() {
    // IDA 0x7704dc: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointD0Ev")]
// 0x770764 — __ZN3RBX9Scripting18DebuggerBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x770764() {
    // IDA 0x770764: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointD1Ev")]
// 0x770804 — __ZN3RBX9Scripting18DebuggerBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x770804() {
    // IDA 0x770804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting18DebuggerBreakpointD0Ev")]
// 0x770808 — __ZThn32_N3RBX9Scripting18DebuggerBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x770808() {
    // IDA 0x770808: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting18DebuggerBreakpointD0Ev")]
// 0x770810 — __ZThn36_N3RBX9Scripting18DebuggerBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x770810() {
    // IDA 0x770810: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting18DebuggerBreakpointD2Ev")]
// 0x770818 — __ZN3RBX9Scripting18DebuggerBreakpointD2Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x770818() {
    // IDA 0x770818: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting18DebuggerBreakpointD1Ev")]
// 0x7708fc — __ZThn32_N3RBX9Scripting18DebuggerBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x7708fc() {
    // IDA 0x7708fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerBreakpoint::~DebuggerBreakpoint()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting18DebuggerBreakpointD1Ev")]
// 0x770904 — __ZThn36_N3RBX9Scripting18DebuggerBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x770904() {
    // IDA 0x770904: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::DebuggerWatch(std::string)")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatchC2ESs")]
// 0x770910 — __ZN3RBX9Scripting13DebuggerWatchC2ESs
pub fn stub_0x770910() {
    // IDA 0x770910: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerManager::getEnabled(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting15DebuggerManager10getEnabledEv")]
// 0x7711c8 — __ZNK3RBX9Scripting15DebuggerManager10getEnabledEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerManager *__hidden this)
pub fn stub_0x7711c8() {
    // IDA 0x7711c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getScript(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger9getScriptEv")]
// 0x771770 — __ZNK3RBX9Scripting14ScriptDebugger9getScriptEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x771770() {
    // IDA 0x771770: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::isDebugging(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger11isDebuggingEv")]
// 0x7717c8 — __ZNK3RBX9Scripting14ScriptDebugger11isDebuggingEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x7717c8() {
    // IDA 0x7717c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::isPaused(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger8isPausedEv")]
// 0x771804 — __ZNK3RBX9Scripting14ScriptDebugger8isPausedEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x771804() {
    // IDA 0x771804: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getCurrentLine(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting14ScriptDebugger14getCurrentLineEv")]
// 0x77181c — __ZNK3RBX9Scripting14ScriptDebugger14getCurrentLineEv
// type: _DWORD __fastcall(RBX::Scripting::ScriptDebugger *__hidden this)
pub fn stub_0x77181c() {
    // IDA 0x77181c: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::DebuggerBreakpoint::getLine(void)const")]
#[doc(alias = "__ZNK3RBX9Scripting18DebuggerBreakpoint7getLineEv")]
// 0x7718b4 — __ZNK3RBX9Scripting18DebuggerBreakpoint7getLineEv
// type: _DWORD __fastcall(RBX::Scripting::DebuggerBreakpoint *__hidden this)
pub fn stub_0x7718b4() {
    // IDA 0x7718b4: script-debugger wiring owned by the script crate — carrier no-op in core.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::onHook(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger6onHookEP9lua_StateP9lua_Debug")]
// 0x772990 — __ZN3RBX9Scripting14ScriptDebugger6onHookEP9lua_StateP9lua_Debug
pub fn stub_0x772990() {
    // IDA 0x772990: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::push_back(RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE9push_backERKS3_")]
// 0x772d2c — __ZNSt6vectorIN3RBX9Scripting14ScriptDebugger12FunctionInfoESaIS3_EE9push_backERKS3_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
pub fn stub_0x772d2c() {
    // IDA 0x772d2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::remove<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&)")]
#[doc(alias = "__ZSt6removeIN9__gnu_cxx17__normal_iteratorIPPN3RBX9Scripting13DebuggerWatchESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_")]
// 0x772fc0 — __ZSt6removeIN9__gnu_cxx17__normal_iteratorIPPN3RBX9Scripting13DebuggerWatchESt6vectorIS5_SaIS5_EEEES5_ET_SB_SB_RKT0_
pub fn stub_0x772fc0() {
    // IDA 0x772fc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::push_back(RBX::Scripting::DebuggerWatch * const&)")]
#[doc(alias = "__ZNSt6vectorIPN3RBX9Scripting13DebuggerWatchESaIS3_EE9push_backERKS3_")]
// 0x772fec — __ZNSt6vectorIPN3RBX9Scripting13DebuggerWatchESaIS3_EE9push_backERKS3_
pub fn stub_0x772fec() {
    // IDA 0x772fec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatchD1Ev")]
// 0x774288 — __ZN3RBX9Scripting13DebuggerWatchD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
pub fn stub_0x774288() {
    // IDA 0x774288: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZN3RBX9Scripting13DebuggerWatchD0Ev")]
// 0x77436c — __ZN3RBX9Scripting13DebuggerWatchD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
pub fn stub_0x77436c() {
    // IDA 0x77436c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting13DebuggerWatchD1Ev")]
// 0x774584 — __ZThn32_N3RBX9Scripting13DebuggerWatchD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
pub fn stub_0x774584() {
    // IDA 0x774584: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn32_N3RBX9Scripting13DebuggerWatchD0Ev")]
// 0x774664 — __ZThn32_N3RBX9Scripting13DebuggerWatchD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
pub fn stub_0x774664() {
    // IDA 0x774664: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting13DebuggerWatchD1Ev")]
// 0x77475c — __ZThn36_N3RBX9Scripting13DebuggerWatchD1Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
pub fn stub_0x77475c() {
    // IDA 0x77475c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerWatch::~DebuggerWatch()")]
#[doc(alias = "__ZThn36_N3RBX9Scripting13DebuggerWatchD0Ev")]
// 0x77483c — __ZThn36_N3RBX9Scripting13DebuggerWatchD0Ev
// type: void __fastcall(RBX::Scripting::DebuggerWatch *__hidden this)
pub fn stub_0x77483c() {
    // IDA 0x77483c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting19sDebuggerBreakpointEEEEvv")]
// 0x7750f4 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting19sDebuggerBreakpointEEEEvv
pub fn stub_0x7750f4() {
    // IDA 0x7750f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting19sDebuggerBreakpointEEEERKS0_v")]
// 0x7750f8 — __ZN3RBX4Name9doDeclareILZNS_9Scripting19sDebuggerBreakpointEEEERKS0_v
pub fn stub_0x7750f8() {
    // IDA 0x7750f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting16sDebuggerManagerEEEEvv")]
// 0x775498 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting16sDebuggerManagerEEEEvv
// type: int()
pub fn stub_0x775498() {
    // IDA 0x775498: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting16sDebuggerManagerEEEERKS0_v")]
// 0x77549c — __ZN3RBX4Name9doDeclareILZNS_9Scripting16sDebuggerManagerEEEERKS0_v
pub fn stub_0x77549c() {
    // IDA 0x77549c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting14sDebuggerWatchEEEEvv")]
// 0x775620 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting14sDebuggerWatchEEEEvv
pub fn stub_0x775620() {
    // IDA 0x775620: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9Scripting15sScriptDebuggerEEEEvv")]
// 0x775770 — __ZN3RBX4Name13callDoDeclareILZNS_9Scripting15sScriptDebuggerEEEEvv
pub fn stub_0x775770() {
    // IDA 0x775770: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::~StepOverBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD1Ev")]
// 0x786c5c — __ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOverBreakpoint *__hidden this)
pub fn stub_0x786c5c() {
    // IDA 0x786c5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::~StepOverBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD0Ev")]
// 0x786c60 — __ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOverBreakpoint *__hidden this)
pub fn stub_0x786c60() {
    // IDA 0x786c60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOverBreakpoint::hitTest(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpoint7hitTestEP9lua_StateP9lua_Debug")]
// 0x786c64 — __ZN3RBX9Scripting14ScriptDebugger18StepOverBreakpoint7hitTestEP9lua_StateP9lua_Debug
pub fn stub_0x786c64() {
    // IDA 0x786c64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::~StepOutBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD1Ev")]
// 0x786d28 — __ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOutBreakpoint *__hidden this)
pub fn stub_0x786d28() {
    // IDA 0x786d28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::~StepOutBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD0Ev")]
// 0x786d2c — __ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepOutBreakpoint *__hidden this)
pub fn stub_0x786d2c() {
    // IDA 0x786d2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepOutBreakpoint::hitTest(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpoint7hitTestEP9lua_StateP9lua_Debug")]
// 0x786d30 — __ZN3RBX9Scripting14ScriptDebugger17StepOutBreakpoint7hitTestEP9lua_StateP9lua_Debug
pub fn stub_0x786d30() {
    // IDA 0x786d30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::~StepInBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD1Ev")]
// 0x786e24 — __ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD1Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepInBreakpoint *__hidden this)
pub fn stub_0x786e24() {
    // IDA 0x786e24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::~StepInBreakpoint()")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD0Ev")]
// 0x786e28 — __ZN3RBX9Scripting14ScriptDebugger16StepInBreakpointD0Ev
// type: void __fastcall(RBX::Scripting::ScriptDebugger::StepInBreakpoint *__hidden this)
pub fn stub_0x786e28() {
    // IDA 0x786e28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::StepInBreakpoint::hitTest(lua_State *,lua_Debug *)")]
#[doc(alias = "__ZN3RBX9Scripting14ScriptDebugger16StepInBreakpoint7hitTestEP9lua_StateP9lua_Debug")]
// 0x786e2c — __ZN3RBX9Scripting14ScriptDebugger16StepInBreakpoint7hitTestEP9lua_StateP9lua_Debug
pub fn stub_0x786e2c() {
    // IDA 0x786e2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentProvider::verifyRequestedScriptSignature(char const*,std::string const&,bool)")]
#[doc(alias = "__ZN3RBX15ContentProvider30verifyRequestedScriptSignatureEPKcRKSsb")]
// 0x7eb54c — __ZN3RBX15ContentProvider30verifyRequestedScriptSignatureEPKcRKSsb
// type: _DWORD __fastcall(RBX::ContentProvider *__hidden this, const char *, const std::string *, bool)
pub fn stub_0x7eb54c() {
    // IDA 0x7eb54c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ContentProvider::verifyScriptSignature(char const*,bool)")]
#[doc(alias = "__ZN3RBX15ContentProvider21verifyScriptSignatureEPKcb")]
// 0x7eb9b0 — __ZN3RBX15ContentProvider21verifyScriptSignatureEPKcb
// type: struct _Unwind_Exception *__fastcall(RBX::ContentProvider *this, const char *, bool)
pub fn stub_0x7eb9b0() {
    // IDA 0x7eb9b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Soundscape::CollisionSoundManager::PlaySound(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX10Soundscape21CollisionSoundManager9PlaySoundEPNS_9PrimitiveE")]
// 0x7f9bb8 — __ZN3RBX10Soundscape21CollisionSoundManager9PlaySoundEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this, RBX::Primitive *)
pub fn stub_0x7f9bb8() {
    // IDA 0x7f9bb8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Soundscape::CollisionSoundManager::~CollisionSoundManager()")]
#[doc(alias = "__ZN3RBX10Soundscape21CollisionSoundManagerD1Ev")]
// 0x7f9ce8 — __ZN3RBX10Soundscape21CollisionSoundManagerD1Ev
// type: void __fastcall(RBX::Soundscape::CollisionSoundManager *__hidden this)
pub fn stub_0x7f9ce8() {
    // IDA 0x7f9ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
