// Auto-generated skeletons for rbx-script — Script/Lua/Yield/lua batch
// Filter: Script|Lua|Yield|lua (5401 filtered, 2519 remaining clean before batch, 2419 after)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x34ca24..0x355c80 EA-sorted asc next 100 Script/Lua/Yield/lua not yet in any crate (global-free 2519-> 2419, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0x34ca24(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x34caf8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x34cb14(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x34cb38(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x34cbfc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x34ccbc(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0x34cd68(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x34cf88(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<int>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x34d0b4(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::TryDispatchRequest<int>(RBX::AsyncHttpCache<RBX::LuaWebService::C~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x34d2a4() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x34d4b4() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>)")]
pub fn stub_0x34d744() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0x34d94c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvbEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x34da98() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvbEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x34db58() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
pub fn stub_0x34dc1c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>> const&)")]
pub fn stub_0x34dd74() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0x34deec(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x34dfc0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x34dfdc(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x34e000(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x34e0c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x34e184(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0x34e230(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x34e450(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<bool>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x34e57c(handle: &crate::slot::InstanceHandle) {
// bool RBX::LuaWebService::TryDispatchRequest<bool>(RBX::AsyncHttpCache<RBX::LuaWebService::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x34e76c() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x34e97c() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>)")]
pub fn stub_0x34ec0c() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0x34ee14() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "rbx_core::WeakPtr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type)")]
pub fn stub_0x35296c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo*)")]
pub fn stub_0x3529bc(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
pub fn stub_0x352ad8(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x352b14(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(rbx_core::WeakPtr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x352b80() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LuaWebService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
pub fn stub_0x352bfc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo, true>")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)const")]
pub fn stub_0x352ce4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo, true>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
pub fn stub_0x352e1c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub fn stub_0x352f14(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p() [0x352f18]")]
pub fn stub_0x352f18(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::dispose(void)")]
pub fn stub_0x352f1c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
pub fn stub_0x352f30() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
pub fn stub_0x352f34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub fn stub_0x353088(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache() [0x353190]")]
pub fn stub_0x353190(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x3532a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
pub fn stub_0x353554(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
pub fn stub_0x353588(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed(void)")]
pub fn stub_0x353b10(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::remove(std::string const&)")]
pub fn stub_0x353b68(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *)")]
pub fn stub_0x353bbc(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x353c18(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x353c44(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>::destroy(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>*)")]
pub fn stub_0x353c84() -> crate::slot::PortedFn {
// IDA 0x353c84: __gnu_cxx::new_allocator<std::pair<std::string, std::pair<unsigned long, RBX::LuaWebService::CachedRawLuaWebServiceInfo>~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x353c84, "__gnu_cxx::new_allocator<std::pair<std::string, std::pair<unsigned long, RBX::LuaWebService::CachedR~")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x353d3c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x353eec(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x353f10(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::~node_constructor()")]
pub fn stub_0x353f60(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x353f80(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x3540a8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x354138(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x354164(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct(void)")]
pub fn stub_0x3541bc() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo> const&)")]
pub fn stub_0x3541f8() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>> const&)")]
pub fn stub_0x3542c4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::~LRUCache()")]
pub fn stub_0x3543d4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
pub fn stub_0x3544e8(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_clear(void)")]
pub fn stub_0x354520(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x354548(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x354580(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::LRUCache(void)")]
pub fn stub_0x3545b4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
pub fn stub_0x354694(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> const&)")]
pub fn stub_0x354718() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
pub fn stub_0x354784() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo, true>")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const")]
pub fn stub_0x35486c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo, true>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
pub fn stub_0x354950() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub fn stub_0x354a48(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p() [0x354a4c]")]
pub fn stub_0x354a4c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::dispose(void)")]
pub fn stub_0x354a50() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
pub fn stub_0x354a60() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
pub fn stub_0x354a64() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub fn stub_0x354bb8(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache() [0x354cc0]")]
pub fn stub_0x354cc0(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x354dd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
pub fn stub_0x355034(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
pub fn stub_0x3550a8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *)")]
pub fn stub_0x35561c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x355678(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x3556a4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>)")]
pub fn stub_0x3556e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x3557bc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x355974(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x355998(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::~node_constructor()")]
pub fn stub_0x3559e8(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x355a08(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x355b30(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x355bc0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x355bec(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct(void)")]
pub fn stub_0x355c44(handle: &crate::slot::InstanceHandle) {
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_no~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo> const&)")]
pub fn stub_0x355c80() -> crate::slot::InstanceHandle {
// std::pair ctor.
crate::slot::InstanceHandle::new("std::pair")
}
