//! core shard mo — 150 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 150 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 2623 uncovered before -> 2473 after, batch 0xf1c670..0xf1fb64).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;


#[doc(alias = "_macho_swap16")]
// 0xf1c670 — _macho_swap16
pub fn stub_0xf1c670() -> ! { todo!("0xf1c670 _macho_swap16") }

#[doc(alias = "_macho_swap32")]
// 0xf1c674 — _macho_swap32
pub fn stub_0xf1c674() -> ! { todo!("0xf1c674 _macho_swap32") }

#[doc(alias = "_macho_swap64")]
// 0xf1c678 — _macho_swap64
pub fn stub_0xf1c678() -> ! { todo!("0xf1c678 _macho_swap64") }

#[doc(alias = "_plcrash_async_macho_next_command_type")]
// 0xf1c680 — _plcrash_async_macho_next_command_type
pub fn stub_0xf1c680() -> ! { todo!("0xf1c680 _plcrash_async_macho_next_command_type") }

#[doc(alias = "_plcrash_async_macho_next_command")]
// 0xf1c6a8 — _plcrash_async_macho_next_command
pub fn stub_0xf1c6a8() -> ! { todo!("0xf1c6a8 _plcrash_async_macho_next_command") }

#[doc(alias = "_plcrash_async_macho_find_command")]
// 0xf1c738 — _plcrash_async_macho_find_command
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf1c738() -> ! { todo!("0xf1c738 _plcrash_async_macho_find_command") }

#[doc(alias = "_plcrash_async_macho_find_segment_cmd")]
// 0xf1c77c — _plcrash_async_macho_find_segment_cmd
pub fn stub_0xf1c77c() -> ! { todo!("0xf1c77c _plcrash_async_macho_find_segment_cmd") }

#[doc(alias = "_plcrash_async_macho_map_segment")]
// 0xf1c7c8 — _plcrash_async_macho_map_segment
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf1c7c8() -> ! { todo!("0xf1c7c8 _plcrash_async_macho_map_segment") }

#[doc(alias = "_plcrash_async_macho_map_section")]
// 0xf1c870 — _plcrash_async_macho_map_section
pub fn stub_0xf1c870() -> ! { todo!("0xf1c870 _plcrash_async_macho_map_section") }

#[doc(alias = "_plcrash_async_macho_find_symbol")]
// 0xf1c960 — _plcrash_async_macho_find_symbol
// type: int __fastcall(int, int, void (__fastcall *)(int, int, int), int)
pub fn stub_0xf1c960() -> ! { todo!("0xf1c960 _plcrash_async_macho_find_symbol") }

#[doc(alias = "_plcrash_async_macho_find_symtab_symbol")]
// 0xf1cb78 — _plcrash_async_macho_find_symtab_symbol
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf1cb78() -> ! { todo!("0xf1cb78 _plcrash_async_macho_find_symtab_symbol") }

#[doc(alias = "_plcrash_async_macho_mapped_segment_free")]
// 0xf1cc6c — _plcrash_async_macho_mapped_segment_free
pub fn stub_0xf1cc6c() -> ! { todo!("0xf1cc6c _plcrash_async_macho_mapped_segment_free") }

#[doc(alias = "_plcrash_nasync_macho_free")]
// 0xf1cc78 — _plcrash_nasync_macho_free
pub fn stub_0xf1cc78() -> ! { todo!("0xf1cc78 _plcrash_nasync_macho_free") }

#[doc(alias = "_plcrash_async_mobject_init")]
// 0xf1ccac — _plcrash_async_mobject_init
// type: int __fastcall(int, vm_map_t target_task)
pub fn stub_0xf1ccac() -> ! { todo!("0xf1ccac _plcrash_async_mobject_init") }

#[doc(alias = "_plcrash_async_mobject_verify_local_pointer")]
// 0xf1cd7c — _plcrash_async_mobject_verify_local_pointer
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1cd7c() -> ! { todo!("0xf1cd7c _plcrash_async_mobject_verify_local_pointer") }

#[doc(alias = "_plcrash_async_mobject_remap_address")]
// 0xf1cda4 — _plcrash_async_mobject_remap_address
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1cda4() -> ! { todo!("0xf1cda4 _plcrash_async_mobject_remap_address") }

#[doc(alias = "_plcrash_async_mobject_free")]
// 0xf1cdd4 — _plcrash_async_mobject_free
// type: int __fastcall(_DWORD)
pub fn stub_0xf1cdd4() -> ! { todo!("0xf1cdd4 _plcrash_async_mobject_free") }

#[doc(alias = "_plcrash_async_objc_cache_init")]
// 0xf1cdf8 — _plcrash_async_objc_cache_init
pub fn stub_0xf1cdf8() -> ! { todo!("0xf1cdf8 _plcrash_async_objc_cache_init") }

#[doc(alias = "_plcrash_async_objc_cache_free")]
// 0xf1ce14 — _plcrash_async_objc_cache_free
pub fn stub_0xf1ce14() -> ! { todo!("0xf1ce14 _plcrash_async_objc_cache_free") }

#[doc(alias = "_free_mapped_sections")]
// 0xf1ce40 — _free_mapped_sections
pub fn stub_0xf1ce40() -> ! { todo!("0xf1ce40 _free_mapped_sections") }

#[doc(alias = "_plcrash_async_objc_find_method")]
// 0xf1ce84 — _plcrash_async_objc_find_method
pub fn stub_0xf1ce84() -> ! { todo!("0xf1ce84 _plcrash_async_objc_find_method") }

#[doc(alias = "_plcrash_async_objc_parse")]
// 0xf1ced4 — _plcrash_async_objc_parse
pub fn stub_0xf1ced4() -> ! { todo!("0xf1ced4 _plcrash_async_objc_parse") }

#[doc(alias = "_pl_async_objc_find_method_search_callback")]
// 0xf1d1cc — _pl_async_objc_find_method_search_callback
pub fn stub_0xf1d1cc() -> ! { todo!("0xf1d1cc _pl_async_objc_find_method_search_callback") }

#[doc(alias = "_pl_async_objc_find_method_call_callback")]
// 0xf1d1e0 — _pl_async_objc_find_method_call_callback
pub fn stub_0xf1d1e0() -> ! { todo!("0xf1d1e0 _pl_async_objc_find_method_call_callback") }

#[doc(alias = "_pl_async_objc_parse_objc2_class")]
// 0xf1d200 — _pl_async_objc_parse_objc2_class
pub fn stub_0xf1d200() -> ! { todo!("0xf1d200 _pl_async_objc_parse_objc2_class") }

#[doc(alias = "_pl_async_parse_obj1_class")]
// 0xf1d4ac — _pl_async_parse_obj1_class
pub fn stub_0xf1d4ac() -> ! { todo!("0xf1d4ac _pl_async_parse_obj1_class") }

#[doc(alias = "_plcrash_async_symbol_cache_init")]
// 0xf1d5b0 — _plcrash_async_symbol_cache_init
pub fn stub_0xf1d5b0() -> ! { todo!("0xf1d5b0 _plcrash_async_symbol_cache_init") }

#[doc(alias = "_plcrash_async_symbol_cache_free")]
// 0xf1d5bc — _plcrash_async_symbol_cache_free
pub fn stub_0xf1d5bc() -> ! { todo!("0xf1d5bc _plcrash_async_symbol_cache_free") }

#[doc(alias = "_plcrash_async_find_symbol")]
// 0xf1d5c8 — _plcrash_async_find_symbol
pub fn stub_0xf1d5c8() -> ! { todo!("0xf1d5c8 _plcrash_async_find_symbol") }

#[doc(alias = "_macho_symbol_callback")]
// 0xf1d65c — _macho_symbol_callback
// type: unsigned int __fastcall(unsigned int result, _BYTE *, int)
pub fn stub_0xf1d65c() -> ! { todo!("0xf1d65c _macho_symbol_callback") }

#[doc(alias = "_objc_symbol_callback")]
// 0xf1d6a4 — _objc_symbol_callback
pub fn stub_0xf1d6a4() -> ! { todo!("0xf1d6a4 _objc_symbol_callback") }

#[doc(alias = "_plcrash_async_macho_string_init")]
// 0xf1d7a4 — _plcrash_async_macho_string_init
pub fn stub_0xf1d7a4() -> ! { todo!("0xf1d7a4 _plcrash_async_macho_string_init") }

#[doc(alias = "_plcrash_async_macho_string_get_length")]
// 0xf1d7b8 — _plcrash_async_macho_string_get_length
pub fn stub_0xf1d7b8() -> ! { todo!("0xf1d7b8 _plcrash_async_macho_string_get_length") }

#[doc(alias = "_plcrash_async_macho_string_read")]
// 0xf1d7d0 — _plcrash_async_macho_string_read
pub fn stub_0xf1d7d0() -> ! { todo!("0xf1d7d0 _plcrash_async_macho_string_read") }

#[doc(alias = "_plcrash_async_macho_string_get_pointer")]
// 0xf1d848 — _plcrash_async_macho_string_get_pointer
pub fn stub_0xf1d848() -> ! { todo!("0xf1d848 _plcrash_async_macho_string_get_pointer") }

#[doc(alias = "_plcrash_async_macho_string_free")]
// 0xf1d878 — _plcrash_async_macho_string_free
pub fn stub_0xf1d878() -> ! { todo!("0xf1d878 _plcrash_async_macho_string_free") }

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo initWithInstructionPointer:symbolInfo:]")]
// 0xf1d88c — -[FlurryPLCrashReportStackFrameInfo initWithInstructionPointer:symbolInfo:]
// type: FlurryPLCrashReportStackFrameInfo *__cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL, unsigned __int64, id)
pub fn stub_0xf1d88c() -> ! { todo!("0xf1d88c -[FlurryPLCrashReportStackFrameInfo initWithInstructionPointer:symbolInfo:]") }

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo dealloc]")]
// 0xf1d8f8 — -[FlurryPLCrashReportStackFrameInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL)
pub fn stub_0xf1d8f8() -> ! { todo!("0xf1d8f8 -[FlurryPLCrashReportStackFrameInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo instructionPointer]")]
// 0xf1d944 — -[FlurryPLCrashReportStackFrameInfo instructionPointer]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL)
pub fn stub_0xf1d944() -> ! { todo!("0xf1d944 -[FlurryPLCrashReportStackFrameInfo instructionPointer]") }

#[doc(alias = "-[FlurryPLCrashReportStackFrameInfo symbolInfo]")]
// 0xf1d95c — -[FlurryPLCrashReportStackFrameInfo symbolInfo]
// type: FlurryPLCrashReportSymbolInfo *__cdecl(FlurryPLCrashReportStackFrameInfo *self, SEL)
pub fn stub_0xf1d95c() -> ! { todo!("0xf1d95c -[FlurryPLCrashReportStackFrameInfo symbolInfo]") }

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo initWithRegisterName:registerValue:]")]
// 0xf1d96c — -[FlurryPLCrashReportRegisterInfo initWithRegisterName:registerValue:]
// type: FlurryPLCrashReportRegisterInfo *__cdecl(FlurryPLCrashReportRegisterInfo *self, SEL, id, unsigned __int64)
pub fn stub_0xf1d96c() -> ! { todo!("0xf1d96c -[FlurryPLCrashReportRegisterInfo initWithRegisterName:registerValue:]") }

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo dealloc]")]
// 0xf1d9e4 — -[FlurryPLCrashReportRegisterInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xf1d9e4() -> ! { todo!("0xf1d9e4 -[FlurryPLCrashReportRegisterInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo registerName]")]
// 0xf1da30 — -[FlurryPLCrashReportRegisterInfo registerName]
// type: NSString *__cdecl(FlurryPLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xf1da30() -> ! { todo!("0xf1da30 -[FlurryPLCrashReportRegisterInfo registerName]") }

#[doc(alias = "-[FlurryPLCrashReportRegisterInfo registerValue]")]
// 0xf1da40 — -[FlurryPLCrashReportRegisterInfo registerValue]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportRegisterInfo *self, SEL)
pub fn stub_0xf1da40() -> ! { todo!("0xf1da40 -[FlurryPLCrashReportRegisterInfo registerValue]") }

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo initWithSymbolName:startAddress:endAddress:]")]
// 0xf1da58 — -[FlurryPLCrashReportSymbolInfo initWithSymbolName:startAddress:endAddress:]
// type: FlurryPLCrashReportSymbolInfo *__cdecl(FlurryPLCrashReportSymbolInfo *self, SEL, id, unsigned __int64, unsigned __int64)
pub fn stub_0xf1da58() -> ! { todo!("0xf1da58 -[FlurryPLCrashReportSymbolInfo initWithSymbolName:startAddress:endAddress:]") }

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo dealloc]")]
// 0xf1daf0 — -[FlurryPLCrashReportSymbolInfo dealloc]
// type: void __cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1daf0() -> ! { todo!("0xf1daf0 -[FlurryPLCrashReportSymbolInfo dealloc]") }

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo symbolName]")]
// 0xf1db3c — -[FlurryPLCrashReportSymbolInfo symbolName]
// type: NSString *__cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1db3c() -> ! { todo!("0xf1db3c -[FlurryPLCrashReportSymbolInfo symbolName]") }

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo startAddress]")]
// 0xf1db4c — -[FlurryPLCrashReportSymbolInfo startAddress]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1db4c() -> ! { todo!("0xf1db4c -[FlurryPLCrashReportSymbolInfo startAddress]") }

#[doc(alias = "-[FlurryPLCrashReportSymbolInfo endAddress]")]
// 0xf1db64 — -[FlurryPLCrashReportSymbolInfo endAddress]
// type: unsigned __int64 __cdecl(FlurryPLCrashReportSymbolInfo *self, SEL)
pub fn stub_0xf1db64() -> ! { todo!("0xf1db64 -[FlurryPLCrashReportSymbolInfo endAddress]") }

#[doc(alias = "-[FlurryPLCrashMachExceptionServer init]")]
// 0xf1db7c — -[FlurryPLCrashMachExceptionServer init]
// type: FlurryPLCrashMachExceptionServer *__cdecl(FlurryPLCrashMachExceptionServer *self, SEL)
pub fn stub_0xf1db7c() -> ! { todo!("0xf1db7c -[FlurryPLCrashMachExceptionServer init]") }

#[doc(alias = "-[FlurryPLCrashMachExceptionServer registerHandlerForTask:thread:withCallback:context:error:]")]
// 0xf1dba8 — -[FlurryPLCrashMachExceptionServer registerHandlerForTask:thread:withCallback:context:error:]
// type: char __cdecl(FlurryPLCrashMachExceptionServer *self, SEL, unsigned int, unsigned int, void *, void *, id *)
pub fn stub_0xf1dba8() -> ! { todo!("0xf1dba8 -[FlurryPLCrashMachExceptionServer registerHandlerForTask:thread:withCallback:context:error:]") }

#[doc(alias = "_exception_server_thread")]
// 0xf1de4c — _exception_server_thread
pub fn stub_0xf1de4c() -> ! { todo!("0xf1de4c _exception_server_thread") }

#[doc(alias = "-[FlurryPLCrashMachExceptionServer deregisterHandlerAndReturnError:]")]
// 0xf1e478 — -[FlurryPLCrashMachExceptionServer deregisterHandlerAndReturnError:]
// type: char __cdecl(FlurryPLCrashMachExceptionServer *self, SEL, id *)
pub fn stub_0xf1e478() -> ! { todo!("0xf1e478 -[FlurryPLCrashMachExceptionServer deregisterHandlerAndReturnError:]") }

#[doc(alias = "_set_exception_ports")]
// 0xf1e5c0 — _set_exception_ports
// type: int __fastcall(task_t task, thread_act_t thread, exception_mask_t exception_mask)
pub fn stub_0xf1e5c0() -> ! { todo!("0xf1e5c0 _set_exception_ports") }

#[doc(alias = "_plcrash_async_allocator_new")]
// 0xf1e648 — _plcrash_async_allocator_new
pub fn stub_0xf1e648() -> ! { todo!("0xf1e648 _plcrash_async_allocator_new") }

#[doc(alias = "_plcrash_async_allocator_alloc")]
// 0xf1e750 — _plcrash_async_allocator_alloc
pub fn stub_0xf1e750() -> ! { todo!("0xf1e750 _plcrash_async_allocator_alloc") }

#[doc(alias = "+[__ARCLite__ load]")]
// 0xf1e79c — +[__ARCLite__ load]
// type: void __cdecl(id, SEL)
pub fn stub_0xf1e79c() -> ! { todo!("0xf1e79c +[__ARCLite__ load]") }

#[doc(alias = "_add_image_hook_ARC")]
// 0xf1e9d0 — _add_image_hook_ARC
pub fn stub_0xf1e9d0() -> ! { todo!("0xf1e9d0 _add_image_hook_ARC") }

#[doc(alias = "___arclite_NSArray_objectAtIndexedSubscript")]
// 0xf1e9e8 — ___arclite_NSArray_objectAtIndexedSubscript
pub fn stub_0xf1e9e8() -> ! { todo!("0xf1e9e8 ___arclite_NSArray_objectAtIndexedSubscript") }

#[doc(alias = "___arclite_NSMutableArray_setObject_atIndexedSubscript")]
// 0xf1ea00 — ___arclite_NSMutableArray_setObject_atIndexedSubscript
// type: int __fastcall(id)
pub fn stub_0xf1ea00() -> ! { todo!("0xf1ea00 ___arclite_NSMutableArray_setObject_atIndexedSubscript") }

#[doc(alias = "___arclite_NSDictionary_objectForKeyedSubscript")]
// 0xf1ea50 — ___arclite_NSDictionary_objectForKeyedSubscript
pub fn stub_0xf1ea50() -> ! { todo!("0xf1ea50 ___arclite_NSDictionary_objectForKeyedSubscript") }

#[doc(alias = "___arclite_NSMutableDictionary__setObject_forKeyedSubscript")]
// 0xf1ea68 — ___arclite_NSMutableDictionary__setObject_forKeyedSubscript
pub fn stub_0xf1ea68() -> ! { todo!("0xf1ea68 ___arclite_NSMutableDictionary__setObject_forKeyedSubscript") }

#[doc(alias = "___arclite_NSOrderedSet_objectAtIndexedSubscript")]
// 0xf1ea80 — ___arclite_NSOrderedSet_objectAtIndexedSubscript
pub fn stub_0xf1ea80() -> ! { todo!("0xf1ea80 ___arclite_NSOrderedSet_objectAtIndexedSubscript") }

#[doc(alias = "___arclite_NSMutableOrderedSet_setObject_atIndexedSubscript")]
// 0xf1ea98 — ___arclite_NSMutableOrderedSet_setObject_atIndexedSubscript
pub fn stub_0xf1ea98() -> ! { todo!("0xf1ea98 ___arclite_NSMutableOrderedSet_setObject_atIndexedSubscript") }

#[doc(alias = "___arclite_objc_autoreleasePoolPop")]
// 0xf1eab0 — ___arclite_objc_autoreleasePoolPop
pub fn stub_0xf1eab0() -> ! { todo!("0xf1eab0 ___arclite_objc_autoreleasePoolPop") }

#[doc(alias = "_patch_lazy_pointers")]
// 0xf1eac8 — _patch_lazy_pointers
pub fn stub_0xf1eac8() -> ! { todo!("0xf1eac8 _patch_lazy_pointers") }

#[doc(alias = "___arclite_objc_autoreleasePoolPush")]
// 0xf1ec64 — ___arclite_objc_autoreleasePoolPush
pub fn stub_0xf1ec64() -> ! { todo!("0xf1ec64 ___arclite_objc_autoreleasePoolPush") }

#[doc(alias = "___arclite_object_setIvar")]
// 0xf1ecc4 — ___arclite_object_setIvar
// type: int __fastcall(id)
pub fn stub_0xf1ecc4() -> ! { todo!("0xf1ecc4 ___arclite_object_setIvar") }

#[doc(alias = "___arclite_object_copy")]
// 0xf1edac — ___arclite_object_copy
// type: char *__fastcall(id, int)
pub fn stub_0xf1edac() -> ! { todo!("0xf1edac ___arclite_object_copy") }

#[doc(alias = "___arclite_objc_retain")]
// 0xf1eeb8 — ___arclite_objc_retain
pub fn stub_0xf1eeb8() -> ! { todo!("0xf1eeb8 ___arclite_objc_retain") }

#[doc(alias = "___arclite_objc_retainBlock")]
// 0xf1eed0 — ___arclite_objc_retainBlock
pub fn stub_0xf1eed0() -> ! { todo!("0xf1eed0 ___arclite_objc_retainBlock") }

#[doc(alias = "___arclite_objc_release")]
// 0xf1eedc — ___arclite_objc_release
pub fn stub_0xf1eedc() -> ! { todo!("0xf1eedc ___arclite_objc_release") }

#[doc(alias = "___arclite_objc_autorelease")]
// 0xf1eef4 — ___arclite_objc_autorelease
pub fn stub_0xf1eef4() -> ! { todo!("0xf1eef4 ___arclite_objc_autorelease") }

#[doc(alias = "___arclite_objc_retainAutorelease")]
// 0xf1ef0c — ___arclite_objc_retainAutorelease
pub fn stub_0xf1ef0c() -> ! { todo!("0xf1ef0c ___arclite_objc_retainAutorelease") }

#[doc(alias = "___arclite_objc_autoreleaseReturnValue")]
// 0xf1ef34 — ___arclite_objc_autoreleaseReturnValue
pub fn stub_0xf1ef34() -> ! { todo!("0xf1ef34 ___arclite_objc_autoreleaseReturnValue") }

#[doc(alias = "___arclite_objc_retainAutoreleaseReturnValue")]
// 0xf1ef4c — ___arclite_objc_retainAutoreleaseReturnValue
pub fn stub_0xf1ef4c() -> ! { todo!("0xf1ef4c ___arclite_objc_retainAutoreleaseReturnValue") }

#[doc(alias = "___arclite_objc_retainAutoreleasedReturnValue")]
// 0xf1ef74 — ___arclite_objc_retainAutoreleasedReturnValue
// type: id __fastcall(void *)
pub fn stub_0xf1ef74() -> ! { todo!("0xf1ef74 ___arclite_objc_retainAutoreleasedReturnValue") }

#[doc(alias = "___arclite_objc_storeStrong")]
// 0xf1ef8c — ___arclite_objc_storeStrong
pub fn stub_0xf1ef8c() -> ! { todo!("0xf1ef8c ___arclite_objc_storeStrong") }

#[doc(alias = "__ZNSt11logic_errorD2Ev$shim")]
// 0xf1efd0 — __ZNSt11logic_errorD2Ev$shim
// type: void __cdecl(std::logic_error *__hidden this)
pub fn stub_0xf1efd0() -> ! { todo!("0xf1efd0 __ZNSt11logic_errorD2Ev$shim") }

#[doc(alias = "__ZdlPv$shim")]
// 0xf1efdc — __ZdlPv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1efdc() -> ! { todo!("0xf1efdc __ZdlPv$shim") }

#[doc(alias = "__ZN19CRenderSettingsItemD2Ev$shim")]
// 0xf1eff4 — __ZN19CRenderSettingsItemD2Ev$shim
// type: void __fastcall(CRenderSettingsItem *__hidden this)
pub fn stub_0xf1eff4() -> ! { todo!("0xf1eff4 __ZN19CRenderSettingsItemD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v$shim")]
// 0xf1f06c — __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f06c() -> ! { todo!("0xf1f06c __ZN3RBX4Name9doDeclareILZ15sRenderSettingsEEERKS0_v$shim") }

#[doc(alias = "__Znwm$shim")]
// 0xf1f084 — __Znwm$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1f084() -> ! { todo!("0xf1f084 __Znwm$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f0f0 — __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f0f0() -> ! { todo!("0xf1f0f0 __ZNSt6vectorIN3RBX15CRenderSettings16ResolutionPresetESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f0fc — __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f0fc() -> ! { todo!("0xf1f0fc __ZNSt6vectorIN3RBX15CRenderSettings12QualityLevelESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f108 — __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f108() -> ! { todo!("0xf1f108 __ZNSt6vectorIN3RBX15CRenderSettings10ShadowModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f114 — __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f114() -> ! { todo!("0xf1f114 __ZNSt6vectorIN3RBX15CRenderSettings16AntialiasingModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f120 — __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f120() -> ! { todo!("0xf1f120 __ZNSt6vectorIN3RBX15CRenderSettings20FrameRateManagerModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f12c — __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int()
pub fn stub_0xf1f12c() -> ! { todo!("0xf1f12c __ZNSt6vectorIN3RBX15CRenderSettings12GraphicsModeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f138 — __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
pub fn stub_0xf1f138() -> ! { todo!("0xf1f138 __ZNSt6vectorIN3RBX15CRenderSettings9AASamplesESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__Block_object_assign$shim")]
// 0xf1f198 — __Block_object_assign$shim
// type: void __cdecl(void *, const void *, const int)
pub fn stub_0xf1f198() -> ! { todo!("0xf1f198 __Block_object_assign$shim") }

#[doc(alias = "__Block_object_dispose$shim")]
// 0xf1f1a4 — __Block_object_dispose$shim
// type: void __cdecl(const void *, const int)
pub fn stub_0xf1f1a4() -> ! { todo!("0xf1f1a4 __Block_object_dispose$shim") }

#[doc(alias = "_objc_msgSend$shim")]
// 0xf1f1b0 — _objc_msgSend$shim
// type: id(id, SEL, ...)
pub fn stub_0xf1f1b0() -> ! { todo!("0xf1f1b0 _objc_msgSend$shim") }

#[doc(alias = "_dispatch_async$shim")]
// 0xf1f1bc — _dispatch_async$shim
// type: void __cdecl(dispatch_queue_t queue, dispatch_block_t block)
pub fn stub_0xf1f1bc() -> ! { todo!("0xf1f1bc _dispatch_async$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v$shim")]
// 0xf1f210 — __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v$shim
pub fn stub_0xf1f210() -> ! { todo!("0xf1f210 __ZN3RBX4Name9doDeclareILZNS_13sLoginServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv$shim")]
// 0xf1f21c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv$shim
// type: int()
pub fn stub_0xf1f21c() -> ! { todo!("0xf1f21c __ZN3RBX15ServiceProvider15doGetClassIndexINS_12LoginServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v$shim")]
// 0xf1f234 — __ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v$shim
pub fn stub_0xf1f234() -> ! { todo!("0xf1f234 __ZN3RBX4Name9doDeclareILZNS_11sGuiServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv$shim")]
// 0xf1f240 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv$shim
pub fn stub_0xf1f240() -> ! { todo!("0xf1f240 __ZN3RBX15ServiceProvider15doGetClassIndexINS_10GuiServiceEEEmv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim")]
// 0xf1f24c — __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim
pub fn stub_0xf1f24c() -> ! { todo!("0xf1f24c __ZN3RBX4Name7declareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim")]
// 0xf1f258 — __ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim
pub fn stub_0xf1f258() -> ! { todo!("0xf1f258 __ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt13runtime_errorD2Ev$shim")]
// 0xf1f294 — __ZNSt13runtime_errorD2Ev$shim
// type: void __cdecl(std::runtime_error *__hidden this)
pub fn stub_0xf1f294() -> ! { todo!("0xf1f294 __ZNSt13runtime_errorD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v$shim")]
// 0xf1f2ac — __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v$shim
pub fn stub_0xf1f2ac() -> ! { todo!("0xf1f2ac __ZN3RBX4Name9doDeclareILZNS_11sRunServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v$shim")]
// 0xf1f2b8 — __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v$shim
pub fn stub_0xf1f2b8() -> ! { todo!("0xf1f2b8 __ZN3RBX4Name9doDeclareILZNS_18sControllerServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv$shim")]
// 0xf1f2d0 — __ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f2d0() -> ! { todo!("0xf1f2d0 __ZN3rbx7signals6signalIFvvEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f300 — __ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f300() -> ! { todo!("0xf1f300 __ZN3rbx7signals6signalIFvvEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "_free$shim")]
// 0xf1f324 — _free$shim
// type: void __cdecl(void *)
pub fn stub_0xf1f324() -> ! { todo!("0xf1f324 _free$shim") }

#[doc(alias = "_puts$shim")]
// 0xf1f33c — _puts$shim
// type: int __cdecl(const char *)
pub fn stub_0xf1f33c() -> ! { todo!("0xf1f33c _puts$shim") }

#[doc(alias = "__ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim")]
// 0xf1f348 — __ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim
pub fn stub_0xf1f348() -> ! { todo!("0xf1f348 __ZNK10RobloxView9RenderJob14getMetricValueERKSs$shim") }

#[doc(alias = "__ZNSt9exceptionD2Ev$shim")]
// 0xf1f354 — __ZNSt9exceptionD2Ev$shim
// type: void __cdecl(std::exception *__hidden this)
pub fn stub_0xf1f354() -> ! { todo!("0xf1f354 __ZNSt9exceptionD2Ev$shim") }

#[doc(alias = "__ZN18iOSSettingsServiceD2Ev$shim")]
// 0xf1f36c — __ZN18iOSSettingsServiceD2Ev$shim
pub fn stub_0xf1f36c() -> ! { todo!("0xf1f36c __ZN18iOSSettingsServiceD2Ev$shim") }

#[doc(alias = "__ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim")]
// 0xf1f378 — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim
pub fn stub_0xf1f378() -> ! { todo!("0xf1f378 __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f384 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f384() -> ! { todo!("0xf1f384 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "___cxa_atexit$shim")]
// 0xf1f3e4 — ___cxa_atexit$shim
// type: int __fastcall(void (__fastcall *lpfunc)(void *), void *obj, void *lpdso_handle)
pub fn stub_0xf1f3e4() -> ! { todo!("0xf1f3e4 ___cxa_atexit$shim") }

#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim")]
// 0xf1f3f0 — __ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1f3f0() -> ! { todo!("0xf1f3f0 __ZN3rbx7signals16signal_with_argsILi0EFvvEEclEv$shim") }

#[doc(alias = "__ZNSsC1ERKSs$shim")]
// 0xf1f3fc — __ZNSsC1ERKSs$shim
// type: int __fastcall(std::string *, const std::string *)
pub fn stub_0xf1f3fc() -> ! { todo!("0xf1f3fc __ZNSsC1ERKSs$shim") }

#[doc(alias = "__ZNSsD2Ev$shim")]
// 0xf1f408 — __ZNSsD2Ev$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1f408() -> ! { todo!("0xf1f408 __ZNSsD2Ev$shim") }

#[doc(alias = "__ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim")]
// 0xf1f414 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim
// type: int(void)
pub fn stub_0xf1f414() -> ! { todo!("0xf1f414 __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim")]
// 0xf1f420 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf1f420() -> ! { todo!("0xf1f420 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f438 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim
// type: int()
pub fn stub_0xf1f438() -> ! { todo!("0xf1f438 __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim")]
// 0xf1f45c — __ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f45c() -> ! { todo!("0xf1f45c __ZN3RBX4Name9doDeclareILZNS_12sHttpServiceEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
// 0xf1f468 — __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
// type: int(void)
pub fn stub_0xf1f468() -> ! { todo!("0xf1f468 __ZNSt6vectorIN3RBX11HttpService15HttpContentTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim")]
// 0xf1f474 — __ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f474() -> ! { todo!("0xf1f474 __ZN3RBX4Name9doDeclareILZNS_6sLightEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim")]
// 0xf1f4b0 — __ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f4b0() -> ! { todo!("0xf1f4b0 __ZN3RBX4Name9doDeclareILZNS_10sSpotLightEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim")]
// 0xf1f4bc — __ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f4bc() -> ! { todo!("0xf1f4bc __ZN3RBX4Name9doDeclareILZNS_11sPointLightEEEERKS0_v$shim") }

#[doc(alias = "___cxa_guard_release$shim")]
// 0xf1f510 — ___cxa_guard_release$shim
// type: void __fastcall(__guard *)
pub fn stub_0xf1f510() -> ! { todo!("0xf1f510 ___cxa_guard_release$shim") }

#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim")]
// 0xf1f7bc — __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0xf1f7bc() -> ! { todo!("0xf1f7bc __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v$shim") }

#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim")]
// 0xf1f7c8 — __ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim
pub fn stub_0xf1f7c8() -> ! { todo!("0xf1f7c8 __ZNK3RBX15ServiceProvider4findINS_5Stats12StatsServiceEEEPT_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
// 0xf1f8ac — __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1f8ac() -> ! { todo!("0xf1f8ac __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim")]
// 0xf1f8b8 — __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim
pub fn stub_0xf1f8b8() -> ! { todo!("0xf1f8b8 __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v$shim") }

#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim")]
// 0xf1f8d0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim
pub fn stub_0xf1f8d0() -> ! { todo!("0xf1f8d0 __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim")]
// 0xf1f8dc — __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim
pub fn stub_0xf1f8dc() -> ! { todo!("0xf1f8dc __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim")]
// 0xf1f8e8 — __ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim
pub fn stub_0xf1f8e8() -> ! { todo!("0xf1f8e8 __ZN3RBX4Name9doDeclareILZNS_16sContentProviderEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim")]
// 0xf1f8f4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim
pub fn stub_0xf1f8f4() -> ! { todo!("0xf1f8f4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv$shim") }

#[doc(alias = "__ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf1f9f0 — __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
pub fn stub_0xf1f9f0() -> ! { todo!("0xf1f9f0 __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim") }

#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
// 0xf1f9fc — __ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim
pub fn stub_0xf1f9fc() -> ! { todo!("0xf1f9fc __ZN3RBX4Name7declareILZNS_5Stats10sStatsItemEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim")]
// 0xf1fa08 — __ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim
pub fn stub_0xf1fa08() -> ! { todo!("0xf1fa08 __ZN3RBX4Name9doDeclareILZNS_5Stats10sStatsItemEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt5dequeISsSaISsEE9push_backERKSs$shim")]
// 0xf1fa2c — __ZNSt5dequeISsSaISsEE9push_backERKSs$shim
pub fn stub_0xf1fa2c() -> ! { todo!("0xf1fa2c __ZNSt5dequeISsSaISsEE9push_backERKSs$shim") }

#[doc(alias = "__ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim")]
// 0xf1fa44 — __ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim
pub fn stub_0xf1fa44() -> ! { todo!("0xf1fa44 __ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs$shim") }

#[doc(alias = "__ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim")]
// 0xf1fa50 — __ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim
pub fn stub_0xf1fa50() -> ! { todo!("0xf1fa50 __ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb$shim") }

#[doc(alias = "__ZNSs6assignERKSs$shim")]
// 0xf1faa4 — __ZNSs6assignERKSs$shim
// type: int __fastcall(_DWORD)
pub fn stub_0xf1faa4() -> ! { todo!("0xf1faa4 __ZNSs6assignERKSs$shim") }

#[doc(alias = "__ZN3RBX15AdvMoveToolBaseD2Ev$shim")]
// 0xf1fab0 — __ZN3RBX15AdvMoveToolBaseD2Ev$shim
// type: void __fastcall(RBX::AdvMoveToolBase *__hidden this)
pub fn stub_0xf1fab0() -> ! { todo!("0xf1fab0 __ZN3RBX15AdvMoveToolBaseD2Ev$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim")]
// 0xf1fabc — __ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fabc() -> ! { todo!("0xf1fabc __ZN3RBX4Name9doDeclareILZNS_10sCloneToolEEEERKS0_v$shim") }

#[doc(alias = "__ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim")]
// 0xf1fac8 — __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim
// type: int(void)
pub fn stub_0xf1fac8() -> ! { todo!("0xf1fac8 __ZNSt6vectorIN3RBX7ExtentsESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim")]
// 0xf1fae0 — __ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fae0() -> ! { todo!("0xf1fae0 __ZN3RBX4Name9doDeclareILZNS_9sGameToolEEEERKS0_v$shim") }

#[doc(alias = "__ZNSs6assignEPKcm$shim")]
// 0xf1faec — __ZNSs6assignEPKcm$shim
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf1faec() -> ! { todo!("0xf1faec __ZNSs6assignEPKcm$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim")]
// 0xf1faf8 — __ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1faf8() -> ! { todo!("0xf1faf8 __ZN3RBX4Name9doDeclareILZNS_9sGrabToolEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim")]
// 0xf1fb04 — __ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fb04() -> ! { todo!("0xf1fb04 __ZN3RBX4Name9doDeclareILZNS_11sHammerToolEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim")]
// 0xf1fb4c — __ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim
// type: int()
pub fn stub_0xf1fb4c() -> ! { todo!("0xf1fb4c __ZN3RBX4Name9doDeclareILZNS_9sNullToolEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim")]
// 0xf1fb58 — __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim
pub fn stub_0xf1fb58() -> ! { todo!("0xf1fb58 __ZN3RBX4Name9doDeclareILZNS_12sNewNullToolEEEERKS0_v$shim") }

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim")]
// 0xf1fb64 — __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim
pub fn stub_0xf1fb64() -> ! { todo!("0xf1fb64 __ZN3RBX4Name9doDeclareILZNS_13sPartDragToolEEEERKS0_v$shim") }
