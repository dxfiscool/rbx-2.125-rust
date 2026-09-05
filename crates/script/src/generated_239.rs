// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler
// Filter: Script|Lua|Yield|CodeGen|Luau (5041 filtered, all already stubbed in crates/script/src) — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4961b0..0x49a2c4 | EA-sorted asc distinct not yet in script (remaining 58050->57950, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "RBX::DialogRoot::getDialogTone(void)const")]
pub fn stub_0x4961b0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DialogRoot getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::~EnumPropDescriptor()")]
pub fn stub_0x4961b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DialogRoot::getConversationDistance(void)const")]
pub fn stub_0x4961d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DialogRoot getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::~PropDescriptor()")]
pub fn stub_0x4961dc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::DialogRoot::getInUse(void)const")]
pub fn stub_0x496200(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::DialogRoot getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::~PropDescriptor()")]
pub fn stub_0x496208(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_0x49622c(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc()")]
pub fn stub_0x496344(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::addPair(RBX::DialogRoot::DialogPurpose,char const*)")]
pub fn stub_0x496368(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::addPair(RBX::DialogRoot::Dialog~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::addPair(RBX::DialogRoot::DialogTone,char const*)")]
pub fn stub_0x4966c8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::addPair(RBX::DialogRoot::DialogTon~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::fireAndReplicateEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x496a28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot()")]
pub fn stub_0x496d48(handle: crate::slot::InstanceHandle) {
// RBX::DialogRoot dtor.
drop(handle);
}

#[doc(alias = "RBX::DialogRoot::~DialogRoot() [0x496d4c]")]
pub fn stub_0x496d4c(handle: crate::slot::InstanceHandle) {
// RBX::DialogRoot dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot()")]
pub fn stub_0x496dfc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot() [0x496e04]")]
pub fn stub_0x496e04(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot() [0x496eb8]")]
pub fn stub_0x496eb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::DialogRoot::~DialogRoot() [0x496ec0]")]
pub fn stub_0x496ec0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot> RBX::Creatable<RBX::Instance>::create<RBX::DialogRoot>(void)")]
pub fn stub_0x4971d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DialogRoot")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::DialogRoot>::shared_ptr<RBX::DialogRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x497280() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DialogRoot")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogRoot,RBX::DialogRoot>(rbx_core::SharedPtr<RBX::DialogRoot> const*,RBX::DialogRoot *)const")]
pub fn stub_0x497348() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::DialogRoot")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_0x497430() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x497538(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x49753c]")]
pub fn stub_0x49753c(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0x497540() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x497560() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DialogRoot *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x497578() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::DialogRoot*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x497918() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x497a38() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::remote_signal(void)")]
pub fn stub_0x497ba4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x497d00(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x497d04(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x497da4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x497dac(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x497e50(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10DialogRootELZNS_11sDialogRootEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sDialogRootEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x497e58(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::resize(unsigned long,RBX::DialogRoot::DialogTone)")]
pub fn stub_0x497efc(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::push_back(RBX::DialogRoot::DialogTone const&)")]
pub fn stub_0x497f30(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::DialogRoot::DialogTone,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x497f58(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
pub fn stub_0x497fb0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
pub fn stub_0x498064(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>,std::_Select1st<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::DialogRoot::DialogTone> const&)")]
pub fn stub_0x4980bc(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,RBX::DialogRoot::DialogTone const&)")]
pub fn stub_0x498124(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_allocate(unsigned long)")]
pub fn stub_0x498208() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::DialogRoot::DialogTone * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *>(RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *,RBX::DialogRoot::DialogTone *)")]
pub fn stub_0x498220(handle: &crate::slot::InstanceHandle) {
// RBX::DialogRoot::DialogTone* std::__copy_backward<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::DialogRoot::DialogTone*,std::vector<RBX::DialogRoot::DialogTone,std::allocator<RBX::DialogRoot::DialogTone>>>,unsigned long,RBX::DialogRoot::DialogTone const&)")]
pub fn stub_0x49825c(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc() [0x4988dc]")]
pub fn stub_0x4988dc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x498990() -> crate::slot::SlotConnection {
// IDA 0x498990: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::isBroadcast(void)const")]
pub fn stub_0x498afc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x498b04() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x498cb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x498cc4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::EventDesc(rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x498cd8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc()")]
pub fn stub_0x498ec8(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::DialogRoot::*>::~EventDesc() [0x498eec]")]
pub fn stub_0x498eec(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::BoundFuncDesc(void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x498fa0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x499170() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc() [0x4991bc]")]
pub fn stub_0x4991bc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DialogRoot,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x4992e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call2Helper<RBX::DialogRoot,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::DialogRoot*,void (RBX::DialogRoot::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x499408() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::PropDescriptor<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>(char const*,char const*,bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x499540(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::~PropDescriptor() [0x499654]")]
pub fn stub_0x499654(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::isReadOnly(void)const")]
pub fn stub_0x499680(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::isWriteOnly(void)const")]
pub fn stub_0x499684(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x499688(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::GetSetImpl<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub fn stub_0x4996ac(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::PropDescriptor<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>(char const*,char const*,float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x4996d0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::~PropDescriptor() [0x4997e4]")]
pub fn stub_0x4997e4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::isReadOnly(void)const")]
pub fn stub_0x499810(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::isWriteOnly(void)const")]
pub fn stub_0x499814(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x499818(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::GetSetImpl<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub fn stub_0x499838(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::EnumPropDescriptor<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>(char const*,char const*,RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x49985c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::~EnumPropDescriptor() [0x499a10]")]
pub fn stub_0x499a10(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::isReadOnly(void)const")]
pub fn stub_0x499a3c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::isWriteOnly(void)const")]
pub fn stub_0x499a4c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x499a5c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_0x499a84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_0x499aa8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_0x499bf4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::hasStringValue(void)const")]
pub fn stub_0x499c18(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x499c1c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x499c40(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x499c80(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x499ca0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x499ee0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_0x499efc(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x499f30(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x499f38(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x499f84(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_0x499fa4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::DialogTone)const")]
pub fn stub_0x499fd8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::Di~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_0x49a048(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isReadOnly(void)const")]
pub fn stub_0x49a088(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::isWriteOnly(void)const")]
pub fn stub_0x49a08c(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x49a090(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::GetSetImpl<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>::setValue(RBX::Reflection::DescribedBase *,RBX::DialogRoot::DialogTone const&)const")]
pub fn stub_0x49a0b0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_0x49a0d4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::~EnumPropDescriptor() [0x49a288]")]
pub fn stub_0x49a288(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isReadOnly(void)const")]
pub fn stub_0x49a2b4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::isWriteOnly(void)const")]
pub fn stub_0x49a2c4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}
