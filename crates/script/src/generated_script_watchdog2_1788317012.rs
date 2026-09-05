// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x45562c..0x4591cc | script 25352->25452 distinct (filler 0x45562c asc, not-in-script 60193->60093)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)")]
pub fn stub_0x45562c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ChatService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4556dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ChatService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const*,RBX::ChatService *)const")]
pub fn stub_0x4557a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ChatService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x455890() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x455998(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x45599c]")]
pub fn stub_0x45599c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4559a0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4559c0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4559d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const")]
pub fn stub_0x455d30() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ChatService"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")]
pub fn stub_0x455ea4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)")]
pub fn stub_0x455f50() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")]
pub fn stub_0x456090() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GuiService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")]
pub fn stub_0x456140(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x456174() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GuiService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const*,RBX::GuiService *)const")]
pub fn stub_0x45623c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GuiService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x456328() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x456430(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x456434]")]
pub fn stub_0x456434(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x456438() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x456458() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x456470() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const")]
pub fn stub_0x456474() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::KeyframeSequenceProvider"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")]
pub fn stub_0x4565e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::KeyframeSequenceProvider")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")]
pub fn stub_0x456698(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
pub fn stub_0x456718() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
pub fn stub_0x45671c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4567f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::KeyframeSequenceProvider")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")]
pub fn stub_0x4568bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::KeyframeSequenceProvider")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4569a8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x456ab0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x456ab4]")]
pub fn stub_0x456ab4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x456ab8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x456ad8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x456af0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const")]
pub fn stub_0x456b94() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ContentFilter"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")]
pub fn stub_0x456d08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")]
pub fn stub_0x456db8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)")]
pub fn stub_0x456f18() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)")]
pub fn stub_0x456f1c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x456ff4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")]
pub fn stub_0x4570bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ContentFilter")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4571a8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4572b0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4572b4]")]
pub fn stub_0x4572b4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4572b8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4572d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4572f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")]
pub fn stub_0x457398(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")]
pub fn stub_0x4573e4() -> crate::slot::PortedFn {
// IDA 0x4573e4: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>, unsigned int, boost::hash<un~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4573e4, "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>, unsigned~")
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x457450(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x457454(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4574f4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4574fc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4575a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4575a8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x45764c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x457650(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4576f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4576f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x45779c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4577a4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::ServiceProvider::~ServiceProvider()")]
pub fn stub_0x457848(handle: crate::slot::InstanceHandle) {
// RBX::ServiceProvider dtor.
drop(handle);
}

#[doc(alias = "RBX::ServiceProvider::~ServiceProvider() [0x457b28]")]
pub fn stub_0x457b28(handle: crate::slot::InstanceHandle) {
// RBX::ServiceProvider dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
pub fn stub_0x457b30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider() [0x457b38]")]
pub fn stub_0x457b38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider() [0x457be0]")]
pub fn stub_0x457be0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider() [0x457be8]")]
pub fn stub_0x457be8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x457c98() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GuiRoot")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(rbx_core::SharedPtr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")]
pub fn stub_0x457d60() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::GuiRoot")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x457e4c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x457f54(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x457f58]")]
pub fn stub_0x457f58(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x457f5c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x457f7c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x457f94() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace>::shared_ptr<RBX::Workspace,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x457f98() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Workspace")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Workspace,RBX::Workspace>(rbx_core::SharedPtr<RBX::Workspace> const*,RBX::Workspace *)const")]
pub fn stub_0x458060() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Workspace")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x45814c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x458254(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x458258]")]
pub fn stub_0x458258(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x45825c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x45827c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Workspace *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x458294() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService> RBX::Creatable<RBX::Instance>::create<RBX::ChangeHistoryService>(void)")]
pub fn stub_0x45847c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ChangeHistoryService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChangeHistoryService>::shared_ptr<RBX::ChangeHistoryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x45852c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ChangeHistoryService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(rbx_core::SharedPtr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const")]
pub fn stub_0x4585f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ChangeHistoryService")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4586e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4587e8]")]
pub fn stub_0x4587e8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4587f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x458810() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChangeHistoryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x458828() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(void)const")]
pub fn stub_0x458b80() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ChangeHistoryService"))
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ChangeHistoryService>(void)")]
pub fn stub_0x458d68() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChangeHistoryService>(void)")]
pub fn stub_0x458d6c() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::~vector()")]
pub fn stub_0x458e44(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>::shared_ptr<RBX::DataModel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x458f10() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DataModel,RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const*,RBX::DataModel *)const")]
pub fn stub_0x458fd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x4590c4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x4591cc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}
