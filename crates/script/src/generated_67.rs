// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x29c588..0x2a47b0 | remaining 3770 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::ScriptContext::setThreadIdentity(lua_State *,RBX::Security::Identities,rbx_core::SharedPtr<RBX::BaseScript>)")]
pub fn stub_0x29c588() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

// 0x29c624 — __ZNK3RBX13ScriptContext14getThreadCountEv
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this)
#[doc(alias = "RBX::ScriptContext::getThreadCount(void)const")]
pub fn stub_0x29c624(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::getThreadCount(void)const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::getGlobalState(lua_State *)")]
pub fn stub_0x29c644(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::getGlobalState(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::call(RBX::Lua::WeakFunctionRef &)")]
pub fn stub_0x29c69c(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::call(RBX::Lua::WeakFunctionRef&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::call(RBX::Lua::WeakFunctionRef &,boost::function1<unsigned long,lua_State *>,boost::function2<void,lua_State *,unsigned long>)")]
pub fn stub_0x29c798(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::call(RBX::Lua::WeakFunctionRef&, boost::function1<unsigned long, lua_S~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "pushNoArguments(lua_State *)")]
pub fn stub_0x29cad4() -> crate::slot::PortedFn {
// IDA 0x29cad4: pushNoArguments(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x29cad4, "pushNoArguments(lua_State*)")
}

// 0x29cad8 — __ZN3RBX13ScriptContext18executeInNewThreadENS_8Security10IdentitiesERKNS_15ProtectedStringEPKc
// type: int __fastcall(int, int, RBX::ProtectedString *)
// was: int __fastcall(int, int, RBX::ProtectedString *)
#[doc(alias = "RBX::ScriptContext::executeInNewThread(RBX::Security::Identities,RBX::ProtectedString const&,char const*)")]
pub fn stub_0x29cad8(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::executeInNewThread(RBX::Security::Identities,RBX::ProtectedString cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x29cd04 — __ZN3RBX13ScriptContext18executeInNewThreadENS_8Security10IdentitiesENS_15ProtectedStringEPKcN5boost9function1ImP9lua_StateEENS6_9function2IvS9_mEENS_7Scripts13ContinuationsES9_
// type: int __fastcall(int, int, int, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ScriptContext::executeInNewThread(RBX::Security::Identities,RBX::ProtectedString,char const*,boost::function1<unsigned long,lua_State *>,boost::function2<void,lua_State *,unsigned long>,RBX::Scripts::Continuations,lua_State *)")]
pub fn stub_0x29cd04(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::executeInNewThread(RBX::Security::Identities,RBX::ProtectedString,char~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::call(RBX::Lua::WeakFunctionRef &,RBX::Reflection::Tuple const&)")]
pub fn stub_0x29db1c(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::call(RBX::Lua::WeakFunctionRef&, RBX::Reflection::Tuple const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x29dc5c — __ZL11readResultsRSt8auto_ptrIN3RBX10Reflection5TupleEEP9lua_Statem
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
// was: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "readResults(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long)")]
pub fn stub_0x29dc5c() -> crate::slot::PortedFn {
// IDA 0x29dc5c: readResults(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x29dc5c, "readResults(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long)")
}

#[doc(alias = "RBX::Lua::Continuations::Continuations(RBX::Scripts::Continuations const&)")]
pub fn stub_0x29dd48() -> crate::slot::InstanceHandle {
// RBX::Lua::Continuations ctor.
crate::slot::InstanceHandle::new("RBX::Lua::Continuations")
}

// 0x29df1c — __ZN3RBX3Lua13Continuations16onSuccessHandlerEP9lua_StateN5boost8functionIFvNS4_10shared_ptrIKNS_10Reflection5TupleEEEEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Lua::Continuations::onSuccessHandler(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>)")]
pub fn stub_0x29df1c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Lua::Continuations::onErrorHandler(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>)")]
pub fn stub_0x29e080() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

// 0x29e2a8 — __ZN3RBX13ScriptContext16extractCallStackEP9lua_StateRN5boost10shared_ptrINS_10BaseScriptEEERi
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// was: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::ScriptContext::extractCallStack(lua_State *,rbx_core::SharedPtr<RBX::BaseScript> &,int &)")]
pub fn stub_0x29e2a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

// 0x29e5dc — __ZN3RBX13ScriptContext10addLibraryERKSsRKNS_15ProtectedStringE
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this, const std::string *, const RBX::ProtectedString *)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this, const std::string *, const RBX::ProtectedString *)
#[doc(alias = "RBX::ScriptContext::addLibrary(std::string const&,RBX::ProtectedString const&)")]
pub fn stub_0x29e5dc(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::addLibrary(std::string const&,RBX::ProtectedString const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::resume(RBX::Lua::ThreadRef,boost::function1<unsigned long,lua_State *>,boost::function2<void,lua_State *,unsigned long>)")]
pub fn stub_0x29e978(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

// 0x29ecbc — __ZN3RBX13ScriptContext6resumeENS_3Lua9ThreadRefEi
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::ScriptContext::resume(RBX::Lua::ThreadRef,int)")]
pub fn stub_0x29ecbc(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

// 0x29f144 — __ZN3RBX13ScriptContext17on_ypcall_successENS_3Lua13WeakThreadRefEP9lua_State
// type: int __fastcall(char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
// was: int __fastcall(char, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ScriptContext::on_ypcall_success(RBX::Lua::WeakThreadRef,lua_State *)")]
pub fn stub_0x29f144(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

// 0x29f378 — __ZN3RBX13ScriptContext17on_ypcall_failureENS_3Lua13WeakThreadRefEP9lua_State
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int)
// was: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ScriptContext::on_ypcall_failure(RBX::Lua::WeakThreadRef,lua_State *)")]
pub fn stub_0x29f378(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x29f5ac(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::onHeartbeat(RBX::Heartbeat const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x29f668 — __ZN3RBX13ScriptContext19startPendingScriptsEv
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this)
#[doc(alias = "RBX::ScriptContext::startPendingScripts(void)")]
pub fn stub_0x29f668(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::startPendingScripts(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::resumeWaitingScripts(RBX::Time)")]
pub fn stub_0x29f900(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::resumeWaitingScripts(RBX::Time) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x29fcc0 — __ZN3RBX13ScriptContext14resumeWithArgsENS_3Lua9ThreadRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// type: int __fastcall(char, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
// was: int __fastcall(char, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::ScriptContext::resumeWithArgs(RBX::Lua::ThreadRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_0x29fcc0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x29fe90 — __ZN3RBX13ScriptContext6stepGcEv
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this)
#[doc(alias = "RBX::ScriptContext::stepGc(void)")]
pub fn stub_0x29fe90(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::stepGc(void) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::startScript(RBX::ScriptContext::ScriptStart)")]
pub fn stub_0x29ff98(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::startScript(RBX::ScriptContext::ScriptStart) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a10b4 — __ZN3RBX13ScriptContext17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::ScriptContext::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x2a10b4(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a1430 — __ZN3RBX13ScriptContext15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::ScriptContext::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x2a1430(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::scriptShouldRun(RBX::BaseScript *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a146c — __ZThn96_N3RBX13ScriptContext15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
#[doc(alias = "non-virtual thunk toRBX::ScriptContext::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x2a146c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0x2a1478 — __ZN3RBX13ScriptContext9addScriptEPNS_10BaseScriptENS0_18ScriptStartOptionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ScriptContext::addScript(RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions)")]
pub fn stub_0x2a1478(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::addScript(RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a16d4 — __ZN3RBX13ScriptContext17disassociateStateEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::ScriptContext::disassociateState(RBX::BaseScript *)")]
pub fn stub_0x2a16d4(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::disassociateState(RBX::BaseScript *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::eraseScript(std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>> &,RBX::BaseScript *)")]
pub fn stub_0x2a1784(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

// 0x2a17a0 — __ZN3RBX13ScriptContext12removeScriptEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
// was: _DWORD __fastcall(RBX::ScriptContext *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::ScriptContext::removeScript(RBX::BaseScript *)")]
pub fn stub_0x2a17a0(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::removeScript(RBX::BaseScript *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a1854 — __ZN3RBX13ScriptContext14printCallStackEP9lua_StatePSs
// type: int __fastcall(_DWORD, _DWORD)
// was: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::ScriptContext::printCallStack(lua_State *,std::string *)")]
pub fn stub_0x2a1854(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::printCallStack(lua_State *,std::string *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a1d3c — __ZN3RBX13ScriptContext11reportErrorEP9lua_State
// type: int __fastcall(int, int)
// was: int __fastcall(int, int)
#[doc(alias = "RBX::ScriptContext::reportError(lua_State *)")]
pub fn stub_0x2a1d3c(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::reportError(lua_State *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a279c — __ZN3RBX13ScriptContext14scheduleResumeENS_3Lua9ThreadRefEN5boost10shared_ptrIKNS_10Reflection5TupleEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptContext::scheduleResume(RBX::Lua::ThreadRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_0x2a279c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

// 0x2a2964 — __ZN3RBX20RuntimeScriptService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
// was: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::RuntimeScriptService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x2a2964(handle: &crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RuntimeScriptService::onRunState(RBX::RunState)")]
pub fn stub_0x2a2a90(handle: &crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService::onRunState(RBX::RunState) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a2ef0 — __ZN3RBX20RuntimeScriptService9runScriptEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this, RBX::BaseScript *)
// was: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::RuntimeScriptService::runScript(RBX::BaseScript *)")]
pub fn stub_0x2a2ef0(handle: &crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService::runScript(RBX::BaseScript *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x2a3344 — __ZN3RBX20RuntimeScriptService13releaseScriptEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this, RBX::BaseScript *)
// was: _DWORD __fastcall(RBX::RuntimeScriptService *__hidden this, RBX::BaseScript *)
#[doc(alias = "RBX::RuntimeScriptService::releaseScript(RBX::BaseScript *)")]
pub fn stub_0x2a3344(handle: &crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService::releaseScript(RBX::BaseScript *) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "illegal(lua_State *)")]
pub fn stub_0x2a36f8() -> crate::slot::PortedFn {
// IDA 0x2a36f8: illegal(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2a36f8, "illegal(lua_State*)")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
pub fn stub_0x2a3818(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x2a383c — __ZN11LuaProfiler11StringCacheD1Ev
// type: void __fastcall(LuaProfiler::StringCache *__hidden this)
// was: void __fastcall(LuaProfiler::StringCache *__hidden this)
#[doc(alias = "LuaProfiler::StringCache::~StringCache()")]
pub fn stub_0x2a383c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_0x2a3858(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
pub fn stub_0x2a3898(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_0x2a38e0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x2a39e4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(int,rbx_core::SharedPtr<RBX::Instance>,std::string),3>::~BoundFuncDesc()")]
pub fn stub_0x2a3a08(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(double),1>::~BoundFuncDesc()")]
pub fn stub_0x2a3b20(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_0x2a3b60(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x2a3ba0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptContext,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_0x2a3be8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ScriptContext,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::ScriptContext::*>::~EventDesc()")]
pub fn stub_0x2a3c28(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "LuaProfiler::hookCall(lua_State *,lua_Debug *)")]
pub fn stub_0x2a3c4c() -> crate::slot::PortedFn {
// IDA 0x2a3c4c: LuaProfiler::hookCall(lua_State*, lua_Debug*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2a3c4c, "LuaProfiler::hookCall(lua_State*, lua_Debug*)")
}

// 0x2a3ce8 — __ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
// was: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEE9singletonEv")]
pub fn stub_0x2a3ce8() -> crate::slot::InstanceHandle {
// settings-item ctor.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettingsItem")
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_index(lua_State *)")]
pub fn stub_0x2a3e8c(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::Bridge<RBX::Lua::Library, true>::on_index(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a3ec0(key: &str) -> ! {
// Bridge<Library>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_index(lua_State *)")]
pub fn stub_0x2a3ef4(value: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// SharedPtrBridge<Instance>::on_index — reflection property
// lookup through the instance bag; missing members push nil.
let _ = value;
if key == "Connect" || key == "Wait" {
    thread.push(crate::lua::LuaStackValue::Function(crate::lua::method_fn_id(key)));
} else {
    panic!("{key} is not a valid member");
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::EventInstance,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a3f28(handle: &Option<rbx_core::SharedPtr<crate::lua::LuaInstanceHandle>>, key: &str, value: &crate::lua::LuaStackValue, bag: &mut std::collections::HashMap<String, crate::lua::ScriptVariant>) -> i32 {
// SharedPtrBridge<Instance>::on_newindex — routes the
// property set through reflection; the host records the
// converted variant in the instance bag.
let _ = handle;
match crate::lua::stack_value_to_variant(value, false) {
    Some(variant) => { bag.insert(key.to_owned(), variant); }
    None => { bag.remove(key); }
}
0
}

#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_index(lua_State *)")]
pub fn stub_0x2a3f5c() -> crate::slot::SlotConnection {
// IDA 0x2a3f5c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a3f90() -> crate::slot::SlotConnection {
// IDA 0x2a3f90: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_index(lua_State *)")]
pub fn stub_0x2a3fc4(value: &crate::lua::LuaCoordinateFrame, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CoordinateFrame>::on_index: X/Y/Z lanes, Position,
// method closures; else invalid member.
if key == "X" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.position.x)));
} else if key == "Y" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.position.y)));
} else if key == "Z" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.position.z)));
} else if key == "Position" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR3, crate::lua::LuaUserdataPayload::Vector3(value.position));
} else if matches!(key, "inverse" | "lerp" | "toWorldSpace" | "toObjectSpace" | "pointToWorldSpace" | "pointToObjectSpace" | "vectorToWorldSpace" | "vectorToObjectSpace" | "components" | "toEulerAnglesXYZ") {
    thread.push(crate::lua::LuaStackValue::Function(crate::lua::method_fn_id(key)));
} else {
    panic!("{key} is not a valid member");
}
1
}

#[doc(alias = "RBX::Lua::Bridge<G3D::CoordinateFrame,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a3ff8(key: &str) -> ! {
// Bridge<CoordinateFrame>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_index(lua_State *)")]
pub fn stub_0x2a402c(value: &crate::lua::LuaRegion3, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x270d8c — same class dispatch.
crate::lua::stub_0x270d8c(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4060(key: &str) -> ! {
// Overload of the primary stub_0x270ec8 — read-only members.
crate::lua::stub_0x270ec8(key)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_index(lua_State *)")]
pub fn stub_0x2a4094(value: &crate::lua::LuaRegion3i16, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x2710a0 — same class dispatch.
crate::lua::stub_0x2710a0(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a40c8(key: &str) -> ! {
// Overload of the primary stub_0x2711d4 — read-only members.
crate::lua::stub_0x2711d4(key)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(lua_State *)")]
pub fn stub_0x2a40fc(value: &crate::lua::LuaVector3i16, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x272268 — same class dispatch.
crate::lua::stub_0x272268(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4130(key: &str) -> ! {
// Overload of the primary stub_0x2723d0 — read-only members.
crate::lua::stub_0x2723d0(key)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(lua_State *)")]
pub fn stub_0x2a4164(value: &crate::lua::LuaVector2i16, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x272804 — same class dispatch.
crate::lua::stub_0x272804(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4198(key: &str) -> ! {
// Overload of the primary stub_0x272940 — read-only members.
crate::lua::stub_0x272940(key)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(lua_State *)")]
pub fn stub_0x2a41cc(value: &crate::lua::LuaVector3, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x271954 — same class dispatch.
crate::lua::stub_0x271954(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4200(key: &str) -> ! {
// Overload of the primary stub_0x271e14 — read-only members.
crate::lua::stub_0x271e14(key)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_index(lua_State *)")]
pub fn stub_0x2a4234(value: &crate::lua::LuaVector2, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x272d70 — same class dispatch.
crate::lua::stub_0x272d70(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4268(key: &str) -> ! {
// Overload of the primary stub_0x272fe4 — read-only members.
crate::lua::stub_0x272fe4(key)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_index(lua_State *)")]
pub fn stub_0x2a429c(value: &crate::lua::LuaRbxRay, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x2708ec — same class dispatch.
crate::lua::stub_0x2708ec(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a42d0(key: &str) -> ! {
// Overload of the primary stub_0x270b98 — read-only members.
crate::lua::stub_0x270b98(key)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(lua_State *)")]
pub fn stub_0x2a4304(value: &crate::lua::LuaColor3, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x2705ec — same class dispatch.
crate::lua::stub_0x2705ec(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4338(key: &str) -> ! {
// Overload of the primary stub_0x270724 — read-only members.
crate::lua::stub_0x270724(key)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_index(lua_State *)")]
pub fn stub_0x2a436c(value: &crate::lua::LuaBrickColor, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Overload of the primary stub_0x2733b0 — same class dispatch.
crate::lua::stub_0x2733b0(value, key, thread)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::BrickColor,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a43a0(key: &str) -> ! {
// Overload of the primary stub_0x2735bc — read-only members.
crate::lua::stub_0x2735bc(key)
}

#[doc(alias = "RBX::Lua::Bridge<RBX::UDim,true>::on_index(lua_State *)")]
pub fn stub_0x2a43d4(value: &crate::lua::LuaUDim, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<UDim>::on_index: Scale, Offset; else invalid member.
if key == "Scale" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.scale)));
} else if key == "Offset" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.offset)));
} else {
    panic!("{key} is not a valid member");
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::UDim,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4408(key: &str) -> ! {
// Bridge<UDim>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::UDim2,true>::on_index(lua_State *)")]
pub fn stub_0x2a443c(value: &crate::lua::LuaUDim2, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<UDim2>::on_index: X, Y, Width, Height; else invalid.
if key == "X" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(value.x));
} else if key == "Y" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::UDIM, crate::lua::LuaUserdataPayload::UDim(value.y));
} else if key == "Width" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.x.offset)));
} else if key == "Height" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.y.offset)));
} else {
    panic!("{key} is not a valid member");
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::UDim2,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4470(key: &str) -> ! {
// Bridge<UDim2>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_index(lua_State *)")]
pub fn stub_0x2a44a4(value: &crate::lua::LuaFaces, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Faces>::on_index: per-face NormalId items
// (Right=0, Top=1, Back=2, Left=3, Bottom=4, Front=5, cf.
// normalIdToVector3, 0x35d1e8); else invalid member.
let bit = match key { "Right" => 0, "Top" => 1, "Back" => 2, "Left" => 3, "Bottom" => 4, "Front" => 5, _ => panic!("{key} is not a valid member"), };
thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "NormalId".to_owned(), value: bit }) }));
let _ = value;
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Faces,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a44d8(key: &str) -> ! {
// Bridge<Faces>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_index(lua_State *)")]
pub fn stub_0x2a450c(value: &crate::lua::LuaAxes, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<Axes>::on_index: X/Y/Z presence flags; else invalid.
let bit = match key { "X" => 1u8, "Y" => 2u8, "Z" => 4u8, _ => panic!("{key} is not a valid member"), };
thread.push(crate::lua::LuaStackValue::Bool(value.bits & bit != 0));
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Axes,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4540(key: &str) -> ! {
// Bridge<Axes>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_index(lua_State *)")]
pub fn stub_0x2a4574(value: &crate::lua::LuaCellId, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<CellID>::on_index: X/Y/Z lanes; else invalid member.
if key == "X" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.x)));
} else if key == "Y" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.y)));
} else if key == "Z" {
    thread.push(crate::lua::LuaStackValue::Number(f64::from(value.z)));
} else {
    panic!("{key} is not a valid member");
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::CellID,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a45a8(key: &str) -> ! {
// Bridge<CellID>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_index(lua_State *)")]
pub fn stub_0x2a45dc(value: &crate::lua::LuaInputObject, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<InputObject>::on_index: KeyCode/UserInputType enums,
// Position/Delta vectors, UserInputState; else invalid.
if key == "KeyCode" {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "KeyCode".to_owned(), value: value.kind as i32 }) }));
} else if key == "UserInputType" {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "UserInputType".to_owned(), value: value.kind as i32 }) }));
} else if key == "UserInputState" {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: "UserInputState".to_owned(), value: 0 }) }));
} else if key == "Position" || key == "Delta" {
    crate::lua::push_new_object(thread, crate::lua::lua_bridge_class::VECTOR3, crate::lua::LuaUserdataPayload::Vector3(crate::lua::LuaVector3 { x: 0.0, y: 0.0, z: 0.0 }));
} else {
    panic!("{key} is not a valid member");
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::InputObject,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4610(key: &str) -> ! {
// Bridge<InputObject>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_index(lua_State *)")]
pub fn stub_0x2a4644() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef::Node")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4678() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef::Node")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_index(lua_State *)")]
pub fn stub_0x2a46ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a46e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_index(lua_State *)")]
pub fn stub_0x2a4714() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(RBX::Lua::IAsyncResult *)>)>>,true>::on_newindex(lua_State *)")]
pub fn stub_0x2a4748() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<void (boost::shared_ptr<RBX::Reflection::Tuple const>, boost::fu~")
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Lua::AllEnumDescriptors const*,true>::registerClassLibrary(lua_State *)")]
pub fn stub_0x2a477c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x2a477c: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor const*,true>::registerClassLibrary(lua_State *)")]
pub fn stub_0x2a47b0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x2a47b0: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}
