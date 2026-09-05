// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua|Yield|lua (case-sensitive) — 5401 filtered, all already stubbed (14053 existing)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x26113c..0x2650e8 | global filler EA-sorted asc after 0x261138 | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(void) [0x26113c]")]
pub fn stub_0x26113c() -> crate::slot::InstanceHandle {
// RBX::Reflection::ClassDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::ClassDescriptor")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(RBX::Reflection::ClassDescriptor&,char const*,RBX::Reflection::ClassDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x261300() -> crate::slot::InstanceHandle {
// RBX::Reflection::ClassDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::ClassDescriptor")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::ClassDescriptor(RBX::Reflection::ClassDescriptor&,char const*,RBX::Reflection::ClassDescriptor::Attributes,RBX::Security::Permissions) [0x26131c]")]
pub fn stub_0x26131c() -> crate::slot::InstanceHandle {
// RBX::Reflection::ClassDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::ClassDescriptor")
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::operator==(RBX::Reflection::ClassDescriptor const&)const")]
pub fn stub_0x2616c0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::ClassDescriptor::operator==(RBX::Reflection::ClassDescriptor const&) cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::operator!=(RBX::Reflection::ClassDescriptor const&)const")]
pub fn stub_0x2616cc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::ClassDescriptor::operator!=(RBX::Reflection::ClassDescriptor const&) cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::isA(RBX::Reflection::ClassDescriptor const&)const")]
pub fn stub_0x2616d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::ClassDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::isA(char const*)const")]
pub fn stub_0x2616f0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::ClassDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::MemberDescriptor::isMemberOf(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x261718(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::MemberDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::Descriptor::Descriptor(char const*,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x261798() -> crate::slot::InstanceHandle {
// RBX::Reflection::Descriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Descriptor")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>*)")]
pub fn stub_0x261830() -> crate::slot::InstanceHandle {
// RBX::Reflection::MemberDescriptorContainer ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::MemberDescriptorContainer")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>*)")]
pub fn stub_0x261948() -> crate::slot::InstanceHandle {
// RBX::Reflection::MemberDescriptorContainer ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::MemberDescriptorContainer")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>*)")]
pub fn stub_0x261a60() -> crate::slot::InstanceHandle {
// RBX::Reflection::MemberDescriptorContainer ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::MemberDescriptorContainer")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::MemberDescriptorContainer(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>*)")]
pub fn stub_0x261c90() -> crate::slot::InstanceHandle {
// RBX::Reflection::MemberDescriptorContainer ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::MemberDescriptorContainer")
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor **,std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>>,RBX::Reflection::ClassDescriptor * const&)")]
pub fn stub_0x261da8() -> crate::slot::PortedFn {
// IDA 0x261da8: std::vector<RBX::Reflection::ClassDescriptor*, std::allocator<RBX::Reflection::ClassDescriptor*>>::insert(__gnu_cxx::__n~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x261da8, "std::vector<RBX::Reflection::ClassDescriptor*, std::allocator<RBX::Reflection::ClassDescriptor*>>::i~")
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::~vector()")]
pub fn stub_0x261de0(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_alloc_>(void)")]
pub fn stub_0x261df8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_ptr boost::exception_detail::get_static_exception_object<boost::exception_detail::bad_exception_>(void)")]
pub fn stub_0x2620f0(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "RBX::Reflection::ClassDescriptor::~ClassDescriptor()")]
pub fn stub_0x2623e8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::ClassDescriptor dtor.
drop(handle);
}

#[doc(alias = "std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::ClassDescriptor **,std::vector<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>>,RBX::Reflection::ClassDescriptor * const&)")]
pub fn stub_0x2624b4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::ClassDescriptor *,std::allocator<RBX::Reflection::ClassDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_0x262594() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> const*)")]
pub fn stub_0x2625ac(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::mergeMemb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> * const&)")]
pub fn stub_0x2625d4(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::Collection::~Collection()")]
pub fn stub_0x262600(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer dtor.
drop(handle);
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> * const&)")]
pub fn stub_0x262618(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor> *>>::_M_allocate(unsigned long)")]
pub fn stub_0x2626f8() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(RBX::Reflection::CallbackDescriptor*)")]
pub fn stub_0x262710(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declare(R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::CallbackDescriptor **,std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>>,RBX::Reflection::CallbackDescriptor * const&)")]
pub fn stub_0x262890() -> crate::slot::PortedFn {
// IDA 0x262890: std::vector<RBX::Reflection::CallbackDescriptor*, std::allocator<RBX::Reflection::CallbackDescriptor*>>::insert(__gnu_cx~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x262890, "std::vector<RBX::Reflection::CallbackDescriptor*, std::allocator<RBX::Reflection::CallbackDescriptor~")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSub(RBX::Reflection::CallbackDescriptor*,RBX::Reflection::CallbackDescriptor*)")]
pub fn stub_0x2628c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::declareSu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::initStaticData(void)")]
pub fn stub_0x262a44(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::initStati~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticData(void)")]
pub fn stub_0x262a48(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::staticDat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
pub fn stub_0x262ab0(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x262c34(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
pub fn stub_0x262c88(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x262db0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
pub fn stub_0x262e40(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x262e6c(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>>>::construct(void)")]
pub fn stub_0x262ec4() -> crate::slot::PortedFn {
// IDA 0x262ec4: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x262ec4, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
pub fn stub_0x262efc(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "RBX::Reflection::StringHashPredicate::operator()(char const*)const")]
pub fn stub_0x262f6c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::StringHashPredicate::operator()(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::CallbackDescriptor **,std::vector<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>>,RBX::Reflection::CallbackDescriptor * const&)")]
pub fn stub_0x262fa4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::CallbackDescriptor *,std::allocator<RBX::Reflection::CallbackDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_0x263084() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>,char const*,RBX::Reflection::CallbackDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::CallbackDescriptor *>>> const&)")]
pub fn stub_0x26309c() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> const*)")]
pub fn stub_0x263748(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::mergeMembers~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> * const&)")]
pub fn stub_0x263770(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
pub fn stub_0x2637a0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> * const&)")]
pub fn stub_0x2637ec(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor> *>>::_M_allocate(unsigned long)")]
pub fn stub_0x2638cc() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::initStaticData(void)")]
pub fn stub_0x2638e8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::initStaticDa~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x2638ec(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
pub fn stub_0x263940(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x263a68(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
pub fn stub_0x263af8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x263b24(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>>>::construct(void)")]
pub fn stub_0x263b7c() -> crate::slot::PortedFn {
// IDA 0x263b7c: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x263b7c, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
pub fn stub_0x263bb4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::EventDescriptor **,std::vector<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>>,RBX::Reflection::EventDescriptor * const&)")]
pub fn stub_0x263c24(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::EventDescriptor *,std::allocator<RBX::Reflection::EventDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_0x263d04() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>,char const*,RBX::Reflection::EventDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::EventDescriptor *>>> const&)")]
pub fn stub_0x263d1c() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> const*)")]
pub fn stub_0x263d88(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::mergeMemb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> * const&)")]
pub fn stub_0x263db0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::Collection::~Collection()")]
pub fn stub_0x263ddc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer dtor.
drop(handle);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
pub fn stub_0x263df0(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> * const&)")]
pub fn stub_0x263e3c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor> *>>::_M_allocate(unsigned long)")]
pub fn stub_0x263f1c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "std::vector<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::FunctionDescriptor **,std::vector<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>>,RBX::Reflection::FunctionDescriptor * const&)")]
pub fn stub_0x263f34() -> crate::slot::PortedFn {
// IDA 0x263f34: std::vector<RBX::Reflection::FunctionDescriptor*, std::allocator<RBX::Reflection::FunctionDescriptor*>>::insert(__gnu_cx~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x263f34, "std::vector<RBX::Reflection::FunctionDescriptor*, std::allocator<RBX::Reflection::FunctionDescriptor~")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declareSub(RBX::Reflection::FunctionDescriptor*,RBX::Reflection::FunctionDescriptor*)")]
pub fn stub_0x263f6c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::declareSu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::initStaticData(void)")]
pub fn stub_0x2640e8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::initStati~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::staticData(void)")]
pub fn stub_0x2640ec(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::staticDat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
pub fn stub_0x264158(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x264280(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
pub fn stub_0x264310(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x26433c(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>>>::construct(void)")]
pub fn stub_0x264394() -> crate::slot::PortedFn {
// IDA 0x264394: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x264394, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
pub fn stub_0x2643cc(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::vector<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::FunctionDescriptor **,std::vector<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>>,RBX::Reflection::FunctionDescriptor * const&)")]
pub fn stub_0x26443c(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::FunctionDescriptor *,std::allocator<RBX::Reflection::FunctionDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_0x26451c() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>,char const*,RBX::Reflection::FunctionDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::FunctionDescriptor *>>> const&)")]
pub fn stub_0x264534() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::mergeMembers(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> const*)")]
pub fn stub_0x2645a0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::mergeMemb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>::push_back(RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> * const&)")]
pub fn stub_0x2645c8(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::Collection::~Collection()")]
pub fn stub_0x2645f4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer dtor.
drop(handle);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::delete_buckets(void)")]
pub fn stub_0x264608(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> **,std::vector<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>>,RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> * const&)")]
pub fn stub_0x264654(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *,std::allocator<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor> *>>::_M_allocate(unsigned long)")]
pub fn stub_0x264734() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declare(RBX::Reflection::PropertyDescriptor*)")]
pub fn stub_0x26474c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declare(R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>::insert(__gnu_cxx::__normal_iterator<RBX::Reflection::PropertyDescriptor **,std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>>,RBX::Reflection::PropertyDescriptor * const&)")]
pub fn stub_0x2648cc() -> crate::slot::PortedFn {
// IDA 0x2648cc: std::vector<RBX::Reflection::PropertyDescriptor*, std::allocator<RBX::Reflection::PropertyDescriptor*>>::insert(__gnu_cx~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2648cc, "std::vector<RBX::Reflection::PropertyDescriptor*, std::allocator<RBX::Reflection::PropertyDescriptor~")
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declareSub(RBX::Reflection::PropertyDescriptor*,RBX::Reflection::PropertyDescriptor*)")]
pub fn stub_0x264904(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::declareSu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::initStaticData(void)")]
pub fn stub_0x264a80(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::initStati~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::staticData(void)")]
pub fn stub_0x264a84(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::PropertyDescriptor>::staticDat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::operator[](char const* const&)")]
pub fn stub_0x264aec(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::create_buckets(unsigned long)")]
pub fn stub_0x264c70(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x264d98(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::rehash_impl(unsigned long)")]
pub fn stub_0x264e28(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x264e54(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>>>::construct(void)")]
pub fn stub_0x264eac() -> crate::slot::PortedFn {
// IDA 0x264eac: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x264eac, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::find_node_impl<char const*,RBX::Reflection::StringEqualPredicate>(unsigned long,char const* const&,RBX::Reflection::StringEqualPredicate const&)const")]
pub fn stub_0x264ee4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::PropertyDescriptor **,std::vector<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>>,RBX::Reflection::PropertyDescriptor * const&)")]
pub fn stub_0x264f54(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::PropertyDescriptor *,std::allocator<RBX::Reflection::PropertyDescriptor *>>::_M_allocate(unsigned long)")]
pub fn stub_0x265034() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::table(unsigned long,RBX::Reflection::StringHashPredicate const&,RBX::Reflection::StringEqualPredicate const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>> const&)")]
pub fn stub_0x26504c() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "boost::thread_resource_error::~thread_resource_error()")]
pub fn stub_0x2650b8() -> crate::slot::PortedFn {
// IDA 0x2650b8: boost::thread_resource_error::~thread_resource_error().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2650b8, "boost::thread_resource_error::~thread_resource_error()")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::rethrow(void)const")]
pub fn stub_0x2650e8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}
