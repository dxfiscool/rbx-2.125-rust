//! core shard kv — 120 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 after ku 0xed19b4..0xed75f4 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 52285 filtered, 4243 remaining before -> 4123 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_tf_session_write_sequence_number")]
#[doc(alias = "_tf_session_write_sequence_number")]
// 0xed19b4 — _tf_session_write_sequence_number
pub fn stub_0xed19b4() -> ! {
    todo!("0xed19b4 _tf_session_write_sequence_number")
}

#[doc(alias = "-[TFSessionManager doApiHandshake]")]
#[doc(alias = "-[TFSessionManager doApiHandshake]")]
// 0xed19d8 — -[TFSessionManager doApiHandshake]
// type: void __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed19d8() -> ! {
    todo!("0xed19d8 -[TFSessionManager doApiHandshake]")
}

#[doc(alias = "___34-[TFSessionManager doApiHandshake]_block_invoke")]
#[doc(alias = "___34-[TFSessionManager doApiHandshake]_block_invoke")]
// 0xed1c78 — ___34-[TFSessionManager doApiHandshake]_block_invoke
// type: int __fastcall(int, int, id, int, int)
pub fn stub_0xed1c78() -> ! {
    todo!("0xed1c78 ___34-[TFSessionManager doApiHandshake]_block_invoke")
}

#[doc(alias = "___copy_helper_block_328")]
#[doc(alias = "___copy_helper_block_328")]
// 0xed1cbc — ___copy_helper_block_328
pub fn stub_0xed1cbc() -> ! {
    todo!("0xed1cbc ___copy_helper_block_328")
}

#[doc(alias = "___destroy_helper_block_329")]
#[doc(alias = "___destroy_helper_block_329")]
// 0xed1cc8 — ___destroy_helper_block_329
pub fn stub_0xed1cc8() -> ! {
    todo!("0xed1cc8 ___destroy_helper_block_329")
}

#[doc(alias = "-[TFSessionManager finishedHandshakeWithData:response:error:]")]
#[doc(alias = "-[TFSessionManager finishedHandshakeWithData:response:error:]")]
// 0xed1cd4 — -[TFSessionManager finishedHandshakeWithData:response:error:]
// type: void __cdecl(TFSessionManager *self, SEL, id, id, id)
pub fn stub_0xed1cd4() -> ! {
    todo!("0xed1cd4 -[TFSessionManager finishedHandshakeWithData:response:error:]")
}

#[doc(alias = "__tf_session_info_folder_path")]
#[doc(alias = "__tf_session_info_folder_path")]
// 0xed1f58 — __tf_session_info_folder_path
pub fn stub_0xed1f58() -> ! {
    todo!("0xed1f58 __tf_session_info_folder_path")
}

#[doc(alias = "____tf_session_info_folder_path_block_invoke")]
#[doc(alias = "____tf_session_info_folder_path_block_invoke")]
// 0xed1f8c — ____tf_session_info_folder_path_block_invoke
// type: void __cdecl(id)
pub fn stub_0xed1f8c() -> ! {
    todo!("0xed1f8c ____tf_session_info_folder_path_block_invoke")
}

#[doc(alias = "-[TFSessionManager sessionFilePathForFileName:extension:]")]
#[doc(alias = "-[TFSessionManager sessionFilePathForFileName:extension:]")]
// 0xed2020 — -[TFSessionManager sessionFilePathForFileName:extension:]
// type: id __cdecl(TFSessionManager *self, SEL, id, id)
pub fn stub_0xed2020() -> ! {
    todo!("0xed2020 -[TFSessionManager sessionFilePathForFileName:extension:]")
}

#[doc(alias = "-[TFSessionManager saveLastSessionInformation]")]
#[doc(alias = "-[TFSessionManager saveLastSessionInformation]")]
// 0xed20ac — -[TFSessionManager saveLastSessionInformation]
// type: void __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed20ac() -> ! {
    todo!("0xed20ac -[TFSessionManager saveLastSessionInformation]")
}

#[doc(alias = "-[TFSessionManager saveEndSessionInformation]")]
#[doc(alias = "-[TFSessionManager saveEndSessionInformation]")]
// 0xed2280 — -[TFSessionManager saveEndSessionInformation]
// type: void __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2280() -> ! {
    todo!("0xed2280 -[TFSessionManager saveEndSessionInformation]")
}

#[doc(alias = "-[TFSessionManager sequenceNumberPathForSessionID:]")]
#[doc(alias = "-[TFSessionManager sequenceNumberPathForSessionID:]")]
// 0xed23cc — -[TFSessionManager sequenceNumberPathForSessionID:]
// type: id __cdecl(TFSessionManager *self, SEL, id)
pub fn stub_0xed23cc() -> ! {
    todo!("0xed23cc -[TFSessionManager sequenceNumberPathForSessionID:]")
}

#[doc(alias = "-[TFSessionManager savedEndSessionInformationForSessionID:]")]
#[doc(alias = "-[TFSessionManager savedEndSessionInformationForSessionID:]")]
// 0xed23ec — -[TFSessionManager savedEndSessionInformationForSessionID:]
// type: id __cdecl(TFSessionManager *self, SEL, id)
pub fn stub_0xed23ec() -> ! {
    todo!("0xed23ec -[TFSessionManager savedEndSessionInformationForSessionID:]")
}

#[doc(alias = "-[TFSessionManager deleteSavedEndSessionInformationForSessionID:]")]
#[doc(alias = "-[TFSessionManager deleteSavedEndSessionInformationForSessionID:]")]
// 0xed2478 — -[TFSessionManager deleteSavedEndSessionInformationForSessionID:]
// type: void __cdecl(TFSessionManager *self, SEL, id)
pub fn stub_0xed2478() -> ! {
    todo!("0xed2478 -[TFSessionManager deleteSavedEndSessionInformationForSessionID:]")
}

#[doc(alias = "_tf_session_end_info_delete_safe")]
#[doc(alias = "_tf_session_end_info_delete_safe")]
// 0xed2534 — _tf_session_end_info_delete_safe
pub fn stub_0xed2534() -> ! {
    todo!("0xed2534 _tf_session_end_info_delete_safe")
}

#[doc(alias = "-[TFSessionManager sessionIDsWithOldSavedSessionInformation]")]
#[doc(alias = "-[TFSessionManager sessionIDsWithOldSavedSessionInformation]")]
// 0xed25a8 — -[TFSessionManager sessionIDsWithOldSavedSessionInformation]
// type: id __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed25a8() -> ! {
    todo!("0xed25a8 -[TFSessionManager sessionIDsWithOldSavedSessionInformation]")
}

#[doc(alias = "___Block_byref_object_copy_")]
#[doc(alias = "___Block_byref_object_copy_")]
// 0xed29a4 — ___Block_byref_object_copy_
pub fn stub_0xed29a4() -> ! {
    todo!("0xed29a4 ___Block_byref_object_copy_")
}

#[doc(alias = "___Block_byref_object_dispose_")]
#[doc(alias = "___Block_byref_object_dispose_")]
// 0xed29b0 — ___Block_byref_object_dispose_
pub fn stub_0xed29b0() -> ! {
    todo!("0xed29b0 ___Block_byref_object_dispose_")
}

#[doc(alias = "___60-[TFSessionManager sessionIDsWithOldSavedSessionInformation]_block_invoke")]
#[doc(alias = "___60-[TFSessionManager sessionIDsWithOldSavedSessionInformation]_block_invoke")]
// 0xed29bc — ___60-[TFSessionManager sessionIDsWithOldSavedSessionInformation]_block_invoke
pub fn stub_0xed29bc() -> ! {
    todo!("0xed29bc ___60-[TFSessionManager sessionIDsWithOldSavedSessionInformation]_block_invoke")
}

#[doc(alias = "___copy_helper_block_396")]
#[doc(alias = "___copy_helper_block_396")]
// 0xed2a04 — ___copy_helper_block_396
pub fn stub_0xed2a04() -> ! {
    todo!("0xed2a04 ___copy_helper_block_396")
}

#[doc(alias = "___destroy_helper_block_397")]
#[doc(alias = "___destroy_helper_block_397")]
// 0xed2a1c — ___destroy_helper_block_397
pub fn stub_0xed2a1c() -> ! {
    todo!("0xed2a1c ___destroy_helper_block_397")
}

#[doc(alias = "-[TFSessionManager shouldSendLogs]")]
#[doc(alias = "-[TFSessionManager shouldSendLogs]")]
// 0xed2a34 — -[TFSessionManager shouldSendLogs]
// type: char __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2a34() -> ! {
    todo!("0xed2a34 -[TFSessionManager shouldSendLogs]")
}

#[doc(alias = "-[TFSessionManager getStringOrNilFrom:withKey:]")]
#[doc(alias = "-[TFSessionManager getStringOrNilFrom:withKey:]")]
// 0xed2a70 — -[TFSessionManager getStringOrNilFrom:withKey:]
// type: id __cdecl(TFSessionManager *self, SEL, id, id)
pub fn stub_0xed2a70() -> ! {
    todo!("0xed2a70 -[TFSessionManager getStringOrNilFrom:withKey:]")
}

#[doc(alias = "-[TFSessionManager getNumberOrNilFrom:withKey:]")]
#[doc(alias = "-[TFSessionManager getNumberOrNilFrom:withKey:]")]
// 0xed2af4 — -[TFSessionManager getNumberOrNilFrom:withKey:]
// type: id __cdecl(TFSessionManager *self, SEL, id, id)
pub fn stub_0xed2af4() -> ! {
    todo!("0xed2af4 -[TFSessionManager getNumberOrNilFrom:withKey:]")
}

#[doc(alias = "-[TFSessionManager unixTime]")]
#[doc(alias = "-[TFSessionManager unixTime]")]
// 0xed2b78 — -[TFSessionManager unixTime]
// type: double __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2b78() -> ! {
    todo!("0xed2b78 -[TFSessionManager unixTime]")
}

#[doc(alias = "-[TFSessionManager getUUID]")]
#[doc(alias = "-[TFSessionManager getUUID]")]
// 0xed2bc4 — -[TFSessionManager getUUID]
// type: id __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2bc4() -> ! {
    todo!("0xed2bc4 -[TFSessionManager getUUID]")
}

#[doc(alias = "-[TFSessionManager useSeqNumber]")]
#[doc(alias = "-[TFSessionManager useSeqNumber]")]
// 0xed2be8 — -[TFSessionManager useSeqNumber]
// type: unsigned int __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2be8() -> ! {
    todo!("0xed2be8 -[TFSessionManager useSeqNumber]")
}

#[doc(alias = "-[TFSessionManager sendLogOnlyOnCrash]")]
#[doc(alias = "-[TFSessionManager sendLogOnlyOnCrash]")]
// 0xed2c24 — -[TFSessionManager sendLogOnlyOnCrash]
// type: char __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2c24() -> ! {
    todo!("0xed2c24 -[TFSessionManager sendLogOnlyOnCrash]")
}

#[doc(alias = "-[TFSessionManager setSendLogOnlyOnCrash:]")]
#[doc(alias = "-[TFSessionManager setSendLogOnlyOnCrash:]")]
// 0xed2c3c — -[TFSessionManager setSendLogOnlyOnCrash:]
// type: void __cdecl(TFSessionManager *self, SEL, char)
pub fn stub_0xed2c3c() -> ! {
    todo!("0xed2c3c -[TFSessionManager setSendLogOnlyOnCrash:]")
}

#[doc(alias = "-[TFSessionManager setSessionKeepAliveTimeout:]")]
#[doc(alias = "-[TFSessionManager setSessionKeepAliveTimeout:]")]
// 0xed2c54 — -[TFSessionManager setSessionKeepAliveTimeout:]
// type: void __cdecl(TFSessionManager *self, SEL, double)
pub fn stub_0xed2c54() -> ! {
    todo!("0xed2c54 -[TFSessionManager setSessionKeepAliveTimeout:]")
}

#[doc(alias = "-[TFSessionManager nextSeqNumber]")]
#[doc(alias = "-[TFSessionManager nextSeqNumber]")]
// 0xed2c68 — -[TFSessionManager nextSeqNumber]
// type: unsigned int __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2c68() -> ! {
    todo!("0xed2c68 -[TFSessionManager nextSeqNumber]")
}

#[doc(alias = "-[TFSessionManager setNextSeqNumber:]")]
#[doc(alias = "-[TFSessionManager setNextSeqNumber:]")]
// 0xed2c78 — -[TFSessionManager setNextSeqNumber:]
// type: void __cdecl(TFSessionManager *self, SEL, unsigned int)
pub fn stub_0xed2c78() -> ! {
    todo!("0xed2c78 -[TFSessionManager setNextSeqNumber:]")
}

#[doc(alias = "-[TFSessionManager inSession]")]
#[doc(alias = "-[TFSessionManager inSession]")]
// 0xed2c88 — -[TFSessionManager inSession]
// type: char __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2c88() -> ! {
    todo!("0xed2c88 -[TFSessionManager inSession]")
}

#[doc(alias = "-[TFSessionManager setInSession:]")]
#[doc(alias = "-[TFSessionManager setInSession:]")]
// 0xed2c98 — -[TFSessionManager setInSession:]
// type: void __cdecl(TFSessionManager *self, SEL, char)
pub fn stub_0xed2c98() -> ! {
    todo!("0xed2c98 -[TFSessionManager setInSession:]")
}

#[doc(alias = "-[TFSessionManager sessionID]")]
#[doc(alias = "-[TFSessionManager sessionID]")]
// 0xed2ca8 — -[TFSessionManager sessionID]
// type: NSString *__cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2ca8() -> ! {
    todo!("0xed2ca8 -[TFSessionManager sessionID]")
}

#[doc(alias = "-[TFSessionManager setSessionID:]")]
#[doc(alias = "-[TFSessionManager setSessionID:]")]
// 0xed2cc0 — -[TFSessionManager setSessionID:]
// type: void __cdecl(TFSessionManager *self, SEL, id)
pub fn stub_0xed2cc0() -> ! {
    todo!("0xed2cc0 -[TFSessionManager setSessionID:]")
}

#[doc(alias = "-[TFSessionManager .cxx_destruct]")]
#[doc(alias = "-[TFSessionManager .cxx_destruct]")]
// 0xed2ce4 — -[TFSessionManager .cxx_destruct]
// type: void __cdecl(TFSessionManager *self, SEL)
pub fn stub_0xed2ce4() -> ! {
    todo!("0xed2ce4 -[TFSessionManager .cxx_destruct]")
}

#[doc(alias = "_TFInstallCrashHandlers")]
#[doc(alias = "_TFInstallCrashHandlers")]
// 0xed2d64 — _TFInstallCrashHandlers
pub fn stub_0xed2d64() -> ! {
    todo!("0xed2d64 _TFInstallCrashHandlers")
}

#[doc(alias = "_TFHandleExceptions")]
#[doc(alias = "_TFHandleExceptions")]
// 0xed2eec — _TFHandleExceptions
pub fn stub_0xed2eec() -> ! {
    todo!("0xed2eec _TFHandleExceptions")
}

#[doc(alias = "_tf_replace_old_signal_handler")]
#[doc(alias = "_tf_replace_old_signal_handler")]
// 0xed3310 — _tf_replace_old_signal_handler
// type: int __fastcall(int)
pub fn stub_0xed3310() -> ! {
    todo!("0xed3310 _tf_replace_old_signal_handler")
}

#[doc(alias = "_TFSignalHandler")]
#[doc(alias = "_TFSignalHandler")]
// 0xed3354 — _TFSignalHandler
pub fn stub_0xed3354() -> ! {
    todo!("0xed3354 _TFSignalHandler")
}

#[doc(alias = "_TFUninstallCrashHandlers")]
#[doc(alias = "_TFUninstallCrashHandlers")]
// 0xed36dc — _TFUninstallCrashHandlers
pub fn stub_0xed36dc() -> ! {
    todo!("0xed36dc _TFUninstallCrashHandlers")
}

#[doc(alias = "_tf_set_new_signal_handler")]
#[doc(alias = "_tf_set_new_signal_handler")]
// 0xed37f4 — _tf_set_new_signal_handler
pub fn stub_0xed37f4() -> ! {
    todo!("0xed37f4 _tf_set_new_signal_handler")
}

#[doc(alias = "_tf_move_current_crash_report_to_old_crashes_folder")]
#[doc(alias = "_tf_move_current_crash_report_to_old_crashes_folder")]
// 0xed3820 — _tf_move_current_crash_report_to_old_crashes_folder
// type: void()
pub fn stub_0xed3820() -> ! {
    todo!("0xed3820 _tf_move_current_crash_report_to_old_crashes_folder")
}

#[doc(alias = "_TFSendOldCrashReports")]
#[doc(alias = "_TFSendOldCrashReports")]
// 0xed395c — _TFSendOldCrashReports
pub fn stub_0xed395c() -> ! {
    todo!("0xed395c _TFSendOldCrashReports")
}

#[doc(alias = "_tf_crash_report_start")]
#[doc(alias = "_tf_crash_report_start")]
// 0xed3b44 — _tf_crash_report_start
pub fn stub_0xed3b44() -> ! {
    todo!("0xed3b44 _tf_crash_report_start")
}

#[doc(alias = "_testflight_backtrace")]
#[doc(alias = "_testflight_backtrace")]
// 0xed3bcc — _testflight_backtrace
pub fn stub_0xed3bcc() -> ! {
    todo!("0xed3bcc _testflight_backtrace")
}

#[doc(alias = "_tf_crash_report_finish")]
#[doc(alias = "_tf_crash_report_finish")]
// 0xed3da0 — _tf_crash_report_finish
pub fn stub_0xed3da0() -> ! {
    todo!("0xed3da0 _tf_crash_report_finish")
}

#[doc(alias = "+[TFEventManager initialize]")]
#[doc(alias = "+[TFEventManager initialize]")]
// 0xed3e50 — +[TFEventManager initialize]
// type: void __cdecl(id, SEL)
pub fn stub_0xed3e50() -> ! {
    todo!("0xed3e50 +[TFEventManager initialize]")
}

#[doc(alias = "___28+[TFEventManager initialize]_block_invoke")]
#[doc(alias = "___28+[TFEventManager initialize]_block_invoke")]
// 0xed3eb4 — ___28+[TFEventManager initialize]_block_invoke
pub fn stub_0xed3eb4() -> ! {
    todo!("0xed3eb4 ___28+[TFEventManager initialize]_block_invoke")
}

#[doc(alias = "___copy_helper_block__32")]
#[doc(alias = "___copy_helper_block__32")]
// 0xed3f78 — ___copy_helper_block__32
pub fn stub_0xed3f78() -> ! {
    todo!("0xed3f78 ___copy_helper_block__32")
}

#[doc(alias = "___destroy_helper_block__32")]
#[doc(alias = "___destroy_helper_block__32")]
// 0xed3f7c — ___destroy_helper_block__32
pub fn stub_0xed3f7c() -> ! {
    todo!("0xed3f7c ___destroy_helper_block__32")
}

#[doc(alias = "+[TFEventManager eventManagerWithQueueKey:url:]")]
#[doc(alias = "+[TFEventManager eventManagerWithQueueKey:url:]")]
// 0xed4234 — +[TFEventManager eventManagerWithQueueKey:url:]
// type: TFEventManager *__cdecl(id, SEL, id, id)
pub fn stub_0xed4234() -> ! {
    todo!("0xed4234 +[TFEventManager eventManagerWithQueueKey:url:]")
}

#[doc(alias = "-[TFEventManager init]")]
#[doc(alias = "-[TFEventManager init]")]
// 0xed4294 — -[TFEventManager init]
// type: TFEventManager *__cdecl(TFEventManager *self, SEL)
pub fn stub_0xed4294() -> ! {
    todo!("0xed4294 -[TFEventManager init]")
}

#[doc(alias = "-[TFEventManager initWithQueueKey:url:]")]
#[doc(alias = "-[TFEventManager initWithQueueKey:url:]")]
// 0xed42bc — -[TFEventManager initWithQueueKey:url:]
// type: TFEventManager *__cdecl(TFEventManager *self, SEL, id, id)
pub fn stub_0xed42bc() -> ! {
    todo!("0xed42bc -[TFEventManager initWithQueueKey:url:]")
}

#[doc(alias = "-[TFEventManager dealloc]")]
#[doc(alias = "-[TFEventManager dealloc]")]
// 0xed4498 — -[TFEventManager dealloc]
// type: void __cdecl(TFEventManager *self, SEL)
pub fn stub_0xed4498() -> ! {
    todo!("0xed4498 -[TFEventManager dealloc]")
}

#[doc(alias = "+[TFEventManager flush]")]
#[doc(alias = "+[TFEventManager flush]")]
// 0xed44e4 — +[TFEventManager flush]
// type: void __cdecl(id, SEL)
pub fn stub_0xed44e4() -> ! {
    todo!("0xed44e4 +[TFEventManager flush]")
}

#[doc(alias = "-[TFEventManager _flush]")]
#[doc(alias = "-[TFEventManager _flush]")]
// 0xed4514 — -[TFEventManager _flush]
// type: void __cdecl(TFEventManager *self, SEL)
pub fn stub_0xed4514() -> ! {
    todo!("0xed4514 -[TFEventManager _flush]")
}

#[doc(alias = "-[TFEventManager enqueueObject:]")]
#[doc(alias = "-[TFEventManager enqueueObject:]")]
// 0xed4810 — -[TFEventManager enqueueObject:]
// type: void __cdecl(TFEventManager *self, SEL, id)
pub fn stub_0xed4810() -> ! {
    todo!("0xed4810 -[TFEventManager enqueueObject:]")
}

#[doc(alias = "-[TFEventManager enqueueObject:andSendImmediately:]")]
#[doc(alias = "-[TFEventManager enqueueObject:andSendImmediately:]")]
// 0xed4828 — -[TFEventManager enqueueObject:andSendImmediately:]
// type: void __cdecl(TFEventManager *self, SEL, id, char)
pub fn stub_0xed4828() -> ! {
    todo!("0xed4828 -[TFEventManager enqueueObject:andSendImmediately:]")
}

#[doc(alias = "-[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]")]
#[doc(alias = "-[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]")]
// 0xed49e4 — -[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]
// type: char __cdecl(TFEventManager *self, SEL, id, id, id)
pub fn stub_0xed49e4() -> ! {
    todo!("0xed49e4 -[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]")
}

#[doc(alias = "___77-[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]_block_invoke")]
#[doc(alias = "___77-[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]_block_invoke")]
// 0xed4c88 — ___77-[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]_block_invoke
// type: int __fastcall(int, id)
pub fn stub_0xed4c88() -> ! {
    todo!("0xed4c88 ___77-[TFEventManager enqueueDictionary:withExtraPairWithKey:andObjectDataWriter:]_block_invoke")
}

#[doc(alias = "___copy_helper_block_92")]
#[doc(alias = "___copy_helper_block_92")]
// 0xed4d54 — ___copy_helper_block_92
pub fn stub_0xed4d54() -> ! {
    todo!("0xed4d54 ___copy_helper_block_92")
}

#[doc(alias = "___destroy_helper_block_93")]
#[doc(alias = "___destroy_helper_block_93")]
// 0xed4d60 — ___destroy_helper_block_93
pub fn stub_0xed4d60() -> ! {
    todo!("0xed4d60 ___destroy_helper_block_93")
}

#[doc(alias = "-[TFEventManager _endCurrentEventQueue]")]
#[doc(alias = "-[TFEventManager _endCurrentEventQueue]")]
// 0xed4d6c — -[TFEventManager _endCurrentEventQueue]
// type: void __cdecl(TFEventManager *self, SEL)
pub fn stub_0xed4d6c() -> ! {
    todo!("0xed4d6c -[TFEventManager _endCurrentEventQueue]")
}

#[doc(alias = "-[TFEventManager _ensureEventQueueIsSetup]")]
#[doc(alias = "-[TFEventManager _ensureEventQueueIsSetup]")]
// 0xed4d8c — -[TFEventManager _ensureEventQueueIsSetup]
// type: void __cdecl(TFEventManager *self, SEL)
pub fn stub_0xed4d8c() -> ! {
    todo!("0xed4d8c -[TFEventManager _ensureEventQueueIsSetup]")
}

#[doc(alias = "-[TFEventManager _createNewQueueFileName]")]
#[doc(alias = "-[TFEventManager _createNewQueueFileName]")]
// 0xed4e30 — -[TFEventManager _createNewQueueFileName]
// type: id __cdecl(TFEventManager *self, SEL)
pub fn stub_0xed4e30() -> ! {
    todo!("0xed4e30 -[TFEventManager _createNewQueueFileName]")
}

#[doc(alias = "-[TFEventManager _createNewQueueAtPath:itemCount:]")]
#[doc(alias = "-[TFEventManager _createNewQueueAtPath:itemCount:]")]
// 0xed4e80 — -[TFEventManager _createNewQueueAtPath:itemCount:]
// type: void __cdecl(TFEventManager *self, SEL, id, unsigned int)
pub fn stub_0xed4e80() -> ! {
    todo!("0xed4e80 -[TFEventManager _createNewQueueAtPath:itemCount:]")
}

#[doc(alias = "-[TFEventManager _getOldQueuePaths]")]
#[doc(alias = "-[TFEventManager _getOldQueuePaths]")]
// 0xed50d8 — -[TFEventManager _getOldQueuePaths]
// type: void __cdecl(TFEventManager *self, SEL)
pub fn stub_0xed50d8() -> ! {
    todo!("0xed50d8 -[TFEventManager _getOldQueuePaths]")
}

#[doc(alias = "___35-[TFEventManager _getOldQueuePaths]_block_invoke")]
#[doc(alias = "___35-[TFEventManager _getOldQueuePaths]_block_invoke")]
// 0xed5300 — ___35-[TFEventManager _getOldQueuePaths]_block_invoke
// type: int __fastcall(int, id)
pub fn stub_0xed5300() -> ! {
    todo!("0xed5300 ___35-[TFEventManager _getOldQueuePaths]_block_invoke")
}

#[doc(alias = "___copy_helper_block_137")]
#[doc(alias = "___copy_helper_block_137")]
// 0xed53d0 — ___copy_helper_block_137
pub fn stub_0xed53d0() -> ! {
    todo!("0xed53d0 ___copy_helper_block_137")
}

#[doc(alias = "___destroy_helper_block_138")]
#[doc(alias = "___destroy_helper_block_138")]
// 0xed53dc — ___destroy_helper_block_138
pub fn stub_0xed53dc() -> ! {
    todo!("0xed53dc ___destroy_helper_block_138")
}

#[doc(alias = "-[TFEventManager url]")]
#[doc(alias = "-[TFEventManager url]")]
// 0xed5494 — -[TFEventManager url]
// type: NSURL *__cdecl(TFEventManager *self, SEL)
pub fn stub_0xed5494() -> ! {
    todo!("0xed5494 -[TFEventManager url]")
}

#[doc(alias = "-[TFEventManager setUrl:]")]
#[doc(alias = "-[TFEventManager setUrl:]")]
// 0xed54a4 — -[TFEventManager setUrl:]
// type: void __cdecl(TFEventManager *self, SEL, id)
pub fn stub_0xed54a4() -> ! {
    todo!("0xed54a4 -[TFEventManager setUrl:]")
}

#[doc(alias = "-[TFEventManager .cxx_destruct]")]
#[doc(alias = "-[TFEventManager .cxx_destruct]")]
// 0xed54c8 — -[TFEventManager .cxx_destruct]
// type: void __cdecl(TFEventManager *self, SEL)
pub fn stub_0xed54c8() -> ! {
    todo!("0xed54c8 -[TFEventManager .cxx_destruct]")
}

#[doc(alias = "+[TFMemoryMonitor startMemoryMonitor]")]
#[doc(alias = "+[TFMemoryMonitor startMemoryMonitor]")]
// 0xed5570 — +[TFMemoryMonitor startMemoryMonitor]
// type: void __cdecl(id, SEL)
pub fn stub_0xed5570() -> ! {
    todo!("0xed5570 +[TFMemoryMonitor startMemoryMonitor]")
}

#[doc(alias = "_tf_get_memory_usage")]
#[doc(alias = "_tf_get_memory_usage")]
// 0xed5574 — _tf_get_memory_usage
// type: int __fastcall(task_info_t task_info_out)
pub fn stub_0xed5574() -> ! {
    todo!("0xed5574 _tf_get_memory_usage")
}

#[doc(alias = "+[TFMemoryMonitor recordUsage]")]
#[doc(alias = "+[TFMemoryMonitor recordUsage]")]
// 0xed55c4 — +[TFMemoryMonitor recordUsage]
// type: void __cdecl(id, SEL)
pub fn stub_0xed55c4() -> ! {
    todo!("0xed55c4 +[TFMemoryMonitor recordUsage]")
}

#[doc(alias = "___copy_helper_block__33")]
#[doc(alias = "___copy_helper_block__33")]
// 0xed5f78 — ___copy_helper_block__33
pub fn stub_0xed5f78() -> ! {
    todo!("0xed5f78 ___copy_helper_block__33")
}

#[doc(alias = "___destroy_helper_block__33")]
#[doc(alias = "___destroy_helper_block__33")]
// 0xed5f94 — ___destroy_helper_block__33
pub fn stub_0xed5f94() -> ! {
    todo!("0xed5f94 ___destroy_helper_block__33")
}

#[doc(alias = "___copy_helper_block_119")]
#[doc(alias = "___copy_helper_block_119")]
// 0xed61d4 — ___copy_helper_block_119
pub fn stub_0xed61d4() -> ! {
    todo!("0xed61d4 ___copy_helper_block_119")
}

#[doc(alias = "___destroy_helper_block_120")]
#[doc(alias = "___destroy_helper_block_120")]
// 0xed61e0 — ___destroy_helper_block_120
pub fn stub_0xed61e0() -> ! {
    todo!("0xed61e0 ___destroy_helper_block_120")
}

#[doc(alias = "___copy_helper_block_127")]
#[doc(alias = "___copy_helper_block_127")]
// 0xed6308 — ___copy_helper_block_127
pub fn stub_0xed6308() -> ! {
    todo!("0xed6308 ___copy_helper_block_127")
}

#[doc(alias = "___destroy_helper_block_128")]
#[doc(alias = "___destroy_helper_block_128")]
// 0xed6314 — ___destroy_helper_block_128
pub fn stub_0xed6314() -> ! {
    todo!("0xed6314 ___destroy_helper_block_128")
}

#[doc(alias = "___copy_helper_block_135")]
#[doc(alias = "___copy_helper_block_135")]
// 0xed64c8 — ___copy_helper_block_135
pub fn stub_0xed64c8() -> ! {
    todo!("0xed64c8 ___copy_helper_block_135")
}

#[doc(alias = "___destroy_helper_block_136")]
#[doc(alias = "___destroy_helper_block_136")]
// 0xed64d8 — ___destroy_helper_block_136
pub fn stub_0xed64d8() -> ! {
    todo!("0xed64d8 ___destroy_helper_block_136")
}

#[doc(alias = "___copy_helper_block_139_0")]
#[doc(alias = "___copy_helper_block_139_0")]
// 0xed64e8 — ___copy_helper_block_139_0
pub fn stub_0xed64e8() -> ! {
    todo!("0xed64e8 ___copy_helper_block_139_0")
}

#[doc(alias = "___destroy_helper_block_140_0")]
#[doc(alias = "___destroy_helper_block_140_0")]
// 0xed6504 — ___destroy_helper_block_140_0
pub fn stub_0xed6504() -> ! {
    todo!("0xed6504 ___destroy_helper_block_140_0")
}

#[doc(alias = "___copy_helper_block_147")]
#[doc(alias = "___copy_helper_block_147")]
// 0xed6598 — ___copy_helper_block_147
pub fn stub_0xed6598() -> ! {
    todo!("0xed6598 ___copy_helper_block_147")
}

#[doc(alias = "___destroy_helper_block_148")]
#[doc(alias = "___destroy_helper_block_148")]
// 0xed65a4 — ___destroy_helper_block_148
pub fn stub_0xed65a4() -> ! {
    todo!("0xed65a4 ___destroy_helper_block_148")
}

#[doc(alias = "+[TFReachability initialize]")]
#[doc(alias = "+[TFReachability initialize]")]
// 0xed66a8 — +[TFReachability initialize]
// type: void __cdecl(id, SEL)
pub fn stub_0xed66a8() -> ! {
    todo!("0xed66a8 +[TFReachability initialize]")
}

#[doc(alias = "___28+[TFReachability initialize]_block_invoke")]
#[doc(alias = "___28+[TFReachability initialize]_block_invoke")]
// 0xed66d4 — ___28+[TFReachability initialize]_block_invoke
// type: void __cdecl(id)
pub fn stub_0xed66d4() -> ! {
    todo!("0xed66d4 ___28+[TFReachability initialize]_block_invoke")
}

#[doc(alias = "+[TFReachability reachabilityWithHostName:]")]
#[doc(alias = "+[TFReachability reachabilityWithHostName:]")]
// 0xed6814 — +[TFReachability reachabilityWithHostName:]
// type: TFReachability *__cdecl(id, SEL, id)
pub fn stub_0xed6814() -> ! {
    todo!("0xed6814 +[TFReachability reachabilityWithHostName:]")
}

#[doc(alias = "+[TFReachability reachabilityWithAddress:localWifiRef:]")]
#[doc(alias = "+[TFReachability reachabilityWithAddress:localWifiRef:]")]
// 0xed689c — +[TFReachability reachabilityWithAddress:localWifiRef:]
// type: TFReachability *__cdecl(id, SEL, const sockaddr_in *, char)
pub fn stub_0xed689c() -> ! {
    todo!("0xed689c +[TFReachability reachabilityWithAddress:localWifiRef:]")
}

#[doc(alias = "+[TFReachability reachabilityForInternetConnection]")]
#[doc(alias = "+[TFReachability reachabilityForInternetConnection]")]
// 0xed68fc — +[TFReachability reachabilityForInternetConnection]
// type: id __cdecl(id, SEL)
pub fn stub_0xed68fc() -> ! {
    todo!("0xed68fc +[TFReachability reachabilityForInternetConnection]")
}

#[doc(alias = "+[TFReachability reachabilityForLocalWiFi]")]
#[doc(alias = "+[TFReachability reachabilityForLocalWiFi]")]
// 0xed694c — +[TFReachability reachabilityForLocalWiFi]
// type: id __cdecl(id, SEL)
pub fn stub_0xed694c() -> ! {
    todo!("0xed694c +[TFReachability reachabilityForLocalWiFi]")
}

#[doc(alias = "-[TFReachability dealloc]")]
#[doc(alias = "-[TFReachability dealloc]")]
// 0xed6a24 — -[TFReachability dealloc]
// type: void __cdecl(TFReachability *self, SEL)
pub fn stub_0xed6a24() -> ! {
    todo!("0xed6a24 -[TFReachability dealloc]")
}

#[doc(alias = "-[TFReachability setReachabilityStatusUpdateBlock:]")]
#[doc(alias = "-[TFReachability setReachabilityStatusUpdateBlock:]")]
// 0xed6a80 — -[TFReachability setReachabilityStatusUpdateBlock:]
// type: void __cdecl(TFReachability *self, SEL, id)
pub fn stub_0xed6a80() -> ! {
    todo!("0xed6a80 -[TFReachability setReachabilityStatusUpdateBlock:]")
}

#[doc(alias = "_TFReachabilityCallback")]
#[doc(alias = "_TFReachabilityCallback")]
// 0xed6b38 — _TFReachabilityCallback
// type: int __fastcall(int, int, id)
pub fn stub_0xed6b38() -> ! {
    todo!("0xed6b38 _TFReachabilityCallback")
}

#[doc(alias = "-[TFReachability status]")]
#[doc(alias = "-[TFReachability status]")]
// 0xed6bac — -[TFReachability status]
// type: int __cdecl(TFReachability *self, SEL)
pub fn stub_0xed6bac() -> ! {
    todo!("0xed6bac -[TFReachability status]")
}

#[doc(alias = "-[TFReachability statusForFlags:]")]
#[doc(alias = "-[TFReachability statusForFlags:]")]
// 0xed6bf0 — -[TFReachability statusForFlags:]
// type: int __cdecl(TFReachability *self, SEL, unsigned int)
pub fn stub_0xed6bf0() -> ! {
    todo!("0xed6bf0 -[TFReachability statusForFlags:]")
}

#[doc(alias = "-[TFReachability reachabilityStatusUpdateBlock]")]
#[doc(alias = "-[TFReachability reachabilityStatusUpdateBlock]")]
// 0xed6c3c — -[TFReachability reachabilityStatusUpdateBlock]
// type: id __cdecl(TFReachability *self, SEL)
pub fn stub_0xed6c3c() -> ! {
    todo!("0xed6c3c -[TFReachability reachabilityStatusUpdateBlock]")
}

#[doc(alias = "-[TFReachability .cxx_destruct]")]
#[doc(alias = "-[TFReachability .cxx_destruct]")]
// 0xed6c54 — -[TFReachability .cxx_destruct]
// type: void __cdecl(TFReachability *self, SEL)
pub fn stub_0xed6c54() -> ! {
    todo!("0xed6c54 -[TFReachability .cxx_destruct]")
}

#[doc(alias = "+[TFURLConnectionOperation initialize]")]
#[doc(alias = "+[TFURLConnectionOperation initialize]")]
// 0xed6d50 — +[TFURLConnectionOperation initialize]
// type: void __cdecl(id, SEL)
pub fn stub_0xed6d50() -> ! {
    todo!("0xed6d50 +[TFURLConnectionOperation initialize]")
}

#[doc(alias = "___38+[TFURLConnectionOperation initialize]_block_invoke")]
#[doc(alias = "___38+[TFURLConnectionOperation initialize]_block_invoke")]
// 0xed6db4 — ___38+[TFURLConnectionOperation initialize]_block_invoke
pub fn stub_0xed6db4() -> ! {
    todo!("0xed6db4 ___38+[TFURLConnectionOperation initialize]_block_invoke")
}

#[doc(alias = "___copy_helper_block__34")]
#[doc(alias = "___copy_helper_block__34")]
// 0xed6e40 — ___copy_helper_block__34
pub fn stub_0xed6e40() -> ! {
    todo!("0xed6e40 ___copy_helper_block__34")
}

#[doc(alias = "___destroy_helper_block__34")]
#[doc(alias = "___destroy_helper_block__34")]
// 0xed6e44 — ___destroy_helper_block__34
pub fn stub_0xed6e44() -> ! {
    todo!("0xed6e44 ___destroy_helper_block__34")
}

#[doc(alias = "+[TFURLConnectionOperation automaticallyNotifiesObserversForKey:]")]
#[doc(alias = "+[TFURLConnectionOperation automaticallyNotifiesObserversForKey:]")]
// 0xed6e60 — +[TFURLConnectionOperation automaticallyNotifiesObserversForKey:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xed6e60() -> ! {
    todo!("0xed6e60 +[TFURLConnectionOperation automaticallyNotifiesObserversForKey:]")
}

#[doc(alias = "-[TFURLConnectionOperation initWithRequest:completionHandler:]")]
#[doc(alias = "-[TFURLConnectionOperation initWithRequest:completionHandler:]")]
// 0xed6f34 — -[TFURLConnectionOperation initWithRequest:completionHandler:]
// type: TFURLConnectionOperation *__cdecl(TFURLConnectionOperation *self, SEL, id, id)
pub fn stub_0xed6f34() -> ! {
    todo!("0xed6f34 -[TFURLConnectionOperation initWithRequest:completionHandler:]")
}

#[doc(alias = "-[TFURLConnectionOperation initWithRequest:streamPath:completionHandler:]")]
#[doc(alias = "-[TFURLConnectionOperation initWithRequest:streamPath:completionHandler:]")]
// 0xed6f78 — -[TFURLConnectionOperation initWithRequest:streamPath:completionHandler:]
// type: TFURLConnectionOperation *__cdecl(TFURLConnectionOperation *self, SEL, id, id, id)
pub fn stub_0xed6f78() -> ! {
    todo!("0xed6f78 -[TFURLConnectionOperation initWithRequest:streamPath:completionHandler:]")
}

#[doc(alias = "-[TFURLConnectionOperation _createConnection]")]
#[doc(alias = "-[TFURLConnectionOperation _createConnection]")]
// 0xed70f4 — -[TFURLConnectionOperation _createConnection]
// type: id __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_0xed70f4() -> ! {
    todo!("0xed70f4 -[TFURLConnectionOperation _createConnection]")
}

#[doc(alias = "-[TFURLConnectionOperation start]")]
#[doc(alias = "-[TFURLConnectionOperation start]")]
// 0xed7144 — -[TFURLConnectionOperation start]
// type: void __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_0xed7144() -> ! {
    todo!("0xed7144 -[TFURLConnectionOperation start]")
}

#[doc(alias = "-[TFURLConnectionOperation cancel]")]
#[doc(alias = "-[TFURLConnectionOperation cancel]")]
// 0xed7184 — -[TFURLConnectionOperation cancel]
// type: void __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_0xed7184() -> ! {
    todo!("0xed7184 -[TFURLConnectionOperation cancel]")
}

#[doc(alias = "-[TFURLConnectionOperation isConcurrent]")]
#[doc(alias = "-[TFURLConnectionOperation isConcurrent]")]
// 0xed71c4 — -[TFURLConnectionOperation isConcurrent]
// type: char __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_0xed71c4() -> ! {
    todo!("0xed71c4 -[TFURLConnectionOperation isConcurrent]")
}

#[doc(alias = "-[TFURLConnectionOperation _start]")]
#[doc(alias = "-[TFURLConnectionOperation _start]")]
// 0xed71c8 — -[TFURLConnectionOperation _start]
// type: void __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_0xed71c8() -> ! {
    todo!("0xed71c8 -[TFURLConnectionOperation _start]")
}

#[doc(alias = "___34-[TFURLConnectionOperation _start]_block_invoke")]
#[doc(alias = "___34-[TFURLConnectionOperation _start]_block_invoke")]
// 0xed7478 — ___34-[TFURLConnectionOperation _start]_block_invoke
pub fn stub_0xed7478() -> ! {
    todo!("0xed7478 ___34-[TFURLConnectionOperation _start]_block_invoke")
}

#[doc(alias = "___copy_helper_block_62")]
#[doc(alias = "___copy_helper_block_62")]
// 0xed74c4 — ___copy_helper_block_62
pub fn stub_0xed74c4() -> ! {
    todo!("0xed74c4 ___copy_helper_block_62")
}

#[doc(alias = "___destroy_helper_block_63")]
#[doc(alias = "___destroy_helper_block_63")]
// 0xed74d4 — ___destroy_helper_block_63
pub fn stub_0xed74d4() -> ! {
    todo!("0xed74d4 ___destroy_helper_block_63")
}

#[doc(alias = "-[TFURLConnectionOperation _cancel]")]
#[doc(alias = "-[TFURLConnectionOperation _cancel]")]
// 0xed74e4 — -[TFURLConnectionOperation _cancel]
// type: void __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_0xed74e4() -> ! {
    todo!("0xed74e4 -[TFURLConnectionOperation _cancel]")
}

#[doc(alias = "-[TFURLConnectionOperation _timeout]")]
#[doc(alias = "-[TFURLConnectionOperation _timeout]")]
// 0xed75f4 — -[TFURLConnectionOperation _timeout]
// type: void __cdecl(TFURLConnectionOperation *self, SEL)
pub fn stub_0xed75f4() -> ! {
    todo!("0xed75f4 -[TFURLConnectionOperation _timeout]")
}
