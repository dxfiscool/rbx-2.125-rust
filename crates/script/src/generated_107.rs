// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0xf30034..0xf401c4 | 5117->5217 covered (filtered), 184 remaining, rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&) [0xf30034]")]
pub fn stub_0xf30034(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *) [0xf30044]")]
pub fn stub_0xf30044(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&) [0xf30094]")]
pub fn stub_0xf30094(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct(void) [0xf300a4]")]
pub fn stub_0xf300a4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::~node_constructor() [0xf300b4]")]
pub fn stub_0xf300b4(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *) [0xf300f4]")]
pub fn stub_0xf300f4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *) [0xf30104]")]
pub fn stub_0xf30104(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long) [0xf30114]")]
pub fn stub_0xf30114(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void) [0xf30124]")]
pub fn stub_0xf30124(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long) [0xf30134]")]
pub fn stub_0xf30134(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void) [0xf30144]")]
pub fn stub_0xf30144(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> const&) [0xf30154]")]
pub fn stub_0xf30154() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>> const*,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)const [0xf301e4]")]
pub fn stub_0xf301e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf301f4]")]
pub fn stub_0xf301f4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const [0xf30204]")]
pub fn stub_0xf30204(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf30214]")]
pub fn stub_0xf30214(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const [0xf30224]")]
pub fn stub_0xf30224(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &)const [0xf30234]")]
pub fn stub_0xf30234(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const [0xf30244]")]
pub fn stub_0xf30244(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::operator()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)const [0xf30254]")]
pub fn stub_0xf30254(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const [0xf30264]")]
pub fn stub_0xf30264(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const [0xf30284]")]
pub fn stub_0xf30284(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const [0xf30294]")]
pub fn stub_0xf30294(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_clear(void) [0xf302c4]")]
pub fn stub_0xf302c4(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>> const&) [0xf302e4]")]
pub fn stub_0xf302e4() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf32214]")]
pub fn stub_0xf32214() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::BadgeService", "bool", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::BoundYieldFuncDesc(void (RBX::BadgeService::*)(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf32224]")]
pub fn stub_0xf32224() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::BadgeService", "bool", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf32234]")]
pub fn stub_0xf32234() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::BadgeService", "bool", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::BoundYieldFuncDesc(void (RBX::BadgeService::*)(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf32244]")]
pub fn stub_0xf32244() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::BadgeService", "bool", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf359e4]")]
pub fn stub_0xf359e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(RBX::Instance::SaveFilter),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Instance::SaveFilter,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf359f4]")]
pub fn stub_0xf359f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::declareSignature(char const*,RBX::Reflection::Variant) [0xf35a04]")]
pub fn stub_0xf35a04() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::DataModel", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string),std::string,1>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf35a14]")]
pub fn stub_0xf35a14() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::DataModel", "std::string", 1)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf35a24]")]
pub fn stub_0xf35a24() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::DataModel", "std::string", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,std::string ()(std::string,std::string),std::string,2>::BoundYieldFuncDesc(void (RBX::DataModel::*)(std::string,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf35a34]")]
pub fn stub_0xf35a34() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::DataModel", "std::string", 2)
}

#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::DataModel,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::DataModel::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf35a44]")]
pub fn stub_0xf35a44() -> crate::slot::DescriptorHandle {
// BoundYieldFuncDesc ctor — registers the yielding member binding.
crate::slot::DescriptorHandle::yield_func("RBX::DataModel", "bool", 0)
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScriptEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf35c74() -> crate::slot::PortedFn {
// IDA 0xf35c74: j___ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8InstanceELZNS_11sBaseScr~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf35c74, "j___ZN3RBX10Reflection9DescribedINS_10BaseScriptELZNS_11sBaseScriptEENS_17NonFactoryProductINS_8Inst~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf35c84() -> crate::slot::PortedFn {
// IDA 0xf35c84: j___ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sS~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf35c84, "j___ZN3RBX10Reflection9DescribedINS_13ScriptServiceELZNS_14sScriptServiceEENS_17NonFactoryProductINS~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev")]
pub fn stub_0xf35ec4() -> crate::slot::PortedFn {
// IDA 0xf35ec4: j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf35ec4, "j___ZN3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7CreatorC2Ev")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15isNullClassNameEv")]
pub fn stub_0xf35f94() -> crate::slot::PortedFn {
// IDA 0xf35f94: j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15isNullClassNameEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf35f94, "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E15~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17static_getCreatorEv")]
pub fn stub_0xf35fa4() -> crate::slot::PortedFn {
// IDA 0xf35fa4: j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17static_getCreatorEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf35fa4, "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E17~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorC2Ev")]
pub fn stub_0xf35fb4() -> crate::slot::PortedFn {
// IDA 0xf35fb4: j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorC2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf35fb4, "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7C~")
}

#[doc(alias = "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD2Ev")]
pub fn stub_0xf35fc4() -> crate::slot::PortedFn {
// IDA 0xf35fc4: j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7CreatorD2Ev.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf35fc4, "j___ZN3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7C~")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptContext>(void) [0xf36154]")]
pub fn stub_0xf36154() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ServerScriptService>(void) [0xf36234]")]
pub fn stub_0xf36234() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ScriptInformationProvider>(void) [0xf36284]")]
pub fn stub_0xf36284() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("Service"))
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v")]
pub fn stub_0xf36594() -> crate::slot::PortedFn {
// IDA 0xf36594: j___ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf36594, "j___ZN3RBX4Name7declareILZNS_20sServerScriptServiceEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v")]
pub fn stub_0xf365e4() -> crate::slot::PortedFn {
// IDA 0xf365e4: j___ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf365e4, "j___ZN3RBX4Name7declareILZNS_26sScriptInformationProviderEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v")]
pub fn stub_0xf36794() -> crate::slot::PortedFn {
// IDA 0xf36794: j___ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf36794, "j___ZN3RBX4Name9doDeclareILZNS_20sServerScriptServiceEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v")]
pub fn stub_0xf367d4() -> crate::slot::PortedFn {
// IDA 0xf367d4: j___ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf367d4, "j___ZN3RBX4Name9doDeclareILZNS_26sScriptInformationProviderEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ScriptService>(void) [0xf368c4]")]
pub fn stub_0xf368c4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService> RBX::Creatable<RBX::Instance>::create<RBX::ServerScriptService>(void) [0xf36994]")]
pub fn stub_0xf36994() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService>::shared_ptr<RBX::ScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf36f94]")]
pub fn stub_0xf36f94() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ServerScriptService>::shared_ptr<RBX::ServerScriptService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf370a4]")]
pub fn stub_0xf370a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider>::shared_ptr<RBX::ScriptInformationProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter) [0xf370f4]")]
pub fn stub_0xf370f4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const&) [0xf371f4]")]
pub fn stub_0xf371f4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalScript *,RBX::Creatable<RBX::Instance>::Deleter) [0xf37864]")]
pub fn stub_0xf37864() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf378f4]")]
pub fn stub_0xf378f4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ServerScriptService *,RBX::Creatable<RBX::Instance>::Deleter) [0xf379d4]")]
pub fn stub_0xf379d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "j___ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_0xf37f64() -> crate::slot::PortedFn {
// IDA 0xf37f64: j___ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7Creator12getClassNam~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf37f64, "j___ZNK3RBX14FactoryProductINS_19ServerScriptServiceENS_8InstanceELZNS_20sServerScriptServiceEES2_E7~")
}

#[doc(alias = "RBX::ServerScriptService * RBX::ServiceProvider::find<RBX::ServerScriptService>(void)const [0xf380e4]")]
pub fn stub_0xf380e4() -> Option<crate::slot::ServiceHandle> {
// ServiceProvider::find — the provider always hosts core
// services in this build.
Some(crate::slot::ServiceHandle::new("RBX::ServerScriptService"))
}

#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::create<RBX::ScriptService>(void)const [0xf381f4]")]
pub fn stub_0xf381f4() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptService")
}

#[doc(alias = "int RBX::Instance::countDescendantsOfType<RBX::BaseScript>(void)const [0xf38344]")]
pub fn stub_0xf38344(handle: &crate::slot::InstanceHandle) {
// int RBX::Instance::countDescendantsOfType<RBX::BaseScript>(void)const [0xf38344] — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScriptService,RBX::ScriptService>(rbx_core::SharedPtr<RBX::ScriptService> const*,RBX::ScriptService *)const [0xf38474]")]
pub fn stub_0xf38474() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptService")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerScriptService,RBX::ServerScriptService>(rbx_core::SharedPtr<RBX::ServerScriptService> const*,RBX::ServerScriptService *)const [0xf38564]")]
pub fn stub_0xf38564() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ServerScriptService")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::~vector() [0xf38bd4]")]
pub fn stub_0xf38bd4(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf3b2d4() -> crate::slot::PortedFn {
// IDA 0xf3b2d4: j___ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sLuaDrag~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3b2d4, "j___ZN3RBX10Reflection9DescribedINS_10LuaDraggerELZNS_11sLuaDraggerEENS_14FactoryProductIS2_NS_8Inst~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_11LocalScriptELZNS_12sLocalScriptEENS_14FactoryProductIS2_NS_6ScriptELZNS_12sLocalScriptEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf3b364() -> crate::slot::PortedFn {
// IDA 0xf3b364: j___ZN3RBX10Reflection9DescribedINS_11LocalScriptELZNS_12sLocalScriptEENS_14FactoryProductIS2_NS_6ScriptELZNS_12sLocalSc~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3b364, "j___ZN3RBX10Reflection9DescribedINS_11LocalScriptELZNS_12sLocalScriptEENS_14FactoryProductIS2_NS_6Sc~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_11LuaSettingsELZNS_12sLuaSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf3b374() -> crate::slot::PortedFn {
// IDA 0xf3b374: j___ZN3RBX10Reflection9DescribedINS_11LuaSettingsELZNS_12sLuaSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSetting~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3b374, "j___ZN3RBX10Reflection9DescribedINS_11LuaSettingsELZNS_12sLuaSettingsEENS_14FactoryProductIS2_NS_22G~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sAdvLuaDraggerEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf3b524() -> crate::slot::PortedFn {
// IDA 0xf3b524: j___ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sA~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3b524, "j___ZN3RBX10Reflection9DescribedINS_13AdvLuaDraggerELZNS_14sAdvLuaDraggerEENS_14FactoryProductIS2_NS~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf3b5f4() -> crate::slot::PortedFn {
// IDA 0xf3b5f4: j___ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sL~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3b5f4, "j___ZN3RBX10Reflection9DescribedINS_13LuaWebServiceELZNS_14sLuaWebServiceEENS_17NonFactoryProductINS~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8InstanceELZNS_21sRuntimeScriptServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf3b8a4() -> crate::slot::PortedFn {
// IDA 0xf3b8a4: j___ZN3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFactoryProductINS_8Inst~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3b8a4, "j___ZN3RBX10Reflection9DescribedINS_20RuntimeScriptServiceELZNS_21sRuntimeScriptServiceEENS_17NonFac~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0xf3b904() -> crate::slot::PortedFn {
// IDA 0xf3b904: j___ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEENS_17NonFactoryProduc~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3b904, "j___ZN3RBX10Reflection9DescribedINS_25ScriptInformationProviderELZNS_26sScriptInformationProviderEEN~")
}

#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8InstanceELZNS2_16sDebuggerManagerEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv")]
pub fn stub_0xf3bd24() -> crate::slot::PortedFn {
// IDA 0xf3bd24: j___ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFactoryProductINS_8Ins~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf3bd24, "j___ZN3RBX10Reflection9DescribedINS_9Scripting15DebuggerManagerELZNS2_16sDebuggerManagerEENS_17NonFa~")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptInformationProvider> RBX::Creatable<RBX::Instance>::create<RBX::ScriptInformationProvider>(void) [0xf3ea24]")]
pub fn stub_0xf3ea24() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const&) [0xf3eae4]")]
pub fn stub_0xf3eae4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScriptInformationProvider *,RBX::Creatable<RBX::Instance>::Deleter) [0xf3eb74]")]
pub fn stub_0xf3eb74() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::ScriptInformationProvider * RBX::ServiceProvider::create<RBX::ScriptInformationProvider>(void)const [0xf3ebd4]")]
pub fn stub_0xf3ebd4() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::Reflection::Call6Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef,bool>::call(RBX::GuiObject*,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,RBX::UDim2 const&,RBX::GuiObject::TweenEasingDirection const&,RBX::GuiObject::TweenEasingStyle const&,float const&,bool const&,RBX::Lua::WeakFunctionRef const&) [0xf3f7d4]")]
pub fn stub_0xf3f7d4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call6Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiOb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::Call7Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef,bool>::call(RBX::GuiObject*,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,RBX::UDim2 const&,RBX::UDim2 const&,RBX::GuiObject::TweenEasingDirection const&,RBX::GuiObject::TweenEasingStyle const&,float const&,bool const&,RBX::Lua::WeakFunctionRef const&) [0xf3f7e4]")]
pub fn stub_0xf3f7e4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call7Helper<RBX::GuiObject,bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf3f814]")]
pub fn stub_0xf3f814() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 6)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::BoundFuncDesc(bool (RBX::GuiObject::*)(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,RBX::GuiObject::TweenEasingDirection,char const*,RBX::GuiObject::TweenEasingStyle,char const*,float,char const*,bool,char const*,RBX::Lua::WeakFunctionRef,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf3f824]")]
pub fn stub_0xf3f824() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 6)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),6>::~BoundFuncDesc() [0xf3f834]")]
pub fn stub_0xf3f834(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant) [0xf3f844]")]
pub fn stub_0xf3f844() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 7)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::BoundFuncDesc(bool (RBX::GuiObject::*)(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,char const*,RBX::GuiObject::TweenEasingDirection,char const*,RBX::GuiObject::TweenEasingStyle,char const*,float,char const*,bool,char const*,RBX::Lua::WeakFunctionRef,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes) [0xf3f854]")]
pub fn stub_0xf3f854() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::GuiObject", "bool", 7)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiObject,bool ()(RBX::UDim2,RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,RBX::Lua::WeakFunctionRef),7>::~BoundFuncDesc() [0xf3f864]")]
pub fn stub_0xf3f864(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,6>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *) [0xf3fa04]")]
pub fn stub_0xf3fa04() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,7>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *) [0xf3fa14]")]
pub fn stub_0xf3fa14() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Reflection::Tuple const")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>) [0xf3ff44]")]
pub fn stub_0xf3ff44() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list1<RBX::GuiObject::TweenStatus &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef) &,boost::_bi::list1<RBX::GuiObject::TweenStatus &> &,int) [0xf3ff54]")]
pub fn stub_0xf3ff54(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf3ff54: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>) [0xf3ff74]")]
pub fn stub_0xf3ff74() -> crate::slot::BindPiece {
// boost::bind fragment (list3) composing a host BoundCall.
crate::slot::BindPiece::new("list3")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::GuiObject::TweenStatus&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::GuiObject::TweenStatus&> &,int) [0xf3ff84]")]
pub fn stub_0xf3ff84(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0xf3ff84: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>) [0xf40034]")]
pub fn stub_0xf40034() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,boost::arg<1>) [0xf40074]")]
pub fn stub_0xf40074() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>::type> boost::bind<void,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef>(void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef) [0xf40084]")]
pub fn stub_0xf40084() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf400d4]")]
pub fn stub_0xf400d4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>) [0xf400e4]")]
pub fn stub_0xf400e4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefEENS7_5list2INS7_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf40104() -> crate::slot::PortedFn {
// IDA 0xf40104: j___ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRef~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf40104, "j___ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3~")
}

#[doc(alias = "j___ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0xf40114() -> crate::slot::PortedFn {
// IDA 0xf40114: j___ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRef~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf40114, "j___ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3~")
}

#[doc(alias = "void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>>) [0xf401c4]")]
pub fn stub_0xf401c4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}
