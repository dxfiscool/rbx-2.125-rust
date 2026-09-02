//! core shard ml — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 2873 uncovered before -> 2773 after, batch 0xf1311c..0xf15dbc).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, unstable_name_collisions, clippy::all, unused_attributes)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[FlurryGlobalVariableStorage age]")]
// 0xf1311c — -[FlurryGlobalVariableStorage age]
// type: int __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf1311c() -> ! { todo!("0xf1311c -[FlurryGlobalVariableStorage age]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setAge:]")]
// 0xf13130 — -[FlurryGlobalVariableStorage setAge:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, int)
pub fn stub_0xf13130() -> ! { todo!("0xf13130 -[FlurryGlobalVariableStorage setAge:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage gender]")]
// 0xf13148 — -[FlurryGlobalVariableStorage gender]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf13148() -> ! { todo!("0xf13148 -[FlurryGlobalVariableStorage gender]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setGender:]")]
// 0xf13160 — -[FlurryGlobalVariableStorage setGender:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf13160() -> ! { todo!("0xf13160 -[FlurryGlobalVariableStorage setGender:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage startSessionCalled]")]
// 0xf13184 — -[FlurryGlobalVariableStorage startSessionCalled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf13184() -> ! { todo!("0xf13184 -[FlurryGlobalVariableStorage startSessionCalled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setStartSessionCalled:]")]
// 0xf1319c — -[FlurryGlobalVariableStorage setStartSessionCalled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf1319c() -> ! { todo!("0xf1319c -[FlurryGlobalVariableStorage setStartSessionCalled:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage launchOptions]")]
// 0xf131b4 — -[FlurryGlobalVariableStorage launchOptions]
// type: NSMutableDictionary *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf131b4() -> ! { todo!("0xf131b4 -[FlurryGlobalVariableStorage launchOptions]") }

#[doc(alias = "-[FlurryGlobalVariableStorage pushToken]")]
// 0xf131c8 — -[FlurryGlobalVariableStorage pushToken]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf131c8() -> ! { todo!("0xf131c8 -[FlurryGlobalVariableStorage pushToken]") }

#[doc(alias = "-[FlurryGlobalVariableStorage location]")]
// 0xf131e0 — -[FlurryGlobalVariableStorage location]
// type: id __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf131e0() -> ! { todo!("0xf131e0 -[FlurryGlobalVariableStorage location]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setLocation:]")]
// 0xf131f8 — -[FlurryGlobalVariableStorage setLocation:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf131f8() -> ! { todo!("0xf131f8 -[FlurryGlobalVariableStorage setLocation:]") }

#[doc(alias = "-[FlurryGlobalVariableStorage macAddressEnabled]")]
// 0xf1321c — -[FlurryGlobalVariableStorage macAddressEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf1321c() -> ! { todo!("0xf1321c -[FlurryGlobalVariableStorage macAddressEnabled]") }

#[doc(alias = "-[FlurryGlobalVariableStorage setMacAddressEnabled:]")]
// 0xf13234 — -[FlurryGlobalVariableStorage setMacAddressEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf13234() -> ! { todo!("0xf13234 -[FlurryGlobalVariableStorage setMacAddressEnabled:]") }

#[doc(alias = "-[FlurryError init]")]
// 0xf1324c — -[FlurryError init]
// type: FlurryError *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf1324c() -> ! { todo!("0xf1324c -[FlurryError init]") }

#[doc(alias = "-[FlurryError initWithCoder:]")]
// 0xf132f0 — -[FlurryError initWithCoder:]
// type: FlurryError *__cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf132f0() -> ! { todo!("0xf132f0 -[FlurryError initWithCoder:]") }

#[doc(alias = "-[FlurryError encodeWithCoder:]")]
// 0xf13454 — -[FlurryError encodeWithCoder:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13454() -> ! { todo!("0xf13454 -[FlurryError encodeWithCoder:]") }

#[doc(alias = "+[FlurryError errorWithString:message:exceptionString:errorType:reportData:]")]
// 0xf1358c — +[FlurryError errorWithString:message:exceptionString:errorType:reportData:]
// type: id __cdecl(id, SEL, id, id, id, int, id)
pub fn stub_0xf1358c() -> ! { todo!("0xf1358c +[FlurryError errorWithString:message:exceptionString:errorType:reportData:]") }

#[doc(alias = "-[FlurryError appendToData:]")]
// 0xf13620 — -[FlurryError appendToData:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13620() -> ! { todo!("0xf13620 -[FlurryError appendToData:]") }

#[doc(alias = "-[FlurryError description]")]
// 0xf137dc — -[FlurryError description]
// type: id __cdecl(FlurryError *self, SEL)
pub fn stub_0xf137dc() -> ! { todo!("0xf137dc -[FlurryError description]") }

#[doc(alias = "-[FlurryError dealloc]")]
// 0xf138a0 — -[FlurryError dealloc]
// type: void __cdecl(FlurryError *self, SEL)
pub fn stub_0xf138a0() -> ! { todo!("0xf138a0 -[FlurryError dealloc]") }

#[doc(alias = "-[FlurryError errorID]")]
// 0xf13934 — -[FlurryError errorID]
// type: int __cdecl(FlurryError *self, SEL)
pub fn stub_0xf13934() -> ! { todo!("0xf13934 -[FlurryError errorID]") }

#[doc(alias = "-[FlurryError setErrorID:]")]
// 0xf13944 — -[FlurryError setErrorID:]
// type: void __cdecl(FlurryError *self, SEL, int)
pub fn stub_0xf13944() -> ! { todo!("0xf13944 -[FlurryError setErrorID:]") }

#[doc(alias = "-[FlurryError date]")]
// 0xf13954 — -[FlurryError date]
// type: NSDate *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf13954() -> ! { todo!("0xf13954 -[FlurryError date]") }

#[doc(alias = "-[FlurryError setDate:]")]
// 0xf13964 — -[FlurryError setDate:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13964() -> ! { todo!("0xf13964 -[FlurryError setDate:]") }

#[doc(alias = "-[FlurryError errorString]")]
// 0xf13988 — -[FlurryError errorString]
// type: NSString *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf13988() -> ! { todo!("0xf13988 -[FlurryError errorString]") }

#[doc(alias = "-[FlurryError setErrorString:]")]
// 0xf13998 — -[FlurryError setErrorString:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13998() -> ! { todo!("0xf13998 -[FlurryError setErrorString:]") }

#[doc(alias = "-[FlurryError errorMessage]")]
// 0xf139bc — -[FlurryError errorMessage]
// type: NSString *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf139bc() -> ! { todo!("0xf139bc -[FlurryError errorMessage]") }

#[doc(alias = "-[FlurryError setErrorMessage:]")]
// 0xf139cc — -[FlurryError setErrorMessage:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf139cc() -> ! { todo!("0xf139cc -[FlurryError setErrorMessage:]") }

#[doc(alias = "-[FlurryError exceptionString]")]
// 0xf139f0 — -[FlurryError exceptionString]
// type: NSString *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf139f0() -> ! { todo!("0xf139f0 -[FlurryError exceptionString]") }

#[doc(alias = "-[FlurryError setExceptionString:]")]
// 0xf13a00 — -[FlurryError setExceptionString:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13a00() -> ! { todo!("0xf13a00 -[FlurryError setExceptionString:]") }

#[doc(alias = "-[FlurryError errorType]")]
// 0xf13a24 — -[FlurryError errorType]
// type: int __cdecl(FlurryError *self, SEL)
pub fn stub_0xf13a24() -> ! { todo!("0xf13a24 -[FlurryError errorType]") }

#[doc(alias = "-[FlurryError setErrorType:]")]
// 0xf13a34 — -[FlurryError setErrorType:]
// type: void __cdecl(FlurryError *self, SEL, int)
pub fn stub_0xf13a34() -> ! { todo!("0xf13a34 -[FlurryError setErrorType:]") }

#[doc(alias = "-[FlurryError reportData]")]
// 0xf13a44 — -[FlurryError reportData]
// type: NSData *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf13a44() -> ! { todo!("0xf13a44 -[FlurryError reportData]") }

#[doc(alias = "-[FlurryError setReportData:]")]
// 0xf13a54 — -[FlurryError setReportData:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13a54() -> ! { todo!("0xf13a54 -[FlurryError setReportData:]") }

#[doc(alias = "+[FlurryKeychainWrapper keychainWrapperWithIdentifier:]")]
// 0xf13a78 — +[FlurryKeychainWrapper keychainWrapperWithIdentifier:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf13a78() -> ! { todo!("0xf13a78 +[FlurryKeychainWrapper keychainWrapperWithIdentifier:]") }

#[doc(alias = "-[FlurryKeychainWrapper initWithIdentifier:]")]
// 0xf13ac0 — -[FlurryKeychainWrapper initWithIdentifier:]
// type: FlurryKeychainWrapper *__cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13ac0() -> ! { todo!("0xf13ac0 -[FlurryKeychainWrapper initWithIdentifier:]") }

#[doc(alias = "-[FlurryKeychainWrapper dealloc]")]
// 0xf13bb4 — -[FlurryKeychainWrapper dealloc]
// type: void __cdecl(FlurryKeychainWrapper *self, SEL)
pub fn stub_0xf13bb4() -> ! { todo!("0xf13bb4 -[FlurryKeychainWrapper dealloc]") }

#[doc(alias = "-[FlurryKeychainWrapper setupSearchForKey:]")]
// 0xf13bf8 — -[FlurryKeychainWrapper setupSearchForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13bf8() -> ! { todo!("0xf13bf8 -[FlurryKeychainWrapper setupSearchForKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper dataForKey:]")]
// 0xf13ce0 — -[FlurryKeychainWrapper dataForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13ce0() -> ! { todo!("0xf13ce0 -[FlurryKeychainWrapper dataForKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper stringForKey:]")]
// 0xf13d88 — -[FlurryKeychainWrapper stringForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13d88() -> ! { todo!("0xf13d88 -[FlurryKeychainWrapper stringForKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper setObject:forKey:]")]
// 0xf13de8 — -[FlurryKeychainWrapper setObject:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13de8() -> ! { todo!("0xf13de8 -[FlurryKeychainWrapper setObject:forKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper objectForKey:]")]
// 0xf13e30 — -[FlurryKeychainWrapper objectForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13e30() -> ! { todo!("0xf13e30 -[FlurryKeychainWrapper objectForKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper setString:forKey:]")]
// 0xf13e6c — -[FlurryKeychainWrapper setString:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13e6c() -> ! { todo!("0xf13e6c -[FlurryKeychainWrapper setString:forKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper setData:forKey:]")]
// 0xf13ea0 — -[FlurryKeychainWrapper setData:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13ea0() -> ! { todo!("0xf13ea0 -[FlurryKeychainWrapper setData:forKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper updateValueData:forKey:]")]
// 0xf13f68 — -[FlurryKeychainWrapper updateValueData:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13f68() -> ! { todo!("0xf13f68 -[FlurryKeychainWrapper updateValueData:forKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper removeObjectForKey:]")]
// 0xf13fd8 — -[FlurryKeychainWrapper removeObjectForKey:]
// type: void __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13fd8() -> ! { todo!("0xf13fd8 -[FlurryKeychainWrapper removeObjectForKey:]") }

#[doc(alias = "-[FlurryKeychainWrapper identifier]")]
// 0xf13ff4 — -[FlurryKeychainWrapper identifier]
// type: NSString *__cdecl(FlurryKeychainWrapper *self, SEL)
pub fn stub_0xf13ff4() -> ! { todo!("0xf13ff4 -[FlurryKeychainWrapper identifier]") }

#[doc(alias = "-[FlurryKeychainWrapper setIdentifier:]")]
// 0xf14004 — -[FlurryKeychainWrapper setIdentifier:]
// type: void __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf14004() -> ! { todo!("0xf14004 -[FlurryKeychainWrapper setIdentifier:]") }

#[doc(alias = "+[FlurryPLCrashSignalHandler sharedHandler]")]
// 0xf14030 — +[FlurryPLCrashSignalHandler sharedHandler]
// type: id __cdecl(id, SEL)
pub fn stub_0xf14030() -> ! { todo!("0xf14030 +[FlurryPLCrashSignalHandler sharedHandler]") }

#[doc(alias = "-[FlurryPLCrashSignalHandler init]")]
// 0xf14080 — -[FlurryPLCrashSignalHandler init]
// type: FlurryPLCrashSignalHandler *__cdecl(FlurryPLCrashSignalHandler *self, SEL)
pub fn stub_0xf14080() -> ! { todo!("0xf14080 -[FlurryPLCrashSignalHandler init]") }

#[doc(alias = "-[FlurryPLCrashSignalHandler registerHandlerForSignal:error:]")]
// 0xf14104 — -[FlurryPLCrashSignalHandler registerHandlerForSignal:error:]
// type: char __cdecl(FlurryPLCrashSignalHandler *self, SEL, int, id *)
pub fn stub_0xf14104() -> ! { todo!("0xf14104 -[FlurryPLCrashSignalHandler registerHandlerForSignal:error:]") }

#[doc(alias = "_fatal_signal_handler_0")]
// 0xf14170 — _fatal_signal_handler_0
pub fn stub_0xf14170() -> ! { todo!("0xf14170 _fatal_signal_handler_0") }

#[doc(alias = "-[FlurryPLCrashSignalHandler registerHandlerWithCallback:context:error:]")]
// 0xf141e4 — -[FlurryPLCrashSignalHandler registerHandlerWithCallback:context:error:]
// type: char __cdecl(FlurryPLCrashSignalHandler *self, SEL, void *, void *, id *)
pub fn stub_0xf141e4() -> ! { todo!("0xf141e4 -[FlurryPLCrashSignalHandler registerHandlerWithCallback:context:error:]") }

#[doc(alias = "_plframe_strerror_0")]
// 0xf142b4 — _plframe_strerror_0
pub fn stub_0xf142b4() -> ! { todo!("0xf142b4 _plframe_strerror_0") }

#[doc(alias = "_plframe_test_thread_spawn_0")]
// 0xf142d4 — _plframe_test_thread_spawn_0
pub fn stub_0xf142d4() -> ! { todo!("0xf142d4 _plframe_test_thread_spawn_0") }

#[doc(alias = "_test_stack_thr_0")]
// 0xf1431c — _test_stack_thr_0
pub fn stub_0xf1431c() -> ! { todo!("0xf1431c _test_stack_thr_0") }

#[doc(alias = "_plframe_test_thread_stop_0")]
// 0xf14344 — _plframe_test_thread_stop_0
pub fn stub_0xf14344() -> ! { todo!("0xf14344 _plframe_test_thread_stop_0") }

#[doc(alias = "_plframe_cursor_init_0")]
// 0xf1436c — _plframe_cursor_init_0
pub fn stub_0xf1436c() -> ! { todo!("0xf1436c _plframe_cursor_init_0") }

#[doc(alias = "_plframe_cursor_thread_init_0")]
// 0xf1437c — _plframe_cursor_thread_init_0
// type: int __fastcall(int, thread_act_t target_act)
pub fn stub_0xf1437c() -> ! { todo!("0xf1437c _plframe_cursor_thread_init_0") }

#[doc(alias = "_plframe_cursor_next_0")]
// 0xf143e8 — _plframe_cursor_next_0
pub fn stub_0xf143e8() -> ! { todo!("0xf143e8 _plframe_cursor_next_0") }

#[doc(alias = "_plframe_get_reg_0")]
// 0xf14454 — _plframe_get_reg_0
// type: int __fastcall(_DWORD *, int, _DWORD *)
pub fn stub_0xf14454() -> ! { todo!("0xf14454 _plframe_get_reg_0") }

#[doc(alias = "_plframe_get_freg_0")]
// 0xf144ac — _plframe_get_freg_0
pub fn stub_0xf144ac() -> ! { todo!("0xf144ac _plframe_get_freg_0") }

#[doc(alias = "_plframe_get_regname_0")]
// 0xf144b0 — _plframe_get_regname_0
pub fn stub_0xf144b0() -> ! { todo!("0xf144b0 _plframe_get_regname_0") }

#[doc(alias = "_plcrash_log_writer_init_0")]
// 0xf145a0 — _plcrash_log_writer_init_0
// type: int __fastcall(void *__b)
pub fn stub_0xf145a0() -> ! { todo!("0xf145a0 _plcrash_log_writer_init_0") }

#[doc(alias = "_plcrash_log_writer_set_exception_0")]
// 0xf14798 — _plcrash_log_writer_set_exception_0
// type: int __fastcall(int, id)
pub fn stub_0xf14798() -> ! { todo!("0xf14798 _plcrash_log_writer_set_exception_0") }

#[doc(alias = "_plcrash_log_writer_close_0")]
// 0xf14954 — _plcrash_log_writer_close_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf14954() -> ! { todo!("0xf14954 _plcrash_log_writer_close_0") }

#[doc(alias = "_plcrash_log_writer_free_0")]
// 0xf14958 — _plcrash_log_writer_free_0
pub fn stub_0xf14958() -> ! { todo!("0xf14958 _plcrash_log_writer_free_0") }

#[doc(alias = "_plcrash_log_writer_write_0")]
// 0xf149d8 — _plcrash_log_writer_write_0
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf149d8() -> ! { todo!("0xf149d8 _plcrash_log_writer_write_0") }

#[doc(alias = "_plcrash_writer_write_system_info_0")]
// 0xf14d00 — _plcrash_writer_write_system_info_0
pub fn stub_0xf14d00() -> ! { todo!("0xf14d00 _plcrash_writer_write_system_info_0") }

#[doc(alias = "_plcrash_writer_write_machine_info_0")]
// 0xf14d94 — _plcrash_writer_write_machine_info_0
pub fn stub_0xf14d94() -> ! { todo!("0xf14d94 _plcrash_writer_write_machine_info_0") }

#[doc(alias = "_plcrash_writer_write_process_info_0")]
// 0xf14e10 — _plcrash_writer_write_process_info_0
pub fn stub_0xf14e10() -> ! { todo!("0xf14e10 _plcrash_writer_write_process_info_0") }

#[doc(alias = "_plcrash_writer_write_thread_0")]
// 0xf14e9c — _plcrash_writer_write_thread_0
pub fn stub_0xf14e9c() -> ! { todo!("0xf14e9c _plcrash_writer_write_thread_0") }

#[doc(alias = "_plcrash_writer_write_binary_image_0")]
// 0xf15020 — _plcrash_writer_write_binary_image_0
pub fn stub_0xf15020() -> ! { todo!("0xf15020 _plcrash_writer_write_binary_image_0") }

#[doc(alias = "_plcrash_writer_write_exception_0")]
// 0xf150e4 — _plcrash_writer_write_exception_0
pub fn stub_0xf150e4() -> ! { todo!("0xf150e4 _plcrash_writer_write_exception_0") }

#[doc(alias = "_plcrash_writer_write_signal_0")]
// 0xf151a4 — _plcrash_writer_write_signal_0
pub fn stub_0xf151a4() -> ! { todo!("0xf151a4 _plcrash_writer_write_signal_0") }

#[doc(alias = "_plcrash_writer_write_thread_frame")]
// 0xf15270 — _plcrash_writer_write_thread_frame
pub fn stub_0xf15270() -> ! { todo!("0xf15270 _plcrash_writer_write_thread_frame") }

#[doc(alias = "_plcrash_writer_write_thread_frame_symbol_cb")]
// 0xf1531c — _plcrash_writer_write_thread_frame_symbol_cb
pub fn stub_0xf1531c() -> ! { todo!("0xf1531c _plcrash_writer_write_thread_frame_symbol_cb") }

#[doc(alias = "_plcrash_writer_write_processor_info_0")]
// 0xf1535c — _plcrash_writer_write_processor_info_0
pub fn stub_0xf1535c() -> ! { todo!("0xf1535c _plcrash_writer_write_processor_info_0") }

#[doc(alias = "_plcrash_async_strerror")]
// 0xf153ac — _plcrash_async_strerror
pub fn stub_0xf153ac() -> ! { todo!("0xf153ac _plcrash_async_strerror") }

#[doc(alias = "_plcrash_async_read_addr")]
// 0xf153cc — _plcrash_async_read_addr
// type: int __fastcall(int, int, vm_address_t data, vm_size_t size)
pub fn stub_0xf153cc() -> ! { todo!("0xf153cc _plcrash_async_read_addr") }

#[doc(alias = "_plcrash_async_strncmp")]
// 0xf153e8 — _plcrash_async_strncmp
// type: int __fastcall(unsigned __int8 *, unsigned __int8 *, int)
pub fn stub_0xf153e8() -> ! { todo!("0xf153e8 _plcrash_async_strncmp") }

#[doc(alias = "_plcrash_async_memcpy_0")]
// 0xf15420 — _plcrash_async_memcpy_0
pub fn stub_0xf15420() -> ! { todo!("0xf15420 _plcrash_async_memcpy_0") }

#[doc(alias = "_plcrash_async_writen")]
// 0xf15434 — _plcrash_async_writen
pub fn stub_0xf15434() -> ! { todo!("0xf15434 _plcrash_async_writen") }

#[doc(alias = "_plcrash_async_file_init_0")]
// 0xf15470 — _plcrash_async_file_init_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf15470() -> ! { todo!("0xf15470 _plcrash_async_file_init_0") }

#[doc(alias = "_plcrash_async_file_write_0")]
// 0xf15484 — _plcrash_async_file_write_0
pub fn stub_0xf15484() -> ! { todo!("0xf15484 _plcrash_async_file_write_0") }

#[doc(alias = "_plcrash_async_file_flush_0")]
// 0xf15538 — _plcrash_async_file_flush_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf15538() -> ! { todo!("0xf15538 _plcrash_async_file_flush_0") }

#[doc(alias = "_plcrash_async_file_close_0")]
// 0xf15560 — _plcrash_async_file_close_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf15560() -> ! { todo!("0xf15560 _plcrash_async_file_close_0") }

#[doc(alias = "_plcrash_writer_pack_0")]
// 0xf15580 — _plcrash_writer_pack_0
pub fn stub_0xf15580() -> ! { todo!("0xf15580 _plcrash_writer_pack_0") }

#[doc(alias = "_uint64_pack_1")]
// 0xf15918 — _uint64_pack_1
pub fn stub_0xf15918() -> ! { todo!("0xf15918 _uint64_pack_1") }

#[doc(alias = "+[FlurryPLCrashReporter initialize]")]
// 0xf159c0 — +[FlurryPLCrashReporter initialize]
// type: void __cdecl(id, SEL)
pub fn stub_0xf159c0() -> ! { todo!("0xf159c0 +[FlurryPLCrashReporter initialize]") }

#[doc(alias = "_image_add_callback_1")]
// 0xf15a44 — _image_add_callback_1
pub fn stub_0xf15a44() -> ! { todo!("0xf15a44 _image_add_callback_1") }

#[doc(alias = "_image_remove_callback_0")]
// 0xf15a94 — _image_remove_callback_0
pub fn stub_0xf15a94() -> ! { todo!("0xf15a94 _image_remove_callback_0") }

#[doc(alias = "+[FlurryPLCrashReporter sharedReporter]")]
// 0xf15aac — +[FlurryPLCrashReporter sharedReporter]
// type: id __cdecl(id, SEL)
pub fn stub_0xf15aac() -> ! { todo!("0xf15aac +[FlurryPLCrashReporter sharedReporter]") }

#[doc(alias = "-[FlurryPLCrashReporter hasPendingCrashReport]")]
// 0xf15b14 — -[FlurryPLCrashReporter hasPendingCrashReport]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15b14() -> ! { todo!("0xf15b14 -[FlurryPLCrashReporter hasPendingCrashReport]") }

#[doc(alias = "-[FlurryPLCrashReporter loadPendingCrashReportData]")]
// 0xf15b60 — -[FlurryPLCrashReporter loadPendingCrashReportData]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15b60() -> ! { todo!("0xf15b60 -[FlurryPLCrashReporter loadPendingCrashReportData]") }

#[doc(alias = "-[FlurryPLCrashReporter loadPendingCrashReportDataAndReturnError:]")]
// 0xf15b78 — -[FlurryPLCrashReporter loadPendingCrashReportDataAndReturnError:]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf15b78() -> ! { todo!("0xf15b78 -[FlurryPLCrashReporter loadPendingCrashReportDataAndReturnError:]") }

#[doc(alias = "-[FlurryPLCrashReporter purgePendingCrashReport]")]
// 0xf15bb8 — -[FlurryPLCrashReporter purgePendingCrashReport]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15bb8() -> ! { todo!("0xf15bb8 -[FlurryPLCrashReporter purgePendingCrashReport]") }

#[doc(alias = "-[FlurryPLCrashReporter purgePendingCrashReportAndReturnError:]")]
// 0xf15bd0 — -[FlurryPLCrashReporter purgePendingCrashReportAndReturnError:]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf15bd0() -> ! { todo!("0xf15bd0 -[FlurryPLCrashReporter purgePendingCrashReportAndReturnError:]") }

#[doc(alias = "-[FlurryPLCrashReporter enableCrashReporter]")]
// 0xf15c20 — -[FlurryPLCrashReporter enableCrashReporter]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15c20() -> ! { todo!("0xf15c20 -[FlurryPLCrashReporter enableCrashReporter]") }

#[doc(alias = "-[FlurryPLCrashReporter enableCrashReporterAndReturnError:]")]
// 0xf15c38 — -[FlurryPLCrashReporter enableCrashReporterAndReturnError:]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf15c38() -> ! { todo!("0xf15c38 -[FlurryPLCrashReporter enableCrashReporterAndReturnError:]") }

#[doc(alias = "_signal_handler_callback_0")]
// 0xf15dbc — _signal_handler_callback_0
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf15dbc() -> ! { todo!("0xf15dbc _signal_handler_callback_0") }
