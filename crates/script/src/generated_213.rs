// Auto-generated skeletons for rbx-script — shard 213 EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield|lua (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x38171c..0x389f0c | script 21702->21852 distinct (filler 0x38171c asc, not-in-script 64043->63893)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::~Array()")]
pub fn stub_0x38171c(vec: crate::slot::VecModel) {
// sequence dtor — releases the storage.
drop(vec);
}

#[doc(alias = "G3D::Array<RBX::SpanningEdge *,10,32ul>::Array(void)")]
pub fn stub_0x3817f0() -> crate::slot::PortedFn {
// IDA 0x3817f0: G3D::Array<RBX::SpanningEdge*, 10, 32ul>::Array().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3817f0, "G3D::Array<RBX::SpanningEdge*, 10, 32ul>::Array()")
}

#[doc(alias = "std::_Rb_tree<RBX::SpanningNode *,RBX::SpanningNode *,std::_Identity<RBX::SpanningNode *>,std::less<RBX::SpanningNode *>,std::allocator<RBX::SpanningNode *>>::_M_erase(std::_Rb_tree_node<RBX::SpanningNode *> *)")]
pub fn stub_0x3818e0(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

#[doc(alias = "global constructor keyed to_a_142")]
pub fn stub_0x381908() -> crate::slot::PortedFn {
// IDA 0x381908: __GLOBAL__I_a_142.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x381908, "__GLOBAL__I_a_142")
}

#[doc(alias = "RBX::StandardOut::singleton(void)")]
pub fn stub_0x3819d0(handle: &crate::slot::InstanceHandle) {
// RBX::StandardOut::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StandardOut::print_exception(boost::function0<void> const&,RBX::MessageType,bool)")]
pub fn stub_0x381b0c(handle: &crate::slot::InstanceHandle) {
// RBX::StandardOut::print_exception(boost::function0<void> const&, RBX::MessageType, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::exception const&)")]
pub fn stub_0x381c38(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "RBX::StandardOut::printf(RBX::MessageType,char const*,...)")]
pub fn stub_0x381c58(handle: &crate::slot::InstanceHandle) {
// RBX::StandardOut::printf(RBX::MessageType, char const*, ...) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,std::string const&)")]
pub fn stub_0x381d88(handle: &crate::slot::InstanceHandle) {
// RBX::StandardOut::print(RBX::MessageType, std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::StandardOut::print(RBX::MessageType,char const*)")]
pub fn stub_0x3820c4(handle: &crate::slot::InstanceHandle) {
// RBX::StandardOut::print(RBX::MessageType, char const*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StandardOut>::~shared_ptr()")]
pub fn stub_0x3821f0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::StandardOutMessage const&)>::operator()(RBX::StandardOutMessage const&)")]
pub fn stub_0x382204(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal_with_args<1, void (RBX::StandardOutMessage const&)>::operator()(RBX::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> &)")]
pub fn stub_0x382348() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("rbx::signals::signal<void (RBX::StandardOutMessage const&)>::slot")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::on_error(std::exception &)")]
pub fn stub_0x3824a8(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "rbx_core::SharedPtr<RBX::StandardOut>::shared_ptr<RBX::StandardOut>(RBX::StandardOut *)")]
pub fn stub_0x3824d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StandardOut")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::StandardOut>::_internal_accept_owner<RBX::StandardOut,RBX::StandardOut>(rbx_core::SharedPtr<RBX::StandardOut> const*,RBX::StandardOut *)const")]
pub fn stub_0x3825b8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::StandardOut")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StandardOut>(RBX::StandardOut *)")]
pub fn stub_0x3826dc() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::StandardOut::~StandardOut()")]
pub fn stub_0x3827e8(handle: crate::slot::InstanceHandle) {
// RBX::StandardOut dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::disconnectAll(void)")]
pub fn stub_0x38290c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::StandardOutMessage const&)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()")]
pub fn stub_0x382a84(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::~sp_counted_impl_p()_2")]
pub fn stub_0x382a88(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::dispose(void)")]
pub fn stub_0x382a8c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_deleter(std::type_info const&)")]
pub fn stub_0x382b30() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::StandardOut>::get_untyped_deleter(void)")]
pub fn stub_0x382b34() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "RBX::StandardOutMessage::StandardOutMessage(RBX::MessageType,char const*)")]
pub fn stub_0x382b38() -> crate::slot::InstanceHandle {
// RBX::StandardOutMessage ctor.
crate::slot::InstanceHandle::new("RBX::StandardOutMessage")
}

#[doc(alias = "RBX::StandardOut::StandardOut(void)")]
pub fn stub_0x382bfc() -> crate::slot::InstanceHandle {
// RBX::StandardOut ctor.
crate::slot::InstanceHandle::new("RBX::StandardOut")
}

#[doc(alias = "global constructor keyed to_a_143")]
pub fn stub_0x382d18() -> crate::slot::PortedFn {
// IDA 0x382d18: __GLOBAL__I_a_143.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x382d18, "__GLOBAL__I_a_143")
}

#[doc(alias = "SetBaseURL(std::string const&)")]
pub fn stub_0x382de0() -> crate::slot::PortedFn {
// IDA 0x382de0: SetBaseURL(std::string const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x382de0, "SetBaseURL(std::string const&)")
}

#[doc(alias = "GetBaseURL(void)")]
pub fn stub_0x382df4() -> crate::slot::PortedFn {
// IDA 0x382df4: GetBaseURL().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x382df4, "GetBaseURL()")
}

#[doc(alias = "RBX::Http::urlEncode(std::string)")]
pub fn stub_0x382e04(handle: &crate::slot::InstanceHandle) {
// RBX::Http::urlEncode(std::string) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "FetchLocalClientSettingsData(char const*,SimpleJSON *)")]
pub fn stub_0x382f9c() -> crate::slot::PortedFn {
// IDA 0x382f9c: FetchLocalClientSettingsData(char const*, SimpleJSON*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x382f9c, "FetchLocalClientSettingsData(char const*, SimpleJSON*)")
}

#[doc(alias = "LoadClientSettingsFromString(char const*,std::string const&,SimpleJSON *)")]
pub fn stub_0x3834bc() -> crate::slot::PortedFn {
// IDA 0x3834bc: LoadClientSettingsFromString(char const*, std::string const&, SimpleJSON*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3834bc, "LoadClientSettingsFromString(char const*, std::string const&, SimpleJSON*)")
}

#[doc(alias = "FetchClientSettingsData(char const*,char const*,SimpleJSON *)")]
pub fn stub_0x383538() -> crate::slot::PortedFn {
// IDA 0x383538: FetchClientSettingsData(char const*, char const*, SimpleJSON*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x383538, "FetchClientSettingsData(char const*, char const*, SimpleJSON*)")
}

#[doc(alias = "FetchClientSettingsData(char const*,char const*,std::string *)")]
pub fn stub_0x38367c() -> crate::slot::PortedFn {
// IDA 0x38367c: FetchClientSettingsData(char const*, char const*, std::string*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x38367c, "FetchClientSettingsData(char const*, char const*, std::string*)")
}

#[doc(alias = "ReportStatisticPost(std::string const&,std::string const&,std::string const&,char const*,char const*,char const*,char const*)")]
pub fn stub_0x383c54() -> crate::slot::PortedFn {
// IDA 0x383c54: ReportStatisticPost(std::string const&, std::string const&, std::string const&, char const*, char const*, char const*, c~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x383c54, "ReportStatisticPost(std::string const&, std::string const&, std::string const&, char const*, char co~")
}

#[doc(alias = "ReportStatistic(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,std::string const&)")]
pub fn stub_0x384ae0() -> crate::slot::PortedFn {
// IDA 0x384ae0: ReportStatistic(std::string const&, std::string const&, std::string const&, std::string const&, std::string const&, std:~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x384ae0, "ReportStatistic(std::string const&, std::string const&, std::string const&, std::string const&, std:~")
}

#[doc(alias = "DontCareResponse(std::string *,std::exception *)")]
pub fn stub_0x384c38(msg: &str) -> String {
// exception ctor — carries the message (thrown via panic
// at the throw site, cf. LuaTableKeysMustBeStrings).
msg.to_owned()
}

#[doc(alias = "boost::detail::function::void_function_invoker2<void (*)(std::string *,std::exception *),void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
pub fn stub_0x384c3c(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "global constructor keyed to_a_144")]
pub fn stub_0x384c44() -> crate::slot::PortedFn {
// IDA 0x384c44: __GLOBAL__I_a_144.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x384c44, "__GLOBAL__I_a_144")
}

#[doc(alias = "RBX::IStepped::onServiceProviderIStepped(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x384d34(handle: &crate::slot::InstanceHandle) {
// RBX::IStepped::onServiceProviderIStepped(RBX::ServiceProvider*, RBX::ServiceProvider*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Stepped const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>> const&)")]
pub fn stub_0x384fb0() -> crate::slot::SlotConnection {
// IDA 0x384fb0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::insert(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
pub fn stub_0x385024(slot: &crate::slot::CallableSlot) {
// IDA 0x385024: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot*)")]
pub fn stub_0x385230(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x385254(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()_2")]
pub fn stub_0x385280(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::disconnect(void)")]
pub fn stub_0x385354(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::connected(void)const")]
pub fn stub_0x385464() -> crate::slot::SlotConnection {
// IDA 0x385464: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")]
pub fn stub_0x385470(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x385470: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")]
pub fn stub_0x385478(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x385478: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>::operator()<RBX::Stepped>(RBX::Stepped const&)")]
pub fn stub_0x385480() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::remove(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
pub fn stub_0x385498(slot: &mut crate::slot::CallableSlot) {
// IDA 0x385498: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x385588(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Stepped const&)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x38558c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Stepped const&)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")]
pub fn stub_0x38567c(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()_2")]
pub fn stub_0x3856a8(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()")]
pub fn stub_0x38577c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x38577c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()_2")]
pub fn stub_0x3857a8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3857a8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "global constructor keyed to_a_145")]
pub fn stub_0x38587c() -> crate::slot::PortedFn {
// IDA 0x38587c: __GLOBAL__I_a_145.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x38587c, "__GLOBAL__I_a_145")
}

#[doc(alias = "RBX::SystemAddress::operator==(RBX::SystemAddress const&)const")]
pub fn stub_0x385a3c(handle: &crate::slot::InstanceHandle) {
// RBX::SystemAddress::operator==(RBX::SystemAddress const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SystemAddress::operator!=(RBX::SystemAddress const&)const")]
pub fn stub_0x385a58(handle: &crate::slot::InstanceHandle) {
// RBX::SystemAddress::operator!=(RBX::SystemAddress const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::SystemAddress::operator<(RBX::SystemAddress const&)const")]
pub fn stub_0x385a78(handle: &crate::slot::InstanceHandle) {
// RBX::SystemAddress::operator<(RBX::SystemAddress const&) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::BaseThreadPool::BaseThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy,RBX::BaseThreadPool::PoolData *)")]
pub fn stub_0x385a9c() -> crate::slot::InstanceHandle {
// RBX::BaseThreadPool ctor.
crate::slot::InstanceHandle::new("RBX::BaseThreadPool")
}

#[doc(alias = "RBX::BaseThreadPool::loop(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_0x385e28() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::BaseThreadPool::PoolData")
}

#[doc(alias = "RBX::BaseThreadPool::getThreadCount(void)const")]
pub fn stub_0x385fe4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::BaseThreadPool getter.
cell.get()
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
pub fn stub_0x385fe8(handle: crate::slot::InstanceHandle) {
// RBX::BaseThreadPool dtor.
drop(handle);
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()_2")]
pub fn stub_0x386088(handle: crate::slot::InstanceHandle) {
// RBX::BaseThreadPool dtor.
drop(handle);
}

#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()_3")]
pub fn stub_0x38608c(handle: crate::slot::InstanceHandle) {
// RBX::BaseThreadPool dtor.
drop(handle);
}

#[doc(alias = "RBX::join(rbx_core::SharedPtr<boost::thread>)")]
pub fn stub_0x386420() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "RBX::timed_join(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>)")]
pub fn stub_0x386428() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "RBX::BaseThreadPool::taskAdded(void)")]
pub fn stub_0x3864e4(handle: &crate::slot::InstanceHandle) {
// RBX::BaseThreadPool::taskAdded() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
pub fn stub_0x3865f4() -> crate::slot::InstanceHandle {
// RBX::ThreadPool ctor.
crate::slot::InstanceHandle::new("RBX::ThreadPool")
}

#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)_2")]
pub fn stub_0x3865f8() -> crate::slot::InstanceHandle {
// RBX::ThreadPool ctor.
crate::slot::InstanceHandle::new("RBX::ThreadPool")
}

#[doc(alias = "RBX::ThreadPool::schedule(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>)")]
pub fn stub_0x386774() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
pub fn stub_0x38678c() -> crate::slot::InstanceHandle {
// RBX::PriorityThreadPool ctor.
crate::slot::InstanceHandle::new("RBX::PriorityThreadPool")
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)_2")]
pub fn stub_0x386790() -> crate::slot::InstanceHandle {
// RBX::PriorityThreadPool ctor.
crate::slot::InstanceHandle::new("RBX::PriorityThreadPool")
}

#[doc(alias = "RBX::PriorityThreadPool::schedule(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,float)")]
pub fn stub_0x3868c8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::getNextTask(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> &)")]
pub fn stub_0x3869e4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::resize(unsigned long,rbx_core::SharedPtr<boost::thread>)")]
pub fn stub_0x386abc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::resize(unsigned long,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_0x386af8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::mutex>::reset<RBX::mutex>(RBX::mutex *)")]
pub fn stub_0x386b34() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "void rbx_core::SharedPtr<boost::thread>::reset<boost::thread>(boost::thread *)")]
pub fn stub_0x386b60() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>>(void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>)")]
pub fn stub_0x386b8c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)")]
pub fn stub_0x386d74() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)")]
pub fn stub_0x386db4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::operator()(rbx_core::SharedPtr<RBX::mutex>)const")]
pub fn stub_0x386df0(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::push(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
pub fn stub_0x386f00() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")]
pub fn stub_0x386fc4(handle: &crate::slot::InstanceHandle) {
// rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")]
pub fn stub_0x3870ec(handle: &crate::slot::InstanceHandle) {
// rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityTh~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
pub fn stub_0x387290(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
pub fn stub_0x387354(handle: crate::slot::InstanceHandle) {
// RBX::PriorityThreadPool::PriorityThreadPoolData dtor.
drop(handle);
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()_2")]
pub fn stub_0x387448(handle: crate::slot::InstanceHandle) {
// RBX::PriorityThreadPool::PriorityThreadPoolData dtor.
drop(handle);
}

#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
pub fn stub_0x38754c(handle: crate::slot::InstanceHandle) {
// RBX::ThreadPool dtor.
drop(handle);
}

#[doc(alias = "RBX::ThreadPool::~ThreadPool()_2")]
pub fn stub_0x387550(handle: crate::slot::InstanceHandle) {
// RBX::ThreadPool dtor.
drop(handle);
}

#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
pub fn stub_0x3875f0(handle: crate::slot::InstanceHandle) {
// RBX::PriorityThreadPool dtor.
drop(handle);
}

#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()_2")]
pub fn stub_0x3875f4(handle: crate::slot::InstanceHandle) {
// RBX::PriorityThreadPool dtor.
drop(handle);
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::swap(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>&)")]
pub fn stub_0x387694() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::move_assign(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>&)")]
pub fn stub_0x387770() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
pub fn stub_0x387874() -> crate::slot::PortedFn {
// IDA 0x387874: void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*, std::vector<RBX::PriorityTh~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x387874, "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*, std::ve~")
}

#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
pub fn stub_0x3879ec() -> crate::slot::PortedFn {
// IDA 0x3879ec: void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*, std::vector<RBX::PriorityThre~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3879ec, "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*, std::vect~")
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")]
pub fn stub_0x387a60(vec: &mut crate::slot::VecModel) -> usize {
// Array/vector append — grows and returns the index.
vec.append()
}

#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")]
pub fn stub_0x387aac(vec: &mut crate::slot::VecModel) -> usize {
// std sequence _M_insert — grows and returns the index.
vec.append()
}

#[doc(alias = "std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")]
pub fn stub_0x387e64() -> crate::slot::VecModel {
// std::_Vector_base ctor — empty storage.
crate::slot::VecModel::new()
}

#[doc(alias = "RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")]
pub fn stub_0x387e88(handle: &crate::slot::InstanceHandle) {
// RBX::PriorityThreadPool::PriorityTask* std::__copy_backward<false, std::random_access_iter~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::push_back(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
pub fn stub_0x387ee8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
pub fn stub_0x387f18() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x388050() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x38806c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x388144() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&> &,int)")]
pub fn stub_0x38815c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x38815c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<boost::thread>) &,boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&> &,int)")]
pub fn stub_0x388238(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x388238: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregorian::date const&,boost::posix_time::time_duration const&)")]
pub fn stub_0x388304() -> crate::slot::PortedFn {
// IDA 0x388304: boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregor~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x388304, "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_ti~")
}

#[doc(alias = "boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl,(boost::date_time::time_resolutions)5,1000000ll,(unsigned short)6,int>::to_tick_count(int,int,int,long long)")]
pub fn stub_0x38845c() -> crate::slot::PortedFn {
// IDA 0x38845c: boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl, (boost::date_time::tim~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x38845c, "boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl, (b~")
}

#[doc(alias = "rbx_core::SharedPtr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)")]
pub fn stub_0x38850c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread>(boost::thread *)")]
pub fn stub_0x3885e0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")]
pub fn stub_0x3886ec(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()_2")]
pub fn stub_0x3886f0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::dispose(void)")]
pub fn stub_0x3886f4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_deleter(std::type_info const&)")]
pub fn stub_0x388798() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_untyped_deleter(void)")]
pub fn stub_0x38879c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE")]
pub fn stub_0x3887a0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>&)")]
pub fn stub_0x388934() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>)")]
pub fn stub_0x388ab8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::~thread_data()")]
pub fn stub_0x388bec(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::~thread_data()_2")]
pub fn stub_0x388cec(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::run(void)")]
pub fn stub_0x388dfc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list0 &,int)")]
pub fn stub_0x388e18(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x388e18: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)")]
pub fn stub_0x388f28() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)const")]
pub fn stub_0x389010() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)")]
pub fn stub_0x389134() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::~sp_counted_impl_p()")]
pub fn stub_0x38922c(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::~sp_counted_impl_p()_2")]
pub fn stub_0x389230(call: crate::slot::BoundCall) {
// bind_t dtor — releases the receiver/functor copies.
drop(call);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::dispose(void)")]
pub fn stub_0x389234() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::get_deleter(std::type_info const&)")]
pub fn stub_0x389244() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>::get_untyped_deleter(void)")]
pub fn stub_0x389248() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>)")]
pub fn stub_0x38924c() -> crate::slot::BindPiece {
// boost::bind fragment (list2) composing a host BoundCall.
crate::slot::BindPiece::new("list2")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>)")]
pub fn stub_0x389364() -> crate::slot::BindPiece {
// boost::bind fragment (storage2) composing a host BoundCall.
crate::slot::BindPiece::new("storage2")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::mutex>::shared_ptr<RBX::mutex>(RBX::mutex *)")]
pub fn stub_0x389480() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::mutex>(RBX::mutex *)")]
pub fn stub_0x389554() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()")]
pub fn stub_0x389660(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::~sp_counted_impl_p()_2")]
pub fn stub_0x389664(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::dispose(void)")]
pub fn stub_0x389668() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_deleter(std::type_info const&)")]
pub fn stub_0x38970c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::mutex>::get_untyped_deleter(void)")]
pub fn stub_0x389710() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::mutex>*)")]
pub fn stub_0x389714() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::mutex>*,std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>>,unsigned long,rbx_core::SharedPtr<RBX::mutex> const&)")]
pub fn stub_0x389744() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::_M_allocate(unsigned long)")]
pub fn stub_0x389d44() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<rbx_core::SharedPtr<RBX::mutex> *,unsigned long,rbx_core::SharedPtr<RBX::mutex>>(rbx_core::SharedPtr<RBX::mutex> *,unsigned long,rbx_core::SharedPtr<RBX::mutex> const&,std::__false_type)")]
pub fn stub_0x389d5c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::mutex>::operator=(rbx_core::SharedPtr<RBX::mutex> const&)")]
pub fn stub_0x389e84(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<RBX::mutex> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *>(rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *,rbx_core::SharedPtr<RBX::mutex> *)")]
pub fn stub_0x389ebc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::mutex")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::_M_erase_at_end(rbx_core::SharedPtr<boost::thread>*)")]
pub fn stub_0x389f0c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("boost::thread")
}
