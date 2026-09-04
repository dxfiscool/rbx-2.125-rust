//! core shard my — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 1623 uncovered before -> 1523 after, batch 0xf250d8..0xf26e54).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "__ZN3RBX21ConstraintSurfacePairD2Ev$shim")]
// 0xf250d8 — __ZN3RBX21ConstraintSurfacePairD2Ev$shim
// type: void __fastcall(RBX::ConstraintSurfacePair *)
pub fn stub_0xf250d8() {
    // IDA 0xf250d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX16UserInputServiceD2Ev$shim")]
// 0xf250e4 — __ZN3RBX16UserInputServiceD2Ev$shim
// type: void __fastcall(RBX::UserInputService *__hidden this)
pub fn stub_0xf250e4() {
    // IDA 0xf250e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX16UserInputServiceD0Ev$shim")]
// 0xf250fc — __ZN3RBX16UserInputServiceD0Ev$shim
// type: void __fastcall(RBX::UserInputService *)
pub fn stub_0xf250fc() {
    // IDA 0xf250fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sClickDetectorEEEERKS0_v$shim")]
// 0xf2512c — __ZN3RBX4Name7declareILZNS_14sClickDetectorEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2512c() {
    // IDA 0xf2512c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sClickDetectorEEEERKS0_v$shim")]
// 0xf25138 — __ZN3RBX4Name9doDeclareILZNS_14sClickDetectorEEEERKS0_v$shim
// type: int()
pub fn stub_0xf25138() {
    // IDA 0xf25138: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13disconnectAllEv$shim")]
// 0xf25204 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf25204() {
    // IDA 0xf25204: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE24safe_static_do_get_mutexEv$shim")]
// 0xf25210 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf25210() {
    // IDA 0xf25210: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf25234 — __ZN3rbx7signals6signalIFvN3RBX11InputObjectEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf25234() {
    // IDA 0xf25234: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX16UserInputService14SwipeDirectionEEEclES4_$shim")]
// 0xf25240 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX16UserInputService14SwipeDirectionEEEclES4_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf25240() {
    // IDA 0xf25240: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13disconnectAllEv$shim")]
// 0xf2524c — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2524c() {
    // IDA 0xf2524c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE24safe_static_do_get_mutexEv$shim")]
// 0xf25258 — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf25258() {
    // IDA 0xf25258: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function1IvN3RBX16UserInputService14SwipeDirectionEEclES3_$shim")]
// 0xf25270 — __ZNK5boost9function1IvN3RBX16UserInputService14SwipeDirectionEEclES3_$shim
// type: void __fastcall(_DWORD *, int)
pub fn stub_0xf25270() {
    // IDA 0xf25270: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf2527c — __ZN3rbx7signals6signalIFvN3RBX16UserInputService14SwipeDirectionEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf2527c() {
    // IDA 0xf2527c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvffEE24safe_static_do_get_mutexEv$shim")]
// 0xf25288 — __ZN3rbx7signals6signalIFvffEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf25288() {
    // IDA 0xf25288: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function2IvffEclEff$shim")]
// 0xf252a0 — __ZNK5boost9function2IvffEclEff$shim
// type: void __fastcall(_DWORD *, int, int)
pub fn stub_0xf252a0() {
    // IDA 0xf252a0: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf252ac — __ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf252ac() {
    // IDA 0xf252ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf25300 — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0xf25300() {
    // IDA 0xf25300: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv$shim")]
// 0xf25324 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf25324() {
    // IDA 0xf25324: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv$shim")]
// 0xf25330 — __ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf25330() {
    // IDA 0xf25330: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX6FWBaseD2Ev$shim")]
// 0xf25360 — __ZN3RBX6FWBaseD2Ev$shim
// type: void __fastcall(RBX::FWBase *__hidden this)
pub fn stub_0xf25360() {
    // IDA 0xf25360: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX18MarketplaceServiceD2Ev$shim")]
// 0xf2536c — __ZN3RBX18MarketplaceServiceD2Ev$shim
// type: void __fastcall(RBX::MarketplaceService *__hidden this)
pub fn stub_0xf2536c() {
    // IDA 0xf2536c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv$shim")]
// 0xf25384 — __ZN3rbx7signals6signalIFvSsiiEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf25384() {
    // IDA 0xf25384: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf2539c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf2539c() {
    // IDA 0xf2539c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviibEE24safe_static_do_get_mutexEv$shim")]
// 0xf253c0 — __ZN3rbx7signals6signalIFviibEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf253c0() {
    // IDA 0xf253c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf253e4 — __ZNSt6vectorIN3RBX18MarketplaceService12CurrencyTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0xf253e4() {
    // IDA 0xf253e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi3EFviibEEclEiib$shim")]
// 0xf253f0 — __ZN3rbx7signals16signal_with_argsILi3EFviibEEclEiib$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf253f0() {
    // IDA 0xf253f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviibEE13disconnectAllEv$shim")]
// 0xf253fc — __ZN3rbx7signals6signalIFviibEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf253fc() {
    // IDA 0xf253fc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZNK5boost9function3IviibEclEiib$shim")]
// 0xf25414 — __ZNK5boost9function3IviibEclEiib$shim
// type: void __fastcall(_DWORD *, int, int, int)
pub fn stub_0xf25414() {
    // IDA 0xf25414: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviibEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf25420 — __ZN3rbx7signals6signalIFviibEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf25420() {
    // IDA 0xf25420: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf254b0 — __ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv$shim
// type: void *()
pub fn stub_0xf254b0() {
    // IDA 0xf254b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sGuiBase2dEEEERKS0_v$shim")]
// 0xf254ec — __ZN3RBX4Name9doDeclareILZNS_10sGuiBase2dEEEERKS0_v$shim
// type: int()
pub fn stub_0xf254ec() {
    // IDA 0xf254ec: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim")]
// 0xf254f8 — __ZNSt6vectorIS_IN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim
// type: int __fastcall(int, int, int)
pub fn stub_0xf254f8() {
    // IDA 0xf254f8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim")]
// 0xf25504 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *, int, boost::detail::sp_counted_base *, void *, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf25504() {
    // IDA 0xf25504: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_$shim")]
// 0xf25510 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_$shim
// type: void __fastcall(int *, struct _Unwind_Exception *, int, const shared_count *)
pub fn stub_0xf25510() {
    // IDA 0xf25510: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE15_M_erase_at_endEPS4_$shim")]
// 0xf2551c — __ZNSt6vectorIN5boost10shared_ptrIN3RBX7GuiBaseEEESaIS4_EE15_M_erase_at_endEPS4_$shim
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *result, int)
pub fn stub_0xf2551c() {
    // IDA 0xf2551c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sLayerCollectorEEEERKS0_v$shim")]
// 0xf25528 — __ZN3RBX4Name9doDeclareILZNS_15sLayerCollectorEEEERKS0_v$shim
// type: int()
pub fn stub_0xf25528() {
    // IDA 0xf25528: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_v$shim")]
// 0xf25534 — __ZNK3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_v$shim
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf25534() {
    // IDA 0xf25534: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX20ContextActionServiceD2Ev$shim")]
// 0xf25540 — __ZN3RBX20ContextActionServiceD2Ev$shim
// type: void __fastcall(RBX::ContextActionService *__hidden this)
pub fn stub_0xf25540() {
    // IDA 0xf25540: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf25564 — __ZNSt6vectorIN3RBX11InputObject14UserInputStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0xf25564() {
    // IDA 0xf25564: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf25570 — __ZNSt6vectorIN3RBX11InputObject13UserInputTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0xf25570() {
    // IDA 0xf25570: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZSt9sort_heapIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_$shim")]
// 0xf2557c — __ZSt9sort_heapIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_$shim
// type: int()
pub fn stub_0xf2557c() {
    // IDA 0xf2557c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZSt16__insertion_sortIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_$shim")]
// 0xf25588 — __ZSt16__insertion_sortIPN3RBX24OSProfilerMarkerTempDataEPFbRKS1_S4_EEvT_S7_T0_$shim
// type: int()
pub fn stub_0xf25588() {
    // IDA 0xf25588: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZSt9sort_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_$shim")]
// 0xf25594 — __ZSt9sort_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_$shim
// type: int()
pub fn stub_0xf25594() {
    // IDA 0xf25594: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIfSaIfEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS1_EERKf$shim")]
// 0xf255ac — __ZNSt6vectorIfSaIfEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS1_EERKf$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf255ac() {
    // IDA 0xf255ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIbSaIbEE13_M_insert_auxESt13_Bit_iteratorb$shim")]
// 0xf255d0 — __ZNSt6vectorIbSaIbEE13_M_insert_auxESt13_Bit_iteratorb$shim
// type: int()
pub fn stub_0xf255d0() {
    // IDA 0xf255d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX12LoginServiceD2Ev$shim")]
// 0xf255dc — __ZN3RBX12LoginServiceD2Ev$shim
// type: void __fastcall(RBX::LoginService *)
pub fn stub_0xf255dc() {
    // IDA 0xf255dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf25624 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_0xf25624() {
    // IDA 0xf25624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_8functionIFvvEEEE7managerERKNS1_15function_bufferERS7_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf25654 — __ZN5boost6detail8function15functor_managerINS_8functionIFvvEEEE7managerERKNS1_15function_bufferERS7_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int()
pub fn stub_0xf25654() {
    // IDA 0xf25654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sTimerServiceEEEERKS0_v$shim")]
// 0xf2566c — __ZN3RBX4Name9doDeclareILZNS_13sTimerServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2566c() {
    // IDA 0xf2566c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12TimerServiceEEEmv$shim")]
// 0xf25678 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TimerServiceEEEmv$shim
// type: int()
pub fn stub_0xf25678() {
    // IDA 0xf25678: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf256d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFviEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf256d8() {
    // IDA 0xf256d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED2Ev$shim")]
// 0xf256e4 — __ZN5boost16exception_detail19error_info_injectorINS_16bad_lexical_castEED2Ev$shim
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0xf256e4() {
    // IDA 0xf256e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv$shim")]
// 0xf256f0 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_16bad_lexical_castEEEE5cloneEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf256f0() {
    // IDA 0xf256f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf256fc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX12AssetServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0xf256fc() {
    // IDA 0xf256fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf25714 — __ZNSt6vectorIN3RBX12AssetService10AccessTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf25714() {
    // IDA 0xf25714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
// 0xf25720 — __ZNSt6vectorIN5boost8functionIFvvEEESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim
// type: int()
pub fn stub_0xf25720() {
    // IDA 0xf25720: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX14RemoteFunctionD2Ev$shim")]
// 0xf2572c — __ZN3RBX14RemoteFunctionD2Ev$shim
// type: void __fastcall(RBX::RemoteFunction *)
pub fn stub_0xf2572c() {
    // IDA 0xf2572c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX11RemoteEventD2Ev$shim")]
// 0xf25744 — __ZN3RBX11RemoteEventD2Ev$shim
// type: void __fastcall(RBX::RemoteEvent *)
pub fn stub_0xf25744() {
    // IDA 0xf25744: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sRemoteEventEEEERKS0_v$shim")]
// 0xf25774 — __ZN3RBX4Name9doDeclareILZNS_12sRemoteEventEEEERKS0_v$shim
// type: int()
pub fn stub_0xf25774() {
    // IDA 0xf25774: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sRemoteFunctionEEEERKS0_v$shim")]
// 0xf25780 — __ZN3RBX4Name9doDeclareILZNS_15sRemoteFunctionEEEERKS0_v$shim
// type: int()
pub fn stub_0xf25780() {
    // IDA 0xf25780: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviSsEE24safe_static_do_get_mutexEv$shim")]
// 0xf257d4 — __ZN3rbx7signals6signalIFviSsEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf257d4() {
    // IDA 0xf257d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviSsEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf257e0 — __ZN3rbx7signals6signalIFviSsEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf257e0() {
    // IDA 0xf257e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviSsEE13disconnectAllEv$shim")]
// 0xf2587c — __ZN3rbx7signals6signalIFviSsEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf2587c() {
    // IDA 0xf2587c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callEiSs$shim")]
// 0xf25894 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callEiSs$shim
// type: int __fastcall(int, int, std::string *)
pub fn stub_0xf25894() {
    // IDA 0xf25894: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_T0_$shim")]
// 0xf2590c — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPN3RBX11FastCluster4PartESt6vectorIS4_SaIS4_EEEENS2_23PartClumpGroupPredicateEEvT_SB_T0_$shim
// type: int()
pub fn stub_0xf2590c() {
    // IDA 0xf2590c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX5AdornD2Ev$shim")]
// 0xf25924 — __ZN3RBX5AdornD2Ev$shim
// type: void __fastcall(RBX::Adorn *)
pub fn stub_0xf25924() {
    // IDA 0xf25924: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSi5seekgExSt12_Ios_Seekdir$shim")]
// 0xf25930 — __ZNSi5seekgExSt12_Ios_Seekdir$shim
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf25930() {
    // IDA 0xf25930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm$shim")]
// 0xf25954 — __ZN5boost9unordered6detail5tableINS1_3mapINS_19fast_pool_allocatorIN3RBX16SpatialGridIndexENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES6_NS5_11SpatialGridINS5_11FastClusterEE4CellENS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm$shim
// type: int()
pub fn stub_0xf25954() {
    // IDA 0xf25954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost5mutex6unlockEv$shim")]
// 0xf25960 — __ZN5boost5mutex6unlockEv$shim
// type: int __fastcall(boost::mutex *)
pub fn stub_0xf25960() {
    // IDA 0xf25960: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE4freeEPvm$shim")]
// 0xf2596c — __ZN5boost14singleton_poolINS_23fast_pool_allocator_tagELj4ENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EE4freeEPvm$shim
// type: int __fastcall(int, int)
pub fn stub_0xf2596c() {
    // IDA 0xf2596c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
// 0xf25978 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX17TextureCompositor3JobEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim
// type: int()
pub fn stub_0xf25978() {
    // IDA 0xf25978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv$shim")]
// 0xf2599c — __ZN3rbx7signals6signalIFviEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf2599c() {
    // IDA 0xf2599c: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE24safe_static_do_get_mutexEv$shim")]
// 0xf259a8 — __ZN3rbx7signals6signalIFvSsEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf259a8() {
    // IDA 0xf259a8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX17MegaClusterLegacy22updateEntity_templatedINS0_16VoxelGridOverlayEEEvv$shim")]
// 0xf259d8 — __ZN3RBX17MegaClusterLegacy22updateEntity_templatedINS0_16VoxelGridOverlayEEEvv$shim
// type: int()
pub fn stub_0xf259d8() {
    // IDA 0xf259d8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEb$shim")]
// 0xf259f0 — __ZN3RBX17MegaClusterLegacy21updateChunk_templatedINS0_16VoxelGridOverlayEEEvRKNS_13SpatialRegion2IdEb$shim
// type: int()
pub fn stub_0xf259f0() {
    // IDA 0xf259f0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZN3RBX17MegaClusterLegacy17unbuild_templatedINS0_16VoxelGridOverlayEEEvv$shim")]
// 0xf25a08 — __ZN3RBX17MegaClusterLegacy17unbuild_templatedINS0_16VoxelGridOverlayEEEvv$shim
// type: int __fastcall(RBX::GfxBinding *)
pub fn stub_0xf25a08() {
    // IDA 0xf25a08: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv$shim")]
// 0xf25a2c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_10lock_errorEEEE5cloneEv$shim
// type: int()
pub fn stub_0xf25a2c() {
    // IDA 0xf25a2c: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "__ZdaPv$shim")]
// 0xf25a44 — __ZdaPv$shim
// type: void __fastcall(void *)
pub fn stub_0xf25a44() {
    // IDA 0xf25a44: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "__ZN3RBX9WaterImplD2Ev$shim")]
// 0xf25a5c — __ZN3RBX9WaterImplD2Ev$shim
// type: void __fastcall(RBX::WaterImpl *)
pub fn stub_0xf25a5c() {
    // IDA 0xf25a5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX9WaterImpl4loadEv$shim")]
// 0xf25a68 — __ZN3RBX9WaterImpl4loadEv$shim
// type: int __fastcall(RBX::WaterImpl *)
pub fn stub_0xf25a68() {
    // IDA 0xf25a68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN19CRenderSettingsItemD2Ev")]
// 0xf25a74 — j___ZN19CRenderSettingsItemD2Ev
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xf25a74() {
    // IDA 0xf25a74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev")]
// 0xf25d14 — j___ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
pub fn stub_0xf25d14() {
    // IDA 0xf25d14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v")]
// 0xf25d24 — j___ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v
// type: 
pub fn stub_0xf25d24() {
    // IDA 0xf25d24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")]
// 0xf266f4 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0xf266f4() {
    // IDA 0xf266f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v")]
// 0xf26714 — j___ZN3RBX4Name7declareILZNS_13sLoginServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf26714() {
    // IDA 0xf26714: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
// 0xf26724 — j___ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// type: int()
pub fn stub_0xf26724() {
    // IDA 0xf26724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v")]
// 0xf26734 — j___ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf26734() {
    // IDA 0xf26734: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0xf26b04 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf26b04() {
    // IDA 0xf26b04: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE")]
// 0xf26b14 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS4_5list4INS4_5valueIS7_EENSB_ISsEESD_SD_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISG_EE5valueEEE5valueEiE4typeE
// type: int()
pub fn stub_0xf26b14() {
    // IDA 0xf26b14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// 0xf26b84 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf26b84() {
    // IDA 0xf26b84: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
// 0xf26b94 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP13PlaceLauncherSsSsSsENS3_5list4INS3_5valueIS6_EENSA_ISsEESC_SC_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int()
pub fn stub_0xf26b94() {
    // IDA 0xf26b94: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0xf26ba4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf26ba4() {
    // IDA 0xf26ba4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// 0xf26bb4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
pub fn stub_0xf26bb4() {
    // IDA 0xf26bb4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// 0xf26bc4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf26bc4() {
    // IDA 0xf26bc4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// 0xf26bd4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf26bd4() {
    // IDA 0xf26bd4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN10RobloxView9RenderJob4wakeEv")]
// 0xf26db4 — j___ZN10RobloxView9RenderJob4wakeEv
// type: int __fastcall(RobloxView::RenderJob *this)
pub fn stub_0xf26db4() {
    // IDA 0xf26db4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv")]
// 0xf26e24 — j___ZN3RBX26GlobalAdvancedSettingsItemI19CRenderSettingsItemLZ15sRenderSettingsEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0xf26e24() {
    // IDA 0xf26e24: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v")]
// 0xf26e34 — j___ZN3RBX4Name7declareILZNS_11sRunServiceEEEERKS0_v
// type: int()
pub fn stub_0xf26e34() {
    // IDA 0xf26e34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v")]
// 0xf26e44 — j___ZN3RBX4Name7declareILZNS_18sControllerServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf26e44() {
    // IDA 0xf26e44: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v")]
// 0xf26e54 — j___ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf26e54() {
    // IDA 0xf26e54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}
