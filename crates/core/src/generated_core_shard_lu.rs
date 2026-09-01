//! core shard lu — 150 core stubs EA-sorted, next uncovered fallback after shard lt (0xef3e7c..0xefa4fc, lowest EA first).
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound (33260 remaining, 3503 uncovered before -> 3353 after, rbx_core::SharedPtr not boost) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "+[PLCrashReporter sharedReporter]")]
// 0xef3e7c — +[PLCrashReporter sharedReporter]
// type: id __cdecl(id, SEL)
pub fn stub_0xef3e7c() -> ! {
    todo!("0xef3e7c +[PLCrashReporter sharedReporter]")
}

#[doc(alias = "-[PLCrashReporter hasPendingCrashReport]")]
// 0xef3ee4 — -[PLCrashReporter hasPendingCrashReport]
// type: char __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef3ee4() -> ! {
    todo!("0xef3ee4 -[PLCrashReporter hasPendingCrashReport]")
}

#[doc(alias = "-[PLCrashReporter loadPendingCrashReportData]")]
// 0xef3f30 — -[PLCrashReporter loadPendingCrashReportData]
// type: id __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef3f30() -> ! {
    todo!("0xef3f30 -[PLCrashReporter loadPendingCrashReportData]")
}

#[doc(alias = "-[PLCrashReporter loadPendingCrashReportDataAndReturnError:]")]
// 0xef3f48 — -[PLCrashReporter loadPendingCrashReportDataAndReturnError:]
// type: id __cdecl(PLCrashReporter *self, SEL, id *)
pub fn stub_0xef3f48() -> ! {
    todo!("0xef3f48 -[PLCrashReporter loadPendingCrashReportDataAndReturnError:]")
}

#[doc(alias = "-[PLCrashReporter purgePendingCrashReport]")]
// 0xef3f88 — -[PLCrashReporter purgePendingCrashReport]
// type: char __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef3f88() -> ! {
    todo!("0xef3f88 -[PLCrashReporter purgePendingCrashReport]")
}

#[doc(alias = "-[PLCrashReporter purgePendingCrashReportAndReturnError:]")]
// 0xef3fa0 — -[PLCrashReporter purgePendingCrashReportAndReturnError:]
// type: char __cdecl(PLCrashReporter *self, SEL, id *)
pub fn stub_0xef3fa0() -> ! {
    todo!("0xef3fa0 -[PLCrashReporter purgePendingCrashReportAndReturnError:]")
}

#[doc(alias = "-[PLCrashReporter enableCrashReporter]")]
// 0xef3ff0 — -[PLCrashReporter enableCrashReporter]
// type: char __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef3ff0() -> ! {
    todo!("0xef3ff0 -[PLCrashReporter enableCrashReporter]")
}

#[doc(alias = "-[PLCrashReporter enableCrashReporterAndReturnError:]")]
// 0xef4008 — -[PLCrashReporter enableCrashReporterAndReturnError:]
// type: char __cdecl(PLCrashReporter *self, SEL, id *)
pub fn stub_0xef4008() -> ! {
    todo!("0xef4008 -[PLCrashReporter enableCrashReporterAndReturnError:]")
}

#[doc(alias = "_signal_handler_callback")]
// 0xef418c — _signal_handler_callback
pub fn stub_0xef418c() -> ! {
    todo!("0xef418c _signal_handler_callback")
}

#[doc(alias = "_uncaught_exception_handler")]
// 0xef4224 — _uncaught_exception_handler
// type: void __fastcall __noreturn(id)
pub fn stub_0xef4224() -> ! {
    todo!("0xef4224 _uncaught_exception_handler")
}

#[doc(alias = "-[PLCrashReporter generateLiveReport]")]
// 0xef4240 — -[PLCrashReporter generateLiveReport]
// type: id __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef4240() -> ! {
    todo!("0xef4240 -[PLCrashReporter generateLiveReport]")
}

#[doc(alias = "-[PLCrashReporter generateLiveReportAndReturnError:]")]
// 0xef4258 — -[PLCrashReporter generateLiveReportAndReturnError:]
// type: id __cdecl(PLCrashReporter *self, SEL, id *)
pub fn stub_0xef4258() -> ! {
    todo!("0xef4258 -[PLCrashReporter generateLiveReportAndReturnError:]")
}

#[doc(alias = "-[PLCrashReporter setCrashCallbacks:]")]
// 0xef4478 — -[PLCrashReporter setCrashCallbacks:]
// type: void __cdecl(PLCrashReporter *self, SEL, PLCrashReporterCallbacks *)
pub fn stub_0xef4478() -> ! {
    todo!("0xef4478 -[PLCrashReporter setCrashCallbacks:]")
}

#[doc(alias = "-[PLCrashReporter initWithApplicationIdentifier:appVersion:]")]
// 0xef4500 — -[PLCrashReporter initWithApplicationIdentifier:appVersion:]
// type: PLCrashReporter *__cdecl(PLCrashReporter *self, SEL, id, id)
pub fn stub_0xef4500() -> ! {
    todo!("0xef4500 -[PLCrashReporter initWithApplicationIdentifier:appVersion:]")
}

#[doc(alias = "-[PLCrashReporter initWithBundle:]")]
// 0xef4618 — -[PLCrashReporter initWithBundle:]
// type: PLCrashReporter *__cdecl(PLCrashReporter *self, SEL, id)
pub fn stub_0xef4618() -> ! {
    todo!("0xef4618 -[PLCrashReporter initWithBundle:]")
}

#[doc(alias = "-[PLCrashReporter dealloc]")]
// 0xef471c — -[PLCrashReporter dealloc]
// type: void __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef471c() -> ! {
    todo!("0xef471c -[PLCrashReporter dealloc]")
}

#[doc(alias = "-[PLCrashReporter populateCrashReportDirectoryAndReturnError:]")]
// 0xef4794 — -[PLCrashReporter populateCrashReportDirectoryAndReturnError:]
// type: char __cdecl(PLCrashReporter *self, SEL, id *)
pub fn stub_0xef4794() -> ! {
    todo!("0xef4794 -[PLCrashReporter populateCrashReportDirectoryAndReturnError:]")
}

#[doc(alias = "-[PLCrashReporter crashReportDirectory]")]
// 0xef48cc — -[PLCrashReporter crashReportDirectory]
// type: id __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef48cc() -> ! {
    todo!("0xef48cc -[PLCrashReporter crashReportDirectory]")
}

#[doc(alias = "-[PLCrashReporter queuedCrashReportDirectory]")]
// 0xef48dc — -[PLCrashReporter queuedCrashReportDirectory]
// type: id __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef48dc() -> ! {
    todo!("0xef48dc -[PLCrashReporter queuedCrashReportDirectory]")
}

#[doc(alias = "-[PLCrashReporter crashReportPath]")]
// 0xef490c — -[PLCrashReporter crashReportPath]
// type: id __cdecl(PLCrashReporter *self, SEL)
pub fn stub_0xef490c() -> ! {
    todo!("0xef490c -[PLCrashReporter crashReportPath]")
}

#[doc(alias = "-[PLCrashReport initWithData:error:]")]
// 0xef493c — -[PLCrashReport initWithData:error:]
// type: PLCrashReport *__cdecl(PLCrashReport *self, SEL, id, id *)
pub fn stub_0xef493c() -> ! {
    todo!("0xef493c -[PLCrashReport initWithData:error:]")
}

#[doc(alias = "_populate_nserror")]
// 0xef4b9c — _populate_nserror
pub fn stub_0xef4b9c() -> ! {
    todo!("0xef4b9c _populate_nserror")
}

#[doc(alias = "-[PLCrashReport dealloc]")]
// 0xef4c0c — -[PLCrashReport dealloc]
// type: void __cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4c0c() -> ! {
    todo!("0xef4c0c -[PLCrashReport dealloc]")
}

#[doc(alias = "-[PLCrashReport imageForAddress:]")]
// 0xef4d18 — -[PLCrashReport imageForAddress:]
// type: id __cdecl(PLCrashReport *self, SEL, unsigned __int64)
pub fn stub_0xef4d18() -> ! {
    todo!("0xef4d18 -[PLCrashReport imageForAddress:]")
}

#[doc(alias = "-[PLCrashReport hasMachineInfo]")]
// 0xef4e58 — -[PLCrashReport hasMachineInfo]
// type: char __cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4e58() -> ! {
    todo!("0xef4e58 -[PLCrashReport hasMachineInfo]")
}

#[doc(alias = "-[PLCrashReport hasProcessInfo]")]
// 0xef4e70 — -[PLCrashReport hasProcessInfo]
// type: char __cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4e70() -> ! {
    todo!("0xef4e70 -[PLCrashReport hasProcessInfo]")
}

#[doc(alias = "-[PLCrashReport hasExceptionInfo]")]
// 0xef4e88 — -[PLCrashReport hasExceptionInfo]
// type: char __cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4e88() -> ! {
    todo!("0xef4e88 -[PLCrashReport hasExceptionInfo]")
}

#[doc(alias = "-[PLCrashReport systemInfo]")]
// 0xef4ea0 — -[PLCrashReport systemInfo]
// type: PLCrashReportSystemInfo *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4ea0() -> ! {
    todo!("0xef4ea0 -[PLCrashReport systemInfo]")
}

#[doc(alias = "-[PLCrashReport machineInfo]")]
// 0xef4eb0 — -[PLCrashReport machineInfo]
// type: PLCrashReportMachineInfo *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4eb0() -> ! {
    todo!("0xef4eb0 -[PLCrashReport machineInfo]")
}

#[doc(alias = "-[PLCrashReport applicationInfo]")]
// 0xef4ec0 — -[PLCrashReport applicationInfo]
// type: PLCrashReportApplicationInfo *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4ec0() -> ! {
    todo!("0xef4ec0 -[PLCrashReport applicationInfo]")
}

#[doc(alias = "-[PLCrashReport processInfo]")]
// 0xef4ed0 — -[PLCrashReport processInfo]
// type: PLCrashReportProcessInfo *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4ed0() -> ! {
    todo!("0xef4ed0 -[PLCrashReport processInfo]")
}

#[doc(alias = "-[PLCrashReport signalInfo]")]
// 0xef4ee0 — -[PLCrashReport signalInfo]
// type: PLCrashReportSignalInfo *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4ee0() -> ! {
    todo!("0xef4ee0 -[PLCrashReport signalInfo]")
}

#[doc(alias = "-[PLCrashReport threads]")]
// 0xef4ef0 — -[PLCrashReport threads]
// type: NSArray *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4ef0() -> ! {
    todo!("0xef4ef0 -[PLCrashReport threads]")
}

#[doc(alias = "-[PLCrashReport images]")]
// 0xef4f00 — -[PLCrashReport images]
// type: NSArray *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4f00() -> ! {
    todo!("0xef4f00 -[PLCrashReport images]")
}

#[doc(alias = "-[PLCrashReport exceptionInfo]")]
// 0xef4f10 — -[PLCrashReport exceptionInfo]
// type: PLCrashReportExceptionInfo *__cdecl(PLCrashReport *self, SEL)
pub fn stub_0xef4f10() -> ! {
    todo!("0xef4f10 -[PLCrashReport exceptionInfo]")
}

#[doc(alias = "-[PLCrashReport decodeCrashData:error:]")]
// 0xef4f20 — -[PLCrashReport decodeCrashData:error:]
// type: _Plcrash__CrashReport *__cdecl(PLCrashReport *self, SEL, id, id *)
pub fn stub_0xef4f20() -> ! {
    todo!("0xef4f20 -[PLCrashReport decodeCrashData:error:]")
}

#[doc(alias = "-[PLCrashReport extractSystemInfo:error:]")]
// 0xef50bc — -[PLCrashReport extractSystemInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__SystemInfo *, id *)
pub fn stub_0xef50bc() -> ! {
    todo!("0xef50bc -[PLCrashReport extractSystemInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractProcessorInfo:error:]")]
// 0xef522c — -[PLCrashReport extractProcessorInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__Processor *, id *)
pub fn stub_0xef522c() -> ! {
    todo!("0xef522c -[PLCrashReport extractProcessorInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractMachineInfo:error:]")]
// 0xef52e0 — -[PLCrashReport extractMachineInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__MachineInfo *, id *)
pub fn stub_0xef52e0() -> ! {
    todo!("0xef52e0 -[PLCrashReport extractMachineInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractApplicationInfo:error:]")]
// 0xef53e0 — -[PLCrashReport extractApplicationInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__ApplicationInfo *, id *)
pub fn stub_0xef53e0() -> ! {
    todo!("0xef53e0 -[PLCrashReport extractApplicationInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractProcessInfo:error:]")]
// 0xef552c — -[PLCrashReport extractProcessInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__ProcessInfo *, id *)
pub fn stub_0xef552c() -> ! {
    todo!("0xef552c -[PLCrashReport extractProcessInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractStackFrameInfo:error:]")]
// 0xef5660 — -[PLCrashReport extractStackFrameInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__Thread__StackFrame *, id *)
pub fn stub_0xef5660() -> ! {
    todo!("0xef5660 -[PLCrashReport extractStackFrameInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractThreadInfo:error:]")]
// 0xef5708 — -[PLCrashReport extractThreadInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport *, id *)
pub fn stub_0xef5708() -> ! {
    todo!("0xef5708 -[PLCrashReport extractThreadInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractImageInfo:error:]")]
// 0xef5950 — -[PLCrashReport extractImageInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport *, id *)
pub fn stub_0xef5950() -> ! {
    todo!("0xef5950 -[PLCrashReport extractImageInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractExceptionInfo:error:]")]
// 0xef5b64 — -[PLCrashReport extractExceptionInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__Exception *, id *)
pub fn stub_0xef5b64() -> ! {
    todo!("0xef5b64 -[PLCrashReport extractExceptionInfo:error:]")
}

#[doc(alias = "-[PLCrashReport extractSignalInfo:error:]")]
// 0xef5d40 — -[PLCrashReport extractSignalInfo:error:]
// type: id __cdecl(PLCrashReport *self, SEL, _Plcrash__CrashReport__Signal *, id *)
pub fn stub_0xef5d40() -> ! {
    todo!("0xef5d40 -[PLCrashReport extractSignalInfo:error:]")
}

#[doc(alias = "_plcrash__crash_report__init")]
// 0xef5e94 — _plcrash__crash_report__init
pub fn stub_0xef5e94() -> ! {
    todo!("0xef5e94 _plcrash__crash_report__init")
}

#[doc(alias = "_plcrash__crash_report__get_packed_size")]
// 0xef5ed0 — _plcrash__crash_report__get_packed_size
pub fn stub_0xef5ed0() -> ! {
    todo!("0xef5ed0 _plcrash__crash_report__get_packed_size")
}

#[doc(alias = "_plcrash__crash_report__pack")]
// 0xef5f10 — _plcrash__crash_report__pack
pub fn stub_0xef5f10() -> ! {
    todo!("0xef5f10 _plcrash__crash_report__pack")
}

#[doc(alias = "_plcrash__crash_report__pack_to_buffer")]
// 0xef5f50 — _plcrash__crash_report__pack_to_buffer
// type: int __fastcall(void **)
pub fn stub_0xef5f50() -> ! {
    todo!("0xef5f50 _plcrash__crash_report__pack_to_buffer")
}

#[doc(alias = "_plcrash__crash_report__unpack")]
// 0xef5f90 — _plcrash__crash_report__unpack
pub fn stub_0xef5f90() -> ! {
    todo!("0xef5f90 _plcrash__crash_report__unpack")
}

#[doc(alias = "_plcrash__crash_report__free_unpacked")]
// 0xef5fac — _plcrash__crash_report__free_unpacked
pub fn stub_0xef5fac() -> ! {
    todo!("0xef5fac _plcrash__crash_report__free_unpacked")
}

#[doc(alias = "_protobuf_c_out_of_memory_default")]
// 0xef5fec — _protobuf_c_out_of_memory_default
pub fn stub_0xef5fec() -> ! {
    todo!("0xef5fec _protobuf_c_out_of_memory_default")
}

#[doc(alias = "_system_alloc")]
// 0xef6014 — _system_alloc
// type: int __fastcall(int, size_t __size)
pub fn stub_0xef6014() -> ! {
    todo!("0xef6014 _system_alloc")
}

#[doc(alias = "_system_free")]
// 0xef603c — _system_free
// type: int __fastcall(int, void *)
pub fn stub_0xef603c() -> ! {
    todo!("0xef603c _system_free")
}

#[doc(alias = "_protobuf_c_buffer_simple_append")]
// 0xef604c — _protobuf_c_buffer_simple_append
// type: int __fastcall(int, size_t __n)
pub fn stub_0xef604c() -> ! {
    todo!("0xef604c _protobuf_c_buffer_simple_append")
}

#[doc(alias = "_protobuf_c_message_get_packed_size")]
// 0xef60c8 — _protobuf_c_message_get_packed_size
pub fn stub_0xef60c8() -> ! {
    todo!("0xef60c8 _protobuf_c_message_get_packed_size")
}

#[doc(alias = "_required_field_get_packed_size")]
// 0xef646c — _required_field_get_packed_size
pub fn stub_0xef646c() -> ! {
    todo!("0xef646c _required_field_get_packed_size")
}

#[doc(alias = "_protobuf_c_message_pack")]
// 0xef6644 — _protobuf_c_message_pack
pub fn stub_0xef6644() -> ! {
    todo!("0xef6644 _protobuf_c_message_pack")
}

#[doc(alias = "_required_field_pack")]
// 0xef67fc — _required_field_pack
pub fn stub_0xef67fc() -> ! {
    todo!("0xef67fc _required_field_pack")
}

#[doc(alias = "_protobuf_c_message_pack_to_buffer")]
// 0xef6b38 — _protobuf_c_message_pack_to_buffer
// type: int(void)
pub fn stub_0xef6b38() -> ! {
    todo!("0xef6b38 _protobuf_c_message_pack_to_buffer")
}

#[doc(alias = "_required_field_pack_to_buffer")]
// 0xef6d04 — _required_field_pack_to_buffer
pub fn stub_0xef6d04() -> ! {
    todo!("0xef6d04 _required_field_pack_to_buffer")
}

#[doc(alias = "_protobuf_c_message_unpack")]
// 0xef7118 — _protobuf_c_message_unpack
pub fn stub_0xef7118() -> ! {
    todo!("0xef7118 _protobuf_c_message_unpack")
}

#[doc(alias = "_protobuf_c_message_free_unpacked")]
// 0xef77f8 — _protobuf_c_message_free_unpacked
pub fn stub_0xef77f8() -> ! {
    todo!("0xef77f8 _protobuf_c_message_free_unpacked")
}

#[doc(alias = "_protobuf_c_service_generated_init")]
// 0xef7974 — _protobuf_c_service_generated_init
pub fn stub_0xef7974() -> ! {
    todo!("0xef7974 _protobuf_c_service_generated_init")
}

#[doc(alias = "_service_machgen_invoke")]
// 0xef79cc — _service_machgen_invoke
pub fn stub_0xef79cc() -> ! {
    todo!("0xef79cc _service_machgen_invoke")
}

#[doc(alias = "_protobuf_c_service_destroy")]
// 0xef7a18 — _protobuf_c_service_destroy
pub fn stub_0xef7a18() -> ! {
    todo!("0xef7a18 _protobuf_c_service_destroy")
}

#[doc(alias = "_parse_required_member")]
// 0xef7cc4 — _parse_required_member
pub fn stub_0xef7cc4() -> ! {
    todo!("0xef7cc4 _parse_required_member")
}

#[doc(alias = "_parse_uint64")]
// 0xef7f30 — _parse_uint64
pub fn stub_0xef7f30() -> ! {
    todo!("0xef7f30 _parse_uint64")
}

#[doc(alias = "_tag_pack")]
// 0xef7fdc — _tag_pack
pub fn stub_0xef7fdc() -> ! {
    todo!("0xef7fdc _tag_pack")
}

#[doc(alias = "_uint64_pack_0")]
// 0xef8064 — _uint64_pack_0
pub fn stub_0xef8064() -> ! {
    todo!("0xef8064 _uint64_pack_0")
}

#[doc(alias = "-[PLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:architecture:timestamp:]")]
// 0xef810c — -[PLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:architecture:timestamp:]
// type: PLCrashReportSystemInfo *__cdecl(PLCrashReportSystemInfo *self, SEL, int, id, int, id)
pub fn stub_0xef810c() -> ! {
    todo!("0xef810c -[PLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:architecture:timestamp:]")
}

#[doc(alias = "-[PLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:operatingSystemBuild:architecture:timestamp:]")]
// 0xef8140 — -[PLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:operatingSystemBuild:architecture:timestamp:]
// type: PLCrashReportSystemInfo *__cdecl(PLCrashReportSystemInfo *self, SEL, int, id, id, int, id)
pub fn stub_0xef8140() -> ! {
    todo!("0xef8140 -[PLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:operatingSystemBuild:architecture:timestamp:]")
}

#[doc(alias = "-[PLCrashReportSystemInfo dealloc]")]
// 0xef81f8 — -[PLCrashReportSystemInfo dealloc]
// type: void __cdecl(PLCrashReportSystemInfo *self, SEL)
pub fn stub_0xef81f8() -> ! {
    todo!("0xef81f8 -[PLCrashReportSystemInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportSystemInfo operatingSystem]")]
// 0xef8270 — -[PLCrashReportSystemInfo operatingSystem]
// type: int __cdecl(PLCrashReportSystemInfo *self, SEL)
pub fn stub_0xef8270() -> ! {
    todo!("0xef8270 -[PLCrashReportSystemInfo operatingSystem]")
}

#[doc(alias = "-[PLCrashReportSystemInfo operatingSystemVersion]")]
// 0xef8280 — -[PLCrashReportSystemInfo operatingSystemVersion]
// type: NSString *__cdecl(PLCrashReportSystemInfo *self, SEL)
pub fn stub_0xef8280() -> ! {
    todo!("0xef8280 -[PLCrashReportSystemInfo operatingSystemVersion]")
}

#[doc(alias = "-[PLCrashReportSystemInfo operatingSystemBuild]")]
// 0xef8290 — -[PLCrashReportSystemInfo operatingSystemBuild]
// type: NSString *__cdecl(PLCrashReportSystemInfo *self, SEL)
pub fn stub_0xef8290() -> ! {
    todo!("0xef8290 -[PLCrashReportSystemInfo operatingSystemBuild]")
}

#[doc(alias = "-[PLCrashReportSystemInfo architecture]")]
// 0xef82a0 — -[PLCrashReportSystemInfo architecture]
// type: int __cdecl(PLCrashReportSystemInfo *self, SEL)
pub fn stub_0xef82a0() -> ! {
    todo!("0xef82a0 -[PLCrashReportSystemInfo architecture]")
}

#[doc(alias = "-[PLCrashReportSystemInfo timestamp]")]
// 0xef82b0 — -[PLCrashReportSystemInfo timestamp]
// type: NSDate *__cdecl(PLCrashReportSystemInfo *self, SEL)
pub fn stub_0xef82b0() -> ! {
    todo!("0xef82b0 -[PLCrashReportSystemInfo timestamp]")
}

#[doc(alias = "-[PLCrashReportApplicationInfo initWithApplicationIdentifier:applicationVersion:]")]
// 0xef82c0 — -[PLCrashReportApplicationInfo initWithApplicationIdentifier:applicationVersion:]
// type: PLCrashReportApplicationInfo *__cdecl(PLCrashReportApplicationInfo *self, SEL, id, id)
pub fn stub_0xef82c0() -> ! {
    todo!("0xef82c0 -[PLCrashReportApplicationInfo initWithApplicationIdentifier:applicationVersion:]")
}

#[doc(alias = "-[PLCrashReportApplicationInfo dealloc]")]
// 0xef8338 — -[PLCrashReportApplicationInfo dealloc]
// type: void __cdecl(PLCrashReportApplicationInfo *self, SEL)
pub fn stub_0xef8338() -> ! {
    todo!("0xef8338 -[PLCrashReportApplicationInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportApplicationInfo applicationIdentifier]")]
// 0xef839c — -[PLCrashReportApplicationInfo applicationIdentifier]
// type: NSString *__cdecl(PLCrashReportApplicationInfo *self, SEL)
pub fn stub_0xef839c() -> ! {
    todo!("0xef839c -[PLCrashReportApplicationInfo applicationIdentifier]")
}

#[doc(alias = "-[PLCrashReportApplicationInfo applicationVersion]")]
// 0xef83ac — -[PLCrashReportApplicationInfo applicationVersion]
// type: NSString *__cdecl(PLCrashReportApplicationInfo *self, SEL)
pub fn stub_0xef83ac() -> ! {
    todo!("0xef83ac -[PLCrashReportApplicationInfo applicationVersion]")
}

#[doc(alias = "-[PLCrashReportThreadInfo initWithThreadNumber:stackFrames:crashed:registers:]")]
// 0xef83bc — -[PLCrashReportThreadInfo initWithThreadNumber:stackFrames:crashed:registers:]
// type: PLCrashReportThreadInfo *__cdecl(PLCrashReportThreadInfo *self, SEL, int, id, char, id)
pub fn stub_0xef83bc() -> ! {
    todo!("0xef83bc -[PLCrashReportThreadInfo initWithThreadNumber:stackFrames:crashed:registers:]")
}

#[doc(alias = "-[PLCrashReportThreadInfo dealloc]")]
// 0xef845c — -[PLCrashReportThreadInfo dealloc]
// type: void __cdecl(PLCrashReportThreadInfo *self, SEL)
pub fn stub_0xef845c() -> ! {
    todo!("0xef845c -[PLCrashReportThreadInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportThreadInfo threadNumber]")]
// 0xef84c0 — -[PLCrashReportThreadInfo threadNumber]
// type: int __cdecl(PLCrashReportThreadInfo *self, SEL)
pub fn stub_0xef84c0() -> ! {
    todo!("0xef84c0 -[PLCrashReportThreadInfo threadNumber]")
}

#[doc(alias = "-[PLCrashReportThreadInfo stackFrames]")]
// 0xef84d0 — -[PLCrashReportThreadInfo stackFrames]
// type: NSArray *__cdecl(PLCrashReportThreadInfo *self, SEL)
pub fn stub_0xef84d0() -> ! {
    todo!("0xef84d0 -[PLCrashReportThreadInfo stackFrames]")
}

#[doc(alias = "-[PLCrashReportThreadInfo crashed]")]
// 0xef84e0 — -[PLCrashReportThreadInfo crashed]
// type: char __cdecl(PLCrashReportThreadInfo *self, SEL)
pub fn stub_0xef84e0() -> ! {
    todo!("0xef84e0 -[PLCrashReportThreadInfo crashed]")
}

#[doc(alias = "-[PLCrashReportThreadInfo registers]")]
// 0xef84f0 — -[PLCrashReportThreadInfo registers]
// type: NSArray *__cdecl(PLCrashReportThreadInfo *self, SEL)
pub fn stub_0xef84f0() -> ! {
    todo!("0xef84f0 -[PLCrashReportThreadInfo registers]")
}

#[doc(alias = "-[PLCrashReportStackFrameInfo initWithInstructionPointer:]")]
// 0xef8500 — -[PLCrashReportStackFrameInfo initWithInstructionPointer:]
// type: PLCrashReportStackFrameInfo *__cdecl(PLCrashReportStackFrameInfo *self, SEL, unsigned __int64)
pub fn stub_0xef8500() -> ! {
    todo!("0xef8500 -[PLCrashReportStackFrameInfo initWithInstructionPointer:]")
}

#[doc(alias = "-[PLCrashReportStackFrameInfo instructionPointer]")]
// 0xef854c — -[PLCrashReportStackFrameInfo instructionPointer]
// type: unsigned __int64 __cdecl(PLCrashReportStackFrameInfo *self, SEL)
pub fn stub_0xef854c() -> ! {
    todo!("0xef854c -[PLCrashReportStackFrameInfo instructionPointer]")
}

#[doc(alias = "-[PLCrashReportRegisterInfo initWithRegisterName:registerValue:]")]
// 0xef8564 — -[PLCrashReportRegisterInfo initWithRegisterName:registerValue:]
// type: PLCrashReportRegisterInfo *__cdecl(PLCrashReportRegisterInfo *self, SEL, id, unsigned __int64)
pub fn stub_0xef8564() -> ! {
    todo!("0xef8564 -[PLCrashReportRegisterInfo initWithRegisterName:registerValue:]")
}

#[doc(alias = "-[PLCrashReportRegisterInfo dealloc]")]
// 0xef85dc — -[PLCrashReportRegisterInfo dealloc]
// type: void __cdecl(PLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xef85dc() -> ! {
    todo!("0xef85dc -[PLCrashReportRegisterInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportRegisterInfo registerName]")]
// 0xef8628 — -[PLCrashReportRegisterInfo registerName]
// type: NSString *__cdecl(PLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xef8628() -> ! {
    todo!("0xef8628 -[PLCrashReportRegisterInfo registerName]")
}

#[doc(alias = "-[PLCrashReportRegisterInfo registerValue]")]
// 0xef8638 — -[PLCrashReportRegisterInfo registerValue]
// type: unsigned __int64 __cdecl(PLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xef8638() -> ! {
    todo!("0xef8638 -[PLCrashReportRegisterInfo registerValue]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo initWithCodeType:baseAddress:size:name:uuid:]")]
// 0xef8650 — -[PLCrashReportBinaryImageInfo initWithCodeType:baseAddress:size:name:uuid:]
// type: PLCrashReportBinaryImageInfo *__cdecl(PLCrashReportBinaryImageInfo *self, SEL, id, unsigned __int64, unsigned __int64, id, id)
pub fn stub_0xef8650() -> ! {
    todo!("0xef8650 -[PLCrashReportBinaryImageInfo initWithCodeType:baseAddress:size:name:uuid:]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo dealloc]")]
// 0xef87bc — -[PLCrashReportBinaryImageInfo dealloc]
// type: void __cdecl(PLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xef87bc() -> ! {
    todo!("0xef87bc -[PLCrashReportBinaryImageInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo codeType]")]
// 0xef8834 — -[PLCrashReportBinaryImageInfo codeType]
// type: PLCrashReportProcessorInfo *__cdecl(PLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xef8834() -> ! {
    todo!("0xef8834 -[PLCrashReportBinaryImageInfo codeType]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo imageBaseAddress]")]
// 0xef8844 — -[PLCrashReportBinaryImageInfo imageBaseAddress]
// type: unsigned __int64 __cdecl(PLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xef8844() -> ! {
    todo!("0xef8844 -[PLCrashReportBinaryImageInfo imageBaseAddress]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo imageSize]")]
// 0xef885c — -[PLCrashReportBinaryImageInfo imageSize]
// type: unsigned __int64 __cdecl(PLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xef885c() -> ! {
    todo!("0xef885c -[PLCrashReportBinaryImageInfo imageSize]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo imageName]")]
// 0xef8874 — -[PLCrashReportBinaryImageInfo imageName]
// type: NSString *__cdecl(PLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xef8874() -> ! {
    todo!("0xef8874 -[PLCrashReportBinaryImageInfo imageName]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo hasImageUUID]")]
// 0xef8884 — -[PLCrashReportBinaryImageInfo hasImageUUID]
// type: char __cdecl(PLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xef8884() -> ! {
    todo!("0xef8884 -[PLCrashReportBinaryImageInfo hasImageUUID]")
}

#[doc(alias = "-[PLCrashReportBinaryImageInfo imageUUID]")]
// 0xef8894 — -[PLCrashReportBinaryImageInfo imageUUID]
// type: NSString *__cdecl(PLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xef8894() -> ! {
    todo!("0xef8894 -[PLCrashReportBinaryImageInfo imageUUID]")
}

#[doc(alias = "-[PLCrashReportExceptionInfo initWithExceptionName:reason:]")]
// 0xef88a4 — -[PLCrashReportExceptionInfo initWithExceptionName:reason:]
// type: PLCrashReportExceptionInfo *__cdecl(PLCrashReportExceptionInfo *self, SEL, id, id)
pub fn stub_0xef88a4() -> ! {
    todo!("0xef88a4 -[PLCrashReportExceptionInfo initWithExceptionName:reason:]")
}

#[doc(alias = "-[PLCrashReportExceptionInfo initWithExceptionName:reason:stackFrames:]")]
// 0xef88c8 — -[PLCrashReportExceptionInfo initWithExceptionName:reason:stackFrames:]
// type: PLCrashReportExceptionInfo *__cdecl(PLCrashReportExceptionInfo *self, SEL, id, id, id)
pub fn stub_0xef88c8() -> ! {
    todo!("0xef88c8 -[PLCrashReportExceptionInfo initWithExceptionName:reason:stackFrames:]")
}

#[doc(alias = "-[PLCrashReportExceptionInfo dealloc]")]
// 0xef895c — -[PLCrashReportExceptionInfo dealloc]
// type: void __cdecl(PLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xef895c() -> ! {
    todo!("0xef895c -[PLCrashReportExceptionInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportExceptionInfo exceptionName]")]
// 0xef89d4 — -[PLCrashReportExceptionInfo exceptionName]
// type: NSString *__cdecl(PLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xef89d4() -> ! {
    todo!("0xef89d4 -[PLCrashReportExceptionInfo exceptionName]")
}

#[doc(alias = "-[PLCrashReportExceptionInfo exceptionReason]")]
// 0xef89e4 — -[PLCrashReportExceptionInfo exceptionReason]
// type: NSString *__cdecl(PLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xef89e4() -> ! {
    todo!("0xef89e4 -[PLCrashReportExceptionInfo exceptionReason]")
}

#[doc(alias = "-[PLCrashReportExceptionInfo stackFrames]")]
// 0xef89f4 — -[PLCrashReportExceptionInfo stackFrames]
// type: NSArray *__cdecl(PLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xef89f4() -> ! {
    todo!("0xef89f4 -[PLCrashReportExceptionInfo stackFrames]")
}

#[doc(alias = "_plcrash_async_signal_sigcode")]
// 0xef8a04 — _plcrash_async_signal_sigcode
pub fn stub_0xef8a04() -> ! {
    todo!("0xef8a04 _plcrash_async_signal_sigcode")
}

#[doc(alias = "_plcrash_async_signal_signame")]
// 0xef8a48 — _plcrash_async_signal_signame
pub fn stub_0xef8a48() -> ! {
    todo!("0xef8a48 _plcrash_async_signal_signame")
}

#[doc(alias = "-[PLCrashReportSignalInfo initWithSignalName:code:address:]")]
// 0xef8a70 — -[PLCrashReportSignalInfo initWithSignalName:code:address:]
// type: PLCrashReportSignalInfo *__cdecl(PLCrashReportSignalInfo *self, SEL, id, id, unsigned __int64)
pub fn stub_0xef8a70() -> ! {
    todo!("0xef8a70 -[PLCrashReportSignalInfo initWithSignalName:code:address:]")
}

#[doc(alias = "-[PLCrashReportSignalInfo dealloc]")]
// 0xef8b08 — -[PLCrashReportSignalInfo dealloc]
// type: void __cdecl(PLCrashReportSignalInfo *self, SEL)
pub fn stub_0xef8b08() -> ! {
    todo!("0xef8b08 -[PLCrashReportSignalInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportSignalInfo name]")]
// 0xef8b6c — -[PLCrashReportSignalInfo name]
// type: NSString *__cdecl(PLCrashReportSignalInfo *self, SEL)
pub fn stub_0xef8b6c() -> ! {
    todo!("0xef8b6c -[PLCrashReportSignalInfo name]")
}

#[doc(alias = "-[PLCrashReportSignalInfo code]")]
// 0xef8b7c — -[PLCrashReportSignalInfo code]
// type: NSString *__cdecl(PLCrashReportSignalInfo *self, SEL)
pub fn stub_0xef8b7c() -> ! {
    todo!("0xef8b7c -[PLCrashReportSignalInfo code]")
}

#[doc(alias = "-[PLCrashReportSignalInfo address]")]
// 0xef8b8c — -[PLCrashReportSignalInfo address]
// type: unsigned __int64 __cdecl(PLCrashReportSignalInfo *self, SEL)
pub fn stub_0xef8b8c() -> ! {
    todo!("0xef8b8c -[PLCrashReportSignalInfo address]")
}

#[doc(alias = "-[PLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]")]
// 0xef8ba4 — -[PLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]
// type: PLCrashReportProcessInfo *__cdecl(PLCrashReportProcessInfo *self, SEL, id, unsigned int, id, id, unsigned int, char)
pub fn stub_0xef8ba4() -> ! {
    todo!("0xef8ba4 -[PLCrashReportProcessInfo initWithProcessName:processID:processPath:parentProcessName:parentProcessID:native:]")
}

#[doc(alias = "-[PLCrashReportProcessInfo dealloc]")]
// 0xef8c6c — -[PLCrashReportProcessInfo dealloc]
// type: void __cdecl(PLCrashReportProcessInfo *self, SEL)
pub fn stub_0xef8c6c() -> ! {
    todo!("0xef8c6c -[PLCrashReportProcessInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportProcessInfo processName]")]
// 0xef8ce4 — -[PLCrashReportProcessInfo processName]
// type: NSString *__cdecl(PLCrashReportProcessInfo *self, SEL)
pub fn stub_0xef8ce4() -> ! {
    todo!("0xef8ce4 -[PLCrashReportProcessInfo processName]")
}

#[doc(alias = "-[PLCrashReportProcessInfo processID]")]
// 0xef8cf4 — -[PLCrashReportProcessInfo processID]
// type: unsigned int __cdecl(PLCrashReportProcessInfo *self, SEL)
pub fn stub_0xef8cf4() -> ! {
    todo!("0xef8cf4 -[PLCrashReportProcessInfo processID]")
}

#[doc(alias = "-[PLCrashReportProcessInfo processPath]")]
// 0xef8d04 — -[PLCrashReportProcessInfo processPath]
// type: NSString *__cdecl(PLCrashReportProcessInfo *self, SEL)
pub fn stub_0xef8d04() -> ! {
    todo!("0xef8d04 -[PLCrashReportProcessInfo processPath]")
}

#[doc(alias = "-[PLCrashReportProcessInfo parentProcessName]")]
// 0xef8d14 — -[PLCrashReportProcessInfo parentProcessName]
// type: NSString *__cdecl(PLCrashReportProcessInfo *self, SEL)
pub fn stub_0xef8d14() -> ! {
    todo!("0xef8d14 -[PLCrashReportProcessInfo parentProcessName]")
}

#[doc(alias = "-[PLCrashReportProcessInfo parentProcessID]")]
// 0xef8d24 — -[PLCrashReportProcessInfo parentProcessID]
// type: unsigned int __cdecl(PLCrashReportProcessInfo *self, SEL)
pub fn stub_0xef8d24() -> ! {
    todo!("0xef8d24 -[PLCrashReportProcessInfo parentProcessID]")
}

#[doc(alias = "-[PLCrashReportProcessInfo native]")]
// 0xef8d34 — -[PLCrashReportProcessInfo native]
// type: char __cdecl(PLCrashReportProcessInfo *self, SEL)
pub fn stub_0xef8d34() -> ! {
    todo!("0xef8d34 -[PLCrashReportProcessInfo native]")
}

#[doc(alias = "+[PLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]")]
// 0xef8d44 — +[PLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]
// type: id __cdecl(id, SEL, id, int)
pub fn stub_0xef8d44() -> ! {
    todo!("0xef8d44 +[PLCrashReportTextFormatter stringValueForCrashReport:withTextFormat:]")
}

#[doc(alias = "_binaryImageSort")]
// 0xef9e10 — _binaryImageSort
pub fn stub_0xef9e10() -> ! {
    todo!("0xef9e10 _binaryImageSort")
}

#[doc(alias = "-[PLCrashReportTextFormatter initWithTextFormat:stringEncoding:]")]
// 0xef9e78 — -[PLCrashReportTextFormatter initWithTextFormat:stringEncoding:]
// type: PLCrashReportTextFormatter *__cdecl(PLCrashReportTextFormatter *self, SEL, int, unsigned int)
pub fn stub_0xef9e78() -> ! {
    todo!("0xef9e78 -[PLCrashReportTextFormatter initWithTextFormat:stringEncoding:]")
}

#[doc(alias = "-[PLCrashReportTextFormatter formatReport:error:]")]
// 0xef9ecc — -[PLCrashReportTextFormatter formatReport:error:]
// type: id __cdecl(PLCrashReportTextFormatter *self, SEL, id, id *)
pub fn stub_0xef9ecc() -> ! {
    todo!("0xef9ecc -[PLCrashReportTextFormatter formatReport:error:]")
}

#[doc(alias = "+[PLCrashReportTextFormatter formatStackFrame:frameIndex:report:]")]
// 0xef9f20 — +[PLCrashReportTextFormatter formatStackFrame:frameIndex:report:]
// type: id __cdecl(id, SEL, id, unsigned int, id)
pub fn stub_0xef9f20() -> ! {
    todo!("0xef9f20 +[PLCrashReportTextFormatter formatStackFrame:frameIndex:report:]")
}

#[doc(alias = "_plcrash_async_image_list_init")]
// 0xefa040 — _plcrash_async_image_list_init
pub fn stub_0xefa040() -> ! {
    todo!("0xefa040 _plcrash_async_image_list_init")
}

#[doc(alias = "_plcrash_async_image_list_free")]
// 0xefa050 — _plcrash_async_image_list_free
pub fn stub_0xefa050() -> ! {
    todo!("0xefa050 _plcrash_async_image_list_free")
}

#[doc(alias = "_plcrash_async_image_list_append")]
// 0xefa078 — _plcrash_async_image_list_append
pub fn stub_0xefa078() -> ! {
    todo!("0xefa078 _plcrash_async_image_list_append")
}

#[doc(alias = "_plcrash_async_image_list_remove")]
// 0xefa0d8 — _plcrash_async_image_list_remove
pub fn stub_0xefa0d8() -> ! {
    todo!("0xefa0d8 _plcrash_async_image_list_remove")
}

#[doc(alias = "_plcrash_async_image_list_set_reading")]
// 0xefa154 — _plcrash_async_image_list_set_reading
pub fn stub_0xefa154() -> ! {
    todo!("0xefa154 _plcrash_async_image_list_set_reading")
}

#[doc(alias = "_plcrash_async_image_list_next")]
// 0xefa170 — _plcrash_async_image_list_next
pub fn stub_0xefa170() -> ! {
    todo!("0xefa170 _plcrash_async_image_list_next")
}

#[doc(alias = "-[PLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]")]
// 0xefa180 — -[PLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]
// type: PLCrashReportProcessorInfo *__cdecl(PLCrashReportProcessorInfo *self, SEL, int, unsigned __int64, unsigned __int64)
pub fn stub_0xefa180() -> ! {
    todo!("0xefa180 -[PLCrashReportProcessorInfo initWithTypeEncoding:type:subtype:]")
}

#[doc(alias = "-[PLCrashReportProcessorInfo typeEncoding]")]
// 0xefa1f8 — -[PLCrashReportProcessorInfo typeEncoding]
// type: int __cdecl(PLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xefa1f8() -> ! {
    todo!("0xefa1f8 -[PLCrashReportProcessorInfo typeEncoding]")
}

#[doc(alias = "-[PLCrashReportProcessorInfo type]")]
// 0xefa208 — -[PLCrashReportProcessorInfo type]
// type: unsigned __int64 __cdecl(PLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xefa208() -> ! {
    todo!("0xefa208 -[PLCrashReportProcessorInfo type]")
}

#[doc(alias = "-[PLCrashReportProcessorInfo subtype]")]
// 0xefa220 — -[PLCrashReportProcessorInfo subtype]
// type: unsigned __int64 __cdecl(PLCrashReportProcessorInfo *self, SEL)
pub fn stub_0xefa220() -> ! {
    todo!("0xefa220 -[PLCrashReportProcessorInfo subtype]")
}

#[doc(alias = "-[PLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]")]
// 0xefa238 — -[PLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]
// type: PLCrashReportMachineInfo *__cdecl(PLCrashReportMachineInfo *self, SEL, id, id, unsigned int, unsigned int)
pub fn stub_0xefa238() -> ! {
    todo!("0xefa238 -[PLCrashReportMachineInfo initWithModelName:processorInfo:processorCount:logicalProcessorCount:]")
}

#[doc(alias = "-[PLCrashReportMachineInfo dealloc]")]
// 0xefa2d8 — -[PLCrashReportMachineInfo dealloc]
// type: void __cdecl(PLCrashReportMachineInfo *self, SEL)
pub fn stub_0xefa2d8() -> ! {
    todo!("0xefa2d8 -[PLCrashReportMachineInfo dealloc]")
}

#[doc(alias = "-[PLCrashReportMachineInfo modelName]")]
// 0xefa33c — -[PLCrashReportMachineInfo modelName]
// type: NSString *__cdecl(PLCrashReportMachineInfo *self, SEL)
pub fn stub_0xefa33c() -> ! {
    todo!("0xefa33c -[PLCrashReportMachineInfo modelName]")
}

#[doc(alias = "-[PLCrashReportMachineInfo processorInfo]")]
// 0xefa34c — -[PLCrashReportMachineInfo processorInfo]
// type: PLCrashReportProcessorInfo *__cdecl(PLCrashReportMachineInfo *self, SEL)
pub fn stub_0xefa34c() -> ! {
    todo!("0xefa34c -[PLCrashReportMachineInfo processorInfo]")
}

#[doc(alias = "-[PLCrashReportMachineInfo processorCount]")]
// 0xefa35c — -[PLCrashReportMachineInfo processorCount]
// type: unsigned int __cdecl(PLCrashReportMachineInfo *self, SEL)
pub fn stub_0xefa35c() -> ! {
    todo!("0xefa35c -[PLCrashReportMachineInfo processorCount]")
}

#[doc(alias = "-[PLCrashReportMachineInfo logicalProcessorCount]")]
// 0xefa36c — -[PLCrashReportMachineInfo logicalProcessorCount]
// type: unsigned int __cdecl(PLCrashReportMachineInfo *self, SEL)
pub fn stub_0xefa36c() -> ! {
    todo!("0xefa36c -[PLCrashReportMachineInfo logicalProcessorCount]")
}

#[doc(alias = "_plcrash_sysctl_string")]
// 0xefa37c — _plcrash_sysctl_string
// type: int __fastcall(char *)
pub fn stub_0xefa37c() -> ! {
    todo!("0xefa37c _plcrash_sysctl_string")
}

#[doc(alias = "_plcrash_sysctl_int")]
// 0xefa400 — _plcrash_sysctl_int
pub fn stub_0xefa400() -> ! {
    todo!("0xefa400 _plcrash_sysctl_int")
}

#[doc(alias = "_plcrash_log_writer_write_curthread")]
// 0xefa430 — _plcrash_log_writer_write_curthread
pub fn stub_0xefa430() -> ! {
    todo!("0xefa430 _plcrash_log_writer_write_curthread")
}

#[doc(alias = "_plcrash_log_writer_write_curthread_stub")]
// 0xefa490 — _plcrash_log_writer_write_curthread_stub
pub fn stub_0xefa490() -> ! {
    todo!("0xefa490 _plcrash_log_writer_write_curthread_stub")
}

#[doc(alias = "_plcrash_populate_error")]
// 0xefa4fc — _plcrash_populate_error
pub fn stub_0xefa4fc() -> ! {
    todo!("0xefa4fc _plcrash_populate_error")
}
