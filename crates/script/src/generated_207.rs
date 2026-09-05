// Auto-generated skeletons for rbx-script — shard 207 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x2f71d8..0x30110c | script 17841->17991 distinct (filler 0x2f71d8 asc, not-in-script 67704->67554)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::~BoxSelectCommand()")]
pub fn stub_0x2f71d8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::BoxSelectCommand::~BoxSelectCommand() [0x2f71e0]")]
pub fn stub_0x2f71e0(handle: crate::slot::InstanceHandle) {
// RBX::BoxSelectCommand dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::~BoxSelectCommand() [0x2f7324]")]
pub fn stub_0x2f7324(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::BoxSelectCommand::selectAnd(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
pub fn stub_0x2f732c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::BoxSelectCommand::selectReverse(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
pub fn stub_0x2f7394() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::BoxSelectCommand::onMouseDown(RBX::UIEvent const&)")]
pub fn stub_0x2f73fc(handle: &crate::slot::InstanceHandle) {
// RBX::BoxSelectCommand::onMouseDown(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BoxSelectCommand::onMouseMove(RBX::UIEvent const&)")]
pub fn stub_0x2f7468(handle: &crate::slot::InstanceHandle) {
// RBX::BoxSelectCommand::onMouseMove(RBX::UIEvent const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BoxSelectCommand::getMouseInstances(std::set<rbx_core::SharedPtr<RBX::Instance>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> &,RBX::UIEvent const&,G3D::Rect2D &)")]
pub fn stub_0x2f75e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
pub fn stub_0x2f7818(handle: &crate::slot::InstanceHandle) {
// RBX::BoxSelectCommand::render2d(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::BoxSelectCommand::render2d(RBX::Adorn *)")]
pub fn stub_0x2f78d0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 4, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 4);
}

#[doc(alias = "RBX::Selection::isSelected(RBX::Instance const*)const")]
pub fn stub_0x2f78d8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Selection getter.
cell.get()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::BoxSelectCommand,RBX::Workspace *>(RBX::Workspace *)")]
pub fn stub_0x2f79c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BoxSelectCommand")
}

#[doc(alias = "RBX::Selection::AddIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
pub fn stub_0x2f7a7c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Selection::RemoveIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
pub fn stub_0x2f7bd4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Selection::ToggleIterator std::set_difference<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
pub fn stub_0x2f7d2c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Selectable * RBX::Instance::queryTypedChild<RBX::Selectable>(int)")]
pub fn stub_0x2f7e84(handle: &crate::slot::InstanceHandle) {
// RBX::Selectable* RBX::Instance::queryTypedChild<RBX::Selectable>(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX5NamedINS_12MouseCommandELZNS_17sBoxSelectCommandEEE7getNameEv")]
pub fn stub_0x2f7ec0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Named getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_17sBoxSelectCommandEEEEvv")]
pub fn stub_0x2f7ee8() -> crate::slot::PortedFn {
// IDA 0x2f7ee8: void RBX::Name::callDoDeclare<RBX::sBoxSelectCommand>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2f7ee8, "void RBX::Name::callDoDeclare<RBX::sBoxSelectCommand>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")]
pub fn stub_0x2f7eec(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sBoxSelectCommand>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert_unique(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x2f7fcc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x2f8034() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x2f8080() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_erase(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
pub fn stub_0x2f8164() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> *)")]
pub fn stub_0x2f818c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Selection::ToggleIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::ToggleIterator)")]
pub fn stub_0x2f81a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::operator=(std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
pub fn stub_0x2f829c(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "std::_Rb_tree<rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,std::_Identity<rbx_core::SharedPtr<RBX::Instance>>,std::less<rbx_core::SharedPtr<RBX::Instance>>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_copy(std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>> const*,std::_Rb_tree_node<rbx_core::SharedPtr<RBX::Instance>>*)")]
pub fn stub_0x2f82e8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Selection::RemoveIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::RemoveIterator)")]
pub fn stub_0x2f843c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Selection::AddIterator std::__copy<false,std::bidirectional_iterator_tag>::copy<std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator>(std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_Rb_tree_const_iterator<rbx_core::SharedPtr<RBX::Instance>>,RBX::Selection::AddIterator)")]
pub fn stub_0x2f8530() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x2f8624() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BoxSelectCommand")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const")]
pub fn stub_0x2f86ec() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BoxSelectCommand")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
pub fn stub_0x2f87d0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0x2f88c8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd() [0x2f88cc]")]
pub fn stub_0x2f88cc(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
pub fn stub_0x2f88d0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0x2f88e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0x2f88f8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService(void)const")]
pub fn stub_0x2f88fc(handle: &crate::slot::InstanceHandle) {
// RBX::ServiceClient<RBX::FilteredSelection<RBX::Instance>>::createService() const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>>::operator=(rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> const&)")]
pub fn stub_0x2f89dc(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FilteredSelection<RBX::Instance>> RBX::shared_from<RBX::FilteredSelection<RBX::Instance>>(RBX::FilteredSelection<RBX::Instance>*)")]
pub fn stub_0x2f8a14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::FilteredSelection<RBX::Instance>")
}

#[doc(alias = "RBX::FilteredSelection<RBX::Instance> * RBX::ServiceProvider::create<RBX::FilteredSelection<RBX::Instance>>(RBX::Instance const*)")]
pub fn stub_0x2f8b84() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::FilteredSelection<RBX::Instance>")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> std::__find<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,rbx_core::SharedPtr<RBX::Instance const> const&,std::random_access_iterator_tag)")]
pub fn stub_0x2f8b9c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "global constructor keyed to_a_103")]
pub fn stub_0x2f8c2c() -> crate::slot::PortedFn {
// IDA 0x2f8c2c: __GLOBAL__I_a_103.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2f8c2c, "__GLOBAL__I_a_103")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void)")]
pub fn stub_0x2f8f04() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::EnumDesc(void) [0x2f8f08]")]
pub fn stub_0x2f8f08() -> crate::slot::InstanceHandle {
// RBX::Reflection::EnumDesc ctor.
crate::slot::InstanceHandle::new("RBX::Reflection::EnumDesc")
}

#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType,char const*)")]
pub fn stub_0x2f910c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::Action::ActionType>::addPair(RBX::Action::ActionType, char ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType)")]
pub fn stub_0x2f946c(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&)")]
pub fn stub_0x2f94a0(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&)")]
pub fn stub_0x2f94c8(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub fn stub_0x2f9520(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub fn stub_0x2f95d4(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Action::ActionType> const&)")]
pub fn stub_0x2f962c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&)")]
pub fn stub_0x2f9694(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long)")]
pub fn stub_0x2f9778() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *)")]
pub fn stub_0x2f9790(handle: &crate::slot::InstanceHandle) {
// RBX::Action::ActionType* std::__copy_backward<false, std::random_access_iterator_tag>::__c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&)")]
pub fn stub_0x2f97cc(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "global constructor keyed to_a_104")]
pub fn stub_0x2f995c() -> crate::slot::PortedFn {
// IDA 0x2f995c: __GLOBAL__I_a_104.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2f995c, "__GLOBAL__I_a_104")
}

#[doc(alias = "RBX::StringConverter<RBX::AnimationId>::convertToValue(std::string const&,RBX::AnimationId&)")]
pub fn stub_0x2f9a24(handle: &crate::slot::InstanceHandle) {
// RBX::StringConverter<RBX::AnimationId>::convertToValue(std::string const&, RBX::AnimationI~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::AnimationId>(void)")]
pub fn stub_0x2f9b48(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::AnimationId>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_0x2f9b4c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::readValue(RBX::Reflection::Des~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_0x2f9d34(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::writeValue(RBX::Reflection::De~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationId & RBX::Reflection::Variant::convert<RBX::AnimationId>(void)")]
pub fn stub_0x2f9edc(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationId& RBX::Reflection::Variant::convert<RBX::AnimationId>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getDataSize(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x2fa0c8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::hasStringValue(void)const")]
pub fn stub_0x2fa124(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_0x2fa128(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::Reflection::TypedPropertyDescriptor getter.
cell.get()
}

#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_0x2fa244(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::Reflection::TypedPropertyDescriptor setter.
cell.set(value)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&)")]
pub fn stub_0x2fa39c() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "RBX::AnimationId & RBX::Reflection::Variant::genericConvert<RBX::AnimationId>(void)")]
pub fn stub_0x2fa3fc(handle: &crate::slot::InstanceHandle) {
// RBX::AnimationId& RBX::Reflection::Variant::genericConvert<RBX::AnimationId>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
pub fn stub_0x2fa6a8(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x2fa700(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void)")]
pub fn stub_0x2fa7f0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::AnimationId>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::construct_func(char const*,char *)")]
pub fn stub_0x2fa85c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::AnimationId>::construct_func(char const*, char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::destruct_func(char *)")]
pub fn stub_0x2fa878(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::AnimationId>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "global constructor keyed to_a_105")]
pub fn stub_0x2fa87c() -> crate::slot::PortedFn {
// IDA 0x2fa87c: __GLOBAL__I_a_105.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x2fa87c, "__GLOBAL__I_a_105")
}

#[doc(alias = "RBX::AsyncHttpQueue::AsyncHttpQueue(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int)")]
pub fn stub_0x2faa84() -> crate::slot::InstanceHandle {
// RBX::AsyncHttpQueue ctor.
crate::slot::InstanceHandle::new("RBX::AsyncHttpQueue")
}

#[doc(alias = "RBX::AsyncHttpQueue::setThreadPool(int)")]
pub fn stub_0x2fad24(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::setThreadPool(int) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider *)")]
pub fn stub_0x2fae00(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::resetStatsItem(RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::getRequestQueueSize(void)const")]
pub fn stub_0x2faf2c(vec: &crate::slot::VecModel) -> usize {
// sequence size.
vec.len()
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue()")]
pub fn stub_0x2faf68(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue() [0x2fb008]")]
pub fn stub_0x2fb008(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::AsyncHttpQueue::~AsyncHttpQueue() [0x2fb00c]")]
pub fn stub_0x2fb00c(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x2fb2ac(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::onHeartbeat(RBX::Heartbeat const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::processRequests(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_0x2fb548() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void ()(RBX::DataModel *)>,RBX::Instance *,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_0x2fc440(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::dispatchGenericCallback(boost::function<void (RBX::DataModel*)>, RBX:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::dispatchCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_0x2fc6a8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::InvokeAsyncCallback(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x2fc874() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::AsyncHttpQueue::isRequestQueueEmpty(void)")]
pub fn stub_0x2fca04(vec: &crate::slot::VecModel) -> bool {
// sequence empty.
vec.is_empty()
}

#[doc(alias = "RBX::checkContentUrl(std::string)")]
pub fn stub_0x2fca3c() -> crate::slot::PortedFn {
// IDA 0x2fca3c: RBX::checkContentUrl(std::string).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x2fca3c, "RBX::checkContentUrl(std::string)")
}

#[doc(alias = "RBX::callback(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")]
pub fn stub_0x2fcfb4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string")
}

#[doc(alias = "RBX::AsyncHttpQueue::FailedUrl::FailedUrl(char const*)")]
pub fn stub_0x2fd150() -> crate::slot::VecModel {
// sequence ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::AsyncHttpQueue::isUrlBad(std::string const&)")]
pub fn stub_0x2fd220(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::isUrlBad(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::asyncRequest(std::string const&,float,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> *,RBX::AsyncHttpQueue::ResultJob,bool)")]
pub fn stub_0x2fd37c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::AsyncHttpQueue::syncRequest(std::string const&)")]
pub fn stub_0x2fd910(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::syncRequest(std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::operator=(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const&)")]
pub fn stub_0x2fded0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue *,RBX::Instance *)")]
pub fn stub_0x2fdf08(handle: &crate::slot::InstanceHandle) {
// RBX::HttpQueueStatsItem::create(RBX::AsyncHttpQueue*, RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_3<rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>)")]
pub fn stub_0x2fdfbc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "rbx_core::Weak<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
pub fn stub_0x2fe168() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpQueue")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x2fe358() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const")]
pub fn stub_0x2fe524(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::operator=(rbx_core::SharedPtr<RBX::Http> const&)")]
pub fn stub_0x2fe5f0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void rbx_core::SharedPtr<std::string>::reset<std::string>(std::string *)")]
pub fn stub_0x2fe628() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::operator=(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub fn stub_0x2fe654(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::AsyncHttpQueue::CallbackWrapper, std::allocator<RBX::AsyncHttpQueue::Call~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>>(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper *,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>>)")]
pub fn stub_0x2fe884() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list_av_4<boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>::type> boost::bind<void,RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>,boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>>(void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::arg<1>,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>)")]
pub fn stub_0x2fe8f4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>)")]
pub fn stub_0x2fea20(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::push_back(RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub fn stub_0x2fea58(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "RBX::AsyncHttpQueue::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x2feaa8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&)")]
pub fn stub_0x2feab0(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::AsyncHttpQueue::CallbackWrapper, std::allocator<RBX::AsyncHttpQueue::Call~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long)")]
pub fn stub_0x2fee5c(handle: &crate::slot::InstanceHandle) {
// std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper, std::allocator<RBX::AsyncHttpQueue~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> const&)")]
pub fn stub_0x2fee80(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
pub fn stub_0x2fef44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&)")]
pub fn stub_0x2ff020() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
pub fn stub_0x2ff128(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper* std::__copy_backward<false, std::random_access_itera~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&)")]
pub fn stub_0x2ff188(handle: &crate::slot::InstanceHandle) {
// std::list<RBX::AsyncHttpQueue::Request, std::allocator<RBX::AsyncHttpQueue::Request>>::_M_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::vector(std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>> const&)")]
pub fn stub_0x2ff2d4() -> crate::slot::InstanceHandle {
// std::vector ctor.
crate::slot::InstanceHandle::new("std::vector")
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&)")]
pub fn stub_0x2ff43c() -> crate::slot::InstanceHandle {
// std::_Vector_base ctor.
crate::slot::InstanceHandle::new("std::_Vector_base")
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::operator()<void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>),boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&>>(boost::_bi::type<void>,void (*)(RBX::AsyncHttpQueue::CallbackWrapper,RBX::Instance *,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string>) &,boost::_bi::list1<RBX::AsyncHttpQueue::CallbackWrapper&> &,int)")]
pub fn stub_0x2ff470(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x2ff470: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>>::list4(boost::arg<1>,boost::_bi::value<RBX::Instance *>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string>>)")]
pub fn stub_0x2ff588() -> crate::slot::BindPiece {
// boost::bind fragment (list4) composing a host BoundCall.
crate::slot::BindPiece::new("list4")
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>)")]
pub fn stub_0x2ff674(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>)")]
pub fn stub_0x2ff758(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper, st~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *)")]
pub fn stub_0x2ff8c0(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*)")]
pub fn stub_0x2ff91c(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false, std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::~vector()")]
pub fn stub_0x2ff978(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&)")]
pub fn stub_0x2ffa44(handle: &crate::slot::InstanceHandle) {
// std::list<RBX::AsyncHttpQueue::FailedUrl, std::allocator<RBX::AsyncHttpQueue::FailedUrl>>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *)")]
pub fn stub_0x2ffb24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Http")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *)")]
pub fn stub_0x2ffbfc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p()")]
pub fn stub_0x2ffd34(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::~sp_counted_impl_p() [0x2ffd38]")]
pub fn stub_0x2ffd38(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::dispose(void)")]
pub fn stub_0x2ffd3c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_deleter(std::type_info const&)")]
pub fn stub_0x2ffe10() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Http>::get_untyped_deleter(void)")]
pub fn stub_0x2ffe14() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(rbx_core::Weak<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0x2ffe1c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpQueue")
}

#[doc(alias = "rbx_core::SharedPtr<std::string>::shared_ptr<std::string>(std::string *)")]
pub fn stub_0x2ffe98() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::~sp_counted_impl_p()")]
pub fn stub_0x2fff70(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<std::string>::get_deleter(std::type_info const&)")]
pub fn stub_0x2fff78() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>)")]
pub fn stub_0x300230(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x30039c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_0x3003b8(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3003d4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x300530(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x300688(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::operator()<void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
pub fn stub_0x300798(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3008a4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
pub fn stub_0x300a60() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
pub fn stub_0x300b6c() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>)")]
pub fn stub_0x300c6c() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")]
pub fn stub_0x300f98(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3010d8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_0x3010f4(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x30110c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}
