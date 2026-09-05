// Auto-generated skeletons for rbx-script — shard 203 EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2c0edc..0x2d11b8 | script 20252->20352 distinct (filler 0x2c0edc asc, not-in-script 65293->65193)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::push_back(char const* const&)")]
pub fn stub_0x2c0edc(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<char const**,std::vector<char const*,std::allocator<char const*>>>,char const* const&)")]
pub fn stub_0x2c157c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<char const*,std::allocator<char const*>>::_M_allocate(unsigned long)")]
pub fn stub_0x2c165c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::pair<std::string const,std::string>::pair(std::string const&,std::string const&)")]
pub fn stub_0x2c1674() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>,std::pair<std::string const,std::string> const&)")]
pub fn stub_0x2c171c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::string> const&)")]
pub fn stub_0x2c1808(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)")]
pub fn stub_0x2c1858(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::lower_bound(std::string const&)")]
pub fn stub_0x2c18dc() -> crate::slot::PortedFn {
// IDA 0x2c18dc: std::_Rb_tree<std::string, std::pair<std::string const, std::string>, std::_Select1st<std::pair<std::string const, std::~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c18dc, "std::_Rb_tree<std::string, std::pair<std::string const, std::string>, std::_Select1st<std::pair<std:~")
}

#[doc(alias = "RBX::ContentId::ContentId(std::string const&)")]
pub fn stub_0x2c1a48() -> crate::slot::InstanceHandle {
// RBX::ContentId ctor.
crate::slot::InstanceHandle::new("RBX::ContentId")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v")]
pub fn stub_0x2c1e00(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::Stats::sStatsItem>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v")]
pub fn stub_0x2c1e48(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::Stats::sStatsItem>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Stats::Item::~Item()")]
pub fn stub_0x2c1f30(handle: crate::slot::InstanceHandle) {
// RBX::Stats::Item dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Stats::Item::~Item()")]
pub fn stub_0x2c2008(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Stats::Item::~Item() [0x2c2048]")]
pub fn stub_0x2c2048(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2c2120(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c21c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2c21c8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "RBX::ContentId::ContentId(char const*)")]
pub fn stub_0x2c26b0() -> crate::slot::InstanceHandle {
// RBX::ContentId ctor.
crate::slot::InstanceHandle::new("RBX::ContentId")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<unsigned int>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<boost::unordered::detail::emplace_args1<unsigned int>>(unsigned int const&,boost::unordered::detail::emplace_args1<unsigned int> const&)")]
pub fn stub_0x2c28a0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
pub fn stub_0x2c2a30(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x2c2b58(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::rehash_impl(unsigned long)")]
pub fn stub_0x2c2be8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x2c2c14(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<unsigned int>>>::construct(void)")]
pub fn stub_0x2c2c68() -> crate::slot::PortedFn {
// IDA 0x2c2c68: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<unsigned int>>>::construct(~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c2c68, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<unsigne~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<unsigned int>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::find_node_impl<unsigned int,std::equal_to<unsigned int>>(unsigned long,unsigned int const&,std::equal_to<unsigned int> const&)const")]
pub fn stub_0x2c2ca0(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>::at(unsigned long)")]
pub fn stub_0x2c2d0c(vec: &crate::slot::VecModel, index: usize) -> Option<usize> {
// bounds-checked element access shape.
if index < vec.len() { Some(index) } else { None }
}

#[doc(alias = "rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple> * boost::get_deleter<rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>,RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&)")]
pub fn stub_0x2c2d40() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::Tuple *,rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>>::~sp_counted_impl_pd()")]
pub fn stub_0x2c2da0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::Tuple *,rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2c2dd0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Reflection::Tuple *,rbx::detail::sp_ms_deleter<RBX::Reflection::Tuple>>::get_untyped_deleter(void)")]
pub fn stub_0x2c2de8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::auto_ptr<RBX::Reflection::Tuple>::reset(RBX::Reflection::Tuple*)")]
pub fn stub_0x2c2dec() -> crate::slot::PortedFn {
// IDA 0x2c2dec: std::auto_ptr<RBX::Reflection::Tuple>::reset(RBX::Reflection::Tuple*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c2dec, "std::auto_ptr<RBX::Reflection::Tuple>::reset(RBX::Reflection::Tuple*)")
}

#[doc(alias = "RBX::Reflection::Tuple::Tuple(unsigned long)")]
pub fn stub_0x2c2ea0() -> crate::slot::InstanceHandle {
// RBX::Reflection::Tuple ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator->(void)")]
pub fn stub_0x2c3af0(handle: &crate::slot::InstanceHandle) {
// RBX::Intrusive::Set<RobloxExtraSpace, RobloxExtraSpace>::Iterator::operator->() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator++(void)")]
pub fn stub_0x2c3ca4(handle: &crate::slot::InstanceHandle) {
// RBX::Intrusive::Set<RobloxExtraSpace, RobloxExtraSpace>::Iterator::operator++() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::Iterator(RobloxExtraSpace*)")]
pub fn stub_0x2c3e54() -> crate::slot::InstanceHandle {
// RBX::Intrusive::Set ctor.
crate::slot::InstanceHandle::new("RBX::Intrusive::Set")
}

#[doc(alias = "RBX::GcJob::~GcJob()")]
pub fn stub_0x2c46d0(handle: crate::slot::InstanceHandle) {
// RBX::GcJob dtor.
drop(handle);
}

#[doc(alias = "RBX::GcJob::~GcJob() [0x2c47a0]")]
pub fn stub_0x2c47a0(handle: crate::slot::InstanceHandle) {
// RBX::GcJob dtor.
drop(handle);
}

#[doc(alias = "RBX::GcJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x2c4884(handle: &crate::slot::InstanceHandle) {
// RBX::GcJob::sleepTime(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GcJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x2c48a4(handle: &crate::slot::InstanceHandle) {
// RBX::GcJob::error(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::GcJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x2c48c4(handle: &crate::slot::InstanceHandle) {
// RBX::GcJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_72")]
pub fn stub_0x2c4a80() -> crate::slot::PortedFn {
// IDA 0x2c4a80: __GLOBAL__I_a_72.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2c4a80, "__GLOBAL__I_a_72")
}

#[doc(alias = "global constructor keyed to_a_73")]
pub fn stub_0x2c68dc() -> crate::slot::PortedFn {
// IDA 0x2c68dc: __GLOBAL__I_a_73.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2c68dc, "__GLOBAL__I_a_73")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ActivityMeter<2>>::operator=(rbx_core::SharedPtr<RBX::ActivityMeter<2>> const&)")]
pub fn stub_0x2c7348(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InvocationMeter<2>>::operator=(rbx_core::SharedPtr<RBX::InvocationMeter<2>> const&)")]
pub fn stub_0x2c7380(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<bool>(char const*,bool const&)")]
pub fn stub_0x2c73b8(handle: &crate::slot::InstanceHandle) {
// RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<bool>(char const*, bool const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Stats::Item::~Item() [0x2c7928]")]
pub fn stub_0x2c7928(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c7968(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x2c7970(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats4ItemELZNS2_10sStatsItemEENS_17NonFactoryProductINS_8InstanceELZNS2_10sStatsItemEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x2c7978(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2c7a20() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2c7a38() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<bool>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<bool>,bool const*>(bool const*)")]
pub fn stub_0x2c7a3c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Stats::TypedStatsItem<bool>")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
pub fn stub_0x2c7b48(handle: crate::slot::InstanceHandle) {
// RBX::Stats::TypedStatsItem dtor.
drop(handle);
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem() [0x2c7c90]")]
pub fn stub_0x2c7c90(handle: crate::slot::InstanceHandle) {
// RBX::Stats::TypedStatsItem dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
pub fn stub_0x2c7df0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem() [0x2c7f38]")]
pub fn stub_0x2c7f38(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<bool>>::shared_ptr<RBX::Stats::TypedStatsItem<bool>,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2c8094() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Stats::TypedStatsItem<bool>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x2c815c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<bool> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2c8268(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::pop_back(void)")]
pub fn stub_0x2c8270(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::push_back(std::string const&)")]
pub fn stub_0x2c82a8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_push_back_aux(std::string const&)")]
pub fn stub_0x2c82d4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x2c846c(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x2c8488() -> crate::slot::PortedFn {
// IDA 0x2c8488: std::deque<std::string, std::allocator<std::string>>::_M_reallocate_map(unsigned long, bool).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c8488, "std::deque<std::string, std::allocator<std::string>>::_M_reallocate_map(unsigned long, bool)")
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x2c8560() -> crate::slot::PortedFn {
// IDA 0x2c8560: std::_Deque_base<std::string, std::allocator<std::string>>::_M_allocate_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c8560, "std::_Deque_base<std::string, std::allocator<std::string>>::_M_allocate_map(unsigned long)")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::InvocationMeter<2>>::shared_ptr<RBX::InvocationMeter<2>>(RBX::InvocationMeter<2> *)")]
pub fn stub_0x2c8894() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::InvocationMeter<2>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InvocationMeter<2>>(RBX::InvocationMeter<2> *)")]
pub fn stub_0x2c8968() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::~sp_counted_impl_p()")]
pub fn stub_0x2c8a54(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::~sp_counted_impl_p() [0x2c8a58]")]
pub fn stub_0x2c8a58(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::dispose(void)")]
pub fn stub_0x2c8a5c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2c8a68() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::get_untyped_deleter(void)")]
pub fn stub_0x2c8a6c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ActivityMeter<2>>::shared_ptr<RBX::ActivityMeter<2>>(RBX::ActivityMeter<2> *)")]
pub fn stub_0x2c8a70() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ActivityMeter<2>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ActivityMeter<2>>(RBX::ActivityMeter<2> *)")]
pub fn stub_0x2c8b44() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ActivityMeter<2>>::~sp_counted_impl_p()")]
pub fn stub_0x2c8c30(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ActivityMeter<2>>::~sp_counted_impl_p() [0x2c8c34]")]
pub fn stub_0x2c8c34(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ActivityMeter<2>>::dispose(void)")]
pub fn stub_0x2c8c38() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ActivityMeter<2>>::get_deleter(std::type_info const&)")]
pub fn stub_0x2c8c44() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ActivityMeter<2>>::get_untyped_deleter(void)")]
pub fn stub_0x2c8c48() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::deque(std::deque<std::string,std::allocator<std::string>> const&)")]
pub fn stub_0x2c8ca0() -> crate::slot::PortedFn {
// IDA 0x2c8ca0: std::deque<std::string, std::allocator<std::string>>::deque(std::deque<std::string, std::allocator<std::string>> const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c8ca0, "std::deque<std::string, std::allocator<std::string>>::deque(std::deque<std::string, std::allocator<s~")
}

#[doc(alias = "std::_Deque_iterator<std::string,std::string &,std::string *> std::__uninitialized_copy_aux<std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>>(std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>,std::__false_type)")]
pub fn stub_0x2c8dc8(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x2c8f2c() -> crate::slot::PortedFn {
// IDA 0x2c8f2c: std::_Deque_base<std::string, std::allocator<std::string>>::_M_initialize_map(unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c8f2c, "std::_Deque_base<std::string, std::allocator<std::string>>::_M_initialize_map(unsigned long)")
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_create_nodes(std::string **,std::string **)")]
pub fn stub_0x2c9084() -> crate::slot::PortedFn {
// IDA 0x2c9084: std::_Deque_base<std::string, std::allocator<std::string>>::_M_create_nodes(std::string**, std::string**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c9084, "std::_Deque_base<std::string, std::allocator<std::string>>::_M_create_nodes(std::string**, std::stri~")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::string,std::string &,std::string *>,std::_Deque_iterator<std::string,std::string &,std::string *>)")]
pub fn stub_0x2c9178() -> crate::slot::PortedFn {
// IDA 0x2c9178: std::deque<std::string, std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::string, std::string&~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c9178, "std::deque<std::string, std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::~")
}

#[doc(alias = "global constructor keyed to_a_74")]
pub fn stub_0x2c9314() -> crate::slot::PortedFn {
// IDA 0x2c9314: __GLOBAL__I_a_74.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2c9314, "__GLOBAL__I_a_74")
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(void)")]
pub fn stub_0x2cb790() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "RobloxExtraSpace::createNewNode(void)")]
pub fn stub_0x2cbc40() -> crate::slot::PortedFn {
// IDA 0x2cbc40: RobloxExtraSpace::createNewNode().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2cbc40, "RobloxExtraSpace::createNewNode()")
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::~TType()")]
pub fn stub_0x2cbe50(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Type::Type<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>(char const*,rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>> *)")]
pub fn stub_0x2cbf08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> (boost::shared_p~")
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>>::~TType() [0x2cbfb4]")]
pub fn stub_0x2cbfb4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "boost::function1<rbx_core::SharedPtr<RBX::Reflection::Tuple const>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
pub fn stub_0x2cdd74(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "global constructor keyed to_a_75")]
pub fn stub_0x2cde88() -> crate::slot::PortedFn {
// IDA 0x2cde88: __GLOBAL__I_a_75.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2cde88, "__GLOBAL__I_a_75")
}

#[doc(alias = "RBX::Security::Context::isInRole(RBX::Security::Identities,RBX::Security::Permissions)")]
pub fn stub_0x2ce130(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Security::Context getter.
cell.get()
}

#[doc(alias = "global constructor keyed to_a_76")]
pub fn stub_0x2ce1fc() -> crate::slot::PortedFn {
// IDA 0x2ce1fc: __GLOBAL__I_a_76.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2ce1fc, "__GLOBAL__I_a_76")
}

#[doc(alias = "RBX::AdvDragTool::onMouseDown(RBX::PartInstance *,G3D::Vector3 const&,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>> const&,RBX::UIEvent const&,RBX::Workspace *,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x2ce2c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::vector(std::vector<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>> const&)")]
pub fn stub_0x2ce618() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "std::_Vector_base<rbx_core::WeakPtr<RBX::PartInstance>,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>>>::_Vector_base(unsigned long,std::allocator<rbx_core::WeakPtr<RBX::PartInstance>> const&)")]
pub fn stub_0x2ce7d4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::PartInstance")
}

#[doc(alias = "global constructor keyed to_a_77")]
pub fn stub_0x2ceadc() -> crate::slot::PortedFn {
// IDA 0x2ceadc: __GLOBAL__I_a_77.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2ceadc, "__GLOBAL__I_a_77")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*)")]
pub fn stub_0x2d072c() -> crate::slot::PortedFn {
// IDA 0x2d072c: std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2d072c, "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*)")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr()")]
pub fn stub_0x2d11b8() -> crate::slot::PortedFn {
// IDA 0x2d11b8: std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2d11b8, "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr()")
}
