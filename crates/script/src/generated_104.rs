// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xf2cd54..0xf2dcf4 | 5111->5211 covered (filtered exhausted, global filler EA-sorted asc after 0xf2cd44), rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::string,std::string &,std::string *>,std::_Deque_iterator<std::string,std::string &,std::string *>) [0xf2cd54]")]
pub fn stub_0xf2cd54() -> crate::slot::PortedFn {
// IDA 0xf2cd54: std::deque<std::string,std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::string,std::string &,~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2cd54, "std::deque<std::string,std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::s~")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_reserve_map_at_back(unsigned long) [0xf2cd64]")]
pub fn stub_0xf2cd64(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::pop_back(void) [0xf2cd74]")]
pub fn stub_0xf2cd74(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::push_back(std::string const&) [0xf2cd84]")]
pub fn stub_0xf2cd84(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::deque(std::deque<std::string,std::allocator<std::string>> const&) [0xf2cd94]")]
pub fn stub_0xf2cd94() -> crate::slot::PortedFn {
// IDA 0xf2cd94: std::deque<std::string,std::allocator<std::string>>::deque(std::deque<std::string,std::allocator<std::string>> const&) [~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2cd94, "std::deque<std::string,std::allocator<std::string>>::deque(std::deque<std::string,std::allocator<std~")
}

#[doc(alias = "std::_Deque_iterator<std::string,std::string &,std::string *> std::__uninitialized_copy_aux<std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>>(std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>,std::__false_type) [0xf2ce04]")]
pub fn stub_0xf2ce04(vec: &crate::slot::VecModel) -> usize {
// std::__copy family — length-preserving element transfer.
vec.len()
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*) [0xf2d0c4]")]
pub fn stub_0xf2d0c4() -> crate::slot::PortedFn {
// IDA 0xf2d0c4: std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*) [0xf2d0c4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2d0c4, "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*) [0xf2d0c4]")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr() [0xf2d0d4]")]
pub fn stub_0xf2d0d4() -> crate::slot::PortedFn {
// IDA 0xf2d0d4: std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr() [0xf2d0d4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2d0d4, "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr() [0xf2d0d4]")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AdvMoveToolBase> RBX::shared_from<RBX::AdvMoveToolBase>(RBX::AdvMoveToolBase*) [0xf2d114]")]
pub fn stub_0xf2d114() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AdvMoveToolBase")
}

#[doc(alias = "RBX::DrawAdorn::resizeColor(void) [0xf2d124]")]
pub fn stub_0xf2d124(handle: &crate::slot::InstanceHandle) {
// RBX::DrawAdorn::resizeColor(void) [0xf2d124] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AxisToolBase> RBX::shared_from<RBX::AxisToolBase>(RBX::AxisToolBase*) [0xf2d184]")]
pub fn stub_0xf2d184() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AxisToolBase")
}

#[doc(alias = "std::auto_ptr<RBX::MegaDragger>::reset(RBX::MegaDragger*) [0xf2d194]")]
pub fn stub_0xf2d194() -> crate::slot::PortedFn {
// IDA 0xf2d194: std::auto_ptr<RBX::MegaDragger>::reset(RBX::MegaDragger*) [0xf2d194].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2d194, "std::auto_ptr<RBX::MegaDragger>::reset(RBX::MegaDragger*) [0xf2d194]")
}

#[doc(alias = "RBX::Extents::negativeMaxExtents(void) [0xf2d214]")]
pub fn stub_0xf2d214(handle: &crate::slot::InstanceHandle) {
// RBX::Extents::negativeMaxExtents(void) [0xf2d214] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::table(unsigned long,boost::hash<RBX::Primitive const*> const&,std::equal_to<RBX::Primitive const*> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> const&) [0xf2d224]")]
pub fn stub_0xf2d224() -> crate::slot::PortedFn {
// IDA 0xf2d224: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2d224, "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,~")
}

#[doc(alias = "RBX::POLY::Edge::getVertex(RBX::POLY::Face const*,unsigned long)const [0xf2d234]")]
pub fn stub_0xf2d234(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::POLY::Edge getter.
cell.get()
}

#[doc(alias = "std::_Vector_base<RBX::Extents,std::allocator<RBX::Extents>>::_M_allocate(unsigned long) [0xf2d244]")]
pub fn stub_0xf2d244() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Extents *,RBX::Extents *>(RBX::Extents *,RBX::Extents *,RBX::Extents *) [0xf2d254]")]
pub fn stub_0xf2d254(handle: &crate::slot::InstanceHandle) {
// RBX::Extents * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Extents*,std::vector<RBX::Extents,std::allocator<RBX::Extents>>>,RBX::Extents const&) [0xf2d264]")]
pub fn stub_0xf2d264(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::reserve(unsigned long) [0xf2d274]")]
pub fn stub_0xf2d274(vec: &mut crate::slot::VecModel, n: usize) {
// Array::realloc/reserve — capacity only grows.
vec.reserve(n);
}

#[doc(alias = "std::vector<RBX::Extents,std::allocator<RBX::Extents>>::push_back(RBX::Extents const&) [0xf2d284]")]
pub fn stub_0xf2d284(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::Joint>,std::allocator<rbx_core::SharedPtr<RBX::Joint>>>::~vector() [0xf2d4b4]")]
pub fn stub_0xf2d4b4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*) [0xf2d4e4]")]
pub fn stub_0xf2d4e4() -> crate::slot::PortedFn {
// IDA 0xf2d4e4: std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*) [0xf2d4e4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2d4e4, "std::auto_ptr<RBX::RunDragger>::reset(RBX::RunDragger*) [0xf2d4e4]")
}

#[doc(alias = "std::auto_ptr<RBX::RunDragger>::~auto_ptr() [0xf2d4f4]")]
pub fn stub_0xf2d4f4() -> crate::slot::PortedFn {
// IDA 0xf2d4f4: std::auto_ptr<RBX::RunDragger>::~auto_ptr() [0xf2d4f4].
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0xf2d4f4, "std::auto_ptr<RBX::RunDragger>::~auto_ptr() [0xf2d4f4]")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::MoveResizeJoinTool> RBX::shared_from<RBX::MoveResizeJoinTool>(RBX::MoveResizeJoinTool*) [0xf2d554]")]
pub fn stub_0xf2d554() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::MoveResizeJoinTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NewNullTool> RBX::shared_from<RBX::NewNullTool>(RBX::NewNullTool*) [0xf2d564]")]
pub fn stub_0xf2d564() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::NewNullTool")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NullTool> RBX::shared_from<RBX::NullTool>(RBX::NullTool*) [0xf2d574]")]
pub fn stub_0xf2d574() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::NullTool")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")]
pub fn stub_0xf2d584() -> crate::slot::PortedFn {
// IDA 0xf2d584: j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d584, "j___ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")]
pub fn stub_0xf2d5b4() -> crate::slot::PortedFn {
// IDA 0xf2d5b4: j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d5b4, "j___ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")]
pub fn stub_0xf2d634() -> crate::slot::PortedFn {
// IDA 0xf2d634: j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2d634, "j___ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BoxSelectCommand>::shared_ptr<RBX::BoxSelectCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter) [0xf2d664]")]
pub fn stub_0xf2d664() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BoxSelectCommand")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::BoxSelectCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter) [0xf2d684]")]
pub fn stub_0xf2d684() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::BoxSelectCommand,RBX::BoxSelectCommand>(rbx_core::SharedPtr<RBX::BoxSelectCommand> const*,RBX::BoxSelectCommand *)const [0xf2d6b4]")]
pub fn stub_0xf2d6b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BoxSelectCommand")
}

#[doc(alias = "std::_Vector_base<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_allocate(unsigned long) [0xf2d7b4]")]
pub fn stub_0xf2d7b4() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Action::ActionType *,RBX::Action::ActionType *>(RBX::Action::ActionType *,RBX::Action::ActionType *,RBX::Action::ActionType *) [0xf2d7c4]")]
pub fn stub_0xf2d7c4(handle: &crate::slot::InstanceHandle) {
// RBX::Action::ActionType * std::__copy_backward<false,std::random_access_iterator_tag>::__c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Action::ActionType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::operator[](RBX::Name const* const&) [0xf2d7d4]")]
pub fn stub_0xf2d7d4(map: &mut crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map operator[] — find path (insertion is declare).
map.find(key)
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,RBX::Action::ActionType const&) [0xf2d7e4]")]
pub fn stub_0xf2d7e4(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Action::ActionType*,std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>>,unsigned long,RBX::Action::ActionType const&) [0xf2d7f4]")]
pub fn stub_0xf2d7f4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// std sequence _M_erase/assign — removes at index.
vec.fast_remove(index)
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::resize(unsigned long,RBX::Action::ActionType) [0xf2d804]")]
pub fn stub_0xf2d804(vec: &mut crate::slot::VecModel, n: usize) {
// Array::resize — truncates or value-fills.
vec.resize(n);
}

#[doc(alias = "std::vector<RBX::Action::ActionType,std::allocator<RBX::Action::ActionType>>::push_back(RBX::Action::ActionType const&) [0xf2d814]")]
pub fn stub_0xf2d814(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Action::ActionType> const&) [0xf2d824]")]
pub fn stub_0xf2d824(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&) [0xf2d834]")]
pub fn stub_0xf2d834(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Action::ActionType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Action::ActionType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Action::ActionType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Action::ActionType> const&) [0xf2d844]")]
pub fn stub_0xf2d844(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AnimationId>(RBX::AnimationId const&) [0xf2d864]")]
pub fn stub_0xf2d864() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void) [0xf2d874]")]
pub fn stub_0xf2d874(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::AnimationId>::singleton(void) [0xf2d874] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AnimationId * rbx::any_cast<RBX::AnimationId,RBX::Region3>(rbx::placement_any<RBX::Region3> *) [0xf2d884]")]
pub fn stub_0xf2d884(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::AnimationId & rbx::any_cast<RBX::AnimationId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &) [0xf2d894]")]
pub fn stub_0xf2d894(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "RBX::HttpQueueStatsItem::init(void) [0xf2d8a4]")]
pub fn stub_0xf2d8a4(handle: &crate::slot::InstanceHandle) {
// RBX::HttpQueueStatsItem::init(void) [0xf2d8a4] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQueue*)")]
pub fn stub_0xf2d8e4(handle: &crate::slot::InstanceHandle) {
// rbx_core::WeakPtr<RBX::AsyncHttpQueue> RBX::weak_from<RBX::AsyncHttpQueue>(RBX::AsyncHttpQ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpQueue>::shared_ptr<RBX::AsyncHttpQueue>(rbx_core::WeakPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_nothrow_tag)")]
pub fn stub_0xf2d8f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpQueue")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::operator=(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const&) [0xf2d914]")]
pub fn stub_0xf2d914(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::shared_ptr<RBX::Http>(RBX::Http *) [0xf2d924]")]
pub fn stub_0xf2d924() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Http")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Http>::operator=(rbx_core::SharedPtr<RBX::Http> const&) [0xf2d934]")]
pub fn stub_0xf2d934(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "void rbx_core::SharedPtr<std::string>::reset<std::string>(std::string *) [0xf2d944]")]
pub fn stub_0xf2d944() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string")
}

#[doc(alias = "rbx_core::SharedPtr<std::string>::shared_ptr<std::string>(std::string *) [0xf2d954]")]
pub fn stub_0xf2d954() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>) [0xf2d974]")]
pub fn stub_0xf2d974() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
pub fn stub_0xf2d994() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&> &,int)")]
pub fn stub_0xf2d9a4(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf2d9a4: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>) [0xf2d9d4]")]
pub fn stub_0xf2d9d4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
pub fn stub_0xf2d9e4() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage3(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>) [0xf2d9f4]")]
pub fn stub_0xf2d9f4() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
pub fn stub_0xf2da04() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_3<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,b")]
pub fn stub_0xf2da24() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQu")]
pub fn stub_0xf2da34() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Http>(RBX::Http *) [0xf2da44]")]
pub fn stub_0xf2da44() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<RBX::AsyncHttpQueue::RequestResult>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::funct")]
pub fn stub_0xf2da74(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xf2da84(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>::operator=(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)> const&) [0xf2da94]")]
pub fn stub_0xf2da94(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2daa4() -> crate::slot::PortedFn {
// IDA 0xf2daa4: j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_L~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2daa4, "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14Asy~")
}

#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS7_5list3INS7_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2dab4() -> crate::slot::PortedFn {
// IDA 0xf2dab4: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shar~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2dab4, "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_14AsyncHttpQueue13Reques~")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(rbx_core::SharedPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")]
pub fn stub_0xf2dac4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpQueue")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::clear(void) [0xf2dad4]")]
pub fn stub_0xf2dad4(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQue")]
pub fn stub_0xf2dae4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2daf4() -> crate::slot::PortedFn {
// IDA 0xf2daf4: j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_Li~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2daf4, "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14Asyn~")
}

#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESA_SE_ENS6_5list3INS6_5valueISG_EENSK_ISA_EENSK_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf2db14() -> crate::slot::PortedFn {
// IDA 0xf2db14: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue13RequestResultEPSiNS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf2db14, "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_14AsyncHttpQueue1~")
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&) [0xf2db24]")]
pub fn stub_0xf2db24(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::move_assign(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&) [0xf2db34]")]
pub fn stub_0xf2db34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::swap(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>&) [0xf2db44]")]
pub fn stub_0xf2db44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost:")]
pub fn stub_0xf2db64(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak")]
pub fn stub_0xf2db74(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak [0xf2db84]")]
pub fn stub_0xf2db84(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::operator()(std::string const&,std::string *)const [0xf2dbc4]")]
pub fn stub_0xf2dbc4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void) [0xf2dbd4]")]
pub fn stub_0xf2dbd4(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void) [0xf2dbe4]")]
pub fn stub_0xf2dbe4(vec: &mut crate::slot::VecModel) {
// sequence clear.
vec.clear();
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long) [0xf2dbf4]")]
pub fn stub_0xf2dbf4(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::A~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**) [0xf2dc04]")]
pub fn stub_0xf2dc04(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::A~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long) [0xf2dc14]")]
pub fn stub_0xf2dc14(handle: &crate::slot::InstanceHandle) {
// std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::A~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base() [0xf2dc24]")]
pub fn stub_0xf2dc24(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate(unsigned long) [0xf2dc34]")]
pub fn stub_0xf2dc34(handle: &crate::slot::InstanceHandle) {
// std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::_Vector_base<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_Vector_base(unsigned long,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper> const&) [0xf2dc44]")]
pub fn stub_0xf2dc44() -> crate::slot::InstanceHandle {
// std::_Vector_base ctor.
crate::slot::InstanceHandle::new("std::_Vector_base")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *) [0xf2dc54]")]
pub fn stub_0xf2dc54(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper * std::__copy_backward<false,std::random_access_itera~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_create_node(RBX::AsyncHttpQueue::Request const&) [0xf2dc64]")]
pub fn stub_0xf2dc64(handle: &crate::slot::InstanceHandle) {
// std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_erase(std::_List_iterator<RBX::AsyncHttpQueue::Request>) [0xf2dc74]")]
pub fn stub_0xf2dc74(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_create_node(RBX::AsyncHttpQueue::FailedUrl const&) [0xf2dc84]")]
pub fn stub_0xf2dc84(handle: &crate::slot::InstanceHandle) {
// std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::list<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::erase(std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>,std::_List_iterator<RBX::AsyncHttpQueue::FailedUrl>) [0xf2dc94]")]
pub fn stub_0xf2dc94(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void) [0xf2dca4]")]
pub fn stub_0xf2dca4(vec: &mut crate::slot::VecModel, index: usize) -> bool {
// IndexArray::fastRemove — swap-with-last.
vec.fast_remove(index)
}

#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&) [0xf2dcb4]")]
pub fn stub_0xf2dcb4() -> crate::slot::InstanceHandle {
// std::deque ctor.
crate::slot::InstanceHandle::new("std::deque")
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*>(RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper const*,RBX::AsyncHttpQueue::CallbackWrapper*) [0xf2dcc4]")]
pub fn stub_0xf2dcc4(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper* std::__copy<false,std::random_access_iterator_tag>::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *>(RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *,RBX::AsyncHttpQueue::CallbackWrapper *) [0xf2dcd4]")]
pub fn stub_0xf2dcd4(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper * std::__copy<false,std::random_access_iterator_tag>:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,RBX::AsyncHttpQueue::CallbackWrapper const&) [0xf2dce4]")]
pub fn stub_0xf2dce4(handle: &crate::slot::InstanceHandle) {
// std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::Callb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>::_M_allocate_and_copy<__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>>(unsigned long,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper const*,std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std::allocator<RBX::AsyncHttpQueue::CallbackWrapper>>>,__gnu_cxx::__normal_iterator<RBX::AsyncHttpQueue::CallbackWrapper co")]
pub fn stub_0xf2dcf4(handle: &crate::slot::InstanceHandle) {
// RBX::AsyncHttpQueue::CallbackWrapper* std::vector<RBX::AsyncHttpQueue::CallbackWrapper,std~ — engine-side; linkage preserved via the alias.
let _ = handle;
}
