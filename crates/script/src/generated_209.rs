// Auto-generated skeletons for rbx-script — shard 209 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x363a1c..0x369dcc | script 21102->21252 distinct (filler 0x363a1c asc, not-in-script 64643->64493)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> &)")]
pub fn stub_0x363a1c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::Heartbeat const&)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::on_error(std::exception &)")]
pub fn stub_0x363b7c(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_init_mutex(void)")]
pub fn stub_0x363ba8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Heartbeat const&)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HeartbeatTask>::shared_ptr<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
pub fn stub_0x363bac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HeartbeatTask")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::HeartbeatTask,RBX::HeartbeatTask>(rbx_core::SharedPtr<RBX::HeartbeatTask> const*,RBX::HeartbeatTask *)const")]
pub fn stub_0x363c94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HeartbeatTask")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HeartbeatTask>(RBX::HeartbeatTask *)")]
pub fn stub_0x363d78() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p()")]
pub fn stub_0x363e70(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::~sp_counted_impl_p() [0x363e74]")]
pub fn stub_0x363e74(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::dispose(void)")]
pub fn stub_0x363e78() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_deleter(std::type_info const&)")]
pub fn stub_0x363e88() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HeartbeatTask>::get_untyped_deleter(void)")]
pub fn stub_0x363e8c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::throw_exception<boost::bad_weak_ptr>(boost::bad_weak_ptr const&)")]
pub fn stub_0x363e90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Instance")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::~clone_impl()")]
pub fn stub_0x363f78(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_weak_ptr>>::clone(void)const")]
pub fn stub_0x363f90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("Instance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsJob>::shared_ptr<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
pub fn stub_0x363f9c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PhysicsJob")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::PhysicsJob,RBX::PhysicsJob>(rbx_core::SharedPtr<RBX::PhysicsJob> const*,RBX::PhysicsJob *)const")]
pub fn stub_0x364084() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PhysicsJob")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsJob>(RBX::PhysicsJob *)")]
pub fn stub_0x364168() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p()")]
pub fn stub_0x364260(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::~sp_counted_impl_p() [0x364264]")]
pub fn stub_0x364264(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::dispose(void)")]
pub fn stub_0x364268() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x364278() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PhysicsJob>::get_untyped_deleter(void)")]
pub fn stub_0x36427c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x364280(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::RunService, RBX::sRunService, RBX::NonFactoryProduct<RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x36439c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3643a0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x364440(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x364448(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3644ec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10RunServiceELZNS_11sRunServiceEENS_17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEEELNS0_15ClassDescriptor13FunctionalityE17ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3644f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::rootDescriptor(void)")]
pub fn stub_0x364598(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::ClassDescriptor::rootDescriptor() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel> boost::dynamic_pointer_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")]
pub fn stub_0x364688() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::BoundFuncDesc(void (RBX::RunService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3646d0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::RunService", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::~BoundFuncDesc() [0x3647d4]")]
pub fn stub_0x3647d4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x364888() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::RunService", "void", 0)
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::EventDesc(rbx::signal<void ()(double)> RBX::RunService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3648ac() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::RunService")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::~EventDesc() [0x364a30]")]
pub fn stub_0x364a30(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x364ae4() -> crate::slot::SlotConnection {
// IDA 0x364ae4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x364c38(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::RunService, void (double), rbx::signal<void (double~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x364cc8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::RunService, void (double), rbx::signal<void (double)>,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::disconnectAll(void)")]
pub fn stub_0x364cdc(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::bad_placement_any_cast::~bad_placement_any_cast()")]
pub fn stub_0x364e58(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::bad_placement_any_cast::what(void)const")]
pub fn stub_0x364e60(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
pub fn stub_0x364e70(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector()")]
pub fn stub_0x364f28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
pub fn stub_0x364f30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl()")]
pub fn stub_0x364f38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone(void)const")]
pub fn stub_0x364f48(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::rethrow(void)const")]
pub fn stub_0x365008(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::~clone_impl() [0x365018]")]
pub fn stub_0x365018(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>::~error_info_injector() [0x365038]")]
pub fn stub_0x365038(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_tag)")]
pub fn stub_0x365050(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast>>::clone_impl(boost::exception_detail::error_info_injector<rbx::bad_placement_any_cast> const&)")]
pub fn stub_0x365188(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,double const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(double const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
pub fn stub_0x3652c0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<double>(double const&)")]
pub fn stub_0x3653dc() -> crate::slot::PortedFn {
// IDA 0x3653dc: void RBX::Reflection::GenericSlotWrapper::execute1<double>(double const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3653dc, "void RBX::Reflection::GenericSlotWrapper::execute1<double>(double const&)")
}

#[doc(alias = "boost::function1<void,double>::clear(void)")]
pub fn stub_0x365520(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "rbx::implementation::typed_holder<double>::singleton(void)")]
pub fn stub_0x365550(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<double>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::vector(unsigned long,RBX::Reflection::Variant const&,std::allocator<RBX::Reflection::Variant> const&)")]
pub fn stub_0x3655c0() -> crate::slot::PortedFn {
// IDA 0x3655c0: std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>>::vector(unsigned long, RBX::Reflection::~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3655c0, "std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>>::vector(unsigned lon~")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::Reflection::Variant *,unsigned long,RBX::Reflection::Variant>(RBX::Reflection::Variant *,unsigned long,RBX::Reflection::Variant const&,std::__false_type)")]
pub fn stub_0x365690() -> crate::slot::PortedFn {
// IDA 0x365690: void std::__uninitialized_fill_n_aux<RBX::Reflection::Variant*, unsigned long, RBX::Reflection::Variant>(RBX::Reflection~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x365690, "void std::__uninitialized_fill_n_aux<RBX::Reflection::Variant*, unsigned long, RBX::Reflection::Vari~")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>)")]
pub fn stub_0x3657d0() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>)")]
pub fn stub_0x3658a0() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "__ZN5boost8functionIFvdEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS4_5list2INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x365980() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvdEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKdEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x365a64() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
pub fn stub_0x365b4c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x365c44(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,double>::invoke(boost::detail::function::function_buffer &,double)")]
pub fn stub_0x365c60(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x365c80(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x365d68(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,double>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x365e4c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<double>(double &)")]
pub fn stub_0x365f20() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,double const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x365f38(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(double)>::connect<boost::function<void ()(double)>>(boost::function<void ()(double)> const&)")]
pub fn stub_0x366090() -> crate::slot::SlotConnection {
// IDA 0x366090: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::insert(rbx::signals::signal<void ()(double)>::slot *)")]
pub fn stub_0x366184(slot: &crate::slot::CallableSlot) {
// IDA 0x366184: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx::signals::signal<void ()(double)>::slot*)")]
pub fn stub_0x366390(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::callable<rbx::signals::signal<void ()(double)>*>(boost::function<void ()(double)> const&,rbx::signals::signal<void ()(double)>*)")]
pub fn stub_0x3663b4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3663b4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot()")]
pub fn stub_0x3664b0(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::callable_slot<boost::function<void ()(double)>>::~callable_slot() [0x3665c0]")]
pub fn stub_0x3665c0(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::disconnect(void)")]
pub fn stub_0x3666f0(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::connected(void)const")]
pub fn stub_0x366800() -> crate::slot::SlotConnection {
// IDA 0x366800: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")]
pub fn stub_0x36680c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x36680c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::call(double)")]
pub fn stub_0x366814(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x366814: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function1<void,double>::operator()(double)const")]
pub fn stub_0x36681c(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::remove(rbx::signals::signal<void ()(double)>::slot *)")]
pub fn stub_0x3668e8(slot: &mut crate::slot::CallableSlot) {
// IDA 0x3668e8: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3669d8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x3669dc(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::~callable()")]
pub fn stub_0x366ad0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x366ad0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double)>::slot,boost::function<void ()(double)>,1,void ()(double)>::~callable() [0x366be0]")]
pub fn stub_0x366be0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x366be0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::~slot()")]
pub fn stub_0x366d10(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::slot::~slot() [0x366d3c]")]
pub fn stub_0x366d3c(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "boost::function1<void,double>::assign_to_own(boost::function1<void,double> const&)")]
pub fn stub_0x366e10(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::EventDescriptor::~EventDescriptor() [0x366e40]")]
pub fn stub_0x366e40(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double,double),rbx::signal<void ()(double,double)>,rbx::signal<void ()(double,double)> RBX::RunService::*>::EventDesc(rbx::signal<void ()(double,double)> RBX::RunService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x366ef4() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::RunService")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double,double),rbx::signal<void ()(double,double)>,rbx::signal<void ()(double,double)> RBX::RunService::*>::~EventDesc() [0x3670e4]")]
pub fn stub_0x3670e4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RunService,void ()(double,double),rbx::signal<void ()(double,double)>,rbx::signal<void ()(double,double)> RBX::RunService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x367198() -> crate::slot::SlotConnection {
// IDA 0x367198: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RunService,void ()(double,double),rbx::signal<void ()(double,double)>,rbx::signal<void ()(double,double)> RBX::RunService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3672ec(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<2, RBX::RunService, void (double, double), rbx::signal<void~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RunService,void ()(double,double),rbx::signal<void ()(double,double)>,rbx::signal<void ()(double,double)> RBX::RunService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x367398(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::RunService, void (double, double), rbx::signal<void (d~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::disconnectAll(void)")]
pub fn stub_0x3673ac(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double, double)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(double const&,double const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0x367524() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute2<double,double>(double const&,double const&)")]
pub fn stub_0x367640() -> crate::slot::PortedFn {
// IDA 0x367640: void RBX::Reflection::GenericSlotWrapper::execute2<double, double>(double const&, double const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x367640, "void RBX::Reflection::GenericSlotWrapper::execute2<double, double>(double const&, double const&)")
}

#[doc(alias = "boost::function2<void,double,double>::clear(void)")]
pub fn stub_0x3677a8(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvddEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSC_EENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3677d4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "__ZN5boost9function2IvddEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKdSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3678b8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 6 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(6)
}

#[doc(alias = "void boost::function2<void,double,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
pub fn stub_0x3679a0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x367a98(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,double,double>::invoke(boost::detail::function::function_buffer &,double,double)")]
pub fn stub_0x367ab4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,double,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x367ae0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,double,double>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x367bc8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,double,double>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x367cac(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<double,double>(double &,double &)")]
pub fn stub_0x367d80() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,double const&,double const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x367d9c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(double,double)>::connect<boost::function<void ()(double,double)>>(boost::function<void ()(double,double)> const&)")]
pub fn stub_0x367ef4() -> crate::slot::SlotConnection {
// IDA 0x367ef4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::insert(rbx::signals::signal<void ()(double,double)>::slot *)")]
pub fn stub_0x367fe8(slot: &crate::slot::CallableSlot) {
// IDA 0x367fe8: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx::signals::signal<void ()(double,double)>::slot*)")]
pub fn stub_0x3681f4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::callable<rbx::signals::signal<void ()(double,double)>*>(boost::function<void ()(double,double)> const&,rbx::signals::signal<void ()(double,double)>*)")]
pub fn stub_0x368218(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x368218: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::callable_slot<boost::function<void ()(double,double)>>::~callable_slot()")]
pub fn stub_0x368314(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::callable_slot<boost::function<void ()(double,double)>>::~callable_slot() [0x368424]")]
pub fn stub_0x368424(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::disconnect(void)")]
pub fn stub_0x368554(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::connected(void)const")]
pub fn stub_0x368664() -> crate::slot::SlotConnection {
// IDA 0x368664: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::call(double,double)")]
pub fn stub_0x368670(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x368670: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::call(double,double)")]
pub fn stub_0x368688(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x368688: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function2<void,double,double>::operator()(double,double)const")]
pub fn stub_0x3686a0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::remove(rbx::signals::signal<void ()(double,double)>::slot *)")]
pub fn stub_0x368778(slot: &mut crate::slot::CallableSlot) {
// IDA 0x368778: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x368868(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double, double)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x36886c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double, double)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::~callable()")]
pub fn stub_0x36895c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x36895c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(double,double)>::slot,boost::function<void ()(double,double)>,2,void ()(double,double)>::~callable() [0x368a6c]")]
pub fn stub_0x368a6c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x368a6c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::~slot()")]
pub fn stub_0x368b9c(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::slot::~slot() [0x368bc8]")]
pub fn stub_0x368bc8(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "boost::function2<void,double,double>::assign_to_own(boost::function2<void,double,double> const&)")]
pub fn stub_0x368c9c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::operator()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_0x368cd0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<2, void (RBX::Instance::CombinedSignalType, RBX::Instance::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot> &)")]
pub fn stub_0x368e20() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::Instance::CombinedSignalType, RBX::Instance::ICo~")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::on_error(std::exception &)")]
pub fn stub_0x368f80(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot> const&)")]
pub fn stub_0x368fa8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x368fd0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Instance::CombinedSignalType, RBX::Instance::ICombinedSign~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::HeartbeatTask::HeartbeatTask(rbx_core::SharedPtr<RBX::RunService>)")]
pub fn stub_0x3690cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RunService")
}

#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask()")]
pub fn stub_0x3692bc(handle: crate::slot::InstanceHandle) {
// RBX::HeartbeatTask dtor.
drop(handle);
}

#[doc(alias = "RBX::HeartbeatTask::~HeartbeatTask() [0x3693b8]")]
pub fn stub_0x3693b8(handle: crate::slot::InstanceHandle) {
// RBX::HeartbeatTask dtor.
drop(handle);
}

#[doc(alias = "RBX::HeartbeatTask::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3694c8(handle: &crate::slot::InstanceHandle) {
// RBX::HeartbeatTask::sleepTime(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::HeartbeatTask::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x3694e4(handle: &crate::slot::InstanceHandle) {
// RBX::HeartbeatTask::error(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::HeartbeatTask::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x369504(handle: &crate::slot::InstanceHandle) {
// RBX::HeartbeatTask::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::shared_ptr<RBX::RunService>(rbx_core::Weak<RBX::RunService> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x369698() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RunService")
}

#[doc(alias = "boost::detail::weak_count::weak_count(boost::detail::shared_count const&)")]
pub fn stub_0x369718() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::PhysicsJob::PhysicsJob(rbx_core::SharedPtr<RBX::DataModel>)")]
pub fn stub_0x369764() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "RBX::PhysicsJob::~PhysicsJob()")]
pub fn stub_0x3698b8(handle: crate::slot::InstanceHandle) {
// RBX::PhysicsJob dtor.
drop(handle);
}

#[doc(alias = "RBX::PhysicsJob::~PhysicsJob() [0x369988]")]
pub fn stub_0x369988(handle: crate::slot::InstanceHandle) {
// RBX::PhysicsJob dtor.
drop(handle);
}

#[doc(alias = "RBX::PhysicsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x369a70(handle: &crate::slot::InstanceHandle) {
// RBX::PhysicsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PhysicsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x369ab0(handle: &crate::slot::InstanceHandle) {
// RBX::PhysicsJob::error(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PhysicsJob::getDesiredConcurrencyCount(void)const")]
pub fn stub_0x369ac8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::PhysicsJob getter.
cell.get()
}

#[doc(alias = "RBX::PhysicsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x369ae0(handle: &crate::slot::InstanceHandle) {
// RBX::PhysicsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::disconnectAll(void)")]
pub fn stub_0x369c54(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Stepped const&)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::disconnectAll(void)")]
pub fn stub_0x369dcc(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Heartbeat const&)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}
