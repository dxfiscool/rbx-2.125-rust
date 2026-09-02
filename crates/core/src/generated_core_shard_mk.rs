//! core shard mk — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 2973 uncovered before -> 2873 after, batch 0xf10e70..0xf130f8).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, unstable_name_collisions, clippy::all, unused_attributes)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[FlurryImpl sendSessionsToServerForCreateSession]")]
// 0xf10e70 — -[FlurryImpl sendSessionsToServerForCreateSession]
// type: void __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf10e70() -> ! { todo!("0xf10e70 -[FlurryImpl sendSessionsToServerForCreateSession]") }

#[doc(alias = "___50-[FlurryImpl sendSessionsToServerForCreateSession]_block_invoke_0")]
// 0xf10f10 — ___50-[FlurryImpl sendSessionsToServerForCreateSession]_block_invoke_0
pub fn stub_0xf10f10() -> ! { todo!("0xf10f10 ___50-[FlurryImpl sendSessionsToServerForCreateSession]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_161")]
// 0xf10f38 — ___copy_helper_block_161
pub fn stub_0xf10f38() -> ! { todo!("0xf10f38 ___copy_helper_block_161") }

#[doc(alias = "___destroy_helper_block_162")]
// 0xf10f48 — ___destroy_helper_block_162
pub fn stub_0xf10f48() -> ! { todo!("0xf10f48 ___destroy_helper_block_162") }

#[doc(alias = "-[FlurryImpl latitude]")]
// 0xf10f58 — -[FlurryImpl latitude]
// type: double __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf10f58() -> ! { todo!("0xf10f58 -[FlurryImpl latitude]") }

#[doc(alias = "___22-[FlurryImpl latitude]_block_invoke_0")]
// 0xf11118 — ___22-[FlurryImpl latitude]_block_invoke_0
pub fn stub_0xf11118() -> ! { todo!("0xf11118 ___22-[FlurryImpl latitude]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_166")]
// 0xf1114c — ___copy_helper_block_166
pub fn stub_0xf1114c() -> ! { todo!("0xf1114c ___copy_helper_block_166") }

#[doc(alias = "___destroy_helper_block_167")]
// 0xf11170 — ___destroy_helper_block_167
pub fn stub_0xf11170() -> ! { todo!("0xf11170 ___destroy_helper_block_167") }

#[doc(alias = "-[FlurryImpl longitude]")]
// 0xf11188 — -[FlurryImpl longitude]
// type: double __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf11188() -> ! { todo!("0xf11188 -[FlurryImpl longitude]") }

#[doc(alias = "___23-[FlurryImpl longitude]_block_invoke_0")]
// 0xf11348 — ___23-[FlurryImpl longitude]_block_invoke_0
pub fn stub_0xf11348() -> ! { todo!("0xf11348 ___23-[FlurryImpl longitude]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_171")]
// 0xf1137c — ___copy_helper_block_171
pub fn stub_0xf1137c() -> ! { todo!("0xf1137c ___copy_helper_block_171") }

#[doc(alias = "___destroy_helper_block_172")]
// 0xf113a0 — ___destroy_helper_block_172
pub fn stub_0xf113a0() -> ! { todo!("0xf113a0 ___destroy_helper_block_172") }

#[doc(alias = "-[FlurryImpl accuracy]")]
// 0xf113b8 — -[FlurryImpl accuracy]
// type: double __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf113b8() -> ! { todo!("0xf113b8 -[FlurryImpl accuracy]") }

#[doc(alias = "___22-[FlurryImpl accuracy]_block_invoke_0")]
// 0xf11580 — ___22-[FlurryImpl accuracy]_block_invoke_0
pub fn stub_0xf11580() -> ! { todo!("0xf11580 ___22-[FlurryImpl accuracy]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_176")]
// 0xf115bc — ___copy_helper_block_176
pub fn stub_0xf115bc() -> ! { todo!("0xf115bc ___copy_helper_block_176") }

#[doc(alias = "___destroy_helper_block_177")]
// 0xf115e0 — ___destroy_helper_block_177
pub fn stub_0xf115e0() -> ! { todo!("0xf115e0 ___destroy_helper_block_177") }

#[doc(alias = "-[FlurryImpl gender]")]
// 0xf115f8 — -[FlurryImpl gender]
// type: int __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf115f8() -> ! { todo!("0xf115f8 -[FlurryImpl gender]") }

#[doc(alias = "___20-[FlurryImpl gender]_block_invoke_0")]
// 0xf117a8 — ___20-[FlurryImpl gender]_block_invoke_0
pub fn stub_0xf117a8() -> ! { todo!("0xf117a8 ___20-[FlurryImpl gender]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_181")]
// 0xf117d8 — ___copy_helper_block_181
pub fn stub_0xf117d8() -> ! { todo!("0xf117d8 ___copy_helper_block_181") }

#[doc(alias = "___destroy_helper_block_182")]
// 0xf117fc — ___destroy_helper_block_182
pub fn stub_0xf117fc() -> ! { todo!("0xf117fc ___destroy_helper_block_182") }

#[doc(alias = "-[FlurryImpl age]")]
// 0xf11814 — -[FlurryImpl age]
// type: id __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf11814() -> ! { todo!("0xf11814 -[FlurryImpl age]") }

#[doc(alias = "___Block_byref_object_copy__0")]
// 0xf119e0 — ___Block_byref_object_copy__0
pub fn stub_0xf119e0() -> ! { todo!("0xf119e0 ___Block_byref_object_copy__0") }

#[doc(alias = "___Block_byref_object_dispose__0")]
// 0xf119f0 — ___Block_byref_object_dispose__0
pub fn stub_0xf119f0() -> ! { todo!("0xf119f0 ___Block_byref_object_dispose__0") }

#[doc(alias = "___17-[FlurryImpl age]_block_invoke_0")]
// 0xf11a00 — ___17-[FlurryImpl age]_block_invoke_0
pub fn stub_0xf11a00() -> ! { todo!("0xf11a00 ___17-[FlurryImpl age]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_186")]
// 0xf11a30 — ___copy_helper_block_186
pub fn stub_0xf11a30() -> ! { todo!("0xf11a30 ___copy_helper_block_186") }

#[doc(alias = "___destroy_helper_block_187")]
// 0xf11a54 — ___destroy_helper_block_187
pub fn stub_0xf11a54() -> ! { todo!("0xf11a54 ___destroy_helper_block_187") }

#[doc(alias = "-[FlurryImpl pageViewCount]")]
// 0xf11a6c — -[FlurryImpl pageViewCount]
// type: int __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf11a6c() -> ! { todo!("0xf11a6c -[FlurryImpl pageViewCount]") }

#[doc(alias = "___27-[FlurryImpl pageViewCount]_block_invoke_0")]
// 0xf11c1c — ___27-[FlurryImpl pageViewCount]_block_invoke_0
pub fn stub_0xf11c1c() -> ! { todo!("0xf11c1c ___27-[FlurryImpl pageViewCount]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_191_0")]
// 0xf11c4c — ___copy_helper_block_191_0
// type: void __fastcall(int, int)
pub fn stub_0xf11c4c() -> ! { todo!("0xf11c4c ___copy_helper_block_191_0") }

#[doc(alias = "___destroy_helper_block_192_0")]
// 0xf11c70 — ___destroy_helper_block_192_0
pub fn stub_0xf11c70() -> ! { todo!("0xf11c70 ___destroy_helper_block_192_0") }

#[doc(alias = "-[FlurryImpl pauseTime]")]
// 0xf11c88 — -[FlurryImpl pauseTime]
// type: NSDate *__cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf11c88() -> ! { todo!("0xf11c88 -[FlurryImpl pauseTime]") }

#[doc(alias = "___23-[FlurryImpl pauseTime]_block_invoke_0")]
// 0xf11e54 — ___23-[FlurryImpl pauseTime]_block_invoke_0
pub fn stub_0xf11e54() -> ! { todo!("0xf11e54 ___23-[FlurryImpl pauseTime]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_196")]
// 0xf11e84 — ___copy_helper_block_196
pub fn stub_0xf11e84() -> ! { todo!("0xf11e84 ___copy_helper_block_196") }

#[doc(alias = "___destroy_helper_block_197")]
// 0xf11ea8 — ___destroy_helper_block_197
pub fn stub_0xf11ea8() -> ! { todo!("0xf11ea8 ___destroy_helper_block_197") }

#[doc(alias = "+[FlurryImpl registerBackgoundTask]")]
// 0xf11ec0 — +[FlurryImpl registerBackgoundTask]
// type: unsigned int __cdecl(id, SEL)
pub fn stub_0xf11ec0() -> ! { todo!("0xf11ec0 +[FlurryImpl registerBackgoundTask]") }

#[doc(alias = "___35+[FlurryImpl registerBackgoundTask]_block_invoke_0")]
// 0xf11f68 — ___35+[FlurryImpl registerBackgoundTask]_block_invoke_0
// type: void __cdecl(id)
pub fn stub_0xf11f68() -> ! { todo!("0xf11f68 ___35+[FlurryImpl registerBackgoundTask]_block_invoke_0") }

#[doc(alias = "+[FlurryImpl unregisterBackgoundTask:]")]
// 0xf11fa0 — +[FlurryImpl unregisterBackgoundTask:]
// type: void __cdecl(id, SEL, unsigned int)
pub fn stub_0xf11fa0() -> ! { todo!("0xf11fa0 +[FlurryImpl unregisterBackgoundTask:]") }

#[doc(alias = "-[FlurryImpl session]")]
// 0xf12010 — -[FlurryImpl session]
// type: FlurrySession *__cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf12010() -> ! { todo!("0xf12010 -[FlurryImpl session]") }

#[doc(alias = "-[FlurryImpl setSession:]")]
// 0xf12020 — -[FlurryImpl setSession:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf12020() -> ! { todo!("0xf12020 -[FlurryImpl setSession:]") }

#[doc(alias = "-[FlurryImpl apiKey]")]
// 0xf12044 — -[FlurryImpl apiKey]
// type: NSString *__cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf12044() -> ! { todo!("0xf12044 -[FlurryImpl apiKey]") }

#[doc(alias = "-[FlurryImpl setApiKey:]")]
// 0xf1205c — -[FlurryImpl setApiKey:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf1205c() -> ! { todo!("0xf1205c -[FlurryImpl setApiKey:]") }

#[doc(alias = "-[FlurryImpl startTime]")]
// 0xf12080 — -[FlurryImpl startTime]
// type: NSDate *__cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf12080() -> ! { todo!("0xf12080 -[FlurryImpl startTime]") }

#[doc(alias = "-[FlurryImpl setStartTime:]")]
// 0xf12098 — -[FlurryImpl setStartTime:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf12098() -> ! { todo!("0xf12098 -[FlurryImpl setStartTime:]") }

#[doc(alias = "-[FlurryImpl queue]")]
// 0xf120bc — -[FlurryImpl queue]
// type: dispatch_queue_s *__cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf120bc() -> ! { todo!("0xf120bc -[FlurryImpl queue]") }

#[doc(alias = "-[FlurryImpl setQueue:]")]
// 0xf120cc — -[FlurryImpl setQueue:]
// type: void __cdecl(FlurryImpl *self, SEL, dispatch_queue_s *)
pub fn stub_0xf120cc() -> ! { todo!("0xf120cc -[FlurryImpl setQueue:]") }

#[doc(alias = "-[FlurryImpl isBackgroundSupported]")]
// 0xf120dc — -[FlurryImpl isBackgroundSupported]
// type: char __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf120dc() -> ! { todo!("0xf120dc -[FlurryImpl isBackgroundSupported]") }

#[doc(alias = "-[FlurryImpl setIsBackgroundSupported:]")]
// 0xf120ec — -[FlurryImpl setIsBackgroundSupported:]
// type: void __cdecl(FlurryImpl *self, SEL, char)
pub fn stub_0xf120ec() -> ! { todo!("0xf120ec -[FlurryImpl setIsBackgroundSupported:]") }

#[doc(alias = "+[FlurrySharedData instance]")]
// 0xf120fc — +[FlurrySharedData instance]
// type: id __cdecl(id, SEL)
pub fn stub_0xf120fc() -> ! { todo!("0xf120fc +[FlurrySharedData instance]") }

#[doc(alias = "-[FlurrySharedData storeAppCloudUserID:]")]
// 0xf121d4 — -[FlurrySharedData storeAppCloudUserID:]
// type: void __cdecl(FlurrySharedData *self, SEL, id)
pub fn stub_0xf121d4() -> ! { todo!("0xf121d4 -[FlurrySharedData storeAppCloudUserID:]") }

#[doc(alias = "-[FlurrySharedData storedAppCloudUserID]")]
// 0xf12248 — -[FlurrySharedData storedAppCloudUserID]
// type: id __cdecl(FlurrySharedData *self, SEL)
pub fn stub_0xf12248() -> ! { todo!("0xf12248 -[FlurrySharedData storedAppCloudUserID]") }

#[doc(alias = "-[FlurryDataSenderBase initialize]")]
// 0xf12284 — -[FlurryDataSenderBase initialize]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12284() -> ! { todo!("0xf12284 -[FlurryDataSenderBase initialize]") }

#[doc(alias = "-[FlurryDataSenderBase init]")]
// 0xf12308 — -[FlurryDataSenderBase init]
// type: FlurryDataSenderBase *__cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12308() -> ! { todo!("0xf12308 -[FlurryDataSenderBase init]") }

#[doc(alias = "-[FlurryDataSenderBase initWithQueue:]")]
// 0xf1234c — -[FlurryDataSenderBase initWithQueue:]
// type: FlurryDataSenderBase *__cdecl(FlurryDataSenderBase *self, SEL, dispatch_queue_s *)
pub fn stub_0xf1234c() -> ! { todo!("0xf1234c -[FlurryDataSenderBase initWithQueue:]") }

#[doc(alias = "-[FlurryDataSenderBase dealloc]")]
// 0xf123ec — -[FlurryDataSenderBase dealloc]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf123ec() -> ! { todo!("0xf123ec -[FlurryDataSenderBase dealloc]") }

#[doc(alias = "-[FlurryDataSenderBase hasOngoingTasksWindow]")]
// 0xf12478 — -[FlurryDataSenderBase hasOngoingTasksWindow]
// type: char __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12478() -> ! { todo!("0xf12478 -[FlurryDataSenderBase hasOngoingTasksWindow]") }

#[doc(alias = "-[FlurryDataSenderBase registerNewTask:]")]
// 0xf124a8 — -[FlurryDataSenderBase registerNewTask:]
// type: void __cdecl(FlurryDataSenderBase *self, SEL, id)
pub fn stub_0xf124a8() -> ! { todo!("0xf124a8 -[FlurryDataSenderBase registerNewTask:]") }

#[doc(alias = "-[FlurryDataSenderBase unregisterTask:completedSuccessfuly:]")]
// 0xf1251c — -[FlurryDataSenderBase unregisterTask:completedSuccessfuly:]
// type: void __cdecl(FlurryDataSenderBase *self, SEL, id, char)
pub fn stub_0xf1251c() -> ! { todo!("0xf1251c -[FlurryDataSenderBase unregisterTask:completedSuccessfuly:]") }

#[doc(alias = "-[FlurryDataSenderBase retransmitNotSentBlocks]")]
// 0xf12600 — -[FlurryDataSenderBase retransmitNotSentBlocks]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12600() -> ! { todo!("0xf12600 -[FlurryDataSenderBase retransmitNotSentBlocks]") }

#[doc(alias = "-[FlurryDataSenderBase didCompleteAllTasks]")]
// 0xf12618 — -[FlurryDataSenderBase didCompleteAllTasks]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12618() -> ! { todo!("0xf12618 -[FlurryDataSenderBase didCompleteAllTasks]") }

#[doc(alias = "-[FlurryDataSenderBase cancelTasks]")]
// 0xf12630 — -[FlurryDataSenderBase cancelTasks]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12630() -> ! { todo!("0xf12630 -[FlurryDataSenderBase cancelTasks]") }

#[doc(alias = "-[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]")]
// 0xf12680 — -[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12680() -> ! { todo!("0xf12680 -[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]") }

#[doc(alias = "___59-[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]_block_invoke_0")]
// 0xf12774 — ___59-[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]_block_invoke_0
// type: void __cdecl(id)
pub fn stub_0xf12774() -> ! { todo!("0xf12774 ___59-[FlurryDataSenderBase startBackgroundTaskTrackingIfNeeded]_block_invoke_0") }

#[doc(alias = "-[FlurryDataSenderBase stopBackgroundTasksTracking]")]
// 0xf127ac — -[FlurryDataSenderBase stopBackgroundTasksTracking]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf127ac() -> ! { todo!("0xf127ac -[FlurryDataSenderBase stopBackgroundTasksTracking]") }

#[doc(alias = "-[FlurryDataSenderBase networkStatusChanged:]")]
// 0xf128ac — -[FlurryDataSenderBase networkStatusChanged:]
// type: void __cdecl(FlurryDataSenderBase *self, SEL, id)
pub fn stub_0xf128ac() -> ! { todo!("0xf128ac -[FlurryDataSenderBase networkStatusChanged:]") }

#[doc(alias = "___45-[FlurryDataSenderBase networkStatusChanged:]_block_invoke_0")]
// 0xf129a0 — ___45-[FlurryDataSenderBase networkStatusChanged:]_block_invoke_0
pub fn stub_0xf129a0() -> ! { todo!("0xf129a0 ___45-[FlurryDataSenderBase networkStatusChanged:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block__42")]
// 0xf129b8 — ___copy_helper_block__42
pub fn stub_0xf129b8() -> ! { todo!("0xf129b8 ___copy_helper_block__42") }

#[doc(alias = "___destroy_helper_block__42")]
// 0xf129c8 — ___destroy_helper_block__42
pub fn stub_0xf129c8() -> ! { todo!("0xf129c8 ___destroy_helper_block__42") }

#[doc(alias = "-[FlurryDataSenderBase performRetransmitNotSentBlocks]")]
// 0xf129d8 — -[FlurryDataSenderBase performRetransmitNotSentBlocks]
// type: void __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf129d8() -> ! { todo!("0xf129d8 -[FlurryDataSenderBase performRetransmitNotSentBlocks]") }

#[doc(alias = "-[FlurryDataSenderBase runningTasks]")]
// 0xf12a70 — -[FlurryDataSenderBase runningTasks]
// type: NSMutableArray *__cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12a70() -> ! { todo!("0xf12a70 -[FlurryDataSenderBase runningTasks]") }

#[doc(alias = "-[FlurryDataSenderBase setRunningTasks:]")]
// 0xf12a80 — -[FlurryDataSenderBase setRunningTasks:]
// type: void __cdecl(FlurryDataSenderBase *self, SEL, id)
pub fn stub_0xf12a80() -> ! { todo!("0xf12a80 -[FlurryDataSenderBase setRunningTasks:]") }

#[doc(alias = "-[FlurryDataSenderBase backgroundTask]")]
// 0xf12aa4 — -[FlurryDataSenderBase backgroundTask]
// type: unsigned int __cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12aa4() -> ! { todo!("0xf12aa4 -[FlurryDataSenderBase backgroundTask]") }

#[doc(alias = "-[FlurryDataSenderBase setBackgroundTask:]")]
// 0xf12ab4 — -[FlurryDataSenderBase setBackgroundTask:]
// type: void __cdecl(FlurryDataSenderBase *self, SEL, unsigned int)
pub fn stub_0xf12ab4() -> ! { todo!("0xf12ab4 -[FlurryDataSenderBase setBackgroundTask:]") }

#[doc(alias = "-[FlurryDataSenderBase queue]")]
// 0xf12ac4 — -[FlurryDataSenderBase queue]
// type: dispatch_queue_s *__cdecl(FlurryDataSenderBase *self, SEL)
pub fn stub_0xf12ac4() -> ! { todo!("0xf12ac4 -[FlurryDataSenderBase queue]") }

#[doc(alias = "-[FlurryDataSenderBase setQueue:]")]
// 0xf12ad4 — -[FlurryDataSenderBase setQueue:]
// type: void __cdecl(FlurryDataSenderBase *self, SEL, dispatch_queue_s *)
pub fn stub_0xf12ad4() -> ! { todo!("0xf12ad4 -[FlurryDataSenderBase setQueue:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage init]")]
// 0xf12bb4 — -[FlurryGlobalVariableStorage init]
// type: FlurryGlobalVariableStorage *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12bb4() -> ! { todo!("0xf12bb4 -[FlurryGlobalVariableStorage init]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setApiKey:]")]
// 0xf12d3c — -[FlurryGlobalVariableStorage setApiKey:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf12d3c() -> ! { todo!("0xf12d3c -[FlurryGlobalVariableStorage setApiKey:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setPushToken:]")]
// 0xf12dd0 — -[FlurryGlobalVariableStorage setPushToken:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf12dd0() -> ! { todo!("0xf12dd0 -[FlurryGlobalVariableStorage setPushToken:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage savedResponseUrl]")]
// 0xf12ec8 — -[FlurryGlobalVariableStorage savedResponseUrl]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12ec8() -> ! { todo!("0xf12ec8 -[FlurryGlobalVariableStorage savedResponseUrl]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setSavedResponseUrl:]")]
// 0xf12ee0 — -[FlurryGlobalVariableStorage setSavedResponseUrl:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf12ee0() -> ! { todo!("0xf12ee0 -[FlurryGlobalVariableStorage setSavedResponseUrl:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage apiKey]")]
// 0xf12f04 — -[FlurryGlobalVariableStorage apiKey]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12f04() -> ! { todo!("0xf12f04 -[FlurryGlobalVariableStorage apiKey]") }

#[doc(alias = "-[FlurryGlobalVariableStorage udidEnabled]")]
// 0xf12f1c — -[FlurryGlobalVariableStorage udidEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12f1c() -> ! { todo!("0xf12f1c -[FlurryGlobalVariableStorage udidEnabled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setUdidEnabled:]")]
// 0xf12f34 — -[FlurryGlobalVariableStorage setUdidEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf12f34() -> ! { todo!("0xf12f34 -[FlurryGlobalVariableStorage setUdidEnabled:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage pauseSecondsBeforeStartingNewSession]")]
// 0xf12f4c — -[FlurryGlobalVariableStorage pauseSecondsBeforeStartingNewSession]
// type: int __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12f4c() -> ! { todo!("0xf12f4c -[FlurryGlobalVariableStorage pauseSecondsBeforeStartingNewSession]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setPauseSecondsBeforeStartingNewSession:]")]
// 0xf12f60 — -[FlurryGlobalVariableStorage setPauseSecondsBeforeStartingNewSession:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, int)
pub fn stub_0xf12f60() -> ! { todo!("0xf12f60 -[FlurryGlobalVariableStorage setPauseSecondsBeforeStartingNewSession:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage pauseTime]")]
// 0xf12f78 — -[FlurryGlobalVariableStorage pauseTime]
// type: NSDate *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12f78() -> ! { todo!("0xf12f78 -[FlurryGlobalVariableStorage pauseTime]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setPauseTime:]")]
// 0xf12f90 — -[FlurryGlobalVariableStorage setPauseTime:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf12f90() -> ! { todo!("0xf12f90 -[FlurryGlobalVariableStorage setPauseTime:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage sessionReportsOnCloseEnabled]")]
// 0xf12fb4 — -[FlurryGlobalVariableStorage sessionReportsOnCloseEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12fb4() -> ! { todo!("0xf12fb4 -[FlurryGlobalVariableStorage sessionReportsOnCloseEnabled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setSessionReportsOnCloseEnabled:]")]
// 0xf12fcc — -[FlurryGlobalVariableStorage setSessionReportsOnCloseEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf12fcc() -> ! { todo!("0xf12fcc -[FlurryGlobalVariableStorage setSessionReportsOnCloseEnabled:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage sessionReportsOnPauseEnabled]")]
// 0xf12fe4 — -[FlurryGlobalVariableStorage sessionReportsOnPauseEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf12fe4() -> ! { todo!("0xf12fe4 -[FlurryGlobalVariableStorage sessionReportsOnPauseEnabled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setSessionReportsOnPauseEnabled:]")]
// 0xf12ffc — -[FlurryGlobalVariableStorage setSessionReportsOnPauseEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf12ffc() -> ! { todo!("0xf12ffc -[FlurryGlobalVariableStorage setSessionReportsOnPauseEnabled:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage backgroundSessionEnabled]")]
// 0xf13014 — -[FlurryGlobalVariableStorage backgroundSessionEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf13014() -> ! { todo!("0xf13014 -[FlurryGlobalVariableStorage backgroundSessionEnabled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setBackgroundSessionEnabled:]")]
// 0xf1302c — -[FlurryGlobalVariableStorage setBackgroundSessionEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf1302c() -> ! { todo!("0xf1302c -[FlurryGlobalVariableStorage setBackgroundSessionEnabled:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage crashReportingEnabled]")]
// 0xf13044 — -[FlurryGlobalVariableStorage crashReportingEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf13044() -> ! { todo!("0xf13044 -[FlurryGlobalVariableStorage crashReportingEnabled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setCrashReportingEnabled:]")]
// 0xf1305c — -[FlurryGlobalVariableStorage setCrashReportingEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf1305c() -> ! { todo!("0xf1305c -[FlurryGlobalVariableStorage setCrashReportingEnabled:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage appVersion]")]
// 0xf13074 — -[FlurryGlobalVariableStorage appVersion]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf13074() -> ! { todo!("0xf13074 -[FlurryGlobalVariableStorage appVersion]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setAppVersion:]")]
// 0xf1308c — -[FlurryGlobalVariableStorage setAppVersion:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf1308c() -> ! { todo!("0xf1308c -[FlurryGlobalVariableStorage setAppVersion:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage eventLoggingEnabled]")]
// 0xf130b0 — -[FlurryGlobalVariableStorage eventLoggingEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf130b0() -> ! { todo!("0xf130b0 -[FlurryGlobalVariableStorage eventLoggingEnabled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setEventLoggingEnabled:]")]
// 0xf130c8 — -[FlurryGlobalVariableStorage setEventLoggingEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf130c8() -> ! { todo!("0xf130c8 -[FlurryGlobalVariableStorage setEventLoggingEnabled:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage userID]")]
// 0xf130e0 — -[FlurryGlobalVariableStorage userID]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf130e0() -> ! { todo!("0xf130e0 -[FlurryGlobalVariableStorage userID]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setUserID:]")]
// 0xf130f8 — -[FlurryGlobalVariableStorage setUserID:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf130f8() -> ! { todo!("0xf130f8 -[FlurryGlobalVariableStorage setUserID:]") }

