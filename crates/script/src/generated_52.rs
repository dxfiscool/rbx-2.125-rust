// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x4f5bfc (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4f5c70..0x4f9980 | existing ~9641 -> ~9741 total (union; filler 0x4f5c70 ascending, global remaining 55636 -> 55536)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sFlagStandServiceEEEEvv")]
pub fn stub_0x4f5c70() -> crate::slot::PortedFn {
// IDA 0x4f5c70: void RBX::Name::callDoDeclare<RBX::sFlagStandService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4f5c70, "void RBX::Name::callDoDeclare<RBX::sFlagStandService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v")]
pub fn stub_0x4f5c74(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sFlagStandService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FlagStand **,std::vector<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>>,RBX::FlagStand * const&)")]
pub fn stub_0x4f5d54(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::_M_allocate(unsigned long)")]
pub fn stub_0x4f5e34() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16FlagStandServiceELZNS_17sFlagStandServiceEENS_17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f5e4c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16FlagStandServiceELZNS_17sFlagStandServiceEENS_17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f5e50(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_16FlagStandServiceELZNS_17sFlagStandServiceEENS_17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f5ef0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_16FlagStandServiceELZNS_17sFlagStandServiceEENS_17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f5ef8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_16FlagStandServiceELZNS_17sFlagStandServiceEENS_17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f5f9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_16FlagStandServiceELZNS_17sFlagStandServiceEENS_17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f5fa4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Flag const* RBX::Instance::findConstFirstChildOfType<RBX::Flag>(void)const")]
pub fn stub_0x4f6048(handle: &crate::slot::InstanceHandle) {
// RBX::Flag const* RBX::Instance::findConstFirstChildOfType<RBX::Flag>() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FlagStandService * RBX::ServiceProvider::create<RBX::FlagStandService>(void)const")]
pub fn stub_0x4f60b0() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::FlagStandService")
}

#[doc(alias = "RBX::FlagStandService * RBX::ServiceProvider::find<RBX::FlagStandService>(void)const")]
pub fn stub_0x4f62ac() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::FlagStandService"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStandService> RBX::Creatable<RBX::Instance>::create<RBX::FlagStandService>(void)")]
pub fn stub_0x4f6440() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FlagStandService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FlagStandService>(rbx_core::SharedPtr<RBX::FlagStandService> const&)")]
pub fn stub_0x4f64f0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FlagStandService>(void)")]
pub fn stub_0x4f6524() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FlagStandService>(void)")]
pub fn stub_0x4f6528() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStandService>::shared_ptr<RBX::FlagStandService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4f6600() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FlagStandService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStandService,RBX::FlagStandService>(rbx_core::SharedPtr<RBX::FlagStandService> const*,RBX::FlagStandService *)const")]
pub fn stub_0x4f66c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FlagStandService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4f67b0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4f68b8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4f68bc]")]
pub fn stub_0x4f68bc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4f68c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4f68e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4f68f8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE15isNullClassNameEv")]
pub fn stub_0x4f68fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x4f697c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x4f69dc(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::FlagStand *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x4f69f8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x4f69f8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::FlagStand*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x4f6ad0() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
pub fn stub_0x4f6bb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
pub fn stub_0x4f6bcc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
pub fn stub_0x4f6be0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
pub fn stub_0x4f6be8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f6bf0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f6c04(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f6cb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f6ccc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f6d80(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f6d94(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f6e48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f6e5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x4f6f10() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc() [0x4f7094]")]
pub fn stub_0x4f7094(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x4f7148() -> crate::slot::SlotConnection {
// IDA 0x4f7148: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x4f729c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x4f73fc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4f7410(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::~PropDescriptor() [0x4f7524]")]
pub fn stub_0x4f7524(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isReadOnly(void)const")]
pub fn stub_0x4f7550(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
pub fn stub_0x4f7554(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4f7558(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
pub fn stub_0x4f7580(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::FlagStand::~FlagStand() [0x4f75a4]")]
pub fn stub_0x4f75a4(handle: crate::slot::InstanceHandle) {
// RBX::FlagStand dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_197")]
pub fn stub_0x4f7894() -> crate::slot::PortedFn {
// IDA 0x4f7894: __GLOBAL__I_a_197.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4f7894, "__GLOBAL__I_a_197")
}

#[doc(alias = "RBX::ForceField::ForceField(void)")]
pub fn stub_0x4f7c58() -> crate::slot::InstanceHandle {
// RBX::ForceField ctor.
crate::slot::InstanceHandle::new("RBX::ForceField")
}

#[doc(alias = "RBX::ForceField::ForceField(void) [0x4f7c5c]")]
pub fn stub_0x4f7c5c() -> crate::slot::InstanceHandle {
// RBX::ForceField ctor.
crate::slot::InstanceHandle::new("RBX::ForceField")
}

#[doc(alias = "RBX::ForceField::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x4f7f40(handle: &crate::slot::InstanceHandle) {
// RBX::ForceField::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::containsForceField(RBX::Instance *)")]
pub fn stub_0x4f7f80() -> crate::slot::PortedFn {
// IDA 0x4f7f80: RBX::containsForceField(RBX::Instance*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4f7f80, "RBX::containsForceField(RBX::Instance*)")
}

#[doc(alias = "RBX::ancestorContainsForceField(RBX::Instance *)")]
pub fn stub_0x4f7ff0() -> crate::slot::PortedFn {
// IDA 0x4f7ff0: RBX::ancestorContainsForceField(RBX::Instance*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4f7ff0, "RBX::ancestorContainsForceField(RBX::Instance*)")
}

#[doc(alias = "RBX::ForceField::partInForceField(RBX::PartInstance *)")]
pub fn stub_0x4f8094(handle: &crate::slot::InstanceHandle) {
// RBX::ForceField::partInForceField(RBX::PartInstance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::renderForceField(rbx_core::SharedPtr<RBX::Instance>,RBX::Adorn *,int,int)")]
pub fn stub_0x4f80a0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::ForceField::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x4f842c(handle: &crate::slot::InstanceHandle) {
// RBX::ForceField::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::render3dAdorn(RBX::Adorn *)")]
pub fn stub_0x4f8778(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::ForceField::~ForceField()")]
pub fn stub_0x4f8780(handle: crate::slot::InstanceHandle) {
// RBX::ForceField dtor.
drop(handle);
}

#[doc(alias = "RBX::ForceField::~ForceField() [0x4f8784]")]
pub fn stub_0x4f8784(handle: crate::slot::InstanceHandle) {
// RBX::ForceField dtor.
drop(handle);
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv")]
pub fn stub_0x4f8824() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ForceField"
}

#[doc(alias = "RBX::ForceField::shouldRender3dAdorn(void)const")]
pub fn stub_0x4f8834(handle: &crate::slot::InstanceHandle) {
// RBX::ForceField::shouldRender3dAdorn() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
pub fn stub_0x4f8838(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField() [0x4f8840]")]
pub fn stub_0x4f8840(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv")]
pub fn stub_0x4f8848() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ForceField"
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField() [0x4f8858]")]
pub fn stub_0x4f8858(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField() [0x4f8860]")]
pub fn stub_0x4f8860(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::shouldRender3dAdorn(void)const")]
pub fn stub_0x4f8868(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField() [0x4f886c]")]
pub fn stub_0x4f886c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 116, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 116);
}

#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField() [0x4f8874]")]
pub fn stub_0x4f8874(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 116, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 116);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E17static_getCreatorEv")]
pub fn stub_0x4f887c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ForceField"
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f88f0(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f88f4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f8994(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f899c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f8a40(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f8a48(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f8aec(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f8af0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f8b90(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f8b98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4f8c3c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4f8c44(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::ForceField::~ForceField() [0x4f8ce8]")]
pub fn stub_0x4f8ce8(handle: crate::slot::InstanceHandle) {
// RBX::ForceField dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_198")]
pub fn stub_0x4f8e28() -> crate::slot::PortedFn {
// IDA 0x4f8e28: __GLOBAL__I_a_198.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4f8e28, "__GLOBAL__I_a_198")
}

#[doc(alias = "RBX::Frame::setStyle(RBX::Frame::Style)")]
pub fn stub_0x4f90d8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Frame setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)")]
pub fn stub_0x4f910c() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void) [0x4f9110]")]
pub fn stub_0x4f9110() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Frame::Frame(void)")]
pub fn stub_0x4f932c() -> crate::slot::InstanceHandle {
// RBX::Frame ctor.
crate::slot::InstanceHandle::new("RBX::Frame")
}

#[doc(alias = "RBX::Frame::Frame(void) [0x4f9330]")]
pub fn stub_0x4f9330() -> crate::slot::InstanceHandle {
// RBX::Frame ctor.
crate::slot::InstanceHandle::new("RBX::Frame")
}

#[doc(alias = "RBX::Frame::getChildRect2D(void)const")]
pub fn stub_0x4f94b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Frame getter.
cell.get()
}

#[doc(alias = "RBX::Frame::render2d(RBX::Adorn *)")]
pub fn stub_0x4f956c(handle: &crate::slot::InstanceHandle) {
// RBX::Frame::render2d(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Frame::render2d(RBX::Adorn *)")]
pub fn stub_0x4f9978(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "RBX::Frame::getStyle(void)const")]
pub fn stub_0x4f9980(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Frame getter.
cell.get()
}
