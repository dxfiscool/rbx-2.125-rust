// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2aff94..0x2b6d04 | remaining 3470 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_0x2aff94(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2aff94: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_0x2aff9c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2aff9c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
pub fn stub_0x2affa4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_0x2b02a0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2b02a0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable() [0x2b02cc]")]
pub fn stub_0x2b02cc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2b02cc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)")]
pub fn stub_0x2b0878() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptStats")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *)")]
pub fn stub_0x2b094c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")]
pub fn stub_0x2b0ba0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p() [0x2b0ba4]")]
pub fn stub_0x2b0ba4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::dispose(void)")]
pub fn stub_0x2b0ba8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b0c80() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_untyped_deleter(void)")]
pub fn stub_0x2b0c84() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int)")]
pub fn stub_0x2b13e0(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2b13e0: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const")]
pub fn stub_0x2b14a4() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::pop_front(void)")]
pub fn stub_0x2b157c(queue: &mut crate::generated::WaitingThreadQueue) -> Option<crate::generated::WaitingThread> {
// deque<WaitingThread>::pop_front.
queue.threads.pop_front()
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_pop_front_aux(void)")]
pub fn stub_0x2b16d4(queue: &mut crate::generated::WaitingThreadQueue) -> Option<crate::generated::WaitingThread> {
// deque<WaitingThread>::pop_front.
queue.threads.pop_front()
}

#[doc(alias = "boost::function1<void,lua_State *>::swap(boost::function1<void,lua_State *>&)")]
pub fn stub_0x2b1a6c() -> crate::slot::PortedFn {
// IDA 0x2b1a6c: boost::function1<void, lua_State*>::swap(boost::function1<void, lua_State*>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b1a6c, "boost::function1<void, lua_State*>::swap(boost::function1<void, lua_State*>&)")
}

#[doc(alias = "boost::function1<void,lua_State *>::move_assign(boost::function1<void,lua_State *>&)")]
pub fn stub_0x2b1b48() -> crate::slot::PortedFn {
// IDA 0x2b1b48: boost::function1<void, lua_State*>::move_assign(boost::function1<void, lua_State*>&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b1b48, "boost::function1<void, lua_State*>::move_assign(boost::function1<void, lua_State*>&)")
}

#[doc(alias = "boost::function1<void,lua_State *>::clear(void)")]
pub fn stub_0x2b1c4c(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b1c78() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENS9_3Lua13WeakThreadRefES2_EENS5_5list3INS5_5valueIPSA_EENSF_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b1d58() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>)")]
pub fn stub_0x2b1e3c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b1f30(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_0x2b1f4c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2b1f68(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2b204c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x2b212c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::operator()<boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *> &,boost::_bi::list1<lua_State *&> &,int)")]
pub fn stub_0x2b21fc(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2b21fc: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>::operator()(RBX::ScriptContext*,RBX::Lua::WeakThreadRef,lua_State *)const")]
pub fn stub_0x2b22cc() -> crate::slot::BindPiece {
// boost::bind fragment (mf2) composing a host BoundCall.
crate::slot::BindPiece::new("mf2")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2b23a8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::list3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
pub fn stub_0x2b24f8() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>>::storage3(boost::_bi::value<RBX::ScriptContext *>,boost::_bi::value<RBX::Lua::WeakThreadRef>,boost::arg<1>)")]
pub fn stub_0x2b25c0() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::dummy::nonnull(void)")]
pub fn stub_0x2b2688() -> crate::slot::PortedFn {
// IDA 0x2b2688: boost::function2<void, lua_State*, unsigned long>::dummy::nonnull().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b2688, "boost::function2<void, lua_State*, unsigned long>::dummy::nonnull()")
}

#[doc(alias = "__ZN5boost9function2IvP9lua_StatemEC2INS_3_bi6bind_tIvPFvS2_iSsENS5_5list3INS_3argILi1EEENSA_ILi2EEENS5_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b268c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "void boost::function2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>)")]
pub fn stub_0x2b27b8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b28f4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,void,lua_State *,unsigned long>::invoke(boost::detail::function::function_buffer &,lua_State *,unsigned long)")]
pub fn stub_0x2b2974(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2b2998(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,lua_State *,unsigned long>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2b2ac4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::operator()<void (*)(lua_State *,int,std::string),boost::_bi::list2<lua_State *&,unsigned long &>>(boost::_bi::type<void>,void (*)(lua_State *,int,std::string) &,boost::_bi::list2<lua_State *&,unsigned long &> &,int)")]
pub fn stub_0x2b2bfc(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2b2bfc: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b2e3c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b2f10() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>)")]
pub fn stub_0x2b2fe4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b30c8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_0x2b30e4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2b3100(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2b31d8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x2b32a8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::operator()<void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>) &,boost::_bi::list1<lua_State *&> &,int)")]
pub fn stub_0x2b336c(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2b3434(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>>::list2(boost::arg<1>,boost::_bi::value<boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>)")]
pub fn stub_0x2b357c() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::dummy::nonnull(void)")]
pub fn stub_0x2b3644() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEC2INS_3_bi6bind_tIvPFvS2_NS0_IFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b3648() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvP9lua_StateEC2INS_3_bi6bind_tIvPFvS2_NS_8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEEENS5_5list2INS_3argILi1EEENS5_5valueISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b371c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>)")]
pub fn stub_0x2b37f0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b38d4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,void,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_0x2b38f0(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2b390c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,lua_State *>::assign_to<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2b39e4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,lua_State *>::assign_functor<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>(boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x2b3ab4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::operator()<void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list1<lua_State *&>>(boost::_bi::type<void>,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>) &,boost::_bi::list1<lua_State *&> &,int)")]
pub fn stub_0x2b3b78(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2b3c40(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b3e54(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::auto_ptr<RBX::Reflection::Tuple> &,lua_State *,unsigned long),boost::_bi::list3<boost::reference_wrapper<std::auto_ptr<RBX::Reflection::Tuple>>,boost::arg<1>,boost::arg<2>>>,void,lua_State *,unsigned long>::invoke(boost::detail::function::function_buffer &,lua_State *,unsigned long)")]
pub fn stub_0x2b3eb4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b3ebc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<int,int (*)(RBX::Reflection::Tuple const&,lua_State *),boost::_bi::list2<boost::reference_wrapper<RBX::Reflection::Tuple const>,boost::arg<1>>>,unsigned long,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_0x2b3f1c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::assign_to_own(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int> const&)")]
pub fn stub_0x2b3f24(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::assign_to_own(boost::function2<void,lua_State *,unsigned long> const&)")]
pub fn stub_0x2b3f84(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::clear(void)")]
pub fn stub_0x2b3fb4(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::assign_to_own(boost::function1<unsigned long,lua_State *> const&)")]
pub fn stub_0x2b3fe0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function1<unsigned long,lua_State *>::clear(void)")]
pub fn stub_0x2b4010(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long,unsigned long (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b403c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<unsigned long,unsigned long (*)(lua_State *),boost::_bi::list1<boost::arg<1>>>,unsigned long,lua_State *>::invoke(boost::detail::function::function_buffer &,lua_State *)")]
pub fn stub_0x2b409c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b44f0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>)")]
pub fn stub_0x2b45d8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b46d0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x2b46ec(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2b46f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2b47dc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x2b48c0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>::operator()(void)")]
pub fn stub_0x2b4994() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2b49ac(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>)")]
pub fn stub_0x2b4b04() -> crate::slot::BindPiece {
// boost::bind fragment (list1) composing a host BoundCall.
crate::slot::BindPiece::new("list1")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::find(std::string const&)")]
pub fn stub_0x2b5600(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_0x2b5b30(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot() [0x2b5b5c]")]
pub fn stub_0x2b5b5c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2b5d4c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2b5d4c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2b5d68(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2b5d68: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x2b5d84(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2b5d84: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ScriptContext*,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x2b5f3c() -> crate::slot::BindPiece {
// boost::bind fragment (mf3) composing a host BoundCall.
crate::slot::BindPiece::new("mf3")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x2b63e8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2b63e8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x2b6414]")]
pub fn stub_0x2b6414(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2b6414: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr()")]
pub fn stub_0x2b64e8() -> crate::slot::PortedFn {
// IDA 0x2b64e8: boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b64e8, "boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr()")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>> *)")]
pub fn stub_0x2b6828() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>*)")]
pub fn stub_0x2b6858() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Script")
}

#[doc(alias = "boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr()")]
pub fn stub_0x2b6a68() -> crate::slot::PortedFn {
// IDA 0x2b6a68: boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b6a68, "boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr()")
}

#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::~vector()")]
pub fn stub_0x2b6b18(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x2b6be4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::ScriptContext, RBX::sScriptContext, RBX::FactoryProduct<RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2b6d00(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptContextELZNS_14sScriptContextEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sScriptContextEES4_EELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2b6d04(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}
