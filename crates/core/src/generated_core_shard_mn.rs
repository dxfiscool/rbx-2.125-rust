//! core shard mn — 50 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 50 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 2673 uncovered before -> 2623 after, batch 0xf1a660..0xf1c66c).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, unstable_name_collisions, clippy::all, unused_attributes)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[FlurryPLCrashReportExceptionInfo exceptionReason]")]
// 0xf1a660 — -[FlurryPLCrashReportExceptionInfo exceptionReason]
// type: NSString *__cdecl(FlurryPLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xf1a660() -> ! { todo!("0xf1a660 -[FlurryPLCrashReportExceptionInfo exceptionReason]") }

#[doc(alias = "-[FlurryPLCrashReportExceptionInfo stackFrames]")]
// 0xf1a670 — -[FlurryPLCrashReportExceptionInfo stackFrames]
// type: NSArray *__cdecl(FlurryPLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xf1a670() -> ! { todo!("0xf1a670 -[FlurryPLCrashReportExceptionInfo stackFrames]") }

#[doc(alias = "_plcrash_async_signal_sigcode_0")]
// 0xf1a680 — _plcrash_async_signal_sigcode_0
pub fn stub_0xf1a680() -> ! { todo!("0xf1a680 _plcrash_async_signal_sigcode_0") }

#[doc(alias = "_plcrash_async_signal_signame_0")]
// 0xf1a6c4 — _plcrash_async_signal_signame_0
pub fn stub_0xf1a6c4() -> ! { todo!("0xf1a6c4 _plcrash_async_signal_signame_0") }

#[doc(alias = "-[FlurryPLCrashReportSignalInfo initWithSignalName:code:address:]")]
// 0xf1a6ec — -[FlurryPLCrashReportSignalInfo initWithSignalName:code:address:]
// type: FlurryPLCrashReportSignalInfo *__cdecl(FlurryPLCrashReportSignalInfo *self, SEL, id, id, unsigned __int64)
pub fn stub_0xf1a6ec() -> ! { todo!("0xf1a6ec -[FlurryPLCrashReportSignalInfo initWithSignalName:code:address:]") }

#[doc(alias = "-[FlurryPLCrashReportSignalInfo dealloc]")]
// 0xf1a784 — -[FlurryPLCrashReportSignalInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a784() -> ! { todo!("0xf1a784 -[FlurryPLCrashReportSignalInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportSignalInfo name]")]
// 0xf1a7e8 — -[FlurryPLCrashReportSignalInfo name]
// type: NSString *__cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a7e8() -> ! { todo!("0xf1a7e8 -[FlurryPLCrashReportSignalInfo name]") }

#[doc(alias = "-[FlurryPLCrashReportSignalInfo code]")]
// 0xf1a7f8 — -[FlurryPLCrashReportSignalInfo code]
// type: NSString *__cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a7f8() -> ! { todo!("0xf1a7f8 -[FlurryPLCrashReportSignalInfo code]") }

#[doc(alias = "-[FlurryPLCrashReportSignalInfo address]")]
// 0xf1a808 — -[FlurryPLCrashReportSignalInfo address]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a808() -> ! { todo!("0xf1a808 -[FlurryPLCrashReportSignalInfo address]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]")]
// 0xf1a820 — -[FlurryPLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]
// type: FlurryPLCrashReportProcessInfo *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL, id, unsigned int, id, id, unsigned int, char)
pub fn stub_0xf1a820() -> ! { todo!("0xf1a820 -[FlurryPLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo dealloc]")]
// 0xf1a8e8 — -[FlurryPLCrashReportProcessInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a8e8() -> ! { todo!("0xf1a8e8 -[FlurryPLCrashReportProcessInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo processName]")]
// 0xf1a960 — -[FlurryPLCrashReportProcessInfo processName]
// type: NSString *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a960() -> ! { todo!("0xf1a960 -[FlurryPLCrashReportProcessInfo processName]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo processID]")]
// 0xf1a970 — -[FlurryPLCrashReportProcessInfo processID]
// type: unsigned int __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a970() -> ! { todo!("0xf1a970 -[FlurryPLCrashReportProcessInfo processID]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo processPath]")]
// 0xf1a980 — -[FlurryPLCrashReportProcessInfo processPath]
// type: NSString *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a980() -> ! { todo!("0xf1a980 -[FlurryPLCrashReportProcessInfo processPath]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo parentProcessName]")]
// 0xf1a990 — -[FlurryPLCrashReportProcessInfo parentProcessName]
// type: NSString *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a990() -> ! { todo!("0xf1a990 -[FlurryPLCrashReportProcessInfo parentProcessName]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo parentProcessID]")]
// 0xf1a9a0 — -[FlurryPLCrashReportProcessInfo parentProcessID]
// type: unsigned int __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a9a0() -> ! { todo!("0xf1a9a0 -[FlurryPLCrashReportProcessInfo parentProcessID]") }

#[doc(alias = "-[FlurryPLCrashReportProcessInfo native]")]
// 0xf1a9b0 — -[FlurryPLCrashReportProcessInfo native]
// type: char __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a9b0() -> ! { todo!("0xf1a9b0 -[FlurryPLCrashReportProcessInfo native]") }

#[doc(alias = "+[FlurryPLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]")]
// 0xf1a9c0 — +[FlurryPLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]
// type: id __cdecl(id, SEL, id, int)
pub fn stub_0xf1a9c0() -> ! { todo!("0xf1a9c0 +[FlurryPLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]") }

#[doc(alias = "_binaryImageSort_0")]
// 0xf1ba80 — _binaryImageSort_0
pub fn stub_0xf1ba80() -> ! { todo!("0xf1ba80 _binaryImageSort_0") }

#[doc(alias = "-[FlurryPLCrashReportTextFormatter initWithTextFormat:stringEncoding:]")]
// 0xf1bae8 — -[FlurryPLCrashReportTextFormatter initWithTextFormat:stringEncoding:]
// type: FlurryPLCrashReportTextFormatter *__cdecl(FlurryPLCrashReportTextFormatter *self, SEL, int, unsigned int)
pub fn stub_0xf1bae8() -> ! { todo!("0xf1bae8 -[FlurryPLCrashReportTextFormatter initWithTextFormat:stringEncoding:]") }

#[doc(alias = "-[FlurryPLCrashReportTextFormatter formatReport:error:]")]
// 0xf1bb3c — -[FlurryPLCrashReportTextFormatter formatReport:error:]
// type: id __cdecl(FlurryPLCrashReportTextFormatter *self, SEL, id, id *)
pub fn stub_0xf1bb3c() -> ! { todo!("0xf1bb3c -[FlurryPLCrashReportTextFormatter formatReport:error:]") }

#[doc(alias = "+[FlurryPLCrashReportTextFormatter formatStackFrame:frameIndex:report:lp64:]")]
// 0xf1bb90 — +[FlurryPLCrashReportTextFormatter formatStackFrame:frameIndex:report:lp64:]
// type: id __cdecl(id, SEL, id, unsigned int, id, char)
pub fn stub_0xf1bb90() -> ! { todo!("0xf1bb90 +[FlurryPLCrashReportTextFormatter formatStackFrame:frameIndex:report:lp64:]") }

#[doc(alias = "_plcrash_nasync_image_list_init")]
// 0xf1be18 — _plcrash_nasync_image_list_init
pub fn stub_0xf1be18() -> ! { todo!("0xf1be18 _plcrash_nasync_image_list_init") }

#[doc(alias = "_plcrash_nasync_image_list_free")]
// 0xf1be48 — _plcrash_nasync_image_list_free
pub fn stub_0xf1be48() -> ! { todo!("0xf1be48 _plcrash_nasync_image_list_free") }

#[doc(alias = "_plcrash_nasync_image_list_append")]
// 0xf1be84 — _plcrash_nasync_image_list_append
pub fn stub_0xf1be84() -> ! { todo!("0xf1be84 _plcrash_nasync_image_list_append") }

#[doc(alias = "_plcrash_nasync_image_list_remove")]
// 0xf1bf00 — _plcrash_nasync_image_list_remove
pub fn stub_0xf1bf00() -> ! { todo!("0xf1bf00 _plcrash_nasync_image_list_remove") }

#[doc(alias = "_plcrash_async_image_list_set_reading_0")]
// 0xf1bf78 — _plcrash_async_image_list_set_reading_0
pub fn stub_0xf1bf78() -> ! { todo!("0xf1bf78 _plcrash_async_image_list_set_reading_0") }

#[doc(alias = "_plcrash_async_image_containing_address")]
// 0xf1bf94 — _plcrash_async_image_containing_address
pub fn stub_0xf1bf94() -> ! { todo!("0xf1bf94 _plcrash_async_image_containing_address") }

#[doc(alias = "_plcrash_async_image_list_next_0")]
// 0xf1bfc4 — _plcrash_async_image_list_next_0
pub fn stub_0xf1bfc4() -> ! { todo!("0xf1bfc4 _plcrash_async_image_list_next_0") }

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]")]
// 0xf1bfd4 — -[FlurryPLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]
// type: FlurryPLCrashReportProcessorInfo *__cdecl(FlurryPLCrashReportProcessorInfo *self, SEL, int, unsigned __int64, unsigned __int64)
pub fn stub_0xf1bfd4() -> ! { todo!("0xf1bfd4 -[FlurryPLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]") }

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo typeEncoding]")]
// 0xf1c04c — -[FlurryPLCrashReportProcessorInfo typeEncoding]
// type: int __cdecl(FlurryPLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xf1c04c() -> ! { todo!("0xf1c04c -[FlurryPLCrashReportProcessorInfo typeEncoding]") }

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo type]")]
// 0xf1c05c — -[FlurryPLCrashReportProcessorInfo type]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xf1c05c() -> ! { todo!("0xf1c05c -[FlurryPLCrashReportProcessorInfo type]") }

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo subtype]")]
// 0xf1c074 — -[FlurryPLCrashReportProcessorInfo subtype]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xf1c074() -> ! { todo!("0xf1c074 -[FlurryPLCrashReportProcessorInfo subtype]") }

#[doc(alias = "-[FlurryPLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]")]
// 0xf1c08c — -[FlurryPLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]
// type: FlurryPLCrashReportMachineInfo *__cdecl(FlurryPLCrashReportMachineInfo *self, SEL, id, id, unsigned int, unsigned int)
pub fn stub_0xf1c08c() -> ! { todo!("0xf1c08c -[FlurryPLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]") }

#[doc(alias = "-[FlurryPLCrashReportMachineInfo dealloc]")]
// 0xf1c12c — -[FlurryPLCrashReportMachineInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c12c() -> ! { todo!("0xf1c12c -[FlurryPLCrashReportMachineInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportMachineInfo modelName]")]
// 0xf1c190 — -[FlurryPLCrashReportMachineInfo modelName]
// type: NSString *__cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c190() -> ! { todo!("0xf1c190 -[FlurryPLCrashReportMachineInfo modelName]") }

#[doc(alias = "-[FlurryPLCrashReportMachineInfo processorInfo]")]
// 0xf1c1a0 — -[FlurryPLCrashReportMachineInfo processorInfo]
// type: FlurryPLCrashReportProcessorInfo *__cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c1a0() -> ! { todo!("0xf1c1a0 -[FlurryPLCrashReportMachineInfo processorInfo]") }

#[doc(alias = "-[FlurryPLCrashReportMachineInfo processorCount]")]
// 0xf1c1b0 — -[FlurryPLCrashReportMachineInfo processorCount]
// type: unsigned int __cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c1b0() -> ! { todo!("0xf1c1b0 -[FlurryPLCrashReportMachineInfo processorCount]") }

#[doc(alias = "-[FlurryPLCrashReportMachineInfo logicalProcessorCount]")]
// 0xf1c1c0 — -[FlurryPLCrashReportMachineInfo logicalProcessorCount]
// type: unsigned int __cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c1c0() -> ! { todo!("0xf1c1c0 -[FlurryPLCrashReportMachineInfo logicalProcessorCount]") }

#[doc(alias = "_plcrash_sysctl_string_0")]
// 0xf1c1d0 — _plcrash_sysctl_string_0
// type: void *__fastcall(char *)
pub fn stub_0xf1c1d0() -> ! { todo!("0xf1c1d0 _plcrash_sysctl_string_0") }

#[doc(alias = "_plcrash_sysctl_int_0")]
// 0xf1c254 — _plcrash_sysctl_int_0
pub fn stub_0xf1c254() -> ! { todo!("0xf1c254 _plcrash_sysctl_int_0") }

#[doc(alias = "_plcrash_log_writer_write_curthread_0")]
// 0xf1c280 — _plcrash_log_writer_write_curthread_0
pub fn stub_0xf1c280() -> ! { todo!("0xf1c280 _plcrash_log_writer_write_curthread_0") }

#[doc(alias = "_plcrash_log_writer_write_curthread_stub_0")]
// 0xf1c2e0 — _plcrash_log_writer_write_curthread_stub_0
pub fn stub_0xf1c2e0() -> ! { todo!("0xf1c2e0 _plcrash_log_writer_write_curthread_stub_0") }

#[doc(alias = "_plcrash_populate_error_0")]
// 0xf1c354 — _plcrash_populate_error_0
pub fn stub_0xf1c354() -> ! { todo!("0xf1c354 _plcrash_populate_error_0") }

#[doc(alias = "_plcrash_populate_mach_error")]
// 0xf1c3f4 — _plcrash_populate_mach_error
pub fn stub_0xf1c3f4() -> ! { todo!("0xf1c3f4 _plcrash_populate_mach_error") }

#[doc(alias = "_plcrash_populate_posix_error_0")]
// 0xf1c440 — _plcrash_populate_posix_error_0
pub fn stub_0xf1c440() -> ! { todo!("0xf1c440 _plcrash_populate_posix_error_0") }

#[doc(alias = "_plcrash_nasync_macho_init")]
// 0xf1c48c — _plcrash_nasync_macho_init
pub fn stub_0xf1c48c() -> ! { todo!("0xf1c48c _plcrash_nasync_macho_init") }

#[doc(alias = "_macho_nswap16")]
// 0xf1c664 — _macho_nswap16
pub fn stub_0xf1c664() -> ! { todo!("0xf1c664 _macho_nswap16") }

#[doc(alias = "_macho_nswap32")]
// 0xf1c668 — _macho_nswap32
pub fn stub_0xf1c668() -> ! { todo!("0xf1c668 _macho_nswap32") }

#[doc(alias = "_macho_nswap64")]
// 0xf1c66c — _macho_nswap64
pub fn stub_0xf1c66c() -> ! { todo!("0xf1c66c _macho_nswap64") }
