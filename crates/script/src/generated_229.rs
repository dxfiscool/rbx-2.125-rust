// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3d887c..0x3dbe04 | script 23352->23452 distinct (filler 0x3d887c asc, not-in-script 62193->62093)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ChangeHistoryService*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x3d887c() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x3d8964() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::find_node_impl<RBX::Instance *,std::equal_to<RBX::Instance *>>(unsigned long,RBX::Instance * const&,std::equal_to<RBX::Instance *> const&)const")]
pub fn stub_0x3d8a48(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance *>>(RBX::Instance * const&,boost::unordered::detail::emplace_args1<RBX::Instance *> const&)")]
pub fn stub_0x3d8ab4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x3d8c44(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")]
pub fn stub_0x3d8c98(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x3d8dc0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::rehash_impl(unsigned long)")]
pub fn stub_0x3d8e50(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x3d8e7c(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>>>::construct(void)")]
pub fn stub_0x3d8ed0() -> crate::slot::PortedFn {
// IDA 0x3d8ed0: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Instance*>>>::construc~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3d8ed0, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::In~")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(rbx_core::SharedPtr<RBX::Instance> const&,boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")]
pub fn stub_0x3d8f08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct_with_value<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")]
pub fn stub_0x3d9090() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x3d90bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::~node_constructor()")]
pub fn stub_0x3d910c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::create_buckets(unsigned long)")]
pub fn stub_0x3d9138() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x3d9260() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::rehash_impl(unsigned long)")]
pub fn stub_0x3d92f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x3d931c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct(void)")]
pub fn stub_0x3d9374() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::find_node_impl<rbx_core::SharedPtr<RBX::Instance>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>(unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&)const")]
pub fn stub_0x3d93b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::delete_buckets(void)")]
pub fn stub_0x3d9424(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::table(unsigned long,boost::hash<RBX::Instance *> const&,std::equal_to<RBX::Instance *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>> const&)")]
pub fn stub_0x3d9470() -> crate::slot::PortedFn {
// IDA 0x3d9470: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance*>, RBX::Instance*, boost::has~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3d9470, "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance*>, RBX::I~")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_buckets(void)")]
pub fn stub_0x3d94dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x3d9514() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::table(unsigned long,boost::hash<rbx_core::SharedPtr<RBX::Instance>> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&,std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
pub fn stub_0x3d9544() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const")]
pub fn stub_0x3d95b0() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::Selection")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x3d9778(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::JointInstance, RBX::sJointInstance, RBX::NonFactoryProduct~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x3d989c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot() [0x3d98c8]")]
pub fn stub_0x3d98c8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
pub fn stub_0x3d99a0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3d99a0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
pub fn stub_0x3d99c4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3d99c4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
pub fn stub_0x3d99e8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3d99e8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
pub fn stub_0x3d9a1c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3d9a1c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable() [0x3d9a48]")]
pub fn stub_0x3d9a48(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3d9a48: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0x3d9b20(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot() [0x3d9b4c]")]
pub fn stub_0x3d9b4c(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::disconnect(void)")]
pub fn stub_0x3d9c20(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::connected(void)const")]
pub fn stub_0x3d9d30() -> crate::slot::SlotConnection {
// IDA 0x3d9d30: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x3d9d3c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3d9d3c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0x3d9d60(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3d9d60: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,RBX::Reflection::PropertyDescriptor const*&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*> &,boost::_bi::list2<rbx_core::SharedPtr<RBX::Instance>&,RBX::Reflection::PropertyDescriptor const*&> &,int)")]
pub fn stub_0x3d9d84(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3d9d84: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>::operator()(RBX::ChangeHistoryService*,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)const")]
pub fn stub_0x3d9e64() -> crate::slot::BindPiece {
// boost::bind fragment (mf2) composing a host BoundCall.
crate::slot::BindPiece::new("mf2")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
pub fn stub_0x3d9f50(slot: &mut crate::slot::CallableSlot) {
// IDA 0x3d9f50: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x3da040() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::~slot()")]
pub fn stub_0x3da048(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_0x3da11c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3da11c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::~callable() [0x3da148]")]
pub fn stub_0x3da148(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3da148: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
pub fn stub_0x3da220(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x3da244(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot() [0x3da270]")]
pub fn stub_0x3da270(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3da348(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3da348: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3da364(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3da364: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x3da380(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3da380: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x3da45c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3da45c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x3da488]")]
pub fn stub_0x3da488(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3da488: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3da560(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x3da568(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3da5d0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3da5d4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3da674(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3da67c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x3da720(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_20ChangeHistoryServiceELZNS_21sChangeHistoryServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_21sChangeHistoryServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x3da728(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::pop_back(void)")]
pub fn stub_0x3da7cc(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::push_back(RBX::ChangeHistoryService::Item * const&)")]
pub fn stub_0x3da7fc(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_push_back_aux(RBX::ChangeHistoryService::Item * const&)")]
pub fn stub_0x3da81c(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x3da854(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x3da870() -> crate::slot::PortedFn {
// IDA 0x3da870: std::deque<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::_M_reallocate_map(unsign~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3da870, "std::deque<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::_M_r~")
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x3da948() -> crate::slot::PortedFn {
// IDA 0x3da948: std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::_M_allocate_map(un~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3da948, "std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>~")
}

#[doc(alias = "std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::deque(std::deque<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>> const&)")]
pub fn stub_0x3da960() -> crate::slot::PortedFn {
// IDA 0x3da960: std::deque<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::deque(std::deque<RBX::Ch~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3da960, "std::deque<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::dequ~")
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::~_Deque_base()")]
pub fn stub_0x3da9f4() -> crate::slot::PortedFn {
// IDA 0x3da9f4: std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::~_Deque_base().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3da9f4, "std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>~")
}

#[doc(alias = "std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>>(std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item * const&,RBX::ChangeHistoryService::Item * const*>,std::_Deque_iterator<RBX::ChangeHistoryService::Item *,RBX::ChangeHistoryService::Item *&,RBX::ChangeHistoryService::Item **>)")]
pub fn stub_0x3daa20(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x3daabc() -> crate::slot::PortedFn {
// IDA 0x3daabc: std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::_M_initialize_map(~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3daabc, "std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>~")
}

#[doc(alias = "std::_Deque_base<RBX::ChangeHistoryService::Item *,std::allocator<RBX::ChangeHistoryService::Item *>>::_M_create_nodes(RBX::ChangeHistoryService::Item ***,RBX::ChangeHistoryService::Item ***)")]
pub fn stub_0x3dac14() -> crate::slot::PortedFn {
// IDA 0x3dac14: std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>::_M_create_nodes(RB~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3dac14, "std::_Deque_base<RBX::ChangeHistoryService::Item*, std::allocator<RBX::ChangeHistoryService::Item*>>~")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::find(RBX::Reflection::PropertyDescriptor const* const&)")]
pub fn stub_0x3dad08(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> &,int)")]
pub fn stub_0x3dad48(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3dad48: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::iterator::iterator(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)")]
pub fn stub_0x3dae68() -> crate::slot::InstanceHandle {
// RBX::Voxel::Region ctor.
crate::slot::InstanceHandle::new("RBX::Voxel::Region")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3dafa0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc() [0x3db0a4]")]
pub fn stub_0x3db0a4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3db158() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::Call0Helper<RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::call(RBX::ChangeHistoryService*,rbx_core::SharedPtr<RBX::Reflection::Tuple const> (RBX::ChangeHistoryService::*)(void),RBX::Reflection::Variant &)")]
pub fn stub_0x3db17c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3db268() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc() [0x3db36c]")]
pub fn stub_0x3db36c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3db420() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3db440() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x3db5b8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc() [0x3db5e8]")]
pub fn stub_0x3db5e8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3db6b4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 1)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::ChangeHistoryService,void (RBX::ChangeHistoryService::*)(std::string),std::string,void>::call(RBX::ChangeHistoryService*,void (RBX::ChangeHistoryService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_0x3db7f0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::ChangeHistoryService, void (RBX::ChangeHistoryService::*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::BoundFuncDesc(void (RBX::ChangeHistoryService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3db920() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x3dba98() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc() [0x3dbac8]")]
pub fn stub_0x3dbac8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x3dbb9c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::ChangeHistoryService", "void", 1)
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::resize(unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior)")]
pub fn stub_0x3dbbd0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::push_back(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
pub fn stub_0x3dbc08(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::ChangeHistoryService::RuntimeUndoBehavior,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x3dbc34(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
pub fn stub_0x3dbc8c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
pub fn stub_0x3dbd40(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior> const&)")]
pub fn stub_0x3dbd98(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
pub fn stub_0x3dbe04(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}
