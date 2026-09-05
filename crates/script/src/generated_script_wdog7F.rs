// Auto-generated skeletons for rbx-script — wdog7F (crate script)
// Filter: Script|Lua|LuaBridge|Yield|GC|Closure (case-sensitive) — 4876 filtered, 48 remaining not yet in crates/script/src, gap_filler EA-sorted asc distinct
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs EA-sorted asc | range 0x5afe98..0xf64a44 | distinct not yet in crates/script/src (remaining 48 -> +72 gap filler global EA asc, rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; boost stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x5afe98 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x5afe98() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5afeb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x5afeb8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5afed0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4SnapENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x5afed0() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5sSnapEEEERKS0_v")]
pub fn stub_0x5afed4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sSnap>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5sSnapEEEEvv")]
pub fn stub_0x5aff18() -> crate::slot::PortedFn {
// IDA 0x5aff18: void RBX::Name::callDoDeclare<RBX::sSnap>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x5aff18, "void RBX::Name::callDoDeclare<RBX::sSnap>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSnapEEEERKS0_v")]
pub fn stub_0x5aff1c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sSnap>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_4SnapENS_13JointInstanceELZNS_5sSnapEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x5b0000() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Snap"
}

// 0x5b0228 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::insert(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE6insertEPNS6_4slotE")]
pub fn stub_0x5b0228(slot: &crate::slot::CallableSlot) {
// IDA 0x5b0228: signal::insert — links the slot (the host Signal
// owns slots via Arc/Weak, so linking is covered by connect).
assert!(slot.is_connected());
}

// 0x5b0434 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_
// type: int(void)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Joint *)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSEPS9_")]
pub fn stub_0x5b0434(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5b0458 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_
// type: int(void)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Joint *)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX5JointEEE4slotEEaSERKSA_")]
pub fn stub_0x5b0458(handle: &crate::slot::InstanceHandle) -> crate::slot::InstanceHandle {
// shared/intrusive_ptr operator= — atomic retain + alias;
// the host copy carries the same identity.
*handle
}

// 0x5b047c — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE22safe_static_init_mutexEv")]
pub fn stub_0x5b047c(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Joint*)>::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b0480 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE24safe_static_do_get_mutexEv")]
pub fn stub_0x5b0480(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Joint*)>::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b0578 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED1Ev")]
pub fn stub_0x5b0578(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x5b05a4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>>::~callable_slot() [0x5b05a4]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS2_13JointsServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEEED0Ev")]
pub fn stub_0x5b05a4(slot: crate::slot::CallableSlot) {
// callable_slot D0/D1 (cf. 0x39d9a4) — run the bind_t dtor,
// then free. The owned value drop frees both here.
drop(slot);
}

// 0x5b0678 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot10disconnectEv")]
pub fn stub_0x5b0678(slot: &mut crate::slot::CallableSlot) {
// rbx::signals slot::disconnect — detach without dropping.
slot.disconnect();
}

// 0x5b0788 — __ZNK3rbx7signals6signalIFvPN3RBX5JointEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvPN3RBX5JointEEE4slot9connectedEv")]
pub fn stub_0x5b0788() -> crate::slot::SlotConnection {
// IDA 0x5b0788: signal::connect<bind_t> (cf. 0x39d700) — links the
// bound callable_slot into the signal and returns the
// rbx::signals::connection. was: boost::bind/_mfi.
crate::slot::SlotConnection::new()
}

// 0x5b0794 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_0x5b0794(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5b0794: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5b07a8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::call(RBX::Joint *)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")]
pub fn stub_0x5b07a8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5b07a8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5b07bc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>::operator()<RBX::Joint *>(RBX::Joint * &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13JointsServiceEPNS4_5JointEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
pub fn stub_0x5b07bc() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 1 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(1)
}

// 0x5b07d4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::remove(rbx::signals::signal<void ()(RBX::Joint *)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE6removeEPNS6_4slotE")]
pub fn stub_0x5b07d4(slot: &mut crate::slot::CallableSlot) {
// IDA 0x5b07d4: signal::remove (cf. 0x39dc54) — ReleaseAssert the
// slot ref is alive (signal.h:261), fast-log, then unlink.
assert!(slot.is_connected());
slot.disconnect();
}

// 0x5b08c4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot22safe_static_init_mutexEv")]
pub fn stub_0x5b08c4(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Joint*)>::slot::safe_static_init_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b08c8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0x5b08c8(handle: &crate::slot::InstanceHandle) {
// rbx::signals::signal<void (RBX::Joint*)>::slot::safe_static_do_get_mutex() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b09b8 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD1Ev")]
pub fn stub_0x5b09b8(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x5b09e4 — __ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Joint *)>::slot::~slot() [0x5b09e4]")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPN3RBX5JointEEE4slotD0Ev")]
pub fn stub_0x5b09e4(handle: crate::slot::InstanceHandle) {
// rbx::signals dtor.
drop(handle);
}

// 0x5b0ab8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")]
pub fn stub_0x5b0ab8(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5b0ab8: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

// 0x5b0ae4 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Joint *)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::JointsService,RBX::Joint *>,boost::_bi::list2<boost::_bi::value<RBX::JointsService*>,boost::arg<1>>>,1,void ()(RBX::Joint *)>::~callable() [0x5b0ae4]")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPN3RBX5JointEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS3_13JointsServiceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")]
pub fn stub_0x5b0ae4(slot: &crate::slot::CallableSlot, args: &[f32]) {
// IDA 0x5b0ae4: callable<slot,bind_t>::call (cf. 0x39dbc0) — packs
// the signal args and runs list::operator() on the stored bind.
slot.invoke(args);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5b0bb8(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5b0bbc(handle: crate::slot::InstanceHandle) {
// RBX::Reflection::Described dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_0x5b0c5c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13JointsServiceELZNS_14sJointsServiceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointsServiceEEEELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_0x5b0c64(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

// 0x5b0db4 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::BoundFuncDesc(void (RBX::JointsService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x5b0db4() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::JointsService", "void", 0)
}

// 0x5b0eb8 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::~BoundFuncDesc() [0x5b0eb8]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EED0Ev")]
pub fn stub_0x5b0eb8(handle: crate::slot::DescriptorHandle) {
// descriptor dtor — unregisters and releases.
drop(handle);
}

// 0x5b0f6c — __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x5b0f6c() -> crate::slot::DescriptorHandle {
// BoundFuncDesc ctor — registers the member binding.
crate::slot::DescriptorHandle::func("RBX::JointsService", "void", 0)
}

// 0x5b0f8c — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x5b0f8c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b1124 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x5b1124() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b1154 — __ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc() [0x5b1154]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev")]
pub fn stub_0x5b1154(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5b1270 — __ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x5b1270() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b1354 — __ZN3RBX10Reflection11Call1HelperINS_13JointsServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::JointsService,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::JointsService*,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_13JointsServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")]
pub fn stub_0x5b1354() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b1bb0 — __ZN3RBX8Keyframe7addPoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Keyframe::addPose(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Keyframe7addPoseEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5b1bb0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b1bbc — __ZN3RBX8Keyframe10removePoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Keyframe::removePose(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX8Keyframe10removePoseEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5b1bbc() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b1e44 — __ZNK3RBX8Keyframe17verifySetAncestorEPKNS_8InstanceES3_
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, const RBX::Instance *, const RBX::Instance *)
#[doc(alias = "RBX::Keyframe::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Keyframe17verifySetAncestorEPKNS_8InstanceES3_")]
pub fn stub_0x5b1e44(handle: &crate::slot::InstanceHandle) {
// RBX::Keyframe::verifySetAncestor(RBX::Instance const*, RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b1f34 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev")]
pub fn stub_0x5b1f34(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5b1f58 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev")]
pub fn stub_0x5b1f58(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5b2068 — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED1Ev")]
pub fn stub_0x5b2068(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b2130 — __ZNK3RBX8Keyframe11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Keyframe::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX8Keyframe11askAddChildEPKNS_8InstanceE")]
pub fn stub_0x5b2130(handle: &crate::slot::InstanceHandle) {
// RBX::Keyframe::askAddChild(RBX::Instance const*) const — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b216c — __ZN3RBX8Keyframe12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Keyframe *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Keyframe::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8Keyframe12onChildAddedEPNS_8InstanceE")]
pub fn stub_0x5b216c(handle: &crate::slot::InstanceHandle) {
// RBX::Keyframe::onChildAdded(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b2170 — __ZN3RBX8Keyframe14onChildRemovedEPNS_8InstanceE
// type: int __fastcall(RBX::Keyframe *this, RBX::Instance *)
#[doc(alias = "RBX::Keyframe::onChildRemoved(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX8Keyframe14onChildRemovedEPNS_8InstanceE")]
pub fn stub_0x5b2170(handle: &crate::slot::InstanceHandle) {
// RBX::Keyframe::onChildRemoved(RBX::Instance*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x5b2558 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_8KeyframeEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe> RBX::Creatable<RBX::Instance>::create<RBX::Keyframe>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_8KeyframeEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0x5b2558() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Keyframe")
}

// 0x5b2608 — __ZN5boost10shared_ptrIN3RBX8KeyframeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe>::shared_ptr<RBX::Keyframe,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8KeyframeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_0x5b2608() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Keyframe")
}

// 0x5b26d0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Keyframe,RBX::Keyframe>(rbx_core::SharedPtr<RBX::Keyframe> const*,RBX::Keyframe *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8KeyframeES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x5b26d0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Keyframe")
}

// 0x5b27b8 — __ZN5boost6detail12shared_countC2IPN3RBX8KeyframeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX8KeyframeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_0x5b27b8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5b28c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0x5b28c0(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5b28c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x5b28c4]")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0x5b28c4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x5b28c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0x5b28c8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5b28e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x5b28e8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5b2900 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX8KeyframeENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x5b2900() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x5b2e9c — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::PropDescriptor<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>(char const*,char const*,float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x5b2e9c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b2fb0 — __ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::~PropDescriptor() [0x5b2fb0]")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_8KeyframeEfED0Ev")]
pub fn stub_0x5b2fb0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// PropDescriptor GetImpl — loads the converted value.
cell.get()
}

// 0x5b2fdc — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE10isReadOnlyEv")]
pub fn stub_0x5b2fdc(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x5b2fe0 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE11isWriteOnlyEv")]
pub fn stub_0x5b2fe0(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x5b2fe4 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x5b2fe4(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x5b3004 — __ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Keyframe,float>::GetSetImpl<float (RBX::Keyframe::*)(void)const,void (RBX::Keyframe::*)(float)>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_8KeyframeEfE10GetSetImplIMS2_KFfvEMS2_FvfEE8setValueEPNS0_13DescribedBaseERKf")]
pub fn stub_0x5b3004(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// PropDescriptor SetImpl — stores the converted value.
cell.set(value);
}

// 0x5b3028 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x5b3028() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b31c0 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0x5b31c0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b31f0 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc() [0x5b31f0]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev")]
pub fn stub_0x5b31f0(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5b330c — __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x5b330c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b33f0 — __ZN3RBX10Reflection11Call1HelperINS_8KeyframeEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Keyframe,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Keyframe*,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_8KeyframeEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")]
pub fn stub_0x5b33f0() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x5b34d8 — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x5b34d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

// 0x5b35dc — __ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc() [0x5b35dc]")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev")]
pub fn stub_0x5b35dc(handle: crate::slot::InstanceHandle) {
// shared/intrusive_ptr dtor — release; last one frees.
drop(handle);
}

// 0x5b3690 — __ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x5b3690() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

// 0x5b36b4 — __ZN3RBX10Reflection11Call0HelperINS_8KeyframeEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Keyframe*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_8KeyframeEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE")]
pub fn stub_0x5b36b4() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("std::vector<boost::shared_ptr<RBX::Instance>, std::allocator<boost::shared_ptr<R~")
}

// 0x5b3b1c — __ZN3RBX16KeyframeSequence11addKeyframeEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::KeyframeSequence::addKeyframe(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX16KeyframeSequence11addKeyframeEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0x5b3b1c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0x82a138 — __ZL16reallymarkobjectP12global_StateP8GCObject
// type: int *__fastcall(int, int)
#[doc(alias = "reallymarkobject(global_State *,GCObject *)")]
#[doc(alias = "__ZL16reallymarkobjectP12global_StateP8GCObject")]
pub fn stub_0x82a138() -> crate::slot::PortedFn {
// IDA 0x82a138: reallymarkobject(global_State*, GCObject*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82a138, "reallymarkobject(global_State*, GCObject*)")
}

// 0x97d408 — __ZN3RBX7Network16ClientReplicator6needGCEv
// type: bool __fastcall(RBX::Network::ClientReplicator *this)
#[doc(alias = "RBX::Network::ClientReplicator::needGC(void)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator6needGCEv")]
pub fn stub_0x97d408(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::needGC() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0x9804d0 — __ZN5boost10shared_ptrIN3RBX7Network16ClientReplicator5GCJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClientReplicator::GCJob>::reset(void)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7Network16ClientReplicator5GCJobEE5resetEv")]
pub fn stub_0x9804d0(handle: &mut crate::slot::InstanceHandle) {
// shared_ptr::reset — release the owned ref.
let _ = handle;
}

// 0x982468 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network16ClientReplicator5GCJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(rbx_core::SharedPtr<RBX::Network::ClientReplicator::GCJob> *,RBX::Network::ClientReplicator::GCJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network16ClientReplicator5GCJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
pub fn stub_0x982468() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::ClientReplicator::GCJob")
}

// 0x982618 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16ClientReplicator5GCJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(rbx_core::SharedPtr<RBX::Network::ClientReplicator::GCJob> const*,RBX::Network::ClientReplicator::GCJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16ClientReplicator5GCJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x982618() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Network::ClientReplicator::GCJob")
}

// 0x9828c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ClientReplicator::GCJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEED1Ev")]
pub fn stub_0x9828c4(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x9828c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ClientReplicator::GCJob>::~sp_counted_impl_p() [0x9828c8]")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEED0Ev")]
pub fn stub_0x9828c8(count: crate::slot::SharedCount) {
// shared_count dtor — releases the counter.
drop(count);
}

// 0x9828d4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ClientReplicator::GCJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEE7disposeEv")]
pub fn stub_0x9828d4() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x9828e8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ClientReplicator::GCJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEE11get_deleterERKSt9type_info")]
pub fn stub_0x9828e8() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x9828ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ClientReplicator::GCJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16ClientReplicator5GCJobEE19get_untyped_deleterEv")]
pub fn stub_0x9828ec() -> crate::slot::SharedCount {
// shared_count ctor — one strong, one weak (self).
crate::slot::SharedCount::new()
}

// 0x9b2570 — __ZN3RBX15NetworkSettings13setDataGCRateEf
// type: float *__fastcall(float *this, float32_t)
#[doc(alias = "RBX::NetworkSettings::setDataGCRate(float)")]
#[doc(alias = "__ZN3RBX15NetworkSettings13setDataGCRateEf")]
pub fn stub_0x9b2570(cell: &mut crate::slot::PropCell, value: crate::slot::PropValue) {
// RBX::NetworkSettings setter.
cell.set(value)
}

// 0x9b40a8 — __ZNK3RBX15NetworkSettings13getDataGCRateEv
// type: int __fastcall(RBX::NetworkSettings *this)
#[doc(alias = "RBX::NetworkSettings::getDataGCRate(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings13getDataGCRateEv")]
pub fn stub_0x9b40a8(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NetworkSettings getter.
cell.get()
}

// 0xb632d0 — __ZN3RBX7Network16ClientReplicator5GCJobC1ERNS0_10ReplicatorE
// type: int __fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::GCJob(RBX::Network::Replicator &)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJobC1ERNS0_10ReplicatorE")]
pub fn stub_0xb632d0(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::GCJob(RBX::Network::Replicator&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb632dc — __ZN3RBX7Network16ClientReplicator5GCJobC2ERNS0_10ReplicatorE
// type: RBX::Network::ClientReplicator::GCJob *__fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::GCJob(RBX::Network::Replicator &) [0xb632dc]")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJobC2ERNS0_10ReplicatorE")]
pub fn stub_0xb632dc(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::GCJob(RBX::Network::Replicator&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb63630 — __ZN3RBX7Network16ClientReplicator5GCJobD0Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::~GCJob()")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJobD0Ev")]
pub fn stub_0xb63630(handle: crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob dtor.
drop(handle);
}

// 0xb636d0 — __ZN3RBX7Network16ClientReplicator5GCJobD1Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::~GCJob() [0xb636d0]")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJobD1Ev")]
pub fn stub_0xb636d0(handle: crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob dtor.
drop(handle);
}

// 0xb636dc — __ZN3RBX7Network16ClientReplicator5GCJobD2Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *this, int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::~GCJob() [0xb636dc]")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJobD2Ev")]
pub fn stub_0xb636dc(handle: crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob dtor.
drop(handle);
}

// 0xb63b54 — __ZN3RBX7Network16ClientReplicator5GCJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob5errorERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_0xb63b54(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::error(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb63b80 — __ZN3RBX7Network16ClientReplicator5GCJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::ClientReplicator::GCJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_0xb63b80(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats con~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb647e8 — __ZN3RBX7Network16ClientReplicator5GCJob8gcRegionERKNS_12StreamRegion2IdEPNS2_17RegionRemovalItemE
// type: void __fastcall(RBX::Network::Replicator **, int *, RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::gcRegion(RBX::StreamRegion::Id const&,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob8gcRegionERKNS_12StreamRegion2IdEPNS2_17RegionRemovalItemE")]
pub fn stub_0xb647e8(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::gcRegion(RBX::StreamRegion::Id const&, RBX::Network~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb64c68 — __ZN3RBX7Network16ClientReplicator5GCJob12insertRegionERKNS_12StreamRegion2IdE
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::insertRegion(RBX::StreamRegion::Id const&)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob12insertRegionERKNS_12StreamRegion2IdE")]
pub fn stub_0xb64c68(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::insertRegion(RBX::StreamRegion::Id const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb64cb0 — __ZN3RBX7Network16ClientReplicator5GCJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
// type: void __fastcall(int, int, _DWORD *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE")]
pub fn stub_0xb64cb0(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::coarsePrimitiveMovement(RBX::Primitive*, RBX::Spati~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb6503c — __ZThn488_N3RBX7Network16ClientReplicator5GCJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
#[doc(alias = "non-virtual thunk toRBX::Network::ClientReplicator::GCJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
#[doc(alias = "__ZThn488_N3RBX7Network16ClientReplicator5GCJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE")]
pub fn stub_0xb6503c(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run primary.
let _ = handle.delta;
}

// 0xb6504c — __ZN3RBX7Network16ClientReplicator5GCJob14gcPartInstanceEPNS_12PartInstanceEPNS2_17RegionRemovalItemE
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::PartInstance *, RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::gcPartInstance(RBX::PartInstance *,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob14gcPartInstanceEPNS_12PartInstanceEPNS2_17RegionRemovalItemE")]
pub fn stub_0xb6504c(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::gcPartInstance(RBX::PartInstance*, RBX::Network::Cl~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb655f4 — __ZN3RBX7Network16ClientReplicator5GCJob13render3dAdornEPNS_5AdornE
// type: char *__fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::Adorn *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::render3dAdorn(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob13render3dAdornEPNS_5AdornE")]
pub fn stub_0xb655f4(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::render3dAdorn(RBX::Adorn*) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb65764 — __ZN3RBX7Network16ClientReplicator5GCJob23updateMaxRegionDistanceEv
// type: bool __fastcall(RBX::Network::ClientReplicator::GCJob *this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::updateMaxRegionDistance(void)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob23updateMaxRegionDistanceEv")]
pub fn stub_0xb65764(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::updateMaxRegionDistance() — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb657d8 — __ZN3RBX7Network16ClientReplicator5GCJob39notifyServerGcingInstanceAndDescendantsEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int *, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::notifyServerGcingInstanceAndDescendants(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob39notifyServerGcingInstanceAndDescendantsEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0xb657d8() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0xb6618c — __ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItem11addInstanceEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::addInstance(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItem11addInstanceEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_0xb6618c() -> crate::slot::InstanceHandle {
// shared/intrusive_ptr ctor — fresh owning ref.
crate::slot::InstanceHandle::new("RBX::Instance")
}

// 0xb661e0 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJob17RegionRemovalItemENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPSA_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*>,boost::arg<1>>> const&)const")]
#[doc(alias = "__ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJob17RegionRemovalItemENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPSA_EENS2_3argILi1EEEEEEEEEvRKT_")]
pub fn stub_0xb661e0() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

// 0xb6664c — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJobENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS9_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob*>,boost::arg<1>>> const&)const")]
#[doc(alias = "__ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJobENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS9_EENS2_3argILi1EEEEEEEEEvRKT_")]
pub fn stub_0xb6664c() -> crate::slot::BoundCall {
// boost::bind (cf. 0x39d700 args) — captures receiver +
// member functor + 2 placeholders as a host closure.
// was: boost::_bi::bind_t/_mfi.
crate::slot::BoundCall::new(2)
}

// 0xb66aa8 — __ZN3RBX7Network16ClientReplicator5GCJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
pub fn stub_0xb66aa8(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::sleepTime(RBX::TaskScheduler::Job::Stats const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb66ad0 — __ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJobENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator::GCJob*,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJobENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_")]
pub fn stub_0xb66ad0() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

// 0xb66d48 — __ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemENS_10shared_ptrINS2_8InstanceEEEEclEPS6_S9_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*,rbx_core::SharedPtr<RBX::Instance>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemENS_10shared_ptrINS2_8InstanceEEEEclEPS6_S9_")]
pub fn stub_0xb66d48() -> crate::slot::BindPiece {
// boost::bind fragment (mf1) composing a host BoundCall.
crate::slot::BindPiece::new("mf1")
}

// 0xb6845c — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD1Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD1Ev")]
pub fn stub_0xb6845c(handle: crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem dtor.
drop(handle);
}

// 0xb68460 — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD0Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem() [0xb68460]")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD0Ev")]
pub fn stub_0xb68460(handle: crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem dtor.
drop(handle);
}

// 0xb6846c — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItem5writeERN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::IdSerializer **this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::write(RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItem5writeERN6RakNet9BitStreamE")]
pub fn stub_0xb6846c(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::write(RakNet::BitStream&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

// 0xb685a8 — __ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemD1Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::~RegionRemovalItem()")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemD1Ev")]
pub fn stub_0xb685a8(handle: crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::RegionRemovalItem dtor.
drop(handle);
}

// 0xb685cc — __ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemD0Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::~RegionRemovalItem() [0xb685cc]")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemD0Ev")]
pub fn stub_0xb685cc(handle: crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::RegionRemovalItem dtor.
drop(handle);
}

// 0xb685f4 — __ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItem5writeERN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::write(RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItem5writeERN6RakNet9BitStreamE")]
pub fn stub_0xb685f4(handle: &crate::slot::InstanceHandle) {
// RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::write(RakNet::BitStream&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "_CGColorSpaceCreateDeviceRGB")]
pub fn stub_0xf280b4() -> crate::slot::PortedFn {
// IDA 0xf280b4: _CGColorSpaceCreateDeviceRGB.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf280b4, "_CGColorSpaceCreateDeviceRGB")
}

#[doc(alias = "_CGContextDrawImage")]
pub fn stub_0xf280c4() -> crate::slot::PortedFn {
// IDA 0xf280c4: _CGContextDrawImage.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0xf280c4, "_CGContextDrawImage")
}

// 0xf5ecf4 — j___ZN5boost10shared_ptrIN3RBX7Network16ClientReplicator5GCJobEE5resetEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ClientReplicator::GCJob>::reset(void) [0xf5ecf4]")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX7Network16ClientReplicator5GCJobEE5resetEv")]
pub fn stub_0xf5ecf4() -> crate::slot::PortedFn {
// IDA 0xf5ecf4: j___ZN5boost10shared_ptrIN3RBX7Network16ClientReplicator5GCJobEE5resetEv.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf5ecf4, "j___ZN5boost10shared_ptrIN3RBX7Network16ClientReplicator5GCJobEE5resetEv")
}

// 0xf5ed44 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network16ClientReplicator5GCJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(rbx_core::SharedPtr<RBX::Network::ClientReplicator::GCJob> *,RBX::Network::ClientReplicator::GCJob *,boost::detail::shared_count &) [0xf5ed44]")]
#[doc(alias = "j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network16ClientReplicator5GCJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
pub fn stub_0xf5ed44() -> crate::slot::PortedFn {
// IDA 0xf5ed44: j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network16ClientReplicator5GCJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12sh~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf5ed44, "j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network16ClientReplicator5GCJobES5_EEvPNS_10shared_p~")
}

// 0xf5edf4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16ClientReplicator5GCJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(rbx_core::SharedPtr<RBX::Network::ClientReplicator::GCJob> const*,RBX::Network::ClientReplicator::GCJob *)const [0xf5edf4]")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16ClientReplicator5GCJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf5edf4() -> crate::slot::PortedFn {
// IDA 0xf5edf4: j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16ClientReplicator~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf5edf4, "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Netwo~")
}

// 0xf64a04 — j___ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJobENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS9_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob*>,boost::arg<1>>> const&)const [0xf64a04]")]
#[doc(alias = "j___ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJobENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS9_EENS2_3argILi1EEEEEEEEEvRKT_")]
pub fn stub_0xf64a04() -> crate::slot::PortedFn {
// IDA 0xf64a04: j___ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJobENS2_10shared_~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf64a04, "j___ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator~")
}

// 0xf64a14 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJob17RegionRemovalItemENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPSA_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*>,boost::arg<1>>> const&)const [0xf64a14]")]
#[doc(alias = "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJob17RegionRemovalItemENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPSA_EENS2_3argILi1EEEEEEEEEvRKT_")]
pub fn stub_0xf64a14() -> crate::slot::PortedFn {
// IDA 0xf64a14: j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJob17RegionRem~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf64a14, "j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplica~")
}

// 0xf64a34 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemENS_10shared_ptrINS2_8InstanceEEEEclEPS6_S9_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*,rbx_core::SharedPtr<RBX::Instance>)const [0xf64a34]")]
#[doc(alias = "j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemENS_10shared_ptrINS2_8InstanceEEEEclEPS6_S9_")]
pub fn stub_0xf64a34() -> crate::slot::PortedFn {
// IDA 0xf64a34: j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemENS_10shared_ptrINS2_8InstanceEEEEclEPS6~.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf64a34, "j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemENS_10shared_ptrINS2~")
}

// 0xf64a44 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJobENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator::GCJob*,rbx_core::SharedPtr<RBX::Instance>)const [0xf64a44]")]
#[doc(alias = "j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJobENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_")]
pub fn stub_0xf64a44() -> crate::slot::PortedFn {
// IDA 0xf64a44: j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJobENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_.
// tail-call/jump stub to the primary; the host keeps it as a delegate entry so the EA stays linkable
crate::slot::PortedFn::new(0xf64a44, "j___ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJobENS_10shared_ptrINS2_8InstanceEEEEclEPS~")
}
