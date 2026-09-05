// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x451030..0x455308 | script 25252->25352 distinct (filler 0x451030 asc, not-in-script 60293->60193)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x451030(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14PhysicsServiceELZNS_15sPhysicsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_15sPhysicsServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x451038(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsService>::shared_ptr<RBX::PhysicsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4510dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PhysicsService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PhysicsService,RBX::PhysicsService>(rbx_core::SharedPtr<RBX::PhysicsService> const*,RBX::PhysicsService *)const")]
pub fn stub_0x4511a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PhysicsService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x451290() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x451398(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x45139c]")]
pub fn stub_0x45139c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4513a0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4513c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PhysicsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4513d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CollectionService> RBX::Creatable<RBX::Instance>::create<RBX::CollectionService>(void)")]
pub fn stub_0x4515f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CollectionService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::CollectionService>(rbx_core::SharedPtr<RBX::CollectionService> const&)")]
pub fn stub_0x4516a0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CollectionService>::shared_ptr<RBX::CollectionService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4518dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CollectionService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CollectionService,RBX::CollectionService>(rbx_core::SharedPtr<RBX::CollectionService> const*,RBX::CollectionService *)const")]
pub fn stub_0x4519a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CollectionService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x451a90() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x451b98(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x451b9c]")]
pub fn stub_0x451b9c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x451ba0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x451bc0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CollectionService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x451bd8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::JointsService> RBX::Creatable<RBX::Instance>::create<RBX::JointsService>(void)")]
pub fn stub_0x451c7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::JointsService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::JointsService>(rbx_core::SharedPtr<RBX::JointsService> const&)")]
pub fn stub_0x451d2c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::JointsService>::shared_ptr<RBX::JointsService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x451d60() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::JointsService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::JointsService,RBX::JointsService>(rbx_core::SharedPtr<RBX::JointsService> const*,RBX::JointsService *)const")]
pub fn stub_0x451e28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::JointsService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x451f14() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x452020(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x452024() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x45203c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x452348(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>>::~callable_slot() [0x452374]")]
pub fn stub_0x452374(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
pub fn stub_0x452564(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x452564: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
pub fn stub_0x452588(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x452588: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::DataModel *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
pub fn stub_0x4525ac(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x4525ac: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
pub fn stub_0x4527ac(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4527ac: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::DataModel,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::DataModel*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable() [0x4527d8]")]
pub fn stub_0x4527d8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4527d8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundService>(void)")]
pub fn stub_0x452950() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService>::shared_ptr<RBX::Soundscape::SoundService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x452a00() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundService,RBX::Soundscape::SoundService>(rbx_core::SharedPtr<RBX::Soundscape::SoundService> const*,RBX::Soundscape::SoundService *)const")]
pub fn stub_0x452ac8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x452bb4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x452cbc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x452cc0]")]
pub fn stub_0x452cc0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x452cc4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::Soundscape::SoundService>(rbx_core::SharedPtr<RBX::Soundscape::SoundService> const&)")]
pub fn stub_0x452f10(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService> RBX::Creatable<RBX::Instance>::create<RBX::RunService>(void)")]
pub fn stub_0x452f48() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RunService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const&)")]
pub fn stub_0x452ff8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService>::shared_ptr<RBX::CoreGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x453040() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreGuiService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(rbx_core::SharedPtr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const")]
pub fn stub_0x453108() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CoreGuiService")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4531f8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService> RBX::Creatable<RBX::Instance>::create<RBX::StarterGuiService>(void)")]
pub fn stub_0x453374() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterGuiService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::StarterGuiService>(rbx_core::SharedPtr<RBX::StarterGuiService> const&)")]
pub fn stub_0x453424(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGuiService>::shared_ptr<RBX::StarterGuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x453660() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterGuiService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterGuiService,RBX::StarterGuiService>(rbx_core::SharedPtr<RBX::StarterGuiService> const*,RBX::StarterGuiService *)const")]
pub fn stub_0x453728() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterGuiService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x453814() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x453920(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x453924() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x45393c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::StarterPackService>(void)")]
pub fn stub_0x453c88() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StarterPackService>::shared_ptr<RBX::StarterPackService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x453d60() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterPackService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterPackService,RBX::StarterPackService>(rbx_core::SharedPtr<RBX::StarterPackService> const*,RBX::StarterPackService *)const")]
pub fn stub_0x453e28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StarterPackService")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x453f18(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE15isNullClassNameEv")]
pub fn stub_0x453f20(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerHUD>::shared_ptr<RBX::PlayerHUD,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x453fc0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PlayerHUD")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerHUD,RBX::PlayerHUD>(rbx_core::SharedPtr<RBX::PlayerHUD> const*,RBX::PlayerHUD *)const")]
pub fn stub_0x454088() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PlayerHUD")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x454174() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x45427c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x454280]")]
pub fn stub_0x454280(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x454284() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4542a4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4542bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const")]
pub fn stub_0x4542c0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::LocalBackpack"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)")]
pub fn stub_0x454434() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalBackpack")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const&)")]
pub fn stub_0x4544e4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLocalBackpackEEEERKS0_v")]
pub fn stub_0x454518(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sLocalBackpack>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLocalBackpackEEEEvv")]
pub fn stub_0x45455c() -> crate::slot::PortedFn {
// IDA 0x45455c: void RBX::Name::callDoDeclare<RBX::sLocalBackpack>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x45455c, "void RBX::Name::callDoDeclare<RBX::sLocalBackpack>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v")]
pub fn stub_0x454560(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sLocalBackpack>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LocalBackpack>(void)")]
pub fn stub_0x454644() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LocalBackpack>(void)")]
pub fn stub_0x454648() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x454720() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalBackpack")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const")]
pub fn stub_0x4547e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LocalBackpack")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4548d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4549dc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4549e0]")]
pub fn stub_0x4549e0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4549e4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x454a04() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x454a1c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE15isNullClassNameEv")]
pub fn stub_0x454a20(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorD2Ev")]
pub fn stub_0x454ac0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MarketplaceService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7Creator6createEv")]
pub fn stub_0x454b60() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MarketplaceService"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)")]
pub fn stub_0x454ca4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MarketplaceService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x454d58() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x454e60(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x454e64]")]
pub fn stub_0x454e64(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x454e68() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x454e88() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x454ea0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v")]
pub fn stub_0x454ea4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sMarketplaceService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv")]
pub fn stub_0x454ee8() -> crate::slot::PortedFn {
// IDA 0x454ee8: void RBX::Name::callDoDeclare<RBX::sMarketplaceService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x454ee8, "void RBX::Name::callDoDeclare<RBX::sMarketplaceService>()")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev")]
pub fn stub_0x454ef0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MarketplaceService"
}

#[doc(alias = "RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const")]
pub fn stub_0x455118() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::MarketplaceService"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)")]
pub fn stub_0x455308() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}
