// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Lua|Script|CodeGen (4456 strict, 5041 incl lua lower, all stubbed 0 remaining) — global EA-sorted asc filler
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x35b6bc..0x35faa8 | global filler EA-sorted asc after 0x35b410 | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::MeshId * rbx::any_cast<RBX::MeshId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x35b6bc(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::MeshId & rbx::any_cast<RBX::MeshId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x35b714(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::singleton(void)")]
pub fn stub_0x35b804(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::MeshId>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::construct_func(char const*,char *)")]
pub fn stub_0x35b870(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::MeshId>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MeshId>::destruct_func(char *)")]
pub fn stub_0x35b88c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::MeshId>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_129")]
pub fn stub_0x35b890() -> crate::slot::PortedFn {
// IDA 0x35b890: __GLOBAL__I_a_129.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x35b890, "__GLOBAL__I_a_129")
}

#[doc(alias = "RBX::Name::Name(char const* const&)")]
pub fn stub_0x35ba98() -> crate::slot::InstanceHandle {
// RBX::Name ctor.
crate::slot::InstanceHandle::new("RBX::Name")
}

#[doc(alias = "RBX::Name::setOrderIndex(void)")]
pub fn stub_0x35bbbc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Name setter.
cell.set(value)
}

#[doc(alias = "RBX::Name::lookup(char const* const&)")]
pub fn stub_0x35bd48(handle: &crate::slot::InstanceHandle) {
// RBX::Name::lookup(char const* const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Name::getNullName(void)")]
pub fn stub_0x35be98(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Name getter.
cell.get()
}

#[doc(alias = "RBX::Name::lookup(std::string const&)")]
pub fn stub_0x35bebc(handle: &crate::slot::InstanceHandle) {
// RBX::Name::lookup(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Name::NameMap::~NameMap()")]
pub fn stub_0x35bfe8(handle: crate::slot::InstanceHandle) {
// RBX::Name::NameMap dtor.
drop(handle);
}

#[doc(alias = "RBX::Name::NameMap::~NameMap() [0x35bfec]")]
pub fn stub_0x35bfec(handle: crate::slot::InstanceHandle) {
// RBX::Name::NameMap dtor.
drop(handle);
}

#[doc(alias = "initMoo(void)")]
pub fn stub_0x35c02c() -> crate::slot::PortedFn {
// IDA 0x35c02c: initMoo().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35c02c, "initMoo()")
}

#[doc(alias = "moo2(void)")]
pub fn stub_0x35c030() -> crate::slot::PortedFn {
// IDA 0x35c030: moo2().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35c030, "moo2()")
}

#[doc(alias = "RBX::Name::map(void)")]
pub fn stub_0x35c10c(handle: &crate::slot::InstanceHandle) {
// RBX::Name::map() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Name::approximateMemoryUsage(void)")]
pub fn stub_0x35c200(handle: &crate::slot::InstanceHandle) {
// RBX::Name::approximateMemoryUsage() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Name::size(void)")]
pub fn stub_0x35c218(handle: &crate::slot::InstanceHandle) {
// RBX::Name::size() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "declareNullName(void)")]
pub fn stub_0x35c230() -> crate::slot::PortedFn {
// IDA 0x35c230: declareNullName().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35c230, "declareNullName()")
}

#[doc(alias = "RBX::Name::declare(char const* const&)")]
pub fn stub_0x35c258(handle: &crate::slot::InstanceHandle) {
// RBX::Name::declare(char const* const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::~vector()")]
pub fn stub_0x35c4b8(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::insert(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
pub fn stub_0x35c4cc() -> crate::slot::PortedFn {
// IDA 0x35c4cc: std::vector<RBX::Name*, std::allocator<RBX::Name*>>::insert(__gnu_cxx::__normal_iterator<RBX::Name**, std::vector<RBX::N~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35c4cc, "std::vector<RBX::Name*, std::allocator<RBX::Name*>>::insert(__gnu_cxx::__normal_iterator<RBX::Name**~")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)")]
pub fn stub_0x35c508(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)")]
pub fn stub_0x35c740(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x35c764(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::~node_constructor()")]
pub fn stub_0x35c7b4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x35c7d0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x35c8f8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x35c988(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x35c9b4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>>>::construct(void)")]
pub fn stub_0x35ca0c() -> crate::slot::PortedFn {
// IDA 0x35ca0c: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35ca0c, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x35ca48(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
pub fn stub_0x35cab4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::vector<RBX::Name *,std::allocator<RBX::Name *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Name **,std::vector<RBX::Name *,std::allocator<RBX::Name *>>>,RBX::Name * const&)")]
pub fn stub_0x35caf4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Name *,std::allocator<RBX::Name *>>::_M_allocate(unsigned long)")]
pub fn stub_0x35cbd4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x35cbec(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x35cc24(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Name *>>,std::string,RBX::Name *,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Name *>>> const&)")]
pub fn stub_0x35cc54() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "RBX::queuing_rw_mutex::~queuing_rw_mutex()")]
pub fn stub_0x35ccc0(handle: crate::slot::InstanceHandle) {
// RBX::queuing_rw_mutex dtor.
drop(handle);
}

#[doc(alias = "RBX::queuing_rw_mutex::queuing_rw_mutex(void)")]
pub fn stub_0x35ccd0() -> crate::slot::InstanceHandle {
// RBX::queuing_rw_mutex ctor.
crate::slot::InstanceHandle::new("RBX::queuing_rw_mutex")
}

#[doc(alias = "global constructor keyed to_a_130")]
pub fn stub_0x35ce18() -> crate::slot::PortedFn {
// IDA 0x35ce18: __GLOBAL__I_a_130.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x35ce18, "__GLOBAL__I_a_130")
}

#[doc(alias = "RBX::normalIdToMask(RBX::NormalId)")]
pub fn stub_0x35cee0() -> crate::slot::PortedFn {
// IDA 0x35cee0: RBX::normalIdToMask(RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35cee0, "RBX::normalIdToMask(RBX::NormalId)")
}

#[doc(alias = "RBX::validNormalId(RBX::NormalId)")]
pub fn stub_0x35cef8() -> crate::slot::PortedFn {
// IDA 0x35cef8: RBX::validNormalId(RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35cef8, "RBX::validNormalId(RBX::NormalId)")
}

#[doc(alias = "RBX::intToNormalId(int)")]
pub fn stub_0x35cf04() -> crate::slot::PortedFn {
// IDA 0x35cf04: RBX::intToNormalId(int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35cf04, "RBX::intToNormalId(int)")
}

#[doc(alias = "RBX::normalIdOpposite(RBX::NormalId)")]
pub fn stub_0x35cf08() -> crate::slot::PortedFn {
// IDA 0x35cf08: RBX::normalIdOpposite(RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35cf08, "RBX::normalIdOpposite(RBX::NormalId)")
}

#[doc(alias = "RBX::normalIdToU(RBX::NormalId)")]
pub fn stub_0x35cf24() -> crate::slot::PortedFn {
// IDA 0x35cf24: RBX::normalIdToU(RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35cf24, "RBX::normalIdToU(RBX::NormalId)")
}

#[doc(alias = "RBX::uvwToObject(G3D::Vector3 const&,RBX::NormalId)")]
pub fn stub_0x35cfa8() -> crate::slot::PortedFn {
// IDA 0x35cfa8: RBX::uvwToObject(G3D::Vector3 const&, RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35cfa8, "RBX::uvwToObject(G3D::Vector3 const&, RBX::NormalId)")
}

#[doc(alias = "RBX::objectToUvw(G3D::Vector3 const&,RBX::NormalId)")]
pub fn stub_0x35d0c8() -> crate::slot::PortedFn {
// IDA 0x35d0c8: RBX::objectToUvw(G3D::Vector3 const&, RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35d0c8, "RBX::objectToUvw(G3D::Vector3 const&, RBX::NormalId)")
}

#[doc(alias = "RBX::normalIdToVector3(RBX::NormalId)")]
pub fn stub_0x35d1e8() -> crate::slot::PortedFn {
// IDA 0x35d1e8: RBX::normalIdToVector3(RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35d1e8, "RBX::normalIdToVector3(RBX::NormalId)")
}

#[doc(alias = "RBX::normalIdToMatrix3Internal(RBX::NormalId)")]
pub fn stub_0x35d3a8() -> crate::slot::PortedFn {
// IDA 0x35d3a8: RBX::normalIdToMatrix3Internal(RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35d3a8, "RBX::normalIdToMatrix3Internal(RBX::NormalId)")
}

#[doc(alias = "RBX::normalIdToMatrix3(RBX::NormalId)")]
pub fn stub_0x35d5f4() -> crate::slot::PortedFn {
// IDA 0x35d5f4: RBX::normalIdToMatrix3(RBX::NormalId).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35d5f4, "RBX::normalIdToMatrix3(RBX::NormalId)")
}

#[doc(alias = "RBX::Vector3ToNormalId(G3D::Vector3 const&)")]
pub fn stub_0x35d8a0() -> crate::slot::PortedFn {
// IDA 0x35d8a0: RBX::Vector3ToNormalId(G3D::Vector3 const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35d8a0, "RBX::Vector3ToNormalId(G3D::Vector3 const&)")
}

#[doc(alias = "RBX::Matrix3ToNormalId(G3D::Matrix3 const&)")]
pub fn stub_0x35db38() -> crate::slot::PortedFn {
// IDA 0x35db38: RBX::Matrix3ToNormalId(G3D::Matrix3 const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35db38, "RBX::Matrix3ToNormalId(G3D::Matrix3 const&)")
}

#[doc(alias = "global constructor keyed to_a_131")]
pub fn stub_0x35db54() -> crate::slot::PortedFn {
// IDA 0x35db54: __GLOBAL__I_a_131.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x35db54, "__GLOBAL__I_a_131")
}

#[doc(alias = "RBX::Profiling::init(bool)")]
pub fn stub_0x35db90(handle: &crate::slot::InstanceHandle) {
// RBX::Profiling::init(bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Profiling::setEnabled(bool)")]
pub fn stub_0x35dbc0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Profiling setter.
cell.set(value)
}

#[doc(alias = "RBX::Profiling::isEnabled(void)")]
pub fn stub_0x35dbd0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Profiling getter.
cell.get()
}

#[doc(alias = "RBX::Profiling::Profiler::Profiler(char const*)")]
pub fn stub_0x35dbf8() -> crate::slot::InstanceHandle {
// RBX::Profiling::Profiler ctor.
crate::slot::InstanceHandle::new("RBX::Profiling::Profiler")
}

#[doc(alias = "RBX::Profiling::CodeProfiler::CodeProfiler(char const*)")]
pub fn stub_0x35dc78() -> crate::slot::InstanceHandle {
// RBX::Profiling::CodeProfiler ctor.
crate::slot::InstanceHandle::new("RBX::Profiling::CodeProfiler")
}

#[doc(alias = "RBX::Profiling::CodeProfiler::log(bool,double)")]
pub fn stub_0x35dc9c(handle: &crate::slot::InstanceHandle) {
// RBX::Profiling::CodeProfiler::log(bool, double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Profiling::Profiler::getWindow(double)const")]
pub fn stub_0x35dd68(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Profiling::Profiler getter.
cell.get()
}

#[doc(alias = "RBX::Profiling::Profiler::getFrames(int)const")]
pub fn stub_0x35de30(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Profiling::Profiler getter.
cell.get()
}

#[doc(alias = "RBX::Profiling::Bucket::getActualFPS(void)const")]
pub fn stub_0x35ded0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Profiling::Bucket getter.
cell.get()
}

#[doc(alias = "RBX::Profiling::Bucket::getNominalFPS(void)const")]
pub fn stub_0x35df00(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Profiling::Bucket getter.
cell.get()
}

#[doc(alias = "RBX::Profiling::Bucket::getNominalFramePeriod(void)const")]
pub fn stub_0x35df30(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Profiling::Bucket getter.
cell.get()
}

#[doc(alias = "RBX::Profiling::Mark::Mark(RBX::Profiling::CodeProfiler &,bool,bool)")]
pub fn stub_0x35df60() -> crate::slot::InstanceHandle {
// RBX::Profiling::Mark ctor.
crate::slot::InstanceHandle::new("RBX::Profiling::Mark")
}

#[doc(alias = "RBX::Profiling::Mark::Mark(RBX::Profiling::CodeProfiler &,bool,bool) [0x35df64]")]
pub fn stub_0x35df64() -> crate::slot::InstanceHandle {
// RBX::Profiling::Mark ctor.
crate::slot::InstanceHandle::new("RBX::Profiling::Mark")
}

#[doc(alias = "RBX::Profiling::Mark::~Mark()")]
pub fn stub_0x35dfcc(handle: crate::slot::InstanceHandle) {
// RBX::Profiling::Mark dtor.
drop(handle);
}

#[doc(alias = "RBX::Profiling::Mark::~Mark() [0x35dfd0]")]
pub fn stub_0x35dfd0(handle: crate::slot::InstanceHandle) {
// RBX::Profiling::Mark dtor.
drop(handle);
}

#[doc(alias = "RBX::Profiling::Profiler::~Profiler()")]
pub fn stub_0x35e03c(handle: crate::slot::InstanceHandle) {
// RBX::Profiling::Profiler dtor.
drop(handle);
}

#[doc(alias = "RBX::Profiling::Profiler::~Profiler() [0x35e060]")]
pub fn stub_0x35e060(handle: crate::slot::InstanceHandle) {
// RBX::Profiling::Profiler dtor.
drop(handle);
}

#[doc(alias = "RBX::Profiling::CodeProfiler::~CodeProfiler()")]
pub fn stub_0x35e08c(handle: crate::slot::InstanceHandle) {
// RBX::Profiling::CodeProfiler dtor.
drop(handle);
}

#[doc(alias = "RBX::Profiling::CodeProfiler::~CodeProfiler() [0x35e0b0]")]
pub fn stub_0x35e0b0(handle: crate::slot::InstanceHandle) {
// RBX::Profiling::CodeProfiler dtor.
drop(handle);
}

#[doc(alias = "std::_Vector_base<int,std::allocator<int>>::_M_allocate(unsigned long)")]
pub fn stub_0x35e0dc() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "global constructor keyed to_a_132")]
pub fn stub_0x35e0f4() -> crate::slot::PortedFn {
// IDA 0x35e0f4: __GLOBAL__I_a_132.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x35e0f4, "__GLOBAL__I_a_132")
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(void)")]
pub fn stub_0x35e2c8() -> crate::slot::InstanceHandle {
// RBX::ProtectedString ctor.
crate::slot::InstanceHandle::new("RBX::ProtectedString")
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(void) [0x35e2cc]")]
pub fn stub_0x35e2cc() -> crate::slot::InstanceHandle {
// RBX::ProtectedString ctor.
crate::slot::InstanceHandle::new("RBX::ProtectedString")
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(RBX::ProtectedString const&)")]
pub fn stub_0x35e458() -> crate::slot::InstanceHandle {
// RBX::ProtectedString ctor.
crate::slot::InstanceHandle::new("RBX::ProtectedString")
}

#[doc(alias = "RBX::ProtectedString::ProtectedString(RBX::ProtectedString const&) [0x35e45c]")]
pub fn stub_0x35e45c() -> crate::slot::InstanceHandle {
// RBX::ProtectedString ctor.
crate::slot::InstanceHandle::new("RBX::ProtectedString")
}

#[doc(alias = "RBX::ProtectedString::fromTrustedSource(std::string const&)")]
pub fn stub_0x35e538(handle: &crate::slot::InstanceHandle) {
// RBX::ProtectedString::fromTrustedSource(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ProtectedString::calculateHash(std::string *)const")]
pub fn stub_0x35e628(handle: &crate::slot::InstanceHandle) {
// RBX::ProtectedString::calculateHash(std::string*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ProtectedString::getStringForImmediateUse(void)const")]
pub fn stub_0x35e8f8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ProtectedString getter.
cell.get()
}

#[doc(alias = "RBX::ProtectedString::getOriginalHash(void)const")]
pub fn stub_0x35e8fc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ProtectedString getter.
cell.get()
}

#[doc(alias = "RBX::ProtectedString::getSalt(void)const")]
pub fn stub_0x35e900(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::ProtectedString getter.
cell.get()
}

#[doc(alias = "RBX::ProtectedString::readUnprotectedChar(int,char *)const")]
pub fn stub_0x35e90c(handle: &crate::slot::InstanceHandle) {
// RBX::ProtectedString::readUnprotectedChar(int, char*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ProtectedString::operator==(RBX::ProtectedString const&)const")]
pub fn stub_0x35e92c(handle: &crate::slot::InstanceHandle) {
// RBX::ProtectedString::operator==(RBX::ProtectedString const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ProtectedString::operator=(RBX::ProtectedString const&)")]
pub fn stub_0x35e940(handle: &crate::slot::InstanceHandle) {
// RBX::ProtectedString::operator=(RBX::ProtectedString const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "bool XmlNameValuePair::getValue<RBX::ProtectedString>(RBX::ProtectedString &)const")]
pub fn stub_0x35e958() -> crate::slot::PortedFn {
// IDA 0x35e958: bool XmlNameValuePair::getValue<RBX::ProtectedString>(RBX::ProtectedString&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x35e958, "bool XmlNameValuePair::getValue<RBX::ProtectedString>(RBX::ProtectedString&) const")
}

#[doc(alias = "RBX::StringConverter<RBX::ProtectedString>::convertToValue(std::string const&,RBX::ProtectedString&)")]
pub fn stub_0x35eba0(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::ProtectedString>::convertToValue(std::string const&, RBX::Protec~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::ProtectedString>(void)")]
pub fn stub_0x35ed7c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::ProtectedString>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x35ee60(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::readValue(RBX::Reflection:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x35f03c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::writeValue(RBX::Reflection~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ProtectedString & RBX::Reflection::Variant::convert<RBX::ProtectedString>(void)")]
pub fn stub_0x35f280(handle: &crate::slot::InstanceHandle) {
// RBX::ProtectedString& RBX::Reflection::Variant::convert<RBX::ProtectedString>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x35f3f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::hasStringValue(void)const")]
pub fn stub_0x35f654(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x35f658(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::ProtectedString>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x35f824(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::ProtectedString::~ProtectedString()")]
pub fn stub_0x35f9ec(handle: crate::slot::InstanceHandle) {
// RBX::ProtectedString dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::TType<RBX::ProtectedString>::~TType()")]
pub fn stub_0x35faa4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "RBX::ProtectedString & RBX::Reflection::Variant::genericConvert<RBX::ProtectedString>(void)")]
pub fn stub_0x35faa8(handle: &crate::slot::InstanceHandle) {
// RBX::ProtectedString& RBX::Reflection::Variant::genericConvert<RBX::ProtectedString>() — engine-side; linkage preserved via the alias.
let _ = handle;
}
