// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3ed820..0x3f3db0 | script 23752->23852 distinct (filler 0x3ed820 asc, not-in-script 61793->61693)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped ticks

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot *,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
pub fn stub_0x3ed820() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::on_error(std::exception &)")]
pub fn stub_0x3ed9a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::remote_signal(void)")]
pub fn stub_0x3ed9c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::disconnectAll(void)")]
pub fn stub_0x3edb24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3edc9c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3edca0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3edd40(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3edd48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3eddec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3eddf4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~RemoteEventDesc() [0x3ee4d0]")]
pub fn stub_0x3ee4d0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3ee584() -> crate::slot::SlotConnection {
// IDA 0x3ee584: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::isBroadcast(void)const")]
pub fn stub_0x3ee6f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ee6f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3ee910() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3ee920() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x3ee934() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&)")]
pub fn stub_0x3eea50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::clear(void)")]
pub fn stub_0x3eebdc(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENSA_5list4INSA_5valueINS1_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3eec08() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3eecec() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x3eedd4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3eeecc(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
pub fn stub_0x3eeee8(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3eef0c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3eeff4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3ef0d8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&> &,int)")]
pub fn stub_0x3ef1ac(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3ef1ac: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3ef1d4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> const&)")]
pub fn stub_0x3ef32c() -> crate::slot::SlotConnection {
// IDA 0x3ef32c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>*)")]
pub fn stub_0x3ef420(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ef420: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~callable_slot()")]
pub fn stub_0x3ef51c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~callable_slot() [0x3ef62c]")]
pub fn stub_0x3ef62c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
pub fn stub_0x3ef75c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ef75c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
pub fn stub_0x3ef8c8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ef8c8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)const")]
pub fn stub_0x3ef8d0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")]
pub fn stub_0x3efa78(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3efa78: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable() [0x3efb88]")]
pub fn stub_0x3efb88(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3efb88: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::assign_to_own(boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor> const&)")]
pub fn stub_0x3efcb8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3efce8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc()")]
pub fn stub_0x3eff44(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::~EventDesc() [0x3eff68]")]
pub fn stub_0x3eff68(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::BoundFuncDesc(void (RBX::ChatService::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),char const*,char const*,char const*,char const*,RBX::ChatService::ChatColor,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3f001c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x3f0290() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::~BoundFuncDesc() [0x3f02f8]")]
pub fn stub_0x3f02f8(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3f042c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call3Helper<RBX::ChatService,void (RBX::ChatService::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor,void>::call(RBX::ChatService*,void (RBX::ChatService::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&)")]
pub fn stub_0x3f05d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::ChatService::ChatColor RBX::Reflection::ArgHelper::getArg<RBX::ChatService::ChatColor,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::ChatService::ChatColor> const&,boost::disable_if<boost::is_same<RBX::ChatService::ChatColor,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x3f0760() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::ChatService::ChatColor>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::ChatService::ChatColor &,boost::enable_if<boost::is_enum<RBX::ChatService::ChatColor>,void>::type *)")]
pub fn stub_0x3f08f4() -> crate::slot::PortedFn {
// IDA 0x3f08f4: bool RBX::Reflection::ArgHelper::try_enum<3, RBX::ChatService::ChatColor>(RBX::Reflection::FunctionDescriptor::Arguments~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3f08f4, "bool RBX::Reflection::ArgHelper::try_enum<3, RBX::ChatService::ChatColor>(RBX::Reflection::FunctionD~")
}

#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~remote_signal()")]
pub fn stub_0x3f0948(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::ClickDetector::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x3f110c(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ClickDetector::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x3f1110(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::ClickDetector::fireMouseClick(float,RBX::Network::Player *)")]
pub fn stub_0x3f1114(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector::fireMouseClick(float, RBX::Network::Player*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ClickDetector::isClickable(rbx_core::SharedPtr<RBX::PartInstance>,float,bool,RBX::Network::Player *)")]
pub fn stub_0x3f1234() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::ClickDetector::updateLastHoverPart(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Player *)")]
pub fn stub_0x3f12e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::ClickDetector::fireMouseHover(RBX::Network::Player *)")]
pub fn stub_0x3f130c(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector::fireMouseHover(RBX::Network::Player*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player *)")]
pub fn stub_0x3f1410(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector::fireMouseHoverLeave(RBX::Network::Player*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ClickDetector::stopHover(rbx_core::SharedPtr<RBX::PartInstance>,RBX::Network::Player *)")]
pub fn stub_0x3f154c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "RBX::ClickDetector::isHovered(RBX::PartInstance *,float,bool,RBX::Network::Player *)")]
pub fn stub_0x3f15b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ClickDetector getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
pub fn stub_0x3f17f8(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::PartInstance>(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_0x3f181c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::ClickDetector*,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3f1850() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ClickDetector> RBX::shared_from<RBX::ClickDetector>(RBX::ClickDetector*)")]
pub fn stub_0x3f1984() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ClickDetector")
}

#[doc(alias = "RBX::ClickDetector::askAddChild(RBX::Instance const*)const")]
pub fn stub_0x3f1b98(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ClickDetector::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x3f1b9c(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ClickDetector::shouldRender3dAdorn(void)const")]
pub fn stub_0x3f1c00(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector::shouldRender3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ClickDetector::shouldRender3dAdorn(void)const")]
pub fn stub_0x3f1c34(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::ClickDetector const* RBX::Instance::findConstFirstChildOfType<RBX::ClickDetector>(void)const")]
pub fn stub_0x3f1cac(handle: &crate::slot::InstanceHandle) {
// RBX::ClickDetector const* RBX::Instance::findConstFirstChildOfType<RBX::ClickDetector>() c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::ClickDetector*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x3f1d14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3f1de8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3f1f34(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3f1f38(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3f1fd8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3f1fe0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3f2084(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13ClickDetectorELZNS_14sClickDetectorEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sClickDetectorEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3f208c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ClickDetector>(char const*,char const*,float RBX::ClickDetector::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x3f2130() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "float")
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::isReadOnly(void)const")]
pub fn stub_0x3f22c0() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "float")
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::isWriteOnly(void)const")]
pub fn stub_0x3f22c4() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "float")
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3f22c8() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "float")
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::ClickDetector>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x3f22d4() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "float")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc() [0x3f2330]")]
pub fn stub_0x3f2330(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3f23e4() -> crate::slot::SlotConnection {
// IDA 0x3f23e4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::isBroadcast(void)const")]
pub fn stub_0x3f2550() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3f2558() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3f26b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3f26c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3f26dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc()")]
pub fn stub_0x3f2860(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::ClickDetector,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::ClickDetector::*>::~EventDesc() [0x3f2884]")]
pub fn stub_0x3f2884(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::CollectionService::removeInstance(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3f3178() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::CollectionService::addInstance(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3f3394() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::CollectionService,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_0x3f351c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::CollectionService,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::CollectionService::*>::~EventDesc()")]
pub fn stub_0x3f355c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::operator[](std::string const&)")]
pub fn stub_0x3f3580() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>, std::alloca~")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::reset<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> *)")]
pub fn stub_0x3f379c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>, std::alloca~")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>> *)")]
pub fn stub_0x3f3c08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>, std::alloca~")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>*)")]
pub fn stub_0x3f3c38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>, std::alloca~")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::shared_ptr<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> *)")]
pub fn stub_0x3f3cdc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>, std::alloca~")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> *)")]
pub fn stub_0x3f3db0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}
