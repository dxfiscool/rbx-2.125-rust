// Auto-generated skeletons for rbx-script — Script/Lua batch
// Filter: Script|Lua (4456 filtered, 1759 remaining global-free before batch, 1659 after)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x355d60..0x436c10 EA-sorted asc next 100 Script|Lua not yet in any crate (global-free 1759-> 1659, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>> const&)")]
pub fn stub_0x355d60(handle: &crate::slot::InstanceHandle) {
// std::list<std::pair<std::string, std::pair<unsigned long, RBX::LuaWebService::CachedLuaWeb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::~LRUCache()")]
pub fn stub_0x355e88(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
pub fn stub_0x355f9c(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_clear(void)")]
pub fn stub_0x356010(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x3560f8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x356130(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::LRUCache(void)")]
pub fn stub_0x356164() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
pub fn stub_0x356244(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> const&)")]
pub fn stub_0x3562bc() -> crate::slot::InstanceHandle {
// boost::unordered::detail::table ctor.
crate::slot::InstanceHandle::new("boost::unordered::detail::table")
}

#[doc(alias = "RBX::ScriptInformationProvider::ScriptInformationProvider(void)")]
pub fn stub_0x36a44c() -> crate::slot::InstanceHandle {
// RBX::ScriptInformationProvider ctor.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::ScriptInformationProvider::ScriptInformationProvider(void) [0x36a450]")]
pub fn stub_0x36a450() -> crate::slot::InstanceHandle {
// RBX::ScriptInformationProvider ctor.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::ScriptInformationProvider::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x36a714(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider::onHeartbeat(RBX::Heartbeat const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_0x36a71c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "RBX::ScriptInformationProvider::HandleHttpResponse(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>)")]
pub fn stub_0x36a724() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::ScriptInformationProvider::getScriptInfo(std::string const&,bool,float,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::AsyncHttpQueue::ResultJob)")]
pub fn stub_0x36a87c(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider::getScriptInfo(std::string const&, bool, float, boost::func~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptInformationProvider::CachedScriptInfo::CachedScriptInfo(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x36b030() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::ScriptInformationProvider::setAssetUrl(std::string)")]
pub fn stub_0x36b240(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider::setAssetUrl(std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ScriptInformationProvider::setAccessKey(std::string)")]
pub fn stub_0x36b33c(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider::setAccessKey(std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::reset<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
pub fn stub_0x36b344() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo, false>")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::findCacheItem(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo*)")]
pub fn stub_0x36b454(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::operator()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)const")]
pub fn stub_0x36b568(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list_av_5<RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::type> boost::bind<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>(boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)")]
pub fn stub_0x36b678() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list_av_4<rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>(void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),rbx_core::WeakPtr<RBX::ScriptInformationProvider>,boost::arg<1>,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>)")]
pub fn stub_0x36b76c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptInformationProvider> RBX::weak_from<RBX::ScriptInformationProvider>(RBX::ScriptInformationProvider*)")]
pub fn stub_0x36bac4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider()")]
pub fn stub_0x36bcbc(handle: crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider dtor.
drop(handle);
}

#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider() [0x36bcc0]")]
pub fn stub_0x36bcc0(handle: crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider dtor.
drop(handle);
}

#[doc(alias = "RBX::ScriptInformationProvider::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x36bd64(handle: &crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider::onServiceProvider(RBX::ServiceProvider*, RBX::ServiceProvi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv")]
pub fn stub_0x36bd70() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider()")]
pub fn stub_0x36bd9c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider() [0x36bda4]")]
pub fn stub_0x36bda4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_26sScriptInformationProviderEEE12getClassNameEv")]
pub fn stub_0x36be48() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider() [0x36be70]")]
pub fn stub_0x36be70(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider() [0x36be78]")]
pub fn stub_0x36be78(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider() [0x36bf20]")]
pub fn stub_0x36bf20(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "non-virtual thunk toRBX::ScriptInformationProvider::~ScriptInformationProvider() [0x36bf28]")]
pub fn stub_0x36bf28(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 96, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 96);
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_26sScriptInformationProviderEEEEvv")]
pub fn stub_0x36bfd0(handle: &crate::slot::InstanceHandle) {
// void RBX::Name::callDoDeclare<RBX::sScriptInformationProvider>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS0_IFvNSE_13RequestResultEbbfbEEEENSB_5list4INSB_5valueISF_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x36bfd8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_25ScriptInformationProviderEEES3_SsNS_8functionIFvNSD_13RequestResultEbbfbEEEENSA_5list4INSA_5valueISE_EENS_3argILi1EEENSM_ISsEENSM_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x36c200() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>> const&)")]
pub fn stub_0x36c42c() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>)")]
pub fn stub_0x36c5a4(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x36c7e0(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x36c7fc(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x36c820(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x36ca44(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x36cc64(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
pub fn stub_0x36cd24(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptInformationProvider>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>),boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x36cf0c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::_bi::list4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::list4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>)")]
pub fn stub_0x36d0dc() -> crate::slot::BindPiece {
// boost::bind fragment (list4) composing a host BoundCall.
crate::slot::BindPiece::new("list4")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>>)")]
pub fn stub_0x36d2b8() -> crate::slot::BindPiece {
// boost::bind fragment (storage4) composing a host BoundCall.
crate::slot::BindPiece::new("storage4")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>,boost::_bi::value<std::string>)")]
pub fn stub_0x36d4c0() -> crate::slot::BindPiece {
// boost::bind fragment (storage3) composing a host BoundCall.
crate::slot::BindPiece::new("storage3")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::arg<1>)")]
pub fn stub_0x36d600() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptInformationProvider>::weak_ptr<RBX::ScriptInformationProvider>(rbx_core::SharedPtr<RBX::ScriptInformationProvider> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptInformationProvider,RBX::ScriptInformationProvider>::type)")]
pub fn stub_0x36d748() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::ScriptInformationProvider")
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS7_5list5INS7_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x36d798() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x36d870() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x36da30(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x36dce8(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to_own(boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool> const&)")]
pub fn stub_0x36de2c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
pub fn stub_0x36ed5c(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x36ed98(map: &crate::slot::TreeMapModel, key: &str) -> Option<String> {
// map find_node_impl — hashes the key and probes the
// bucket run (cf. 0x263574).
map.find(key)
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
pub fn stub_0x36ee08() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo, false>")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>> const*,RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)const")]
pub fn stub_0x36eef0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo, false>")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>(RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false> *)")]
pub fn stub_0x36f018() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p()")]
pub fn stub_0x36f110(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::~sp_counted_impl_p() [0x36f114]")]
pub fn stub_0x36f114(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::dispose(void)")]
pub fn stub_0x36f118() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_deleter(std::type_info const&)")]
pub fn stub_0x36f12c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>>::get_untyped_deleter(void)")]
pub fn stub_0x36f130() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache()")]
pub fn stub_0x36f284(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::~AsyncHttpCache() [0x36f38c]")]
pub fn stub_0x36f38c(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
pub fn stub_0x36f4a4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::string const")
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")]
pub fn stub_0x36f694(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::insert(std::string const&,RBX::ScriptInformationProvider::CachedScriptInfo const&,unsigned long)")]
pub fn stub_0x36f6c8(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::removeLeastRecentlyUsed(void)")]
pub fn stub_0x36fadc(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::remove(std::string const&)")]
pub fn stub_0x36fb34(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> *)")]
pub fn stub_0x36fb88(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36fbe4(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x36fc10(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
pub fn stub_0x36fc50(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>> const&)")]
pub fn stub_0x36fe00(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x36fe24(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::~node_constructor()")]
pub fn stub_0x36fe74(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x36fe90(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x36ffb8(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x370048(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x370074(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>>>::construct(void)")]
pub fn stub_0x3700cc() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>> const&)")]
pub fn stub_0x370108() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::~LRUCache()")]
pub fn stub_0x3701ec(map: crate::slot::TreeMapModel) {
// ordered/unordered map dtor — releases every node.
drop(map);
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")]
pub fn stub_0x370300(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>::_M_clear(void)")]
pub fn stub_0x370338(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x370360(map: &mut crate::slot::TreeMapModel) {
// map rehash_impl — reallocates and re-links every node
// (cf. 0x2634b8); capacity is host-managed.
let _ = map;
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x370398(map: &mut crate::slot::TreeMapModel) {
// map clear — releases every node.
map.clear();
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::LRUCache(void)")]
pub fn stub_0x3703cc() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::ScriptInformationProvider::CachedScriptInfo>::resize(unsigned long)")]
pub fn stub_0x3704ac(map: &crate::slot::TreeMapModel) -> usize {
// map size.
map.len()
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::ScriptInformationProvider::CachedScriptInfo>>>>>> const&)")]
pub fn stub_0x370530() -> crate::slot::TreeMapModel {
// script-info LRU cache ctor.
crate::slot::TreeMapModel::new()
}

#[doc(alias = "RBX::ScriptInformationProvider::~ScriptInformationProvider() [0x371070]")]
pub fn stub_0x371070(handle: crate::slot::InstanceHandle) {
// RBX::ScriptInformationProvider dtor.
drop(handle);
}

#[doc(alias = "boost::function5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::clear(void)")]
pub fn stub_0x371220(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "RBX::Backpack::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x3b1014(handle: &crate::slot::InstanceHandle) {
// RBX::Backpack::scriptShouldRun(RBX::BaseScript*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "non-virtual thunk toRBX::Backpack::scriptShouldRun(RBX::BaseScript *)")]
pub fn stub_0x3b1218(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 148, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 148);
}

#[doc(alias = "RBX::ScriptService * RBX::ServiceProvider::create<RBX::ScriptService>(void)const")]
pub fn stub_0x436c10() -> crate::slot::ServiceHandle {
// ServiceProvider::create — constructs + registers.
crate::slot::ServiceHandle::new("RBX::ScriptService")
}
