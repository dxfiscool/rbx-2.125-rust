// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 100 not yet in script
// Filter: Script|Lua|Yield|CodeGen (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x48c0c0..0x491750 | script 27055->27155 distinct (filler 0x48c0c0 asc, not-in-script 58490->58390)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::resize(unsigned long,RBX::EThrottle::EThrottleType)")]
pub fn stub_0x48c0c0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::push_back(RBX::EThrottle::EThrottleType const&)")]
pub fn stub_0x48c0f4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::EThrottle::EThrottleType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x48c11c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
pub fn stub_0x48c174(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
pub fn stub_0x48c228(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType> const&)")]
pub fn stub_0x48c280(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,RBX::EThrottle::EThrottleType const&)")]
pub fn stub_0x48c2e8(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_allocate(unsigned long)")]
pub fn stub_0x48c3cc() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::EThrottle::EThrottleType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *>(RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *,RBX::EThrottle::EThrottleType *)")]
pub fn stub_0x48c3e4(handle: &crate::slot::InstanceHandle) {
// RBX::EThrottle::EThrottleType* std::__copy_backward<false, std::random_access_iterator_tag~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::EThrottle::EThrottleType*,std::vector<RBX::EThrottle::EThrottleType,std::allocator<RBX::EThrottle::EThrottleType>>>,unsigned long,RBX::EThrottle::EThrottleType const&)")]
pub fn stub_0x48c420(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::resize(unsigned long,RBX::DebugSettings::ErrorReporting)")]
pub fn stub_0x48c5b0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::push_back(RBX::DebugSettings::ErrorReporting const&)")]
pub fn stub_0x48c5e4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DebugSettings::ErrorReporting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x48c60c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
pub fn stub_0x48c664(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
pub fn stub_0x48c718(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting> const&)")]
pub fn stub_0x48c770(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,RBX::DebugSettings::ErrorReporting const&)")]
pub fn stub_0x48c7d8(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_allocate(unsigned long)")]
pub fn stub_0x48c8bc() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DebugSettings::ErrorReporting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *>(RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *,RBX::DebugSettings::ErrorReporting *)")]
pub fn stub_0x48c8d4(handle: &crate::slot::InstanceHandle) {
// RBX::DebugSettings::ErrorReporting* std::__copy_backward<false, std::random_access_iterato~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DebugSettings::ErrorReporting*,std::vector<RBX::DebugSettings::ErrorReporting,std::allocator<RBX::DebugSettings::ErrorReporting>>>,unsigned long,RBX::DebugSettings::ErrorReporting const&)")]
pub fn stub_0x48c910(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::resize(unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod)")]
pub fn stub_0x48caa0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::push_back(RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
pub fn stub_0x48cad4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::Job::SleepAdjustMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x48cafc(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
pub fn stub_0x48cb54(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
pub fn stub_0x48cc08(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod> const&)")]
pub fn stub_0x48cc60(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
pub fn stub_0x48ccc8(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_allocate(unsigned long)")]
pub fn stub_0x48cdac() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::TaskScheduler::Job::SleepAdjustMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *>(RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *,RBX::TaskScheduler::Job::SleepAdjustMethod *)")]
pub fn stub_0x48cdc4(handle: &crate::slot::InstanceHandle) {
// RBX::TaskScheduler::Job::SleepAdjustMethod* std::__copy_backward<false, std::random_access~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::Job::SleepAdjustMethod*,std::vector<RBX::TaskScheduler::Job::SleepAdjustMethod,std::allocator<RBX::TaskScheduler::Job::SleepAdjustMethod>>>,unsigned long,RBX::TaskScheduler::Job::SleepAdjustMethod const&)")]
pub fn stub_0x48ce00(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::resize(unsigned long,RBX::TaskScheduler::PriorityMethod)")]
pub fn stub_0x48cf90(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::push_back(RBX::TaskScheduler::PriorityMethod const&)")]
pub fn stub_0x48cfc4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::PriorityMethod,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x48cfec(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
pub fn stub_0x48d044(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
pub fn stub_0x48d0f8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod> const&)")]
pub fn stub_0x48d150(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,RBX::TaskScheduler::PriorityMethod const&)")]
pub fn stub_0x48d1b8(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_allocate(unsigned long)")]
pub fn stub_0x48d29c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::TaskScheduler::PriorityMethod * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *>(RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *,RBX::TaskScheduler::PriorityMethod *)")]
pub fn stub_0x48d2b4(handle: &crate::slot::InstanceHandle) {
// RBX::TaskScheduler::PriorityMethod* std::__copy_backward<false, std::random_access_iterato~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::PriorityMethod*,std::vector<RBX::TaskScheduler::PriorityMethod,std::allocator<RBX::TaskScheduler::PriorityMethod>>>,unsigned long,RBX::TaskScheduler::PriorityMethod const&)")]
pub fn stub_0x48d2f0(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::resize(unsigned long,RBX::TaskScheduler::ThreadPoolConfig)")]
pub fn stub_0x48d480(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::map<RBX::Name const*,RBX::TaskScheduler::ThreadPoolConfig,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x48d4b4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
pub fn stub_0x48d50c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
pub fn stub_0x48d5c0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig> const&)")]
pub fn stub_0x48d618(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,unsigned long,RBX::TaskScheduler::ThreadPoolConfig const&)")]
pub fn stub_0x48d680(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::_Vector_base<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_allocate(unsigned long)")]
pub fn stub_0x48d810() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::TaskScheduler::ThreadPoolConfig * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *>(RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *,RBX::TaskScheduler::ThreadPoolConfig *)")]
pub fn stub_0x48d828(handle: &crate::slot::InstanceHandle) {
// RBX::TaskScheduler::ThreadPoolConfig* std::__copy_backward<false, std::random_access_itera~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::push_back(RBX::TaskScheduler::ThreadPoolConfig const&)")]
pub fn stub_0x48d864(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::TaskScheduler::ThreadPoolConfig*,std::vector<RBX::TaskScheduler::ThreadPoolConfig,std::allocator<RBX::TaskScheduler::ThreadPoolConfig>>>,RBX::TaskScheduler::ThreadPoolConfig const&)")]
pub fn stub_0x48d88c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x48dc2c() -> crate::slot::PortedFn {
// IDA 0x48dc2c: DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48dc2c, "DummyJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")
}

#[doc(alias = "DummyJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x48dc34() -> crate::slot::PortedFn {
// IDA 0x48dc34: DummyJob::error(RBX::TaskScheduler::Job::Stats const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48dc34, "DummyJob::error(RBX::TaskScheduler::Job::Stats const&)")
}

#[doc(alias = "DummyJob::step(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_0x48dc58() -> crate::slot::PortedFn {
// IDA 0x48dc58: DummyJob::step(RBX::TaskScheduler::Job::Stats const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48dc58, "DummyJob::step(RBX::TaskScheduler::Job::Stats const&)")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate(void)const")]
pub fn stub_0x48dc60() -> crate::slot::PortedFn {
// IDA 0x48dc60: RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate() const.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x48dc60, "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)2>::rate() const")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Time::SampleMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Time::SampleMethod>> *)")]
pub fn stub_0x48dcc0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::EThrottle::EThrottleType>> *)")]
pub fn stub_0x48dce8(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::DebugSettings::ErrorReporting>> *)")]
pub fn stub_0x48dd10(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::Job::SleepAdjustMethod>> *)")]
pub fn stub_0x48dd38(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::PriorityMethod>> *)")]
pub fn stub_0x48dd60(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TaskScheduler::ThreadPoolConfig>> *)")]
pub fn stub_0x48dd88(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::Decal::setTexture(RBX::TextureId)")]
pub fn stub_0x48f7f4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Decal setter.
cell.set(value)
}

#[doc(alias = "RBX::Decal::setSpecular(float)")]
pub fn stub_0x48f82c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Decal setter.
cell.set(value)
}

#[doc(alias = "RBX::Decal::setShiny(float)")]
pub fn stub_0x48f860(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Decal setter.
cell.set(value)
}

#[doc(alias = "RBX::Decal::setTransparency(float)")]
pub fn stub_0x48f894(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Decal setter.
cell.set(value)
}

#[doc(alias = "RBX::Decal::Decal(void)")]
pub fn stub_0x48f8bc() -> crate::slot::InstanceHandle {
// RBX::Decal ctor.
crate::slot::InstanceHandle::new("RBX::Decal")
}

#[doc(alias = "RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&,RBX::TextureId&)")]
pub fn stub_0x48fb04(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::TextureId>::convertToValue(std::string const&, RBX::TextureId&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextureId>(void)")]
pub fn stub_0x48fc28(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::TextureId>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x48fc2c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::readValue(RBX::Reflection::Descr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x48fe14(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::writeValue(RBX::Reflection::Desc~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::convert<RBX::TextureId>(void)")]
pub fn stub_0x48ffbc(handle: &crate::slot::InstanceHandle) {
// RBX::TextureId& RBX::Reflection::Variant::convert<RBX::TextureId>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x4901a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::hasStringValue(void)const")]
pub fn stub_0x490204(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x490208(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::TextureId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x490324(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Texture::setStudsPerTileU(float)")]
pub fn stub_0x49047c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Texture setter.
cell.set(value)
}

#[doc(alias = "RBX::Texture::setStudsPerTileV(float)")]
pub fn stub_0x4904b0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Texture setter.
cell.set(value)
}

#[doc(alias = "RBX::Texture::Texture(void)")]
pub fn stub_0x4904e4() -> crate::slot::InstanceHandle {
// RBX::Texture ctor.
crate::slot::InstanceHandle::new("RBX::Texture")
}

#[doc(alias = "RBX::Decal::getTexture(void)const")]
pub fn stub_0x49076c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Decal getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::~PropDescriptor()")]
pub fn stub_0x490770(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Decal::getSpecular(void)const")]
pub fn stub_0x490794(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Decal getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::~PropDescriptor()")]
pub fn stub_0x49079c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Decal::getShiny(void)const")]
pub fn stub_0x4907c0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Decal getter.
cell.get()
}

#[doc(alias = "RBX::Decal::getTransparency(void)const")]
pub fn stub_0x4907c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Decal getter.
cell.get()
}

#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")]
pub fn stub_0x4907d0(handle: &crate::slot::InstanceHandle) {
// RBX::TextureId& RBX::Reflection::Variant::genericConvert<RBX::TextureId>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Texture::getStudsPerTileU(void)const")]
pub fn stub_0x490a7c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Texture getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::~PropDescriptor()")]
pub fn stub_0x490a84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Texture::getStudsPerTileV(void)const")]
pub fn stub_0x490aa8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Texture getter.
cell.get()
}

#[doc(alias = "RBX::Decal::~Decal()")]
pub fn stub_0x490ab8(handle: crate::slot::InstanceHandle) {
// RBX::Decal dtor.
drop(handle);
}

#[doc(alias = "RBX::Decal::~Decal() [0x490af8]")]
pub fn stub_0x490af8(handle: crate::slot::InstanceHandle) {
// RBX::Decal dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal()")]
pub fn stub_0x490be4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal() [0x490c28]")]
pub fn stub_0x490c28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal() [0x490d14]")]
pub fn stub_0x490d14(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Decal::~Decal() [0x490d58]")]
pub fn stub_0x490d58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Texture::~Texture()")]
pub fn stub_0x490e34(handle: crate::slot::InstanceHandle) {
// RBX::Texture dtor.
drop(handle);
}

#[doc(alias = "RBX::Texture::~Texture() [0x490e74]")]
pub fn stub_0x490e74(handle: crate::slot::InstanceHandle) {
// RBX::Texture dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture()")]
pub fn stub_0x490f60(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture() [0x490fa4]")]
pub fn stub_0x490fa4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture() [0x491090]")]
pub fn stub_0x491090(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Texture::~Texture() [0x4910d4]")]
pub fn stub_0x4910d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Texture> RBX::Creatable<RBX::Instance>::create<RBX::Texture>(void)")]
pub fn stub_0x491750() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Texture")
}
