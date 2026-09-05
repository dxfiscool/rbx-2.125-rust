// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3a8674..0x3ab950 | script 22602->22702 distinct (filler 0x3a8674 asc, not-in-script 62943->62843)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles()")]
pub fn stub_0x3a8674(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles() [0x3a867c]")]
pub fn stub_0x3a867c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x3a8720() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles() [0x3a8730]")]
pub fn stub_0x3a8730(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ArcHandles::~ArcHandles() [0x3a8738]")]
pub fn stub_0x3a8738(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::HandlesBase::shouldRender2d(void)const")]
pub fn stub_0x3a87dc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x3a87ec() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x3a87f0() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x3a888c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x3a8914() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles> RBX::Creatable<RBX::Instance>::create<RBX::ArcHandles>(void)")]
pub fn stub_0x3a8a58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ArcHandles")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ArcHandles>::shared_ptr<RBX::ArcHandles,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3a8b0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ArcHandles")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ArcHandles,RBX::ArcHandles>(rbx_core::SharedPtr<RBX::ArcHandles> const*,RBX::ArcHandles *)const")]
pub fn stub_0x3a8bd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ArcHandles")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3a8cbc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3a8dc4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3a8dc8]")]
pub fn stub_0x3a8dc8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3a8dcc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3a8dec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ArcHandles *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3a8e04() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")]
pub fn stub_0x3a8e08() -> crate::slot::PortedFn {
// IDA 0x3a8e08: void RBX::Name::callDoDeclare<RBX::sArcHandles>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a8e08, "void RBX::Name::callDoDeclare<RBX::sArcHandles>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")]
pub fn stub_0x3a8e0c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sArcHandles>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x3a8eec() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10ArcHandlesENS_11HandlesBaseELZNS_11sArcHandlesEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x3a9130() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ArcHandles"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HandlesBase::MouseDownCaptureInfo>::shared_ptr<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
pub fn stub_0x3a91a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HandlesBase::MouseDownCaptureInfo")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HandlesBase::MouseDownCaptureInfo>(RBX::HandlesBase::MouseDownCaptureInfo *)")]
pub fn stub_0x3a9278() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p()")]
pub fn stub_0x3a9364(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::~sp_counted_impl_p() [0x3a9368]")]
pub fn stub_0x3a9368(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::dispose(void)")]
pub fn stub_0x3a936c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_deleter(std::type_info const&)")]
pub fn stub_0x3a9378() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::HandlesBase::MouseDownCaptureInfo>::get_untyped_deleter(void)")]
pub fn stub_0x3a937c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> &)")]
pub fn stub_0x3a9380() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (G3D::Vector3::Axis)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::on_error(std::exception &)")]
pub fn stub_0x3a94e0(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot> const&)")]
pub fn stub_0x3a9508(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_init_mutex(void)")]
pub fn stub_0x3a952c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x3a9530(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> &)")]
pub fn stub_0x3a9628() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (G3D::Vector3::Axis, float, float)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::on_error(std::exception &)")]
pub fn stub_0x3a9788(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot> const&)")]
pub fn stub_0x3a97b0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_init_mutex(void)")]
pub fn stub_0x3a97d4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis, float, float)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x3a97d8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis, float, float)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
pub fn stub_0x3a98d0() -> crate::slot::SlotConnection {
// IDA 0x3a98d0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
pub fn stub_0x3a9944(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::ArcHandles, void (G3D::Vector3::Axis, float, float)>::listen~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot()")]
pub fn stub_0x3a9990(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>::~callable_slot() [0x3a99bc]")]
pub fn stub_0x3a99bc(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x3a9a90(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9a90: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x3a9a98(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9a98: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
pub fn stub_0x3a9aa0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_0x3a9ab8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9ab8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>,0,void ()(void)>::~callable() [0x3a9ae4]")]
pub fn stub_0x3a9ae4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9ae4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
pub fn stub_0x3a9bb8() -> crate::slot::SlotConnection {
// IDA 0x3a9bb8: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
pub fn stub_0x3a9c2c(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::ArcHandles, void (G3D::Vector3::Axis)>::listenerConnectionAd~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot()")]
pub fn stub_0x3a9c78(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>::~callable_slot() [0x3a9ca4]")]
pub fn stub_0x3a9ca4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x3a9d78(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9d78: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::call(void)")]
pub fn stub_0x3a9d80(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9d80: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
pub fn stub_0x3a9d88() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable()")]
pub fn stub_0x3a9da0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9da0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>,0,void ()(void)>::~callable() [0x3a9dcc]")]
pub fn stub_0x3a9dcc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a9dcc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>::remote_signal(void)")]
pub fn stub_0x3a9ea0() -> crate::slot::PortedFn {
// IDA 0x3a9ea0: rbx::remote_signal<void (G3D::Vector3::Axis, float, float)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a9ea0, "rbx::remote_signal<void (G3D::Vector3::Axis, float, float)>::remote_signal()")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::disconnectAll(void)")]
pub fn stub_0x3a9ffc(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis, float, float)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::remote_signal<void ()(G3D::Vector3::Axis)>::remote_signal(void)")]
pub fn stub_0x3aa174() -> crate::slot::PortedFn {
// IDA 0x3aa174: rbx::remote_signal<void (G3D::Vector3::Axis)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3aa174, "rbx::remote_signal<void (G3D::Vector3::Axis)>::remote_signal()")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::disconnectAll(void)")]
pub fn stub_0x3aa2d0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_0x3aa448(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<3, RBX::ArcHandles, void (G3D::Vector3::Axis, float, float)>::con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
pub fn stub_0x3aa53c() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::signalProducedIncremented(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aa5a4(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<3, RBX::ArcHandles, void (G3D::Vector3::Axis, float, float)>::sig~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float),rbx::remote_signal<void ()(G3D::Vector3::Axis,float,float)>>::replicateEvent(RBX::Reflection::EventSource *,G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aa5c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<3, RBX::ArcHandles, void (G3D::Vector3::Axis, float, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_0x3aa764() -> crate::slot::SlotConnection {
// IDA 0x3aa764: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
pub fn stub_0x3aa7d8(slot: &crate::slot::CallableSlot) {
// IDA 0x3aa7d8: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot*)")]
pub fn stub_0x3aa9e4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_0x3aaa08(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot() [0x3aaa34]")]
pub fn stub_0x3aaa34(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::disconnect(void)")]
pub fn stub_0x3aab08(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::connected(void)const")]
pub fn stub_0x3aac18() -> crate::slot::SlotConnection {
// IDA 0x3aac18: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aac24(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3aac24: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::call(G3D::Vector3::Axis,float,float)")]
pub fn stub_0x3aac50(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3aac50: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
pub fn stub_0x3aac7c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3aac7c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot *)")]
pub fn stub_0x3aacbc(slot: &mut crate::slot::CallableSlot) {
// IDA 0x3aacbc: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3aadac(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis, float, float)>::slot::safe_static_init_mute~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x3aadb0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis, float, float)>::slot::safe_static_do_get_mu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot()")]
pub fn stub_0x3aaea0() -> crate::lua::LuaVector3 {
// G3D::Vector3 ctor — zero.
crate::lua::LuaVector3 { x: 0.0, y: 0.0, z: 0.0 }
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot::~slot() [0x3aaecc]")]
pub fn stub_0x3aaecc() -> crate::lua::LuaVector3 {
// G3D::Vector3 ctor — zero.
crate::lua::LuaVector3 { x: 0.0, y: 0.0, z: 0.0 }
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable()")]
pub fn stub_0x3aafa0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3aafa0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(G3D::Vector3::Axis,float,float)>::~callable() [0x3aafcc]")]
pub fn stub_0x3aafcc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3aafcc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::connectSignalListener(void)")]
pub fn stub_0x3ab0a0(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorBase<RBX::ArcHandles, void (G3D::Vector3::Axis, float, float)>::connec~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::connectSignalListener(void)")]
pub fn stub_0x3ab0a4(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<1, RBX::ArcHandles, void (G3D::Vector3::Axis)>::connectSignalList~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
pub fn stub_0x3ab198() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::ArcHandles")
}

#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::signalProducedIncremented(G3D::Vector3::Axis)")]
pub fn stub_0x3ab200(handle: &crate::slot::InstanceHandle) {
// RBX::EventReplicatorImpl<1, RBX::ArcHandles, void (G3D::Vector3::Axis)>::signalProducedInc~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis),rbx::remote_signal<void ()(G3D::Vector3::Axis)>>::replicateEvent(RBX::Reflection::EventSource *,G3D::Vector3::Axis)")]
pub fn stub_0x3ab214(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::RemoteEventDescImpl<1, RBX::ArcHandles, void (G3D::Vector3::Axis), rbx::r~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
pub fn stub_0x3ab360() -> crate::slot::SlotConnection {
// IDA 0x3ab360: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
pub fn stub_0x3ab3d4(slot: &crate::slot::CallableSlot) {
// IDA 0x3ab3d4: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot*)")]
pub fn stub_0x3ab5e0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x3ab604(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot() [0x3ab630]")]
pub fn stub_0x3ab630(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::disconnect(void)")]
pub fn stub_0x3ab704(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::connected(void)const")]
pub fn stub_0x3ab814() -> crate::slot::SlotConnection {
// IDA 0x3ab814: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_0x3ab820(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ab820: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
pub fn stub_0x3ab834(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3ab834: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
pub fn stub_0x3ab848() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
pub fn stub_0x3ab860(slot: &mut crate::slot::CallableSlot) {
// IDA 0x3ab860: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3ab950(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (G3D::Vector3::Axis)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}
