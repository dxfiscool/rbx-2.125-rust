//! core shard mr — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 2323 uncovered before -> 2223 after, batch 0xf20c44..0xf2155c).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv$shim")]
// 0xf20c44 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv$shim
// type: int()
pub fn stub_0xf20c44() -> ! { todo!("0xf20c44 __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvRKSsEE24safe_static_do_get_mutexEv$shim")]
// 0xf20c50 — __ZN3rbx7signals6signalIFvRKSsEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf20c50() -> ! { todo!("0xf20c50 __ZN3rbx7signals6signalIFvRKSsEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZNSs6appendEmc$shim")]
// 0xf20c68 — __ZNSs6appendEmc$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf20c68() -> ! { todo!("0xf20c68 __ZNSs6appendEmc$shim") }

#[doc(alias = "__ZNSs6appendEPKcm$shim")]
// 0xf20c74 — __ZNSs6appendEPKcm$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf20c74() -> ! { todo!("0xf20c74 __ZNSs6appendEPKcm$shim") }

#[doc(alias = "__ZNSt6localeaSERKS_$shim")]
// 0xf20cbc — __ZNSt6localeaSERKS_$shim
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf20cbc() -> ! { todo!("0xf20cbc __ZNSt6localeaSERKS_$shim") }

#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv$shim")]
// 0xf20cd4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf20cd4() -> ! { todo!("0xf20cd4 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv$shim")]
// 0xf20ce0 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv$shim
// type: int(void)
pub fn stub_0xf20ce0() -> ! { todo!("0xf20ce0 __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v$shim")]
// 0xf20cec — __ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20cec() -> ! { todo!("0xf20cec __ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v$shim")]
// 0xf20cf8 — __ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20cf8() -> ! { todo!("0xf20cf8 __ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_11TestServiceEEEmv$shim")]
// 0xf20d04 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_11TestServiceEEEmv$shim
// type: int()
pub fn stub_0xf20d04() -> ! { todo!("0xf20d04 __ZN3RBX15ServiceProvider15doGetClassIndexINS_11TestServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v$shim")]
// 0xf20d10 — __ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v$shim
pub fn stub_0xf20d10() -> ! { todo!("0xf20d10 __ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v$shim")]
// 0xf20d1c — __ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20d1c() -> ! { todo!("0xf20d1c __ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14SpawnerServiceEEEmv$shim")]
// 0xf20d28 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14SpawnerServiceEEEmv$shim
// type: int()
pub fn stub_0xf20d28() -> ! { todo!("0xf20d28 __ZN3RBX15ServiceProvider15doGetClassIndexINS_14SpawnerServiceEEEmv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE24safe_static_do_get_mutexEv$shim")]
// 0xf20d34 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf20d34() -> ! { todo!("0xf20d34 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_17sUserInputServiceEEEERKS0_v$shim")]
// 0xf20d40 — __ZN3RBX4Name7declareILZNS_17sUserInputServiceEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20d40() -> ! { todo!("0xf20d40 __ZN3RBX4Name7declareILZNS_17sUserInputServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_16UserInputServiceEEEmv$shim")]
// 0xf20d4c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_16UserInputServiceEEEmv$shim
// type: int()
pub fn stub_0xf20d4c() -> ! { todo!("0xf20d4c __ZN3RBX15ServiceProvider15doGetClassIndexINS_16UserInputServiceEEEmv$shim") }

#[doc(alias = "__ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_$shim")]
// 0xf20d58 — __ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf20d58() -> ! { todo!("0xf20d58 __ZSt9sort_heapIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_$shim") }

#[doc(alias = "__ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_$shim")]
// 0xf20d64 — __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf20d64() -> ! { todo!("0xf20d64 __ZSt16__insertion_sortIN9__gnu_cxx17__normal_iteratorIPPN3RBX10IAdornableESt6vectorIS4_SaIS4_EEEEPFbPKS3_SB_EEvT_SE_T0_$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v$shim")]
// 0xf20d94 — __ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20d94() -> ! { todo!("0xf20d94 __ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v$shim")]
// 0xf20da0 — __ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v$shim
pub fn stub_0xf20da0() -> ! { todo!("0xf20da0 __ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v$shim")]
// 0xf20dac — __ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20dac() -> ! { todo!("0xf20dac __ZN3RBX4Name7declareILZNS_14sServerStorageEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v$shim")]
// 0xf20db8 — __ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20db8() -> ! { todo!("0xf20db8 __ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13ServerStorageEEEmv$shim")]
// 0xf20dc4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ServerStorageEEEmv$shim
// type: int()
pub fn stub_0xf20dc4() -> ! { todo!("0xf20dc4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ServerStorageEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v$shim")]
// 0xf20df4 — __ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v$shim
pub fn stub_0xf20df4() -> ! { todo!("0xf20df4 __ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v$shim") }

#[doc(alias = "__ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE12manage_smallERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE$shim")]
// 0xf20e00 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE12manage_smallERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE$shim
// type: int(void)
pub fn stub_0xf20e00() -> ! { todo!("0xf20e00 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE12manage_smallERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE$shim") }

#[doc(alias = "__ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim")]
// 0xf20e0c — __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim
// type: int(void)
pub fn stub_0xf20e0c() -> ! { todo!("0xf20e0c __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_$shim") }

#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// 0xf20e18 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: int(void)
pub fn stub_0xf20e18() -> ! { todo!("0xf20e18 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v$shim")]
// 0xf20e24 — __ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20e24() -> ! { todo!("0xf20e24 __ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12AssetServiceEEEmv$shim")]
// 0xf20e30 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12AssetServiceEEEmv$shim
// type: int()
pub fn stub_0xf20e30() -> ! { todo!("0xf20e30 __ZN3RBX15ServiceProvider15doGetClassIndexINS_12AssetServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v$shim")]
// 0xf20e60 — __ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20e60() -> ! { todo!("0xf20e60 __ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_20ContextActionServiceEEEmv$shim")]
// 0xf20e6c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ContextActionServiceEEEmv$shim
// type: int()
pub fn stub_0xf20e6c() -> ! { todo!("0xf20e6c __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ContextActionServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v$shim")]
// 0xf20e78 — __ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20e78() -> ! { todo!("0xf20e78 __ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_21PersonalServerServiceEEEmv$shim")]
// 0xf20e84 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_21PersonalServerServiceEEEmv$shim
// type: int()
pub fn stub_0xf20e84() -> ! { todo!("0xf20e84 __ZN3RBX15ServiceProvider15doGetClassIndexINS_21PersonalServerServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v$shim")]
// 0xf20e90 — __ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v$shim
pub fn stub_0xf20e90() -> ! { todo!("0xf20e90 __ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15TeleportServiceEEEmv$shim")]
// 0xf20e9c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15TeleportServiceEEEmv$shim
// type: int()
pub fn stub_0xf20e9c() -> ! { todo!("0xf20e9c __ZN3RBX15ServiceProvider15doGetClassIndexINS_15TeleportServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v$shim")]
// 0xf20ea8 — __ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20ea8() -> ! { todo!("0xf20ea8 __ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_14CookiesServiceEEEmv$shim")]
// 0xf20eb4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CookiesServiceEEEmv$shim
// type: int()
pub fn stub_0xf20eb4() -> ! { todo!("0xf20eb4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CookiesServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v$shim")]
// 0xf20ec0 — __ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20ec0() -> ! { todo!("0xf20ec0 __ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv$shim")]
// 0xf20ecc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv$shim
// type: int()
pub fn stub_0xf20ecc() -> ! { todo!("0xf20ecc __ZN3RBX15ServiceProvider15doGetClassIndexINS_13DebrisServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v$shim")]
// 0xf20ed8 — __ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20ed8() -> ! { todo!("0xf20ed8 __ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv$shim")]
// 0xf20ee4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv$shim
// type: int()
pub fn stub_0xf20ee4() -> ! { todo!("0xf20ee4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v$shim")]
// 0xf20ef0 — __ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20ef0() -> ! { todo!("0xf20ef0 __ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv$shim")]
// 0xf20efc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv$shim
// type: int()
pub fn stub_0xf20efc() -> ! { todo!("0xf20efc __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv$shim")]
// 0xf20f08 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv$shim
// type: int()
pub fn stub_0xf20f08() -> ! { todo!("0xf20f08 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v$shim")]
// 0xf20f14 — __ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20f14() -> ! { todo!("0xf20f14 __ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv$shim")]
// 0xf20f20 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv$shim
// type: int()
pub fn stub_0xf20f20() -> ! { todo!("0xf20f20 __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v$shim")]
// 0xf20f2c — __ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20f2c() -> ! { todo!("0xf20f2c __ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v$shim")]
// 0xf20f38 — __ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v$shim
pub fn stub_0xf20f38() -> ! { todo!("0xf20f38 __ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv$shim")]
// 0xf20f44 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv$shim
// type: int()
pub fn stub_0xf20f44() -> ! { todo!("0xf20f44 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v$shim")]
// 0xf20f50 — __ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20f50() -> ! { todo!("0xf20f50 __ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv$shim")]
// 0xf20f5c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv$shim
// type: int()
pub fn stub_0xf20f5c() -> ! { todo!("0xf20f5c __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v$shim")]
// 0xf20f68 — __ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20f68() -> ! { todo!("0xf20f68 __ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v$shim")]
// 0xf20f74 — __ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf20f74() -> ! { todo!("0xf20f74 __ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv$shim")]
// 0xf20f80 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv$shim
// type: int()
pub fn stub_0xf20f80() -> ! { todo!("0xf20f80 __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v$shim")]
// 0xf20f8c — __ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v$shim
pub fn stub_0xf20f8c() -> ! { todo!("0xf20f8c __ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v$shim")]
// 0xf20f98 — __ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf20f98() -> ! { todo!("0xf20f98 __ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv$shim")]
// 0xf20fa4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv$shim
// type: int()
pub fn stub_0xf20fa4() -> ! { todo!("0xf20fa4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_17CollectionServiceEEEmv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf20fb0 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf20fb0() -> ! { todo!("0xf20fb0 __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv$shim")]
// 0xf20fbc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv$shim
pub fn stub_0xf20fbc() -> ! { todo!("0xf20fbc __ZN3RBX15ServiceProvider15doGetClassIndexINS_10RunServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sStarterGuiServiceEEEERKS0_v$shim")]
// 0xf20fc8 — __ZN3RBX4Name9doDeclareILZNS_18sStarterGuiServiceEEEERKS0_v$shim
pub fn stub_0xf20fc8() -> ! { todo!("0xf20fc8 __ZN3RBX4Name9doDeclareILZNS_18sStarterGuiServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv$shim")]
// 0xf20fd4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv$shim
pub fn stub_0xf20fd4() -> ! { todo!("0xf20fd4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_17StarterGuiServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sStarterPackServiceEEEERKS0_v$shim")]
// 0xf20fe0 — __ZN3RBX4Name9doDeclareILZNS_19sStarterPackServiceEEEERKS0_v$shim
pub fn stub_0xf20fe0() -> ! { todo!("0xf20fe0 __ZN3RBX4Name9doDeclareILZNS_19sStarterPackServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v$shim")]
// 0xf20fec — __ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v$shim
pub fn stub_0xf20fec() -> ! { todo!("0xf20fec __ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13LocalBackpackEEEmv$shim")]
// 0xf20ff8 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13LocalBackpackEEEmv$shim
pub fn stub_0xf20ff8() -> ! { todo!("0xf20ff8 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13LocalBackpackEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sMarketplaceServiceEEEERKS0_v$shim")]
// 0xf21004 — __ZN3RBX4Name9doDeclareILZNS_19sMarketplaceServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf21004() -> ! { todo!("0xf21004 __ZN3RBX4Name9doDeclareILZNS_19sMarketplaceServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v$shim")]
// 0xf21010 — __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v$shim
pub fn stub_0xf21010() -> ! { todo!("0xf21010 __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_25sKeyframeSequenceProviderEEEERKS0_v$shim")]
// 0xf2101c — __ZN3RBX4Name9doDeclareILZNS_25sKeyframeSequenceProviderEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf2101c() -> ! { todo!("0xf2101c __ZN3RBX4Name9doDeclareILZNS_25sKeyframeSequenceProviderEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_24KeyframeSequenceProviderEEEmv$shim")]
// 0xf21028 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_24KeyframeSequenceProviderEEEmv$shim
pub fn stub_0xf21028() -> ! { todo!("0xf21028 __ZN3RBX15ServiceProvider15doGetClassIndexINS_24KeyframeSequenceProviderEEEmv$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_13ContentFilterEEEmv$shim")]
// 0xf21034 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ContentFilterEEEmv$shim
pub fn stub_0xf21034() -> ! { todo!("0xf21034 __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ContentFilterEEEmv$shim") }

#[doc(alias = "__ZN3RBX15ServiceProviderD2Ev$shim")]
// 0xf21040 — __ZN3RBX15ServiceProviderD2Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf21040() -> ! { todo!("0xf21040 __ZN3RBX15ServiceProviderD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sServiceProviderEEEERKS0_v$shim")]
// 0xf2104c — __ZN3RBX4Name9doDeclareILZNS_16sServiceProviderEEEERKS0_v$shim
pub fn stub_0xf2104c() -> ! { todo!("0xf2104c __ZN3RBX4Name9doDeclareILZNS_16sServiceProviderEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_20ChangeHistoryServiceEEEmv$shim")]
// 0xf21058 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ChangeHistoryServiceEEEmv$shim
pub fn stub_0xf21058() -> ! { todo!("0xf21058 __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ChangeHistoryServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v$shim")]
// 0xf21064 — __ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v$shim
// type: int(void)
pub fn stub_0xf21064() -> ! { todo!("0xf21064 __ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v$shim")]
// 0xf21070 — __ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v$shim
pub fn stub_0xf21070() -> ! { todo!("0xf21070 __ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_5VisitEEEmv$shim")]
// 0xf2107c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5VisitEEEmv$shim
// type: int()
pub fn stub_0xf2107c() -> ! { todo!("0xf2107c __ZN3RBX15ServiceProvider15doGetClassIndexINS_5VisitEEEmv$shim") }

#[doc(alias = "__ZNK5boost9function1IvbEclEb$shim")]
// 0xf2110c — __ZNK5boost9function1IvbEclEb$shim
pub fn stub_0xf2110c() -> ! { todo!("0xf2110c __ZNK5boost9function1IvbEclEb$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf21118 — __ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf21118() -> ! { todo!("0xf21118 __ZN3rbx7signals6signalIFvbEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm$shim")]
// 0xf21220 — __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm$shim
pub fn stub_0xf21220() -> ! { todo!("0xf21220 __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm$shim") }

#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm$shim")]
// 0xf2122c — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm$shim
pub fn stub_0xf2122c() -> ! { todo!("0xf2122c __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm$shim") }

#[doc(alias = "__ZN5boost6detail15sp_counted_base12weak_releaseEv$shim")]
// 0xf21298 — __ZN5boost6detail15sp_counted_base12weak_releaseEv$shim
// type: int(void)
pub fn stub_0xf21298() -> ! { todo!("0xf21298 __ZN5boost6detail15sp_counted_base12weak_releaseEv$shim") }

#[doc(alias = "__ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv$shim")]
// 0xf212bc — __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv$shim
pub fn stub_0xf212bc() -> ! { todo!("0xf212bc __ZNK3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE2EE4rateEv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim")]
// 0xf21340 — __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim
pub fn stub_0xf21340() -> ! { todo!("0xf21340 __ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim")]
// 0xf21358 — __ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim
pub fn stub_0xf21358() -> ! { todo!("0xf21358 __ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev$shim")]
// 0xf21364 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev$shim
pub fn stub_0xf21364() -> ! { todo!("0xf21364 __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev$shim") }

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev$shim")]
// 0xf2137c — __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev$shim
pub fn stub_0xf2137c() -> ! { todo!("0xf2137c __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf21424 — __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf21424() -> ! { todo!("0xf21424 __ZNSt6vectorIN3RBX4Time12SampleMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf21430 — __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf21430() -> ! { todo!("0xf21430 __ZNSt6vectorIN3RBX9EThrottle13EThrottleTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2143c — __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf2143c() -> ! { todo!("0xf2143c __ZNSt6vectorIN3RBX13DebugSettings14ErrorReportingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim")]
// 0xf21448 — __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim
pub fn stub_0xf21448() -> ! { todo!("0xf21448 __ZNSt6vectorIN3RBX13TaskScheduler3Job17SleepAdjustMethodESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf21454 — __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf21454() -> ! { todo!("0xf21454 __ZNSt6vectorIN3RBX13TaskScheduler14PriorityMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf21460 — __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf21460() -> ! { todo!("0xf21460 __ZNSt6vectorIN3RBX13TaskScheduler16ThreadPoolConfigESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v$shim")]
// 0xf214a8 — __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v$shim
pub fn stub_0xf214a8() -> ! { todo!("0xf214a8 __ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v$shim")]
// 0xf214b4 — __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v$shim
pub fn stub_0xf214b4() -> ! { todo!("0xf214b4 __ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v$shim")]
// 0xf214d8 — __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v$shim
pub fn stub_0xf214d8() -> ! { todo!("0xf214d8 __ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX10DialogRootD2Ev$shim")]
// 0xf214e4 — __ZN3RBX10DialogRootD2Ev$shim
pub fn stub_0xf214e4() -> ! { todo!("0xf214e4 __ZN3RBX10DialogRootD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v$shim")]
// 0xf21508 — __ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v$shim
pub fn stub_0xf21508() -> ! { todo!("0xf21508 __ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf21514 — __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf21514() -> ! { todo!("0xf21514 __ZNSt6vectorIN3RBX10DialogRoot10DialogToneESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf21520 — __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf21520() -> ! { todo!("0xf21520 __ZNSt6vectorIN3RBX10DialogRoot13DialogPurposeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf21550 — __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf21550() -> ! { todo!("0xf21550 __ZNSt6vectorIN3RBX9GuiObject14SizeConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf2155c — __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf2155c() -> ! { todo!("0xf2155c __ZNSt6vectorIN3RBX7Handles11VisualStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }
