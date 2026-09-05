// Auto-generated skeletons for rbx-script — Script/Lua/Yield/CodeGen/Luau gap filler
// Filter: Script|Lua|Yield|CodeGen|Luau — filtered all already stubbed — EA-sorted asc gap filler distinct not yet in crates/script/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x4b1c44..0x4b5db0 | EA-sorted asc distinct not yet in script (remaining 57450->57350, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x4b1c44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x4b1c44() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x4b1c64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x4b1c64() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x4b1c7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEvent *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11CustomEventENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x4b1c7c() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x4b2220 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEv — boost::shared_ptr<RBX::CustomEventReceiver> RBX::Creatable<RBX::Instance>::create<RBX::CustomEventReceiver>(void)
#[doc(alias = "boost::shared_ptr<RBX::CustomEventReceiver> RBX::Creatable<RBX::Instance>::create<RBX::CustomEventReceiver>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19CustomEventReceiverEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x4b2220() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CustomEventReceiver")
}

// 0x4b22d0 — __ZN3RBX19CustomEventReceiverC2Ev — RBX::CustomEventReceiver::CustomEventReceiver(void) — _DWORD __fastcall(RBX::CustomEventReceiver *__hidden this)
#[doc(alias = "RBX::CustomEventReceiver::CustomEventReceiver(void)")]
#[doc(alias = "__ZN3RBX19CustomEventReceiverC2Ev")]
pub fn stub_0x4b22d0() -> crate::slot::InstanceHandle {
// RBX::CustomEventReceiver ctor.
crate::slot::InstanceHandle::new("RBX::CustomEventReceiver")
}

// 0x4b2680 — __ZN3rbx7signals6signalIFvfEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>> const&)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x4b2680() -> crate::slot::SlotConnection {
// IDA 0x4b2680: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x4b26f4 — __ZN3RBX19CustomEventReceiver15setCurrentValueEf — RBX::CustomEventReceiver::setCurrentValue(float) — _DWORD __fastcall(RBX::CustomEventReceiver *__hidden this, float)
#[doc(alias = "RBX::CustomEventReceiver::setCurrentValue(float)")]
#[doc(alias = "__ZN3RBX19CustomEventReceiver15setCurrentValueEf")]
pub fn stub_0x4b26f4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::CustomEventReceiver setter.
cell.set(value)
}

// 0x4b2768 — __ZNK3RBX19CustomEventReceiver14askForbidChildEPKNS_8InstanceE — RBX::CustomEventReceiver::askForbidChild(RBX::Instance const*)const — _DWORD __fastcall(RBX::CustomEventReceiver *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::CustomEventReceiver::askForbidChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX19CustomEventReceiver14askForbidChildEPKNS_8InstanceE")]
pub fn stub_0x4b2768(handle: &crate::slot::InstanceHandle) {
// RBX::CustomEventReceiver::askForbidChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b276c — __ZNK3RBX19CustomEventReceiver12askSetParentEPKNS_8InstanceE — RBX::CustomEventReceiver::askSetParent(RBX::Instance const*)const — _DWORD __fastcall(RBX::CustomEventReceiver *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::CustomEventReceiver::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX19CustomEventReceiver12askSetParentEPKNS_8InstanceE")]
pub fn stub_0x4b276c(handle: &crate::slot::InstanceHandle) {
// RBX::CustomEventReceiver::askSetParent(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b283c — __ZN3rbx7signals6signalIFvfEE13disconnectAllEv — rbx::signals::signal<void ()(float)>::disconnectAll(void) — int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE13disconnectAllEv")]
pub fn stub_0x4b283c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float)>::disconnectAll() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b29b4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_ — boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> const&) — int *__fastcall(int *, int *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_")]
pub fn stub_0x4b29b4(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x4b29d8 — __ZN3rbx7signals6signalIFvfEE22safe_static_init_mutexEv — rbx::signals::signal<void ()(float)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(float)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE22safe_static_init_mutexEv")]
pub fn stub_0x4b29d8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b29dc — __ZN3rbx7signals6signalIFvfEE24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(float)>::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(float)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE24safe_static_do_get_mutexEv")]
pub fn stub_0x4b29dc(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b2ad4 — __ZN3rbx7signals6signalIFvfEE6insertEPNS3_4slotE — rbx::signals::signal<void ()(float)>::insert(rbx::signals::signal<void ()(float)>::slot *) — int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(float)>::insert(rbx::signals::signal<void ()(float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE6insertEPNS3_4slotE")]
pub fn stub_0x4b2ad4(slot: &crate::slot::CallableSlot) {
// IDA 0x4b2ad4: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

// 0x4b2ce0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_ — boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_")]
pub fn stub_0x4b2ce0(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x4b2d04 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev — rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x4b2d04(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x4b2d30 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev — rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot() [0x4b2d30]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x4b2d30(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x4b2e04 — __ZN3rbx7signals6signalIFvfEE4slot10disconnectEv — rbx::signals::signal<void ()(float)>::slot::disconnect(void)
#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE4slot10disconnectEv")]
pub fn stub_0x4b2e04(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

// 0x4b2f14 — __ZNK3rbx7signals6signalIFvfEE4slot9connectedEv — rbx::signals::signal<void ()(float)>::slot::connected(void)const
#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvfEE4slot9connectedEv")]
pub fn stub_0x4b2f14() -> crate::slot::SlotConnection {
// IDA 0x4b2f14: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x4b2f20 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf — rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf")]
pub fn stub_0x4b2f20(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4b2f20: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x4b2f34 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf — non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf")]
pub fn stub_0x4b2f34(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4b2f34: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x4b2f48 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_ — void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_")]
pub fn stub_0x4b2f48() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x4b2f68 — __ZN3rbx7signals6signalIFvfEE6removeEPNS3_4slotE — rbx::signals::signal<void ()(float)>::remove(rbx::signals::signal<void ()(float)>::slot *) — int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(float)>::remove(rbx::signals::signal<void ()(float)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE6removeEPNS3_4slotE")]
pub fn stub_0x4b2f68(slot: &mut crate::slot::CallableSlot) {
// IDA 0x4b2f68: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

// 0x4b3058 — __ZN3rbx7signals6signalIFvfEE4slot22safe_static_init_mutexEv — rbx::signals::signal<void ()(float)>::slot::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x4b3058(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b305c — __ZN3rbx7signals6signalIFvfEE4slot24safe_static_do_get_mutexEv — rbx::signals::signal<void ()(float)>::slot::safe_static_do_get_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x4b305c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (float)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b314c — __ZN3rbx7signals6signalIFvfEE4slotD1Ev — rbx::signals::signal<void ()(float)>::slot::~slot() — int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE4slotD1Ev")]
pub fn stub_0x4b314c(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x4b3178 — __ZN3rbx7signals6signalIFvfEE4slotD0Ev — rbx::signals::signal<void ()(float)>::slot::~slot()
#[doc(alias = "rbx::signals::signal<void ()(float)>::slot::~slot() [0x4b3178]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvfEE4slotD0Ev")]
pub fn stub_0x4b3178(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x4b324c — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev — rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev")]
pub fn stub_0x4b324c(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4b324c: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x4b3278 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev — rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable() [0x4b3278]")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev")]
pub fn stub_0x4b3278(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x4b3278: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CustomEventReceiverELZNS_20sCustomEventReceiverEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sCustomEventReceiverEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_0x4b334c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Described<RBX::CustomEventReceiver, RBX::sCustomEventReceiver, RBX::Facto~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CustomEventReceiverELZNS_20sCustomEventReceiverEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sCustomEventReceiverEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4b346c(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CustomEventReceiverELZNS_20sCustomEventReceiverEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sCustomEventReceiverEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4b3470(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CustomEventReceiverELZNS_20sCustomEventReceiverEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sCustomEventReceiverEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4b3510(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CustomEventReceiverELZNS_20sCustomEventReceiverEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sCustomEventReceiverEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4b3518(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CustomEventReceiverELZNS_20sCustomEventReceiverEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sCustomEventReceiverEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x4b35bc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CustomEventReceiverELZNS_20sCustomEventReceiverEENS_14FactoryProductIS2_NS_8InstanceELZNS_20sCustomEventReceiverEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x4b35c4(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

// 0x4b3668 — __ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_ — boost::shared_ptr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "boost::shared_ptr<RBX::CustomEventReceiver>::shared_ptr<RBX::CustomEventReceiver,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19CustomEventReceiverEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x4b3668() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CustomEventReceiver")
}

// 0x4b3730 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_ — void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEventReceiver,RBX::CustomEventReceiver>(boost::shared_ptr<RBX::CustomEventReceiver> const*,RBX::CustomEventReceiver *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CustomEventReceiver,RBX::CustomEventReceiver>(boost::shared_ptr<RBX::CustomEventReceiver> const*,RBX::CustomEventReceiver *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CustomEventReceiverES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x4b3730() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::CustomEventReceiver")
}

// 0x4b3818 — __ZN5boost6detail12shared_countC2IPN3RBX19CustomEventReceiverENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_ — boost::detail::shared_count::shared_count<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter) — int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX19CustomEventReceiverENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x4b3818() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x4b3920 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev — boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x4b3920(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x4b3924 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev — boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() — int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x4b3924]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x4b3924(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x4b3928 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv — boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x4b3928() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x4b3948 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x4b3948() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x4b3960 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CustomEventReceiver *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19CustomEventReceiverENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x4b3960() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x4b3cb8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv")]
pub fn stub_0x4b3cb8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType> ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b3da8 — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED1Ev — RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED1Ev")]
pub fn stub_0x4b3da8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b3dac — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED2Ev — RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::~EnumDesc() [0x4b3dac]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED2Ev")]
pub fn stub_0x4b3dac(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b3f80 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringEmRSs")]
pub fn stub_0x4b3f80(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToString(unsigned l~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b40c4 — __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE14construct_funcEPKcPc")]
pub fn stub_0x4b40c4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::construct_func(cha~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b40d0 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToItem(RBX::StarterGuiService::CoreGuiType const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToItem(RBX::StarterGuiService::CoreGuiType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE13convertToItemERKS3_")]
pub fn stub_0x4b40d0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToItem(RBX::Starter~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b419c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE13initSingletonEv")]
pub fn stub_0x4b419c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType> co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b41a0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType> const>::doGetSingleton(void) — void *()
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE14doGetSingletonEv")]
pub fn stub_0x4b41a0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType> co~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4290 — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED1Ev — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED1Ev")]
pub fn stub_0x4b4290(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b4294 — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED2Ev — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::~EnumDesc() [0x4b4294]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED2Ev")]
pub fn stub_0x4b4294(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b4468 — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED0Ev — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::~EnumDesc() [0x4b4468]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED0Ev")]
pub fn stub_0x4b4468(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b4508 — __ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE6lookupEPKc")]
pub fn stub_0x4b4508(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4538 — __ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4b4538(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::lookup(RBX::Reflection::Vari~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4558 — __ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4b4558(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToValue(unsigned long~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b45b4 — __ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE15convertToStringEmRSs")]
pub fn stub_0x4b45b4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToString(unsigned lon~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b46f8 — __ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToString(RBX::HttpService::HttpContentType const&)const — int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToString(RBX::HttpService::HttpContentType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE15convertToStringERKS3_")]
pub fn stub_0x4b46f8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToString(RBX::HttpSer~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4898 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11HttpService15HttpContentTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HttpService::HttpContentType>(RBX::HttpService::HttpContentType const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::HttpService::HttpContentType>(RBX::HttpService::HttpContentType const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11HttpService15HttpContentTypeEEERS3_RKT_")]
pub fn stub_0x4b4898() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4b48e8 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE9singletonEv")]
pub fn stub_0x4b48e8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4954 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE14construct_funcEPKcPc")]
pub fn stub_0x4b4954(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::construct_func(char ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4960 — __ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11HttpService15HttpContentTypeEE13destruct_funcEPc")]
pub fn stub_0x4b4960(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::HttpService::HttpContentType>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4964 — __ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToItem(RBX::HttpService::HttpContentType const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToItem(RBX::HttpService::HttpContentType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE13convertToItemERKS3_")]
pub fn stub_0x4b4964(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToItem(RBX::HttpServi~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4a30 — __ZN3rbx8any_castIRKN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::HttpService::HttpContentType const& rbx::any_cast<RBX::HttpService::HttpContentType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::HttpService::HttpContentType const& rbx::any_cast<RBX::HttpService::HttpContentType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX11HttpService15HttpContentTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4b4a30(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4b4b20 — __ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToValue(RBX::Name const&,RBX::HttpService::HttpContentType&)const — int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToValue(RBX::Name const&,RBX::HttpService::HttpContentType&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4b4b20(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::HttpService::HttpContentType>::convertToValue(RBX::Name con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4b9c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::HttpService::HttpContentType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11HttpService15HttpContentTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4b4b9c(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4b4bc4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12AssetService10AccessTypeEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::AssetService::AccessType> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::AssetService::AccessType> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12AssetService10AccessTypeEEEE13initSingletonEv")]
pub fn stub_0x4b4bc4(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::AssetService::AccessType> const>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4bc8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12AssetService10AccessTypeEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::AssetService::AccessType> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::AssetService::AccessType> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12AssetService10AccessTypeEEEE14doGetSingletonEv")]
pub fn stub_0x4b4bc8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::AssetService::AccessType> const>~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4cb8 — __ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED1Ev — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED1Ev")]
pub fn stub_0x4b4cb8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b4cbc — __ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED2Ev — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::~EnumDesc() [0x4b4cbc]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED2Ev")]
pub fn stub_0x4b4cbc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b4e90 — __ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED0Ev — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::~EnumDesc() [0x4b4e90]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED0Ev")]
pub fn stub_0x4b4e90(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b4f30 — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE6lookupEPKc")]
pub fn stub_0x4b4f30(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4f60 — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4b4f60(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::lookup(RBX::Reflection::Variant ~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4f80 — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4b4f80(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToValue(unsigned long, RB~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b4fdc — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE15convertToStringEmRSs")]
pub fn stub_0x4b4fdc(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToString(unsigned long, s~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5120 — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToString(RBX::AssetService::AccessType const&)const — int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToString(RBX::AssetService::AccessType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE15convertToStringERKS3_")]
pub fn stub_0x4b5120(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToString(RBX::AssetServic~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b52c0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::AssetService::AccessType>(RBX::AssetService::AccessType const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12AssetService10AccessTypeEEERS3_RKT_")]
pub fn stub_0x4b52c0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4b5310 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE9singletonEv")]
pub fn stub_0x4b5310(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::AssetService::AccessType>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b537c — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE14construct_funcEPKcPc")]
pub fn stub_0x4b537c(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::AssetService::AccessType>::construct_func(char cons~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5388 — __ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char *) — void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX12AssetService10AccessTypeEE13destruct_funcEPc")]
pub fn stub_0x4b5388(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::AssetService::AccessType>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b538c — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE13convertToItemERKS3_ — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToItem(RBX::AssetService::AccessType const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToItem(RBX::AssetService::AccessType const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE13convertToItemERKS3_")]
pub fn stub_0x4b538c(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToItem(RBX::AssetService:~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5458 — __ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE — RBX::AssetService::AccessType const& rbx::any_cast<RBX::AssetService::AccessType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
#[doc(alias = "RBX::AssetService::AccessType const& rbx::any_cast<RBX::AssetService::AccessType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX12AssetService10AccessTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x4b5458(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

// 0x4b5548 — __ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE14convertToValueERKNS_4NameERS3_ — RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToValue(RBX::Name const&,RBX::AssetService::AccessType&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToValue(RBX::Name const&,RBX::AssetService::AccessType&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0x4b5548(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::AssetService::AccessType>::convertToValue(RBX::Name const&,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b55c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::AssetService::AccessType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::AssetService::AccessType>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12AssetService10AccessTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0x4b55c4(map: &mut crate::slot::TreeMapModel, key: &str) -> bool {
// map erase — unlinks the node.
map.erase(key)
}

// 0x4b55ec — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE13initSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> const>::initSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> const>::initSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE13initSingletonEv")]
pub fn stub_0x4b55ec(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b55f0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE14doGetSingletonEv — RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> const>::doGetSingleton(void)
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> const>::doGetSingleton(void)")]
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE14doGetSingletonEv")]
pub fn stub_0x4b55f0(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState> con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b56e0 — __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED1Ev — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc()")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED1Ev")]
pub fn stub_0x4b56e0(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b56e4 — __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED2Ev — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc() [0x4b56e4]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED2Ev")]
pub fn stub_0x4b56e4(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b58b8 — __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED0Ev — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::~EnumDesc() [0x4b58b8]")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED0Ev")]
pub fn stub_0x4b58b8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc dtor.
drop(handle);
}

// 0x4b5958 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupEPKc — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(char const*)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(char const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupEPKc")]
pub fn stub_0x4b5958(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(char const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5988 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupERKNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(RBX::Reflection::Variant const&)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE6lookupERKNS0_7VariantE")]
pub fn stub_0x4b5988(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::lookup(RBX::Reflection::Varia~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b59a8 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE14convertToValueEmRNS0_7VariantE — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE14convertToValueEmRNS0_7VariantE")]
pub fn stub_0x4b59a8(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToValue(unsigned long,~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5a04 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringEmRSs — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(unsigned long,std::string &)const
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(unsigned long,std::string &)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringEmRSs")]
pub fn stub_0x4b5a04(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(unsigned long~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5b48 — __ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringERKS3_ — RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(RBX::InputObject::UserInputState const&)const — int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(RBX::InputObject::UserInputState const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEE15convertToStringERKS3_")]
pub fn stub_0x4b5b48(handle: &crate::slot::InstanceHandle) {
// RBX::Reflection::EnumDesc<RBX::InputObject::UserInputState>::convertToString(RBX::InputObj~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5ce8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_ — rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&) — void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject::UserInputState>(RBX::InputObject::UserInputState const&)")]
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObject14UserInputStateEEERS3_RKT_")]
pub fn stub_0x4b5ce8() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

// 0x4b5d38 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton(void)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE9singletonEv")]
pub fn stub_0x4b5d38(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5da4 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char const*,char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char const*,char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE14construct_funcEPKcPc")]
pub fn stub_0x4b5da4(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::construct_func(char c~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x4b5db0 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc — rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char *)")]
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX11InputObject14UserInputStateEE13destruct_funcEPc")]
pub fn stub_0x4b5db0(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::InputObject::UserInputState>::destruct_func(char*) — engine-side; linkage preserved via the alias.
let _ = handle;
}
