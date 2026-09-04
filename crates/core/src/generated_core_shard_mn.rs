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
pub fn stub_0xf1a660() {
    // IDA 0xf1a660: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportExceptionInfo stackFrames]")]
// 0xf1a670 — -[FlurryPLCrashReportExceptionInfo stackFrames]
// type: NSArray *__cdecl(FlurryPLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xf1a670() {
    // IDA 0xf1a670: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_signal_sigcode_0")]
// 0xf1a680 — _plcrash_async_signal_sigcode_0
pub fn stub_0xf1a680() {
    // IDA 0xf1a680: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_signal_signame_0")]
// 0xf1a6c4 — _plcrash_async_signal_signame_0
pub fn stub_0xf1a6c4() {
    // IDA 0xf1a6c4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSignalInfo initWithSignalName:code:address:]")]
// 0xf1a6ec — -[FlurryPLCrashReportSignalInfo initWithSignalName:code:address:]
// type: FlurryPLCrashReportSignalInfo *__cdecl(FlurryPLCrashReportSignalInfo *self, SEL, id, id, unsigned __int64)
pub fn stub_0xf1a6ec() {
    // IDA 0xf1a6ec: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSignalInfo dealloc]")]
// 0xf1a784 — -[FlurryPLCrashReportSignalInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a784() {
    // IDA 0xf1a784: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSignalInfo name]")]
// 0xf1a7e8 — -[FlurryPLCrashReportSignalInfo name]
// type: NSString *__cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a7e8() {
    // IDA 0xf1a7e8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSignalInfo code]")]
// 0xf1a7f8 — -[FlurryPLCrashReportSignalInfo code]
// type: NSString *__cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a7f8() {
    // IDA 0xf1a7f8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSignalInfo address]")]
// 0xf1a808 — -[FlurryPLCrashReportSignalInfo address]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportSignalInfo *self, SEL)
pub fn stub_0xf1a808() {
    // IDA 0xf1a808: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]")]
// 0xf1a820 — -[FlurryPLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]
// type: FlurryPLCrashReportProcessInfo *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL, id, unsigned int, id, id, unsigned int, char)
pub fn stub_0xf1a820() {
    // IDA 0xf1a820: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo dealloc]")]
// 0xf1a8e8 — -[FlurryPLCrashReportProcessInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a8e8() {
    // IDA 0xf1a8e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo processName]")]
// 0xf1a960 — -[FlurryPLCrashReportProcessInfo processName]
// type: NSString *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a960() {
    // IDA 0xf1a960: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo processID]")]
// 0xf1a970 — -[FlurryPLCrashReportProcessInfo processID]
// type: unsigned int __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a970() {
    // IDA 0xf1a970: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo processPath]")]
// 0xf1a980 — -[FlurryPLCrashReportProcessInfo processPath]
// type: NSString *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a980() {
    // IDA 0xf1a980: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo parentProcessName]")]
// 0xf1a990 — -[FlurryPLCrashReportProcessInfo parentProcessName]
// type: NSString *__cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a990() {
    // IDA 0xf1a990: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo parentProcessID]")]
// 0xf1a9a0 — -[FlurryPLCrashReportProcessInfo parentProcessID]
// type: unsigned int __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a9a0() {
    // IDA 0xf1a9a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessInfo native]")]
// 0xf1a9b0 — -[FlurryPLCrashReportProcessInfo native]
// type: char __cdecl(FlurryPLCrashReportProcessInfo *self, SEL)
pub fn stub_0xf1a9b0() {
    // IDA 0xf1a9b0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryPLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]")]
// 0xf1a9c0 — +[FlurryPLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]
// type: id __cdecl(id, SEL, id, int)
pub fn stub_0xf1a9c0() {
    // IDA 0xf1a9c0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_binaryImageSort_0")]
// 0xf1ba80 — _binaryImageSort_0
pub fn stub_0xf1ba80() {
    // IDA 0xf1ba80: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "-[FlurryPLCrashReportTextFormatter initWithTextFormat:stringEncoding:]")]
// 0xf1bae8 — -[FlurryPLCrashReportTextFormatter initWithTextFormat:stringEncoding:]
// type: FlurryPLCrashReportTextFormatter *__cdecl(FlurryPLCrashReportTextFormatter *self, SEL, int, unsigned int)
pub fn stub_0xf1bae8() {
    // IDA 0xf1bae8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "-[FlurryPLCrashReportTextFormatter formatReport:error:]")]
// 0xf1bb3c — -[FlurryPLCrashReportTextFormatter formatReport:error:]
// type: id __cdecl(FlurryPLCrashReportTextFormatter *self, SEL, id, id *)
pub fn stub_0xf1bb3c() {
    // IDA 0xf1bb3c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "+[FlurryPLCrashReportTextFormatter formatStackFrame:frameIndex:report:lp64:]")]
// 0xf1bb90 — +[FlurryPLCrashReportTextFormatter formatStackFrame:frameIndex:report:lp64:]
// type: id __cdecl(id, SEL, id, unsigned int, id, char)
pub fn stub_0xf1bb90() {
    // IDA 0xf1bb90: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_nasync_image_list_init")]
// 0xf1be18 — _plcrash_nasync_image_list_init
pub fn stub_0xf1be18() {
    // IDA 0xf1be18: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_nasync_image_list_free")]
// 0xf1be48 — _plcrash_nasync_image_list_free
pub fn stub_0xf1be48() {
    // IDA 0xf1be48: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_nasync_image_list_append")]
// 0xf1be84 — _plcrash_nasync_image_list_append
pub fn stub_0xf1be84() {
    // IDA 0xf1be84: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_nasync_image_list_remove")]
// 0xf1bf00 — _plcrash_nasync_image_list_remove
pub fn stub_0xf1bf00() {
    // IDA 0xf1bf00: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_image_list_set_reading_0")]
// 0xf1bf78 — _plcrash_async_image_list_set_reading_0
pub fn stub_0xf1bf78() {
    // IDA 0xf1bf78: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_image_containing_address")]
// 0xf1bf94 — _plcrash_async_image_containing_address
pub fn stub_0xf1bf94() {
    // IDA 0xf1bf94: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_image_list_next_0")]
// 0xf1bfc4 — _plcrash_async_image_list_next_0
pub fn stub_0xf1bfc4() {
    // IDA 0xf1bfc4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]")]
// 0xf1bfd4 — -[FlurryPLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]
// type: FlurryPLCrashReportProcessorInfo *__cdecl(FlurryPLCrashReportProcessorInfo *self, SEL, int, unsigned __int64, unsigned __int64)
pub fn stub_0xf1bfd4() {
    // IDA 0xf1bfd4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo typeEncoding]")]
// 0xf1c04c — -[FlurryPLCrashReportProcessorInfo typeEncoding]
// type: int __cdecl(FlurryPLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xf1c04c() {
    // IDA 0xf1c04c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo type]")]
// 0xf1c05c — -[FlurryPLCrashReportProcessorInfo type]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xf1c05c() {
    // IDA 0xf1c05c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportProcessorInfo subtype]")]
// 0xf1c074 — -[FlurryPLCrashReportProcessorInfo subtype]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xf1c074() {
    // IDA 0xf1c074: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]")]
// 0xf1c08c — -[FlurryPLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]
// type: FlurryPLCrashReportMachineInfo *__cdecl(FlurryPLCrashReportMachineInfo *self, SEL, id, id, unsigned int, unsigned int)
pub fn stub_0xf1c08c() {
    // IDA 0xf1c08c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportMachineInfo dealloc]")]
// 0xf1c12c — -[FlurryPLCrashReportMachineInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c12c() {
    // IDA 0xf1c12c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportMachineInfo modelName]")]
// 0xf1c190 — -[FlurryPLCrashReportMachineInfo modelName]
// type: NSString *__cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c190() {
    // IDA 0xf1c190: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportMachineInfo processorInfo]")]
// 0xf1c1a0 — -[FlurryPLCrashReportMachineInfo processorInfo]
// type: FlurryPLCrashReportProcessorInfo *__cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c1a0() {
    // IDA 0xf1c1a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportMachineInfo processorCount]")]
// 0xf1c1b0 — -[FlurryPLCrashReportMachineInfo processorCount]
// type: unsigned int __cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c1b0() {
    // IDA 0xf1c1b0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportMachineInfo logicalProcessorCount]")]
// 0xf1c1c0 — -[FlurryPLCrashReportMachineInfo logicalProcessorCount]
// type: unsigned int __cdecl(FlurryPLCrashReportMachineInfo *self, SEL)
pub fn stub_0xf1c1c0() {
    // IDA 0xf1c1c0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_sysctl_string_0")]
// 0xf1c1d0 — _plcrash_sysctl_string_0
// type: void *__fastcall(char *)
pub fn stub_0xf1c1d0() {
    // IDA 0xf1c1d0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_sysctl_int_0")]
// 0xf1c254 — _plcrash_sysctl_int_0
pub fn stub_0xf1c254() {
    // IDA 0xf1c254: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_write_curthread_0")]
// 0xf1c280 — _plcrash_log_writer_write_curthread_0
pub fn stub_0xf1c280() {
    // IDA 0xf1c280: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_write_curthread_stub_0")]
// 0xf1c2e0 — _plcrash_log_writer_write_curthread_stub_0
pub fn stub_0xf1c2e0() {
    // IDA 0xf1c2e0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_populate_error_0")]
// 0xf1c354 — _plcrash_populate_error_0
pub fn stub_0xf1c354() {
    // IDA 0xf1c354: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_populate_mach_error")]
// 0xf1c3f4 — _plcrash_populate_mach_error
pub fn stub_0xf1c3f4() {
    // IDA 0xf1c3f4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_populate_posix_error_0")]
// 0xf1c440 — _plcrash_populate_posix_error_0
pub fn stub_0xf1c440() {
    // IDA 0xf1c440: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_nasync_macho_init")]
// 0xf1c48c — _plcrash_nasync_macho_init
pub fn stub_0xf1c48c() {
    // IDA 0xf1c48c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_macho_nswap16")]
// 0xf1c664 — _macho_nswap16
pub fn stub_0xf1c664() {
    // IDA 0xf1c664: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_macho_nswap32")]
// 0xf1c668 — _macho_nswap32
pub fn stub_0xf1c668() {
    // IDA 0xf1c668: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_macho_nswap64")]
// 0xf1c66c — _macho_nswap64
pub fn stub_0xf1c66c() {
    // IDA 0xf1c66c: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}
