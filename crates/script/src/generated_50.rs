// Auto-generated skeletons for rbx-script — filler EA-sorted ascending after 0x4e9740 (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4e9760..0x4efbb0 | existing ~5171 -> ~5271 total (union; filler 0x4e9760 ascending, global remaining 63065 -> 62965)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x4e9760() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::VelocityMotor *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x4e9778() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12MotorFeatureELZNS_13sMotorFeatureEENS_14FactoryProductIS2_NS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e977c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12MotorFeatureELZNS_13sMotorFeatureEENS_14FactoryProductIS2_NS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9780(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12MotorFeatureELZNS_13sMotorFeatureEENS_14FactoryProductIS2_NS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9820(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12MotorFeatureELZNS_13sMotorFeatureEENS_14FactoryProductIS2_NS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9828(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12MotorFeatureELZNS_13sMotorFeatureEENS_14FactoryProductIS2_NS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e98cc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12MotorFeatureELZNS_13sMotorFeatureEENS_14FactoryProductIS2_NS_7FeatureELZNS_13sMotorFeatureEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e98d4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4HoleELZNS_5sHoleEENS_14FactoryProductIS2_NS_7FeatureELZNS_5sHoleEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9978(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_4HoleELZNS_5sHoleEENS_14FactoryProductIS2_NS_7FeatureELZNS_5sHoleEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e997c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4HoleELZNS_5sHoleEENS_14FactoryProductIS2_NS_7FeatureELZNS_5sHoleEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9a1c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_4HoleELZNS_5sHoleEENS_14FactoryProductIS2_NS_7FeatureELZNS_5sHoleEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9a24(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4HoleELZNS_5sHoleEENS_14FactoryProductIS2_NS_7FeatureELZNS_5sHoleEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9ac8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_4HoleELZNS_5sHoleEENS_14FactoryProductIS2_NS_7FeatureELZNS_5sHoleEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9ad0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7FeatureELZNS_8sFeatureEENS_17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9b74(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7FeatureELZNS_8sFeatureEENS_17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9b78(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7FeatureELZNS_8sFeatureEENS_17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9c18(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7FeatureELZNS_8sFeatureEENS_17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9c20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7FeatureELZNS_8sFeatureEENS_17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4e9cc4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7FeatureELZNS_8sFeatureEENS_17NonFactoryProductINS_8InstanceELZNS_8sFeatureEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4e9ccc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::resize(unsigned long,RBX::Feature::InOut)")]
pub fn stub_0x4ebee0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::push_back(RBX::Feature::InOut const&)")]
pub fn stub_0x4ebf14(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::InOut,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x4ebf3c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
pub fn stub_0x4ebf94(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
pub fn stub_0x4ec048(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::InOut>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::InOut>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::InOut>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::InOut> const&)")]
pub fn stub_0x4ec0a0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,RBX::Feature::InOut const&)")]
pub fn stub_0x4ec108(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_allocate(unsigned long)")]
pub fn stub_0x4ec1ec() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Feature::InOut * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::InOut *,RBX::Feature::InOut *>(RBX::Feature::InOut *,RBX::Feature::InOut *,RBX::Feature::InOut *)")]
pub fn stub_0x4ec204(handle: &crate::slot::InstanceHandle) {
// RBX::Feature::InOut* std::__copy_backward<false, std::random_access_iterator_tag>::__copy_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::InOut*,std::vector<RBX::Feature::InOut,std::allocator<RBX::Feature::InOut>>>,unsigned long,RBX::Feature::InOut const&)")]
pub fn stub_0x4ec240(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::resize(unsigned long,RBX::Feature::LeftRight)")]
pub fn stub_0x4ec3d0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::push_back(RBX::Feature::LeftRight const&)")]
pub fn stub_0x4ec404(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::LeftRight,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x4ec42c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
pub fn stub_0x4ec484(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
pub fn stub_0x4ec538(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::LeftRight>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::LeftRight>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::LeftRight> const&)")]
pub fn stub_0x4ec590(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,RBX::Feature::LeftRight const&)")]
pub fn stub_0x4ec5f8(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_allocate(unsigned long)")]
pub fn stub_0x4ec6dc() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Feature::LeftRight * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::LeftRight *,RBX::Feature::LeftRight *>(RBX::Feature::LeftRight *,RBX::Feature::LeftRight *,RBX::Feature::LeftRight *)")]
pub fn stub_0x4ec6f4(handle: &crate::slot::InstanceHandle) {
// RBX::Feature::LeftRight* std::__copy_backward<false, std::random_access_iterator_tag>::__c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::LeftRight*,std::vector<RBX::Feature::LeftRight,std::allocator<RBX::Feature::LeftRight>>>,unsigned long,RBX::Feature::LeftRight const&)")]
pub fn stub_0x4ec730(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::resize(unsigned long,RBX::Feature::TopBottom)")]
pub fn stub_0x4ec8c0(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::push_back(RBX::Feature::TopBottom const&)")]
pub fn stub_0x4ec8f4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Feature::TopBottom,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x4ec91c(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
pub fn stub_0x4ec974(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
pub fn stub_0x4eca28(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Feature::TopBottom>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Feature::TopBottom>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Feature::TopBottom> const&)")]
pub fn stub_0x4eca80(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,RBX::Feature::TopBottom const&)")]
pub fn stub_0x4ecae8(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_allocate(unsigned long)")]
pub fn stub_0x4ecbcc() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Feature::TopBottom * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Feature::TopBottom *,RBX::Feature::TopBottom *>(RBX::Feature::TopBottom *,RBX::Feature::TopBottom *,RBX::Feature::TopBottom *)")]
pub fn stub_0x4ecbe4(handle: &crate::slot::InstanceHandle) {
// RBX::Feature::TopBottom* std::__copy_backward<false, std::random_access_iterator_tag>::__c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Feature::TopBottom*,std::vector<RBX::Feature::TopBottom,std::allocator<RBX::Feature::TopBottom>>>,unsigned long,RBX::Feature::TopBottom const&)")]
pub fn stub_0x4ecc20(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "global constructor keyed to_a_192")]
pub fn stub_0x4ed6e0() -> crate::slot::PortedFn {
// IDA 0x4ed6e0: __GLOBAL__I_a_192.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4ed6e0, "__GLOBAL__I_a_192")
}

#[doc(alias = "RBX::FileMesh::FileMesh(void)")]
pub fn stub_0x4edc88() -> crate::slot::InstanceHandle {
// RBX::FileMesh ctor.
crate::slot::InstanceHandle::new("RBX::FileMesh")
}

#[doc(alias = "RBX::FileMesh::FileMesh(void) [0x4edc8c]")]
pub fn stub_0x4edc8c() -> crate::slot::InstanceHandle {
// RBX::FileMesh ctor.
crate::slot::InstanceHandle::new("RBX::FileMesh")
}

#[doc(alias = "RBX::FileMesh::setMeshId(RBX::MeshId const&)")]
pub fn stub_0x4ede2c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::FileMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::FileMesh::setTextureId(RBX::TextureId const&)")]
pub fn stub_0x4ede6c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::FileMesh setter.
cell.set(value)
}

#[doc(alias = "RBX::FileMesh::getMeshId(void)const")]
pub fn stub_0x4edeac(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FileMesh getter.
cell.get()
}

#[doc(alias = "RBX::FileMesh::getTextureId(void)const")]
pub fn stub_0x4eded4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FileMesh getter.
cell.get()
}

#[doc(alias = "RBX::MeshId const& rbx::any_cast<RBX::MeshId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x4eeb18(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "global constructor keyed to_a_193")]
pub fn stub_0x4eecb8() -> crate::slot::PortedFn {
// IDA 0x4eecb8: __GLOBAL__I_a_193.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4eecb8, "__GLOBAL__I_a_193")
}

#[doc(alias = "RBX::FilterInvisibleNonColliding::FilterInvisibleNonColliding(void)")]
pub fn stub_0x4eef38() -> crate::slot::InstanceHandle {
// RBX::FilterInvisibleNonColliding ctor.
crate::slot::InstanceHandle::new("RBX::FilterInvisibleNonColliding")
}

#[doc(alias = "RBX::FilterInvisibleNonColliding::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4eef48(handle: &crate::slot::InstanceHandle) {
// RBX::FilterInvisibleNonColliding::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::PartByLocalCharacter::PartByLocalCharacter(RBX::Instance *)")]
pub fn stub_0x4eef84() -> crate::slot::InstanceHandle {
// RBX::PartByLocalCharacter ctor.
crate::slot::InstanceHandle::new("RBX::PartByLocalCharacter")
}

#[doc(alias = "RBX::PartByLocalCharacter::PartByLocalCharacter(RBX::Instance *) [0x4eef88]")]
pub fn stub_0x4eef88() -> crate::slot::InstanceHandle {
// RBX::PartByLocalCharacter ctor.
crate::slot::InstanceHandle::new("RBX::PartByLocalCharacter")
}

#[doc(alias = "RBX::PartByLocalCharacter::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4ef0f4(handle: &crate::slot::InstanceHandle) {
// RBX::PartByLocalCharacter::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::UnlockedPartByLocalCharacter::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4ef164(handle: &crate::slot::InstanceHandle) {
// RBX::UnlockedPartByLocalCharacter::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilterDescendents::FilterDescendents(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x4ef18c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::FilterDescendents::FilterDescendents(rbx_core::SharedPtr<RBX::Instance>) [0x4ef190]")]
pub fn stub_0x4ef190() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::FilterDescendents::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4ef260(handle: &crate::slot::InstanceHandle) {
// RBX::FilterDescendents::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilterDescendentsList::FilterDescendentsList(std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const*)")]
pub fn stub_0x4ef28c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::FilterDescendentsList::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4ef2a0(handle: &crate::slot::InstanceHandle) {
// RBX::FilterDescendentsList::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilterCharacterOcclusion::FilterCharacterOcclusion(float)")]
pub fn stub_0x4ef2e0() -> crate::slot::InstanceHandle {
// RBX::FilterCharacterOcclusion ctor.
crate::slot::InstanceHandle::new("RBX::FilterCharacterOcclusion")
}

#[doc(alias = "RBX::FilterCharacterOcclusion::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4ef2f4(handle: &crate::slot::InstanceHandle) {
// RBX::FilterCharacterOcclusion::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::FilterHumanoidParts::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4ef388(handle: &crate::slot::InstanceHandle) {
// RBX::FilterHumanoidParts::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::MergedFilter::MergedFilter(RBX::HitTestFilter const*,RBX::HitTestFilter const*)")]
pub fn stub_0x4ef3a4() -> crate::slot::InstanceHandle {
// RBX::MergedFilter ctor.
crate::slot::InstanceHandle::new("RBX::MergedFilter")
}

#[doc(alias = "RBX::MergedFilter::filterResult(RBX::Primitive const*)const")]
pub fn stub_0x4ef3b8(handle: &crate::slot::InstanceHandle) {
// RBX::MergedFilter::filterResult(RBX::Primitive const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ModelInstance>::operator=(rbx_core::SharedPtr<RBX::ModelInstance> const&)")]
pub fn stub_0x4ef3ec(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding()")]
pub fn stub_0x4ef424(handle: crate::slot::InstanceHandle) {
// RBX::FilterInvisibleNonColliding dtor.
drop(handle);
}

#[doc(alias = "RBX::FilterInvisibleNonColliding::~FilterInvisibleNonColliding() [0x4ef428]")]
pub fn stub_0x4ef428(handle: crate::slot::InstanceHandle) {
// RBX::FilterInvisibleNonColliding dtor.
drop(handle);
}

#[doc(alias = "RBX::FilterDescendentsList::~FilterDescendentsList()")]
pub fn stub_0x4ef42c(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::FilterDescendentsList::~FilterDescendentsList() [0x4ef430]")]
pub fn stub_0x4ef430(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion()")]
pub fn stub_0x4ef434(handle: crate::slot::InstanceHandle) {
// RBX::FilterCharacterOcclusion dtor.
drop(handle);
}

#[doc(alias = "RBX::FilterCharacterOcclusion::~FilterCharacterOcclusion() [0x4ef438]")]
pub fn stub_0x4ef438(handle: crate::slot::InstanceHandle) {
// RBX::FilterCharacterOcclusion dtor.
drop(handle);
}

#[doc(alias = "RBX::MergedFilter::~MergedFilter()")]
pub fn stub_0x4ef43c(handle: crate::slot::InstanceHandle) {
// RBX::MergedFilter dtor.
drop(handle);
}

#[doc(alias = "RBX::MergedFilter::~MergedFilter() [0x4ef440]")]
pub fn stub_0x4ef440(handle: crate::slot::InstanceHandle) {
// RBX::MergedFilter dtor.
drop(handle);
}

#[doc(alias = "RBX::FilterHumanoidParts::~FilterHumanoidParts()")]
pub fn stub_0x4ef444(handle: crate::slot::InstanceHandle) {
// RBX::FilterHumanoidParts dtor.
drop(handle);
}

#[doc(alias = "RBX::FilterHumanoidParts::~FilterHumanoidParts() [0x4ef448]")]
pub fn stub_0x4ef448(handle: crate::slot::InstanceHandle) {
// RBX::FilterHumanoidParts dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_194")]
pub fn stub_0x4ef44c() -> crate::slot::PortedFn {
// IDA 0x4ef44c: __GLOBAL__I_a_194.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x4ef44c, "__GLOBAL__I_a_194")
}

#[doc(alias = "RBX::Fire::setColor(G3D::Color3)")]
pub fn stub_0x4ef6f0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Fire setter.
cell.set(value)
}

#[doc(alias = "RBX::Fire::setSecondaryColor(G3D::Color3)")]
pub fn stub_0x4ef758(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Fire setter.
cell.set(value)
}

#[doc(alias = "RBX::Fire::setSizeUi(float)")]
pub fn stub_0x4ef7c0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Fire setter.
cell.set(value)
}

#[doc(alias = "RBX::Fire::setHeatUi(float)")]
pub fn stub_0x4ef80c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Fire setter.
cell.set(value)
}

#[doc(alias = "RBX::Fire::setSize(float)")]
pub fn stub_0x4ef858(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Fire setter.
cell.set(value)
}

#[doc(alias = "RBX::Fire::setHeat(float)")]
pub fn stub_0x4ef898(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Fire setter.
cell.set(value)
}

#[doc(alias = "RBX::Fire::Fire(void)")]
pub fn stub_0x4ef8d8() -> crate::slot::InstanceHandle {
// RBX::Fire ctor.
crate::slot::InstanceHandle::new("RBX::Fire")
}

#[doc(alias = "RBX::Fire::~Fire()")]
pub fn stub_0x4efaf4(handle: crate::slot::InstanceHandle) {
// RBX::Fire dtor.
drop(handle);
}

#[doc(alias = "RBX::Fire::~Fire() [0x4efb94]")]
pub fn stub_0x4efb94(handle: crate::slot::InstanceHandle) {
// RBX::Fire dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Fire::~Fire()")]
pub fn stub_0x4efb98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Fire::~Fire() [0x4efba0]")]
pub fn stub_0x4efba0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Fire::~Fire() [0x4efba8]")]
pub fn stub_0x4efba8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Fire::~Fire() [0x4efbb0]")]
pub fn stub_0x4efbb0(handle: crate::slot::InstanceHandle) {
// RBX::Fire dtor.
drop(handle);
}
