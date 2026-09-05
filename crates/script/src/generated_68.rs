// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2a47e4..0x2a9dac | remaining 3670 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Reflection::EnumDescriptor::Item const*,true>::registerClassLibrary(lua_State *)")]
pub fn stub_0x2a47e4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// IDA 0x2a47e4: luaL_register(L, className, classLibrary) (cf.
// 0x2708b0) + setreadonly + pop. Host no-op; no values returned.
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::registerClassLibrary(lua_State *)")]
pub fn stub_0x2a4818() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Lua::ObjectBridge::registerInstanceClassLibrary(lua_State *)")]
pub fn stub_0x2a4854(thread: &mut crate::lua::LuaThreadState) -> i32 {
// ObjectBridge::registerInstanceClassLibrary —
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

#[doc(alias = "RBX::Lua::SharedPtrBridge<RBX::Instance>::push(lua_State *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2a4890() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext> RBX::shared_from<RBX::ScriptContext>(RBX::ScriptContext*)")]
pub fn stub_0x2a4ab4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "rbx::safe_queue<RBX::ScriptContext::WaitingThread>::clear(void)")]
pub fn stub_0x2a4ba0(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_0x2a4ca0() -> crate::slot::SlotConnection {
// IDA 0x2a4ca0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::ScriptContext>>::type> boost::bind<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::ScriptContext>>(void (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<RBX::ScriptContext>)")]
pub fn stub_0x2a4ebc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "RBX::Lua::LuaArguments::pushTuple(RBX::Reflection::Tuple const&,lua_State *)")]
pub fn stub_0x2a4fd8(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::LuaArguments::pushTuple(RBX::Reflection::Tuple const&, lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESQ_")]
pub fn stub_0x2a5004() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list_av_2<boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::type> boost::bind<void,lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>(void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>)")]
pub fn stub_0x2a5104() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESQ_")]
pub fn stub_0x2a5200() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list_av_2<boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>::type> boost::bind<void,lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>,boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>(void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>)")]
pub fn stub_0x2a5300() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::Lua::LuaArguments::getValues(lua_State *)")]
pub fn stub_0x2a53fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Lua::LuaArguments getter.
cell.get()
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::operator()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)const")]
pub fn stub_0x2a5660(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list_av_3<boost::arg<1>,boost::arg<2>,std::string>::type> boost::bind<void,lua_State *,int,std::string,boost::arg<1>,boost::arg<2>,std::string>(void (*)(lua_State *,int,std::string),boost::arg<1>,boost::arg<2>,std::string)")]
pub fn stub_0x2a5778() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::scoped_ptr<RBX::Lua::Continuations>::reset(RBX::Lua::Continuations*)")]
pub fn stub_0x2a5920() -> crate::slot::PortedFn {
// IDA 0x2a5920: boost::scoped_ptr<RBX::Lua::Continuations>::reset(RBX::Lua::Continuations*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2a5920, "boost::scoped_ptr<RBX::Lua::Continuations>::reset(RBX::Lua::Continuations*)")
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::operator()(lua_State *)const")]
pub fn stub_0x2a59f4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::operator()(lua_State *,unsigned long)const")]
pub fn stub_0x2a5abc(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESP_")]
pub fn stub_0x2a5b84() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list_av_3<RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>>::type> boost::bind<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *,RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>>(void (RBX::ScriptContext::*)(RBX::Lua::WeakThreadRef,lua_State *),RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>)")]
pub fn stub_0x2a5c90() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "rbx::safe_queue<RBX::ScriptContext::WaitingThread>::pop_if_present(RBX::ScriptContext::WaitingThread&)")]
pub fn stub_0x2a5f64(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)")]
pub fn stub_0x2a6160() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::ScriptContext::ScriptStart::ScriptStart(RBX::ScriptContext::ScriptStart const&)")]
pub fn stub_0x2a61b0() -> crate::slot::InstanceHandle {
// RBX::ScriptContext::ScriptStart ctor.
crate::slot::InstanceHandle::new("RBX::ScriptContext::ScriptStart")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::ScriptStats>::reset<RBX::ScriptStats>(RBX::ScriptStats *)")]
pub fn stub_0x2a633c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptStats")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaStatsItem>::operator=(rbx_core::SharedPtr<RBX::LuaStatsItem> const&)")]
pub fn stub_0x2a64a4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::LuaStatsItem::create(RBX::ScriptContext *)")]
pub fn stub_0x2a64dc(handle: &crate::slot::InstanceHandle) {
// RBX::LuaStatsItem::create(RBX::ScriptContext*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::operator=(rbx_core::SharedPtr<RBX::ScriptStats> const&)")]
pub fn stub_0x2a6590(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> const&)")]
pub fn stub_0x2a6600() -> crate::slot::SlotConnection {
// IDA 0x2a6600: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterScript> RBX::Creatable<RBX::Instance>::create<RBX::StarterScript,RBX::ContentId>(RBX::ContentId)")]
pub fn stub_0x2a6674() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&)")]
pub fn stub_0x2a6728(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreScript> RBX::Creatable<RBX::Instance>::create<RBX::CoreScript,RBX::ContentId>(RBX::ContentId)")]
pub fn stub_0x2a675c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript> RBX::shared_from<RBX::BaseScript>(RBX::BaseScript*)")]
pub fn stub_0x2a6810() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

#[doc(alias = "std::map<std::string,RBX::ScriptContext::ScriptStatInformation,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::operator[](std::string const&)")]
pub fn stub_0x2a68f8(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::erase(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>)")]
pub fn stub_0x2a6e04(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript>::operator=(rbx_core::SharedPtr<RBX::BaseScript> const&)")]
pub fn stub_0x2a6e2c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::safe_queue<RBX::ScriptContext::WaitingThread>::push(RBX::ScriptContext::WaitingThread const&)")]
pub fn stub_0x2a715c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "boost::function1<void,lua_State *>::operator()(lua_State *)const")]
pub fn stub_0x2a7220(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::push_back(RBX::ScriptContext::ScriptStart const&)")]
pub fn stub_0x2a72e4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>::operator=(RBX::Lua::WeakThreadRef::Node*)")]
pub fn stub_0x2a7324(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(lua_State *)>::operator()(lua_State *)")]
pub fn stub_0x2a74b4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (lua_State*)>::operator()(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>> const&)")]
pub fn stub_0x2a75f8() -> crate::slot::SlotConnection {
// IDA 0x2a75f8: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::RuntimeScriptService::onRunTransition(RBX::RunTransition)")]
pub fn stub_0x2a766c(handle: &crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService::onRunTransition(RBX::RunTransition) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>)")]
pub fn stub_0x2a7674() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list_av_3<RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>::type> boost::bind<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions,RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>(void (RBX::ScriptContext::*)(RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions),RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions)")]
pub fn stub_0x2a7800() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)")]
pub fn stub_0x2a7c0c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::find<RBX::ScriptContext>(RBX::Instance const*)")]
pub fn stub_0x2a7c60() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ScriptContext"))
}

#[doc(alias = "RBX::RuntimeScriptService::~RuntimeScriptService()")]
pub fn stub_0x2a7c7c(handle: crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService dtor.
drop(handle);
}

#[doc(alias = "RBX::RuntimeScriptService::~RuntimeScriptService() [0x2a7c80]")]
pub fn stub_0x2a7c80(handle: crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEE12getClassNameEv")]
pub fn stub_0x2a7d20() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::RuntimeScriptService::~RuntimeScriptService()")]
pub fn stub_0x2a7d4c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::RuntimeScriptService::~RuntimeScriptService() [0x2a7d54]")]
pub fn stub_0x2a7d54(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEE12getClassNameEv")]
pub fn stub_0x2a7df8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::RuntimeScriptService::~RuntimeScriptService() [0x2a7e20]")]
pub fn stub_0x2a7e20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::RuntimeScriptService::~RuntimeScriptService() [0x2a7e28]")]
pub fn stub_0x2a7e28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E12getClassNameEv")]
pub fn stub_0x2a7ed0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ScriptContext"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E12getClassNameEv")]
pub fn stub_0x2a7ee0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ScriptContext"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x2a7ef8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LocalScript"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorD1Ev")]
pub fn stub_0x2a7f00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E17static_getCreatorEv")]
pub fn stub_0x2a7f08() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ScriptContext"
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sScriptContextEEEERKS0_v")]
pub fn stub_0x2a7f80(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sScriptContext>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7Creator12getClassNameEv")]
pub fn stub_0x2a8060() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7Creator6createEv")]
pub fn stub_0x2a80cc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::DebuggerWatch>(void)")]
pub fn stub_0x2a8210() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E12getClassNameEv")]
pub fn stub_0x2a8380() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E12getClassNameEv")]
pub fn stub_0x2a8390() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E17static_getCreatorEv")]
pub fn stub_0x2a83a0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x2a8414(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2a8418(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x2a84b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2a84c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x2a8564(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2a856c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch>::shared_ptr<RBX::Scripting::DebuggerWatch,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2a8610() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::DebuggerWatch,RBX::Scripting::DebuggerWatch>(rbx_core::SharedPtr<RBX::Scripting::DebuggerWatch> const*,RBX::Scripting::DebuggerWatch *)const")]
pub fn stub_0x2a86d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerWatch")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2a87c4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2a88cc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x2a88d0]")]
pub fn stub_0x2a88d0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2a88d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2a88f4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::DebuggerWatch *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2a890c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v")]
pub fn stub_0x2a8910(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::Scripting::sDebuggerWatch>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting14sDebuggerWatchEEEERKS0_v")]
pub fn stub_0x2a8958(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::Scripting::sDebuggerWatch>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting13DebuggerWatchENS_8InstanceELZNS1_14sDebuggerWatchEES3_E7CreatorC2Ev")]
pub fn stub_0x2a8a3c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::DebuggerWatch"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorD2Ev")]
pub fn stub_0x2a8c64() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7Creator12getClassNameEv")]
pub fn stub_0x2a8d00() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> RBX::Creatable<RBX::Instance>::create<RBX::Scripting::ScriptDebugger>(void)")]
pub fn stub_0x2a8d70() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::ScriptDebugger(void)")]
pub fn stub_0x2a8e20() -> crate::slot::InstanceHandle {
// RBX::Scripting::ScriptDebugger ctor — fresh debugger identity.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E12getClassNameEv")]
pub fn stub_0x2a93b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E12getClassNameEv")]
pub fn stub_0x2a93c8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E17static_getCreatorEv")]
pub fn stub_0x2a93d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::clear(void)")]
pub fn stub_0x2a9450(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::table(unsigned long,boost::hash<int> const&,std::equal_to<int> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> const&)")]
pub fn stub_0x2a9ac8() -> crate::generated::DebuggerBreakpoint {
// DebuggerBreakpoint ctor — disabled breakpoint id 0.
crate::generated::DebuggerBreakpoint { id: 0, enabled: false }
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
pub fn stub_0x2a9b34(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Scripting::ScriptDebugger, RBX::Scripting::sScriptDebugger~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x2a9c54(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2a9c58(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x2a9cf8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2a9d00(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x2a9da4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting14ScriptDebuggerELZNS2_15sScriptDebuggerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_15sScriptDebuggerEES5_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x2a9dac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}
