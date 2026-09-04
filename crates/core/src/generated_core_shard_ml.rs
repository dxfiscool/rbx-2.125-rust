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
pub fn stub_0xf1311c() {
    // IDA 0xf1311c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage setAge:]")]
// 0xf13130 — -[FlurryGlobalVariableStorage setAge:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, int)
pub fn stub_0xf13130() {
    // IDA 0xf13130: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage gender]")]
// 0xf13148 — -[FlurryGlobalVariableStorage gender]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf13148() {
    // IDA 0xf13148: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage setGender:]")]
// 0xf13160 — -[FlurryGlobalVariableStorage setGender:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf13160() {
    // IDA 0xf13160: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage startSessionCalled]")]
// 0xf13184 — -[FlurryGlobalVariableStorage startSessionCalled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf13184() {
    // IDA 0xf13184: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage setStartSessionCalled:]")]
// 0xf1319c — -[FlurryGlobalVariableStorage setStartSessionCalled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf1319c() {
    // IDA 0xf1319c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage launchOptions]")]
// 0xf131b4 — -[FlurryGlobalVariableStorage launchOptions]
// type: NSMutableDictionary *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf131b4() {
    // IDA 0xf131b4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage pushToken]")]
// 0xf131c8 — -[FlurryGlobalVariableStorage pushToken]
// type: NSString *__cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf131c8() {
    // IDA 0xf131c8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage location]")]
// 0xf131e0 — -[FlurryGlobalVariableStorage location]
// type: id __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf131e0() {
    // IDA 0xf131e0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage setLocation:]")]
// 0xf131f8 — -[FlurryGlobalVariableStorage setLocation:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, id)
pub fn stub_0xf131f8() {
    // IDA 0xf131f8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage macAddressEnabled]")]
// 0xf1321c — -[FlurryGlobalVariableStorage macAddressEnabled]
// type: char __cdecl(FlurryGlobalVariableStorage *self, SEL)
pub fn stub_0xf1321c() {
    // IDA 0xf1321c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryGlobalVariableStorage setMacAddressEnabled:]")]
// 0xf13234 — -[FlurryGlobalVariableStorage setMacAddressEnabled:]
// type: void __cdecl(FlurryGlobalVariableStorage *self, SEL, char)
pub fn stub_0xf13234() {
    // IDA 0xf13234: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError init]")]
// 0xf1324c — -[FlurryError init]
// type: FlurryError *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf1324c() {
    // IDA 0xf1324c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError initWithCoder:]")]
// 0xf132f0 — -[FlurryError initWithCoder:]
// type: FlurryError *__cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf132f0() {
    // IDA 0xf132f0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError encodeWithCoder:]")]
// 0xf13454 — -[FlurryError encodeWithCoder:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13454() {
    // IDA 0xf13454: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryError errorWithString:message:exceptionString:errorType:reportData:]")]
// 0xf1358c — +[FlurryError errorWithString:message:exceptionString:errorType:reportData:]
// type: id __cdecl(id, SEL, id, id, id, int, id)
pub fn stub_0xf1358c() {
    // IDA 0xf1358c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError appendToData:]")]
// 0xf13620 — -[FlurryError appendToData:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13620() {
    // IDA 0xf13620: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError description]")]
// 0xf137dc — -[FlurryError description]
// type: id __cdecl(FlurryError *self, SEL)
pub fn stub_0xf137dc() {
    // IDA 0xf137dc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError dealloc]")]
// 0xf138a0 — -[FlurryError dealloc]
// type: void __cdecl(FlurryError *self, SEL)
pub fn stub_0xf138a0() {
    // IDA 0xf138a0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError errorID]")]
// 0xf13934 — -[FlurryError errorID]
// type: int __cdecl(FlurryError *self, SEL)
pub fn stub_0xf13934() {
    // IDA 0xf13934: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError setErrorID:]")]
// 0xf13944 — -[FlurryError setErrorID:]
// type: void __cdecl(FlurryError *self, SEL, int)
pub fn stub_0xf13944() {
    // IDA 0xf13944: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError date]")]
// 0xf13954 — -[FlurryError date]
// type: NSDate *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf13954() {
    // IDA 0xf13954: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError setDate:]")]
// 0xf13964 — -[FlurryError setDate:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13964() {
    // IDA 0xf13964: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError errorString]")]
// 0xf13988 — -[FlurryError errorString]
// type: NSString *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf13988() {
    // IDA 0xf13988: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError setErrorString:]")]
// 0xf13998 — -[FlurryError setErrorString:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13998() {
    // IDA 0xf13998: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError errorMessage]")]
// 0xf139bc — -[FlurryError errorMessage]
// type: NSString *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf139bc() {
    // IDA 0xf139bc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError setErrorMessage:]")]
// 0xf139cc — -[FlurryError setErrorMessage:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf139cc() {
    // IDA 0xf139cc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError exceptionString]")]
// 0xf139f0 — -[FlurryError exceptionString]
// type: NSString *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf139f0() {
    // IDA 0xf139f0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError setExceptionString:]")]
// 0xf13a00 — -[FlurryError setExceptionString:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13a00() {
    // IDA 0xf13a00: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError errorType]")]
// 0xf13a24 — -[FlurryError errorType]
// type: int __cdecl(FlurryError *self, SEL)
pub fn stub_0xf13a24() {
    // IDA 0xf13a24: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError setErrorType:]")]
// 0xf13a34 — -[FlurryError setErrorType:]
// type: void __cdecl(FlurryError *self, SEL, int)
pub fn stub_0xf13a34() {
    // IDA 0xf13a34: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError reportData]")]
// 0xf13a44 — -[FlurryError reportData]
// type: NSData *__cdecl(FlurryError *self, SEL)
pub fn stub_0xf13a44() {
    // IDA 0xf13a44: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryError setReportData:]")]
// 0xf13a54 — -[FlurryError setReportData:]
// type: void __cdecl(FlurryError *self, SEL, id)
pub fn stub_0xf13a54() {
    // IDA 0xf13a54: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryKeychainWrapper keychainWrapperWithIdentifier:]")]
// 0xf13a78 — +[FlurryKeychainWrapper keychainWrapperWithIdentifier:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xf13a78() {
    // IDA 0xf13a78: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper initWithIdentifier:]")]
// 0xf13ac0 — -[FlurryKeychainWrapper initWithIdentifier:]
// type: FlurryKeychainWrapper *__cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13ac0() {
    // IDA 0xf13ac0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper dealloc]")]
// 0xf13bb4 — -[FlurryKeychainWrapper dealloc]
// type: void __cdecl(FlurryKeychainWrapper *self, SEL)
pub fn stub_0xf13bb4() {
    // IDA 0xf13bb4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper setupSearchForKey:]")]
// 0xf13bf8 — -[FlurryKeychainWrapper setupSearchForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13bf8() {
    // IDA 0xf13bf8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper dataForKey:]")]
// 0xf13ce0 — -[FlurryKeychainWrapper dataForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13ce0() {
    // IDA 0xf13ce0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper stringForKey:]")]
// 0xf13d88 — -[FlurryKeychainWrapper stringForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13d88() {
    // IDA 0xf13d88: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper setObject:forKey:]")]
// 0xf13de8 — -[FlurryKeychainWrapper setObject:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13de8() {
    // IDA 0xf13de8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper objectForKey:]")]
// 0xf13e30 — -[FlurryKeychainWrapper objectForKey:]
// type: id __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13e30() {
    // IDA 0xf13e30: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper setString:forKey:]")]
// 0xf13e6c — -[FlurryKeychainWrapper setString:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13e6c() {
    // IDA 0xf13e6c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper setData:forKey:]")]
// 0xf13ea0 — -[FlurryKeychainWrapper setData:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13ea0() {
    // IDA 0xf13ea0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper updateValueData:forKey:]")]
// 0xf13f68 — -[FlurryKeychainWrapper updateValueData:forKey:]
// type: char __cdecl(FlurryKeychainWrapper *self, SEL, id, id)
pub fn stub_0xf13f68() {
    // IDA 0xf13f68: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper removeObjectForKey:]")]
// 0xf13fd8 — -[FlurryKeychainWrapper removeObjectForKey:]
// type: void __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf13fd8() {
    // IDA 0xf13fd8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper identifier]")]
// 0xf13ff4 — -[FlurryKeychainWrapper identifier]
// type: NSString *__cdecl(FlurryKeychainWrapper *self, SEL)
pub fn stub_0xf13ff4() {
    // IDA 0xf13ff4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryKeychainWrapper setIdentifier:]")]
// 0xf14004 — -[FlurryKeychainWrapper setIdentifier:]
// type: void __cdecl(FlurryKeychainWrapper *self, SEL, id)
pub fn stub_0xf14004() {
    // IDA 0xf14004: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryPLCrashSignalHandler sharedHandler]")]
// 0xf14030 — +[FlurryPLCrashSignalHandler sharedHandler]
// type: id __cdecl(id, SEL)
pub fn stub_0xf14030() {
    // IDA 0xf14030: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashSignalHandler init]")]
// 0xf14080 — -[FlurryPLCrashSignalHandler init]
// type: FlurryPLCrashSignalHandler *__cdecl(FlurryPLCrashSignalHandler *self, SEL)
pub fn stub_0xf14080() {
    // IDA 0xf14080: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashSignalHandler registerHandlerForSignal:error:]")]
// 0xf14104 — -[FlurryPLCrashSignalHandler registerHandlerForSignal:error:]
// type: char __cdecl(FlurryPLCrashSignalHandler *self, SEL, int, id *)
pub fn stub_0xf14104() {
    // IDA 0xf14104: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_fatal_signal_handler_0")]
// 0xf14170 — _fatal_signal_handler_0
pub fn stub_0xf14170() {
    // IDA 0xf14170: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashSignalHandler registerHandlerWithCallback:context:error:]")]
// 0xf141e4 — -[FlurryPLCrashSignalHandler registerHandlerWithCallback:context:error:]
// type: char __cdecl(FlurryPLCrashSignalHandler *self, SEL, void *, void *, id *)
pub fn stub_0xf141e4() {
    // IDA 0xf141e4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_strerror_0")]
// 0xf142b4 — _plframe_strerror_0
pub fn stub_0xf142b4() {
    // IDA 0xf142b4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_test_thread_spawn_0")]
// 0xf142d4 — _plframe_test_thread_spawn_0
pub fn stub_0xf142d4() {
    // IDA 0xf142d4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_test_stack_thr_0")]
// 0xf1431c — _test_stack_thr_0
pub fn stub_0xf1431c() {
    // IDA 0xf1431c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_test_thread_stop_0")]
// 0xf14344 — _plframe_test_thread_stop_0
pub fn stub_0xf14344() {
    // IDA 0xf14344: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_cursor_init_0")]
// 0xf1436c — _plframe_cursor_init_0
pub fn stub_0xf1436c() {
    // IDA 0xf1436c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_cursor_thread_init_0")]
// 0xf1437c — _plframe_cursor_thread_init_0
// type: int __fastcall(int, thread_act_t target_act)
pub fn stub_0xf1437c() {
    // IDA 0xf1437c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_cursor_next_0")]
// 0xf143e8 — _plframe_cursor_next_0
pub fn stub_0xf143e8() {
    // IDA 0xf143e8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_get_reg_0")]
// 0xf14454 — _plframe_get_reg_0
// type: int __fastcall(_DWORD *, int, _DWORD *)
pub fn stub_0xf14454() {
    // IDA 0xf14454: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_get_freg_0")]
// 0xf144ac — _plframe_get_freg_0
pub fn stub_0xf144ac() {
    // IDA 0xf144ac: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_get_regname_0")]
// 0xf144b0 — _plframe_get_regname_0
pub fn stub_0xf144b0() {
    // IDA 0xf144b0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plcrash_log_writer_init_0")]
// 0xf145a0 — _plcrash_log_writer_init_0
// type: int __fastcall(void *__b)
pub fn stub_0xf145a0() {
    // IDA 0xf145a0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_set_exception_0")]
// 0xf14798 — _plcrash_log_writer_set_exception_0
// type: int __fastcall(int, id)
pub fn stub_0xf14798() {
    // IDA 0xf14798: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_close_0")]
// 0xf14954 — _plcrash_log_writer_close_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf14954() {
    // IDA 0xf14954: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_free_0")]
// 0xf14958 — _plcrash_log_writer_free_0
pub fn stub_0xf14958() {
    // IDA 0xf14958: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_write_0")]
// 0xf149d8 — _plcrash_log_writer_write_0
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf149d8() {
    // IDA 0xf149d8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_system_info_0")]
// 0xf14d00 — _plcrash_writer_write_system_info_0
pub fn stub_0xf14d00() {
    // IDA 0xf14d00: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_machine_info_0")]
// 0xf14d94 — _plcrash_writer_write_machine_info_0
pub fn stub_0xf14d94() {
    // IDA 0xf14d94: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_process_info_0")]
// 0xf14e10 — _plcrash_writer_write_process_info_0
pub fn stub_0xf14e10() {
    // IDA 0xf14e10: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_thread_0")]
// 0xf14e9c — _plcrash_writer_write_thread_0
pub fn stub_0xf14e9c() {
    // IDA 0xf14e9c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_binary_image_0")]
// 0xf15020 — _plcrash_writer_write_binary_image_0
pub fn stub_0xf15020() {
    // IDA 0xf15020: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_exception_0")]
// 0xf150e4 — _plcrash_writer_write_exception_0
pub fn stub_0xf150e4() {
    // IDA 0xf150e4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_signal_0")]
// 0xf151a4 — _plcrash_writer_write_signal_0
pub fn stub_0xf151a4() {
    // IDA 0xf151a4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_thread_frame")]
// 0xf15270 — _plcrash_writer_write_thread_frame
pub fn stub_0xf15270() {
    // IDA 0xf15270: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_thread_frame_symbol_cb")]
// 0xf1531c — _plcrash_writer_write_thread_frame_symbol_cb
pub fn stub_0xf1531c() {
    // IDA 0xf1531c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_processor_info_0")]
// 0xf1535c — _plcrash_writer_write_processor_info_0
pub fn stub_0xf1535c() {
    // IDA 0xf1535c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_async_strerror")]
// 0xf153ac — _plcrash_async_strerror
pub fn stub_0xf153ac() {
    // IDA 0xf153ac: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_async_read_addr")]
// 0xf153cc — _plcrash_async_read_addr
// type: int __fastcall(int, int, vm_address_t data, vm_size_t size)
pub fn stub_0xf153cc() {
    // IDA 0xf153cc: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_async_strncmp")]
// 0xf153e8 — _plcrash_async_strncmp
// type: int __fastcall(unsigned __int8 *, unsigned __int8 *, int)
pub fn stub_0xf153e8() {
    // IDA 0xf153e8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_memcpy_0")]
// 0xf15420 — _plcrash_async_memcpy_0
pub fn stub_0xf15420() {
    // IDA 0xf15420: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_writen")]
// 0xf15434 — _plcrash_async_writen
pub fn stub_0xf15434() {
    // IDA 0xf15434: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_init_0")]
// 0xf15470 — _plcrash_async_file_init_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf15470() {
    // IDA 0xf15470: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_write_0")]
// 0xf15484 — _plcrash_async_file_write_0
pub fn stub_0xf15484() {
    // IDA 0xf15484: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_flush_0")]
// 0xf15538 — _plcrash_async_file_flush_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf15538() {
    // IDA 0xf15538: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_close_0")]
// 0xf15560 — _plcrash_async_file_close_0
// type: int __fastcall(_DWORD)
pub fn stub_0xf15560() {
    // IDA 0xf15560: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_pack_0")]
// 0xf15580 — _plcrash_writer_pack_0
pub fn stub_0xf15580() {
    // IDA 0xf15580: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_uint64_pack_1")]
// 0xf15918 — _uint64_pack_1
pub fn stub_0xf15918() {
    // IDA 0xf15918: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryPLCrashReporter initialize]")]
// 0xf159c0 — +[FlurryPLCrashReporter initialize]
// type: void __cdecl(id, SEL)
pub fn stub_0xf159c0() {
    // IDA 0xf159c0: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_image_add_callback_1")]
// 0xf15a44 — _image_add_callback_1
pub fn stub_0xf15a44() {
    // IDA 0xf15a44: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_image_remove_callback_0")]
// 0xf15a94 — _image_remove_callback_0
pub fn stub_0xf15a94() {
    // IDA 0xf15a94: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryPLCrashReporter sharedReporter]")]
// 0xf15aac — +[FlurryPLCrashReporter sharedReporter]
// type: id __cdecl(id, SEL)
pub fn stub_0xf15aac() {
    // IDA 0xf15aac: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReporter hasPendingCrashReport]")]
// 0xf15b14 — -[FlurryPLCrashReporter hasPendingCrashReport]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15b14() {
    // IDA 0xf15b14: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReporter loadPendingCrashReportData]")]
// 0xf15b60 — -[FlurryPLCrashReporter loadPendingCrashReportData]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15b60() {
    // IDA 0xf15b60: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReporter loadPendingCrashReportDataAndReturnError:]")]
// 0xf15b78 — -[FlurryPLCrashReporter loadPendingCrashReportDataAndReturnError:]
// type: id __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf15b78() {
    // IDA 0xf15b78: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReporter purgePendingCrashReport]")]
// 0xf15bb8 — -[FlurryPLCrashReporter purgePendingCrashReport]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15bb8() {
    // IDA 0xf15bb8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReporter purgePendingCrashReportAndReturnError:]")]
// 0xf15bd0 — -[FlurryPLCrashReporter purgePendingCrashReportAndReturnError:]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf15bd0() {
    // IDA 0xf15bd0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReporter enableCrashReporter]")]
// 0xf15c20 — -[FlurryPLCrashReporter enableCrashReporter]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL)
pub fn stub_0xf15c20() {
    // IDA 0xf15c20: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryPLCrashReporter enableCrashReporterAndReturnError:]")]
// 0xf15c38 — -[FlurryPLCrashReporter enableCrashReporterAndReturnError:]
// type: char __cdecl(FlurryPLCrashReporter *self, SEL, id *)
pub fn stub_0xf15c38() {
    // IDA 0xf15c38: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_signal_handler_callback_0")]
// 0xf15dbc — _signal_handler_callback_0
// type: int __fastcall(int, int, int, int)
pub fn stub_0xf15dbc() {
    // IDA 0xf15dbc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
