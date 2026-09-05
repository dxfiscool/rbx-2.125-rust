// Auto-generated skeletons for rbx-script — Lua/Script filtered
// Filter: Lua|Script (4456 filtered, 1133 remaining not yet in any crate) -> next 120 EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x817bb4..0x90ebd4 | script 14091->14211 distinct (filtered)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; " and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Lua::LibraryBridge::push(lua_State *,RBX::Lua::Library const&)")]
pub fn stub_0x817bb4(thread: &mut crate::lua::LuaThreadState, value: &crate::lua::LuaStackValue) -> i32 {
// LibraryBridge::push — pushes the saved result value.
thread.push(value.clone());
1
}

#[doc(alias = "RBX::Lua::LibraryBridge::find(lua_State *,std::string const&)")]
pub fn stub_0x817dd4(lib: &crate::slot::InstanceHandle, name: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// LibraryBridge::find — resolves the named host closure.
let _ = lib;
thread.push(crate::lua::LuaStackValue::Function(crate::lua::method_fn_id(name)));
1
}

#[doc(alias = "RBX::Lua::LibraryBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x817ebc(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaL_register + setreadonly + pop (cf. 0x2708b0).
let _ = thread;
0
}

#[doc(alias = "RBX::LibraryService::LibraryService(RBX::ScriptContext *)")]
pub fn stub_0x817ef4() -> crate::slot::InstanceHandle {
// RBX::LibraryService ctor.
crate::slot::InstanceHandle::new("RBX::LibraryService")
}

#[doc(alias = "RBX::LibraryService::ContentReadyHelper(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x818730() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::LibraryService::registerDevelopmentLibrary(std::string const&,rbx_core::SharedPtr<RBX::Script>)")]
pub fn stub_0x81957c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "RBX::Lua::Library* RBX::Lua::Bridge<RBX::Lua::Library,true>::pushNewObject<RBX::Lua::Library>(lua_State *,RBX::Lua::Library)")]
pub fn stub_0x81aac0(thread: &mut crate::lua::LuaThreadState, value: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// Bridge pushNewObject — pushes the host handle identity.
thread.push(crate::lua::LuaStackValue::Number(value.id as f64));
*value
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::operator[](std::string const&)")]
pub fn stub_0x81b960() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::operator=(rbx_core::SharedPtr<RBX::Script> const&)")]
pub fn stub_0x81bb7c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x81c20c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*)")]
pub fn stub_0x81c550() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)")]
pub fn stub_0x81c940() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x81ca80() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x81cc30() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
pub fn stub_0x81cde4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x81cf2c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x81d0f0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x81d10c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x81d130(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x81d2e8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x81d498(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&> &,int)")]
pub fn stub_0x81d558(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x81d558: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x81d7e4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x81d938() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x81db6c() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x81dda0() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>)")]
pub fn stub_0x81dfd4() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_0x81e208() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>)")]
pub fn stub_0x81e3e4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type)")]
pub fn stub_0x81e558() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sLuaSettingsEEEERKS0_v")]
pub fn stub_0x81eff8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sLuaSettings>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sLuaSettingsEEEERKS0_v")]
pub fn stub_0x81f040(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLuaSettings>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>::pair(std::string const&,rbx_core::SharedPtr<RBX::Script> const&)")]
pub fn stub_0x81fc84() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81fd40() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81fe2c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_insert_unique(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81fe7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_create_node(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>> const&)")]
pub fn stub_0x81ff00() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::lower_bound(std::string const&)")]
pub fn stub_0x820008() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::find(std::string const&)")]
pub fn stub_0x820038() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "RBX::ClientAppSettings::ReadValueMinNumberScriptExecutionsToGetPrize(char const*)")]
pub fn stub_0x8560a8(handle: &crate::slot::InstanceHandle) {
// RBX::ClientAppSettings::ReadValueMinNumberScriptExecutionsToGetPrize(char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::getCellScript(int,int,int)")]
pub fn stub_0x86be0c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MegaClusterInstance getter.
cell.get()
}

#[doc(alias = "RBX::MegaClusterInstance::setCellScript(int,int,int,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation)")]
pub fn stub_0x86c0b4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::MegaClusterInstance setter.
cell.set(value)
}

#[doc(alias = "RBX::MegaClusterInstance::setCellsScript(RBX::Region3int16,RBX::Voxel::CellMaterial,RBX::Voxel::CellBlock,RBX::Voxel::CellOrientation)")]
pub fn stub_0x86c178(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::MegaClusterInstance setter.
cell.set(value)
}

#[doc(alias = "RBX::MegaClusterInstance::getWaterCellScript(int,int,int)")]
pub fn stub_0x86c21c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::MegaClusterInstance getter.
cell.get()
}

#[doc(alias = "RBX::MegaClusterInstance::setWaterCellScript(int,int,int,RBX::Voxel::WaterCellForce,RBX::Voxel::WaterCellDirection)")]
pub fn stub_0x86c490(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::MegaClusterInstance setter.
cell.set(value)
}

#[doc(alias = "RBX::MegaClusterInstance::autoWedgeCellScript(int,int,int)")]
pub fn stub_0x86c528(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::autoWedgeCellScript(int, int, int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::autoWedgeCellsScript(RBX::Region3int16)")]
pub fn stub_0x86c9b8(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::autoWedgeCellsScript(RBX::Region3int16) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::cellCenterToWorldScript(int,int,int)")]
pub fn stub_0x86ca30(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::cellCenterToWorldScript(int, int, int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::worldToCellPreferSolidScript(G3D::Vector3)")]
pub fn stub_0x86ca80(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::worldToCellPreferSolidScript(G3D::Vector3) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::worldToCellPreferEmptyScript(G3D::Vector3)")]
pub fn stub_0x86caec(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::worldToCellPreferEmptyScript(G3D::Vector3) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::worldToCellScript(G3D::Vector3)")]
pub fn stub_0x86cb58(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::worldToCellScript(G3D::Vector3) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::countCellsScript(void)")]
pub fn stub_0x86cc5c(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::countCellsScript() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MegaClusterInstance::cellCornerToWorldScript(int,int,int)")]
pub fn stub_0x8714e8(handle: &crate::slot::InstanceHandle) {
// RBX::MegaClusterInstance::cellCornerToWorldScript(int, int, int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Plugin::getMouseLua(void)")]
pub fn stub_0x885014(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Plugin getter.
cell.get()
}

#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(RBX::Instance const*)")]
pub fn stub_0x8d05d8() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::create<RBX::LuaWebService>(void)const")]
pub fn stub_0x8d05f0() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "RBX::LuaWebService * RBX::ServiceProvider::find<RBX::LuaWebService>(void)const")]
pub fn stub_0x8d07b8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::LuaWebService"))
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLuaWebServiceEEEERKS0_v")]
pub fn stub_0x8d0930(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sLuaWebService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLuaWebServiceEEEERKS0_v")]
pub fn stub_0x8d0978(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLuaWebService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LuaWebService>(void)")]
pub fn stub_0x8d0a60() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x8d0b38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x8d0ce8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x8d0df0]")]
pub fn stub_0x8d0df0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x8d0df8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x8d0e18() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x8d0e30() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ScriptService::waitForChild(rbx_core::WeakPtr<RBX::Instance>,std::string,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x8e81e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::ScriptService::onChildAdded(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x8e83c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>> const&)")]
pub fn stub_0x8e8690() -> crate::slot::SlotConnection {
// IDA 0x8e8690: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::push_back(rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
pub fn stub_0x8e8704() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::erase(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>)")]
pub fn stub_0x8e8754() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>)")]
pub fn stub_0x8e8780() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::ScriptService::Info>*)")]
pub fn stub_0x8e87a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *)")]
pub fn stub_0x8e87d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::operator=(rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
pub fn stub_0x8e881c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_copy_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>)")]
pub fn stub_0x8e8854() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>,std::random_access_iterator_tag)")]
pub fn stub_0x8e887c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
pub fn stub_0x8e88f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_allocate(unsigned long)")]
pub fn stub_0x8e8cbc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *)")]
pub fn stub_0x8e8cd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x8e8d24(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>>::~callable_slot() [0x8e8d50]")]
pub fn stub_0x8e8d50(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x8e8e24(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x8e8e24: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ScriptService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x8e8e40(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x8e8e40: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x8e8e5c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x8e8e5c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::shared_ptr<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")]
pub fn stub_0x8e911c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService::Info")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")]
pub fn stub_0x8e91f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ScriptService::Info::~Info()")]
pub fn stub_0x8e92fc(handle: crate::slot::InstanceHandle) {
// RBX::ScriptService::Info dtor.
drop(handle);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p()")]
pub fn stub_0x8e9440(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p() [0x8e9444]")]
pub fn stub_0x8e9444(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::dispose(void)")]
pub fn stub_0x8e9448() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_deleter(std::type_info const&)")]
pub fn stub_0x8e94ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_untyped_deleter(void)")]
pub fn stub_0x8e94f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::OverlayDataModel::unloadGameFromScript(boost::function<void ()(void)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x8fae10(handle: &crate::slot::InstanceHandle) {
// RBX::OverlayDataModel::unloadGameFromScript(boost::function<void ()>, boost::function<void~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::OverlayDataModel::processSignedScript(std::string const*,std::exception const*)")]
pub fn stub_0x8fcd28(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::OverlayDataModel::executeSignedScriptFromUrl(std::string const&)")]
pub fn stub_0x8fd3b8(handle: &crate::slot::InstanceHandle) {
// RBX::OverlayDataModel::executeSignedScriptFromUrl(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::OverlayDataModel::loadJoinScript(std::string const&)")]
pub fn stub_0x8fd7ec(handle: &crate::slot::InstanceHandle) {
// RBX::OverlayDataModel::loadJoinScript(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorD1Ev")]
pub fn stub_0x9012f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ScriptContext"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator6createEv")]
pub fn stub_0x908028() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ScriptContext"
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptContext,RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const*,RBX::ScriptContext *)const")]
pub fn stub_0x908238() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptContext")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v")]
pub fn stub_0x908330(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sScriptContext>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ServerScriptService::ServerScriptService(void)")]
pub fn stub_0x90e500() -> crate::slot::InstanceHandle {
// RBX::ServerScriptService ctor.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "RBX::ServerScriptService::ServerScriptService(void) [0x90e504]")]
pub fn stub_0x90e504() -> crate::slot::InstanceHandle {
// RBX::ServerScriptService ctor.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "RBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x90e76c(handle: &crate::slot::InstanceHandle) {
// RBX::ServerScriptService::scriptShouldRun(RBX::BaseScript*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x90e830(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "RBX::ServerScriptService::~ServerScriptService()")]
pub fn stub_0x90e83c(handle: crate::slot::InstanceHandle) {
// RBX::ServerScriptService dtor.
drop(handle);
}

#[doc(alias = "RBX::ServerScriptService::~ServerScriptService() [0x90e840]")]
pub fn stub_0x90e840(handle: crate::slot::InstanceHandle) {
// RBX::ServerScriptService dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv")]
pub fn stub_0x90e904() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")]
pub fn stub_0x90e914(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService() [0x90e91c]")]
pub fn stub_0x90e91c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E12getClassNameEv")]
pub fn stub_0x90e9c0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService() [0x90e9d0]")]
pub fn stub_0x90e9d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService() [0x90e9d8]")]
pub fn stub_0x90e9d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x90ea7c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x90ea80(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x90eb20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x90eb28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x90ebcc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x90ebd4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}
