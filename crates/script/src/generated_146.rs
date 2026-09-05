// Auto-generated skeletons for rbx-script — global filler EA-sorted asc continuation
// Filter: Script|Lua|Yield|lua (case-sensitive) — 5401 filtered, all already stubbed (14153 existing)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x2652b0..0x2684e0 | global filler EA-sorted asc after 0x2650e8 | rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::rethrow(void)const")]
pub fn stub_0x2652b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::~clone_impl()")]
pub fn stub_0x2652c0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::thread_resource_error>::~error_info_injector()")]
pub fn stub_0x2652e0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 20, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 20);
}

#[doc(alias = "boost::exception_detail::clone_base::~clone_base()")]
pub fn stub_0x2652f8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::thread_resource_error>>::clone_tag)")]
pub fn stub_0x265300(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::bad_exception_::~bad_exception_()")]
pub fn stub_0x2654d8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_exception_::~bad_exception_()")]
pub fn stub_0x265590(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 20, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 20);
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl() [0x265598]")]
pub fn stub_0x265598(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 20, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 20);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>::~clone_impl()")]
pub fn stub_0x2655a0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>(boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_> *)")]
pub fn stub_0x2655b0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::exception_detail::clone_impl<boost::exception_detail::bad_exception_>>::~sp_counted_impl_p()")]
pub fn stub_0x2656a8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_()")]
pub fn stub_0x2656b0(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::bad_alloc_::~bad_alloc_()")]
pub fn stub_0x265768(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 20, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 20);
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
pub fn stub_0x265770(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 20, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 20);
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::bad_alloc_>::~clone_impl()")]
pub fn stub_0x265778(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "boost::exception_detail::bad_alloc_::~bad_alloc_() [0x265788]")]
pub fn stub_0x265788(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "RBX::Reflection::Descriptor::~Descriptor()")]
pub fn stub_0x2657a0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Descriptor dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_59")]
pub fn stub_0x2657a4() -> crate::slot::PortedFn {
// IDA 0x2657a4: __GLOBAL__I_a_59.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2657a4, "__GLOBAL__I_a_59")
}

#[doc(alias = "RBX::Reflection::hash_value(RBX::Reflection::ConstProperty const&)")]
pub fn stub_0x2658d4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Reflection::PropertyDescriptor const*>(void)")]
pub fn stub_0x2658f8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Reflection::Property~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::allEnumsNameLookup(void)")]
pub fn stub_0x2659dc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor::allEnumsNameLookup() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::allEnumsTypeLookup(void)")]
pub fn stub_0x265a40(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor::allEnumsTypeLookup() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::allEnums(void)")]
pub fn stub_0x265b34(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor::allEnums() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::EnumDescriptor(char const*,std::type_info const&)")]
pub fn stub_0x265b8c() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDescriptor")
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::~EnumDescriptor()")]
pub fn stub_0x265cd4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::~EnumDescriptor() [0x265d74]")]
pub fn stub_0x265d74(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDescriptor::~EnumDescriptor() [0x265d78]")]
pub fn stub_0x265d78(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::Attributes::deprecated(RBX::Reflection::MemberDescriptor const&,RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0x265efc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::PropertyDescriptor::Attributes::deprecated(RBX::Reflection::MemberDescrip~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::Attributes::deprecated(RBX::Reflection::PropertyDescriptor::Functionality)")]
pub fn stub_0x265f08(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::PropertyDescriptor::Attributes::deprecated(RBX::Reflection::PropertyDescr~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::PropertyDescriptor(RBX::Reflection::ClassDescriptor &,RBX::Reflection::Type const&,char const*,char const*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions,bool)")]
pub fn stub_0x265f14() -> crate::slot::InstanceHandle {
// RBX::Reflection::PropertyDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::PropertyDescriptor")
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::write(RBX::Reflection::DescribedBase const*,bool)const")]
pub fn stub_0x26605c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::PropertyDescriptor::write(RBX::Reflection::DescribedBase const*, bool) co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::read(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x26629c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::PropertyDescriptor::read(RBX::Reflection::DescribedBase*, XmlElement cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TType<RBX::Reflection::PropertyDescriptor const*>::~TType()")]
pub fn stub_0x266338(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Reflection::EnumDescriptor const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::~map()")]
pub fn stub_0x26633c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::unordered_map<std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual,std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>>::~unordered_map()")]
pub fn stub_0x26634c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::~vector()")]
pub fn stub_0x26635c(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::Reflection::Type::Type(char const*,char const*,std::type_info const&,bool,bool)")]
pub fn stub_0x266370() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type")
}

#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::push_back(RBX::Reflection::EnumDescriptor const* const&)")]
pub fn stub_0x266408(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Reflection::EnumDescriptor const*,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x266434(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "RBX::Allocator<XmlElement>::operator new(unsigned long)")]
pub fn stub_0x26648c(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<XmlElement>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::~PropertyDescriptor()")]
pub fn stub_0x266500(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::PropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::~PropertyDescriptor() [0x266504]")]
pub fn stub_0x266504(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::PropertyDescriptor dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x266508(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::PropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropertyDescriptor::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x266530(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::PropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::operator new(unsigned long)")]
pub fn stub_0x266544(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<XmlAttribute>::operator new(unsigned long) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::lock_error::~lock_error()")]
pub fn stub_0x2665b8(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
pub fn stub_0x2665e8(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
pub fn stub_0x2665f8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 20, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 20);
}

#[doc(alias = "XmlAttribute::XmlAttribute<RBX::Name const*>(RBX::Name const&,RBX::Name const*)")]
pub fn stub_0x266600() -> crate::slot::PortedFn {
// IDA 0x266600: XmlAttribute::XmlAttribute<RBX::Name const*>(RBX::Name const&, RBX::Name const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x266600, "XmlAttribute::XmlAttribute<RBX::Name const*>(RBX::Name const&, RBX::Name const*)")
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::Allocator(void)")]
pub fn stub_0x2666c0() -> crate::slot::InstanceHandle {
// RBX::Allocator ctor.
crate::slot::InstanceHandle::new("RBX::Allocator")
}

#[doc(alias = "RBX::Allocator<XmlAttribute>::releaseMemory(void)")]
pub fn stub_0x266728(handle: &crate::slot::InstanceHandle) {
// RBX::Allocator<XmlAttribute>::releaseMemory() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<bool (*)(void),std::allocator<bool (*)(void)>>::_M_insert_aux(__gnu_cxx::__normal_iterator<bool (**)(void),std::vector<bool (*)(void),std::allocator<bool (*)(void)>>>,bool (* const&)(void))")]
pub fn stub_0x266748(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<bool (*)(void),std::allocator<bool (*)(void)>>::_M_allocate(unsigned long)")]
pub fn stub_0x266828() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::singleton_pool<XmlAttribute,20u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::release_memory(void)")]
pub fn stub_0x266840() -> crate::slot::PortedFn {
// std sync primitive linkage.
crate::slot::PortedFn::new(0x266840, "boost::singleton_pool<XmlAttribute, 20u, boost::default_user_allocator_malloc_free, boost::mutex, 32~")
}

#[doc(alias = "boost::pool<boost::default_user_allocator_malloc_free>::release_memory(void)")]
pub fn stub_0x266870() -> crate::slot::PortedFn {
// IDA 0x266870: boost::pool<boost::default_user_allocator_malloc_free>::release_memory().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x266870, "boost::pool<boost::default_user_allocator_malloc_free>::release_memory()")
}

#[doc(alias = "boost::simple_segregated_storage<unsigned long>::segregate(void *,unsigned long,unsigned long,void *)")]
pub fn stub_0x266960() -> crate::slot::PortedFn {
// IDA 0x266960: boost::simple_segregated_storage<unsigned long>::segregate(void*, unsigned long, unsigned long, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x266960, "boost::simple_segregated_storage<unsigned long>::segregate(void*, unsigned long, unsigned long, void~")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<char const* const,RBX::Reflection::PropertyDescriptor *>>,char const*,RBX::Reflection::PropertyDescriptor *,RBX::Reflection::StringHashPredicate,RBX::Reflection::StringEqualPredicate>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x2669c8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>> std::__find<__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,RBX::Reflection::EnumDescriptor*>(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,RBX::Reflection::EnumDescriptor* const&,std::random_access_iterator_tag)")]
pub fn stub_0x266a1c() -> crate::slot::PortedFn {
// IDA 0x266a1c: __gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**, std::vector<RBX::Reflection::EnumDescriptor const*~.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x266a1c, "__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**, std::vector<RBX::Reflection::E~")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::operator[](std::type_info const* const&)")]
pub fn stub_0x266aac(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x266c30(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::create_buckets(unsigned long)")]
pub fn stub_0x266c80(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x266da8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::rehash_impl(unsigned long)")]
pub fn stub_0x266e38(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x266e64(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>>>::construct(void)")]
pub fn stub_0x266ebc() -> crate::slot::PortedFn {
// std::type_info linkage data.
crate::slot::PortedFn::new(0x266ebc, "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pa~")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::find_node_impl<std::type_info const*,RBX::Reflection::EnumDescriptor::TypeEqual>(unsigned long,std::type_info const* const&,RBX::Reflection::EnumDescriptor::TypeEqual const&)const")]
pub fn stub_0x266ef4(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*> const&)")]
pub fn stub_0x266f6c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*> const&)")]
pub fn stub_0x267020(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*> const&)")]
pub fn stub_0x267078(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor const**,std::vector<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>>,RBX::Reflection::EnumDescriptor const* const&)")]
pub fn stub_0x2670e0(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Reflection::EnumDescriptor const*,std::allocator<RBX::Reflection::EnumDescriptor const*>>::_M_allocate(unsigned long)")]
pub fn stub_0x2671c0() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::delete_buckets(void)")]
pub fn stub_0x2671d8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>,std::type_info const*,RBX::Reflection::EnumDescriptor const*,RBX::Reflection::EnumDescriptor::TypeHash,RBX::Reflection::EnumDescriptor::TypeEqual>>::table(unsigned long,RBX::Reflection::EnumDescriptor::TypeHash const&,RBX::Reflection::EnumDescriptor::TypeEqual const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::type_info const* const,RBX::Reflection::EnumDescriptor const*>>> const&)")]
pub fn stub_0x267224() -> crate::slot::PortedFn {
// std::type_info linkage data.
crate::slot::PortedFn::new(0x267224, "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::type_inf~")
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::Reflection::PropertyDescriptor const*>(char const*,RBX::Reflection::PropertyDescriptor const* *)")]
pub fn stub_0x267290() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type::Type")
}

#[doc(alias = "RBX::Reflection::TType<RBX::Reflection::PropertyDescriptor const*>::~TType() [0x26733c]")]
pub fn stub_0x26733c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Type::~Type()")]
pub fn stub_0x267340(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Type dtor.
drop(handle);
}

#[doc(alias = "RBX::Reflection::Descriptor::~Descriptor() [0x267348]")]
pub fn stub_0x267348(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Descriptor dtor.
drop(handle);
}

#[doc(alias = "XmlElement::XmlElement(RBX::Name const&)")]
pub fn stub_0x267350() -> crate::slot::PortedFn {
// IDA 0x267350: XmlElement::XmlElement(RBX::Name const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x267350, "XmlElement::XmlElement(RBX::Name const&)")
}

#[doc(alias = "RBX::Allocator<XmlElement>::Allocator(void)")]
pub fn stub_0x267420() -> crate::slot::InstanceHandle {
// RBX::Allocator ctor.
crate::slot::InstanceHandle::new("RBX::Allocator")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Reflection::EnumDescriptor const*>> *)")]
pub fn stub_0x267488(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "global constructor keyed to_a_60")]
pub fn stub_0x2674b0() -> crate::slot::PortedFn {
// IDA 0x2674b0: __GLOBAL__I_a_60.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2674b0, "__GLOBAL__I_a_60")
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(void)")]
pub fn stub_0x2675e0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Reflection::Variant>(void)")]
pub fn stub_0x2676c4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Reflection::Variant>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Variant & RBX::Reflection::Variant::convert<RBX::Reflection::Variant>(void)")]
pub fn stub_0x2677a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Variant& RBX::Reflection::Variant::convert<RBX::Reflection::Variant>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> & RBX::Reflection::Variant::convert<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(void)")]
pub fn stub_0x2677ac() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>> ~")
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_0x26796c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> & RBX::Reflection::Variant::convert<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_0x267a50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_0x267c30() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> & RBX::Reflection::Variant::convert<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(void)")]
pub fn stub_0x267d18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::map<std::string, RBX::Reflection::Variant, std::less<std::string>, std::all~")
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<void>(void)")]
pub fn stub_0x267e5c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<void>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type::addToAllTypes(void)")]
pub fn stub_0x267f44(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type::addToAllTypes() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type::getAllTypes(void)")]
pub fn stub_0x267fb0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::Type getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::SignatureDescriptor::SignatureDescriptor(void)")]
pub fn stub_0x267fc0() -> crate::slot::InstanceHandle {
// RBX::Reflection::SignatureDescriptor ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::SignatureDescriptor")
}

#[doc(alias = "RBX::Reflection::SignatureDescriptor::Item::Item(RBX::Name const*,RBX::Reflection::Type const*,RBX::Reflection::Variant const&)")]
pub fn stub_0x267fec() -> crate::slot::InstanceHandle {
// RBX::Reflection::SignatureDescriptor::Item ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::SignatureDescriptor::Item")
}

#[doc(alias = "RBX::Reflection::SignatureDescriptor::Item::Item(RBX::Name const*,RBX::Reflection::Type const*)")]
pub fn stub_0x26813c() -> crate::slot::InstanceHandle {
// RBX::Reflection::SignatureDescriptor::Item ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::SignatureDescriptor::Item")
}

#[doc(alias = "RBX::Reflection::SignatureDescriptor::Item::Item(RBX::Name const*,RBX::Reflection::Type const*) [0x268140]")]
pub fn stub_0x268140() -> crate::slot::InstanceHandle {
// RBX::Reflection::SignatureDescriptor::Item ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::SignatureDescriptor::Item")
}

#[doc(alias = "RBX::Reflection::SignatureDescriptor::addArgument(RBX::Name const&,RBX::Reflection::Type const&,RBX::Reflection::Variant const&)")]
pub fn stub_0x2682e8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::SignatureDescriptor::addArgument(RBX::Name const&, RBX::Reflection::Type ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::~TType()")]
pub fn stub_0x268484(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> * rbx::any_cast<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x268488() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::unordered::unordered_map<std::string, RBX::Reflection::Variant, boost::ha~")
}

#[doc(alias = "RBX::Reflection::TType<rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::~TType()")]
pub fn stub_0x2684e0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}
