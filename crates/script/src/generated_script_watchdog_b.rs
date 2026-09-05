// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x70e9f8..0x74e294 | script 26973->27073 distinct (filler 0x70e9f8 asc, not-in-crates 815->715)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::FWBase>::get_deleter(std::type_info const&)")]
pub fn stub_0x70e9f8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,bool),boost::_bi::list2<boost::arg<1>,boost::_bi::value<bool>>>,RBX::Primitive *)")]
pub fn stub_0x717cac() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Assembly,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::Assembly*>,boost::arg<1>>>,RBX::Primitive *)")]
pub fn stub_0x717d64() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x71b054() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x71b054, "boost::singleton_pool<RBX::BallPolyContact, 212u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x71d1cc(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockCorners,96u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x71d1fc(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x71e2b0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x71e2e0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x723c68(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x723c98() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x723c98, "boost::singleton_pool<RBX::GeoPairConnector, 264u, boost::default_user_allocator_malloc_free, boost:~")
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x723cd0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x723d08() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x723d08, "boost::singleton_pool<RBX::BallBallConnector, 272u, boost::default_user_allocator_malloc_free, boost~")
}

#[doc(alias = "boost::singleton_pool<RBX::GeoPairConnector,264u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x724034() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x724034, "boost::singleton_pool<RBX::GeoPairConnector, 264u, boost::default_user_allocator_malloc_free, boost:~")
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockConnector,288u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x7240e4(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallConnector,272u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x724194() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x724194, "boost::singleton_pool<RBX::BallBallConnector, 272u, boost::default_user_allocator_malloc_free, boost~")
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::begin(void)const")]
pub fn stub_0x727a94(handle: &crate::slot::InstanceHandle) {
// RBX::DenseHashSet<RBX::Primitive*, boost::hash<RBX::Primitive*>, std::allocator<RBX::Primi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>::const_iterator::operator++(void)")]
pub fn stub_0x727acc(handle: &crate::slot::InstanceHandle) {
// RBX::DenseHashSet<RBX::Primitive*, boost::hash<RBX::Primitive*>, std::allocator<RBX::Primi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesTouchingGrids(RBX::Extents const&,boost::unordered::unordered_set<RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>,std::allocator<RBX::Primitive const*>> const&,unsigned long,boost::unordered::unordered_set<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::equal_to<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
pub fn stub_0x727b44(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpatialHash getter.
cell.get()
}

#[doc(alias = "bool RBX::ContactManager::anyExtentsOverlapsOrTouchesPrimitives<RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>>>(RBX::Extents const&,RBX::DenseHashSet<RBX::Primitive *,boost::hash<RBX::Primitive *>,std::allocator<RBX::Primitive *>> const&)const")]
pub fn stub_0x7283d4() -> crate::slot::PortedFn {
// IDA 0x7283d4: bool RBX::ContactManager::anyExtentsOverlapsOrTouchesPrimitives<RBX::DenseHashSet<RBX::Primitive*, boost::hash<RBX::Prim~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x7283d4, "bool RBX::ContactManager::anyExtentsOverlapsOrTouchesPrimitives<RBX::DenseHashSet<RBX::Primitive*, b~")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>> std::for_each<boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>>(boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::unordered::iterator_detail::c_iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>,boost::unordered::detail::ptr_node<RBX::Primitive *> const*>,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list5<boost::_bi::value<RBX::ContactManager*>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>>)")]
pub fn stub_0x729bd8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::clear(void)")]
pub fn stub_0x729c3c(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x729c6c(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::ContactManager *>,boost::arg<1>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool>,boost::_bi::list1<RBX::Primitive * const&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::ContactManager,RBX::Primitive *,bool,bool,bool> &,boost::_bi::list1<RBX::Primitive * const&> &,int)")]
pub fn stub_0x729d8c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x729d8c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id>>(RBX::SpatialRegion::Id const&,boost::unordered::detail::emplace_args1<RBX::SpatialRegion::Id> const&)")]
pub fn stub_0x729dd8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x729f7c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::create_buckets(unsigned long)")]
pub fn stub_0x729fd0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x72a0f8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::rehash_impl(unsigned long)")]
pub fn stub_0x72a188(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x72a1b4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>>::construct(void)")]
pub fn stub_0x72a20c() -> crate::slot::PortedFn {
// IDA 0x72a20c: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>>>::~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72a20c, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Sp~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::find_node_impl<RBX::SpatialRegion::Id,std::equal_to<RBX::SpatialRegion::Id>>(unsigned long,RBX::SpatialRegion::Id const&,std::equal_to<RBX::SpatialRegion::Id> const&)const")]
pub fn stub_0x72a244(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72a2c0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72a2c0, "boost::singleton_pool<RBX::PolyCellContact, 232u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::PolyCellContact,232u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72a30c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72a30c, "boost::singleton_pool<RBX::PolyCellContact, 232u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72a344() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72a344, "boost::singleton_pool<RBX::BallCellContact, 228u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::BallCellContact,228u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72a390() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72a390, "boost::singleton_pool<RBX::BallCellContact, 228u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72b770() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72b770, "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::Spatia~")
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72b8e0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72b8e0, "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::TreeNo~")
}

#[doc(alias = "boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::construct(void)")]
pub fn stub_0x72c074() -> crate::slot::PortedFn {
// IDA 0x72c074: boost::object_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::TreeNode, RBX::roblox_allocat~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72c074, "boost::object_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::TreeNode,~")
}

#[doc(alias = "boost::pool<RBX::roblox_allocator>::ordered_malloc_need_resize(void)")]
pub fn stub_0x72c17c() -> crate::slot::PortedFn {
// IDA 0x72c17c: boost::pool<RBX::roblox_allocator>::ordered_malloc_need_resize().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72c17c, "boost::pool<RBX::roblox_allocator>::ordered_malloc_need_resize()")
}

#[doc(alias = "boost::simple_segregated_storage<unsigned long>::add_ordered_block(void *,unsigned long,unsigned long)")]
pub fn stub_0x72c2b0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x72c468() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c468, "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::TreeNo~")
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72c49c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c49c, "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::TreeNo~")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode* boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::construct<int,int,RBX::Vector3int32>(int &,int &,RBX::Vector3int32 const&)")]
pub fn stub_0x72c544(handle: &crate::slot::InstanceHandle) {
// RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::SpatialNode* boost~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x72c70c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c70c, "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::Spatia~")
}

#[doc(alias = "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,32u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72c73c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c73c, "boost::singleton_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::Spatia~")
}

#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72c7c0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c7c0, "boost::singleton_pool<RBX::PolyPolyContact, 216u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72c80c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c80c, "boost::singleton_pool<RBX::PolyPolyContact, 216u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72c844(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::BlockBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72c890(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72c8c8() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c8c8, "boost::singleton_pool<RBX::BallPolyContact, 212u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::BallPolyContact,212u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72c914() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c914, "boost::singleton_pool<RBX::BallPolyContact, 212u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72c94c(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x72c988() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72c988, "boost::singleton_pool<RBX::BallBallContact, 52u, boost::default_user_allocator_malloc_free, boost::m~")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Primitive *>>(RBX::Primitive * const&,boost::unordered::detail::emplace_args1<RBX::Primitive *> const&)")]
pub fn stub_0x72cbe0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x72cd70(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::create_buckets(unsigned long)")]
pub fn stub_0x72cdc0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x72cee8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::rehash_impl(unsigned long)")]
pub fn stub_0x72cf78(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x72cfa4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive *>>>::construct(void)")]
pub fn stub_0x72cff8() -> crate::slot::PortedFn {
// IDA 0x72cff8: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive*>>>::constru~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72cff8, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Pr~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::find_node_impl<RBX::Primitive *,std::equal_to<RBX::Primitive *>>(unsigned long,RBX::Primitive * const&,std::equal_to<RBX::Primitive *> const&)const")]
pub fn stub_0x72d030(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::clear(void)")]
pub fn stub_0x72d0a0(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x72d0d0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::pool<RBX::roblox_allocator>::purge_memory(void)")]
pub fn stub_0x72d69c() -> crate::slot::PortedFn {
// IDA 0x72d69c: boost::pool<RBX::roblox_allocator>::purge_memory().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72d69c, "boost::pool<RBX::roblox_allocator>::purge_memory()")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::delete_buckets(void)")]
pub fn stub_0x72d750(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::delete_buckets(void)")]
pub fn stub_0x72d780(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>,RBX::SpatialRegion::Id,RBX::SpatialRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::SpatialRegion::Id>>>::table(unsigned long,RBX::SpatialRegion::Id::boost_compatible_hash_value const&,std::equal_to<RBX::SpatialRegion::Id> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::SpatialRegion::Id>> const&)")]
pub fn stub_0x72d7b0() -> crate::slot::PortedFn {
// IDA 0x72d7b0: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>, RBX::SpatialRegion~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72d7b0, "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::SpatialRegion::Id>~")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive *>,RBX::Primitive *,boost::hash<RBX::Primitive *>,std::equal_to<RBX::Primitive *>>>::table(unsigned long,boost::hash<RBX::Primitive *> const&,std::equal_to<RBX::Primitive *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive *>> const&)")]
pub fn stub_0x72d81c() -> crate::slot::PortedFn {
// IDA 0x72d81c: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive*>, RBX::Primitive*, boost::h~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72d81c, "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive*>, RBX::~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::MegaClusterMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x72d888() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72d888, "boost::singleton_pool<RBX::POLY::MegaClusterMesh, 48u, boost::default_user_allocator_malloc_free, bo~")
}

#[doc(alias = "boost::singleton_pool<RBX::BallBlockContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x72dca4(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x72e008() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x72e008, "boost::singleton_pool<RBX::BallBallContact, 52u, boost::default_user_allocator_malloc_free, boost::m~")
}

#[doc(alias = "boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::TreeNode,RBX::roblox_allocator>::~object_pool()")]
pub fn stub_0x72e1f8() -> crate::slot::PortedFn {
// IDA 0x72e1f8: boost::object_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::TreeNode, RBX::roblox_allocat~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72e1f8, "boost::object_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::TreeNode,~")
}

#[doc(alias = "boost::object_pool<RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::SpatialNode,RBX::roblox_allocator>::~object_pool()")]
pub fn stub_0x72e338() -> crate::slot::PortedFn {
// IDA 0x72e338: boost::object_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::SpatialNode, RBX::roblox_allo~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72e338, "boost::object_pool<RBX::SpatialHash<RBX::Primitive, RBX::Contact, RBX::ContactManager, 4>::SpatialNo~")
}

#[doc(alias = "RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::getPrimitivesTouchingGrids(RBX::Extents const&,RBX::Primitive const*,unsigned long,boost::unordered::unordered_set<RBX::Primitive*,boost::hash<RBX::Primitive*>,std::equal_to<RBX::Primitive*>,std::allocator<RBX::Primitive*>> &)")]
pub fn stub_0x72f230(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::SpatialHash getter.
cell.get()
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>,RBX::Primitive*>,boost::_bi::list2<boost::_bi::value<RBX::ContactManagerSpatialHash *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>,RBX::Primitive*>,boost::_bi::list2<boost::_bi::value<RBX::ContactManagerSpatialHash *>,boost::arg<1>>>,RBX::Primitive*)")]
pub fn stub_0x72f604() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::simple_segregated_storage<unsigned long>::ordered_free(void *)")]
pub fn stub_0x72f888() -> crate::slot::PortedFn {
// IDA 0x72f888: boost::simple_segregated_storage<unsigned long>::ordered_free(void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x72f888, "boost::simple_segregated_storage<unsigned long>::ordered_free(void*)")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x7324f0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x7324f0, "boost::singleton_pool<RBX::POLY::CornerWedgeMesh, 48u, boost::default_user_allocator_malloc_free, bo~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x732520() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x732520, "boost::singleton_pool<RBX::POLY::CornerWedgeMesh, 48u, boost::default_user_allocator_malloc_free, bo~")
}

#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EdgeBuffer,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::EdgeBuffer*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EdgeBuffer,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::EdgeBuffer*>,boost::arg<1>>>,RBX::Primitive *)")]
pub fn stub_0x733194() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x734678() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x734678, "boost::singleton_pool<RBX::NormalBreakConnector, 48u, boost::default_user_allocator_malloc_free, boo~")
}

#[doc(alias = "boost::singleton_pool<RBX::NormalBreakConnector,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x734730() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x734730, "boost::singleton_pool<RBX::NormalBreakConnector, 48u, boost::default_user_allocator_malloc_free, boo~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::WedgeMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x739914() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x739914, "boost::singleton_pool<RBX::POLY::WedgeMesh, 36u, boost::default_user_allocator_malloc_free, boost::m~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::CornerWedgeMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x739960() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x739960, "boost::singleton_pool<RBX::POLY::CornerWedgeMesh, 48u, boost::default_user_allocator_malloc_free, bo~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::RightAngleRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x7399ac() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x7399ac, "boost::singleton_pool<RBX::POLY::RightAngleRampMesh, 48u, boost::default_user_allocator_malloc_free,~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x7399f8() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x7399f8, "boost::singleton_pool<RBX::POLY::ParallelRampMesh, 48u, boost::default_user_allocator_malloc_free, b~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PyramidMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x739a44() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x739a44, "boost::singleton_pool<RBX::POLY::PyramidMesh, 56u, boost::default_user_allocator_malloc_free, boost:~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::PrismMesh,56u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x739a90() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x739a90, "boost::singleton_pool<RBX::POLY::PrismMesh, 56u, boost::default_user_allocator_malloc_free, boost::m~")
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::JointStage *>,boost::arg<1>,boost::arg<2>,boost::reference_wrapper<std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>>>::operator()<boost::_mfi::mf3<void,RBX::JointStage,RBX::Primitive *,RBX::Joint *,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>&>,boost::_bi::list2<RBX::Primitive * const&,RBX::Joint * const&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::JointStage,RBX::Primitive *,RBX::Joint *,std::vector<RBX::Joint *,std::allocator<RBX::Joint *>>&> const&,boost::_bi::list2<RBX::Primitive * const&,RBX::Joint * const&> &,int)const")]
pub fn stub_0x73b270(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x73b270: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void RBX::IndexedTree::visitMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AssemblyStage,RBX::Assembly*>,boost::_bi::list2<boost::_bi::value<RBX::AssemblyStage*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AssemblyStage,RBX::Assembly*>,boost::_bi::list2<boost::_bi::value<RBX::AssemblyStage*>,boost::arg<1>>>)")]
pub fn stub_0x73cbc8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x742328() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x742328, "boost::singleton_pool<RBX::D6Link, 252u, boost::default_user_allocator_malloc_free, boost::mutex, 32~")
}

#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x742374() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x742374, "boost::singleton_pool<RBX::D6Link, 252u, boost::default_user_allocator_malloc_free, boost::mutex, 32~")
}

#[doc(alias = "boost::singleton_pool<RBX::D6Link,252u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x74242c() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x74242c, "boost::singleton_pool<RBX::D6Link, 252u, boost::default_user_allocator_malloc_free, boost::mutex, 32~")
}

#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x742c78() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x742c78, "boost::singleton_pool<RBX::RevoluteLink, 208u, boost::default_user_allocator_malloc_free, boost::mut~")
}

#[doc(alias = "boost::singleton_pool<RBX::RevoluteLink,208u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x742dfc() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x742dfc, "boost::singleton_pool<RBX::RevoluteLink, 208u, boost::default_user_allocator_malloc_free, boost::mut~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x7455f8() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x7455f8, "boost::singleton_pool<RBX::POLY::ParallelRampMesh, 48u, boost::default_user_allocator_malloc_free, b~")
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::ParallelRampMesh,48u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x745628() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x745628, "boost::singleton_pool<RBX::POLY::ParallelRampMesh, 48u, boost::default_user_allocator_malloc_free, b~")
}

#[doc(alias = "boost::singleton_pool<RBX::PolyPolyContact,216u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x74a0e0() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x74a0e0, "boost::singleton_pool<RBX::PolyPolyContact, 216u, boost::default_user_allocator_malloc_free, boost::~")
}

#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
pub fn stub_0x74cf14() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x74cf14, "boost::singleton_pool<RBX::Body, 276u, boost::default_user_allocator_malloc_free, boost::mutex, 32u,~")
}

#[doc(alias = "boost::singleton_pool<RBX::Body,276u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::malloc(void)")]
pub fn stub_0x74cf64() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x74cf64, "boost::singleton_pool<RBX::Body, 276u, boost::default_user_allocator_malloc_free, boost::mutex, 32u,~")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token>::operator=(rbx_core::SharedPtr<RBX::GeometryPool<RBX::Vector3_2Ints,RBX::POLY::PrismMesh,RBX::Vector3_2IntsComparer>::Token> const&)")]
pub fn stub_0x74e294(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}
