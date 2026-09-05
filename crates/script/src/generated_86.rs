// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x77ad64..0x77ffe8 | remaining 1990 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::slot::~slot() [0x77ae80]")]
pub fn stub_0x77ae80(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(lua_State *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Scripting::ScriptDebugger,lua_State *>,boost::_bi::list2<boost::_bi::value<RBX::Scripting::ScriptDebugger*>,boost::arg<1>>>,1,void ()(lua_State *)>::~callable() [0x77af80]")]
pub fn stub_0x77af80(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x77af80: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&)")]
pub fn stub_0x77b058(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::Scripting::DebuggerWatch*, std::allocator<RBX::Scripting::DebuggerWatch*>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>::_M_allocate(unsigned long)")]
pub fn stub_0x77b138(handle: &crate::slot::InstanceHandle) {
// std::_Vector_base<RBX::Scripting::DebuggerWatch*, std::allocator<RBX::Scripting::DebuggerW~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::operator[](int const&)")]
pub fn stub_0x77b150(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x77b2cc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::create_buckets(unsigned long)")]
pub fn stub_0x77b320(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x77b448(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::rehash_impl(unsigned long)")]
pub fn stub_0x77b4d8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x77b504(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>>>::construct(void)")]
pub fn stub_0x77b55c(handle: &crate::slot::InstanceHandle) {
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_no~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::find_node_impl<int,std::equal_to<int>>(unsigned long,int const&,std::equal_to<int> const&)const")]
pub fn stub_0x77b594(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch *>(__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch **,std::vector<RBX::Scripting::DebuggerWatch *,std::allocator<RBX::Scripting::DebuggerWatch *>>>,RBX::Scripting::DebuggerWatch * const&,std::random_access_iterator_tag)")]
pub fn stub_0x77b600() -> crate::slot::PortedFn {
// IDA 0x77b600: __gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch**, std::vector<RBX::Scripting::DebuggerWatch*, std::allocator~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x77b600, "__gnu_cxx::__normal_iterator<RBX::Scripting::DebuggerWatch**, std::vector<RBX::Scripting::DebuggerWa~")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::erase_key(int const&)")]
pub fn stub_0x77b690(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x77b710(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x77b74c(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting13DebuggerWatchELZNS2_14sDebuggerWatchEENS_14FactoryProductIS3_NS_8InstanceELZNS2_14sDebuggerWatchEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
pub fn stub_0x77b790(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Scripting::DebuggerWatch, RBX::Scripting::sDebuggerWatch, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Scripting::ScriptDebugger::FunctionInfo*,std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>>,RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
pub fn stub_0x77b8ac(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo, std::allocator<RBX::Scripting::S~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo::operator=(RBX::Scripting::ScriptDebugger::FunctionInfo const&)")]
pub fn stub_0x77c1c4(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::FunctionInfo::operator=(RBX::Scripting::ScriptDebugger::Fu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::_M_allocate(unsigned long)")]
pub fn stub_0x77c214(handle: &crate::slot::InstanceHandle) {
// std::_Vector_base<RBX::Scripting::ScriptDebugger::FunctionInfo, std::allocator<RBX::Script~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__uninitialized_copy_a<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>)")]
pub fn stub_0x77c238(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::FunctionInfo* std::__uninitialized_copy_a<RBX::Scripting::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Scripting::ScriptDebugger::FunctionInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *>(RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *,RBX::Scripting::ScriptDebugger::FunctionInfo *)")]
pub fn stub_0x77c538(handle: &crate::slot::InstanceHandle) {
// RBX::Scripting::ScriptDebugger::FunctionInfo* std::__copy_backward<false, std::random_acce~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Scripting::ScriptDebugger::FunctionInfo,std::allocator<RBX::Scripting::ScriptDebugger::FunctionInfo>>::~vector()")]
pub fn stub_0x77c594(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::operator[](RBX::Script const* const&)")]
pub fn stub_0x77e760(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x77e8e0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::create_buckets(unsigned long)")]
pub fn stub_0x77e930(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x77ea58(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::rehash_impl(unsigned long)")]
pub fn stub_0x77eae8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x77eb14(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>>>::construct(void)")]
pub fn stub_0x77eb6c(handle: &crate::slot::InstanceHandle) {
// boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_no~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::find_node_impl<RBX::Script const*,std::equal_to<RBX::Script const*>>(unsigned long,RBX::Script const* const&,std::equal_to<RBX::Script const*> const&)const")]
pub fn stub_0x77eba4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::erase_key(RBX::Script const* const&)")]
pub fn stub_0x77ec10(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x77ec98(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x77ecd4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript>::shared_ptr<RBX::BaseScript>(rbx_core::WeakPtr<RBX::BaseScript> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x77ed18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseScript")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::delete_buckets(void)")]
pub fn stub_0x77f214(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>,RBX::Script const*,RBX::Scripting::ScriptDebugger *,boost::hash<RBX::Script const*>,std::equal_to<RBX::Script const*>>>::table(unsigned long,boost::hash<RBX::Script const*> const&,std::equal_to<RBX::Script const*> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Script const* const,RBX::Scripting::ScriptDebugger *>>> const&)")]
pub fn stub_0x77f244() -> crate::slot::InstanceHandle {
// boost::unordered::detail::table ctor — fresh debugger identity.
crate::slot::InstanceHandle::new("boost::unordered::detail::table")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x77f2b0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x77f2b4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x77f354(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x77f35c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
pub fn stub_0x77f400(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
pub fn stub_0x77f408(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::BoundFuncDesc(void (RBX::Scripting::DebuggerWatch::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x77f4ac() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::DebuggerWatch", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::~BoundFuncDesc() [0x77f5b0]")]
pub fn stub_0x77f5b0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Scripting::DebuggerWatch,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x77f664() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::Scripting::DebuggerWatch", "void", 0)
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerWatch>(char const*,char const*,std::string  RBX::Scripting::DebuggerWatch::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x77f688() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isReadOnly(void)const")]
pub fn stub_0x77f81c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::isWriteOnly(void)const")]
pub fn stub_0x77f820() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x77f824() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerWatch>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x77f83c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,std::string  RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x77f8a4() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
pub fn stub_0x77fa34() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
pub fn stub_0x77fa38() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x77fa3c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x77fa54() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "std::string")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,bool RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x77fabc() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
pub fn stub_0x77fc50() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
pub fn stub_0x77fc54() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x77fc58() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x77fc64() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "bool")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Scripting::DebuggerBreakpoint>(char const*,char const*,int RBX::Scripting::DebuggerBreakpoint::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x77fcb4() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isReadOnly(void)const")]
pub fn stub_0x77fe48() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::isWriteOnly(void)const")]
pub fn stub_0x77fe4c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x77fe50() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Scripting::DebuggerBreakpoint>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
pub fn stub_0x77fe5c() -> crate::slot::DescriptorHandle {
// BoundProp ctor — registers the property binding.
crate::slot::DescriptorHandle::prop("(RBX::Reflection::Mutability)1", "int")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::PropDescriptor<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int>(char const*,char const*,int (RBX::Scripting::DebuggerBreakpoint::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x77feac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::~PropDescriptor() [0x77ffb8]")]
pub fn stub_0x77ffb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isReadOnly(void)const")]
pub fn stub_0x77ffe4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Scripting::DebuggerBreakpoint,int>::GetImpl<int (RBX::Scripting::DebuggerBreakpoint::*)(void)const>::isWriteOnly(void)const")]
pub fn stub_0x77ffe8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}
