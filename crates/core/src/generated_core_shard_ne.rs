//! core shard ne — 100 core stubs EA-sorted asc global gap filler not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33260, 1219->1119 uncovered, 41780->41880 distinct, batch 0xf36644..0xf417a4).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")]
// 0xf36644 — j___ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf36644() -> ! { todo!("0xf36644 j___ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v")]
// 0xf36654 — j___ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v
pub fn stub_0xf36654() -> ! { todo!("0xf36654 j___ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v")]
// 0xf36664 — j___ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v
pub fn stub_0xf36664() -> ! { todo!("0xf36664 j___ZN3RBX4Name9doDeclareILZNS_13sAssetServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v")]
// 0xf36674 — j___ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v
pub fn stub_0xf36674() -> ! { todo!("0xf36674 j___ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")]
// 0xf36684 — j___ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf36684() -> ! { todo!("0xf36684 j___ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v")]
// 0xf36694 — j___ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v
pub fn stub_0xf36694() -> ! { todo!("0xf36694 j___ZN3RBX4Name9doDeclareILZNS_14sDebrisServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v")]
// 0xf366a4 — j___ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v
pub fn stub_0xf366a4() -> ! { todo!("0xf366a4 j___ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v")]
// 0xf366b4 — j___ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v
pub fn stub_0xf366b4() -> ! { todo!("0xf366b4 j___ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v")]
// 0xf366c4 — j___ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf366c4() -> ! { todo!("0xf366c4 j___ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v")]
// 0xf366d4 — j___ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v
pub fn stub_0xf366d4() -> ! { todo!("0xf366d4 j___ZN3RBX4Name9doDeclareILZNS_14sServerStorageEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v")]
// 0xf366e4 — j___ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v
pub fn stub_0xf366e4() -> ! { todo!("0xf366e4 j___ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v")]
// 0xf366f4 — j___ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v
pub fn stub_0xf366f4() -> ! { todo!("0xf366f4 j___ZN3RBX4Name9doDeclareILZNS_15sCookiesServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v")]
// 0xf36704 — j___ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v
pub fn stub_0xf36704() -> ! { todo!("0xf36704 j___ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v")]
// 0xf36714 — j___ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v
pub fn stub_0xf36714() -> ! { todo!("0xf36714 j___ZN3RBX4Name9doDeclareILZNS_15sSpawnerServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v")]
// 0xf36724 — j___ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v
pub fn stub_0xf36724() -> ! { todo!("0xf36724 j___ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v")]
// 0xf36734 — j___ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v
pub fn stub_0xf36734() -> ! { todo!("0xf36734 j___ZN3RBX4Name9doDeclareILZNS_16sTeleportServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_18sCollectionServiceEEEERKS0_v")]
// 0xf36744 — j___ZN3RBX4Name9doDeclareILZNS_18sCollectionServiceEEEERKS0_v
pub fn stub_0xf36744() -> ! { todo!("0xf36744 j___ZN3RBX4Name9doDeclareILZNS_18sCollectionServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v")]
// 0xf36754 — j___ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v
pub fn stub_0xf36754() -> ! { todo!("0xf36754 j___ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_18sStarterGuiServiceEEEERKS0_v")]
// 0xf36764 — j___ZN3RBX4Name9doDeclareILZNS_18sStarterGuiServiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf36764() -> ! { todo!("0xf36764 j___ZN3RBX4Name9doDeclareILZNS_18sStarterGuiServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v")]
// 0xf36774 — j___ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v
pub fn stub_0xf36774() -> ! { todo!("0xf36774 j___ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_19sStarterPackServiceEEEERKS0_v")]
// 0xf36784 — j___ZN3RBX4Name9doDeclareILZNS_19sStarterPackServiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf36784() -> ! { todo!("0xf36784 j___ZN3RBX4Name9doDeclareILZNS_19sStarterPackServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v")]
// 0xf367a4 — j___ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf367a4() -> ! { todo!("0xf367a4 j___ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v")]
// 0xf367b4 — j___ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v
pub fn stub_0xf367b4() -> ! { todo!("0xf367b4 j___ZN3RBX4Name9doDeclareILZNS_21sContextActionServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v")]
// 0xf367c4 — j___ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v
pub fn stub_0xf367c4() -> ! { todo!("0xf367c4 j___ZN3RBX4Name9doDeclareILZNS_22sPersonalServerServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v")]
// 0xf367e4 — j___ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf367e4() -> ! { todo!("0xf367e4 j___ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v")]
// 0xf367f4 — j___ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v
pub fn stub_0xf367f4() -> ! { todo!("0xf367f4 j___ZN3RBX4Name9doDeclareILZNS_9sLightingEEEERKS0_v") }

#[doc(alias = "j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0xf37b74 — j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf37b74() -> ! { todo!("0xf37b74 j___ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE") }

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// 0xf37c14 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_0xf37c14() -> ! { todo!("0xf37c14 j___ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE") }

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// 0xf37c24 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_0xf37c24() -> ! { todo!("0xf37c24 j___ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE") }

#[doc(alias = "j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0xf37d24 — j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xf37d24() -> ! { todo!("0xf37d24 j___ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev")]
// 0xf39714 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev
// type: int()
pub fn stub_0xf39714() -> ! { todo!("0xf39714 j___ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEEC2Ev") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// 0xf39724 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf39724() -> ! { todo!("0xf39724 j___ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev")]
// 0xf39734 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev
// type: int()
pub fn stub_0xf39734() -> ! { todo!("0xf39734 j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev")]
// 0xf39744 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf39744() -> ! { todo!("0xf39744 j___ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v")]
// 0xf39754 — j___ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf39754() -> ! { todo!("0xf39754 j___ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
// 0xf39764 — j___ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf39764() -> ! { todo!("0xf39764 j___ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v") }

#[doc(alias = "DummyJob::DummyJob(bool,double)")]
// 0xf399a4 — j___ZN8DummyJobC2Ebd — DummyJob::DummyJob(bool,double)
// type: void __fastcall(DummyJob *this, bool, double)
pub fn stub_0xf399a4() -> ! { todo!("0xf399a4 j___ZN8DummyJobC2Ebd") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v")]
// 0xf3a0d4 — j___ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf3a0d4() -> ! { todo!("0xf3a0d4 j___ZN3RBX4Name9doDeclareILZNS_6sDecalEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v")]
// 0xf3a0e4 — j___ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf3a0e4() -> ! { todo!("0xf3a0e4 j___ZN3RBX4Name9doDeclareILZNS_8sTextureEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v")]
// 0xf3a204 — j___ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf3a204() -> ! { todo!("0xf3a204 j___ZN3RBX4Name9doDeclareILZNS_13sDialogChoiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v")]
// 0xf3a384 — j___ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf3a384() -> ! { todo!("0xf3a384 j___ZN3RBX4Name9doDeclareILZNS_11sDialogRootEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v")]
// 0xf3aa94 — j___ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf3aa94() -> ! { todo!("0xf3aa94 j___ZN3RBX4Name9doDeclareILZNS_10sExplosionEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v")]
// 0xf3aeb4 — j___ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf3aeb4() -> ! { todo!("0xf3aeb4 j___ZN3RBX4Name9doDeclareILZNS_13sExtrudedPartEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v")]
// 0xf3c284 — j___ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v
pub fn stub_0xf3c284() -> ! { todo!("0xf3c284 j___ZN3RBX4Name7declareILZNS_10sAnimationEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v")]
// 0xf3c294 — j___ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v
pub fn stub_0xf3c294() -> ! { todo!("0xf3c294 j___ZN3RBX4Name7declareILZNS_10sBasicPartEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v")]
// 0xf3c2a4 — j___ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v
pub fn stub_0xf3c2a4() -> ! { todo!("0xf3c2a4 j___ZN3RBX4Name7declareILZNS_12sCustomEventEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v")]
// 0xf3c2b4 — j___ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v
pub fn stub_0xf3c2b4() -> ! { todo!("0xf3c2b4 j___ZN3RBX4Name7declareILZNS_14sBindableEventEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v")]
// 0xf3c2c4 — j___ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v
pub fn stub_0xf3c2c4() -> ! { todo!("0xf3c2c4 j___ZN3RBX4Name7declareILZNS_17sBindableFunctionEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v")]
// 0xf3c2d4 — j___ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v
pub fn stub_0xf3c2d4() -> ! { todo!("0xf3c2d4 j___ZN3RBX4Name7declareILZNS_20sCustomEventReceiverEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v")]
// 0xf3c2e4 — j___ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v
pub fn stub_0xf3c2e4() -> ! { todo!("0xf3c2e4 j___ZN3RBX4Name7declareILZNS_9sSparklesEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v")]
// 0xf3c2f4 — j___ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v
pub fn stub_0xf3c2f4() -> ! { todo!("0xf3c2f4 j___ZN3RBX4Name9doDeclareILZNS_10sAnimationEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v")]
// 0xf3c304 — j___ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v
// type: int(void)
pub fn stub_0xf3c304() -> ! { todo!("0xf3c304 j___ZN3RBX4Name9doDeclareILZNS_10sBasicPartEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v")]
// 0xf3c314 — j___ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v
pub fn stub_0xf3c314() -> ! { todo!("0xf3c314 j___ZN3RBX4Name9doDeclareILZNS_11sForceFieldEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v")]
// 0xf3c324 — j___ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v
pub fn stub_0xf3c324() -> ! { todo!("0xf3c324 j___ZN3RBX4Name9doDeclareILZNS_12sCustomEventEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v")]
// 0xf3c334 — j___ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v
pub fn stub_0xf3c334() -> ! { todo!("0xf3c334 j___ZN3RBX4Name9doDeclareILZNS_14sBindableEventEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v")]
// 0xf3c344 — j___ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v
pub fn stub_0xf3c344() -> ! { todo!("0xf3c344 j___ZN3RBX4Name9doDeclareILZNS_17sBindableFunctionEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v")]
// 0xf3c354 — j___ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v
pub fn stub_0xf3c354() -> ! { todo!("0xf3c354 j___ZN3RBX4Name9doDeclareILZNS_20sCustomEventReceiverEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v")]
// 0xf3c364 — j___ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v
pub fn stub_0xf3c364() -> ! { todo!("0xf3c364 j___ZN3RBX4Name9doDeclareILZNS_9sSparklesEEEERKS0_v") }

#[doc(alias = "j___ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_")]
// 0xf3cfe4 — j___ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_
pub fn stub_0xf3cfe4() -> ! { todo!("0xf3cfe4 j___ZN5boost8functionIFvRSt9exceptionEEaSIPS3_EENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeES8_") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v")]
// 0xf3e054 — j___ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e054() -> ! { todo!("0xf3e054 j___ZN3RBX4Name9doDeclareILZNS_13sMotorFeatureEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v")]
// 0xf3e064 — j___ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e064() -> ! { todo!("0xf3e064 j___ZN3RBX4Name9doDeclareILZNS_14sVelocityMotorEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v")]
// 0xf3e074 — j___ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e074() -> ! { todo!("0xf3e074 j___ZN3RBX4Name9doDeclareILZNS_5sHoleEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v")]
// 0xf3e084 — j___ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e084() -> ! { todo!("0xf3e084 j___ZN3RBX4Name9doDeclareILZNS_8sFeatureEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v")]
// 0xf3e4b4 — j___ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e4b4() -> ! { todo!("0xf3e4b4 j___ZN3RBX4Name9doDeclareILZNS_5sFireEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v")]
// 0xf3e564 — j___ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e564() -> ! { todo!("0xf3e564 j___ZN3RBX4Name9doDeclareILZNS_5sFlagEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v")]
// 0xf3e6a4 — j___ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e6a4() -> ! { todo!("0xf3e6a4 j___ZN3RBX4Name9doDeclareILZNS_10sFlagStandEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v")]
// 0xf3e6b4 — j___ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e6b4() -> ! { todo!("0xf3e6b4 j___ZN3RBX4Name9doDeclareILZNS_17sFlagStandServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv")]
// 0xf3e9e4 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0xf3e9e4() -> ! { todo!("0xf3e9e4 j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEE9singletonEv") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v")]
// 0xf3e9f4 — j___ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0xf3e9f4() -> ! { todo!("0xf3e9f4 j___ZN3RBX4Name7declareILZNS_13sGameSettingsEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v")]
// 0xf3ea04 — j___ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0xf3ea04() -> ! { todo!("0xf3ea04 j___ZN3RBX4Name9doDeclareILZNS_13sGameSettingsEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev")]
// 0xf3ece4 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev
pub fn stub_0xf3ece4() -> ! { todo!("0xf3ece4 j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEEC2Ev") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev")]
// 0xf3ecf4 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev
// type: int(void)
pub fn stub_0xf3ecf4() -> ! { todo!("0xf3ecf4 j___ZN3RBX26GlobalAdvancedSettingsItemINS_12GameSettingsELZNS_13sGameSettingsEEED0Ev") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v")]
// 0xf3eee4 — j___ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v
pub fn stub_0xf3eee4() -> ! { todo!("0xf3eee4 j___ZN3RBX4Name9doDeclareILZNS_16sGeometryServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v")]
// 0xf3f0f4 — j___ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f0f4() -> ! { todo!("0xf3f0f4 j___ZN3RBX4Name7declareILZNS_10sSelectionEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_16sServiceProviderEEEERKS0_v")]
// 0xf3f104 — j___ZN3RBX4Name7declareILZNS_16sServiceProviderEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f104() -> ! { todo!("0xf3f104 j___ZN3RBX4Name7declareILZNS_16sServiceProviderEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v")]
// 0xf3f114 — j___ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f114() -> ! { todo!("0xf3f114 j___ZN3RBX4Name9doDeclareILZNS_10sSelectionEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_16sServiceProviderEEEERKS0_v")]
// 0xf3f124 — j___ZN3RBX4Name9doDeclareILZNS_16sServiceProviderEEEERKS0_v
pub fn stub_0xf3f124() -> ! { todo!("0xf3f124 j___ZN3RBX4Name9doDeclareILZNS_16sServiceProviderEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v")]
// 0xf3f134 — j___ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f134() -> ! { todo!("0xf3f134 j___ZN3RBX4Name9doDeclareILZNS_20sGlobalBasicSettingsEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v")]
// 0xf3f144 — j___ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f144() -> ! { todo!("0xf3f144 j___ZN3RBX4Name9doDeclareILZNS_23sGlobalAdvancedSettingsEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v")]
// 0xf3f154 — j___ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f154() -> ! { todo!("0xf3f154 j___ZN3RBX4Name9doDeclareILZNS_9sSettingsEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_8sGuiBaseEEEERKS0_v")]
// 0xf3f334 — j___ZN3RBX4Name9doDeclareILZNS_8sGuiBaseEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f334() -> ! { todo!("0xf3f334 j___ZN3RBX4Name9doDeclareILZNS_8sGuiBaseEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sGuiBase3dEEEERKS0_v")]
// 0xf3f374 — j___ZN3RBX4Name9doDeclareILZNS_10sGuiBase3dEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f374() -> ! { todo!("0xf3f374 j___ZN3RBX4Name9doDeclareILZNS_10sGuiBase3dEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv")]
// 0xf3f404 — j___ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv
// type: int(void)
pub fn stub_0xf3f404() -> ! { todo!("0xf3f404 j___ZN3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE15isNullClassNameEv") }

#[doc(alias = "j___ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEE9singletonEv")]
// 0xf3f434 — j___ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
pub fn stub_0xf3f434() -> ! { todo!("0xf3f434 j___ZN3RBX26GlobalAdvancedSettingsItemINS_15PhysicsSettingsELZNS_16sPhysicsSettingsEEE9singletonEv") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v")]
// 0xf3f444 — j___ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f444() -> ! { todo!("0xf3f444 j___ZN3RBX4Name7declareILZNS_15sCoreGuiServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_16sPhysicsSettingsEEEERKS0_v")]
// 0xf3f454 — j___ZN3RBX4Name7declareILZNS_16sPhysicsSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f454() -> ! { todo!("0xf3f454 j___ZN3RBX4Name7declareILZNS_16sPhysicsSettingsEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v")]
// 0xf3f464 — j___ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v
pub fn stub_0xf3f464() -> ! { todo!("0xf3f464 j___ZN3RBX4Name9doDeclareILZNS_15sCoreGuiServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_16sPhysicsSettingsEEEERKS0_v")]
// 0xf3f474 — j___ZN3RBX4Name9doDeclareILZNS_16sPhysicsSettingsEEEERKS0_v
// type: int(void)
pub fn stub_0xf3f474() -> ! { todo!("0xf3f474 j___ZN3RBX4Name9doDeclareILZNS_16sPhysicsSettingsEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sTweenServiceEEEERKS0_v")]
// 0xf3fc64 — j___ZN3RBX4Name7declareILZNS_13sTweenServiceEEEERKS0_v
pub fn stub_0xf3fc64() -> ! { todo!("0xf3fc64 j___ZN3RBX4Name7declareILZNS_13sTweenServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sGuiButtonEEEERKS0_v")]
// 0xf3fc74 — j___ZN3RBX4Name9doDeclareILZNS_10sGuiButtonEEEERKS0_v
// type: int(void)
pub fn stub_0xf3fc74() -> ! { todo!("0xf3fc74 j___ZN3RBX4Name9doDeclareILZNS_10sGuiButtonEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sTweenServiceEEEERKS0_v")]
// 0xf3fc84 — j___ZN3RBX4Name9doDeclareILZNS_13sTweenServiceEEEERKS0_v
// type: int(void)
pub fn stub_0xf3fc84() -> ! { todo!("0xf3fc84 j___ZN3RBX4Name9doDeclareILZNS_13sTweenServiceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sGuiLabelEEEERKS0_v")]
// 0xf3fc94 — j___ZN3RBX4Name9doDeclareILZNS_9sGuiLabelEEEERKS0_v
// type: int(void)
pub fn stub_0xf3fc94() -> ! { todo!("0xf3fc94 j___ZN3RBX4Name9doDeclareILZNS_9sGuiLabelEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sBodyForceEEEERKS0_v")]
// 0xf41734 — j___ZN3RBX4Name9doDeclareILZNS_10sBodyForceEEEERKS0_v
// type: int(void)
pub fn stub_0xf41734() -> ! { todo!("0xf41734 j___ZN3RBX4Name9doDeclareILZNS_10sBodyForceEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_10sBodyMoverEEEERKS0_v")]
// 0xf41744 — j___ZN3RBX4Name9doDeclareILZNS_10sBodyMoverEEEERKS0_v
// type: int(void)
pub fn stub_0xf41744() -> ! { todo!("0xf41744 j___ZN3RBX4Name9doDeclareILZNS_10sBodyMoverEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_11sBodyThrustEEEERKS0_v")]
// 0xf41754 — j___ZN3RBX4Name9doDeclareILZNS_11sBodyThrustEEEERKS0_v
// type: int(void)
pub fn stub_0xf41754() -> ! { todo!("0xf41754 j___ZN3RBX4Name9doDeclareILZNS_11sBodyThrustEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sBodyPositionEEEERKS0_v")]
// 0xf41764 — j___ZN3RBX4Name9doDeclareILZNS_13sBodyPositionEEEERKS0_v
// type: int(void)
pub fn stub_0xf41764() -> ! { todo!("0xf41764 j___ZN3RBX4Name9doDeclareILZNS_13sBodyPositionEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_13sBodyVelocityEEEERKS0_v")]
// 0xf41774 — j___ZN3RBX4Name9doDeclareILZNS_13sBodyVelocityEEEERKS0_v
// type: int(void)
pub fn stub_0xf41774() -> ! { todo!("0xf41774 j___ZN3RBX4Name9doDeclareILZNS_13sBodyVelocityEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_20sBodyAngularVelocityEEEERKS0_v")]
// 0xf41784 — j___ZN3RBX4Name9doDeclareILZNS_20sBodyAngularVelocityEEEERKS0_v
// type: int(void)
pub fn stub_0xf41784() -> ! { todo!("0xf41784 j___ZN3RBX4Name9doDeclareILZNS_20sBodyAngularVelocityEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_7sRocketEEEERKS0_v")]
// 0xf41794 — j___ZN3RBX4Name9doDeclareILZNS_7sRocketEEEERKS0_v
// type: int(void)
pub fn stub_0xf41794() -> ! { todo!("0xf41794 j___ZN3RBX4Name9doDeclareILZNS_7sRocketEEEERKS0_v") }

#[doc(alias = "j___ZN3RBX4Name9doDeclareILZNS_9sBodyGyroEEEERKS0_v")]
// 0xf417a4 — j___ZN3RBX4Name9doDeclareILZNS_9sBodyGyroEEEERKS0_v
// type: int(void)
pub fn stub_0xf417a4() -> ! { todo!("0xf417a4 j___ZN3RBX4Name9doDeclareILZNS_9sBodyGyroEEEERKS0_v") }
