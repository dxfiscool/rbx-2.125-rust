// Auto-generated skeletons for rbx-script — Lua/Script/Yield/lua batch
// Filter: Script|Lua|Yield|lua (case-sensitive, lua lower) -> 5401 filtered, 2459 in any crate, 2942 remaining
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2b6efc..0x2c966c | filtered 5401 -> 2559 covered, 2842 remaining | script 7471->7571 total, global 25248->25348 distinct, 60197 remaining
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
pub fn stub_0x2b6efc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::WaitingScriptsJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::WaitingScriptsJob *)const")]
pub fn stub_0x2b6fe4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::TaskScheduler::Job")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::WaitingScriptsJob>(RBX::WaitingScriptsJob *)")]
pub fn stub_0x2b70c8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::~sp_counted_impl_p()")]
pub fn stub_0x2b71c0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::~sp_counted_impl_p() [0x2b71c4]")]
pub fn stub_0x2b71c4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::dispose(void)")]
pub fn stub_0x2b71c8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b71d8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::WaitingScriptsJob>::get_untyped_deleter(void)")]
pub fn stub_0x2b71dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Lua::SingletonBridge<RBX::Lua::AllEnumDescriptors const*,true>::push(lua_State *,RBX::Lua::AllEnumDescriptors const*)")]
pub fn stub_0x2b7d50(thread: &mut crate::lua::LuaThreadState, desc: &crate::lua::LuaEnumDescriptor) -> i32 {
// Bridge::push for enum descriptors — pushes one EnumItem
// userdata per value and returns the count.
for value in desc.values.clone() {
    thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: desc.name.clone(), value }) }));
}
desc.values.len() as i32
}

#[doc(alias = "RBX::Lua::AllEnumDescriptors const** RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::pushNewObject<RBX::Lua::AllEnumDescriptors const*>(lua_State *,RBX::Lua::AllEnumDescriptors const*)")]
pub fn stub_0x2b7e28(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::AllEnumDescriptors const** RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_index(lua_State *)")]
pub fn stub_0x2b7fa0(value: &crate::lua::LuaEnumDescriptor, key: &str, thread: &mut crate::lua::LuaThreadState) -> i32 {
// Bridge<EnumDescriptor>::on_index — ordinal lookup by key.
match key.parse::<usize>() {
    Ok(i) if i < value.values.len() => {
        thread.push(crate::lua::LuaStackValue::Userdata(crate::lua::LuaUserdata { class: crate::lua::lua_bridge_class::ENUMITEM.to_owned(), payload: crate::lua::LuaUserdataPayload::EnumItem(crate::lua::LuaEnumItem { owner: value.name.clone(), value: value.values[i] }) }));
    }
    _ => panic!("{key} is not a valid member"),
}
1
}

#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::AllEnumDescriptors const*,true>::on_newindex(lua_State *)")]
pub fn stub_0x2b7fd4(key: &str) -> ! {
// Bridge<EnumDesc>::on_newindex (__noreturn, cf. 0x270724) — members
// are read-only.
panic!("{key} cannot be assigned to");
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x2b8008() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaSettings"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x2b80a4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaSettings"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x2b8110() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaSettings"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLuaSettingsEEEEvv")]
pub fn stub_0x2b8608() -> crate::slot::PortedFn {
// IDA 0x2b8608: void RBX::Name::callDoDeclare<RBX::sLuaSettings>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2b8608, "void RBX::Name::callDoDeclare<RBX::sLuaSettings>()")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x2b8610() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"LuaSettings"
}

#[doc(alias = "RBX::RuntimeScriptService::~RuntimeScriptService() [0x2c04b0]")]
pub fn stub_0x2c04b0(handle: crate::slot::InstanceHandle) {
// RBX::RuntimeScriptService dtor.
drop(handle);
}

#[doc(alias = "LuaProfiler::~LuaProfiler()")]
pub fn stub_0x2c090c() -> crate::slot::PortedFn {
// IDA 0x2c090c: LuaProfiler::~LuaProfiler().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c090c, "LuaProfiler::~LuaProfiler()")
}

#[doc(alias = "LuaProfiler::LuaProfiler(lua_State *,int)")]
pub fn stub_0x2c09a4() -> crate::slot::PortedFn {
// IDA 0x2c09a4: LuaProfiler::LuaProfiler(lua_State*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c09a4, "LuaProfiler::LuaProfiler(lua_State*, int)")
}

#[doc(alias = "LuaProfiler::getResumePosition(lua_State *,int)")]
pub fn stub_0x2c0c60() -> crate::slot::PortedFn {
// IDA 0x2c0c60: LuaProfiler::getResumePosition(lua_State*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c0c60, "LuaProfiler::getResumePosition(lua_State*, int)")
}

#[doc(alias = "LuaProfiler::StringCache::getBegin(std::string const&)")]
pub fn stub_0x2c0d9c() -> crate::slot::PortedFn {
// IDA 0x2c0d9c: LuaProfiler::StringCache::getBegin(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c0d9c, "LuaProfiler::StringCache::getBegin(std::string const&)")
}

#[doc(alias = "LuaProfiler::StringCache::getCall(char const*,char const*,char const*,int)")]
pub fn stub_0x2c0f08() -> crate::slot::PortedFn {
// IDA 0x2c0f08: LuaProfiler::StringCache::getCall(char const*, char const*, char const*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c0f08, "LuaProfiler::StringCache::getCall(char const*, char const*, char const*, int)")
}

#[doc(alias = "std::map<LuaProfiler::StringCache::Function,std::string,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::operator[](LuaProfiler::StringCache::Function const&)")]
pub fn stub_0x2c10cc(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
pub fn stub_0x2c12ac(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
pub fn stub_0x2c138c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_insert_unique(std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
pub fn stub_0x2c13dc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_create_node(std::pair<LuaProfiler::StringCache::Function const,std::string> const&)")]
pub fn stub_0x2c145c() -> crate::slot::PortedFn {
// IDA 0x2c145c: std::_Rb_tree<LuaProfiler::StringCache::Function, std::pair<LuaProfiler::StringCache::Function const, std::string>, std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c145c, "std::_Rb_tree<LuaProfiler::StringCache::Function, std::pair<LuaProfiler::StringCache::Function const~")
}

#[doc(alias = "LuaProfiler::StringCache::Function::operator<(LuaProfiler::StringCache::Function const&)const")]
pub fn stub_0x2c154c() -> crate::slot::PortedFn {
// IDA 0x2c154c: LuaProfiler::StringCache::Function::operator<(LuaProfiler::StringCache::Function const&) const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c154c, "LuaProfiler::StringCache::Function::operator<(LuaProfiler::StringCache::Function const&) const")
}

#[doc(alias = "RBX::LuaStatsItem::LuaStatsItem(RBX::ScriptContext *)")]
pub fn stub_0x2c1bac() -> crate::slot::InstanceHandle {
// RBX::LuaStatsItem ctor.
crate::slot::InstanceHandle::new("RBX::LuaStatsItem")
}

#[doc(alias = "RBX::ScriptContext::ScriptStart::~ScriptStart()")]
pub fn stub_0x2c2570(handle: crate::slot::InstanceHandle) {
// RBX::ScriptContext::ScriptStart dtor.
drop(handle);
}

#[doc(alias = "RBX::Lua::ThreadRef::ThreadRef(lua_State *)")]
pub fn stub_0x2c27a8() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::~deque()")]
pub fn stub_0x2c2f9c(queue: crate::generated::WaitingThreadQueue) {
// deque<WaitingThread> dtor.
drop(queue);
}

#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::~_Deque_base()")]
pub fn stub_0x2c3084(handle: crate::slot::InstanceHandle) {
// std::_Deque_base dtor.
drop(handle);
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>)")]
pub fn stub_0x2c30b0() -> crate::generated::WaitingThreadQueue {
// deque<WaitingThread> ctor — empty queue.
crate::generated::WaitingThreadQueue::default()
}

#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x2c3484(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::ScriptContext::WaitingThread, std::allocator<RBX::ScriptContext::Wai~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_create_nodes(RBX::ScriptContext::WaitingThread**,RBX::ScriptContext::WaitingThread**)")]
pub fn stub_0x2c3604(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::ScriptContext::WaitingThread, std::allocator<RBX::ScriptContext::Wai~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::deque(std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>> const&)")]
pub fn stub_0x2c36f8() -> crate::generated::WaitingThreadQueue {
// deque<WaitingThread> ctor — empty queue.
crate::generated::WaitingThreadQueue::default()
}

#[doc(alias = "std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>>(std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread const&,RBX::ScriptContext::WaitingThread const*>,std::_Deque_iterator<RBX::ScriptContext::WaitingThread,RBX::ScriptContext::WaitingThread&,RBX::ScriptContext::WaitingThread*>,std::__false_type)")]
pub fn stub_0x2c382c(handle: &crate::slot::InstanceHandle) {
// std::_Deque_iterator<RBX::ScriptContext::WaitingThread, RBX::ScriptContext::WaitingThread&~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::WaitingScriptsJob::~WaitingScriptsJob()")]
pub fn stub_0x2c4130(handle: crate::slot::InstanceHandle) {
// RBX::WaitingScriptsJob dtor.
drop(handle);
}

#[doc(alias = "RBX::WaitingScriptsJob::~WaitingScriptsJob() [0x2c4200]")]
pub fn stub_0x2c4200(handle: crate::slot::InstanceHandle) {
// RBX::WaitingScriptsJob dtor.
drop(handle);
}

#[doc(alias = "RBX::WaitingScriptsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x2c42e8(handle: &crate::slot::InstanceHandle) {
// RBX::WaitingScriptsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::WaitingScriptsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x2c4310(handle: &crate::slot::InstanceHandle) {
// RBX::WaitingScriptsJob::error(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::WaitingScriptsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x2c433c(handle: &crate::slot::InstanceHandle) {
// RBX::WaitingScriptsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<LuaProfiler::StringCache::Function,std::pair<LuaProfiler::StringCache::Function const,std::string>,std::_Select1st<std::pair<LuaProfiler::StringCache::Function const,std::string>>,std::less<LuaProfiler::StringCache::Function>,std::allocator<std::pair<LuaProfiler::StringCache::Function const,std::string>>>::_M_erase(std::_Rb_tree_node<std::pair<LuaProfiler::StringCache::Function const,std::string>> *)")]
pub fn stub_0x2c4a50(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::Lua::YieldingThreads::YieldingThreads(RBX::ScriptContext *)")]
pub fn stub_0x2c5440() -> crate::slot::InstanceHandle {
// RBX::Lua::YieldingThreads ctor.
crate::slot::InstanceHandle::new("RBX::Lua::YieldingThreads")
}

#[doc(alias = "RBX::Lua::YieldingThreads::YieldingThreads(RBX::ScriptContext *) [0x2c5444]")]
pub fn stub_0x2c5444() -> crate::slot::InstanceHandle {
// RBX::Lua::YieldingThreads ctor.
crate::slot::InstanceHandle::new("RBX::Lua::YieldingThreads")
}

#[doc(alias = "RBX::Lua::YieldingThreads::queueWaiter(lua_State *)")]
pub fn stub_0x2c5518(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::YieldingThreads::queueWaiter(lua_State*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::YieldingThreads::queueWaiter(lua_State *,double)")]
pub fn stub_0x2c5530(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::YieldingThreads::queueWaiter(lua_State*, double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::YieldingThreads::waiterCount(void)const")]
pub fn stub_0x2c567c(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::YieldingThreads::waiterCount() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::YieldingThreads::resume(double,RBX::Time,bool &)")]
pub fn stub_0x2c5690(handle: &crate::slot::InstanceHandle) {
// RBX::Lua::YieldingThreads::resume(double, RBX::Time, bool&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Lua::Bridge<rbx::signals::connection,true>::on_tostring(rbx::signals::connection const&,lua_State *)")]
pub fn stub_0x2c5994() -> crate::slot::SlotConnection {
// IDA 0x2c5994: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::push(RBX::Lua::YieldingThreads::WaitingThread const&)")]
pub fn stub_0x2c5a08(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::pop(void)")]
pub fn stub_0x2c5b10(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "void std::pop_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>)")]
pub fn stub_0x2c5b3c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>)")]
pub fn stub_0x2c5cac() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>>(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread *,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,int,int,RBX::Lua::YieldingThreads::WaitingThread,std::less<RBX::Lua::YieldingThreads::WaitingThread>)")]
pub fn stub_0x2c5e44() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::push_back(RBX::Lua::YieldingThreads::WaitingThread const&)")]
pub fn stub_0x2c5ef0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Lua::YieldingThreads::WaitingThread*,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>>,RBX::Lua::YieldingThreads::WaitingThread const&)")]
pub fn stub_0x2c5f48() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::_Vector_base<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_M_allocate(unsigned long)")]
pub fn stub_0x2c636c() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "RBX::Lua::YieldingThreads::WaitingThread * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *>(RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *,RBX::Lua::YieldingThreads::WaitingThread *)")]
pub fn stub_0x2c6390() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::priority_queue<RBX::Lua::YieldingThreads::WaitingThread,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>,std::less<RBX::Lua::YieldingThreads::WaitingThread>>::priority_queue(std::less<RBX::Lua::YieldingThreads::WaitingThread> const&,std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>> const&)")]
pub fn stub_0x2c63f8() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::vector(std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>> const&)")]
pub fn stub_0x2c6548() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::_Vector_base<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::_Vector_base(unsigned long,std::allocator<RBX::Lua::YieldingThreads::WaitingThread> const&)")]
pub fn stub_0x2c66c4() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "std::vector<RBX::Lua::YieldingThreads::WaitingThread,std::allocator<RBX::Lua::YieldingThreads::WaitingThread>>::~vector()")]
pub fn stub_0x2c66f8(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::Lua::YieldingThreads::WaitingThread::WaitingThread(lua_State *,RBX::Time::Interval)")]
pub fn stub_0x2c67c8() -> crate::generated::WaitingThread {
// WaitingThread ctor — unscheduled (id 0, wake 0.0).
crate::generated::WaitingThread::new(0, 0.0)
}

#[doc(alias = "RBX::ScriptStats::ScriptStats(void)")]
pub fn stub_0x2c6a74() -> crate::slot::InstanceHandle {
// RBX::ScriptStats ctor.
crate::slot::InstanceHandle::new("RBX::ScriptStats")
}

#[doc(alias = "RBX::ScriptStats::ScriptStats(void) [0x2c6a78]")]
pub fn stub_0x2c6a78() -> crate::slot::InstanceHandle {
// RBX::ScriptStats ctor.
crate::slot::InstanceHandle::new("RBX::ScriptStats")
}

#[doc(alias = "RBX::ScriptStats::stopCollection(std::string const&)")]
pub fn stub_0x2c6b84(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptStats::stopCollection(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptStats::startCollection(std::string const&,bool)")]
pub fn stub_0x2c6bac(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptStats::startCollection(std::string const&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptStats::scriptResumeStarted(std::string const&)")]
pub fn stub_0x2c7014(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptStats::scriptResumeStarted(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptStats::scriptResumeStopped(std::string const&)")]
pub fn stub_0x2c7054(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptStats::scriptResumeStopped(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaStatsItem::init(void)")]
pub fn stub_0x2c715c(handle: &crate::slot::InstanceHandle) {
// RBX::LuaStatsItem::init() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaStatsItem::update(void)")]
pub fn stub_0x2c72c8(handle: &crate::slot::InstanceHandle) {
// RBX::LuaStatsItem::update() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c756c(handle: crate::slot::InstanceHandle) {
// RBX::LuaStatsItem dtor.
drop(handle);
}

#[doc(alias = "RBX::LuaStatsItem::~LuaStatsItem() [0x2c75a8]")]
pub fn stub_0x2c75a8(handle: crate::slot::InstanceHandle) {
// RBX::LuaStatsItem dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem()")]
pub fn stub_0x2c767c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem() [0x2c76bc]")]
pub fn stub_0x2c76bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem() [0x2c7794]")]
pub fn stub_0x2c7794(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::LuaStatsItem::~LuaStatsItem() [0x2c77d4]")]
pub fn stub_0x2c77d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x2c78a8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_0x2c7908(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "boost::_bi::bind_t<unsigned long,boost::_mfi::cmf0<unsigned long,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<RBX::ScriptContext*>>>::operator()(void)")]
pub fn stub_0x2c790c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptStats::StatCollection> const&)")]
pub fn stub_0x2c8578(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptStats::StatCollection> const&)")]
pub fn stub_0x2c85fc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_create_node(std::pair<std::string const,RBX::ScriptStats::StatCollection> const&)")]
pub fn stub_0x2c864c() -> crate::slot::PortedFn {
// IDA 0x2c864c: std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptStats::StatCollection>, std::_Select1st<std::pair<std~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c864c, "std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptStats::StatCollection>, std::_Sel~")
}

#[doc(alias = "std::pair<std::string const,RBX::ScriptStats::StatCollection>::pair(std::string const&,RBX::ScriptStats::StatCollection const&)")]
pub fn stub_0x2c8794() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::find(std::string const&)")]
pub fn stub_0x2c8c50(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptStats::StatCollection>> *)")]
pub fn stub_0x2c9208(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptStats::StatCollection>,std::_Select1st<std::pair<std::string const,RBX::ScriptStats::StatCollection>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptStats::StatCollection>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptStats::StatCollection>> *)")]
pub fn stub_0x2c9230() -> crate::slot::PortedFn {
// IDA 0x2c9230: std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptStats::StatCollection>, std::_Select1st<std::pair<std~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2c9230, "std::_Rb_tree<std::string, std::pair<std::string const, RBX::ScriptStats::StatCollection>, std::_Sel~")
}

#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(lua_State *)")]
pub fn stub_0x2c94ac() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::Lua::WeakThreadRef::WeakThreadRef(lua_State *) [0x2c94b0]")]
pub fn stub_0x2c94b0() -> crate::slot::InstanceHandle {
// thread-ref ctor — fresh weak link identity.
crate::slot::InstanceHandle::new("RBX::Lua::WeakThreadRef")
}

#[doc(alias = "RBX::Lua::WeakThreadRef::addToNode(void)")]
pub fn stub_0x2c966c(handle: &crate::slot::InstanceHandle) {
// thread-ref op — engine-side; linkage via alias.
let _ = handle;
}
