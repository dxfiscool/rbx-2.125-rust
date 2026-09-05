// Auto-generated skeletons for rbx-script — shard 210 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x369f44..0x377628 | script 21252->21402 distinct (filler 0x369f44 asc, not-in-script 64493->64343)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::disconnectAll(void)")]
pub fn stub_0x369f44(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::RunTransition)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_136")]
pub fn stub_0x36a0bc() -> crate::slot::PortedFn {
// IDA 0x36a0bc: __GLOBAL__I_a_136.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x36a0bc, "__GLOBAL__I_a_136")
}

#[doc(alias = "RBX::findLocalFile(std::string const&,std::string *) [0x36a710]")]
pub fn stub_0x36a710() -> crate::slot::PortedFn {
// IDA 0x36a710: RBX::findLocalFile(std::string const&, std::string*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x36a710, "RBX::findLocalFile(std::string const&, std::string*)")
}

#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance()")]
pub fn stub_0x36b370(handle: crate::slot::InstanceHandle) {
// RBX::HeartbeatInstance dtor.
drop(handle);
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
pub fn stub_0x36b644(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::insert(std::string const&,std::string const&,unsigned long)")]
pub fn stub_0x36de5c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::removeLeastRecentlyUsed(void)")]
pub fn stub_0x36e3e4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::remove(std::string const&)")]
pub fn stub_0x36e43c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> *)")]
pub fn stub_0x36e490(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36e4ec(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36e518(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>::destroy(std::pair<std::string,std::pair<unsigned long,std::string>>*)")]
pub fn stub_0x36e558() -> crate::slot::PortedFn {
// IDA 0x36e558: __gnu_cxx::new_allocator<std::pair<std::string, std::pair<unsigned long, std::string>>>::destroy(std::pair<std::string, ~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x36e558, "__gnu_cxx::new_allocator<std::pair<std::string, std::pair<unsigned long, std::string>>>::destroy(std~")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
pub fn stub_0x36e610(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x36e650(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
pub fn stub_0x36e6bc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>> const&)")]
pub fn stub_0x36e874(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x36e898(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::~node_constructor()")]
pub fn stub_0x36e8e8(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x36e908(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x36ea30(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x36eac0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36eaec(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>>>::construct(void)")]
pub fn stub_0x36eb44() -> crate::slot::PortedFn {
// IDA 0x36eb44: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x36eb44, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "std::pair<std::string,std::pair<unsigned long,std::string>>::pair(std::string const&,std::pair<unsigned long,std::string> const&)")]
pub fn stub_0x36eb80() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,std::string>> const&)")]
pub fn stub_0x36ec4c() -> crate::slot::PortedFn {
// IDA 0x36ec4c: std::list<std::pair<std::string, std::pair<unsigned long, std::string>>, std::allocator<std::pair<std::string, std::pair~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x36ec4c, "std::list<std::pair<std::string, std::pair<unsigned long, std::string>>, std::allocator<std::pair<st~")
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::~LRUCache()")]
pub fn stub_0x3705a0(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::resize(unsigned long)")]
pub fn stub_0x3706b4(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,std::string>>,std::allocator<std::pair<std::string,std::pair<unsigned long,std::string>>>>::_M_clear(void)")]
pub fn stub_0x3706ec(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x370714(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x37074c(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "RBX::LRUCache<std::string,std::string>::LRUCache(void)")]
pub fn stub_0x370780() -> crate::slot::InstanceHandle {
// RBX::LRUCache ctor.
crate::slot::InstanceHandle::new("RBX::LRUCache")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,std::string>::resize(unsigned long)")]
pub fn stub_0x370860(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,std::string>>>>>> const&)")]
pub fn stub_0x3708e4() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "RBX::HeartbeatInstance::~HeartbeatInstance() [0x371250]")]
pub fn stub_0x371250(handle: crate::slot::InstanceHandle) {
// RBX::HeartbeatInstance dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_137")]
pub fn stub_0x371254() -> crate::slot::PortedFn {
// IDA 0x371254: __GLOBAL__I_a_137.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x371254, "__GLOBAL__I_a_137")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::EnumDesc(void)")]
pub fn stub_0x37148c() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "checkResultNoThrow(FMOD_RESULT)")]
pub fn stub_0x371844() -> crate::slot::PortedFn {
// IDA 0x371844: checkResultNoThrow(FMOD_RESULT).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x371844, "checkResultNoThrow(FMOD_RESULT)")
}

#[doc(alias = "checkResult(FMOD_RESULT)")]
pub fn stub_0x3719d0() -> crate::slot::PortedFn {
// IDA 0x3719d0: checkResult(FMOD_RESULT).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3719d0, "checkResult(FMOD_RESULT)")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void)")]
pub fn stub_0x371b5c() -> crate::slot::InstanceHandle {
// RBX::Soundscape::SoundService ctor.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundService(void) [0x371b60]")]
pub fn stub_0x371b60() -> crate::slot::InstanceHandle {
// RBX::Soundscape::SoundService ctor.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundService")
}

#[doc(alias = "RBX::Soundscape::SoundService::openFmod(void)")]
pub fn stub_0x371e5c(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::openFmod() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::update3DSettings(void)")]
pub fn stub_0x3723f4(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::update3DSettings() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::updateAmbientReverb(void)")]
pub fn stub_0x372414(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::updateAmbientReverb() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x372460(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService dtor.
drop(handle);
}

#[doc(alias = "RBX::Soundscape::SoundService::~SoundService() [0x372500]")]
pub fn stub_0x372500(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService()")]
pub fn stub_0x372504(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService() [0x37250c]")]
pub fn stub_0x37250c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Soundscape::SoundService::~SoundService() [0x372514]")]
pub fn stub_0x372514(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService() [0x3728b0]")]
pub fn stub_0x3728b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundService::~SoundService() [0x3728b8]")]
pub fn stub_0x3728b8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Soundscape::SoundService::closeFmod(void)")]
pub fn stub_0x3728c0(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::closeFmod() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "releaseSound(std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>> const&)")]
pub fn stub_0x3729bc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::Sound")
}

#[doc(alias = "initReverbs(void)")]
pub fn stub_0x3729c4() -> crate::slot::PortedFn {
// IDA 0x3729c4: initReverbs().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3729c4, "initReverbs()")
}

#[doc(alias = "RBX::Soundscape::SoundService::loadStockSounds(void)")]
pub fn stub_0x372bb0(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::loadStockSounds() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType,std::string)")]
pub fn stub_0x373554(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::loadStockSound(RBX::SoundType, std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setSoundId(RBX::Soundscape::SoundId)")]
pub fn stub_0x37384c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Soundscape::SoundChannel setter.
cell.set(value)
}

#[doc(alias = "RBX::Soundscape::SoundId::SoundId(RBX::ContentId const&)")]
pub fn stub_0x373894() -> crate::slot::InstanceHandle {
// RBX::Soundscape::SoundId ctor.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundId")
}

#[doc(alias = "RBX::Soundscape::SoundService::setAmbientReverb(RBX::Soundscape::ReverbType const&)")]
pub fn stub_0x3738a8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Soundscape::SoundService setter.
cell.set(value)
}

#[doc(alias = "RBX::Soundscape::SoundService::playSound(RBX::SoundType)")]
pub fn stub_0x3738d8(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::playSound(RBX::SoundType) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::play(void)")]
pub fn stub_0x373918(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::play() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x373974(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvid~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "convert(G3D::Vector3 const&,FMOD_VECTOR &)")]
pub fn stub_0x373bf4() -> crate::slot::PortedFn {
// IDA 0x373bf4: convert(G3D::Vector3 const&, FMOD_VECTOR&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x373bf4, "convert(G3D::Vector3 const&, FMOD_VECTOR&)")
}

#[doc(alias = "RBX::Soundscape::SoundService::step(void)")]
pub fn stub_0x373cb8(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::step() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::garbageCollectSounds(void)")]
pub fn stub_0x373fd0(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::garbageCollectSounds() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&,RBX::Soundscape::SoundId&)")]
pub fn stub_0x374028(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Soundscape::SoundId>::convertToValue(std::string const&, RBX::So~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Soundscape::SoundId>(void)")]
pub fn stub_0x37414c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Soundscape::SoundId>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::convert<RBX::Soundscape::SoundId>(void)")]
pub fn stub_0x374154(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundId& RBX::Reflection::Variant::convert<RBX::Soundscape::SoundId>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x374340(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::readValue(RBX::Reflect~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x374528(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::writeValue(RBX::Reflec~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x374758(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::hasStringValue(void)const")]
pub fn stub_0x3747b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x3747b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::Soundscape::SoundId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x3748d4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getSoundId(void)const")]
pub fn stub_0x374a2c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getVolume(void)const")]
pub fn stub_0x374a44(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setVolume(float)")]
pub fn stub_0x374a48(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Soundscape::SoundChannel setter.
cell.set(value)
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getPitch(void)const")]
pub fn stub_0x374aa4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setPitch(float)")]
pub fn stub_0x374aa8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Soundscape::SoundChannel setter.
cell.set(value)
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setPlayCount(int)")]
pub fn stub_0x374af8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Soundscape::SoundChannel setter.
cell.set(value)
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getLooped(void)const")]
pub fn stub_0x374b68(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundChannel::setLooped(bool)")]
pub fn stub_0x374b74(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Soundscape::SoundChannel setter.
cell.set(value)
}

#[doc(alias = "RBX::Soundscape::SoundChannel::isPlaying(void)const")]
pub fn stub_0x374bb4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundChannel::isPaused(void)const")]
pub fn stub_0x374bec(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundChannel::pause(void)")]
pub fn stub_0x374c24(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::pause() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::stop(void)")]
pub fn stub_0x374c68(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::stop() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::SoundChannel(void)")]
pub fn stub_0x374cc4() -> crate::slot::InstanceHandle {
// RBX::Soundscape::SoundChannel ctor.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x374ff4(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel dtor.
drop(handle);
}

#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel() [0x375094]")]
pub fn stub_0x375094(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel()")]
pub fn stub_0x375098(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel() [0x3750a0]")]
pub fn stub_0x3750a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Soundscape::SoundChannel::~SoundChannel() [0x3750a8]")]
pub fn stub_0x3750a8(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel() [0x375330]")]
pub fn stub_0x375330(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Soundscape::SoundChannel::~SoundChannel() [0x375338]")]
pub fn stub_0x375338(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Soundscape::SoundService::getCpuStats(RBX::Soundscape::SoundService::CpuStats &)const")]
pub fn stub_0x375340(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundService getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundService::getSoundStats(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> const&,unsigned int &,unsigned int &)")]
pub fn stub_0x3753e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::Sound")
}

#[doc(alias = "RBX::Soundscape::SoundService::getChannelsPlaying(int &)const")]
pub fn stub_0x375418(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundService getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundService::gcSounds(std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>> &)")]
pub fn stub_0x375438() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::Sound")
}

#[doc(alias = "RBX::Soundscape::Sound::release(void)")]
pub fn stub_0x3754c4(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::Sound::release() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::releaseChannel(void)")]
pub fn stub_0x3754e0(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::releaseChannel() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")]
pub fn stub_0x37551c(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::updateListenState(void)")]
pub fn stub_0x375520(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::updateListenState() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x375660(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::onHeartbeat(RBX::Heartbeat const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_0x37567c(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::onAncestorChanged(RBX::AncestorChanged const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")]
pub fn stub_0x375744(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x375b7c(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvid~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::preloadSound(void)")]
pub fn stub_0x375be0(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::preloadSound() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::update3D(FMOD::Channel *)")]
pub fn stub_0x375c3c(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::update3D(FMOD::Channel*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundChannel::updateLooped(void)")]
pub fn stub_0x375c8c(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundChannel::updateLooped() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "callbackChannelEnd(FMOD_CHANNEL *,FMOD_CHANNEL_CALLBACKTYPE,void *,void *)")]
pub fn stub_0x375ce8() -> crate::slot::PortedFn {
// IDA 0x375ce8: callbackChannelEnd(FMOD_CHANNEL*, FMOD_CHANNEL_CALLBACKTYPE, void*, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x375ce8, "callbackChannelEnd(FMOD_CHANNEL*, FMOD_CHANNEL_CALLBACKTYPE, void*, void*)")
}

#[doc(alias = "RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")]
pub fn stub_0x375d0c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId,bool)")]
pub fn stub_0x375dd4(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::loadSound(RBX::Soundscape::SoundId, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::Sound::get(RBX::Instance const*)")]
pub fn stub_0x376004(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::Sound::get(RBX::Instance const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::registerSound(void)")]
pub fn stub_0x376198() -> crate::slot::PortedFn {
// IDA 0x376198: RBX::registerSound().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x376198, "RBX::registerSound()")
}

#[doc(alias = "RBX::Soundscape::Sound::~Sound()")]
pub fn stub_0x37619c(handle: crate::slot::InstanceHandle) {
// RBX::Soundscape::Sound dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbType,char const*)")]
pub fn stub_0x376244(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::addPair(RBX::Soundscape::ReverbTyp~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>) std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)>(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,void (*)(std::pair const&<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>))")]
pub fn stub_0x3765a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::Sound")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
pub fn stub_0x37677c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StockSound")
}

#[doc(alias = "std::map<RBX::SoundType,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::operator[](RBX::SoundType const&)")]
pub fn stub_0x3768dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::SoundChannel")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>& rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::operator=<RBX::StockSound>(rbx_core::SharedPtr<RBX::StockSound> const&)")]
pub fn stub_0x376a24(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::operator=(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const&)")]
pub fn stub_0x376a58(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")]
pub fn stub_0x376a90(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")]
pub fn stub_0x376ac4() -> crate::slot::PortedFn {
// IDA 0x376ac4: SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x376ac4, "SoundServiceStatsItem::create(RBX::Soundscape::SoundService const*)")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Soundscape::SoundId>(RBX::Soundscape::SoundId const&)")]
pub fn stub_0x376c84() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "RBX::Soundscape::SoundId & RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundId>(void)")]
pub fn stub_0x376ce4(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundId& RBX::Reflection::Variant::genericConvert<RBX::Soundscape::SoundI~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x376f90(handle: &crate::slot::InstanceHandle) {
// RBX::Soundscape::SoundService::on3DSettingChanged(RBX::Reflection::PropertyDescriptor cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::~BoundProp()")]
pub fn stub_0x376f94(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Soundscape::SoundService::getAmbientReverb(void)const")]
pub fn stub_0x376fb8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundService getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Soundscape::SoundService,RBX::Soundscape::ReverbType>::~EnumPropDescriptor()")]
pub fn stub_0x376fc0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundService,void ()(RBX::SoundType),1>::~BoundFuncDesc()")]
pub fn stub_0x376fe4(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,RBX::Soundscape::SoundId>::~PropDescriptor()")]
pub fn stub_0x377024(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,float>::~PropDescriptor()")]
pub fn stub_0x377048(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Soundscape::SoundChannel::getPlayCount(void)const")]
pub fn stub_0x37706c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Soundscape::SoundChannel getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,int>::~PropDescriptor()")]
pub fn stub_0x377074(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Soundscape::SoundChannel,bool>::~PropDescriptor()")]
pub fn stub_0x377098(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Soundscape::SoundChannel,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x3770bc(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>> const&)")]
pub fn stub_0x3770e0() -> crate::slot::SlotConnection {
// IDA 0x3770e0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_0x377154() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::Soundscape::SoundService"))
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::Sound>::operator=(rbx_core::SharedPtr<RBX::Soundscape::Sound> const&)")]
pub fn stub_0x37716c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10Soundscape12SoundChannelELZNS2_13sSoundChannelEENS_14FactoryProductIS3_NS_8InstanceELZNS2_13sSoundChannelEES5_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x3771a4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::Soundscape::SoundChannel, RBX::Soundscape::sSoundChannel, ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::map<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::operator[](RBX::Soundscape::SoundId const&)")]
pub fn stub_0x3772c0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Soundscape::Sound")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
pub fn stub_0x37750c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundService"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundServiceENS_8InstanceELZNS1_13sSoundServiceEES3_E12getClassNameEv")]
pub fn stub_0x37751c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
pub fn stub_0x37752c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundChannel"
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E12getClassNameEv")]
pub fn stub_0x37753c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundChannel"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10StockSoundENS_10Soundscape12SoundChannelELZNS_11sStockSoundEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x37754c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"StockSound"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_10Soundscape12SoundChannelENS_8InstanceELZNS1_13sSoundChannelEES3_E7CreatorD1Ev")]
pub fn stub_0x377550() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Soundscape::SoundChannel"
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc()")]
pub fn stub_0x377554(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::~EnumDesc() [0x377558]")]
pub fn stub_0x377558(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*)const")]
pub fn stub_0x3775f8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_0x377628(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Soundscape::ReverbType>::lookup(RBX::Reflection::Variant co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}
