//! core shard mj — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 33887, 3173 uncovered before -> 3073 after, batch 0xf0ec40..0xf10e60).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "-[FlurryDataSenderIndex removeBlockInfoWithIdentifier:forDataKey:]")]
// 0xf0ec40 — -[FlurryDataSenderIndex removeBlockInfoWithIdentifier:forDataKey:]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL, id, id)
pub fn stub_0xf0ec40() -> ! { todo!("0xf0ec40 -[FlurryDataSenderIndex removeBlockInfoWithIdentifier:forDataKey:]") }

#[doc(alias = "-[FlurryDataSenderIndex setupForCurrentDataKey:]")]
// 0xf0ed80 — -[FlurryDataSenderIndex setupForCurrentDataKey:]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL, id)
pub fn stub_0xf0ed80() -> ! { todo!("0xf0ed80 -[FlurryDataSenderIndex setupForCurrentDataKey:]") }

#[doc(alias = "-[FlurryDataSenderIndex discardOutdatedBlocksForDataKey:]")]
// 0xf0efa0 — -[FlurryDataSenderIndex discardOutdatedBlocksForDataKey:]
// type: char __cdecl(FlurryDataSenderIndex *self, SEL, id)
pub fn stub_0xf0efa0() -> ! { todo!("0xf0efa0 -[FlurryDataSenderIndex discardOutdatedBlocksForDataKey:]") }

#[doc(alias = "-[FlurryDataSenderIndex notSentBlocksForDataKey:]")]
// 0xf0f0e8 — -[FlurryDataSenderIndex notSentBlocksForDataKey:]
// type: id __cdecl(FlurryDataSenderIndex *self, SEL, id)
pub fn stub_0xf0f0e8() -> ! { todo!("0xf0f0e8 -[FlurryDataSenderIndex notSentBlocksForDataKey:]") }

#[doc(alias = "-[FlurryDataSenderIndex indexMap]")]
// 0xf0f18c — -[FlurryDataSenderIndex indexMap]
// type: NSMutableDictionary *__cdecl(FlurryDataSenderIndex *self, SEL)
pub fn stub_0xf0f18c() -> ! { todo!("0xf0f18c -[FlurryDataSenderIndex indexMap]") }

#[doc(alias = "-[FlurryDataSenderIndex setIndexMap:]")]
// 0xf0f19c — -[FlurryDataSenderIndex setIndexMap:]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL, id)
pub fn stub_0xf0f19c() -> ! { todo!("0xf0f19c -[FlurryDataSenderIndex setIndexMap:]") }

#[doc(alias = "-[FlurryDataSenderIndex sentReportSuccessfully]")]
// 0xf0f1c0 — -[FlurryDataSenderIndex sentReportSuccessfully]
// type: char __cdecl(FlurryDataSenderIndex *self, SEL)
pub fn stub_0xf0f1c0() -> ! { todo!("0xf0f1c0 -[FlurryDataSenderIndex sentReportSuccessfully]") }

#[doc(alias = "-[FlurryDataSenderIndex setSentReportSuccessfully:]")]
// 0xf0f1d0 — -[FlurryDataSenderIndex setSentReportSuccessfully:]
// type: void __cdecl(FlurryDataSenderIndex *self, SEL, char)
pub fn stub_0xf0f1d0() -> ! { todo!("0xf0f1d0 -[FlurryDataSenderIndex setSentReportSuccessfully:]") }

#[doc(alias = "+[FlurryImpl instance]")]
// 0xf0f1e0 — +[FlurryImpl instance]
// type: id __cdecl(id, SEL)
pub fn stub_0xf0f1e0() -> ! { todo!("0xf0f1e0 +[FlurryImpl instance]") }

#[doc(alias = "-[FlurryImpl init]")]
// 0xf0f2b8 — -[FlurryImpl init]
// type: FlurryImpl *__cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf0f2b8() -> ! { todo!("0xf0f2b8 -[FlurryImpl init]") }

#[doc(alias = "-[FlurryImpl resetStartDate]")]
// 0xf0f3dc — -[FlurryImpl resetStartDate]
// type: void __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf0f3dc() -> ! { todo!("0xf0f3dc -[FlurryImpl resetStartDate]") }

#[doc(alias = "-[FlurryImpl setupForApiKey:]")]
// 0xf0f414 — -[FlurryImpl setupForApiKey:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf0f414() -> ! { todo!("0xf0f414 -[FlurryImpl setupForApiKey:]") }

#[doc(alias = "___29-[FlurryImpl setupForApiKey:]_block_invoke_0")]
// 0xf0f564 — ___29-[FlurryImpl setupForApiKey:]_block_invoke_0
pub fn stub_0xf0f564() -> ! { todo!("0xf0f564 ___29-[FlurryImpl setupForApiKey:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block__41")]
// 0xf0f63c — ___copy_helper_block__41
pub fn stub_0xf0f63c() -> ! { todo!("0xf0f63c ___copy_helper_block__41") }

#[doc(alias = "___destroy_helper_block__41")]
// 0xf0f660 — ___destroy_helper_block__41
pub fn stub_0xf0f660() -> ! { todo!("0xf0f660 ___destroy_helper_block__41") }

#[doc(alias = "-[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]")]
// 0xf0f678 — -[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]
// type: void __cdecl(FlurryImpl *self, SEL, double, double, float, float)
pub fn stub_0xf0f678() -> ! { todo!("0xf0f678 -[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]") }

#[doc(alias = "___72-[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]_block_invoke_0")]
// 0xf0f768 — ___72-[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]_block_invoke_0
pub fn stub_0xf0f768() -> ! { todo!("0xf0f768 ___72-[FlurryImpl setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_45")]
// 0xf0f7b8 — ___copy_helper_block_45
pub fn stub_0xf0f7b8() -> ! { todo!("0xf0f7b8 ___copy_helper_block_45") }

#[doc(alias = "___destroy_helper_block_46")]
// 0xf0f7c8 — ___destroy_helper_block_46
pub fn stub_0xf0f7c8() -> ! { todo!("0xf0f7c8 ___destroy_helper_block_46") }

#[doc(alias = "-[FlurryImpl setEventLoggingEnabled:]")]
// 0xf0f7d8 — -[FlurryImpl setEventLoggingEnabled:]
// type: void __cdecl(FlurryImpl *self, SEL, char)
pub fn stub_0xf0f7d8() -> ! { todo!("0xf0f7d8 -[FlurryImpl setEventLoggingEnabled:]") }

#[doc(alias = "___37-[FlurryImpl setEventLoggingEnabled:]_block_invoke_0")]
// 0xf0f880 — ___37-[FlurryImpl setEventLoggingEnabled:]_block_invoke_0
pub fn stub_0xf0f880() -> ! { todo!("0xf0f880 ___37-[FlurryImpl setEventLoggingEnabled:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_50_0")]
// 0xf0f8b0 — ___copy_helper_block_50_0
pub fn stub_0xf0f8b0() -> ! { todo!("0xf0f8b0 ___copy_helper_block_50_0") }

#[doc(alias = "___destroy_helper_block_51_0")]
// 0xf0f8c0 — ___destroy_helper_block_51_0
pub fn stub_0xf0f8c0() -> ! { todo!("0xf0f8c0 ___destroy_helper_block_51_0") }

#[doc(alias = "-[FlurryImpl setUserID:]")]
// 0xf0f8d0 — -[FlurryImpl setUserID:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf0f8d0() -> ! { todo!("0xf0f8d0 -[FlurryImpl setUserID:]") }

#[doc(alias = "___24-[FlurryImpl setUserID:]_block_invoke_0")]
// 0xf0f974 — ___24-[FlurryImpl setUserID:]_block_invoke_0
pub fn stub_0xf0f974() -> ! { todo!("0xf0f974 ___24-[FlurryImpl setUserID:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_55_0")]
// 0xf0f9a0 — ___copy_helper_block_55_0
pub fn stub_0xf0f9a0() -> ! { todo!("0xf0f9a0 ___copy_helper_block_55_0") }

#[doc(alias = "___destroy_helper_block_56_0")]
// 0xf0f9c4 — ___destroy_helper_block_56_0
pub fn stub_0xf0f9c4() -> ! { todo!("0xf0f9c4 ___destroy_helper_block_56_0") }

#[doc(alias = "-[FlurryImpl setSessionReportsOnCloseEnabled:]")]
// 0xf0f9dc — -[FlurryImpl setSessionReportsOnCloseEnabled:]
// type: void __cdecl(FlurryImpl *self, SEL, char)
pub fn stub_0xf0f9dc() -> ! { todo!("0xf0f9dc -[FlurryImpl setSessionReportsOnCloseEnabled:]") }

#[doc(alias = "___46-[FlurryImpl setSessionReportsOnCloseEnabled:]_block_invoke_0")]
// 0xf0fa84 — ___46-[FlurryImpl setSessionReportsOnCloseEnabled:]_block_invoke_0
pub fn stub_0xf0fa84() -> ! { todo!("0xf0fa84 ___46-[FlurryImpl setSessionReportsOnCloseEnabled:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_60")]
// 0xf0fab4 — ___copy_helper_block_60
pub fn stub_0xf0fab4() -> ! { todo!("0xf0fab4 ___copy_helper_block_60") }

#[doc(alias = "___destroy_helper_block_61")]
// 0xf0fac4 — ___destroy_helper_block_61
pub fn stub_0xf0fac4() -> ! { todo!("0xf0fac4 ___destroy_helper_block_61") }

#[doc(alias = "-[FlurryImpl setSessionReportsOnPauseEnabled:]")]
// 0xf0fad4 — -[FlurryImpl setSessionReportsOnPauseEnabled:]
// type: void __cdecl(FlurryImpl *self, SEL, char)
pub fn stub_0xf0fad4() -> ! { todo!("0xf0fad4 -[FlurryImpl setSessionReportsOnPauseEnabled:]") }

#[doc(alias = "___46-[FlurryImpl setSessionReportsOnPauseEnabled:]_block_invoke_0")]
// 0xf0fb7c — ___46-[FlurryImpl setSessionReportsOnPauseEnabled:]_block_invoke_0
pub fn stub_0xf0fb7c() -> ! { todo!("0xf0fb7c ___46-[FlurryImpl setSessionReportsOnPauseEnabled:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_65_0")]
// 0xf0fbac — ___copy_helper_block_65_0
pub fn stub_0xf0fbac() -> ! { todo!("0xf0fbac ___copy_helper_block_65_0") }

#[doc(alias = "___destroy_helper_block_66_0")]
// 0xf0fbbc — ___destroy_helper_block_66_0
pub fn stub_0xf0fbbc() -> ! { todo!("0xf0fbbc ___destroy_helper_block_66_0") }

#[doc(alias = "-[FlurryImpl setCrashReportingEnabled:]")]
// 0xf0fbcc — -[FlurryImpl setCrashReportingEnabled:]
// type: void __cdecl(FlurryImpl *self, SEL, char)
pub fn stub_0xf0fbcc() -> ! { todo!("0xf0fbcc -[FlurryImpl setCrashReportingEnabled:]") }

#[doc(alias = "___39-[FlurryImpl setCrashReportingEnabled:]_block_invoke_0")]
// 0xf0fc74 — ___39-[FlurryImpl setCrashReportingEnabled:]_block_invoke_0
pub fn stub_0xf0fc74() -> ! { todo!("0xf0fc74 ___39-[FlurryImpl setCrashReportingEnabled:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_70_0")]
// 0xf0fca4 — ___copy_helper_block_70_0
pub fn stub_0xf0fca4() -> ! { todo!("0xf0fca4 ___copy_helper_block_70_0") }

#[doc(alias = "___destroy_helper_block_71_0")]
// 0xf0fcb4 — ___destroy_helper_block_71_0
pub fn stub_0xf0fcb4() -> ! { todo!("0xf0fcb4 ___destroy_helper_block_71_0") }

#[doc(alias = "-[FlurryImpl maybeIncrementPageView]")]
// 0xf0fcc4 — -[FlurryImpl maybeIncrementPageView]
// type: void __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf0fcc4() -> ! { todo!("0xf0fcc4 -[FlurryImpl maybeIncrementPageView]") }

#[doc(alias = "___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0")]
// 0xf0fd64 — ___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0
pub fn stub_0xf0fd64() -> ! { todo!("0xf0fd64 ___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_75")]
// 0xf0fd8c — ___copy_helper_block_75
pub fn stub_0xf0fd8c() -> ! { todo!("0xf0fd8c ___copy_helper_block_75") }

#[doc(alias = "___destroy_helper_block_76")]
// 0xf0fd9c — ___destroy_helper_block_76
pub fn stub_0xf0fd9c() -> ! { todo!("0xf0fd9c ___destroy_helper_block_76") }

#[doc(alias = "-[FlurryImpl setGenderAsString:]")]
// 0xf0fdac — -[FlurryImpl setGenderAsString:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf0fdac() -> ! { todo!("0xf0fdac -[FlurryImpl setGenderAsString:]") }

#[doc(alias = "___32-[FlurryImpl setGenderAsString:]_block_invoke_0")]
// 0xf0fe50 — ___32-[FlurryImpl setGenderAsString:]_block_invoke_0
pub fn stub_0xf0fe50() -> ! { todo!("0xf0fe50 ___32-[FlurryImpl setGenderAsString:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_80_0")]
// 0xf0fe7c — ___copy_helper_block_80_0
pub fn stub_0xf0fe7c() -> ! { todo!("0xf0fe7c ___copy_helper_block_80_0") }

#[doc(alias = "___destroy_helper_block_81_0")]
// 0xf0fea0 — ___destroy_helper_block_81_0
pub fn stub_0xf0fea0() -> ! { todo!("0xf0fea0 ___destroy_helper_block_81_0") }

#[doc(alias = "-[FlurryImpl setAgeInYears:]")]
// 0xf0feb8 — -[FlurryImpl setAgeInYears:]
// type: void __cdecl(FlurryImpl *self, SEL, int)
pub fn stub_0xf0feb8() -> ! { todo!("0xf0feb8 -[FlurryImpl setAgeInYears:]") }

#[doc(alias = "___28-[FlurryImpl setAgeInYears:]_block_invoke_0")]
// 0xf0ff5c — ___28-[FlurryImpl setAgeInYears:]_block_invoke_0
pub fn stub_0xf0ff5c() -> ! { todo!("0xf0ff5c ___28-[FlurryImpl setAgeInYears:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_85")]
// 0xf0ff88 — ___copy_helper_block_85
pub fn stub_0xf0ff88() -> ! { todo!("0xf0ff88 ___copy_helper_block_85") }

#[doc(alias = "___destroy_helper_block_86")]
// 0xf0ff98 — ___destroy_helper_block_86
pub fn stub_0xf0ff98() -> ! { todo!("0xf0ff98 ___destroy_helper_block_86") }

#[doc(alias = "-[FlurryImpl setPushToken:]")]
// 0xf0ffa8 — -[FlurryImpl setPushToken:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf0ffa8() -> ! { todo!("0xf0ffa8 -[FlurryImpl setPushToken:]") }

#[doc(alias = "___27-[FlurryImpl setPushToken:]_block_invoke_0")]
// 0xf1004c — ___27-[FlurryImpl setPushToken:]_block_invoke_0
pub fn stub_0xf1004c() -> ! { todo!("0xf1004c ___27-[FlurryImpl setPushToken:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_90_0")]
// 0xf10078 — ___copy_helper_block_90_0
pub fn stub_0xf10078() -> ! { todo!("0xf10078 ___copy_helper_block_90_0") }

#[doc(alias = "___destroy_helper_block_91_0")]
// 0xf1009c — ___destroy_helper_block_91_0
pub fn stub_0xf1009c() -> ! { todo!("0xf1009c ___destroy_helper_block_91_0") }

#[doc(alias = "-[FlurryImpl recordEvent:withParameters:]")]
// 0xf100b4 — -[FlurryImpl recordEvent:withParameters:]
// type: void __cdecl(FlurryImpl *self, SEL, id, id)
pub fn stub_0xf100b4() -> ! { todo!("0xf100b4 -[FlurryImpl recordEvent:withParameters:]") }

#[doc(alias = "___41-[FlurryImpl recordEvent:withParameters:]_block_invoke_0")]
// 0xf10160 — ___41-[FlurryImpl recordEvent:withParameters:]_block_invoke_0
pub fn stub_0xf10160() -> ! { todo!("0xf10160 ___41-[FlurryImpl recordEvent:withParameters:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_95")]
// 0xf10190 — ___copy_helper_block_95
pub fn stub_0xf10190() -> ! { todo!("0xf10190 ___copy_helper_block_95") }

#[doc(alias = "___destroy_helper_block_96")]
// 0xf101c0 — ___destroy_helper_block_96
pub fn stub_0xf101c0() -> ! { todo!("0xf101c0 ___destroy_helper_block_96") }

#[doc(alias = "-[FlurryImpl recordEvent:withParameters:timed:]")]
// 0xf101e0 — -[FlurryImpl recordEvent:withParameters:timed:]
// type: void __cdecl(FlurryImpl *self, SEL, id, id, char)
pub fn stub_0xf101e0() -> ! { todo!("0xf101e0 -[FlurryImpl recordEvent:withParameters:timed:]") }

#[doc(alias = "___47-[FlurryImpl recordEvent:withParameters:timed:]_block_invoke_0")]
// 0xf102a0 — ___47-[FlurryImpl recordEvent:withParameters:timed:]_block_invoke_0
pub fn stub_0xf102a0() -> ! { todo!("0xf102a0 ___47-[FlurryImpl recordEvent:withParameters:timed:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_100_0")]
// 0xf102d8 — ___copy_helper_block_100_0
pub fn stub_0xf102d8() -> ! { todo!("0xf102d8 ___copy_helper_block_100_0") }

#[doc(alias = "___destroy_helper_block_101_0")]
// 0xf10308 — ___destroy_helper_block_101_0
pub fn stub_0xf10308() -> ! { todo!("0xf10308 ___destroy_helper_block_101_0") }

#[doc(alias = "-[FlurryImpl endTimedEvent:withParameters:]")]
// 0xf10328 — -[FlurryImpl endTimedEvent:withParameters:]
// type: void __cdecl(FlurryImpl *self, SEL, id, id)
pub fn stub_0xf10328() -> ! { todo!("0xf10328 -[FlurryImpl endTimedEvent:withParameters:]") }

#[doc(alias = "___43-[FlurryImpl endTimedEvent:withParameters:]_block_invoke_0")]
// 0xf103d4 — ___43-[FlurryImpl endTimedEvent:withParameters:]_block_invoke_0
pub fn stub_0xf103d4() -> ! { todo!("0xf103d4 ___43-[FlurryImpl endTimedEvent:withParameters:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_105")]
// 0xf10404 — ___copy_helper_block_105
pub fn stub_0xf10404() -> ! { todo!("0xf10404 ___copy_helper_block_105") }

#[doc(alias = "___destroy_helper_block_106")]
// 0xf10434 — ___destroy_helper_block_106
pub fn stub_0xf10434() -> ! { todo!("0xf10434 ___destroy_helper_block_106") }

#[doc(alias = "-[FlurryImpl recordError:message:exception:liveReport:]")]
// 0xf10454 — -[FlurryImpl recordError:message:exception:liveReport:]
// type: void __cdecl(FlurryImpl *self, SEL, id, id, id, id)
pub fn stub_0xf10454() -> ! { todo!("0xf10454 -[FlurryImpl recordError:message:exception:liveReport:]") }

#[doc(alias = "___55-[FlurryImpl recordError:message:exception:liveReport:]_block_invoke_0")]
// 0xf10518 — ___55-[FlurryImpl recordError:message:exception:liveReport:]_block_invoke_0
pub fn stub_0xf10518() -> ! { todo!("0xf10518 ___55-[FlurryImpl recordError:message:exception:liveReport:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_110_0")]
// 0xf1055c — ___copy_helper_block_110_0
pub fn stub_0xf1055c() -> ! { todo!("0xf1055c ___copy_helper_block_110_0") }

#[doc(alias = "___destroy_helper_block_111_0")]
// 0xf105a4 — ___destroy_helper_block_111_0
pub fn stub_0xf105a4() -> ! { todo!("0xf105a4 ___destroy_helper_block_111_0") }

#[doc(alias = "-[FlurryImpl recordError:message:error:liveReport:]")]
// 0xf105d4 — -[FlurryImpl recordError:message:error:liveReport:]
// type: void __cdecl(FlurryImpl *self, SEL, id, id, id, id)
pub fn stub_0xf105d4() -> ! { todo!("0xf105d4 -[FlurryImpl recordError:message:error:liveReport:]") }

#[doc(alias = "___51-[FlurryImpl recordError:message:error:liveReport:]_block_invoke_0")]
// 0xf10698 — ___51-[FlurryImpl recordError:message:error:liveReport:]_block_invoke_0
pub fn stub_0xf10698() -> ! { todo!("0xf10698 ___51-[FlurryImpl recordError:message:error:liveReport:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_115")]
// 0xf106dc — ___copy_helper_block_115
pub fn stub_0xf106dc() -> ! { todo!("0xf106dc ___copy_helper_block_115") }

#[doc(alias = "___destroy_helper_block_116")]
// 0xf10724 — ___destroy_helper_block_116
pub fn stub_0xf10724() -> ! { todo!("0xf10724 ___destroy_helper_block_116") }

#[doc(alias = "-[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]")]
// 0xf10754 — -[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]
// type: void __cdecl(FlurryImpl *self, SEL, id, id, id, int, id)
pub fn stub_0xf10754() -> ! { todo!("0xf10754 -[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]") }

#[doc(alias = "___71-[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]_block_invoke_0")]
// 0xf1082c — ___71-[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]_block_invoke_0
pub fn stub_0xf1082c() -> ! { todo!("0xf1082c ___71-[FlurryImpl recordError:message:exceptionString:errorType:liveReport:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_120")]
// 0xf10874 — ___copy_helper_block_120
pub fn stub_0xf10874() -> ! { todo!("0xf10874 ___copy_helper_block_120") }

#[doc(alias = "___destroy_helper_block_121")]
// 0xf108bc — ___destroy_helper_block_121
pub fn stub_0xf108bc() -> ! { todo!("0xf108bc ___destroy_helper_block_121") }

#[doc(alias = "-[FlurryImpl recordPurchaseItem:]")]
// 0xf108ec — -[FlurryImpl recordPurchaseItem:]
// type: void __cdecl(FlurryImpl *self, SEL, id)
pub fn stub_0xf108ec() -> ! { todo!("0xf108ec -[FlurryImpl recordPurchaseItem:]") }

#[doc(alias = "___33-[FlurryImpl recordPurchaseItem:]_block_invoke_0")]
// 0xf10990 — ___33-[FlurryImpl recordPurchaseItem:]_block_invoke_0
pub fn stub_0xf10990() -> ! { todo!("0xf10990 ___33-[FlurryImpl recordPurchaseItem:]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_125_0")]
// 0xf109bc — ___copy_helper_block_125_0
pub fn stub_0xf109bc() -> ! { todo!("0xf109bc ___copy_helper_block_125_0") }

#[doc(alias = "___destroy_helper_block_126_0")]
// 0xf109e0 — ___destroy_helper_block_126_0
pub fn stub_0xf109e0() -> ! { todo!("0xf109e0 ___destroy_helper_block_126_0") }

#[doc(alias = "-[FlurryImpl pauseSession]")]
// 0xf109f8 — -[FlurryImpl pauseSession]
// type: void __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf109f8() -> ! { todo!("0xf109f8 -[FlurryImpl pauseSession]") }

#[doc(alias = "___26-[FlurryImpl pauseSession]_block_invoke_0")]
// 0xf10af0 — ___26-[FlurryImpl pauseSession]_block_invoke_0
// type: void __cdecl(id)
pub fn stub_0xf10af0() -> ! { todo!("0xf10af0 ___26-[FlurryImpl pauseSession]_block_invoke_0") }

#[doc(alias = "___26-[FlurryImpl pauseSession]_block_invoke_0136")]
// 0xf10b28 — ___26-[FlurryImpl pauseSession]_block_invoke_0136
pub fn stub_0xf10b28() -> ! { todo!("0xf10b28 ___26-[FlurryImpl pauseSession]_block_invoke_0136") }

#[doc(alias = "___copy_helper_block_141")]
// 0xf10b98 — ___copy_helper_block_141
pub fn stub_0xf10b98() -> ! { todo!("0xf10b98 ___copy_helper_block_141") }

#[doc(alias = "___destroy_helper_block_142")]
// 0xf10ba8 — ___destroy_helper_block_142
pub fn stub_0xf10ba8() -> ! { todo!("0xf10ba8 ___destroy_helper_block_142") }

#[doc(alias = "-[FlurryImpl markSessionAsResuming]")]
// 0xf10bb8 — -[FlurryImpl markSessionAsResuming]
// type: void __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf10bb8() -> ! { todo!("0xf10bb8 -[FlurryImpl markSessionAsResuming]") }

#[doc(alias = "___35-[FlurryImpl markSessionAsResuming]_block_invoke_0")]
// 0xf10c58 — ___35-[FlurryImpl markSessionAsResuming]_block_invoke_0
pub fn stub_0xf10c58() -> ! { todo!("0xf10c58 ___35-[FlurryImpl markSessionAsResuming]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_146")]
// 0xf10c80 — ___copy_helper_block_146
pub fn stub_0xf10c80() -> ! { todo!("0xf10c80 ___copy_helper_block_146") }

#[doc(alias = "___destroy_helper_block_147")]
// 0xf10c90 — ___destroy_helper_block_147
pub fn stub_0xf10c90() -> ! { todo!("0xf10c90 ___destroy_helper_block_147") }

#[doc(alias = "-[FlurryImpl resumeSession]")]
// 0xf10ca0 — -[FlurryImpl resumeSession]
// type: void __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf10ca0() -> ! { todo!("0xf10ca0 -[FlurryImpl resumeSession]") }

#[doc(alias = "___27-[FlurryImpl resumeSession]_block_invoke_0")]
// 0xf10d40 — ___27-[FlurryImpl resumeSession]_block_invoke_0
pub fn stub_0xf10d40() -> ! { todo!("0xf10d40 ___27-[FlurryImpl resumeSession]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_151")]
// 0xf10d68 — ___copy_helper_block_151
pub fn stub_0xf10d68() -> ! { todo!("0xf10d68 ___copy_helper_block_151") }

#[doc(alias = "___destroy_helper_block_152")]
// 0xf10d78 — ___destroy_helper_block_152
pub fn stub_0xf10d78() -> ! { todo!("0xf10d78 ___destroy_helper_block_152") }

#[doc(alias = "-[FlurryImpl endSession]")]
// 0xf10d88 — -[FlurryImpl endSession]
// type: void __cdecl(FlurryImpl *self, SEL)
pub fn stub_0xf10d88() -> ! { todo!("0xf10d88 -[FlurryImpl endSession]") }

#[doc(alias = "___24-[FlurryImpl endSession]_block_invoke_0")]
// 0xf10e28 — ___24-[FlurryImpl endSession]_block_invoke_0
pub fn stub_0xf10e28() -> ! { todo!("0xf10e28 ___24-[FlurryImpl endSession]_block_invoke_0") }

#[doc(alias = "___copy_helper_block_156")]
// 0xf10e50 — ___copy_helper_block_156
pub fn stub_0xf10e50() -> ! { todo!("0xf10e50 ___copy_helper_block_156") }

#[doc(alias = "___destroy_helper_block_157")]
// 0xf10e60 — ___destroy_helper_block_157
pub fn stub_0xf10e60() -> ! { todo!("0xf10e60 ___destroy_helper_block_157") }
