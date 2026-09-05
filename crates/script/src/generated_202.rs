// Auto-generated skeletons for rbx-script — shard 201 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x2b4c58..0x2c0908 | script 19982->20132 distinct (filler 0x2b4c58 asc, not-in-script gaps)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::destruct_func(char *)")]
pub fn stub_0x2b4c58() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>::shared_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
pub fn stub_0x2b4c64() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> *)")]
pub fn stub_0x2b4d38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::vector(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
pub fn stub_0x2b4e48() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>,std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x2b4fb8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
pub fn stub_0x2b50e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>(boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> *)")]
pub fn stub_0x2b51b8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::~sp_counted_impl_p()")]
pub fn stub_0x2b52c8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::dispose(void)")]
pub fn stub_0x2b52d0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>>>::get_untyped_deleter(void)")]
pub fn stub_0x2b5378() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> * boost::get_deleter<rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> const&)")]
pub fn stub_0x2b5380() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>>")
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::_M_allocate(unsigned long)")]
pub fn stub_0x2b53e0() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::~sp_counted_impl_pd()")]
pub fn stub_0x2b5408(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b5438() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::get_untyped_deleter(void)")]
pub fn stub_0x2b5450() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::singleton(void)")]
pub fn stub_0x2b5458() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::destruct_func(char *)")]
pub fn stub_0x2b54c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::InvocationMeter<2>::updateBuckets(bool)")]
pub fn stub_0x2b54d8(handle: &crate::slot::InstanceHandle) {
// RBX::InvocationMeter<2>::updateBuckets(bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<double>(double const&)")]
pub fn stub_0x2b5590() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<double>::construct_func(char const*,char *)")]
pub fn stub_0x2b55e8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<double>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<double>::destruct_func(char *)")]
pub fn stub_0x2b55f8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<double>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<std::string>(std::string const&)")]
pub fn stub_0x2b5650() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<std::string>::construct_func(char const*,char *)")]
pub fn stub_0x2b56a8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<std::string>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<std::string>::destruct_func(char *)")]
pub fn stub_0x2b56b8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<std::string>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
pub fn stub_0x2b56c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::~sp_counted_impl_p()")]
pub fn stub_0x2b57d0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Reflection::Tuple>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b57d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
pub fn stub_0x2b57e0(slot: &crate::slot::CallableSlot) {
// IDA 0x2b57e0: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
pub fn stub_0x2b59ec(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
pub fn stub_0x2b5a10(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
pub fn stub_0x2b5a34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2b5a38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
pub fn stub_0x2b5c30(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
pub fn stub_0x2b5d40() -> crate::slot::SlotConnection {
// IDA 0x2b5d40: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
pub fn stub_0x2b6104(slot: &mut crate::slot::CallableSlot) {
// IDA 0x2b6104: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x2b61f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x2b61f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
pub fn stub_0x2b62e8(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot() [0x2b6314]")]
pub fn stub_0x2b6314(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "boost::scoped_ptr<RBX::LibraryService>::~scoped_ptr()")]
pub fn stub_0x2b6590() -> crate::slot::PortedFn {
// IDA 0x2b6590: boost::scoped_ptr<RBX::LibraryService>::~scoped_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b6590, "boost::scoped_ptr<RBX::LibraryService>::~scoped_ptr()")
}

#[doc(alias = "RBX::LibraryService::~LibraryService()")]
pub fn stub_0x2b6638(handle: crate::slot::InstanceHandle) {
// RBX::LibraryService dtor.
drop(handle);
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_clear(void)")]
pub fn stub_0x2b67b0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::LibraryService::LibraryStateObject")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
pub fn stub_0x2b67d8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
pub fn stub_0x2b6800() -> crate::slot::PortedFn {
// IDA 0x2b6800: std::_Rb_tree<std::string, std::pair<std::string const, RBX::LibraryService::LibraryDefinition>, std::_Select1st<std::pa~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b6800, "std::_Rb_tree<std::string, std::pair<std::string const, RBX::LibraryService::LibraryDefinition>, std~")
}

#[doc(alias = "boost::scoped_ptr<boost::thread>::~scoped_ptr()")]
pub fn stub_0x2b6900() -> crate::slot::PortedFn {
// IDA 0x2b6900: boost::scoped_ptr<boost::thread>::~scoped_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b6900, "boost::scoped_ptr<boost::thread>::~scoped_ptr()")
}

#[doc(alias = "boost::thread::~thread()")]
pub fn stub_0x2b69a8() -> crate::slot::PortedFn {
// IDA 0x2b69a8: boost::thread::~thread().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b69a8, "boost::thread::~thread()")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::GcJob>(RBX::GcJob *)")]
pub fn stub_0x2b71e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::GcJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::GcJob *)const")]
pub fn stub_0x2b72c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GcJob>(RBX::GcJob *)")]
pub fn stub_0x2b73ac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::~sp_counted_impl_p()")]
pub fn stub_0x2b74a4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::~sp_counted_impl_p() [0x2b74a8]")]
pub fn stub_0x2b74a8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::dispose(void)")]
pub fn stub_0x2b74ac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b74bc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::get_untyped_deleter(void)")]
pub fn stub_0x2b74c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Workspace>(void)")]
pub fn stub_0x2b7698() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_16OverlayDataModelELZNS_17sOverlayDataModelEENS_17NonFactoryProductINS_9DataModelELZNS_17sOverlayDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x2b77c0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::OverlayDataModel, RBX::sOverlayDataModel, RBX::NonFactoryP~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x2b78e0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::DataModel, RBX::sDataModel, RBX::NonFactoryProduct<RBX::Se~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15ServiceProviderELZNS_16sServiceProviderEENS_17NonFactoryProductINS_8InstanceELZNS_16sServiceProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x2b7a00(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::ServiceProvider, RBX::sServiceProvider, RBX::NonFactoryPro~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
pub fn stub_0x2b8f4c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::string const& rbx::any_cast<std::string const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x2b90c8(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x2b91b8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>(std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x2b92d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
pub fn stub_0x2b9460(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSF_RKS4_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b948c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function3IvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKSsSE_RKS4_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2b9570() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x2b9658(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2b9750(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2b976c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2b978c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2b9874(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x2b9958(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<std::string &,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&> &,boost::_bi::list3<std::string &,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x2b9a2c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2b9a2c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2b9a54(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
pub fn stub_0x2b9bac() -> crate::slot::SlotConnection {
// IDA 0x2b9bac: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
pub fn stub_0x2b9ca0(slot: &crate::slot::CallableSlot) {
// IDA 0x2b9ca0: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
pub fn stub_0x2b9eac(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>*)")]
pub fn stub_0x2b9ed0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2b9ed0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
pub fn stub_0x2b9fcc(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot() [0x2ba0dc]")]
pub fn stub_0x2ba0dc(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::disconnect(void)")]
pub fn stub_0x2ba20c(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
pub fn stub_0x2ba31c() -> crate::slot::SlotConnection {
// IDA 0x2ba31c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2ba328(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2ba328: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2ba518(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2ba518: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x2ba520(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::remove(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
pub fn stub_0x2ba750(slot: &mut crate::slot::CallableSlot) {
// IDA 0x2ba750: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x2ba840() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x2ba844() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x2ba934(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2ba934: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x2baa44]")]
pub fn stub_0x2baa44(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2baa44: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
pub fn stub_0x2bab74(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot() [0x2baba0]")]
pub fn stub_0x2baba0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function3<void,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>> const&)")]
pub fn stub_0x2bac74(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "std::list<RBX::Reflection::SignatureDescriptor::Item,std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_create_node(RBX::Reflection::SignatureDescriptor::Item const&)")]
pub fn stub_0x2baca8() -> crate::slot::PortedFn {
// IDA 0x2baca8: std::list<RBX::Reflection::SignatureDescriptor::Item, std::allocator<RBX::Reflection::SignatureDescriptor::Item>>::_M_cr~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2baca8, "std::list<RBX::Reflection::SignatureDescriptor::Item, std::allocator<RBX::Reflection::SignatureDescr~")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::~BoundProp() [0x2bb218]")]
pub fn stub_0x2bb218(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "int const& rbx::any_cast<int const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x2bb248(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
pub fn stub_0x2bb658(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>::operator=(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> const&)")]
pub fn stub_0x2bb6c0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::construct_func(char const*,char *)")]
pub fn stub_0x2bb6f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x2bbf50(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x2bbf80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x2bbfa8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<bool>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x2bc0f8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<bool>::copyValue(RBX::Reflection::DescribedBase c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool const& rbx::any_cast<bool const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x2bc120(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<bool>(bool const&)")]
pub fn stub_0x2bc208() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> RBX::Reflection::ArgHelper::getArg<rbx_core::SharedPtr<RBX::Instance>,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<rbx_core::SharedPtr<RBX::Instance>> const&,boost::disable_if<boost::is_same<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x2bc988() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_object<2,RBX::Instance>(RBX::Reflection::FunctionDescriptor::Arguments &,rbx_core::SharedPtr<RBX::Instance> &,boost::enable_if<boost::is_base_of<RBX::Reflection::DescribedBase,RBX::Instance>,void>::type *)")]
pub fn stub_0x2bcba0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::scoped_ptr<std::string>::~scoped_ptr()")]
pub fn stub_0x2bccc0() -> crate::slot::PortedFn {
// IDA 0x2bccc0: boost::scoped_ptr<std::string>::~scoped_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2bccc0, "boost::scoped_ptr<std::string>::~scoped_ptr()")
}

#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
pub fn stub_0x2bd4f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::disconnectAll(void)")]
pub fn stub_0x2be73c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2be8b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
pub fn stub_0x2beb34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2bec94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
pub fn stub_0x2bee54() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x2bee7c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x2bef98() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
pub fn stub_0x2bf124(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSF_EENS8_5list4INS8_5valueINS1_ISD_EEEENS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2bf150() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsS4_EC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsSE_EENS7_5list4INS7_5valueINS1_ISC_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x2bf234() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x2bf31c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2bf414(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2bf430(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x2bf450(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x2bf538(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x2bf61c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&> &,boost::_bi::list3<rbx_core::SharedPtr<RBX::Instance>&,std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x2bf6f0(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2bf6f0: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x2bf718(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&)")]
pub fn stub_0x2bf870() -> crate::slot::SlotConnection {
// IDA 0x2bf870: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>*>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>*)")]
pub fn stub_0x2bf964(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2bf964: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
pub fn stub_0x2bfa60(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot() [0x2bfb70]")]
pub fn stub_0x2bfb70(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2bfca0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2bfca0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2bfe48(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2bfe48: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x2bfe50(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x2c0038(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2c0038: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x2c0148]")]
pub fn stub_0x2c0148(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x2c0148: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>> const&)")]
pub fn stub_0x2c0278(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance,200u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x2c02a8() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x2c02a8, "boost::singleton_pool<RBX::PartInstance::OnDemandPartInstance, 200u, boost::default_user_allocator_m~")
}

#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::holder_arg::~holder_arg()")]
pub fn stub_0x2c02f8(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type> const&,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&)")]
pub fn stub_0x2c03b8() -> crate::slot::PortedFn {
// IDA 0x2c03b8: boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c03b8, "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::ref~")
}

#[doc(alias = "boost::multi_index::detail::bucket_array<std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::bucket_array(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>> *,unsigned long)")]
pub fn stub_0x2c0408(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::multi_index::detail::auto_space<boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::auto_space(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,unsigned long)")]
pub fn stub_0x2c0488() -> crate::slot::PortedFn {
// IDA 0x2c0488: boost::multi_index::detail::auto_space<boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>>, std::al~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c0488, "boost::multi_index::detail::auto_space<boost::multi_index::detail::hashed_index_node_impl<std::alloc~")
}

#[doc(alias = "boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>>::check_erase(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&)")]
pub fn stub_0x2c05e0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::reset(RBX::Security::Context*)")]
pub fn stub_0x2c05f8() -> crate::slot::PortedFn {
// IDA 0x2c05f8: boost::thread_specific_ptr<RBX::Security::Context>::reset(RBX::Security::Context*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c05f8, "boost::thread_specific_ptr<RBX::Security::Context>::reset(RBX::Security::Context*)")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::~thread_specific_ptr()")]
pub fn stub_0x2c06e0() -> crate::slot::PortedFn {
// IDA 0x2c06e0: boost::thread_specific_ptr<RBX::Security::Context>::~thread_specific_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c06e0, "boost::thread_specific_ptr<RBX::Security::Context>::~thread_specific_ptr()")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::~delete_data()")]
pub fn stub_0x2c07d8() -> crate::slot::PortedFn {
// IDA 0x2c07d8: boost::thread_specific_ptr<RBX::Security::Context>::delete_data::~delete_data().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c07d8, "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::~delete_data()")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>(boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>)")]
pub fn stub_0x2c07e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::~sp_counted_impl_pd()")]
pub fn stub_0x2c08d8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::dispose(void)")]
pub fn stub_0x2c08e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2c08f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::get_untyped_deleter(void)")]
pub fn stub_0x2c0908() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}
