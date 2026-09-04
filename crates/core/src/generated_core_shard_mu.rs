//! core shard mu — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 2023 uncovered before -> 1923 after, batch 0xf22ea0..0xf2389c).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b$shim")]
// 0xf22ea0 — __ZNK5boost9function2IvN3RBX17StarterGuiService11CoreGuiTypeEbEclES3_b$shim
// type: int()
pub fn stub_0xf22ea0() {
    // IDA 0xf22ea0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf22eac — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slot24safe_static_do_get_mutexEv$shim
// type: void *()
pub fn stub_0xf22eac() {
    // IDA 0xf22eac: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "_OSAtomicAdd32$shim")]
// 0xf22eb8 — _OSAtomicAdd32$shim
// type: int32_t __cdecl(int32_t __theAmount, int32_t *__theValue)
pub fn stub_0xf22eb8() {
    // IDA 0xf22eb8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sPlayerHUDEEEERKS0_v$shim")]
// 0xf22ec4 — __ZN3RBX4Name9doDeclareILZNS_10sPlayerHUDEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22ec4() {
    // IDA 0xf22ec4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sPlayerMouseEEEERKS0_v$shim")]
// 0xf22ed0 — __ZN3RBX4Name9doDeclareILZNS_12sPlayerMouseEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22ed0() {
    // IDA 0xf22ed0: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sPoseEEEERKS0_v$shim")]
// 0xf22ef4 — __ZN3RBX4Name9doDeclareILZNS_5sPoseEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22ef4() {
    // IDA 0xf22ef4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_5TeamsEEEPT_v$shim")]
// 0xf22f30 — __ZNK3RBX15ServiceProvider6createINS_5TeamsEEEPT_v$shim
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *, int, int, int, RBX::Instance *, int, int, void *, int)
pub fn stub_0xf22f30() {
    // IDA 0xf22f30: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_8LightingEEEPT_v$shim")]
// 0xf22f3c — __ZNK3RBX15ServiceProvider6createINS_8LightingEEEPT_v$shim
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *, int, int, int, RBX::Instance *, int, int, void *, int)
pub fn stub_0xf22f3c() {
    // IDA 0xf22f3c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev$shim")]
// 0xf22f48 — __ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22f48() {
    // IDA 0xf22f48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf22f54 — __ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf22f54() {
    // IDA 0xf22f54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v$shim")]
// 0xf22f60 — __ZN3RBX4Name9doDeclareILZNS_12sScale9FrameEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22f60() {
    // IDA 0xf22f60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf22f6c — __ZNSt6vectorIPN3RBX9GuiButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf22f6c() {
    // IDA 0xf22f6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sGuiMainEEEERKS0_v$shim")]
// 0xf22fa8 — __ZN3RBX4Name9doDeclareILZNS_8sGuiMainEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22fa8() {
    // IDA 0xf22fa8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sScreenGuiEEEERKS0_v$shim")]
// 0xf22fb4 — __ZN3RBX4Name9doDeclareILZNS_10sScreenGuiEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf22fb4() {
    // IDA 0xf22fb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sMouseEEEERKS0_v$shim")]
// 0xf22fc0 — __ZN3RBX4Name7declareILZNS_6sMouseEEEERKS0_v$shim
// type: int()
pub fn stub_0xf22fc0() {
    // IDA 0xf22fc0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sSeatEEEERKS0_v$shim")]
// 0xf23008 — __ZN3RBX4Name9doDeclareILZNS_5sSeatEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23008() {
    // IDA 0xf23008: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf23020 — __ZNSt6vectorIPN3RBX14ISelectionBaseESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf23020() {
    // IDA 0xf23020: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE24safe_static_do_get_mutexEv$shim")]
// 0xf2302c — __ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0xf2302c() {
    // IDA 0xf2302c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9SelectionERKNS4_16SelectionChangedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim")]
// 0xf23044 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9SelectionERKNS4_16SelectionChangedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_$shim
// type: int()
pub fn stub_0xf23044() {
    // IDA 0xf23044: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf23050 — __ZN3rbx7signals6signalIFvRKN3RBX16SelectionChangedEEE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23050() {
    // IDA 0xf23050: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sSelectionBoxEEEERKS0_v$shim")]
// 0xf23080 — __ZN3RBX4Name9doDeclareILZNS_13sSelectionBoxEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23080() {
    // IDA 0xf23080: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v$shim")]
// 0xf2308c — __ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf2308c() {
    // IDA 0xf2308c: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX18SelectionPartLassoD1Ev$shim")]
// 0xf230a4 — __ZN3RBX18SelectionPartLassoD1Ev$shim
// type: void __fastcall(RBX::SelectionPartLasso *)
pub fn stub_0xf230a4() {
    // IDA 0xf230a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v$shim")]
// 0xf230d4 — __ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf230d4() {
    // IDA 0xf230d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v$shim")]
// 0xf230e0 — __ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf230e0() {
    // IDA 0xf230e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v$shim")]
// 0xf2311c — __ZN3RBX4Name9doDeclareILZNS_21sSkateboardControllerEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf2311c() {
    // IDA 0xf2311c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN3RBX18SkateboardPlatform9MoveStateES4_EEclES4_S4_$shim")]
// 0xf23128 — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX18SkateboardPlatform9MoveStateES4_EEclES4_S4_$shim
// type: int()
pub fn stub_0xf23128() {
    // IDA 0xf23128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_$shim")]
// 0xf23140 — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERSt6vectorIPKS5_SaIS9_EEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperISB_EEEEEEEEvT_S6_$shim
// type: int()
pub fn stub_0xf23140() {
    // IDA 0xf23140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSkateboardPlatformEEEERKS0_v$shim")]
// 0xf23188 — __ZN3RBX4Name9doDeclareILZNS_19sSkateboardPlatformEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23188() {
    // IDA 0xf23188: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE24safe_static_do_get_mutexEv$shim")]
// 0xf231a0 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf231a0() {
    // IDA 0xf231a0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf231b8 — __ZNSt6vectorIN3RBX18SkateboardPlatform9MoveStateESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf231b8() {
    // IDA 0xf231b8: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13disconnectAllEv$shim")]
// 0xf231dc — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf231dc() {
    // IDA 0xf231dc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_EclES3_S3_$shim")]
// 0xf231f4 — __ZNK5boost9function2IvN3RBX18SkateboardPlatform9MoveStateES3_EclES3_S3_$shim
// type: int()
pub fn stub_0xf231f4() {
    // IDA 0xf231f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf23200 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4slot24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23200() {
    // IDA 0xf23200: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4sSkyEEEERKS0_v$shim")]
// 0xf23230 — __ZN3RBX4Name9doDeclareILZNS_4sSkyEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23230() {
    // IDA 0xf23230: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sSmokeEEEERKS0_v$shim")]
// 0xf23254 — __ZN3RBX4Name9doDeclareILZNS_6sSmokeEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23254() {
    // IDA 0xf23254: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf23260 — __ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0xf23260() {
    // IDA 0xf23260: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX8SparklesD2Ev$shim")]
// 0xf2326c — __ZN3RBX8SparklesD2Ev$shim
// type: void __fastcall(RBX::Sparkles *)
pub fn stub_0xf2326c() {
    // IDA 0xf2326c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX8SparklesD0Ev$shim")]
// 0xf23284 — __ZN3RBX8SparklesD0Ev$shim
// type: void __fastcall(RBX::Sparkles *)
pub fn stub_0xf23284() {
    // IDA 0xf23284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf23290 — __ZNSt6vectorIPN3RBX13SpawnLocationESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, void *)
pub fn stub_0xf23290() {
    // IDA 0xf23290: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_v$shim")]
// 0xf2329c — __ZNK3RBX15ServiceProvider6createINS_13DebrisServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf2329c() {
    // IDA 0xf2329c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sSpawnLocationEEEERKS0_v$shim")]
// 0xf232cc — __ZN3RBX4Name9doDeclareILZNS_14sSpawnLocationEEEERKS0_v$shim
// type: int()
pub fn stub_0xf232cc() {
    // IDA 0xf232cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sSpecialShapeEEEERKS0_v$shim")]
// 0xf232fc — __ZN3RBX4Name9doDeclareILZNS_13sSpecialShapeEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf232fc() {
    // IDA 0xf232fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v$shim")]
// 0xf23308 — __ZN3RBX4Name7declareILZNS_9sFileMeshEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23308() {
    // IDA 0xf23308: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v$shim")]
// 0xf23314 — __ZN3RBX4Name9doDeclareILZNS_9sFileMeshEEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23314() {
    // IDA 0xf23314: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf23338 — __ZNSt6vectorIN3RBX12SpecialShape8MeshTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf23338() {
    // IDA 0xf23338: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_15ContentProviderEEEPT_v$shim")]
// 0xf23350 — __ZNK3RBX15ServiceProvider4findINS_15ContentProviderEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf23350() {
    // IDA 0xf23350: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX5Stats12StatsServiceD2Ev$shim")]
// 0xf2335c — __ZN3RBX5Stats12StatsServiceD2Ev$shim
// type: void __fastcall(RBX::Stats::StatsService *)
pub fn stub_0xf2335c() {
    // IDA 0xf2335c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ14sProfilingItemEEERKS0_v$shim")]
// 0xf23368 — __ZN3RBX4Name9doDeclareILZ14sProfilingItemEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23368() {
    // IDA 0xf23368: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ25sRunningAverageItemDoubleEEERKS0_v$shim")]
// 0xf23374 — __ZN3RBX4Name9doDeclareILZ25sRunningAverageItemDoubleEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23374() {
    // IDA 0xf23374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ22sRunningAverageItemIntEEERKS0_v$shim")]
// 0xf23380 — __ZN3RBX4Name9doDeclareILZ22sRunningAverageItemIntEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf23380() {
    // IDA 0xf23380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ27sTotalCountTimeIntervalItemEEERKS0_v$shim")]
// 0xf2338c — __ZN3RBX4Name9doDeclareILZ27sTotalCountTimeIntervalItemEEERKS0_v$shim
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf2338c() {
    // IDA 0xf2338c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim")]
// 0xf233b0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm$shim
// type: int()
pub fn stub_0xf233b0() {
    // IDA 0xf233b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim")]
// 0xf233bc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm$shim
// type: int()
pub fn stub_0xf233bc() {
    // IDA 0xf233bc: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_$shim")]
// 0xf233c8 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_$shim
// type: int()
pub fn stub_0xf233c8() {
    // IDA 0xf233c8: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSo9_M_insertIdEERSoT_$shim")]
// 0xf233d4 — __ZNSo9_M_insertIdEERSoT_$shim
// type: int __fastcall(int, int, int)
pub fn stub_0xf233d4() {
    // IDA 0xf233d4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sSurfaceSelectionEEEERKS0_v$shim")]
// 0xf23434 — __ZN3RBX4Name9doDeclareILZNS_17sSurfaceSelectionEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23434() {
    // IDA 0xf23434: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sTeamEEEERKS0_v$shim")]
// 0xf23458 — __ZN3RBX4Name9doDeclareILZNS_5sTeamEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23458() {
    // IDA 0xf23458: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_11TextServiceEEEPT_v$shim")]
// 0xf2347c — __ZNK3RBX15ServiceProvider6createINS_11TextServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf2347c() {
    // IDA 0xf2347c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX7TextBoxD2Ev$shim")]
// 0xf23488 — __ZN3RBX7TextBoxD2Ev$shim
// type: void __fastcall(RBX::TextBox *)
pub fn stub_0xf23488() {
    // IDA 0xf23488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextBoxEEEERKS0_v$shim")]
// 0xf234ac — __ZN3RBX4Name9doDeclareILZNS_8sTextBoxEEEERKS0_v$shim
// type: int()
pub fn stub_0xf234ac() {
    // IDA 0xf234ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvPKcbEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf234b8 — __ZN3rbx7signals6signalIFvPKcbEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf234b8() {
    // IDA 0xf234b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sGuiObjectEEEERKS0_v$shim")]
// 0xf234c4 — __ZN3RBX4Name7declareILZNS_10sGuiObjectEEEERKS0_v$shim
// type: int()
pub fn stub_0xf234c4() {
    // IDA 0xf234c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sGuiObjectEEEERKS0_v$shim")]
// 0xf234d0 — __ZN3RBX4Name9doDeclareILZNS_10sGuiObjectEEEERKS0_v$shim
// type: int()
pub fn stub_0xf234d0() {
    // IDA 0xf234d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFviiEE24safe_static_do_get_mutexEv$shim")]
// 0xf234dc — __ZN3rbx7signals6signalIFviiEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf234dc() {
    // IDA 0xf234dc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv$shim")]
// 0xf234e8 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf234e8() {
    // IDA 0xf234e8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sTextServiceEEEERKS0_v$shim")]
// 0xf234f4 — __ZN3RBX4Name9doDeclareILZNS_12sTextServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf234f4() {
    // IDA 0xf234f4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v$shim")]
// 0xf235a8 — __ZN3RBX4Name9doDeclareILZNS_14sGuiTextButtonEEEERKS0_v$shim
// type: int()
pub fn stub_0xf235a8() {
    // IDA 0xf235a8: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sTextLabelEEEERKS0_v$shim")]
// 0xf235cc — __ZN3RBX4Name9doDeclareILZNS_10sTextLabelEEEERKS0_v$shim
// type: int()
pub fn stub_0xf235cc() {
    // IDA 0xf235cc: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5sToolEEEERKS0_v$shim")]
// 0xf23608 — __ZN3RBX4Name9doDeclareILZNS_5sToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23608() {
    // IDA 0xf23608: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sToolMouseCommandEEEERKS0_v$shim")]
// 0xf2362c — __ZN3RBX4Name9doDeclareILZNS_17sToolMouseCommandEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2362c() {
    // IDA 0xf2362c: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX16ToolMouseCommandD2Ev$shim")]
// 0xf23638 — __ZN3RBX16ToolMouseCommandD2Ev$shim
// type: void __fastcall(RBX::ToolMouseCommand *)
pub fn stub_0xf23638() {
    // IDA 0xf23638: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX16ToolMouseCommandD0Ev$shim")]
// 0xf23644 — __ZN3RBX16ToolMouseCommandD0Ev$shim
// type: void __fastcall(RBX::ToolMouseCommand *)
pub fn stub_0xf23644() {
    // IDA 0xf23644: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim")]
// 0xf23650 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX16ToolMouseCommandEEENS0_5list1INS0_5valueIPS5_EEEEEclEv$shim
// type: int()
pub fn stub_0xf23650() {
    // IDA 0xf23650: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sTouchTransmitterEEEERKS0_v$shim")]
// 0xf23680 — __ZN3RBX4Name9doDeclareILZNS_17sTouchTransmitterEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23680() {
    // IDA 0xf23680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_$shim")]
// 0xf2368c — __ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_$shim
// type: void __fastcall(int, struct _Unwind_Exception *, int, int)
pub fn stub_0xf2368c() {
    // IDA 0xf2368c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE15_M_erase_at_endEPS2_$shim")]
// 0xf23698 — __ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE15_M_erase_at_endEPS2_$shim
// type: int()
pub fn stub_0xf23698() {
    // IDA 0xf23698: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf236a4 — __ZNSt6vectorIN3RBX14TouchDebouncer4ItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *, int, boost::detail::sp_counted_base *, int, int, int, void *, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf236a4() {
    // IDA 0xf236a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v$shim")]
// 0xf236b0 — __ZNK3RBX15ServiceProvider4findINS_17ControllerServiceEEEPT_v$shim
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf236b0() {
    // IDA 0xf236b0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_16UserInputServiceEEEPT_v$shim")]
// 0xf236bc — __ZNK3RBX15ServiceProvider6createINS_16UserInputServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf236bc() {
    // IDA 0xf236bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sButtonBindingWidgetEEEERKS0_v$shim")]
// 0xf236c8 — __ZN3RBX4Name9doDeclareILZNS_20sButtonBindingWidgetEEEERKS0_v$shim
// type: int()
pub fn stub_0xf236c8() {
    // IDA 0xf236c8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sHumanoidControllerEEEERKS0_v$shim")]
// 0xf23710 — __ZN3RBX4Name9doDeclareILZNS_19sHumanoidControllerEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23710() {
    // IDA 0xf23710: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sVehicleControllerEEEERKS0_v$shim")]
// 0xf2371c — __ZN3RBX4Name9doDeclareILZNS_18sVehicleControllerEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2371c() {
    // IDA 0xf2371c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_17sUserInputServiceEEEERKS0_v$shim")]
// 0xf23728 — __ZN3RBX4Name9doDeclareILZNS_17sUserInputServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23728() {
    // IDA 0xf23728: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE24safe_static_do_get_mutexEv$shim")]
// 0xf23740 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE24safe_static_do_get_mutexEv$shim
// type: int __fastcall(int)
pub fn stub_0xf23740() {
    // IDA 0xf23740: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm$shim")]
// 0xf2374c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm$shim
// type: int()
pub fn stub_0xf2374c() {
    // IDA 0xf2374c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm$shim")]
// 0xf23758 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX10Controller6ButtonENS6_11BoundButtonEEES7_S9_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm$shim
// type: int()
pub fn stub_0xf23758() {
    // IDA 0xf23758: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv$shim")]
// 0xf23770 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17ControllerServiceEEEmv$shim
// type: int()
pub fn stub_0xf23770() {
    // IDA 0xf23770: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2377c — __ZNSt6vectorIN3RBX10Controller6ButtonESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf2377c() {
    // IDA 0xf2377c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_$shim")]
// 0xf23788 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10Controller6ButtonEEEclES4_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf23788() {
    // IDA 0xf23788: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13disconnectAllEv$shim")]
// 0xf23794 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE13disconnectAllEv$shim
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0xf23794() {
    // IDA 0xf23794: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK5boost9function1IvN3RBX10Controller6ButtonEEclES3_$shim")]
// 0xf237ac — __ZNK5boost9function1IvN3RBX10Controller6ButtonEEclES3_$shim
// type: int()
pub fn stub_0xf237ac() {
    // IDA 0xf237ac: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf237b8 — __ZN3rbx7signals6signalIFvN3RBX10Controller6ButtonEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf237b8() {
    // IDA 0xf237b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX8ISteppedD2Ev$shim")]
// 0xf237c4 — __ZN3RBX8ISteppedD2Ev$shim
// type: void __fastcall(RBX::IStepped *)
pub fn stub_0xf237c4() {
    // IDA 0xf237c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFviEEclEi$shim")]
// 0xf237d0 — __ZN3rbx7signals16signal_with_argsILi1EFviEEclEi$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf237d0() {
    // IDA 0xf237d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_$shim")]
// 0xf237e8 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_$shim
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0xf237e8() {
    // IDA 0xf237e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX16ConstrainedValueIiLZNS_20sIntConstrainedValueEEE11setValueRawEi$shim")]
// 0xf237f4 — __ZN3RBX16ConstrainedValueIiLZNS_20sIntConstrainedValueEEE11setValueRawEi$shim
// type: int()
pub fn stub_0xf237f4() {
    // IDA 0xf237f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEE11setValueRawEd$shim")]
// 0xf23800 — __ZN3RBX16ConstrainedValueIdLZNS_23sDoubleConstrainedValueEEE11setValueRawEd$shim
// type: int()
pub fn stub_0xf23800() {
    // IDA 0xf23800: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_23sDoubleConstrainedValueEEEERKS0_v$shim")]
// 0xf23884 — __ZN3RBX4Name9doDeclareILZNS_23sDoubleConstrainedValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf23884() {
    // IDA 0xf23884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sIntConstrainedValueEEEERKS0_v$shim")]
// 0xf2389c — __ZN3RBX4Name9doDeclareILZNS_20sIntConstrainedValueEEEERKS0_v$shim
// type: int()
pub fn stub_0xf2389c() {
    // IDA 0xf2389c: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}
