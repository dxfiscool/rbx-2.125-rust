// Auto-generated skeletons for rbx-script — Script/Lua batch
// Filter: Script|Lua (4456 filtered, 2402 existing, 2054 remaining) -> gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x34ef60..0x449b10 EA-sorted asc next 120 Script|Lua not yet in script (rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x34ef60() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x34f398() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x34f458() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
pub fn stub_0x34f51c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>> const&)")]
pub fn stub_0x34f674() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0x34f7ec(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x34f8c0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x34f8dc(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x34f900(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x34f9c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x34fa84(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0x34fb30(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x34fd50(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x34fe7c() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x35008c() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>>::storage4(boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>)")]
pub fn stub_0x35031c() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0x350524() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x351258() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x351318() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
pub fn stub_0x3513dc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>> const&)")]
pub fn stub_0x351534() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
pub fn stub_0x3516ac(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x351780(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x35179c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3517c0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x351884(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x351944(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0x3519f0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x351c10(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_0x351d3c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x352174() -> crate::slot::BindPiece {
// boost::bind fragment (list5) composing a host BoundCall.
crate::slot::BindPiece::new("list5")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
pub fn stub_0x352384() -> crate::slot::BindPiece {
// boost::bind fragment (storage5) composing a host BoundCall.
crate::slot::BindPiece::new("storage5")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)")]
pub fn stub_0x352614() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::Weak<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::Weak<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
pub fn stub_0x35281c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
pub fn stub_0x352f38() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
pub fn stub_0x354a68() -> crate::slot::InstanceHandle {
// RBX::AsyncHttpCache ctor.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x356388(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x35638c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x35642c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x356434(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3564d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3564e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_0x36b248(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>)")]
pub fn stub_0x36d948(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_0x36da4c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x36da78(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x36db50(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x36dc24(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
pub fn stub_0x36f134() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x370950(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x370954(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3709f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3709fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x370aa0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x370aa8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::BoundFuncDesc(void (RBX::ScriptInformationProvider::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x370b4c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptInformationProvider", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x370cc8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptInformationProvider", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::~BoundFuncDesc() [0x370cf8]")]
pub fn stub_0x370cf8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ScriptInformationProvider,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x370e00() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ScriptInformationProvider", "void", 1)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ScriptInformationProvider,void (RBX::ScriptInformationProvider::*)(std::string),std::string,void>::call(RBX::ScriptInformationProvider*,void (RBX::ScriptInformationProvider::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_0x370f3c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::ScriptInformationProvider, void (RBX::ScriptInformationP~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_0x39f7ac() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isScriptable(void)const")]
pub fn stub_0x39fdf8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isScriptable(void)const")]
pub fn stub_0x3a15a8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isScriptable(void)const")]
pub fn stub_0x3a1b58() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::isScriptable(void)const")]
pub fn stub_0x3ad038() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::isScriptable(void)const")]
pub fn stub_0x3ae7e4() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_0x3b9d24() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::BadgeService")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::isScriptable(void)const")]
pub fn stub_0x3ee6e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::isScriptable(void)const")]
pub fn stub_0x3f2548() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::DataModel::loadCoreScripts(void)")]
pub fn stub_0x4210d8(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::loadCoreScripts() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DataModel::startCoreScripts(RBX::Adorn *,bool)")]
pub fn stub_0x421b60(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::startCoreScripts(RBX::Adorn*, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::BaseScript>(void)const")]
pub fn stub_0x437a30(handle: &crate::slot::InstanceHandle) {
// int RBX::Instance::countDescendantsOfType<RBX::BaseScript>() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD1Ev")]
pub fn stub_0x4387e8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LocalScriptENS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x43b548() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LocalScript"
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x43b690() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptContext>(void)")]
pub fn stub_0x43c178() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4419a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::BaseScript, RBX::sBaseScript, RBX::NonFactoryProduct<RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev")]
pub fn stub_0x441ac8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ScriptContext"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD2Ev")]
pub fn stub_0x443280() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_0x44331c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator6createEv")]
pub fn stub_0x443388() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ServerScriptService>(void)")]
pub fn stub_0x4434cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService>::shared_ptr<RBX::ServerScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x44357c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerScriptService,RBX::ServerScriptService>(rbx_core::SharedPtr<RBX::ServerScriptService> const*,RBX::ServerScriptService *)const")]
pub fn stub_0x443644() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x443730() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x443838(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x44383c]")]
pub fn stub_0x44383c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x443840() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x443860() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x443878() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v")]
pub fn stub_0x44387c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sServerScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sServerScriptServiceEEEEvv")]
pub fn stub_0x4438c0(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::sServerScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v")]
pub fn stub_0x4438c4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sServerScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorC2Ev")]
pub fn stub_0x4439a8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "RBX::ServerScriptService * RBX::ServiceProvider::find<RBX::ServerScriptService>(void)const")]
pub fn stub_0x443bd0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ServerScriptService"))
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x443d44(handle: &crate::slot::InstanceHandle) {
// RBX::FactoryProduct<RBX::ServerScriptService, RBX::Instance, RBX::sServerScriptService, RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17static_getCreatorEv")]
pub fn stub_0x443dac() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ServerScriptService"
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ServerScriptService>(void)")]
pub fn stub_0x443e20() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerScriptService>(void)")]
pub fn stub_0x443e24() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ScriptService>(void)")]
pub fn stub_0x448fa4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const&)")]
pub fn stub_0x449148(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sScriptServiceEEEEvv")]
pub fn stub_0x449180(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::sScriptService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ScriptService>(void)")]
pub fn stub_0x449188() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "RBX::ScriptService::~ScriptService()")]
pub fn stub_0x44918c(handle: crate::slot::InstanceHandle) {
// RBX::ScriptService dtor.
drop(handle);
}

#[doc(alias = "RBX::ScriptService::~ScriptService() [0x449270]")]
pub fn stub_0x449270(handle: crate::slot::InstanceHandle) {
// RBX::ScriptService dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv")]
pub fn stub_0x449368() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService()")]
pub fn stub_0x44936c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService() [0x449450]")]
pub fn stub_0x449450(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEE12getClassNameEv")]
pub fn stub_0x449548() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService() [0x44954c]")]
pub fn stub_0x44954c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptService::~ScriptService() [0x449630]")]
pub fn stub_0x449630(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::~vector()")]
pub fn stub_0x449728(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4497f4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::ScriptService, RBX::sScriptService, RBX::NonFactoryProduct~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x449914(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x449918(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4499b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4499c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x449a64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x449a6c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService>::shared_ptr<RBX::ScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x449b10() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService")
}
