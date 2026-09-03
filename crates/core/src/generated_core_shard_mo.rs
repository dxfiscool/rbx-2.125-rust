//! core shard mo — 150 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 150 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 2623 uncovered before -> 2473 after, batch 0xf1c670..0xf1fb64).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "_macho_swap16")]
// 0xf1c670 — _macho_swap16
pub fn stub_0xf1c670() {
    // IDA 0xf1c670: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_macho_swap32")]
// 0xf1c674 — _macho_swap32
pub fn stub_0xf1c674() {
    // IDA 0xf1c674: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_macho_swap64")]
// 0xf1c678 — _macho_swap64
pub fn stub_0xf1c678() {
    // IDA 0xf1c678: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_plcrash_async_macho_next_command_type")]
// 0xf1c680 — _plcrash_async_macho_next_command_type
pub fn stub_0xf1c680() {
    // IDA 0xf1c680: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_plcrash_async_macho_next_command")]
// 0xf1c6a8 — _plcrash_async_macho_next_command
pub fn stub_0xf1c6a8() {
    // IDA 0xf1c6a8: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_plcrash_async_macho_find_command")]
// 0xf1c738 — _plcrash_async_macho_find_command
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf1c738() {
    // IDA 0xf1c738: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_plcrash_async_macho_find_segment_cmd")]
// 0xf1c77c — _plcrash_async_macho_find_segment_cmd
pub fn stub_0xf1c77c() {
    // IDA 0xf1c77c: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_plcrash_async_macho_map_segment")]
// 0xf1c7c8 — _plcrash_async_macho_map_segment
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf1c7c8() {
    // IDA 0xf1c7c8: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_plcrash_async_macho_map_section")]
// 0xf1c870 — _plcrash_async_macho_map_section
pub fn stub_0xf1c870() {
    // IDA 0xf1c870: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_plcrash_async_macho_find_symbol")]
// 0xf1c960 — _plcrash_async_macho_find_symbol
// type: int __fastcall(int, int, void (__fastcall *)(int, int, int), int)
pub fn stub_0xf1c960() {
    // IDA 0xf1c960: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_macho_find_symtab_symbol")]
// 0xf1cb78 — _plcrash_async_macho_find_symtab_symbol
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf1cb78() {
    // IDA 0xf1cb78: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_macho_mapped_segment_free")]
// 0xf1cc6c — _plcrash_async_macho_mapped_segment_free
pub fn stub_0xf1cc6c() {
    // IDA 0xf1cc6c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_nasync_macho_free")]
// 0xf1cc78 — _plcrash_nasync_macho_free
pub fn stub_0xf1cc78() {
    // IDA 0xf1cc78: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_mobject_init")]
// 0xf1ccac — _plcrash_async_mobject_init
// type: int __fastcall(int, vm_map_t target_task)
pub fn stub_0xf1ccac() {
    // IDA 0xf1ccac: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_mobject_verify_local_pointer")]
// 0xf1cd7c — _plcrash_async_mobject_verify_local_pointer
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1cd7c() {
    // IDA 0xf1cd7c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_mobject_remap_address")]
// 0xf1cda4 — _plcrash_async_mobject_remap_address
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1cda4() {
    // IDA 0xf1cda4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_mobject_free")]
// 0xf1cdd4 — _plcrash_async_mobject_free
// type: int __fastcall(_DWORD)
pub fn stub_0xf1cdd4() {
    // IDA 0xf1cdd4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_objc_cache_init")]
// 0xf1cdf8 — _plcrash_async_objc_cache_init
pub fn stub_0xf1cdf8() {
    // IDA 0xf1cdf8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_objc_cache_free")]
// 0xf1ce14 — _plcrash_async_objc_cache_free
pub fn stub_0xf1ce14() {
    // IDA 0xf1ce14: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_free_mapped_sections")]
// 0xf1ce40 — _free_mapped_sections
pub fn stub_0xf1ce40() {
    // IDA 0xf1ce40: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_objc_find_method")]
// 0xf1ce84 — _plcrash_async_objc_find_method
pub fn stub_0xf1ce84() {
    // IDA 0xf1ce84: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_objc_parse")]
// 0xf1ced4 — _plcrash_async_objc_parse
pub fn stub_0xf1ced4() {
    // IDA 0xf1ced4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_pl_async_objc_find_method_search_callback")]
// 0xf1d1cc — _pl_async_objc_find_method_search_callback
pub fn stub_0xf1d1cc() {
    // IDA 0xf1d1cc: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_pl_async_objc_find_method_call_callback")]
// 0xf1d1e0 — _pl_async_objc_find_method_call_callback
pub fn stub_0xf1d1e0() {
    // IDA 0xf1d1e0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_pl_async_objc_parse_objc2_class")]
// 0xf1d200 — _pl_async_objc_parse_objc2_class
pub fn stub_0xf1d200() {
    // IDA 0xf1d200: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_pl_async_parse_obj1_class")]
// 0xf1d4ac — _pl_async_parse_obj1_class
pub fn stub_0xf1d4ac() {
    // IDA 0xf1d4ac: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_symbol_cache_init")]
// 0xf1d5b0 — _plcrash_async_symbol_cache_init
pub fn stub_0xf1d5b0() {
    // IDA 0xf1d5b0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_symbol_cache_free")]
// 0xf1d5bc — _plcrash_async_symbol_cache_free
pub fn stub_0xf1d5bc() {
    // IDA 0xf1d5bc: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_find_symbol")]
// 0xf1d5c8 — _plcrash_async_find_symbol
pub fn stub_0xf1d5c8() {
    // IDA 0xf1d5c8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_macho_symbol_callback")]
// 0xf1d65c — _macho_symbol_callback
// type: unsigned int __fastcall(unsigned int result, _BYTE *, int)
pub fn stub_0xf1d65c() {
    // IDA 0xf1d65c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_symbol_callback")]
// 0xf1d6a4 — _objc_symbol_callback
pub fn stub_0xf1d6a4() {
    // IDA 0xf1d6a4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_macho_string_init")]
// 0xf1d7a4 — _plcrash_async_macho_string_init
pub fn stub_0xf1d7a4() {
    // IDA 0xf1d7a4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_macho_string_get_length")]
// 0xf1d7b8 — _plcrash_async_macho_string_get_length
pub fn stub_0xf1d7b8() {
    // IDA 0xf1d7b8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_macho_string_read")]
// 0xf1d7d0 — _plcrash_async_macho_string_read
pub fn stub_0xf1d7d0() {
    // IDA 0xf1d7d0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_macho_string_get_pointer")]
// 0xf1d848 — _plcrash_async_macho_string_get_pointer
pub fn stub_0xf1d848() {
    // IDA 0xf1d848: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_macho_string_free")]
// 0xf1d878 — _plcrash_async_macho_string_free
pub fn stub_0xf1d878() {
    // IDA 0xf1d878: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo initWithInstructionPointer:symbolInfo:]")]
// 0xf1d88c — -[FlurryPLCrashReportStackFrameInfo initWithInstructionPointer:symbolInfo:]
// type: FlurryPLCrashReportStackFrameInfo *__cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL, unsigned __int64, id)
pub fn stub_0xf1d88c() {
    // IDA 0xf1d88c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo dealloc]")]
// 0xf1d8f8 — -[FlurryPLCrashReportStackFrameInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL)
pub fn stub_0xf1d8f8() {
    // IDA 0xf1d8f8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo instructionPointer]")]
// 0xf1d944 — -[FlurryPLCrashReportStackFrameInfo instructionPointer]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL)
pub fn stub_0xf1d944() {
    // IDA 0xf1d944: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo symbolInfo]")]
// 0xf1d95c — -[FlurryPLCrashReportStackFrameInfo symbolInfo]
// type: FlurryPLCrashReportSymbolInfo *__cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL)
pub fn stub_0xf1d95c() {
    // IDA 0xf1d95c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo initWithRegisterName:registerValue:]")]
// 0xf1d96c — -[FlurryPLCrashReportRegisterInfo initWithRegisterName:registerValue:]
// type: FlurryPLCrashReportRegisterInfo *__cdecl(FlurryPLCrashReportRegisterInfo *self, SEL, id, unsigned __int64)
pub fn stub_0xf1d96c() {
    // IDA 0xf1d96c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo dealloc]")]
// 0xf1d9e4 — -[FlurryPLCrashReportRegisterInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xf1d9e4() {
    // IDA 0xf1d9e4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo registerName]")]
// 0xf1da30 — -[FlurryPLCrashReportRegisterInfo registerName]
// type: NSString *__cdecl(FlurryPLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xf1da30() {
    // IDA 0xf1da30: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo registerValue]")]
// 0xf1da40 — -[FlurryPLCrashReportRegisterInfo registerValue]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xf1da40() {
    // IDA 0xf1da40: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo initWithSymbolName:startAddress:endAddress:]")]
// 0xf1da58 — -[FlurryPLCrashReportSymbolInfo initWithSymbolName:startAddress:endAddress:]
// type: FlurryPLCrashReportSymbolInfo *__cdecl(FlurryPLCrashReportSymbolInfo *self, SEL, id, unsigned __int64, unsigned __int64)
pub fn stub_0xf1da58() {
    // IDA 0xf1da58: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo dealloc]")]
// 0xf1daf0 — -[FlurryPLCrashReportSymbolInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1daf0() {
    // IDA 0xf1daf0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo symbolName]")]
// 0xf1db3c — -[FlurryPLCrashReportSymbolInfo symbolName]
// type: NSString *__cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1db3c() {
    // IDA 0xf1db3c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo startAddress]")]
// 0xf1db4c — -[FlurryPLCrashReportSymbolInfo startAddress]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1db4c() {
    // IDA 0xf1db4c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo endAddress]")]
// 0xf1db64 — -[FlurryPLCrashReportSymbolInfo endAddress]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1db64() {
    // IDA 0xf1db64: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashMachExceptionServer init]")]
// 0xf1db7c — -[FlurryPLCrashMachExceptionServer init]
// type: FlurryPLCrashMachExceptionServer *__cdecl(FlurryPLCrashMachExceptionServer *self, SEL)
pub fn stub_0xf1db7c() {
    // IDA 0xf1db7c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashMachExceptionServer registerHandlerForTask:thread:withCallback:context:error:]")]
// 0xf1dba8 — -[FlurryPLCrashMachExceptionServer registerHandlerForTask:thread:withCallback:context:error:]
// type: char __cdecl(FlurryPLCrashMachExceptionServer *self, SEL, unsigned int, unsigned int, void *, void *, id *)
pub fn stub_0xf1dba8() {
    // IDA 0xf1dba8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_exception_server_thread")]
// 0xf1de4c — _exception_server_thread
pub fn stub_0xf1de4c() {
    // IDA 0xf1de4c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashMachExceptionServer deregisterHandlerAndReturnError:]")]
// 0xf1e478 — -[FlurryPLCrashMachExceptionServer deregisterHandlerAndReturnError:]
// type: char __cdecl(FlurryPLCrashMachExceptionServer *self, SEL, id *)
pub fn stub_0xf1e478() {
    // IDA 0xf1e478: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_set_exception_ports")]
// 0xf1e5c0 — _set_exception_ports
// type: int __fastcall(task_t task, thread_act_t thread, exception_mask_t exception_mask)
pub fn stub_0xf1e5c0() {
    // IDA 0xf1e5c0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_allocator_new")]
// 0xf1e648 — _plcrash_async_allocator_new
pub fn stub_0xf1e648() {
    // IDA 0xf1e648: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_allocator_alloc")]
// 0xf1e750 — _plcrash_async_allocator_alloc
pub fn stub_0xf1e750() {
    // IDA 0xf1e750: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[__ARCLite__ load]")]
// 0xf1e79c — +[__ARCLite__ load]
// type: void __cdecl(id, SEL)
pub fn stub_0xf1e79c() {
    // IDA 0xf1e79c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_add_image_hook_ARC")]
// 0xf1e9d0 — _add_image_hook_ARC
pub fn stub_0xf1e9d0() {
    // IDA 0xf1e9d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_NSArray_objectAtIndexedSubscript")]
// 0xf1e9e8 — ___arclite_NSArray_objectAtIndexedSubscript
pub fn stub_0xf1e9e8() {
    // IDA 0xf1e9e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_NSMutableArray_setObject_atIndexedSubscript")]
// 0xf1ea00 — ___arclite_NSMutableArray_setObject_atIndexedSubscript
// type: int __fastcall(id)
pub fn stub_0xf1ea00() {
    // IDA 0xf1ea00: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_NSDictionary_objectForKeyedSubscript")]
// 0xf1ea50 — ___arclite_NSDictionary_objectForKeyedSubscript
pub fn stub_0xf1ea50() {
    // IDA 0xf1ea50: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_NSMutableDictionary__setObject_forKeyedSubscript")]
// 0xf1ea68 — ___arclite_NSMutableDictionary__setObject_forKeyedSubscript
pub fn stub_0xf1ea68() {
    // IDA 0xf1ea68: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_NSOrderedSet_objectAtIndexedSubscript")]
// 0xf1ea80 — ___arclite_NSOrderedSet_objectAtIndexedSubscript
pub fn stub_0xf1ea80() {
    // IDA 0xf1ea80: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_NSMutableOrderedSet_setObject_atIndexedSubscript")]
// 0xf1ea98 — ___arclite_NSMutableOrderedSet_setObject_atIndexedSubscript
pub fn stub_0xf1ea98() {
    // IDA 0xf1ea98: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_autoreleasePoolPop")]
// 0xf1eab0 — ___arclite_objc_autoreleasePoolPop
pub fn stub_0xf1eab0() {
    // IDA 0xf1eab0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_patch_lazy_pointers")]
// 0xf1eac8 — _patch_lazy_pointers
pub fn stub_0xf1eac8() {
    // IDA 0xf1eac8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_autoreleasePoolPush")]
// 0xf1ec64 — ___arclite_objc_autoreleasePoolPush
pub fn stub_0xf1ec64() {
    // IDA 0xf1ec64: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_object_setIvar")]
// 0xf1ecc4 — ___arclite_object_setIvar
// type: int __fastcall(id)
pub fn stub_0xf1ecc4() {
    // IDA 0xf1ecc4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_object_copy")]
// 0xf1edac — ___arclite_object_copy
// type: char *__fastcall(id, int)
pub fn stub_0xf1edac() {
    // IDA 0xf1edac: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_retain")]
// 0xf1eeb8 — ___arclite_objc_retain
pub fn stub_0xf1eeb8() {
    // IDA 0xf1eeb8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_retainBlock")]
// 0xf1eed0 — ___arclite_objc_retainBlock
pub fn stub_0xf1eed0() {
    // IDA 0xf1eed0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_release")]
// 0xf1eedc — ___arclite_objc_release
pub fn stub_0xf1eedc() {
    // IDA 0xf1eedc: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_autorelease")]
// 0xf1eef4 — ___arclite_objc_autorelease
pub fn stub_0xf1eef4() {
    // IDA 0xf1eef4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_retainAutorelease")]
// 0xf1ef0c — ___arclite_objc_retainAutorelease
pub fn stub_0xf1ef0c() {
    // IDA 0xf1ef0c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_autoreleaseReturnValue")]
// 0xf1ef34 — ___arclite_objc_autoreleaseReturnValue
pub fn stub_0xf1ef34() {
    // IDA 0xf1ef34: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_retainAutoreleaseReturnValue")]
// 0xf1ef4c — ___arclite_objc_retainAutoreleaseReturnValue
pub fn stub_0xf1ef4c() {
    // IDA 0xf1ef4c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_retainAutoreleasedReturnValue")]
// 0xf1ef74 — ___arclite_objc_retainAutoreleasedReturnValue
// type: id __fastcall(void *)
pub fn stub_0xf1ef74() {
    // IDA 0xf1ef74: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "___arclite_objc_storeStrong")]
// 0xf1ef8c — ___arclite_objc_storeStrong
pub fn stub_0xf1ef8c() {
    // IDA 0xf1ef8c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "__ZNSt11logic_errorD2Ev$shim")]
// 0xf1efd0 — __ZNSt11logic_errorD2Ev$shim
// type: void __cdecl(std::logic_error *__hidden this)
pub fn stub_0xf1efd0() {
    // IDA 0xf1efd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZdlPv$shim")]
// 0xf1efdc — __ZdlPv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1efdc() {
    // IDA 0xf1efdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN19CRenderSettingsItemD2Ev$shim")]
// 0xf1eff4 — __ZN19CRenderSettingsItemD2Ev$shim
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xf1eff4() {
    // IDA 0xf1eff4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v$shim")]
// 0xf1f06c — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f06c() {
    // IDA 0xf1f06c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__Znwm$shim")]
// 0xf1f084 — __Znwm$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1f084() {
    // IDA 0xf1f084: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f0f0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f0f0() {
    // IDA 0xf1f0f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f0fc — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f0fc() {
    // IDA 0xf1f0fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f108 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f108() {
    // IDA 0xf1f108: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f114 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f114() {
    // IDA 0xf1f114: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f120 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f120() {
    // IDA 0xf1f120: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f12c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf1f12c() {
    // IDA 0xf1f12c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f138 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f138() {
    // IDA 0xf1f138: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__Block_object_assign$shim")]
// 0xf1f198 — __Block_object_assign$shim
// type: void __cdecl(void *, const void *, const int)
pub fn stub_0xf1f198() {
    // IDA 0xf1f198: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__Block_object_dispose$shim")]
// 0xf1f1a4 — __Block_object_dispose$shim
// type: void __cdecl(const void *, const int)
pub fn stub_0xf1f1a4() {
    // IDA 0xf1f1a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "_objc_msgSend$shim")]
// 0xf1f1b0 — _objc_msgSend$shim
// type: id(id, SEL, ...)
pub fn stub_0xf1f1b0() {
    // IDA 0xf1f1b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "_dispatch_async$shim")]
// 0xf1f1bc — _dispatch_async$shim
// type: void __cdecl(dispatch_queue_t queue, dispatch_block_t block)
pub fn stub_0xf1f1bc() {
    // IDA 0xf1f1bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v$shim")]
// 0xf1f210 — __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v$shim
pub fn stub_0xf1f210() {
    // IDA 0xf1f210: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv$shim")]
// 0xf1f21c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv$shim
// type: int()
pub fn stub_0xf1f21c() {
    // IDA 0xf1f21c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v$shim")]
// 0xf1f234 — __ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v$shim
pub fn stub_0xf1f234() {
    // IDA 0xf1f234: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv$shim")]
// 0xf1f240 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv$shim
pub fn stub_0xf1f240() {
    // IDA 0xf1f240: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim")]
// 0xf1f24c — __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim
pub fn stub_0xf1f24c() {
    // IDA 0xf1f24c: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim")]
// 0xf1f258 — __ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim
pub fn stub_0xf1f258() {
    // IDA 0xf1f258: task-scheduler helper (IDA 0x245c64: dtor; cf. TaskScheduler in task_scheduler.rs). Drop glue — carrier no-op.
}

#[doc(alias = "__ZNSt13runtime_errorD2Ev$shim")]
// 0xf1f294 — __ZNSt13runtime_errorD2Ev$shim
// type: void __cdecl(std::runtime_error *__hidden this)
pub fn stub_0xf1f294() {
    // IDA 0xf1f294: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v$shim")]
// 0xf1f2ac — __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v$shim
pub fn stub_0xf1f2ac() {
    // IDA 0xf1f2ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v$shim")]
// 0xf1f2b8 — __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v$shim
pub fn stub_0xf1f2b8() {
    // IDA 0xf1f2b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv$shim")]
// 0xf1f2d0 — __ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f2d0() {
    // IDA 0xf1f2d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f300 — __ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f300() {
    // IDA 0xf1f300: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_free$shim")]
// 0xf1f324 — _free$shim
// type: void __cdecl(void *)
pub fn stub_0xf1f324() {
    // IDA 0xf1f324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "_puts$shim")]
// 0xf1f33c — _puts$shim
// type: int __cdecl(const char *)
pub fn stub_0xf1f33c() {
    // IDA 0xf1f33c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim")]
// 0xf1f348 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim
pub fn stub_0xf1f348() {
    // IDA 0xf1f348: physics world/stage/poly geometry type. Collision-world internals owned by higher crates — carrier no-op in core.
}

#[doc(alias = "__ZNSt9exceptionD2Ev$shim")]
// 0xf1f354 — __ZNSt9exceptionD2Ev$shim
// type: void __cdecl(std::exception *__hidden this)
pub fn stub_0xf1f354() {
    // IDA 0xf1f354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN18iOSSettingsServiceD2Ev$shim")]
// 0xf1f36c — __ZN18iOSSettingsServiceD2Ev$shim
pub fn stub_0xf1f36c() {
    // IDA 0xf1f36c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim")]
// 0xf1f378 — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim
pub fn stub_0xf1f378() {
    // IDA 0xf1f378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f384 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f384() {
    // IDA 0xf1f384: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "___cxa_atexit$shim")]
// 0xf1f3e4 — ___cxa_atexit$shim
// type: int __fastcall(void (__fastcall *lpfunc)(void *), void *obj, void *lpdso_handle)
pub fn stub_0xf1f3e4() {
    // IDA 0xf1f3e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim")]
// 0xf1f3f0 — __ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1f3f0() {
    // IDA 0xf1f3f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSsC1ERKSs$shim")]
// 0xf1f3fc — __ZNSsC1ERKSs$shim
// type: int __fastcall(std::string *, const std::string *)
pub fn stub_0xf1f3fc() {
    // IDA 0xf1f3fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSsD2Ev$shim")]
// 0xf1f408 — __ZNSsD2Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1f408() {
    // IDA 0xf1f408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim")]
// 0xf1f414 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim
// type: int(void)
pub fn stub_0xf1f414() {
    // IDA 0xf1f414: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim")]
// 0xf1f420 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf1f420() {
    // IDA 0xf1f420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f438 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf1f438() {
    // IDA 0xf1f438: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim")]
// 0xf1f45c — __ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f45c() {
    // IDA 0xf1f45c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f468 — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf1f468() {
    // IDA 0xf1f468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim")]
// 0xf1f474 — __ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f474() {
    // IDA 0xf1f474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim")]
// 0xf1f4b0 — __ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f4b0() {
    // IDA 0xf1f4b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim")]
// 0xf1f4bc — __ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f4bc() {
    // IDA 0xf1f4bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "___cxa_guard_release$shim")]
// 0xf1f510 — ___cxa_guard_release$shim
// type: void __fastcall(__guard *)
pub fn stub_0xf1f510() {
    // IDA 0xf1f510: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim")]
// 0xf1f7bc — __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf1f7bc() {
    // IDA 0xf1f7bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim")]
// 0xf1f7c8 — __ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim
pub fn stub_0xf1f7c8() {
    // IDA 0xf1f7c8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
// 0xf1f8ac — __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f8ac() {
    // IDA 0xf1f8ac: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim")]
// 0xf1f8b8 — __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim
pub fn stub_0xf1f8b8() {
    // IDA 0xf1f8b8: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f8d0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f8d0() {
    // IDA 0xf1f8d0: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
// 0xf1f8dc — __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim
pub fn stub_0xf1f8dc() {
    // IDA 0xf1f8dc: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim")]
// 0xf1f8e8 — __ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim
pub fn stub_0xf1f8e8() {
    // IDA 0xf1f8e8: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim")]
// 0xf1f8f4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim
pub fn stub_0xf1f8f4() {
    // IDA 0xf1f8f4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "__ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf1f9f0 — __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
pub fn stub_0xf1f9f0() {
    // IDA 0xf1f9f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
// 0xf1f9fc — __ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim
pub fn stub_0xf1f9fc() {
    // IDA 0xf1f9fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
// 0xf1fa08 — __ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim
pub fn stub_0xf1fa08() {
    // IDA 0xf1fa08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE9push_backERKSs$shim")]
// 0xf1fa2c — __ZNSt5dequeISsSaISsEE9push_backERKSs$shim
pub fn stub_0xf1fa2c() {
    // IDA 0xf1fa2c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim")]
// 0xf1fa44 — __ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim
pub fn stub_0xf1fa44() {
    // IDA 0xf1fa44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim")]
// 0xf1fa50 — __ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim
pub fn stub_0xf1fa50() {
    // IDA 0xf1fa50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZNSs6assignERKSs$shim")]
// 0xf1faa4 — __ZNSs6assignERKSs$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1faa4() {
    // IDA 0xf1faa4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX15AdvMoveToolBaseD2Ev$shim")]
// 0xf1fab0 — __ZN3RBX15AdvMoveToolBaseD2Ev$shim
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
pub fn stub_0xf1fab0() {
    // IDA 0xf1fab0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim")]
// 0xf1fabc — __ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fabc() {
    // IDA 0xf1fabc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf1fac8 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int(void)
pub fn stub_0xf1fac8() {
    // IDA 0xf1fac8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim")]
// 0xf1fae0 — __ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fae0() {
    // IDA 0xf1fae0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNSs6assignEPKcm$shim")]
// 0xf1faec — __ZNSs6assignEPKcm$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1faec() {
    // IDA 0xf1faec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim")]
// 0xf1faf8 — __ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1faf8() {
    // IDA 0xf1faf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim")]
// 0xf1fb04 — __ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fb04() {
    // IDA 0xf1fb04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim")]
// 0xf1fb4c — __ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fb4c() {
    // IDA 0xf1fb4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim")]
// 0xf1fb58 — __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim
pub fn stub_0xf1fb58() {
    // IDA 0xf1fb58: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim")]
// 0xf1fb64 — __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim
pub fn stub_0xf1fb64() {
    // IDA 0xf1fb64: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}
