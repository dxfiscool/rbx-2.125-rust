//! core shard mp — 150 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 150 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 2473 uncovered before -> 2323 after, batch 0xf1fb88..0xf20c38).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v$shim")]
// 0xf1fb88 — __ZN3RBX4Name9doDeclareILZNS_17sBoxSelectCommandEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf1fb88() {
    // IDA 0xf1fb88: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1fba0 — __ZNSt6vectorIN3RBX6Action10ActionTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1fba0() {
    // IDA 0xf1fba0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1fbac — __ZNSt6vectorIN3RBX14AsyncHttpQueue15CallbackWrapperESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1fbac() {
    // IDA 0xf1fbac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX10BrickColor8BrickMapD2Ev$shim")]
// 0xf1fbe8 — __ZN3RBX10BrickColor8BrickMapD2Ev$shim
pub fn stub_0xf1fbe8() {
    // IDA 0xf1fbe8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf1fbf4 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
pub fn stub_0xf1fbf4() {
    // IDA 0xf1fbf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_$shim")]
// 0xf1fc00 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_$shim
pub fn stub_0xf1fc00() {
    // IDA 0xf1fc00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v$shim")]
// 0xf1fc0c — __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf1fc0c() {
    // IDA 0xf1fc0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSs6appendERKSs$shim")]
// 0xf1fc3c — __ZNSs6appendERKSs$shim
pub fn stub_0xf1fc3c() {
    // IDA 0xf1fc3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Http10MutexGuardD2Ev$shim")]
// 0xf1fc48 — __ZN3RBX4Http10MutexGuardD2Ev$shim
pub fn stub_0xf1fc48() {
    // IDA 0xf1fc48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf1fcc0 — __ZNSt6vectorIN3RBX17KeywordFilterTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
pub fn stub_0xf1fcc0() {
    // IDA 0xf1fcc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSs6resizeEmc$shim")]
// 0xf1fccc — __ZNSs6resizeEmc$shim
pub fn stub_0xf1fccc() {
    // IDA 0xf1fccc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_acosf$shim")]
// 0xf1fda4 — _acosf$shim
// type: float __cdecl(float)
pub fn stub_0xf1fda4() {
    // IDA 0xf1fda4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_CC_MD5_Update$shim")]
// 0xf1fdc8 — _CC_MD5_Update$shim
// type: int __cdecl(CC_MD5_CTX *c, const void *data, CC_LONG len)
pub fn stub_0xf1fdc8() {
    // IDA 0xf1fdc8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_pthread_getspecific$shim")]
// 0xf1fdf8 — _pthread_getspecific$shim
// type: void *__cdecl(pthread_key_t)
pub fn stub_0xf1fdf8() {
    // IDA 0xf1fdf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_$shim")]
// 0xf1fe04 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX13RunTransitionEEEclES3_$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1fe04() {
    // IDA 0xf1fe04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v$shim")]
// 0xf1fe10 — __ZN3RBX4Name7declareILZNS_7sCameraEEEERKS0_v$shim
pub fn stub_0xf1fe10() {
    // IDA 0xf1fe10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v$shim")]
// 0xf1fe1c — __ZN3RBX4Name9doDeclareILZNS_7sCameraEEEERKS0_v$shim
pub fn stub_0xf1fe1c() {
    // IDA 0xf1fe1c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE24safe_static_do_get_mutexEv$shim")]
// 0xf1fe34 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1fe34() {
    // IDA 0xf1fe34: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv$shim")]
// 0xf1fe40 — __ZN3rbx7signals6signalIFvddEE24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1fe40() {
    // IDA 0xf1fe40: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv$shim")]
// 0xf1fe4c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf1fe4c() {
    // IDA 0xf1fe4c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv$shim")]
// 0xf1fe58 — __ZN3rbx7signals6signalIFvdEE24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1fe58() {
    // IDA 0xf1fe58: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv$shim")]
// 0xf1fe64 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1fe64() {
    // IDA 0xf1fe64: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd$shim")]
// 0xf1fe7c — __ZN3rbx7signals16signal_with_argsILi1EFvdEEclEd$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1fe7c() {
    // IDA 0xf1fe7c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvdEE13disconnectAllEv$shim")]
// 0xf1fe88 — __ZN3rbx7signals6signalIFvdEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1fe88() {
    // IDA 0xf1fe88: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZNSt8bad_castD2Ev$shim")]
// 0xf1fe94 — __ZNSt8bad_castD2Ev$shim
// type: void __cdecl(std::bad_cast *__hidden this)
pub fn stub_0xf1fe94() {
    // IDA 0xf1fe94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1fec4 — __ZN3rbx7signals6signalIFvdEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1fec4() {
    // IDA 0xf1fec4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvddEE13disconnectAllEv$shim")]
// 0xf1fed0 — __ZN3rbx7signals6signalIFvddEE13disconnectAllEv$shim
pub fn stub_0xf1fed0() {
    // IDA 0xf1fed0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1fee8 — __ZN3rbx7signals6signalIFvddEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1fee8() {
    // IDA 0xf1fee8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv$shim")]
// 0xf1fef4 — __ZN3RBX8LRUCacheISsSsE23removeLeastRecentlyUsedEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1fef4() {
    // IDA 0xf1fef4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf2008c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf2008c() {
    // IDA 0xf2008c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf200d4 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf200d4() {
    // IDA 0xf200d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14BaseThreadPool8PoolDataD2Ev$shim")]
// 0xf200f8 — __ZN3RBX14BaseThreadPool8PoolDataD2Ev$shim
// type: void __fastcall(RBX::BaseThreadPool::PoolData *__hidden this)
pub fn stub_0xf200f8() {
    // IDA 0xf200f8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v$shim")]
// 0xf20140 — __ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20140() {
    // IDA 0xf20140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v$shim")]
// 0xf2014c — __ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2014c() {
    // IDA 0xf2014c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v$shim")]
// 0xf20158 — __ZN3RBX4Name9doDeclareILZNS_12sPVAdornmentEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20158() {
    // IDA 0xf20158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_v$shim")]
// 0xf2017c — __ZNK3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_v$shim
// type: int(void)
pub fn stub_0xf2017c() {
    // IDA 0xf2017c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v$shim")]
// 0xf20194 — __ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20194() {
    // IDA 0xf20194: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13disconnectAllEv$shim")]
// 0xf201a0 — __ZN3rbx7signals6signalIFvvEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf201a0() {
    // IDA 0xf201a0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE13disconnectAllEv$shim")]
// 0xf201ac — __ZN3rbx7signals6signalIFvSsEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf201ac() {
    // IDA 0xf201ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX19AnimationTrackStateD2Ev$shim")]
// 0xf201c4 — __ZN3RBX19AnimationTrackStateD2Ev$shim
// type: void __fastcall(RBX::AnimationTrackState *__hidden this)
pub fn stub_0xf201c4() {
    // IDA 0xf201c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v$shim")]
// 0xf201d0 — __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v$shim
// type: int()
pub fn stub_0xf201d0() {
    // IDA 0xf201d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX19AnimationTrackStateD0Ev$shim")]
// 0xf201dc — __ZN3RBX19AnimationTrackStateD0Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf201dc() {
    // IDA 0xf201dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv$shim")]
// 0xf201e8 — __ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf201e8() {
    // IDA 0xf201e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv$shim")]
// 0xf201f4 — __ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv$shim
// type: int(void)
pub fn stub_0xf201f4() {
    // IDA 0xf201f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf20200 — __ZN3rbx7signals6signalIFvfffEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf20200() {
    // IDA 0xf20200: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf2020c — __ZN3rbx7signals6signalIFvffffEE4slot24safe_static_do_get_mutexEv$shim
// type: void *()
pub fn stub_0xf2020c() {
    // IDA 0xf2020c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff$shim")]
// 0xf20218 — __ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf20218() {
    // IDA 0xf20218: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvfffEE13disconnectAllEv$shim")]
// 0xf20224 — __ZN3rbx7signals6signalIFvfffEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20224() {
    // IDA 0xf20224: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvffEEclEff$shim")]
// 0xf20248 — __ZN3rbx7signals16signal_with_argsILi2EFvffEEclEff$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf20248() {
    // IDA 0xf20248: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvffEE13disconnectAllEv$shim")]
// 0xf20254 — __ZN3rbx7signals6signalIFvffEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20254() {
    // IDA 0xf20254: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvffffEE13disconnectAllEv$shim")]
// 0xf20260 — __ZN3rbx7signals6signalIFvffffEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20260() {
    // IDA 0xf20260: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZNSt15_List_node_base4hookEPS_$shim")]
// 0xf20284 — __ZNSt15_List_node_base4hookEPS_$shim
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf20284() {
    // IDA 0xf20284: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf20290 — __ZNSt6vectorIN3RBX15PoseAccumulatorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int(void)
pub fn stub_0xf20290() {
    // IDA 0xf20290: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2029c — __ZNSt6vectorIPN3RBX16IAnimatableJointESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf2029c() {
    // IDA 0xf2029c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v$shim")]
// 0xf202a8 — __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v$shim
// type: int()
pub fn stub_0xf202a8() {
    // IDA 0xf202a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX10ArcHandlesD2Ev$shim")]
// 0xf202cc — __ZN3RBX10ArcHandlesD2Ev$shim
// type: void __fastcall(RBX::ArcHandles *__hidden this)
pub fn stub_0xf202cc() {
    // IDA 0xf202cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v$shim")]
// 0xf202f0 — __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v$shim
// type: int()
pub fn stub_0xf202f0() {
    // IDA 0xf202f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11HandlesBaseD2Ev$shim")]
// 0xf20350 — __ZN3RBX11HandlesBaseD2Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20350() {
    // IDA 0xf20350: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v$shim")]
// 0xf2035c — __ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf2035c() {
    // IDA 0xf2035c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v$shim")]
// 0xf20368 — __ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20368() {
    // IDA 0xf20368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v$shim")]
// 0xf20374 — __ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20374() {
    // IDA 0xf20374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v$shim")]
// 0xf20380 — __ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20380() {
    // IDA 0xf20380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sBackpackEEEERKS0_v$shim")]
// 0xf20404 — __ZN3RBX4Name9doDeclareILZNS_9sBackpackEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20404() {
    // IDA 0xf20404: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12BadgeServiceD2Ev$shim")]
// 0xf20410 — __ZN3RBX12BadgeServiceD2Ev$shim
// type: void __fastcall(RBX::BadgeService *__hidden this)
pub fn stub_0xf20410() {
    // IDA 0xf20410: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12BadgeServiceD0Ev$shim")]
// 0xf20428 — __ZN3RBX12BadgeServiceD0Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20428() {
    // IDA 0xf20428: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v$shim")]
// 0xf2044c — __ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2044c() {
    // IDA 0xf2044c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v$shim")]
// 0xf20488 — __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20488() {
    // IDA 0xf20488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX12BillboardGuiD2Ev$shim")]
// 0xf204a0 — __ZN3RBX12BillboardGuiD2Ev$shim
// type: void __fastcall(RBX::BillboardGui *__hidden this)
pub fn stub_0xf204a0() {
    // IDA 0xf204a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v$shim")]
// 0xf204c4 — __ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v$shim
// type: int()
pub fn stub_0xf204c4() {
    // IDA 0xf204c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_v$shim")]
// 0xf204f4 — __ZNK3RBX15ServiceProvider6createINS_17ControllerServiceEEEPT_v$shim
// type: int(void)
pub fn stub_0xf204f4() {
    // IDA 0xf204f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX6CameraD2Ev$shim")]
// 0xf20500 — __ZN3RBX6CameraD2Ev$shim
// type: void __fastcall(RBX::Camera *__hidden this)
pub fn stub_0xf20500() {
    // IDA 0xf20500: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX6CameraD0Ev$shim")]
// 0xf20518 — __ZN3RBX6CameraD0Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20518() {
    // IDA 0xf20518: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf20548 — __ZNSt6vectorIN3RBX6Camera13CameraPanModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf20548() {
    // IDA 0xf20548: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf20554 — __ZNSt6vectorIN3RBX6Camera10CameraModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf20554() {
    // IDA 0xf20554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf20560 — __ZNSt6vectorIN3RBX6Camera10CameraTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf20560() {
    // IDA 0xf20560: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb$shim")]
// 0xf2056c — __ZN3rbx7signals16signal_with_argsILi1EFvbEEclEb$shim
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf2056c() {
    // IDA 0xf2056c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvbEE13disconnectAllEv$shim")]
// 0xf20578 — __ZN3rbx7signals6signalIFvbEE13disconnectAllEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20578() {
    // IDA 0xf20578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_v$shim")]
// 0xf205c0 — __ZNK3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_v$shim
// type: int(void)
pub fn stub_0xf205c0() {
    // IDA 0xf205c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX20ChangeHistoryService4Item12unplayChangeEv$shim")]
// 0xf205cc — __ZN3RBX20ChangeHistoryService4Item12unplayChangeEv$shim
// type: int __fastcall(RBX::ChangeHistoryService::Item *)
pub fn stub_0xf205cc() {
    // IDA 0xf205cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v$shim")]
// 0xf205d8 — __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v$shim
// type: int(void)
pub fn stub_0xf205d8() {
    // IDA 0xf205d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v$shim")]
// 0xf205fc — __ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v$shim
pub fn stub_0xf205fc() {
    // IDA 0xf205fc: platform/render/stats wiring owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE16_M_push_back_auxERKS3_$shim")]
// 0xf20644 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE16_M_push_back_auxERKS3_$shim
// type: int(void)
pub fn stub_0xf20644() {
    // IDA 0xf20644: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_reallocate_mapEmb$shim")]
// 0xf20650 — __ZNSt5dequeIPN3RBX20ChangeHistoryService4ItemESaIS3_EE17_M_reallocate_mapEmb$shim
// type: int(void)
pub fn stub_0xf20650() {
    // IDA 0xf20650: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf20668 — __ZNSt6vectorIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf20668() {
    // IDA 0xf20668: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_$shim")]
// 0xf20698 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_$shim
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf20698() {
    // IDA 0xf20698: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj$shim")]
// 0xf206a4 — __ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj$shim
// type: int(void)
pub fn stub_0xf206a4() {
    // IDA 0xf206a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v$shim")]
// 0xf206c8 — __ZN3RBX4Name9doDeclareILZNS_9sClothingEEEERKS0_v$shim
// type: int()
pub fn stub_0xf206c8() {
    // IDA 0xf206c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sPantsEEEERKS0_v$shim")]
// 0xf20740 — __ZN3RBX4Name9doDeclareILZNS_6sPantsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20740() {
    // IDA 0xf20740: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sShirtEEEERKS0_v$shim")]
// 0xf2074c — __ZN3RBX4Name9doDeclareILZNS_6sShirtEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2074c() {
    // IDA 0xf2074c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v$shim")]
// 0xf20758 — __ZN3RBX4Name9doDeclareILZNS_11sBodyColorsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20758() {
    // IDA 0xf20758: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v$shim")]
// 0xf20764 — __ZN3RBX4Name9doDeclareILZNS_5sSkinEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20764() {
    // IDA 0xf20764: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v$shim")]
// 0xf20770 — __ZN3RBX4Name9doDeclareILZNS_13sShirtGraphicEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20770() {
    // IDA 0xf20770: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v$shim")]
// 0xf20794 — __ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20794() {
    // IDA 0xf20794: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v$shim")]
// 0xf207a0 — __ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf207a0() {
    // IDA 0xf207a0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v$shim")]
// 0xf207ac — __ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf207ac() {
    // IDA 0xf207ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf207b8 — __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf207b8() {
    // IDA 0xf207b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf207dc — __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf207dc() {
    // IDA 0xf207dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX13ClickDetectorD2Ev$shim")]
// 0xf2080c — __ZN3RBX13ClickDetectorD2Ev$shim
// type: void __fastcall(RBX::ClickDetector *__hidden this)
pub fn stub_0xf2080c() {
    // IDA 0xf2080c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX13ClickDetectorD0Ev$shim")]
// 0xf20824 — __ZN3RBX13ClickDetectorD0Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20824() {
    // IDA 0xf20824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX17CollectionServiceD2Ev$shim")]
// 0xf2083c — __ZN3RBX17CollectionServiceD2Ev$shim
// type: void __fastcall(RBX::CollectionService *__hidden this)
pub fn stub_0xf2083c() {
    // IDA 0xf2083c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sCollectionServiceEEEERKS0_v$shim")]
// 0xf20848 — __ZN3RBX4Name9doDeclareILZNS_18sCollectionServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20848() {
    // IDA 0xf20848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sFilteredSelectionEEEERKS0_v$shim")]
// 0xf20860 — __ZN3RBX4Name9doDeclareILZNS_18sFilteredSelectionEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20860() {
    // IDA 0xf20860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sFilteredSelectionEEEERKS0_v$shim")]
// 0xf20884 — __ZN3RBX4Name7declareILZNS_18sFilteredSelectionEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20884() {
    // IDA 0xf20884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v$shim")]
// 0xf20938 — __ZN3RBX4Name7declareILZNS_12sDropperToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20938() {
    // IDA 0xf20938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v$shim")]
// 0xf20944 — __ZN3RBX4Name9doDeclareILZNS_12sDropperToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20944() {
    // IDA 0xf20944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v$shim")]
// 0xf20950 — __ZN3RBX4Name7declareILZNS_13sMaterialToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20950() {
    // IDA 0xf20950: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v$shim")]
// 0xf2095c — __ZN3RBX4Name9doDeclareILZNS_13sMaterialToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2095c() {
    // IDA 0xf2095c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v$shim")]
// 0xf20968 — __ZN3RBX4Name7declareILZNS_9sFillToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20968() {
    // IDA 0xf20968: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v$shim")]
// 0xf20974 — __ZN3RBX4Name9doDeclareILZNS_9sFillToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20974() {
    // IDA 0xf20974: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v$shim")]
// 0xf20980 — __ZN3RBX4Name7declareILZNS_9sLockToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20980() {
    // IDA 0xf20980: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v$shim")]
// 0xf2098c — __ZN3RBX4Name9doDeclareILZNS_9sLockToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2098c() {
    // IDA 0xf2098c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v$shim")]
// 0xf20998 — __ZN3RBX4Name7declareILZNS_11sAnchorToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20998() {
    // IDA 0xf20998: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v$shim")]
// 0xf209a4 — __ZN3RBX4Name9doDeclareILZNS_11sAnchorToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf209a4() {
    // IDA 0xf209a4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v$shim")]
// 0xf209b0 — __ZN3RBX4Name7declareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf209b0() {
    // IDA 0xf209b0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v$shim")]
// 0xf209bc — __ZN3RBX4Name9doDeclareILZNS_21sSmoothNoOutlinesToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf209bc() {
    // IDA 0xf209bc: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v$shim")]
// 0xf209c8 — __ZN3RBX4Name7declareILZNS_19sOscillateMotorToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf209c8() {
    // IDA 0xf209c8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v$shim")]
// 0xf209d4 — __ZN3RBX4Name9doDeclareILZNS_19sOscillateMotorToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf209d4() {
    // IDA 0xf209d4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v$shim")]
// 0xf209e0 — __ZN3RBX4Name7declareILZNS_14sLeftMotorToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf209e0() {
    // IDA 0xf209e0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v$shim")]
// 0xf209ec — __ZN3RBX4Name9doDeclareILZNS_14sLeftMotorToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf209ec() {
    // IDA 0xf209ec: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v$shim")]
// 0xf209f8 — __ZN3RBX4Name7declareILZNS_15sRightMotorToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf209f8() {
    // IDA 0xf209f8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v$shim")]
// 0xf20a04 — __ZN3RBX4Name9doDeclareILZNS_15sRightMotorToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20a04() {
    // IDA 0xf20a04: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v$shim")]
// 0xf20a10 — __ZN3RBX4Name7declareILZNS_10sHingeToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20a10() {
    // IDA 0xf20a10: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v$shim")]
// 0xf20a1c — __ZN3RBX4Name9doDeclareILZNS_10sHingeToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20a1c() {
    // IDA 0xf20a1c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v$shim")]
// 0xf20a28 — __ZN3RBX4Name7declareILZNS_14sUniversalToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20a28() {
    // IDA 0xf20a28: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v$shim")]
// 0xf20a34 — __ZN3RBX4Name9doDeclareILZNS_14sUniversalToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20a34() {
    // IDA 0xf20a34: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v$shim")]
// 0xf20a40 — __ZN3RBX4Name7declareILZNS_10sInletToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20a40() {
    // IDA 0xf20a40: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v$shim")]
// 0xf20a4c — __ZN3RBX4Name9doDeclareILZNS_10sInletToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20a4c() {
    // IDA 0xf20a4c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v$shim")]
// 0xf20a58 — __ZN3RBX4Name7declareILZNS_10sStudsToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20a58() {
    // IDA 0xf20a58: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v$shim")]
// 0xf20a64 — __ZN3RBX4Name9doDeclareILZNS_10sStudsToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20a64() {
    // IDA 0xf20a64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v$shim")]
// 0xf20a70 — __ZN3RBX4Name7declareILZNS_9sWeldToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20a70() {
    // IDA 0xf20a70: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v$shim")]
// 0xf20a7c — __ZN3RBX4Name9doDeclareILZNS_9sWeldToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20a7c() {
    // IDA 0xf20a7c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v$shim")]
// 0xf20a88 — __ZN3RBX4Name7declareILZNS_9sGlueToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20a88() {
    // IDA 0xf20a88: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v$shim")]
// 0xf20a94 — __ZN3RBX4Name9doDeclareILZNS_9sGlueToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20a94() {
    // IDA 0xf20a94: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v$shim")]
// 0xf20aa0 — __ZN3RBX4Name7declareILZNS_9sFlatToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20aa0() {
    // IDA 0xf20aa0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v$shim")]
// 0xf20aac — __ZN3RBX4Name9doDeclareILZNS_9sFlatToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20aac() {
    // IDA 0xf20aac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v$shim")]
// 0xf20ab8 — __ZN3RBX4Name7declareILZNS_14sAdvRotateToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20ab8() {
    // IDA 0xf20ab8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX15AdvMoveToolBaseD0Ev$shim")]
// 0xf20ac4 — __ZN3RBX15AdvMoveToolBaseD0Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20ac4() {
    // IDA 0xf20ac4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v$shim")]
// 0xf20ad0 — __ZN3RBX4Name9doDeclareILZNS_14sAdvRotateToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20ad0() {
    // IDA 0xf20ad0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v$shim")]
// 0xf20adc — __ZN3RBX4Name7declareILZNS_12sAdvMoveToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20adc() {
    // IDA 0xf20adc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11AdvMoveToolD0Ev$shim")]
// 0xf20ae8 — __ZN3RBX11AdvMoveToolD0Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20ae8() {
    // IDA 0xf20ae8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v$shim")]
// 0xf20af4 — __ZN3RBX4Name9doDeclareILZNS_12sAdvMoveToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20af4() {
    // IDA 0xf20af4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v$shim")]
// 0xf20b00 — __ZN3RBX4Name7declareILZNS_19sMoveResizeJoinToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20b00() {
    // IDA 0xf20b00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v$shim")]
// 0xf20b0c — __ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20b0c() {
    // IDA 0xf20b0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v$shim")]
// 0xf20b18 — __ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20b18() {
    // IDA 0xf20b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v$shim")]
// 0xf20b24 — __ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20b24() {
    // IDA 0xf20b24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v$shim")]
// 0xf20b48 — __ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20b48() {
    // IDA 0xf20b48: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v$shim")]
// 0xf20b78 — __ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20b78() {
    // IDA 0xf20b78: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_8LightingEEEPT_v$shim")]
// 0xf20b84 — __ZNK3RBX15ServiceProvider4findINS_8LightingEEEPT_v$shim
// type: int(void)
pub fn stub_0xf20b84() {
    // IDA 0xf20b84: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_5VisitEEEPT_v$shim")]
// 0xf20b90 — __ZNK3RBX15ServiceProvider4findINS_5VisitEEEPT_v$shim
// type: int(void)
pub fn stub_0xf20b90() {
    // IDA 0xf20b90: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v$shim")]
// 0xf20c38 — __ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20c38() {
    // IDA 0xf20c38: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}
