// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xf2b1a4..0xf30024 | 5017->5117 covered (filtered), 284 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEE9singletonEv")]
pub fn stub_0xf2b1a4() -> crate::slot::PortedFn {
// IDA 0xf2b1a4: j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEE9singletonEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b1a4, "j___ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEE9singletonEv")
}

#[doc(alias = "RBX::Lua::LuaArguments::getValues(lua_State *) [0xf2b1c4]")]
pub fn stub_0xf2b1c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Lua::LuaArguments getter.
cell.get()
}

#[doc(alias = "RBX::Lua::ObjectBridge::registerInstanceClassLibrary(lua_State *) [0xf2b1d4]")]
pub fn stub_0xf2b1d4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// ObjectBridge::registerInstanceClassLibrary —
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::registerClassLibrary(lua_State *) [0xf2b1e4]")]
pub fn stub_0xf2b1e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::push(lua_State *,rbx_core::SharedPtr<RBX::Instance>) [0xf2b1f4]")]
pub fn stub_0xf2b1f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Lua::AllEnumDescriptors const*,true>::registerClassLibrary(lua_State *) [0xf2b204]")]
pub fn stub_0xf2b204(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2b204: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Lua::AllEnumDescriptors const*,true>::push(lua_State *,RBX::Lua::AllEnumDescriptors const*) [0xf2b214]")]
pub fn stub_0xf2b214(thread: &mut crate::lua::LuaThreadState, desc: &crate::lua::LuaEnumDescriptor) -> i32 {
// Bridge::push for enum descriptors — pushes one EnumItem
// userdata per value and returns the count.
for value in desc.values.clone() {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: desc.name.clone(), value }) }));
}
desc.values.len() as i32
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor::Item const*,true>::registerClassLibrary(lua_State *) [0xf2b224]")]
pub fn stub_0xf2b224(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2b224: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor::Item const*,true>::push(lua_State *,RBX::Reflection::EnumDescriptor::Item const*) [0xf2b234]")]
pub fn stub_0xf2b234(thread: &mut crate::lua::LuaThreadState, desc: &crate::lua::LuaEnumDescriptor) -> i32 {
// Bridge::push for enum descriptors — pushes one EnumItem
// userdata per value and returns the count.
for value in desc.values.clone() {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: desc.name.clone(), value }) }));
}
desc.values.len() as i32
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor const*,true>::registerClassLibrary(lua_State *) [0xf2b244]")]
pub fn stub_0xf2b244(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0xf2b244: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor const*,true>::push(lua_State *,RBX::Reflection::EnumDescriptor const*) [0xf2b254]")]
pub fn stub_0xf2b254(thread: &mut crate::lua::LuaThreadState, desc: &crate::lua::LuaEnumDescriptor) -> i32 {
// Bridge::push for enum descriptors — pushes one EnumItem
// userdata per value and returns the count.
for value in desc.values.clone() {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: desc.name.clone(), value }) }));
}
desc.values.len() as i32
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>* RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Instance>,false>::pushNewObject<rbx_core::SharedPtr<RBX::Instance>>(lua_State *,rbx_core::SharedPtr<RBX::Instance>) [0xf2b264]")]
pub fn stub_0xf2b264() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::AllEnumDescriptors const** RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::pushNewObject<RBX::Lua::AllEnumDescriptors const*>(lua_State *,RBX::Lua::AllEnumDescriptors const*) [0xf2b274]")]
pub fn stub_0xf2b274(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::AllEnumDescriptors const** RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::Item const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor::Item const*,true>::pushNewObject<RBX::Reflection::EnumDescriptor::Item const*>(lua_State *,RBX::Reflection::EnumDescriptor::Item const*) [0xf2b284]")]
pub fn stub_0xf2b284(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaEnumItem) -> crate::lua::LuaEnumItem {
// Bridge<EnumItem>::pushNewObject.
thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(value.clone()) }));
value.clone()
}

#[doc(alias = "RBX::Reflection::EnumDescriptor const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor const*,true>::pushNewObject<RBX::Reflection::EnumDescriptor const*>(lua_State *,RBX::Reflection::EnumDescriptor const*) [0xf2b294]")]
pub fn stub_0xf2b294(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor const** RBX::Lua::Bridge<RBX::Reflection::EnumDescriptor c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::ThreadRef::ThreadRef(lua_State *) [0xf2b2a4]")]
pub fn stub_0xf2b2a4() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sLocalScriptEEEERKS0_v")]
pub fn stub_0xf2b2c4() -> crate::slot::PortedFn {
// IDA 0xf2b2c4: j___ZN3RBX4Name7declareILZNS_12sLocalScriptEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b2c4, "j___ZN3RBX4Name7declareILZNS_12sLocalScriptEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v")]
pub fn stub_0xf2b304() -> crate::slot::PortedFn {
// IDA 0xf2b304: j___ZN3RBX4Name7declareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b304, "j___ZN3RBX4Name7declareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v")]
pub fn stub_0xf2b314() -> crate::slot::PortedFn {
// IDA 0xf2b314: j___ZN3RBX4Name7declareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b314, "j___ZN3RBX4Name7declareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sLocalScriptEEEERKS0_v")]
pub fn stub_0xf2b334() -> crate::slot::PortedFn {
// IDA 0xf2b334: j___ZN3RBX4Name9doDeclareILZNS_12sLocalScriptEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b334, "j___ZN3RBX4Name9doDeclareILZNS_12sLocalScriptEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sScriptContextEEEERKS0_v")]
pub fn stub_0xf2b354() -> crate::slot::PortedFn {
// IDA 0xf2b354: j___ZN3RBX4Name9doDeclareILZNS_14sScriptContextEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b354, "j___ZN3RBX4Name9doDeclareILZNS_14sScriptContextEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_21sRuntimeScriptServiceEEEERKS0_v")]
pub fn stub_0xf2b364() -> crate::slot::PortedFn {
// IDA 0xf2b364: j___ZN3RBX4Name9doDeclareILZNS_21sRuntimeScriptServiceEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b364, "j___ZN3RBX4Name9doDeclareILZNS_21sRuntimeScriptServiceEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v")]
pub fn stub_0xf2b394() -> crate::slot::PortedFn {
// IDA 0xf2b394: j___ZN3RBX4Name9doDeclareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b394, "j___ZN3RBX4Name9doDeclareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v")]
pub fn stub_0xf2b3a4() -> crate::slot::PortedFn {
// IDA 0xf2b3a4: j___ZN3RBX4Name9doDeclareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2b3a4, "j___ZN3RBX4Name9doDeclareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v")
}

#[doc(alias = "RBX::GcJob::GcJob(rbx_core::SharedPtr<RBX::ScriptContext>)")]
pub fn stub_0xf2b3b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreScript> RBX::Creatable<RBX::Instance>::create<RBX::CoreScript,RBX::ContentId>(RBX::ContentId) [0xf2b404]")]
pub fn stub_0xf2b404() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript> RBX::Creatable<RBX::Instance>::create<RBX::LocalScript>(void) [0xf2b414]")]
pub fn stub_0xf2b414() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings> RBX::Creatable<RBX::Instance>::create<RBX::LuaSettings>(void) [0xf2b424]")]
pub fn stub_0xf2b424() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::LuaStatsItem,RBX::ScriptContext *>(RBX::ScriptContext *)")]
pub fn stub_0xf2b434() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaStatsItem")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext> RBX::Creatable<RBX::Instance>::create<RBX::ScriptContext>(void) [0xf2b454]")]
pub fn stub_0xf2b454() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterScript> RBX::Creatable<RBX::Instance>::create<RBX::StarterScript,RBX::ContentId>(RBX::ContentId) [0xf2b464]")]
pub fn stub_0xf2b464() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch>(void) [0xf2b494]")]
pub fn stub_0xf2b494() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger>(void) [0xf2b4a4]")]
pub fn stub_0xf2b4a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::ScriptDebugger(void) [0xf2b4e4]")]
pub fn stub_0xf2b4e4() -> crate::slot::InstanceHandle {
// RBX::Scripting::ScriptDebugger ctor — fresh debugger identity.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "rbx::safe_queue<RBX::ScriptContext::WaitingThread>::pop_if_present(RBX::ScriptContext::WaitingThread&) [0xf2b4f4]")]
pub fn stub_0xf2b4f4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "rbx::safe_queue<RBX::ScriptContext::WaitingThread>::push(RBX::ScriptContext::WaitingThread const&) [0xf2b504]")]
pub fn stub_0xf2b504(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "rbx::safe_queue<RBX::ScriptContext::WaitingThread>::clear(void) [0xf2b514]")]
pub fn stub_0xf2b514(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(lua_State *)>::operator()(lua_State *) [0xf2b5a4]")]
pub fn stub_0xf2b5a4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1,void ()(lua_State *)>::operator()(lua_State *) [0xf2b5a4] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>> const&) [0xf2b604]")]
pub fn stub_0xf2b604() -> crate::slot::SlotConnection {
// IDA 0xf2b604: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&) [0xf2b694]")]
pub fn stub_0xf2b694() -> crate::slot::SlotConnection {
// IDA 0xf2b694: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_do_get_mutex(void) [0xf2b6c4]")]
pub fn stub_0xf2b6c4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void ()(lua_State *)>::safe_static_do_get_mutex(void) [0xf2b6c4] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> &) [0xf2b6d4]")]
pub fn stub_0xf2b6d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void ()(lua_State *)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::on_error(std::exception &) [0xf2b6e4]")]
pub fn stub_0xf2b6e4(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> const&) [0xf2b734]")]
pub fn stub_0xf2b734() -> crate::slot::SlotConnection {
// IDA 0xf2b734: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr() [0xf2b864]")]
pub fn stub_0xf2b864() -> crate::slot::PortedFn {
// IDA 0xf2b864: boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr() [0xf2b864].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2b864, "boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr() [0xf2b864]")
}

#[doc(alias = "boost::scoped_ptr<RBX::Lua::Continuations>::reset(RBX::Lua::Continuations*) [0xf2b884]")]
pub fn stub_0xf2b884() -> crate::slot::PortedFn {
// IDA 0xf2b884: boost::scoped_ptr<RBX::Lua::Continuations>::reset(RBX::Lua::Continuations*) [0xf2b884].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2b884, "boost::scoped_ptr<RBX::Lua::Continuations>::reset(RBX::Lua::Continuations*) [0xf2b884]")
}

#[doc(alias = "boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr() [0xf2b894]")]
pub fn stub_0xf2b894() -> crate::slot::PortedFn {
// IDA 0xf2b894: boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr() [0xf2b894].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2b894, "boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr() [0xf2b894]")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript>::operator=(rbx_core::SharedPtr<RBX::BaseScript> const&) [0xf2b8e4]")]
pub fn stub_0xf2b8e4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreScript>::shared_ptr<RBX::CoreScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2b8f4]")]
pub fn stub_0xf2b8f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript>::shared_ptr<RBX::LocalScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2b914]")]
pub fn stub_0xf2b914() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaSettings>::shared_ptr<RBX::LuaSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2b924]")]
pub fn stub_0xf2b924() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaSettings")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::ScriptStats>::reset<RBX::ScriptStats>(RBX::ScriptStats *) [0xf2b934]")]
pub fn stub_0xf2b934() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptStats")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *) [0xf2b944]")]
pub fn stub_0xf2b944() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptStats")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::operator=(rbx_core::SharedPtr<RBX::ScriptStats> const&) [0xf2b954]")]
pub fn stub_0xf2b954(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaStatsItem>::shared_ptr<RBX::LuaStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0xf2b964() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaStatsItem")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaStatsItem>::operator=(rbx_core::SharedPtr<RBX::LuaStatsItem> const&) [0xf2b974]")]
pub fn stub_0xf2b974(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext>::shared_ptr<RBX::ScriptContext>(rbx_core::WeakPtr<RBX::ScriptContext> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0xf2b994() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterScript>::shared_ptr<RBX::StarterScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2b9a4]")]
pub fn stub_0xf2b9a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *) [0xf2b9b4]")]
pub fn stub_0xf2b9b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::WaitingScriptsJob")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&) [0xf2b9f4]")]
pub fn stub_0xf2b9f4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch>::shared_ptr<RBX::Scripting::DebuggerWatch,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2ba24]")]
pub fn stub_0xf2ba24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger>::shared_ptr<RBX::Scripting::ScriptDebugger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter) [0xf2ba34]")]
pub fn stub_0xf2ba34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>::operator=(RBX::Lua::WeakThreadRef::Node*) [0xf2baa4]")]
pub fn stub_0xf2baa4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef>::operator=(rbx_core::SharedPtr<RBX::Lua::detail::LiveThreadRef> const&)")]
pub fn stub_0xf2bab4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> const&) [0xf2bb04]")]
pub fn stub_0xf2bb04(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef::Node,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef::Node,int,0> const*) [0xf2bbb4]")]
pub fn stub_0xf2bbb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef::Node")
}

#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::detail::LiveThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::detail::LiveThreadRef,int,0> const*)")]
pub fn stub_0xf2bbc4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::detail::LiveThreadRef")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>) [0xf2bbe4]")]
pub fn stub_0xf2bbe4() -> crate::slot::BindPiece {
// boost::bind fragment (list1) composing a host BoundCall.
crate::slot::BindPiece::new("list1")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int) [0xf2bbf4]")]
pub fn stub_0xf2bbf4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2bbf4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int) [0xf2bc04]")]
pub fn stub_0xf2bc04(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2bc04: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::operator()<void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>) &,boost::_bi::list1<lua_State *&> &,int) [0xf2bc24]")]
pub fn stub_0xf2bc24(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>) [0xf2bc34]")]
pub fn stub_0xf2bc34() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::operator()<void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>) &,boost::_bi::list1<lua_State *&> &,int) [0xf2bc44]")]
pub fn stub_0xf2bc44(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>) [0xf2bc54]")]
pub fn stub_0xf2bc54() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *> &,boost::_bi::list1<lua_State *&> &,int) [0xf2bc64]")]
pub fn stub_0xf2bc64(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2bc64: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>) [0xf2bc74]")]
pub fn stub_0xf2bc74() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list1<RBX::BaseScript * const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions> &,boost::_bi::list1<RBX::BaseScript * const&> &,int) [0xf2bc84]")]
pub fn stub_0xf2bc84(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2bc84: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::operator()<void (*)(lua_State *,int,std::string),boost::_bi::list2<lua_State *&,unsigned long &>>(boost::_bi::type<void>,void (*)(lua_State *,int,std::string) &,boost::_bi::list2<lua_State *&,unsigned long &> &,int) [0xf2bca4]")]
pub fn stub_0xf2bca4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2bca4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int) [0xf2bcd4]")]
pub fn stub_0xf2bcd4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2bcd4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>::operator()(void) [0xf2bce4]")]
pub fn stub_0xf2bce4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&) [0xf2bcf4]")]
pub fn stub_0xf2bcf4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::storage3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>) [0xf2bd04]")]
pub fn stub_0xf2bd04() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list_av_3<RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>>::type> boost::bind<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *,RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>>(void (RBX::ScriptContext::*)(RBX::Lua::WeakThreadRef,lua_State *),RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>) [0xf2bd34]")]
pub fn stub_0xf2bd34() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::ScriptContext>>::type> boost::bind<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::ScriptContext>>(void (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<RBX::ScriptContext>) [0xf2bd44]")]
pub fn stub_0xf2bd44() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list_av_3<RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>::type> boost::bind<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions,RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>(void (RBX::ScriptContext::*)(RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions),RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions) [0xf2bd54]")]
pub fn stub_0xf2bd54() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list_av_2<boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::type> boost::bind<void,lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>(void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>) [0xf2bd64]")]
pub fn stub_0xf2bd64() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list_av_2<boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>::type> boost::bind<void,lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>,boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>(void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>) [0xf2bd74]")]
pub fn stub_0xf2bd74() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list_av_3<boost::arg<1>,boost::arg<2>,std::string>::type> boost::bind<void,lua_State *,int,std::string,boost::arg<1>,boost::arg<2>,std::string>(void (*)(lua_State *,int,std::string),boost::arg<1>,boost::arg<2>,std::string) [0xf2bd84]")]
pub fn stub_0xf2bd84() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *) [0xf2bda4]")]
pub fn stub_0xf2bda4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *) [0xf2bdb4]")]
pub fn stub_0xf2bdb4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS7_5list5INS7_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ff94() -> crate::slot::PortedFn {
// IDA 0xf2ff94: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvNS1_25ScriptInformationProvider13R~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ff94, "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvNS1_25ScriptIn~")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptInformationProvider>::weak_ptr<RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptInformationProvider,RBX::ScriptInformationProvider>::type) [0xf2ffa4]")]
pub fn stub_0xf2ffa4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>) [0xf2ffb4]")]
pub fn stub_0xf2ffb4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ffc4() -> crate::slot::PortedFn {
// IDA 0xf2ffc4: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProv~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ffc4, "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25S~")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>) [0xf2ffd4]")]
pub fn stub_0xf2ffd4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2ffe4() -> crate::slot::PortedFn {
// IDA 0xf2ffe4: j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2ffe4, "j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bin~")
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to_own(boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool> const&) [0xf2fff4]")]
pub fn stub_0xf2fff4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::clear(void) [0xf30004]")]
pub fn stub_0xf30004(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *) [0xf30014]")]
pub fn stub_0xf30014(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long) [0xf30024]")]
pub fn stub_0xf30024(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}
