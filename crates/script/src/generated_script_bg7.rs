// Auto-generated skeletons for rbx-script — script-bg filler EA-sorted asc next 120 not yet in script
// Filter: Script|Lua|Yield|CodeGen (5401 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs | range 0x589f58..0x590904 | script 19301->19421 total (gap filler 0x589f58 asc, not-in-script)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_0x589f58() -> crate::slot::SlotConnection {
// IDA 0x589f58: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0x589fcc(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot() [0x589ff8]")]
pub fn stub_0x589ff8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
pub fn stub_0x58a0cc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58a0cc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
pub fn stub_0x58a0e8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58a0e8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list2<std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string> &,boost::_bi::list2<std::string &,std::string &> &,int)")]
pub fn stub_0x58a104(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x58a104: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>::operator()(RBX::InsertService*,std::string,std::string)const")]
pub fn stub_0x58a2ac() -> crate::slot::BindPiece {
// boost::bind fragment (mf2) composing a host BoundCall.
crate::slot::BindPiece::new("mf2")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")]
pub fn stub_0x58a470(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58a470: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable() [0x58a49c]")]
pub fn stub_0x58a49c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58a49c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_0x58a570() -> crate::slot::SlotConnection {
// IDA 0x58a570: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot*)")]
pub fn stub_0x58a5e4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0x58a608(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot() [0x58a634]")]
pub fn stub_0x58a634(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::connected(void)const")]
pub fn stub_0x58a708() -> crate::slot::SlotConnection {
// IDA 0x58a708: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x58a714(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58a714: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::call(std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x58a730(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58a730: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<std::string &,rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list2<std::string &,rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
pub fn stub_0x58a74c(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x58a74c: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::InsertService*,std::string,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_0x58a8c0() -> crate::slot::BindPiece {
// boost::bind fragment (mf2) composing a host BoundCall.
crate::slot::BindPiece::new("mf2")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x58aa4c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x58aa50() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
pub fn stub_0x58ab40(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_0x58ac14(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58ac14: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable() [0x58ac40]")]
pub fn stub_0x58ac40(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58ac40: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "std::pair<std::string const,RBX::InsertService::Callback>::pair(std::string const&,RBX::InsertService::Callback const&)")]
pub fn stub_0x58ad14() -> (String, String) {
// std::pair ctor — empty pair.
(String::new(), String::new())
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InsertService::Callback>>,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
pub fn stub_0x58ae00(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
pub fn stub_0x58aeec(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
pub fn stub_0x58af3c(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_create_node(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
pub fn stub_0x58afc0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::lower_bound(std::string const&)")]
pub fn stub_0x58b0f0(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")]
pub fn stub_0x58b120() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
pub fn stub_0x58b124(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::find(std::string const&)")]
pub fn stub_0x58b204(map: &mut crate::slot::TreeMapModel, key: &str, value: &str) -> bool {
// map declare/insert — lower-bound + sorted insert (cf.
// 0x260638); true when the key was new.
map.insert(key, value)
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_0x58b254() -> crate::slot::SlotConnection {
// IDA 0x58b254: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,int)>::slot*)")]
pub fn stub_0x58b2c8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_0x58b2ec(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot() [0x58b318]")]
pub fn stub_0x58b318(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::disconnect(void)")]
pub fn stub_0x58b3ec(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::connected(void)const")]
pub fn stub_0x58b4fc() -> crate::slot::SlotConnection {
// IDA 0x58b4fc: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
pub fn stub_0x58b508(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58b508: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
pub fn stub_0x58b530(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58b530: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int> &,boost::_bi::list3<std::string &,int &,int &> &,int)")]
pub fn stub_0x58b558(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x58b558: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>::operator()(RBX::InsertService*,std::string,int,int)const")]
pub fn stub_0x58b688() -> crate::slot::BindPiece {
// boost::bind fragment (mf3) composing a host BoundCall.
crate::slot::BindPiece::new("mf3")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x58b7c4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, int, int)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::~slot()")]
pub fn stub_0x58b8b4(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")]
pub fn stub_0x58b8e0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58b8e0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable() [0x58b90c]")]
pub fn stub_0x58b90c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58b90c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_0x58b9e0() -> crate::slot::SlotConnection {
// IDA 0x58b9e0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::insert(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
pub fn stub_0x58ba54(slot: &crate::slot::CallableSlot) {
// IDA 0x58ba54: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot*)")]
pub fn stub_0x58bc60(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> const&)")]
pub fn stub_0x58bc84(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_init_mutex(void)")]
pub fn stub_0x58bca8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, RBX::ContentId)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x58bcac(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, RBX::ContentId)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0x58bda4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot() [0x58bdd0]")]
pub fn stub_0x58bdd0(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::disconnect(void)")]
pub fn stub_0x58bea4(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::connected(void)const")]
pub fn stub_0x58bfb4() -> crate::slot::SlotConnection {
// IDA 0x58bfb4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
pub fn stub_0x58bfc0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58bfc0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
pub fn stub_0x58bfdc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58bfdc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list2<std::string &,RBX::ContentId&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId> &,boost::_bi::list2<std::string &,RBX::ContentId&> &,int)")]
pub fn stub_0x58bff8(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x58bff8: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>::operator()(RBX::InsertService*,std::string,RBX::ContentId)const")]
pub fn stub_0x58c1a8() -> crate::slot::BindPiece {
// boost::bind fragment (mf2) composing a host BoundCall.
crate::slot::BindPiece::new("mf2")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::remove(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
pub fn stub_0x58c374(slot: &mut crate::slot::CallableSlot) {
// IDA 0x58c374: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x58c464(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, RBX::ContentId)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x58c468(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, RBX::ContentId)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot()")]
pub fn stub_0x58c558(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot() [0x58c584]")]
pub fn stub_0x58c584(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
pub fn stub_0x58c658(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58c658: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable() [0x58c684]")]
pub fn stub_0x58c684(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x58c684: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>> const&)")]
pub fn stub_0x58c758(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::remote_signal(void)")]
pub fn stub_0x58c914() -> crate::slot::PortedFn {
// IDA 0x58c914: rbx::remote_signal<void (std::string, std::string)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x58c914, "rbx::remote_signal<void (std::string, std::string)>::remote_signal()")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::remote_signal(void)")]
pub fn stub_0x58ca70() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::remote_signal(void)")]
pub fn stub_0x58cbcc() -> crate::slot::PortedFn {
// IDA 0x58cbcc: rbx::remote_signal<void (std::string, int, int)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x58cbcc, "rbx::remote_signal<void (std::string, int, int)>::remote_signal()")
}

#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::remote_signal(void)")]
pub fn stub_0x58cd28() -> crate::slot::PortedFn {
// IDA 0x58cd28: rbx::remote_signal<void (std::string, RBX::ContentId)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x58cd28, "rbx::remote_signal<void (std::string, RBX::ContentId)>::remote_signal()")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::disconnectAll(void)")]
pub fn stub_0x58ce84(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (std::string, RBX::ContentId)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x58d1f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x58d390() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc() [0x58d3c0]")]
pub fn stub_0x58d3c0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x58d4dc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::InsertService,void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::InsertService*,void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x58d5c0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::BoundFuncDesc(void (RBX::InsertService::*)(bool,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x58dac4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_0x58dcc0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 2)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::~BoundFuncDesc() [0x58dd0c]")]
pub fn stub_0x58dd0c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x58ddec() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 2)
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
pub fn stub_0x58e25c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x58e358() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x58e42c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 3 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(3)
}

#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
pub fn stub_0x58e500(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x58e5e4(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
pub fn stub_0x58e600(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x58e618(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x58e6f0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x58e7c0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list1<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>) &,boost::_bi::list1<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&> &,int)")]
pub fn stub_0x58e884(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x58e990(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::BoundFuncDesc(void (RBX::InsertService::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x58f340() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x58f4b8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::~BoundFuncDesc() [0x58f4e8]")]
pub fn stub_0x58f4e8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x58f5bc() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(float),1>::BoundFuncDesc(void (RBX::InsertService::*)(float),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x58f5f0() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(float),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x58f768() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(float),1>::~BoundFuncDesc() [0x58f798]")]
pub fn stub_0x58f798(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(float),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x58f86c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(std::string),1>::BoundFuncDesc(void (RBX::InsertService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x58f8a8() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_0x58fa20() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(std::string),1>::~BoundFuncDesc() [0x58fa50]")]
pub fn stub_0x58fa50(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_0x58fb1c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::InsertService", "void", 1)
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::InsertService,void (RBX::InsertService::*)(std::string),std::string,void>::call(RBX::InsertService*,void (RBX::InsertService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
pub fn stub_0x58fc58(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Call1Helper<RBX::InsertService, void (RBX::InsertService::*)(std::string)~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::~RemoteEventDesc() [0x58fd88]")]
pub fn stub_0x58fd88(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x58fe3c() -> crate::slot::SlotConnection {
// IDA 0x58fe3c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::isBroadcast(void)const")]
pub fn stub_0x58ffa8() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::InsertService")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x58ffb0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<2, RBX::InsertService, void (std::string, std::string), rbx~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x5901ec() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::InsertService")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x5901fc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::InsertService, void (std::string, std::string), rbx::r~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::EventDesc(rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x590210() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::InsertService")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::~EventDesc()")]
pub fn stub_0x590400(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::InsertService,void ()(std::string,std::string),rbx::remote_signal<void ()(std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string)> RBX::InsertService::*>::~EventDesc() [0x590424]")]
pub fn stub_0x590424(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::~RemoteEventDesc() [0x5904d8]")]
pub fn stub_0x5904d8(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::InsertService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x59058c() -> crate::slot::SlotConnection {
// IDA 0x59058c: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::isBroadcast(void)const")]
pub fn stub_0x5906f8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::InsertService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x590700() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::InsertService,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x590904() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}
