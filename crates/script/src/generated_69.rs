// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2a9e50..0x2afda4 | remaining 3570 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger>::shared_ptr<RBX::Scripting::ScriptDebugger,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2a9e50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scripting::ScriptDebugger,RBX::Scripting::ScriptDebugger>(rbx_core::SharedPtr<RBX::Scripting::ScriptDebugger> const*,RBX::Scripting::ScriptDebugger *)const")]
pub fn stub_0x2a9f18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2aa004() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2aa10c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x2aa110]")]
pub fn stub_0x2aa110(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2aa114() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2aa134() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scripting::ScriptDebugger *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2aa14c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v")]
pub fn stub_0x2aa150(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::Scripting::sScriptDebugger>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9Scripting15sScriptDebuggerEEEERKS0_v")]
pub fn stub_0x2aa198(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::Scripting::sScriptDebugger>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9Scripting14ScriptDebuggerENS_8InstanceELZNS1_15sScriptDebuggerEES3_E7CreatorC2Ev")]
pub fn stub_0x2aa27c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Scripting::ScriptDebugger"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x2aa4a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LocalScript"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript> RBX::Creatable<RBX::Instance>::create<RBX::LocalScript>(void)")]
pub fn stub_0x2aa518() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalScript")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalScript>::shared_ptr<RBX::LocalScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2aa5c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalScript")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalScript,RBX::LocalScript>(rbx_core::SharedPtr<RBX::LocalScript> const*,RBX::LocalScript *)const")]
pub fn stub_0x2aa690() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalScript")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2aa780(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sLocalScriptEEEERKS0_v")]
pub fn stub_0x2aa788(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sLocalScript>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLocalScriptEEEERKS0_v")]
pub fn stub_0x2aa7d0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLocalScript>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sRuntimeScriptServiceEEEEvv")]
pub fn stub_0x2aa8b4(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::sRuntimeScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sRuntimeScriptServiceEEEERKS0_v")]
pub fn stub_0x2aa8b8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sRuntimeScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::find<RBX::ScriptContext>(void)const")]
pub fn stub_0x2aa998() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ScriptContext"))
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E15isNullClassNameEv")]
pub fn stub_0x2aab10(handle: &crate::slot::InstanceHandle) {
// RBX::FactoryProduct<RBX::ScriptContext, RBX::Instance, RBX::sScriptContext, RBX::Instance>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ScriptContext>(void)")]
pub fn stub_0x2ab2d0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10CoreScriptELZNS_11sCoreScriptEENS_17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x2ab2d4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::CoreScript, RBX::sCoreScript, RBX::NonFactoryProduct<RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list1<RBX::BaseScript * const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions> &,boost::_bi::list1<RBX::BaseScript * const&> &,int)")]
pub fn stub_0x2ab3f4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2ab3f4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>::operator()(RBX::ScriptContext*,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions)const")]
pub fn stub_0x2ab620() -> crate::slot::BindPiece {
// boost::bind fragment (mf2) composing a host BoundCall.
crate::slot::BindPiece::new("mf2")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>)")]
pub fn stub_0x2ab868() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::swap(std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>&)")]
pub fn stub_0x2abb24(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<RBX::BaseScript*, RBX::BaseScript*, std::_Identity<RBX::BaseScript*>, std::l~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique<std::_Rb_tree_const_iterator<RBX::BaseScript *>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>)")]
pub fn stub_0x2abb90(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(std::_Rb_tree_iterator<RBX::BaseScript *>,RBX::BaseScript * const&)")]
pub fn stub_0x2abbc4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::BaseScript * const&)")]
pub fn stub_0x2abc78(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(RBX::BaseScript * const&)")]
pub fn stub_0x2abcd0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD2Ev")]
pub fn stub_0x2abd38() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ScriptContext"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext> RBX::Creatable<RBX::Instance>::create<RBX::ScriptContext>(void)")]
pub fn stub_0x2abdd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2abe90() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2abf98(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2abfa0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2abfc0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptContext *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2abfd8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::create<RBX::ScriptContext>(void)const")]
pub fn stub_0x2abfe0() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x2ac1e4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot() [0x2ac210]")]
pub fn stub_0x2ac210(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
pub fn stub_0x2ac2e8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2ac2e8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
pub fn stub_0x2ac30c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2ac30c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
pub fn stub_0x2ac330(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2ac330: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
pub fn stub_0x2ac488(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2ac488: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable() [0x2ac4b4]")]
pub fn stub_0x2ac4b4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2ac4b4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> &)")]
pub fn stub_0x2ac58c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (lua_State*)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::on_error(std::exception &)")]
pub fn stub_0x2ac6ec(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> const&)")]
pub fn stub_0x2ac718(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2ac740(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (lua_State*)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,RBX::ScriptContext::ScriptStart const&)")]
pub fn stub_0x2ac840(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::ScriptContext::ScriptStart, std::allocator<RBX::ScriptContext::ScriptStar~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptContext::ScriptStart::operator=(RBX::ScriptContext::ScriptStart const&)")]
pub fn stub_0x2acbc8(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStart::operator=(RBX::ScriptContext::ScriptStart const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_allocate(unsigned long)")]
pub fn stub_0x2acc00(handle: &crate::slot::InstanceHandle) {
// std::_Vector_base<RBX::ScriptContext::ScriptStart, std::allocator<RBX::ScriptContext::Scri~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)> const&)")]
pub fn stub_0x2acfe8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
pub fn stub_0x2ad0ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
pub fn stub_0x2ad188() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::clear(void)")]
pub fn stub_0x2ad28c(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")]
pub fn stub_0x2ad4c4(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStart* std::__copy_backward<false, std::random_access_iterator_t~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::function1<void,lua_State *>::dummy::nonnull(void)")]
pub fn stub_0x2ad520() -> crate::slot::PortedFn {
// IDA 0x2ad520: boost::function1<void, lua_State*>::dummy::nonnull().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2ad520, "boost::function1<void, lua_State*>::dummy::nonnull()")
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::push_back(RBX::ScriptContext::WaitingThread const&)")]
pub fn stub_0x2ad524(queue: &mut crate::generated::WaitingThreadQueue, thread: crate::generated::WaitingThread) {
// deque<WaitingThread>::push_back.
queue.threads.push_back(thread);
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_push_back_aux(RBX::ScriptContext::WaitingThread const&)")]
pub fn stub_0x2ad68c(queue: &mut crate::generated::WaitingThreadQueue, thread: crate::generated::WaitingThread) {
// deque<WaitingThread>::push_back.
queue.threads.push_back(thread);
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x2ada18() -> crate::generated::WaitingThreadQueue {
// deque<WaitingThread> ctor — empty queue.
crate::generated::WaitingThreadQueue::default()
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x2ada34() -> crate::generated::WaitingThreadQueue {
// deque<WaitingThread> ctor — empty queue.
crate::generated::WaitingThreadQueue::default()
}

#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x2adb0c(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::ScriptContext::WaitingThread, std::allocator<RBX::ScriptContext::Wai~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(RBX::BaseScript * const&)")]
pub fn stub_0x2aead4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::equal_range(RBX::BaseScript * const&)")]
pub fn stub_0x2aeafc(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<RBX::BaseScript*, RBX::BaseScript*, std::_Identity<RBX::BaseScript*>, std::l~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(std::_Rb_tree_iterator<RBX::BaseScript *>,std::_Rb_tree_iterator<RBX::BaseScript *>)")]
pub fn stub_0x2aeb48(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_erase(std::_Rb_tree_node<RBX::BaseScript *> *)")]
pub fn stub_0x2aeba8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")]
pub fn stub_0x2aebd0(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStart* std::__copy<false, std::random_access_iterator_tag>::copy~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef::Node,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef::Node,int,0> const*)")]
pub fn stub_0x2aec28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef::Node")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::string const&)")]
pub fn stub_0x2aed3c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>)")]
pub fn stub_0x2aed64(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")]
pub fn stub_0x2aedc8(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptContext::ScriptStatInfo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")]
pub fn stub_0x2aee8c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::lower_bound(std::string const&)")]
pub fn stub_0x2aeeb4(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptContext::ScriptStatInfo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::upper_bound(std::string const&)")]
pub fn stub_0x2aeee4(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptContext::ScriptStatInfo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>::pair(std::string const&,RBX::ScriptContext::ScriptStatInformation const&)")]
pub fn stub_0x2aef64() -> crate::slot::InstanceHandle {
// std::pair ctor.
crate::slot::InstanceHandle::new("std::pair")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af034(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af120(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af170(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_create_node(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af1f4(handle: &crate::slot::InstanceHandle) {
// std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptContext::ScriptStatInfo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreScript>::shared_ptr<RBX::CoreScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2af428() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreScript")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreScript,RBX::CoreScript>(rbx_core::SharedPtr<RBX::CoreScript> const*,RBX::CoreScript *)const")]
pub fn stub_0x2af4f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreScript")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2af5dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2af6e4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x2af6e8]")]
pub fn stub_0x2af6e8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2af6ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2af70c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2af724() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterScript>::shared_ptr<RBX::StarterScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2af728() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterScript")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterScript,RBX::StarterScript>(rbx_core::SharedPtr<RBX::StarterScript> const*,RBX::StarterScript *)const")]
pub fn stub_0x2af7f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterScript")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2af8dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2af9e4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x2af9e8]")]
pub fn stub_0x2af9e8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x2af9ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2afa0c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2afa24() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x2afd78(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot() [0x2afda4]")]
pub fn stub_0x2afda4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}
