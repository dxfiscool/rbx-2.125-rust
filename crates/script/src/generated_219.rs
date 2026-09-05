// Auto-generated skeletons for rbx-script — script filler EA-sorted asc next 150 not yet in script
// Filter: Script|Lua|Yield (4818 filtered, all already stubbed) — global gap filler EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +150 stubs | range 0x39d700..0x3a4354 | script 22352->22502 distinct (filler 0x39d700 asc, not-in-script 63193->63043)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
pub fn stub_0x39d700() -> crate::slot::SlotConnection {
// IDA 0x39d700: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
pub fn stub_0x39d774(slot: &crate::slot::CallableSlot) {
// IDA 0x39d774: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)")]
pub fn stub_0x39d980(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
pub fn stub_0x39d9a4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot() [0x39d9d0]")]
pub fn stub_0x39d9d0(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::disconnect(void)")]
pub fn stub_0x39daa4(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::connected(void)const")]
pub fn stub_0x39dbb4() -> crate::slot::SlotConnection {
// IDA 0x39dbb4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
pub fn stub_0x39dbc0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39dbc0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
pub fn stub_0x39dbec(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39dbec: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float> &,boost::_bi::list3<float &,float &,float &> &,int)")]
pub fn stub_0x39dc18(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x39dc18: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
pub fn stub_0x39dc54(slot: &mut crate::slot::CallableSlot) {
// IDA 0x39dc54: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x39dd44(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x39dd48(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::~slot()")]
pub fn stub_0x39de38(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::~slot() [0x39de64]")]
pub fn stub_0x39de64(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable()")]
pub fn stub_0x39df38(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39df38: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::~callable() [0x39df64]")]
pub fn stub_0x39df64(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39df64: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_0x39e038() -> crate::slot::SlotConnection {
// IDA 0x39e038: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0x39e0ac(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot() [0x39e0d8]")]
pub fn stub_0x39e0d8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
pub fn stub_0x39e1ac(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39e1ac: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::call(float,float)")]
pub fn stub_0x39e1d4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39e1d4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list2<float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float> &,boost::_bi::list2<float &,float &> &,int)")]
pub fn stub_0x39e1fc(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x39e1fc: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable()")]
pub fn stub_0x39e228(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39e228: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::AnimationTrackState,float,float>,boost::_bi::list3<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>>>,2,void ()(float,float)>::~callable() [0x39e254]")]
pub fn stub_0x39e254(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39e254: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>> const&)")]
pub fn stub_0x39e328() -> crate::slot::SlotConnection {
// IDA 0x39e328: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
pub fn stub_0x39e39c(slot: &crate::slot::CallableSlot) {
// IDA 0x39e39c: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float,float)>::slot*)")]
pub fn stub_0x39e5a8(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot()")]
pub fn stub_0x39e5cc(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::~callable_slot() [0x39e5f8]")]
pub fn stub_0x39e5f8(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::disconnect(void)")]
pub fn stub_0x39e6cc(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::connected(void)const")]
pub fn stub_0x39e7dc() -> crate::slot::SlotConnection {
// IDA 0x39e7dc: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
pub fn stub_0x39e7e8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39e7e8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
pub fn stub_0x39e824(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39e824: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState *>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")]
pub fn stub_0x39e860(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x39e860: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::remove(rbx::signals::signal<void ()(float,float,float,float)>::slot *)")]
pub fn stub_0x39e8b0(slot: &mut crate::slot::CallableSlot) {
// IDA 0x39e8b0: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x39e9a0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float, float)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x39e9a4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float, float)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot()")]
pub fn stub_0x39ea94(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::slot::~slot() [0x39eac0]")]
pub fn stub_0x39eac0(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable()")]
pub fn stub_0x39eb94(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39eb94: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::AnimationTrackState,float,float,float,float>,boost::_bi::list5<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(float,float,float,float)>::~callable() [0x39ebc0]")]
pub fn stub_0x39ebc0(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x39ebc0: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::remote_signal(void)")]
pub fn stub_0x39ec94() -> crate::slot::PortedFn {
// IDA 0x39ec94: rbx::remote_signal<void (float, float, float)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x39ec94, "rbx::remote_signal<void (float, float, float)>::remote_signal()")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::disconnectAll(void)")]
pub fn stub_0x39edf0(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "rbx::remote_signal<void ()(float,float)>::remote_signal(void)")]
pub fn stub_0x39ef68() -> crate::slot::PortedFn {
// IDA 0x39ef68: rbx::remote_signal<void (float, float)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x39ef68, "rbx::remote_signal<void (float, float)>::remote_signal()")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::remote_signal(void)")]
pub fn stub_0x39f0c4() -> crate::slot::PortedFn {
// IDA 0x39f0c4: rbx::remote_signal<void (float, float, float, float)>::remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x39f0c4, "rbx::remote_signal<void (float, float, float, float)>::remote_signal()")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::disconnectAll(void)")]
pub fn stub_0x39f220(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float, float, float, float)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39f398(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x39f39c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39f43c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x39f444(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x39f4e8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19AnimationTrackStateELZNS_20sAnimationTrackStateEENS_17NonFactoryProductINS_8InstanceELZNS_20sAnimationTrackStateEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x39f4f0(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc() [0x39f594]")]
pub fn stub_0x39f594(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x39f648() -> crate::slot::SlotConnection {
// IDA 0x39f648: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
pub fn stub_0x39f7b4() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39f7bc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<1, RBX::AnimationTrackState, void (std::string), rbx::remot~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39f960() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x39f970(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::AnimationTrackState, void (std::string), rbx::remote_s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x39f984() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x39fb08(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::AnimationTrackState::*>::~EventDesc() [0x39fb2c]")]
pub fn stub_0x39fb2c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::~RemoteEventDesc() [0x39fbe0]")]
pub fn stub_0x39fbe0(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x39fc94() -> crate::slot::SlotConnection {
// IDA 0x39fc94: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::isBroadcast(void)const")]
pub fn stub_0x39fe00() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39fe08(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<3, RBX::AnimationTrackState, void (float, float, float), rb~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x39feb0() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x39fec0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::AnimationTrackState, void (float, float, float), rbx::~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
pub fn stub_0x39fed4() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<float,float,float>(float const&,float const&,float const&)")]
pub fn stub_0x39fff0() -> crate::slot::PortedFn {
// IDA 0x39fff0: void RBX::Reflection::GenericSlotWrapper::execute3<float, float, float>(float const&, float const&, float const&).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x39fff0, "void RBX::Reflection::GenericSlotWrapper::execute3<float, float, float>(float const&, float const&, ~")
}

#[doc(alias = "boost::function3<void,float,float,float>::clear(void)")]
pub fn stub_0x3a017c(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvfffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_EENS4_5list4INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3a01a8() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function3IvfffEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_EENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3a028c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void boost::function3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
pub fn stub_0x3a0374(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3a046c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,float,float,float>::invoke(boost::detail::function::function_buffer &,float,float,float)")]
pub fn stub_0x3a0488(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3a04b0(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable3<void,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3a0598(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable3<void,float,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3a067c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list3<float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&> &,boost::_bi::list3<float &,float &,float &> &,int)")]
pub fn stub_0x3a0750(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3a0750: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&>,boost::_bi::list4<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3a0778(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::function<void ()(float,float,float)>>(boost::function<void ()(float,float,float)> const&)")]
pub fn stub_0x3a08d0() -> crate::slot::SlotConnection {
// IDA 0x3a08d0: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float)>*>(boost::function<void ()(float,float,float)> const&,rbx::signals::signal<void ()(float,float,float)>*)")]
pub fn stub_0x3a09c4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a09c4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot()")]
pub fn stub_0x3a0ac0(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::function<void ()(float,float,float)>>::~callable_slot() [0x3a0bd0]")]
pub fn stub_0x3a0bd0(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
pub fn stub_0x3a0d00(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a0d00: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::call(float,float,float)")]
pub fn stub_0x3a0d08(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a0d08: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,float,float,float>::operator()(float,float,float)const")]
pub fn stub_0x3a0d10(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable()")]
pub fn stub_0x3a0dec(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a0dec: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::function<void ()(float,float,float)>,3,void ()(float,float,float)>::~callable() [0x3a0efc]")]
pub fn stub_0x3a0efc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a0efc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function3<void,float,float,float>::assign_to_own(boost::function3<void,float,float,float> const&)")]
pub fn stub_0x3a102c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3a105c() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x3a12b8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float),rbx::remote_signal<void ()(float,float,float)>,rbx::remote_signal<void ()(float,float,float)> RBX::AnimationTrackState::*>::~EventDesc() [0x3a12dc]")]
pub fn stub_0x3a12dc(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::~RemoteEventDesc() [0x3a1390]")]
pub fn stub_0x3a1390(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3a1444() -> crate::slot::SlotConnection {
// IDA 0x3a1444: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::isBroadcast(void)const")]
pub fn stub_0x3a15b0() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a15b8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<2, RBX::AnimationTrackState, void (float, float), rbx::remo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a1654() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3a1664(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::AnimationTrackState, void (float, float), rbx::remote_~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3a1678() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x3a1868(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float),rbx::remote_signal<void ()(float,float)>,rbx::remote_signal<void ()(float,float)> RBX::AnimationTrackState::*>::~EventDesc() [0x3a188c]")]
pub fn stub_0x3a188c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::~RemoteEventDesc() [0x3a1940]")]
pub fn stub_0x3a1940(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_0x3a19f4() -> crate::slot::SlotConnection {
// IDA 0x3a19f4: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::isBroadcast(void)const")]
pub fn stub_0x3a1b60() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<4,RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a1b68(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescImpl<4, RBX::AnimationTrackState, void (float, float, float, flo~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_0x3a1c2c() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDescBase<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_0x3a1c3c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EventDescBase<RBX::AnimationTrackState, void (float, float, float, float)~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list_av_5<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>(void (RBX::Reflection::GenericSlotWrapper::*)(float const&,float const&,float const&,float const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
pub fn stub_0x3a1c50() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute4<float,float,float,float>(float const&,float const&,float const&,float const&)")]
pub fn stub_0x3a1d6c() -> crate::slot::PortedFn {
// IDA 0x3a1d6c: void RBX::Reflection::GenericSlotWrapper::execute4<float, float, float, float>(float const&, float const&, float const&,~.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a1d6c, "void RBX::Reflection::GenericSlotWrapper::execute4<float, float, float, float>(float const&, float c~")
}

#[doc(alias = "boost::function4<void,float,float,float,float>::clear(void)")]
pub fn stub_0x3a1f18(slot: &mut crate::slot::FnSlot) {
// boost::function::clear — drops the stored functor.
slot.clear();
}

#[doc(alias = "__ZN5boost8functionIFvffffEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSC_SC_SC_EENS4_5list5INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEENSJ_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3a1f44() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "__ZN5boost9function4IvffffEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKfSB_SB_SB_EENS3_5list5INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEENSI_ILi3EEENSI_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_0x3a2028() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 8 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(8)
}

#[doc(alias = "void boost::function4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>)")]
pub fn stub_0x3a2110(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_0x3a2208(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker4<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,void,float,float,float,float>::invoke(boost::detail::function::function_buffer &,float,float,float,float)")]
pub fn stub_0x3a2224(slot: &crate::slot::FnSlot) {
// boost::function void_function_obj_invoker — dispatches the
// stored functor, no-op when empty.
slot.invoke();
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_0x3a225c(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "bool boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0x3a2344(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::detail::function::basic_vtable4<void,float,float,float,float>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_0x3a2428(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::operator()<boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list4<float &,float &,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&> &,boost::_bi::list4<float &,float &,float &,float &> &,int)")]
pub fn stub_0x3a24fc(call: &crate::slot::BoundCall, args: &[f32]) {
// IDA 0x3a24fc: list::operator() (cf. 0x39dc18) — resolves the
// member pointer (incl. the virtual-call branch) and invokes
// mf(object, args...).
call.apply(args);
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Reflection::GenericSlotWrapper,float const&,float const&,float const&,float const&>,boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0x3a252c(slot: &crate::slot::FnSlot) -> crate::slot::FnSlot {
// boost::function functor_manager::manage — clone/destroy op
// over the type-erased buffer; the host clones the slot.
slot.clone_op()
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float,float)>::connect<boost::function<void ()(float,float,float,float)>>(boost::function<void ()(float,float,float,float)> const&)")]
pub fn stub_0x3a2684() -> crate::slot::SlotConnection {
// IDA 0x3a2684: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::callable<rbx::signals::signal<void ()(float,float,float,float)>*>(boost::function<void ()(float,float,float,float)> const&,rbx::signals::signal<void ()(float,float,float,float)>*)")]
pub fn stub_0x3a2778(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a2778: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot()")]
pub fn stub_0x3a2874(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::callable_slot<boost::function<void ()(float,float,float,float)>>::~callable_slot() [0x3a2984]")]
pub fn stub_0x3a2984(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
pub fn stub_0x3a2ab4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a2ab4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::call(float,float,float,float)")]
pub fn stub_0x3a2abc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a2abc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function4<void,float,float,float,float>::operator()(float,float,float,float)const")]
pub fn stub_0x3a2ac4(slot: &crate::slot::FnSlot) {
// boost::function::operator() — dispatches the stored functor.
slot.invoke();
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable()")]
pub fn stub_0x3a2bac(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a2bac: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float,float)>::slot,boost::function<void ()(float,float,float,float)>,4,void ()(float,float,float,float)>::~callable() [0x3a2cbc]")]
pub fn stub_0x3a2cbc(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x3a2cbc: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "boost::function4<void,float,float,float,float>::assign_to_own(boost::function4<void,float,float,float,float> const&)")]
pub fn stub_0x3a2dec(slot: &mut crate::slot::FnSlot) -> bool {
// boost::function basic_vtable assign — always stores into
// the functor slot. was: boost::function<R(ARGS)>.
slot.assign()
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::EventDesc(rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*,char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_0x3a2e1c() -> crate::slot::DescriptorHandle {
// EventDesc ctor — registers the event.
crate::slot::DescriptorHandle::event("RBX::AnimationTrackState")
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc()")]
pub fn stub_0x3a30e8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrackState,void ()(float,float,float,float),rbx::remote_signal<void ()(float,float,float,float)>,rbx::remote_signal<void ()(float,float,float,float)> RBX::AnimationTrackState::*>::~EventDesc() [0x3a310c]")]
pub fn stub_0x3a310c(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float,float)>::~remote_signal()")]
pub fn stub_0x3a31c0() -> crate::slot::PortedFn {
// IDA 0x3a31c0: rbx::remote_signal<void (float, float, float, float)>::~remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a31c0, "rbx::remote_signal<void (float, float, float, float)>::~remote_signal()")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float,float)>::~remote_signal()")]
pub fn stub_0x3a330c() -> crate::slot::PortedFn {
// IDA 0x3a330c: rbx::remote_signal<void (float, float, float)>::~remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a330c, "rbx::remote_signal<void (float, float, float)>::~remote_signal()")
}

#[doc(alias = "rbx::remote_signal<void ()(float,float)>::~remote_signal()")]
pub fn stub_0x3a3458() -> crate::slot::PortedFn {
// IDA 0x3a3458: rbx::remote_signal<void (float, float)>::~remote_signal().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x3a3458, "rbx::remote_signal<void (float, float)>::~remote_signal()")
}

#[doc(alias = "global constructor keyed to_a_157")]
pub fn stub_0x3a35a4() -> crate::slot::PortedFn {
// IDA 0x3a35a4: __GLOBAL__I_a_157.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x3a35a4, "__GLOBAL__I_a_157")
}

#[doc(alias = "RBX::Animator::loadAnimation(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_0x3a395c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

#[doc(alias = "RBX::Animator::Animator(RBX::Instance *)")]
pub fn stub_0x3a3d44() -> crate::slot::InstanceHandle {
// RBX::Animator ctor.
crate::slot::InstanceHandle::new("RBX::Animator")
}

#[doc(alias = "RBX::Animator::Animator(RBX::Instance *) [0x3a3d48]")]
pub fn stub_0x3a3d48() -> crate::slot::InstanceHandle {
// RBX::Animator ctor.
crate::slot::InstanceHandle::new("RBX::Animator")
}

#[doc(alias = "RBX::Animator::~Animator()")]
pub fn stub_0x3a40b8(handle: crate::slot::InstanceHandle) {
// RBX::Animator dtor.
drop(handle);
}

#[doc(alias = "RBX::Animator::~Animator() [0x3a4158]")]
pub fn stub_0x3a4158(handle: crate::slot::InstanceHandle) {
// RBX::Animator dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator()")]
pub fn stub_0x3a415c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator() [0x3a4164]")]
pub fn stub_0x3a4164(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator() [0x3a416c]")]
pub fn stub_0x3a416c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 92, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 92);
}

#[doc(alias = "RBX::Animator::~Animator() [0x3a4174]")]
pub fn stub_0x3a4174(handle: crate::slot::InstanceHandle) {
// RBX::Animator dtor.
drop(handle);
}

#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator() [0x3a434c]")]
pub fn stub_0x3a434c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "non-virtual thunk toRBX::Animator::~Animator() [0x3a4354]")]
pub fn stub_0x3a4354(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}
