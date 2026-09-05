// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x3dbee8..0x3e11c8 | script 23452->23552 distinct (filler 0x3dbee8 asc, not-in-script 62093->61993)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "std::_Vector_base<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_allocate(unsigned long)")]
pub fn stub_0x3dbee8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::ChangeHistoryService::RuntimeUndoBehavior * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *>(RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *,RBX::ChangeHistoryService::RuntimeUndoBehavior *)")]
pub fn stub_0x3dbf00(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::RuntimeUndoBehavior* std::__copy_backward<false, std::random_ac~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ChangeHistoryService::RuntimeUndoBehavior*,std::vector<RBX::ChangeHistoryService::RuntimeUndoBehavior,std::allocator<RBX::ChangeHistoryService::RuntimeUndoBehavior>>>,unsigned long,RBX::ChangeHistoryService::RuntimeUndoBehavior const&)")]
pub fn stub_0x3dbf40(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_clear(void)")]
pub fn stub_0x3dc308() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayChange(void)")]
pub fn stub_0x3dc330(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::unplayChange() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::operator()(boost::function<void ()(void)>,std::string)const")]
pub fn stub_0x3dc500(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "RBX::ChangeHistoryService::Item::unplayClusterChange(void)")]
pub fn stub_0x3dc698(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::unplayClusterChange() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
pub fn stub_0x3dc6d0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
pub fn stub_0x3dc72c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3dc72c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::vector(std::vector<unsigned int,std::allocator<unsigned int>> const&)")]
pub fn stub_0x3dc7f4() -> crate::slot::PortedFn {
// IDA 0x3dc7f4: std::vector<unsigned int, std::allocator<unsigned int>>::vector(std::vector<unsigned int, std::allocator<unsigned int>> ~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3dc7f4, "std::vector<unsigned int, std::allocator<unsigned int>>::vector(std::vector<unsigned int, std::alloc~")
}

#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_Vector_base(unsigned long,std::allocator<unsigned int> const&)")]
pub fn stub_0x3dc82c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::_Vector_base<unsigned int,std::allocator<unsigned int>>::_M_allocate(unsigned long)")]
pub fn stub_0x3dc85c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3dc874(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x3dc8d4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ChangeHistoryService::Item>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService::Item*>>>::operator()(void)")]
pub fn stub_0x3dc8d8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::dummy::nonnull(void)")]
pub fn stub_0x3dc8f0(slot: &crate::slot::FnSlot) {
// boost::function invocation — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> *)")]
pub fn stub_0x3dc8f4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::ChangeHistoryService::Item::addValue(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3dc928(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::addValue(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::map<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::operator[](RBX::Reflection::PropertyDescriptor const* const&)")]
pub fn stub_0x3dcb74(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
pub fn stub_0x3dccdc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
pub fn stub_0x3dcd90(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_insert_unique(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
pub fn stub_0x3dcddc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_create_node(std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant> const&)")]
pub fn stub_0x3dce44() -> crate::slot::PortedFn {
// IDA 0x3dce44: std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*, std::pair<RBX::Reflection::PropertyDescriptor const* const, RB~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3dce44, "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*, std::pair<RBX::Reflection::PropertyDescrip~")
}

#[doc(alias = "RBX::ChangeHistoryService::Item::recordClusterDataGetChunk(int)")]
pub fn stub_0x3dcf44(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::recordClusterDataGetChunk(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::map<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::operator[](unsigned int const&)")]
pub fn stub_0x3dd084(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
pub fn stub_0x3dd1a4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
pub fn stub_0x3dd258(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_insert_unique(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
pub fn stub_0x3dd2a4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_create_node(std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
pub fn stub_0x3dd30c() -> crate::slot::PortedFn {
// IDA 0x3dd30c: std::_Rb_tree<unsigned int, std::pair<unsigned int const, std::vector<unsigned int, std::allocator<unsigned int>>>, std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3dd30c, "std::_Rb_tree<unsigned int, std::pair<unsigned int const, std::vector<unsigned int, std::allocator<u~")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::find(unsigned int const&)")]
pub fn stub_0x3dd3f0(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::operator==(RBX::Voxel::Region<RBX::Voxel::Grid::Chunk> const&)const")]
pub fn stub_0x3dd430(handle: &crate::slot::InstanceHandle) {
// RBX::Voxel::Region<RBX::Voxel::Grid::Chunk>::operator==(RBX::Voxel::Region<RBX::Voxel::Gri~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
pub fn stub_0x3dd488() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::ChangeHistoryService::Item::addValueIfNotParentProperty(RBX::Reflection::Property const&)")]
pub fn stub_0x3dd54c(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::addValueIfNotParentProperty(RBX::Reflection::Property con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::map<RBX::Instance *,unsigned int,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::operator[](RBX::Instance * const&)")]
pub fn stub_0x3dd564(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance * const,unsigned int>>,std::pair<RBX::Instance * const,unsigned int> const&)")]
pub fn stub_0x3dd5bc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,unsigned int> const&)")]
pub fn stub_0x3dd670(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")]
pub fn stub_0x3dd6c8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
pub fn stub_0x3dd730(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
pub fn stub_0x3dd758() -> crate::slot::PortedFn {
// IDA 0x3dd758: std::_Rb_tree<unsigned int, std::pair<unsigned int const, std::vector<unsigned int, std::allocator<unsigned int>>>, std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3dd758, "std::_Rb_tree<unsigned int, std::pair<unsigned int const, std::vector<unsigned int, std::allocator<u~")
}

#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_create_node(RBX::ChangeHistoryService::Item const&)")]
pub fn stub_0x3dd774() -> crate::slot::PortedFn {
// IDA 0x3dd774: std::list<RBX::ChangeHistoryService::Item, std::allocator<RBX::ChangeHistoryService::Item>>::_M_create_node(RBX::ChangeH~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3dd774, "std::list<RBX::ChangeHistoryService::Item, std::allocator<RBX::ChangeHistoryService::Item>>::_M_crea~")
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_Rb_tree(std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>> const&)")]
pub fn stub_0x3dd900() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> const*,std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>*)")]
pub fn stub_0x3dd944() -> crate::slot::PortedFn {
// IDA 0x3dd944: std::_Rb_tree<unsigned int, std::pair<unsigned int const, std::vector<unsigned int, std::allocator<unsigned int>>>, std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3dd944, "std::_Rb_tree<unsigned int, std::pair<unsigned int const, std::vector<unsigned int, std::allocator<u~")
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_Rb_tree(std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>> const&)")]
pub fn stub_0x3dda98() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>*)")]
pub fn stub_0x3ddadc() -> crate::slot::PortedFn {
// IDA 0x3ddadc: std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*, std::pair<RBX::Reflection::PropertyDescriptor const* const, RB~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3ddadc, "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*, std::pair<RBX::Reflection::PropertyDescrip~")
}

#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem> RBX::Creatable<RBX::Instance>::create<ChangeHistoryStatsItem>(void)")]
pub fn stub_0x3ddc30() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("ChangeHistoryStatsItem")
}

#[doc(alias = "RBX::ChangeHistoryService::getWaypointDataSize(void)const")]
pub fn stub_0x3ddce4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ChangeHistoryService getter.
cell.get()
}

#[doc(alias = "RBX::ChangeHistoryService::getWaypointCount(void)const")]
pub fn stub_0x3ddcec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ChangeHistoryService getter.
cell.get()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3ddd08(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x3ddd68(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>::operator()(void)")]
pub fn stub_0x3ddd6c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::function0<int>::assign_to_own(boost::function0<int> const&)")]
pub fn stub_0x3ddd88(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::TypedStatsItem(boost::function0<int>)")]
pub fn stub_0x3dddb8() -> crate::slot::InstanceHandle {
// RBX::Stats::TypedStatsItem ctor.
crate::slot::InstanceHandle::new("RBX::Stats::TypedStatsItem")
}

#[doc(alias = "RBX::Stats::Item::Item(void)")]
pub fn stub_0x3dded0() -> crate::slot::InstanceHandle {
// RBX::Stats::Item ctor.
crate::slot::InstanceHandle::new("RBX::Stats::Item")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
pub fn stub_0x3de020(handle: crate::slot::InstanceHandle) {
// RBX::Stats::TypedStatsItem dtor.
drop(handle);
}

#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem() [0x3de168]")]
pub fn stub_0x3de168(handle: crate::slot::InstanceHandle) {
// RBX::Stats::TypedStatsItem dtor.
drop(handle);
}

#[doc(alias = "ChangeHistoryStatsItem::ChangeHistoryStatsItem(void)")]
pub fn stub_0x3de2c8() -> crate::slot::PortedFn {
// IDA 0x3de2c8: ChangeHistoryStatsItem::ChangeHistoryStatsItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3de2c8, "ChangeHistoryStatsItem::ChangeHistoryStatsItem()")
}

#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
pub fn stub_0x3de47c() -> crate::slot::PortedFn {
// IDA 0x3de47c: ChangeHistoryStatsItem::~ChangeHistoryStatsItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3de47c, "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem() [0x3de4b8]")]
pub fn stub_0x3de4b8() -> crate::slot::PortedFn {
// IDA 0x3de4b8: ChangeHistoryStatsItem::~ChangeHistoryStatsItem().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3de4b8, "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
pub fn stub_0x3de58c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem() [0x3de5cc]")]
pub fn stub_0x3de5cc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem() [0x3de6a0]")]
pub fn stub_0x3de6a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem() [0x3de6e0]")]
pub fn stub_0x3de6e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem>::shared_ptr<ChangeHistoryStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3de7b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("ChangeHistoryStatsItem")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<ChangeHistoryStatsItem,ChangeHistoryStatsItem>(rbx_core::SharedPtr<ChangeHistoryStatsItem> const*,ChangeHistoryStatsItem *)const")]
pub fn stub_0x3de87c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("ChangeHistoryStatsItem")
}

#[doc(alias = "boost::detail::shared_count::shared_count<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3de964() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x3dea6c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x3dea70]")]
pub fn stub_0x3dea70(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x3dea74() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x3dea94() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x3deaac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ChangeHistoryService::Item::computeDataSize(void)const")]
pub fn stub_0x3deab0(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::computeDataSize() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::removeItem(RBX::Instance *)")]
pub fn stub_0x3deb04(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Waypoint::removeItem(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ChangeHistoryService::Item::absorb(RBX::ChangeHistoryService::Item const&)")]
pub fn stub_0x3deba8(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::absorb(RBX::ChangeHistoryService::Item const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ChangeHistoryService::Waypoint::addItem(RBX::ChangeHistoryService::Item const&)")]
pub fn stub_0x3ded00(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Waypoint::addItem(RBX::ChangeHistoryService::Item const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
pub fn stub_0x3ded38() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::ChangeHistoryService::Item::absorbProp(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
pub fn stub_0x3ded94(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::absorbProp(std::pair<RBX::Reflection::PropertyDescriptor ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
pub fn stub_0x3dedb4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "RBX::ChangeHistoryService::Item::absorbClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
pub fn stub_0x3dee10(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::absorbClusterData(std::pair<unsigned int, std::vector<uns~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::push_back(unsigned int const&)")]
pub fn stub_0x3defec(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,unsigned int const&)")]
pub fn stub_0x3df014(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
pub fn stub_0x3df0f0(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3df0f0: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> &,int)")]
pub fn stub_0x3df1b8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3df1b8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_erase(std::_List_iterator<RBX::ChangeHistoryService::Item>)")]
pub fn stub_0x3df2d8(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::_List_base<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_clear(void)")]
pub fn stub_0x3df3fc(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,unsigned int>> *)")]
pub fn stub_0x3df534(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::ChangeHistoryService::Item::onSetWaypoint(void)")]
pub fn stub_0x3df55c(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::onSetWaypoint() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void std::vector<unsigned int,std::allocator<unsigned int>>::_M_assign_aux<__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>>(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,std::forward_iterator_tag)")]
pub fn stub_0x3df6fc(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::clear(void)")]
pub fn stub_0x3df798(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "RBX::ChangeHistoryService::Item::playClusterChange(void)")]
pub fn stub_0x3df7c4(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::playClusterChange() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ChangeHistoryService::Item::applyClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
pub fn stub_0x3df7fc(handle: &crate::slot::InstanceHandle) {
// RBX::ChangeHistoryService::Item::applyClusterData(std::pair<unsigned int, std::vector<unsi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>> *)")]
pub fn stub_0x3df920(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::CharacterAppearance::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x3e113c(handle: &crate::slot::InstanceHandle) {
// RBX::CharacterAppearance::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ShirtGraphic::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3e1178(handle: &crate::slot::InstanceHandle) {
// RBX::ShirtGraphic::dataChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x3e117c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Clothing::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x3e11a0(handle: &crate::slot::InstanceHandle) {
// RBX::Clothing::dataChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_0x3e11a4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_0x3e11c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}
