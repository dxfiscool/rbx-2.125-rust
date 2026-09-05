// Auto-generated skeletons for rbx-script — shard 208 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x301238..0x363924 | script 20952->21102 distinct (filler 0x301238 asc, not-in-script 64793->64643)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x301238(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x301360(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&> &,int)")]
pub fn stub_0x301478(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x301478: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3015d8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
pub fn stub_0x301770() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
pub fn stub_0x30188c() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
pub fn stub_0x3019a8() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "rbx_core::Weak<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(rbx_core::SharedPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")]
pub fn stub_0x301afc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpQueue")
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
pub fn stub_0x301b4c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x301b80(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask, std::allocator<RBX::AsyncHttpQueue::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
pub fn stub_0x301b98(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
pub fn stub_0x301c90() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<RBX::Reflection::Variant, std::allocator<RBX::Reflection::Variant>>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
pub fn stub_0x301d98() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::~sp_counted_impl_pd() [0x301ea0]")]
pub fn stub_0x301ea0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::dispose(void)")]
pub fn stub_0x301f58() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
pub fn stub_0x301f74() -> crate::slot::InstanceHandle {
// std::deque ctor.
crate::slot::InstanceHandle::new("std::deque")
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
pub fn stub_0x302028(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x302054(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask, std::allocator<RBX::AsyncHttpQueue::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
pub fn stub_0x3021d4(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask, std::allocator<RBX::AsyncHttpQueue::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")]
pub fn stub_0x3022c8(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::clear(void)")]
pub fn stub_0x3022f8(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")]
pub fn stub_0x302324() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HttpQueueStatsItem")
}

#[doc(alias = "RBX::HttpQueueStatsItem::init(void)")]
pub fn stub_0x3023dc(handle: &crate::slot::InstanceHandle) {
// RBX::HttpQueueStatsItem::init() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")]
pub fn stub_0x302418() -> crate::slot::InstanceHandle {
// RBX::HttpQueueStatsItem ctor.
crate::slot::InstanceHandle::new("RBX::HttpQueueStatsItem")
}

#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_0x30266c(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem() [0x3026a8]")]
pub fn stub_0x3026a8(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::HttpQueueStatsItem::update(void)")]
pub fn stub_0x30277c(handle: &crate::slot::InstanceHandle) {
// RBX::HttpQueueStatsItem::update() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
pub fn stub_0x3027d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem() [0x302810]")]
pub fn stub_0x302810(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem() [0x3028e8]")]
pub fn stub_0x3028e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem() [0x302928]")]
pub fn stub_0x302928(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x3029fc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HttpQueueStatsItem")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpQueueStatsItem,RBX::HttpQueueStatsItem>(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const*,RBX::HttpQueueStatsItem *)const")]
pub fn stub_0x302ac4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::HttpQueueStatsItem")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x302bac() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x302cb4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x302cb8]")]
pub fn stub_0x302cb8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x302cbc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x302cdc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x302cf4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
pub fn stub_0x302cf8(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "global constructor keyed to_a_106")]
pub fn stub_0x302d20() -> crate::slot::PortedFn {
// IDA 0x302d20: __GLOBAL__I_a_106.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x302d20, "__GLOBAL__I_a_106")
}

#[doc(alias = "RBX::Axes::Axes(int)")]
pub fn stub_0x302eb8() -> crate::slot::InstanceHandle {
// RBX::Axes ctor.
crate::slot::InstanceHandle::new("RBX::Axes")
}

#[doc(alias = "RBX::Axes::normalIdToAxis(RBX::NormalId)")]
pub fn stub_0x302ebc(handle: &crate::slot::InstanceHandle) {
// RBX::Axes::normalIdToAxis(RBX::NormalId) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Axes::getAxisByNormalId(RBX::NormalId)const")]
pub fn stub_0x302ef0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Axes getter.
cell.get()
}

#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")]
pub fn stub_0x302f30(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void)")]
pub fn stub_0x303124() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::EnumDesc(void) [0x303128]")]
pub fn stub_0x303128() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "G3D::Vector3::Axis & RBX::Reflection::Variant::convert<G3D::Vector3::Axis>(void)")]
pub fn stub_0x303300() -> crate::slot::PortedFn {
// IDA 0x303300: G3D::Vector3::Axis& RBX::Reflection::Variant::convert<G3D::Vector3::Axis>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x303300, "G3D::Vector3::Axis& RBX::Reflection::Variant::convert<G3D::Vector3::Axis>()")
}

#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")]
pub fn stub_0x303418(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&, RBX::Axes&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::addPair(G3D::Vector3::Axis,char const*)")]
pub fn stub_0x30367c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<G3D::Vector3::Axis>::addPair(G3D::Vector3::Axis, char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "G3D::Vector3::Axis & RBX::Reflection::Variant::genericConvert<G3D::Vector3::Axis>(void)")]
pub fn stub_0x3039dc() -> crate::slot::PortedFn {
// IDA 0x3039dc: G3D::Vector3::Axis& RBX::Reflection::Variant::genericConvert<G3D::Vector3::Axis>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3039dc, "G3D::Vector3::Axis& RBX::Reflection::Variant::genericConvert<G3D::Vector3::Axis>()")
}

#[doc(alias = "RBX::ProtectedString * rbx::any_cast<RBX::ProtectedString,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x35fdd0(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ProtectedString>(RBX::ProtectedString const&)")]
pub fn stub_0x35fe28() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "RBX::ProtectedString & rbx::any_cast<RBX::ProtectedString &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x35fe84(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::singleton(void)")]
pub fn stub_0x35ff74(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::ProtectedString>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::construct_func(char const*,char *)")]
pub fn stub_0x35ffe0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::ProtectedString>::construct_func(char const*, char*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::ProtectedString>::destruct_func(char *)")]
pub fn stub_0x35fff0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::ProtectedString>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type::Type<RBX::ProtectedString>(char const*,RBX::ProtectedString *)")]
pub fn stub_0x3600a4() -> crate::slot::InstanceHandle {
// RBX::Reflection::Type::Type ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::Type::Type")
}

#[doc(alias = "RBX::Reflection::TType<RBX::ProtectedString>::~TType() [0x360150]")]
pub fn stub_0x360150(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::TType dtor.
drop(handle);
}

#[doc(alias = "global constructor keyed to_a_133")]
pub fn stub_0x360154() -> crate::slot::PortedFn {
// IDA 0x360154: __GLOBAL__I_a_133.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x360154, "__GLOBAL__I_a_133")
}

#[doc(alias = "RBX::Quaternion::operator=(RBX::Quaternion const&)")]
pub fn stub_0x3602a8(handle: &crate::slot::InstanceHandle) {
// RBX::Quaternion::operator=(RBX::Quaternion const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Quaternion::Quaternion(G3D::Matrix3 const&)")]
pub fn stub_0x3602bc() -> crate::slot::InstanceHandle {
// RBX::Quaternion ctor.
crate::slot::InstanceHandle::new("RBX::Quaternion")
}

#[doc(alias = "RBX::Quaternion::Quaternion(G3D::Matrix3 const&) [0x3602c0]")]
pub fn stub_0x3602c0() -> crate::slot::InstanceHandle {
// RBX::Quaternion ctor.
crate::slot::InstanceHandle::new("RBX::Quaternion")
}

#[doc(alias = "RBX::Quaternion::toRotationMatrix(G3D::Matrix3 &)const")]
pub fn stub_0x360478(handle: &crate::slot::InstanceHandle) {
// RBX::Quaternion::toRotationMatrix(G3D::Matrix3&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_134")]
pub fn stub_0x360528() -> crate::slot::PortedFn {
// IDA 0x360528: __GLOBAL__I_a_134.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x360528, "__GLOBAL__I_a_134")
}

#[doc(alias = "RBX::Rect::positionPoint(RBX::Rect::Location,RBX::Rect::Location)const")]
pub fn stub_0x360560(handle: &crate::slot::InstanceHandle) {
// RBX::Rect::positionPoint(RBX::Rect::Location, RBX::Rect::Location) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Rect::positionChild(RBX::Rect const&,RBX::Rect::Location,RBX::Rect::Location)const")]
pub fn stub_0x360678(handle: &crate::slot::InstanceHandle) {
// RBX::Rect::positionChild(RBX::Rect const&, RBX::Rect::Location, RBX::Rect::Location) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_135")]
pub fn stub_0x3607f4() -> crate::slot::PortedFn {
// IDA 0x3607f4: __GLOBAL__I_a_135.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3607f4, "__GLOBAL__I_a_135")
}

#[doc(alias = "RBX::RunService::RunService(void)")]
pub fn stub_0x36082c() -> crate::slot::InstanceHandle {
// RBX::RunService ctor.
crate::slot::InstanceHandle::new("RBX::RunService")
}

#[doc(alias = "RBX::RunService::RunService(void) [0x360830]")]
pub fn stub_0x360830() -> crate::slot::InstanceHandle {
// RBX::RunService ctor.
crate::slot::InstanceHandle::new("RBX::RunService")
}

#[doc(alias = "RBX::RunService::stopTasks(void)")]
pub fn stub_0x360dd4(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::stopTasks() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::start(void)")]
pub fn stub_0x360f34(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::start() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::~RunService()")]
pub fn stub_0x3611ec(handle: crate::slot::InstanceHandle) {
// RBX::RunService dtor.
drop(handle);
}

#[doc(alias = "RBX::RunService::~RunService() [0x36128c]")]
pub fn stub_0x36128c(handle: crate::slot::InstanceHandle) {
// RBX::RunService dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService()")]
pub fn stub_0x361290(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService() [0x361298]")]
pub fn stub_0x361298(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::RunService::~RunService() [0x3612a0]")]
pub fn stub_0x3612a0(handle: crate::slot::InstanceHandle) {
// RBX::RunService dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService() [0x3616a8]")]
pub fn stub_0x3616a8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::RunService::~RunService() [0x3616b0]")]
pub fn stub_0x3616b0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::RunService::getPhysicsJob(void)")]
pub fn stub_0x3616b8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::RunService getter.
cell.get()
}

#[doc(alias = "RBX::RunService::raiseHeartbeat(double,RBX::Time::Interval const&)")]
pub fn stub_0x3616bc(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::raiseHeartbeat(double, RBX::Time::Interval const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::gameStepped(double)")]
pub fn stub_0x361750(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::gameStepped(double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::setRunState(RBX::RunState)")]
pub fn stub_0x3617b8(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::RunService setter.
cell.set(value)
}

#[doc(alias = "RBX::RunService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x361818(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::smoothFps(void)const")]
pub fn stub_0x361824(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::smoothFps() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::heartbeatFps(void)const")]
pub fn stub_0x36182c(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::heartbeatFps() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::physicsAverageStep(void)const")]
pub fn stub_0x361834(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::physicsAverageStep() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::heartbeatAverageStep(void)const")]
pub fn stub_0x36183c(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::heartbeatAverageStep() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::physicsCpuFraction(void)const")]
pub fn stub_0x361844(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::physicsCpuFraction() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::heartbeatCpuFraction(void)const")]
pub fn stub_0x36184c(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::heartbeatCpuFraction() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double,double),rbx::signal<void ()(double,double)>,rbx::signal<void ()(double,double)> RBX::RunService::*>::~EventDesc()")]
pub fn stub_0x361858(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::RunService,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::RunService::*>::~EventDesc()")]
pub fn stub_0x36187c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::RunService::run(void)")]
pub fn stub_0x3618a0(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::run() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RunService,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_0x3618a8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::RunService::pause(void)")]
pub fn stub_0x3618cc(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::pause() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::RunService::stop(void)")]
pub fn stub_0x3618d4(handle: &crate::slot::InstanceHandle) {
// RBX::RunService::stop() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PhysicsJob>::operator=(rbx_core::SharedPtr<RBX::PhysicsJob> const&)")]
pub fn stub_0x3618e0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel> RBX::shared_from_dynamic_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
pub fn stub_0x361918() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DataModel")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HeartbeatTask>::operator=(rbx_core::SharedPtr<RBX::HeartbeatTask> const&)")]
pub fn stub_0x361a78(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::RunService> RBX::shared_from<RBX::RunService>(RBX::RunService*)")]
pub fn stub_0x361ab0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::RunService")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Heartbeat const&)>::operator()(RBX::Heartbeat const&)")]
pub fn stub_0x361c20(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (RBX::Heartbeat const&)>::operator()(RBX::Heartbeat~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(double)>::operator()(double)")]
pub fn stub_0x361d64(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (double)>::operator()(double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Stepped const&)>::operator()(RBX::Stepped const&)")]
pub fn stub_0x361eb0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (RBX::Stepped const&)>::operator()(RBX::Stepped con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(double,double)>::operator()(double,double)")]
pub fn stub_0x361ff4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<2, void (double, double)>::operator()(double, double) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::RunTransition)>::operator()(RBX::RunTransition)")]
pub fn stub_0x362158(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (RBX::RunTransition)>::operator()(RBX::RunTransitio~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Instance::raiseEventInvocation(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const*)")]
pub fn stub_0x3622c8(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::raiseEventInvocation(RBX::Reflection::EventDescriptor const&, std::vector<R~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Instance::verifyAddChild(RBX::Instance const*)const")]
pub fn stub_0x362300(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::verifyAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Instance::onChildRemoving(RBX::Instance*)")]
pub fn stub_0x362308(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::onChildRemoving(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Instance::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0x362310(handle: &crate::slot::InstanceHandle) {
// RBX::Instance::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE12getClassNameEv")]
pub fn stub_0x362314() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_11sRunServiceEEE12getClassNameEv")]
pub fn stub_0x362340() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::getCreators(void)")]
pub fn stub_0x362368() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7Creator12getClassNameEv")]
pub fn stub_0x3623d8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Camera"
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Camera>::shared_ptr<RBX::Camera,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x362448() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Camera")
}

#[doc(alias = "boost::detail::weak_count::operator=(boost::detail::shared_count const&)")]
pub fn stub_0x362510() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x362570() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x362678() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v")]
pub fn stub_0x362698(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sCamera>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v")]
pub fn stub_0x3626e0(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sCamera>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> &)")]
pub fn stub_0x3627c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::RunTransition)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::on_error(std::exception &)")]
pub fn stub_0x362924(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "boost::bad_function_call::bad_function_call(void)")]
pub fn stub_0x362950() -> crate::slot::PortedFn {
// IDA 0x362950: boost::bad_function_call::bad_function_call().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x362950, "boost::bad_function_call::bad_function_call()")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl()")]
pub fn stub_0x362a98(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::bad_function_call>::~error_info_injector()")]
pub fn stub_0x362aa8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::~clone_impl() [0x362ab0]")]
pub fn stub_0x362ab0(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_tag)")]
pub fn stub_0x362ac8(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::bad_function_call>>::clone_impl(boost::exception_detail::error_info_injector<boost::bad_function_call> const&)")]
pub fn stub_0x362c30(msg: &str) -> String {
// std::exception ctor — message carried by panic.
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot> const&)")]
pub fn stub_0x362d98(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::rethrow(void)const")]
pub fn stub_0x362dc0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::rethrow(void)const")]
pub fn stub_0x362f88(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::~clone_impl()")]
pub fn stub_0x362f98(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::lock_error>::~error_info_injector()")]
pub fn stub_0x362fb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 20, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 20);
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::lock_error>>::clone_tag)")]
pub fn stub_0x362fd0(guard: &mut bool) {
// std lock op — the host guard flips.
*guard = true;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::safe_static_init_mutex(void)")]
pub fn stub_0x3631a8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::RunTransition)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void rbx_core::SharedPtr_add_ref<rbx::signals::connection::islot,int,0,0>(rbx::intrusive_ptr_target<rbx::signals::connection::islot,int,0,0> const*)")]
pub fn stub_0x3631b0() -> crate::slot::SlotConnection {
// IDA 0x3631b0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot> &)")]
pub fn stub_0x363224() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (double, double)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::on_error(std::exception &)")]
pub fn stub_0x363384(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(double,double)>::slot> const&)")]
pub fn stub_0x3633ac(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_init_mutex(void)")]
pub fn stub_0x3633d0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double, double)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(double,double)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x3633d4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double, double)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> &)")]
pub fn stub_0x3634cc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::Stepped const&)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::on_error(std::exception &)")]
pub fn stub_0x36362c(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot> const&)")]
pub fn stub_0x363654(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_init_mutex(void)")]
pub fn stub_0x363678(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Stepped const&)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x36367c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Stepped const&)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot> &)")]
pub fn stub_0x363774() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (double)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::on_error(std::exception &)")]
pub fn stub_0x3638d4(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(double)>::slot> const&)")]
pub fn stub_0x3638fc(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_init_mutex(void)")]
pub fn stub_0x363920(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(double)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x363924(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (double)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}
