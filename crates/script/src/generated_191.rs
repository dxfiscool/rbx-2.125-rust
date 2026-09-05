// Auto-generated skeletons for rbx-script — Lua/Script/lua filtered
// Filter: Lua|Script|lua (5041 filtered, 1777 remaining not yet in any crate) -> next 120 EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x58c788..0x76fa0c | script 13851->13971 distinct (filtered)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  " and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService> RBX::Creatable<RBX::Instance>::create<RBX::LuaWebService>(void)")]
pub fn stub_0x58c788() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&)")]
pub fn stub_0x58c838(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LuaWebService>(void)")]
pub fn stub_0x58c86c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x58c870(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_9<boost::arg<1>,boost::arg<4>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>::type> boost::bind<void,RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>,boost::arg<1>,boost::arg<4>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>(void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::arg<1>,boost::arg<4>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x594f3c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_7<rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>>(void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x596684() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::list7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x596f44() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::storage7(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x597118() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>)")]
pub fn stub_0x597310() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>)")]
pub fn stub_0x5989ec(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x5991d8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
pub fn stub_0x5991f4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x599210(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x5999e8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x59a1bc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
pub fn stub_0x59a324(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x59a4f8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::list9(boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x59a754() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::storage9(boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>)")]
pub fn stub_0x59a928() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "boost::_bi::storage8<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>>::storage8(boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>)")]
pub fn stub_0x59ab20() -> crate::slot::BindPiece {
// boost::bind fragment (value) composing a host BoundCall.
crate::slot::BindPiece::new("value")
}

#[doc(alias = "void boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>)")]
pub fn stub_0x59c1fc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x59c9e8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker5<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::invoke(boost::detail::function::function_buffer &,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)")]
pub fn stub_0x59ca04(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x59ca4c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x59d224(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x59d9f8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>::operator()<void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &>>(boost::_bi::type<void>,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list5<RBX::ScriptInformationProvider::RequestResult&,bool &,bool &,float &,bool &> &,int)")]
pub fn stub_0x59db60(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x59dd4c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider>::shared_ptr<RBX::ScriptInformationProvider>(rbx_core::WeakPtr<RBX::ScriptInformationProvider> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x59dfa8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *>,boost::_bi::value<RBX::ScriptInformationProvider *>,boost::arg<1>>::operator()<void (*)(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *,RBX::ScriptInformationProvider *,rbx_core::SharedPtr<RBX::Instance>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_0x59e024(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x59e024: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "RBX::KeyframeSequenceProvider::getKeyframeSequenceLua(RBX::ContentId)")]
pub fn stub_0x5ba2b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::KeyframeSequenceProvider getter.
cell.get()
}

#[doc(alias = "RBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fb8e0(handle: &crate::slot::InstanceHandle) {
// RBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::BasePlayerGui::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fba7c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fd8b4(handle: &crate::slot::InstanceHandle) {
// RBX::StarterGuiService::scriptShouldRun(RBX::BaseScript*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fda38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fdcb4(handle: &crate::slot::InstanceHandle) {
// RBX::CoreGuiService::scriptShouldRun(RBX::BaseScript*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x5fe170(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *)")]
pub fn stub_0x614a00() -> crate::slot::InstanceHandle {
// RBX::ScriptMouseCommand ctor.
crate::slot::InstanceHandle::new("RBX::ScriptMouseCommand")
}

#[doc(alias = "RBX::ScriptMouseCommand::ScriptMouseCommand(RBX::Workspace *) [0x614a04]")]
pub fn stub_0x614a04() -> crate::slot::InstanceHandle {
// RBX::ScriptMouseCommand ctor.
crate::slot::InstanceHandle::new("RBX::ScriptMouseCommand")
}

#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand()")]
pub fn stub_0x614b58(handle: crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand dtor.
drop(handle);
}

#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand() [0x614bf8]")]
pub fn stub_0x614bf8(handle: crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand()")]
pub fn stub_0x614bfc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::ScriptMouseCommand::~ScriptMouseCommand() [0x614c04]")]
pub fn stub_0x614c04(handle: crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptMouseCommand::~ScriptMouseCommand() [0x614d30]")]
pub fn stub_0x614d30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::ScriptMouseCommand::getCursorId(void)const")]
pub fn stub_0x614d38(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ScriptMouseCommand getter.
cell.get()
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x614d48(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseHover(RBX::UIEvent const&)")]
pub fn stub_0x614e20(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onMouseHover(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseIdle(RBX::UIEvent const&)")]
pub fn stub_0x614e2c(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onMouseIdle(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseWheelForward(RBX::UIEvent const&)")]
pub fn stub_0x614e38(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onMouseWheelForward(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseWheelBackward(RBX::UIEvent const&)")]
pub fn stub_0x614f10(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onMouseWheelBackward(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onRightMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x614fe8(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onRightMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onRightMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x6150c0(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onRightMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onMouseUp(RBX::UIEvent const&)")]
pub fn stub_0x615198(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onMouseUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onPeekKeyDown(RBX::UIEvent const&)")]
pub fn stub_0x615270(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onPeekKeyDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::onPeekKeyUp(RBX::UIEvent const&)")]
pub fn stub_0x615348(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptMouseCommand::onPeekKeyUp(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptMouseCommand::getName(void)const")]
pub fn stub_0x615420(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ScriptMouseCommand getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptMouseCommand> RBX::shared_from<RBX::ScriptMouseCommand>(RBX::ScriptMouseCommand*)")]
pub fn stub_0x615424() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptMouseCommand")
}

#[doc(alias = "RBX::Selection::propagateChangeSignalToLua(RBX::SelectionChanged const&)")]
pub fn stub_0x619080(handle: &crate::slot::InstanceHandle) {
// RBX::Selection::propagateChangeSignalToLua(RBX::SelectionChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Stats::StatsService::tryToStartScript(void)")]
pub fn stub_0x647bd4(handle: &crate::slot::InstanceHandle) {
// RBX::Stats::StatsService::tryToStartScript() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sScriptContextEEEEvv")]
pub fn stub_0x652a70(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::sScriptContext>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_18ScriptMouseCommandELZNS_17sToolMouseCommandEEE7getNameEv")]
pub fn stub_0x689260(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "RBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x6d0138(handle: &crate::slot::InstanceHandle) {
// RBX::Workspace::scriptShouldRun(RBX::BaseScript*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Workspace::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x6d02e4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 388, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 388);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv")]
pub fn stub_0x6d3ca0() -> crate::slot::PortedFn {
// IDA 0x6d3ca0: void RBX::Name::callDoDeclare<RBX::sLocalScript>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x6d3ca0, "void RBX::Name::callDoDeclare<RBX::sLocalScript>()")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef rbx::any_cast<RBX::Lua::WeakFunctionRef,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x6f98e4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Instance::luaClone(void)")]
pub fn stub_0x701470(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::luaClone() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::find<RBX::ScriptService>(void)const")]
pub fn stub_0x705bb0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ScriptService"))
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE15isNullClassNameEv")]
pub fn stub_0x705d28(handle: &crate::slot::InstanceHandle) {
// RBX::NonFactoryProduct<RBX::Instance, RBX::sScriptService>::isNullClassName() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sScriptServiceEEEERKS0_v")]
pub fn stub_0x705dc8(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sScriptServiceEEEERKS0_v")]
pub fn stub_0x705e10(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptService>(void)")]
pub fn stub_0x705ef8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19ServerScriptServiceELZNS_20sServerScriptServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sServerScriptServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x7105b8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::ServerScriptService, RBX::sServerScriptService, RBX::Facto~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::World::onAssemblyInSimluationStage(RBX::Assembly *)")]
pub fn stub_0x762f10(handle: &crate::slot::InstanceHandle) {
// RBX::World::onAssemblyInSimluationStage(RBX::Assembly*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerManager::enableDebugging(void)")]
pub fn stub_0x767b10(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager::enableDebugging() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::resume(void)")]
pub fn stub_0x76829c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::resume() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOver(void)")]
pub fn stub_0x7685c4(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::stepOver() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::stepInto(void)")]
pub fn stub_0x768750(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::stepInto() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::stepOut(void)")]
pub fn stub_0x7688d8(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::stepOut() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getLocals(int)")]
pub fn stub_0x769338(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getUpvalues(int)")]
pub fn stub_0x769414(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getGlobals(void)")]
pub fn stub_0x7694f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getScriptPath(void)const")]
pub fn stub_0x769db0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::setScriptPath(std::string)")]
pub fn stub_0x769f7c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Scripting::ScriptDebugger setter — stores the converted value.
cell.set(value)
}

#[doc(alias = "RBX::Scripting::DebuggerWatch::checkExpressionSyntax(void)")]
pub fn stub_0x76a5c0(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerWatch::checkExpressionSyntax() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerManager::singleton(void)")]
pub fn stub_0x76a92c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerManager::DebuggerManager(void)")]
pub fn stub_0x76ab8c() -> crate::slot::InstanceHandle {
// RBX::Scripting::DebuggerManager ctor — fresh debugger identity.
crate::slot::InstanceHandle::new("RBX::Scripting::DebuggerManager")
}

#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager()")]
pub fn stub_0x76aec4(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager() [0x76af64]")]
pub fn stub_0x76af64(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager()")]
pub fn stub_0x76af68(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager() [0x76af70]")]
pub fn stub_0x76af70(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Scripting::DebuggerManager::~DebuggerManager() [0x76af78]")]
pub fn stub_0x76af78(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager() [0x76b128]")]
pub fn stub_0x76b128(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::DebuggerManager::~DebuggerManager() [0x76b130]")]
pub fn stub_0x76b130(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(lua_State *)")]
pub fn stub_0x76b13c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager::findDebugger(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerManager::findDebugger(RBX::Script *)")]
pub fn stub_0x76b2b0(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager::findDebugger(RBX::Script*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::DebuggerManager::addDebugger(RBX::Script *)")]
pub fn stub_0x76b470(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::DebuggerManager::addDebugger(RBX::Script*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::ScriptDebugger(RBX::Script &)")]
pub fn stub_0x76b99c() -> crate::slot::InstanceHandle {
// RBX::Scripting::ScriptDebugger ctor — fresh debugger identity.
crate::slot::InstanceHandle::new("RBX::Scripting::ScriptDebugger")
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::setScript(RBX::Script *)")]
pub fn stub_0x76c054(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Scripting::ScriptDebugger setter — stores the converted value.
cell.set(value)
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
pub fn stub_0x76c3a4(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger dtor.
drop(handle);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76c444]")]
pub fn stub_0x76c444(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger()")]
pub fn stub_0x76c448(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76c450]")]
pub fn stub_0x76c450(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76c458]")]
pub fn stub_0x76c458(handle: crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76ca0c]")]
pub fn stub_0x76ca0c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Scripting::ScriptDebugger::~ScriptDebugger() [0x76ca14]")]
pub fn stub_0x76ca14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::addWatch(std::string)")]
pub fn stub_0x76ca1c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::addWatch(std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatchValue(RBX::Scripting::DebuggerWatch *)")]
pub fn stub_0x76cb6c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "readWatchValue(std::string,lua_State *)")]
pub fn stub_0x76cd58() -> crate::slot::PortedFn {
// IDA 0x76cd58: readWatchValue(std::string, lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x76cd58, "readWatchValue(std::string, lua_State*)")
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::hook(lua_State *,lua_Debug *)")]
pub fn stub_0x76d500(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::hook(lua_State*, lua_Debug*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::debuggerBreak(lua_State *,lua_Debug *)")]
pub fn stub_0x76d5e0(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::debuggerBreak(lua_State*, lua_Debug*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readLocals(int,lua_State *)")]
pub fn stub_0x76d95c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::readLocals(int, lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readGlobals(lua_State *)")]
pub fn stub_0x76dc5c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::readGlobals(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readUpvalues(int,lua_State *)")]
pub fn stub_0x76dfcc(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::readUpvalues(int, lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::readStack(lua_State *)")]
pub fn stub_0x76e434(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::readStack(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::getScriptForLuaState(lua_State *)")]
pub fn stub_0x76e860(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::onLineHook(lua_State *,lua_Debug *)")]
pub fn stub_0x76ea28(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::onLineHook(lua_State*, lua_Debug*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::findBreakpoint(int)")]
pub fn stub_0x76ecb0(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::findBreakpoint(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::shouldBreak(RBX::Scripting::DebuggerBreakpoint *,lua_State *)")]
pub fn stub_0x76ece8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Scripting::ScriptDebugger getter — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::setBreakpoint(int)")]
pub fn stub_0x76f488(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Scripting::ScriptDebugger setter — stores the converted value.
cell.set(value)
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::onScriptStarting(lua_State *)")]
pub fn stub_0x76fa0c(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::onScriptStarting(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}
