//! core shard mm — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 2773 uncovered before -> 2673 after, batch 0xf15e5c..0xf1a650).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, unstable_name_collisions, clippy::all, unused_attributes)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_uncaught_exception_handler_0")]
// 0xf15e5c — _uncaught_exception_handler_0
// type: void __fastcall __noreturn(id)
pub fn stub_0xf15e5c() -> ! { todo!("0xf15e5c _uncaught_exception_handler_0") }

#[doc(alias = "-[FlurryPLCrashReporter generateLiveReportWithThread:]")]
// 0xf15e78 — -[FlurryPLCrashReporter generateLiveReportWithThread:]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL, unsigned int)
pub fn stub_0xf15e78() -> ! { todo!("0xf15e78 -[FlurryPLCrashReporter generateLiveReportWithThread:]") }

#[doc(alias = "-[FlurryPLCrashReporter generateLiveReportWithThread:error:]")]
// 0xf15e90 — -[FlurryPLCrashReporter generateLiveReportWithThread:error:]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL, unsigned int, id *)
pub fn stub_0xf15e90() -> ! { todo!("0xf15e90 -[FlurryPLCrashReporter generateLiveReportWithThread:error:]") }

#[doc(alias = "-[FlurryPLCrashReporter generateLiveReport]")]
// 0xf160dc — -[FlurryPLCrashReporter generateLiveReport]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf160dc() -> ! { todo!("0xf160dc -[FlurryPLCrashReporter generateLiveReport]") }

#[doc(alias = "-[FlurryPLCrashReporter generateLiveReportAndReturnError:]")]
// 0xf160f4 — -[FlurryPLCrashReporter generateLiveReportAndReturnError:]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf160f4() -> ! { todo!("0xf160f4 -[FlurryPLCrashReporter generateLiveReportAndReturnError:]") }

#[doc(alias = "-[FlurryPLCrashReporter setCrashCallbacks:]")]
// 0xf16118 — -[FlurryPLCrashReporter setCrashCallbacks:]
// type: void __cdecl(FlurryPLCrashReporter *self, SEL, PLCrashReporterCallbacks *)
pub fn stub_0xf16118() -> ! { todo!("0xf16118 -[FlurryPLCrashReporter setCrashCallbacks:]") }

#[doc(alias = "-[FlurryPLCrashReporter initWithApplicationIdentifier:appVersion:]")]
// 0xf161a0 — -[FlurryPLCrashReporter initWithApplicationIdentifier:appVersion:]
// type: FlurryPLCrashReporter *__cdecl(FlurryPLCrashReporter *self, SEL, id, id)
pub fn stub_0xf161a0() -> ! { todo!("0xf161a0 -[FlurryPLCrashReporter initWithApplicationIdentifier:appVersion:]") }

#[doc(alias = "-[FlurryPLCrashReporter initWithBundle:]")]
// 0xf162b8 — -[FlurryPLCrashReporter initWithBundle:]
// type: FlurryPLCrashReporter *__cdecl(FlurryPLCrashReporter *self, SEL, id)
pub fn stub_0xf162b8() -> ! { todo!("0xf162b8 -[FlurryPLCrashReporter initWithBundle:]") }

#[doc(alias = "-[FlurryPLCrashReporter dealloc]")]
// 0xf163bc — -[FlurryPLCrashReporter dealloc]
// type: void __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf163bc() -> ! { todo!("0xf163bc -[FlurryPLCrashReporter dealloc]") }

#[doc(alias = "-[FlurryPLCrashReporter populateCrashReportDirectoryAndReturnError:]")]
// 0xf16434 — -[FlurryPLCrashReporter populateCrashReportDirectoryAndReturnError:]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf16434() -> ! { todo!("0xf16434 -[FlurryPLCrashReporter populateCrashReportDirectoryAndReturnError:]") }

#[doc(alias = "-[FlurryPLCrashReporter crashReportDirectory]")]
// 0xf1656c — -[FlurryPLCrashReporter crashReportDirectory]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf1656c() -> ! { todo!("0xf1656c -[FlurryPLCrashReporter crashReportDirectory]") }

#[doc(alias = "-[FlurryPLCrashReporter queuedCrashReportDirectory]")]
// 0xf1657c — -[FlurryPLCrashReporter queuedCrashReportDirectory]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf1657c() -> ! { todo!("0xf1657c -[FlurryPLCrashReporter queuedCrashReportDirectory]") }

#[doc(alias = "-[FlurryPLCrashReporter crashReportPath]")]
// 0xf165ac — -[FlurryPLCrashReporter crashReportPath]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf165ac() -> ! { todo!("0xf165ac -[FlurryPLCrashReporter crashReportPath]") }

#[doc(alias = "-[FlurryPLCrashReport initWithData:error:]")]
// 0xf165dc — -[FlurryPLCrashReport initWithData:error:]
// type: FlurryPLCrashReport *__cdecl(FlurryPLCrashReport *self, SEL, id, id *)
pub fn stub_0xf165dc() -> ! { todo!("0xf165dc -[FlurryPLCrashReport initWithData:error:]") }

#[doc(alias = "_populate_nserror_0")]
// 0xf16844 — _populate_nserror_0
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf16844() -> ! { todo!("0xf16844 _populate_nserror_0") }

#[doc(alias = "-[FlurryPLCrashReport dealloc]")]
// 0xf168b4 — -[FlurryPLCrashReport dealloc]
// type: void __cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf168b4() -> ! { todo!("0xf168b4 -[FlurryPLCrashReport dealloc]") }

#[doc(alias = "-[FlurryPLCrashReport imageForAddress:]")]
// 0xf169c0 — -[FlurryPLCrashReport imageForAddress:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, unsigned __int64)
pub fn stub_0xf169c0() -> ! { todo!("0xf169c0 -[FlurryPLCrashReport imageForAddress:]") }

#[doc(alias = "-[FlurryPLCrashReport hasMachineInfo]")]
// 0xf16b00 — -[FlurryPLCrashReport hasMachineInfo]
// type: char __cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b00() -> ! { todo!("0xf16b00 -[FlurryPLCrashReport hasMachineInfo]") }

#[doc(alias = "-[FlurryPLCrashReport hasProcessInfo]")]
// 0xf16b18 — -[FlurryPLCrashReport hasProcessInfo]
// type: char __cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b18() -> ! { todo!("0xf16b18 -[FlurryPLCrashReport hasProcessInfo]") }

#[doc(alias = "-[FlurryPLCrashReport hasExceptionInfo]")]
// 0xf16b30 — -[FlurryPLCrashReport hasExceptionInfo]
// type: char __cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b30() -> ! { todo!("0xf16b30 -[FlurryPLCrashReport hasExceptionInfo]") }

#[doc(alias = "-[FlurryPLCrashReport systemInfo]")]
// 0xf16b48 — -[FlurryPLCrashReport systemInfo]
// type: FlurryPLCrashReportSystemInfo *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b48() -> ! { todo!("0xf16b48 -[FlurryPLCrashReport systemInfo]") }

#[doc(alias = "-[FlurryPLCrashReport machineInfo]")]
// 0xf16b58 — -[FlurryPLCrashReport machineInfo]
// type: FlurryPLCrashReportMachineInfo *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b58() -> ! { todo!("0xf16b58 -[FlurryPLCrashReport machineInfo]") }

#[doc(alias = "-[FlurryPLCrashReport applicationInfo]")]
// 0xf16b68 — -[FlurryPLCrashReport applicationInfo]
// type: FlurryPLCrashReportApplicationInfo *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b68() -> ! { todo!("0xf16b68 -[FlurryPLCrashReport applicationInfo]") }

#[doc(alias = "-[FlurryPLCrashReport processInfo]")]
// 0xf16b78 — -[FlurryPLCrashReport processInfo]
// type: FlurryPLCrashReportProcessInfo *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b78() -> ! { todo!("0xf16b78 -[FlurryPLCrashReport processInfo]") }

#[doc(alias = "-[FlurryPLCrashReport signalInfo]")]
// 0xf16b88 — -[FlurryPLCrashReport signalInfo]
// type: FlurryPLCrashReportSignalInfo *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b88() -> ! { todo!("0xf16b88 -[FlurryPLCrashReport signalInfo]") }

#[doc(alias = "-[FlurryPLCrashReport threads]")]
// 0xf16b98 — -[FlurryPLCrashReport threads]
// type: NSArray *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16b98() -> ! { todo!("0xf16b98 -[FlurryPLCrashReport threads]") }

#[doc(alias = "-[FlurryPLCrashReport images]")]
// 0xf16ba8 — -[FlurryPLCrashReport images]
// type: NSArray *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16ba8() -> ! { todo!("0xf16ba8 -[FlurryPLCrashReport images]") }

#[doc(alias = "-[FlurryPLCrashReport exceptionInfo]")]
// 0xf16bb8 — -[FlurryPLCrashReport exceptionInfo]
// type: FlurryPLCrashReportExceptionInfo *__cdecl(FlurryPLCrashReport *self, SEL)
pub fn stub_0xf16bb8() -> ! { todo!("0xf16bb8 -[FlurryPLCrashReport exceptionInfo]") }

#[doc(alias = "-[FlurryPLCrashReport decodeCrashData:error:]")]
// 0xf16bc8 — -[FlurryPLCrashReport decodeCrashData:error:]
// type: _Plcrash__CrashReport *__cdecl(FlurryPLCrashReport *self, SEL, id, id *)
pub fn stub_0xf16bc8() -> ! { todo!("0xf16bc8 -[FlurryPLCrashReport decodeCrashData:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractSystemInfo:error:]")]
// 0xf16d64 — -[FlurryPLCrashReport extractSystemInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__SystemInfo *, id *)
pub fn stub_0xf16d64() -> ! { todo!("0xf16d64 -[FlurryPLCrashReport extractSystemInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractProcessorInfo:error:]")]
// 0xf16ed4 — -[FlurryPLCrashReport extractProcessorInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__Processor *, id *)
pub fn stub_0xf16ed4() -> ! { todo!("0xf16ed4 -[FlurryPLCrashReport extractProcessorInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractMachineInfo:error:]")]
// 0xf16f88 — -[FlurryPLCrashReport extractMachineInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__MachineInfo *, id *)
pub fn stub_0xf16f88() -> ! { todo!("0xf16f88 -[FlurryPLCrashReport extractMachineInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractApplicationInfo:error:]")]
// 0xf17088 — -[FlurryPLCrashReport extractApplicationInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__ApplicationInfo *, id *)
pub fn stub_0xf17088() -> ! { todo!("0xf17088 -[FlurryPLCrashReport extractApplicationInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractProcessInfo:error:]")]
// 0xf171d4 — -[FlurryPLCrashReport extractProcessInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__ProcessInfo *, id *)
pub fn stub_0xf171d4() -> ! { todo!("0xf171d4 -[FlurryPLCrashReport extractProcessInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractSymbolInfo:error:]")]
// 0xf17308 — -[FlurryPLCrashReport extractSymbolInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__Symbol *, id *)
pub fn stub_0xf17308() -> ! { todo!("0xf17308 -[FlurryPLCrashReport extractSymbolInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractStackFrameInfo:error:]")]
// 0xf173e8 — -[FlurryPLCrashReport extractStackFrameInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__Thread__StackFrame *, id *)
pub fn stub_0xf173e8() -> ! { todo!("0xf173e8 -[FlurryPLCrashReport extractStackFrameInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractThreadInfo:error:]")]
// 0xf174b8 — -[FlurryPLCrashReport extractThreadInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport *, id *)
pub fn stub_0xf174b8() -> ! { todo!("0xf174b8 -[FlurryPLCrashReport extractThreadInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractImageInfo:error:]")]
// 0xf176fc — -[FlurryPLCrashReport extractImageInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport *, id *)
pub fn stub_0xf176fc() -> ! { todo!("0xf176fc -[FlurryPLCrashReport extractImageInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractExceptionInfo:error:]")]
// 0xf17910 — -[FlurryPLCrashReport extractExceptionInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__Exception *, id *)
pub fn stub_0xf17910() -> ! { todo!("0xf17910 -[FlurryPLCrashReport extractExceptionInfo:error:]") }

#[doc(alias = "-[FlurryPLCrashReport extractSignalInfo:error:]")]
// 0xf17aec — -[FlurryPLCrashReport extractSignalInfo:error:]
// type: id __cdecl(FlurryPLCrashReport *self, SEL, _Plcrash__CrashReport__Signal *, id *)
pub fn stub_0xf17aec() -> ! { todo!("0xf17aec -[FlurryPLCrashReport extractSignalInfo:error:]") }

#[doc(alias = "_plcrash__crash_report__init_0")]
// 0xf17c40 — _plcrash__crash_report__init_0
pub fn stub_0xf17c40() -> ! { todo!("0xf17c40 _plcrash__crash_report__init_0") }

#[doc(alias = "_plcrash__crash_report__get_packed_size_0")]
// 0xf17c8c — _plcrash__crash_report__get_packed_size_0
pub fn stub_0xf17c8c() -> ! { todo!("0xf17c8c _plcrash__crash_report__get_packed_size_0") }

#[doc(alias = "_plcrash__crash_report__pack_0")]
// 0xf17ccc — _plcrash__crash_report__pack_0
pub fn stub_0xf17ccc() -> ! { todo!("0xf17ccc _plcrash__crash_report__pack_0") }

#[doc(alias = "_plcrash__crash_report__pack_to_buffer_0")]
// 0xf17d0c — _plcrash__crash_report__pack_to_buffer_0
pub fn stub_0xf17d0c() -> ! { todo!("0xf17d0c _plcrash__crash_report__pack_to_buffer_0") }

#[doc(alias = "_plcrash__crash_report__unpack_0")]
// 0xf17d4c — _plcrash__crash_report__unpack_0
pub fn stub_0xf17d4c() -> ! { todo!("0xf17d4c _plcrash__crash_report__unpack_0") }

#[doc(alias = "_plcrash__crash_report__free_unpacked_0")]
// 0xf17d68 — _plcrash__crash_report__free_unpacked_0
pub fn stub_0xf17d68() -> ! { todo!("0xf17d68 _plcrash__crash_report__free_unpacked_0") }

#[doc(alias = "_protobuf_c_out_of_memory_default_0")]
// 0xf17da8 — _protobuf_c_out_of_memory_default_0
pub fn stub_0xf17da8() -> ! { todo!("0xf17da8 _protobuf_c_out_of_memory_default_0") }

#[doc(alias = "_system_alloc_0")]
// 0xf17dd0 — _system_alloc_0
// type: int __fastcall(int, size_t __size)
pub fn stub_0xf17dd0() -> ! { todo!("0xf17dd0 _system_alloc_0") }

#[doc(alias = "_system_free_0")]
// 0xf17df8 — _system_free_0
// type: int __fastcall(int, void *)
pub fn stub_0xf17df8() -> ! { todo!("0xf17df8 _system_free_0") }

#[doc(alias = "_protobuf_c_buffer_simple_append_0")]
// 0xf17e08 — _protobuf_c_buffer_simple_append_0
// type: int __fastcall(int, size_t __n)
pub fn stub_0xf17e08() -> ! { todo!("0xf17e08 _protobuf_c_buffer_simple_append_0") }

#[doc(alias = "_protobuf_c_message_get_packed_size_0")]
// 0xf17e84 — _protobuf_c_message_get_packed_size_0
pub fn stub_0xf17e84() -> ! { todo!("0xf17e84 _protobuf_c_message_get_packed_size_0") }

#[doc(alias = "_required_field_get_packed_size_0")]
// 0xf18228 — _required_field_get_packed_size_0
pub fn stub_0xf18228() -> ! { todo!("0xf18228 _required_field_get_packed_size_0") }

#[doc(alias = "_protobuf_c_message_pack_0")]
// 0xf18400 — _protobuf_c_message_pack_0
pub fn stub_0xf18400() -> ! { todo!("0xf18400 _protobuf_c_message_pack_0") }

#[doc(alias = "_required_field_pack_0")]
// 0xf185bc — _required_field_pack_0
pub fn stub_0xf185bc() -> ! { todo!("0xf185bc _required_field_pack_0") }

#[doc(alias = "_protobuf_c_message_pack_to_buffer_0")]
// 0xf188ec — _protobuf_c_message_pack_to_buffer_0
pub fn stub_0xf188ec() -> ! { todo!("0xf188ec _protobuf_c_message_pack_to_buffer_0") }

#[doc(alias = "_required_field_pack_to_buffer_0")]
// 0xf18abc — _required_field_pack_to_buffer_0
pub fn stub_0xf18abc() -> ! { todo!("0xf18abc _required_field_pack_to_buffer_0") }

#[doc(alias = "_protobuf_c_message_unpack_0")]
// 0xf18ed8 — _protobuf_c_message_unpack_0
pub fn stub_0xf18ed8() -> ! { todo!("0xf18ed8 _protobuf_c_message_unpack_0") }

#[doc(alias = "_protobuf_c_message_free_unpacked_0")]
// 0xf195dc — _protobuf_c_message_free_unpacked_0
pub fn stub_0xf195dc() -> ! { todo!("0xf195dc _protobuf_c_message_free_unpacked_0") }

#[doc(alias = "_protobuf_c_service_generated_init_0")]
// 0xf19758 — _protobuf_c_service_generated_init_0
pub fn stub_0xf19758() -> ! { todo!("0xf19758 _protobuf_c_service_generated_init_0") }

#[doc(alias = "_service_machgen_invoke_0")]
// 0xf197b0 — _service_machgen_invoke_0
// type: int __fastcall(_DWORD *, unsigned int)
pub fn stub_0xf197b0() -> ! { todo!("0xf197b0 _service_machgen_invoke_0") }

#[doc(alias = "_protobuf_c_service_destroy_0")]
// 0xf197fc — _protobuf_c_service_destroy_0
pub fn stub_0xf197fc() -> ! { todo!("0xf197fc _protobuf_c_service_destroy_0") }

#[doc(alias = "_protobuf_c_enum_descriptor_get_value_by_name_0")]
// 0xf19808 — _protobuf_c_enum_descriptor_get_value_by_name_0
// type: int __fastcall(int, char *__s2)
pub fn stub_0xf19808() -> ! { todo!("0xf19808 _protobuf_c_enum_descriptor_get_value_by_name_0") }

#[doc(alias = "_protobuf_c_enum_descriptor_get_value_0")]
// 0xf19888 — _protobuf_c_enum_descriptor_get_value_0
pub fn stub_0xf19888() -> ! { todo!("0xf19888 _protobuf_c_enum_descriptor_get_value_0") }

#[doc(alias = "_protobuf_c_message_descriptor_get_field_by_name_0")]
// 0xf19918 — _protobuf_c_message_descriptor_get_field_by_name_0
// type: int __fastcall(int, char *__s2)
pub fn stub_0xf19918() -> ! { todo!("0xf19918 _protobuf_c_message_descriptor_get_field_by_name_0") }

#[doc(alias = "_protobuf_c_message_descriptor_get_field_0")]
// 0xf19998 — _protobuf_c_message_descriptor_get_field_0
pub fn stub_0xf19998() -> ! { todo!("0xf19998 _protobuf_c_message_descriptor_get_field_0") }

#[doc(alias = "_protobuf_c_service_descriptor_get_method_by_name_0")]
// 0xf19a28 — _protobuf_c_service_descriptor_get_method_by_name_0
// type: int __fastcall(int, char *__s2)
pub fn stub_0xf19a28() -> ! { todo!("0xf19a28 _protobuf_c_service_descriptor_get_method_by_name_0") }

#[doc(alias = "_parse_required_member_0")]
// 0xf19aa8 — _parse_required_member_0
pub fn stub_0xf19aa8() -> ! { todo!("0xf19aa8 _parse_required_member_0") }

#[doc(alias = "_parse_uint64_0")]
// 0xf19d08 — _parse_uint64_0
pub fn stub_0xf19d08() -> ! { todo!("0xf19d08 _parse_uint64_0") }

#[doc(alias = "_tag_pack_0")]
// 0xf19db4 — _tag_pack_0
pub fn stub_0xf19db4() -> ! { todo!("0xf19db4 _tag_pack_0") }

#[doc(alias = "_uint64_pack_2")]
// 0xf19e30 — _uint64_pack_2
pub fn stub_0xf19e30() -> ! { todo!("0xf19e30 _uint64_pack_2") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:architecture:timestamp:]")]
// 0xf19ed8 — -[FlurryPLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:architecture:timestamp:]
// type: FlurryPLCrashReportSystemInfo *__cdecl(FlurryPLCrashReportSystemInfo *self, SEL, int, id, int, id)
pub fn stub_0xf19ed8() -> ! { todo!("0xf19ed8 -[FlurryPLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:architecture:timestamp:]") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:operatingSystemBuild:architecture:timestamp:]")]
// 0xf19f0c — -[FlurryPLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:operatingSystemBuild:architecture:timestamp:]
// type: FlurryPLCrashReportSystemInfo *__cdecl(FlurryPLCrashReportSystemInfo *self, SEL, int, id, id, int, id)
pub fn stub_0xf19f0c() -> ! { todo!("0xf19f0c -[FlurryPLCrashReportSystemInfo initWithOperatingSystem:operatingSystemVersion:operatingSystemBuild:architecture:timestamp:]") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo dealloc]")]
// 0xf19fc4 — -[FlurryPLCrashReportSystemInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportSystemInfo *self, SEL)
pub fn stub_0xf19fc4() -> ! { todo!("0xf19fc4 -[FlurryPLCrashReportSystemInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo operatingSystem]")]
// 0xf1a03c — -[FlurryPLCrashReportSystemInfo operatingSystem]
// type: int __cdecl(FlurryPLCrashReportSystemInfo *self, SEL)
pub fn stub_0xf1a03c() -> ! { todo!("0xf1a03c -[FlurryPLCrashReportSystemInfo operatingSystem]") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo operatingSystemVersion]")]
// 0xf1a04c — -[FlurryPLCrashReportSystemInfo operatingSystemVersion]
// type: NSString *__cdecl(FlurryPLCrashReportSystemInfo *self, SEL)
pub fn stub_0xf1a04c() -> ! { todo!("0xf1a04c -[FlurryPLCrashReportSystemInfo operatingSystemVersion]") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo operatingSystemBuild]")]
// 0xf1a05c — -[FlurryPLCrashReportSystemInfo operatingSystemBuild]
// type: NSString *__cdecl(FlurryPLCrashReportSystemInfo *self, SEL)
pub fn stub_0xf1a05c() -> ! { todo!("0xf1a05c -[FlurryPLCrashReportSystemInfo operatingSystemBuild]") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo architecture]")]
// 0xf1a06c — -[FlurryPLCrashReportSystemInfo architecture]
// type: int __cdecl(FlurryPLCrashReportSystemInfo *self, SEL)
pub fn stub_0xf1a06c() -> ! { todo!("0xf1a06c -[FlurryPLCrashReportSystemInfo architecture]") }

#[doc(alias = "-[FlurryPLCrashReportSystemInfo timestamp]")]
// 0xf1a07c — -[FlurryPLCrashReportSystemInfo timestamp]
// type: NSDate *__cdecl(FlurryPLCrashReportSystemInfo *self, SEL)
pub fn stub_0xf1a07c() -> ! { todo!("0xf1a07c -[FlurryPLCrashReportSystemInfo timestamp]") }

#[doc(alias = "-[FlurryPLCrashReportApplicationInfo initWithApplicationIdentifier:applicationVersion:]")]
// 0xf1a08c — -[FlurryPLCrashReportApplicationInfo initWithApplicationIdentifier:applicationVersion:]
// type: FlurryPLCrashReportApplicationInfo *__cdecl(FlurryPLCrashReportApplicationInfo *self, SEL, id, id)
pub fn stub_0xf1a08c() -> ! { todo!("0xf1a08c -[FlurryPLCrashReportApplicationInfo initWithApplicationIdentifier:applicationVersion:]") }

#[doc(alias = "-[FlurryPLCrashReportApplicationInfo dealloc]")]
// 0xf1a104 — -[FlurryPLCrashReportApplicationInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportApplicationInfo *self, SEL)
pub fn stub_0xf1a104() -> ! { todo!("0xf1a104 -[FlurryPLCrashReportApplicationInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportApplicationInfo applicationIdentifier]")]
// 0xf1a168 — -[FlurryPLCrashReportApplicationInfo applicationIdentifier]
// type: NSString *__cdecl(FlurryPLCrashReportApplicationInfo *self, SEL)
pub fn stub_0xf1a168() -> ! { todo!("0xf1a168 -[FlurryPLCrashReportApplicationInfo applicationIdentifier]") }

#[doc(alias = "-[FlurryPLCrashReportApplicationInfo applicationVersion]")]
// 0xf1a178 — -[FlurryPLCrashReportApplicationInfo applicationVersion]
// type: NSString *__cdecl(FlurryPLCrashReportApplicationInfo *self, SEL)
pub fn stub_0xf1a178() -> ! { todo!("0xf1a178 -[FlurryPLCrashReportApplicationInfo applicationVersion]") }

#[doc(alias = "-[FlurryPLCrashReportThreadInfo initWithThreadNumber:stackFrames:crashed:registers:]")]
// 0xf1a188 — -[FlurryPLCrashReportThreadInfo initWithThreadNumber:stackFrames:crashed:registers:]
// type: FlurryPLCrashReportThreadInfo *__cdecl(FlurryPLCrashReportThreadInfo *self, SEL, int, id, char, id)
pub fn stub_0xf1a188() -> ! { todo!("0xf1a188 -[FlurryPLCrashReportThreadInfo initWithThreadNumber:stackFrames:crashed:registers:]") }

#[doc(alias = "-[FlurryPLCrashReportThreadInfo dealloc]")]
// 0xf1a228 — -[FlurryPLCrashReportThreadInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportThreadInfo *self, SEL)
pub fn stub_0xf1a228() -> ! { todo!("0xf1a228 -[FlurryPLCrashReportThreadInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportThreadInfo threadNumber]")]
// 0xf1a28c — -[FlurryPLCrashReportThreadInfo threadNumber]
// type: int __cdecl(FlurryPLCrashReportThreadInfo *self, SEL)
pub fn stub_0xf1a28c() -> ! { todo!("0xf1a28c -[FlurryPLCrashReportThreadInfo threadNumber]") }

#[doc(alias = "-[FlurryPLCrashReportThreadInfo stackFrames]")]
// 0xf1a29c — -[FlurryPLCrashReportThreadInfo stackFrames]
// type: NSArray *__cdecl(FlurryPLCrashReportThreadInfo *self, SEL)
pub fn stub_0xf1a29c() -> ! { todo!("0xf1a29c -[FlurryPLCrashReportThreadInfo stackFrames]") }

#[doc(alias = "-[FlurryPLCrashReportThreadInfo crashed]")]
// 0xf1a2ac — -[FlurryPLCrashReportThreadInfo crashed]
// type: char __cdecl(FlurryPLCrashReportThreadInfo *self, SEL)
pub fn stub_0xf1a2ac() -> ! { todo!("0xf1a2ac -[FlurryPLCrashReportThreadInfo crashed]") }

#[doc(alias = "-[FlurryPLCrashReportThreadInfo registers]")]
// 0xf1a2bc — -[FlurryPLCrashReportThreadInfo registers]
// type: NSArray *__cdecl(FlurryPLCrashReportThreadInfo *self, SEL)
pub fn stub_0xf1a2bc() -> ! { todo!("0xf1a2bc -[FlurryPLCrashReportThreadInfo registers]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo initWithCodeType:baseAddress:size:name:uuid:]")]
// 0xf1a2cc — -[FlurryPLCrashReportBinaryImageInfo initWithCodeType:baseAddress:size:name:uuid:]
// type: FlurryPLCrashReportBinaryImageInfo *__cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL, id, unsigned __int64, unsigned __int64, id, id)
pub fn stub_0xf1a2cc() -> ! { todo!("0xf1a2cc -[FlurryPLCrashReportBinaryImageInfo initWithCodeType:baseAddress:size:name:uuid:]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo dealloc]")]
// 0xf1a438 — -[FlurryPLCrashReportBinaryImageInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xf1a438() -> ! { todo!("0xf1a438 -[FlurryPLCrashReportBinaryImageInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo codeType]")]
// 0xf1a4b0 — -[FlurryPLCrashReportBinaryImageInfo codeType]
// type: FlurryPLCrashReportProcessorInfo *__cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xf1a4b0() -> ! { todo!("0xf1a4b0 -[FlurryPLCrashReportBinaryImageInfo codeType]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo imageBaseAddress]")]
// 0xf1a4c0 — -[FlurryPLCrashReportBinaryImageInfo imageBaseAddress]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xf1a4c0() -> ! { todo!("0xf1a4c0 -[FlurryPLCrashReportBinaryImageInfo imageBaseAddress]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo imageSize]")]
// 0xf1a4d8 — -[FlurryPLCrashReportBinaryImageInfo imageSize]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xf1a4d8() -> ! { todo!("0xf1a4d8 -[FlurryPLCrashReportBinaryImageInfo imageSize]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo imageName]")]
// 0xf1a4f0 — -[FlurryPLCrashReportBinaryImageInfo imageName]
// type: NSString *__cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xf1a4f0() -> ! { todo!("0xf1a4f0 -[FlurryPLCrashReportBinaryImageInfo imageName]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo hasImageUUID]")]
// 0xf1a500 — -[FlurryPLCrashReportBinaryImageInfo hasImageUUID]
// type: char __cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xf1a500() -> ! { todo!("0xf1a500 -[FlurryPLCrashReportBinaryImageInfo hasImageUUID]") }

#[doc(alias = "-[FlurryPLCrashReportBinaryImageInfo imageUUID]")]
// 0xf1a510 — -[FlurryPLCrashReportBinaryImageInfo imageUUID]
// type: NSString *__cdecl(FlurryPLCrashReportBinaryImageInfo *self, SEL)
pub fn stub_0xf1a510() -> ! { todo!("0xf1a510 -[FlurryPLCrashReportBinaryImageInfo imageUUID]") }

#[doc(alias = "-[FlurryPLCrashReportExceptionInfo initWithExceptionName:reason:]")]
// 0xf1a520 — -[FlurryPLCrashReportExceptionInfo initWithExceptionName:reason:]
// type: FlurryPLCrashReportExceptionInfo *__cdecl(FlurryPLCrashReportExceptionInfo *self, SEL, id, id)
pub fn stub_0xf1a520() -> ! { todo!("0xf1a520 -[FlurryPLCrashReportExceptionInfo initWithExceptionName:reason:]") }

#[doc(alias = "-[FlurryPLCrashReportExceptionInfo initWithExceptionName:reason:stackFrames:]")]
// 0xf1a544 — -[FlurryPLCrashReportExceptionInfo initWithExceptionName:reason:stackFrames:]
// type: FlurryPLCrashReportExceptionInfo *__cdecl(FlurryPLCrashReportExceptionInfo *self, SEL, id, id, id)
pub fn stub_0xf1a544() -> ! { todo!("0xf1a544 -[FlurryPLCrashReportExceptionInfo initWithExceptionName:reason:stackFrames:]") }

#[doc(alias = "-[FlurryPLCrashReportExceptionInfo dealloc]")]
// 0xf1a5d8 — -[FlurryPLCrashReportExceptionInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xf1a5d8() -> ! { todo!("0xf1a5d8 -[FlurryPLCrashReportExceptionInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportExceptionInfo exceptionName]")]
// 0xf1a650 — -[FlurryPLCrashReportExceptionInfo exceptionName]
// type: NSString *__cdecl(FlurryPLCrashReportExceptionInfo *self, SEL)
pub fn stub_0xf1a650() -> ! { todo!("0xf1a650 -[FlurryPLCrashReportExceptionInfo exceptionName]") }
