// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4591d0..0x45cb18 | script 25452->25552 distinct (filler 0x4591d0 asc, not-in-script 60093->59993)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4591d0]")]
pub fn stub_0x4591d0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x4591d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4591f4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DataModel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x45920c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel::GenericJob>::shared_ptr<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
pub fn stub_0x459210() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::GenericJob")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::DataModel::GenericJob,RBX::DataModel::GenericJob>(rbx_core::SharedPtr<RBX::DataModel::GenericJob> const*,RBX::DataModel::GenericJob *)const")]
pub fn stub_0x4592f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel::GenericJob")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DataModel::GenericJob>(RBX::DataModel::GenericJob *)")]
pub fn stub_0x4593dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p()")]
pub fn stub_0x4594d4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::~sp_counted_impl_p() [0x4594d8]")]
pub fn stub_0x4594d8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::dispose(void)")]
pub fn stub_0x4594dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x4594ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::DataModel::GenericJob>::get_untyped_deleter(void)")]
pub fn stub_0x4594f0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Visit * RBX::ServiceProvider::find<RBX::Visit>(void)const")]
pub fn stub_0x4594f8() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::Visit"))
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Visit>(void)")]
pub fn stub_0x4598e0() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Visit>(void)")]
pub fn stub_0x4598e4() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Visit> RBX::Creatable<RBX::Instance>::create<RBX::Visit>(void)")]
pub fn stub_0x459b9c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Visit")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Visit>::shared_ptr<RBX::Visit,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x459c4c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Visit")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(rbx_core::SharedPtr<RBX::Visit> const*,RBX::Visit *)const")]
pub fn stub_0x459d14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Visit")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x459e00() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x459f08(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x459f0c]")]
pub fn stub_0x459f0c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x459f10() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x459f30() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Visit *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x459f48() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter> const>::initSingleton(void)")]
pub fn stub_0x45a174(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter> const>::in~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter> const>::doGetSingleton(void)")]
pub fn stub_0x45a178(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Instance::SaveFilter> const>::do~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearType> const>::initSingleton(void)")]
pub fn stub_0x45a268(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearType> const>::ini~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearType> const>::doGetSingleton(void)")]
pub fn stub_0x45a26c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearType> const>::doG~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting> const>::initSingleton(void)")]
pub fn stub_0x45a35c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting> const>::doGetSingleton(void)")]
pub fn stub_0x45a360(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::GearGenreSetting> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::Genre> const>::initSingleton(void)")]
pub fn stub_0x45a450(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::Genre> const>::initSi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::Genre> const>::doGetSingleton(void)")]
pub fn stub_0x45a454(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::Genre> const>::doGetS~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType> const>::initSingleton(void)")]
pub fn stub_0x45a544(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType> const>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType> const>::doGetSingleton(void)")]
pub fn stub_0x45a548(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DataModel::CreatorType> const>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Instance::SaveFilter * rbx::any_cast<RBX::Instance::SaveFilter,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45a638(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::string rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45a690(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::Instance::SaveFilter & rbx::any_cast<RBX::Instance::SaveFilter &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45a78c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::resize(unsigned long,RBX::Instance::SaveFilter)")]
pub fn stub_0x45a880(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::push_back(RBX::Instance::SaveFilter const&)")]
pub fn stub_0x45a8b8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Instance::SaveFilter,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x45a8e4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter> const&)")]
pub fn stub_0x45a93c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter> const&)")]
pub fn stub_0x45a9f0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Instance::SaveFilter>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Instance::SaveFilter> const&)")]
pub fn stub_0x45aa48(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Instance::SaveFilter*,std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>>,RBX::Instance::SaveFilter const&)")]
pub fn stub_0x45aab4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_allocate(unsigned long)")]
pub fn stub_0x45ab98() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Instance::SaveFilter * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *>(RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *,RBX::Instance::SaveFilter *)")]
pub fn stub_0x45abb0(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::SaveFilter* std::__copy_backward<false, std::random_access_iterator_tag>::_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Instance::SaveFilter*,std::vector<RBX::Instance::SaveFilter,std::allocator<RBX::Instance::SaveFilter>>>,unsigned long,RBX::Instance::SaveFilter const&)")]
pub fn stub_0x45abf0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::DataModel::GearType * rbx::any_cast<RBX::DataModel::GearType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45ad84(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::DataModel::GearType & rbx::any_cast<RBX::DataModel::GearType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45addc(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::resize(unsigned long,RBX::DataModel::GearType)")]
pub fn stub_0x45aecc(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::push_back(RBX::DataModel::GearType const&)")]
pub fn stub_0x45af00(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::GearType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x45af28(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
pub fn stub_0x45af80(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
pub fn stub_0x45b034(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::GearType> const&)")]
pub fn stub_0x45b08c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,RBX::DataModel::GearType const&)")]
pub fn stub_0x45b0f4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_allocate(unsigned long)")]
pub fn stub_0x45b1d8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DataModel::GearType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::GearType *,RBX::DataModel::GearType *>(RBX::DataModel::GearType *,RBX::DataModel::GearType *,RBX::DataModel::GearType *)")]
pub fn stub_0x45b1f0(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::GearType* std::__copy_backward<false, std::random_access_iterator_tag>::__~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::GearType*,std::vector<RBX::DataModel::GearType,std::allocator<RBX::DataModel::GearType>>>,unsigned long,RBX::DataModel::GearType const&)")]
pub fn stub_0x45b22c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::DataModel::GearGenreSetting * rbx::any_cast<RBX::DataModel::GearGenreSetting,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45b3bc(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::DataModel::GearGenreSetting & rbx::any_cast<RBX::DataModel::GearGenreSetting &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45b414(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::resize(unsigned long,RBX::DataModel::GearGenreSetting)")]
pub fn stub_0x45b504(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::push_back(RBX::DataModel::GearGenreSetting const&)")]
pub fn stub_0x45b538(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::GearGenreSetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x45b560(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
pub fn stub_0x45b5b8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
pub fn stub_0x45b66c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::GearGenreSetting> const&)")]
pub fn stub_0x45b6c4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::GearGenreSetting*,std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>>,RBX::DataModel::GearGenreSetting const&)")]
pub fn stub_0x45b72c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_allocate(unsigned long)")]
pub fn stub_0x45b810() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DataModel::GearGenreSetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *>(RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *,RBX::DataModel::GearGenreSetting *)")]
pub fn stub_0x45b828(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::GearGenreSetting* std::__copy_backward<false, std::random_access_iterator_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::GearGenreSetting*,std::vector<RBX::DataModel::GearGenreSetting,std::allocator<RBX::DataModel::GearGenreSetting>>>,unsigned long,RBX::DataModel::GearGenreSetting const&)")]
pub fn stub_0x45b864(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::DataModel::Genre * rbx::any_cast<RBX::DataModel::Genre,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45b9f4(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::DataModel::Genre & rbx::any_cast<RBX::DataModel::Genre &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45ba4c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::resize(unsigned long,RBX::DataModel::Genre)")]
pub fn stub_0x45bb3c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::push_back(RBX::DataModel::Genre const&)")]
pub fn stub_0x45bb70(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::Genre,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x45bb98(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
pub fn stub_0x45bbf0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
pub fn stub_0x45bca4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::Genre>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::Genre>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::Genre> const&)")]
pub fn stub_0x45bcfc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,RBX::DataModel::Genre const&)")]
pub fn stub_0x45bd64(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_allocate(unsigned long)")]
pub fn stub_0x45be48() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DataModel::Genre * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::Genre *,RBX::DataModel::Genre *>(RBX::DataModel::Genre *,RBX::DataModel::Genre *,RBX::DataModel::Genre *)")]
pub fn stub_0x45be60(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::Genre* std::__copy_backward<false, std::random_access_iterator_tag>::__cop~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::Genre*,std::vector<RBX::DataModel::Genre,std::allocator<RBX::DataModel::Genre>>>,unsigned long,RBX::DataModel::Genre const&)")]
pub fn stub_0x45be9c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::DataModel::CreatorType * rbx::any_cast<RBX::DataModel::CreatorType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x45c02c(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::DataModel::CreatorType & rbx::any_cast<RBX::DataModel::CreatorType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x45c084(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::resize(unsigned long,RBX::DataModel::CreatorType)")]
pub fn stub_0x45c174(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::push_back(RBX::DataModel::CreatorType const&)")]
pub fn stub_0x45c1a8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DataModel::CreatorType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x45c1d0(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
pub fn stub_0x45c228(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
pub fn stub_0x45c2dc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DataModel::CreatorType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DataModel::CreatorType> const&)")]
pub fn stub_0x45c334(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,RBX::DataModel::CreatorType const&)")]
pub fn stub_0x45c39c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_allocate(unsigned long)")]
pub fn stub_0x45c480() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DataModel::CreatorType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *>(RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *,RBX::DataModel::CreatorType *)")]
pub fn stub_0x45c498(handle: &crate::slot::InstanceHandle) {
// RBX::DataModel::CreatorType* std::__copy_backward<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DataModel::CreatorType*,std::vector<RBX::DataModel::CreatorType,std::allocator<RBX::DataModel::CreatorType>>>,unsigned long,RBX::DataModel::CreatorType const&)")]
pub fn stub_0x45c4d4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::BoundFuncDesc(void (RBX::DataModel::*)(std::string,std::string),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x45c664() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x45c82c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::~BoundFuncDesc() [0x45c878]")]
pub fn stub_0x45c878(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DataModel,void ()(std::string,std::string),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x45c94c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::DataModel", "void", 2)
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DataModel,void (RBX::DataModel::*)(std::string,std::string),std::string,std::string,void>::call(RBX::DataModel*,void (RBX::DataModel::*)(std::string,std::string),RBX::Reflection::Variant &,std::string const&,std::string const&)")]
pub fn stub_0x45cb18(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call2Helper<RBX::DataModel, void (RBX::DataModel::*)(std::string, std::st~ — engine-side; linkage preserved via the alias.
let _ = handle;
}
