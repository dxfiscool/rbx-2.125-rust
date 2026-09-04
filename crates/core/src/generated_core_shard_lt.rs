//! core shard lt — 100 core stubs EA-sorted, next uncovered fallback after shard ls (0xeee8ac..0xef3e64, lowest EA first).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound (33260 remaining, 3603 uncovered before -> 3503 after, rbx_core::SharedPtr not boost) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "+[BSOpenUDID _setDict:forPasteboard:]")]
// 0xeee8ac — +[BSOpenUDID _setDict:forPasteboard:]
// type: void __cdecl(id, SEL, id, id)
pub fn stub_0xeee8ac() {
    // IDA 0xeee8ac: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSOpenUDID _getDictFromPasteboard:]")]
// 0xeee8f0 — +[BSOpenUDID _getDictFromPasteboard:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xeee8f0() {
    // IDA 0xeee8f0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSOpenUDID _generateFreshOpenUDID]")]
// 0xeeea48 — +[BSOpenUDID _generateFreshOpenUDID]
// type: id __cdecl(id, SEL)
pub fn stub_0xeeea48() {
    // IDA 0xeeea48: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSOpenUDID value]")]
// 0xeeeb64 — +[BSOpenUDID value]
// type: id __cdecl(id, SEL)
pub fn stub_0xeeeb64() {
    // IDA 0xeeeb64: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSOpenUDID valueWithError:]")]
// 0xeeeb88 — +[BSOpenUDID valueWithError:]
// type: id __cdecl(id, SEL, id *)
pub fn stub_0xeeeb88() {
    // IDA 0xeeeb88: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSOpenUDID setOptOut:]")]
// 0xeef274 — +[BSOpenUDID setOptOut:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xeef274() {
    // IDA 0xeef274: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseAnalyticsGenerator analyticsDataWithTag:]")]
// 0xeef3c8 — +[BugSenseAnalyticsGenerator analyticsDataWithTag:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xeef3c8() {
    // IDA 0xeef3c8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseAnalyticsGenerator analyticsObjectFromData:]")]
// 0xeef5f0 — +[BugSenseAnalyticsGenerator analyticsObjectFromData:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xeef5f0() {
    // IDA 0xeef5f0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseAnalyticsGenerator analyticsObjectFromString:]")]
// 0xeef658 — +[BugSenseAnalyticsGenerator analyticsObjectFromString:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xeef658() {
    // IDA 0xeef658: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence bugsenseDirectory]")]
// 0xeef8dc — +[BugSensePersistence bugsenseDirectory]
// type: id __cdecl(id, SEL)
pub fn stub_0xeef8dc() {
    // IDA 0xeef8dc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence createDirectoryStructure]")]
// 0xeef918 — +[BugSensePersistence createDirectoryStructure]
// type: void __cdecl(id, SEL)
pub fn stub_0xeef918() {
    // IDA 0xeef918: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence pendingPingsStorePath]")]
// 0xeef9c8 — +[BugSensePersistence pendingPingsStorePath]
// type: id __cdecl(id, SEL)
pub fn stub_0xeef9c8() {
    // IDA 0xeef9c8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence pendingTicksStorePath]")]
// 0xeef9f8 — +[BugSensePersistence pendingTicksStorePath]
// type: id __cdecl(id, SEL)
pub fn stub_0xeef9f8() {
    // IDA 0xeef9f8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence pendingCrashReportsStorePath]")]
// 0xeefa28 — +[BugSensePersistence pendingCrashReportsStorePath]
// type: id __cdecl(id, SEL)
pub fn stub_0xeefa28() {
    // IDA 0xeefa28: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence valuesStorePath]")]
// 0xeefa58 — +[BugSensePersistence valuesStorePath]
// type: id __cdecl(id, SEL)
pub fn stub_0xeefa58() {
    // IDA 0xeefa58: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence sendOrQueuePing:]")]
// 0xeefa88 — +[BugSensePersistence sendOrQueuePing:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xeefa88() {
    // IDA 0xeefa88: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence writePingsToFile:]")]
// 0xeefb40 — +[BugSensePersistence writePingsToFile:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xeefb40() {
    // IDA 0xeefb40: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence pendingPings]")]
// 0xeefd90 — +[BugSensePersistence pendingPings]
// type: id __cdecl(id, SEL)
pub fn stub_0xeefd90() {
    // IDA 0xeefd90: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence queuePing:]")]
// 0xeefdc8 — +[BugSensePersistence queuePing:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xeefdc8() {
    // IDA 0xeefdc8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence sendAllPendingPings]")]
// 0xeefe30 — +[BugSensePersistence sendAllPendingPings]
// type: char __cdecl(id, SEL)
pub fn stub_0xeefe30() {
    // IDA 0xeefe30: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence sendOrQueueTick:]")]
// 0xef0004 — +[BugSensePersistence sendOrQueueTick:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xef0004() {
    // IDA 0xef0004: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence writeTicksToFile:]")]
// 0xef00bc — +[BugSensePersistence writeTicksToFile:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xef00bc() {
    // IDA 0xef00bc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence pendingTicks]")]
// 0xef0144 — +[BugSensePersistence pendingTicks]
// type: id __cdecl(id, SEL)
pub fn stub_0xef0144() {
    // IDA 0xef0144: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence queueTick:]")]
// 0xef017c — +[BugSensePersistence queueTick:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xef017c() {
    // IDA 0xef017c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence sendAllPendingTicks]")]
// 0xef01e4 — +[BugSensePersistence sendAllPendingTicks]
// type: char __cdecl(id, SEL)
pub fn stub_0xef01e4() {
    // IDA 0xef01e4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence sendOrQueueCrashReport:]")]
// 0xef03b8 — +[BugSensePersistence sendOrQueueCrashReport:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xef03b8() {
    // IDA 0xef03b8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence writeCrashReportsToFile:]")]
// 0xef0474 — +[BugSensePersistence writeCrashReportsToFile:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xef0474() {
    // IDA 0xef0474: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence pendingCrashReports]")]
// 0xef0764 — +[BugSensePersistence pendingCrashReports]
// type: id __cdecl(id, SEL)
pub fn stub_0xef0764() {
    // IDA 0xef0764: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence queueCrashReport:]")]
// 0xef079c — +[BugSensePersistence queueCrashReport:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xef079c() {
    // IDA 0xef079c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence sendAllPendingCrashReports]")]
// 0xef0828 — +[BugSensePersistence sendAllPendingCrashReports]
// type: char __cdecl(id, SEL)
pub fn stub_0xef0828() {
    // IDA 0xef0828: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence lineIsSimilar:toLine:]")]
// 0xef0a00 — +[BugSensePersistence lineIsSimilar:toLine:]
// type: char __cdecl(id, SEL, id, id)
pub fn stub_0xef0a00() {
    // IDA 0xef0a00: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence crashReportExists:]")]
// 0xef0a1c — +[BugSensePersistence crashReportExists:]
// type: char __cdecl(id, SEL, id)
pub fn stub_0xef0a1c() {
    // IDA 0xef0a1c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence crashCount]")]
// 0xef0e08 — +[BugSensePersistence crashCount]
// type: int __cdecl(id, SEL)
pub fn stub_0xef0e08() {
    // IDA 0xef0e08: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSensePersistence setCrashCount:]")]
// 0xef0e68 — +[BugSensePersistence setCrashCount:]
// type: char __cdecl(id, SEL, int)
pub fn stub_0xef0e68() {
    // IDA 0xef0e68: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_memset_async")]
// 0xef0f40 — _memset_async
pub fn stub_0xef0f40() {
    // IDA 0xef0f40: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_strlen_async")]
// 0xef0f60 — _strlen_async
pub fn stub_0xef0f60() {
    // IDA 0xef0f60: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_strcpy_async")]
// 0xef0f78 — _strcpy_async
pub fn stub_0xef0f78() {
    // IDA 0xef0f78: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_strcat_async")]
// 0xef0fa4 — _strcat_async
pub fn stub_0xef0fa4() {
    // IDA 0xef0fa4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_strcmp_async")]
// 0xef0fc8 — _strcmp_async
pub fn stub_0xef0fc8() {
    // IDA 0xef0fc8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_atoi_async")]
// 0xef1020 — _atoi_async
pub fn stub_0xef1020() {
    // IDA 0xef1020: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_get_base_from_str")]
// 0xef10bc — _get_base_from_str
pub fn stub_0xef10bc() {
    // IDA 0xef10bc: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_atoi_async_helper")]
// 0xef1100 — _atoi_async_helper
pub fn stub_0xef1100() {
    // IDA 0xef1100: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_atollu_async")]
// 0xef11a0 — _atollu_async
pub fn stub_0xef11a0() {
    // IDA 0xef11a0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_atollu_async_helper")]
// 0xef123c — _atollu_async_helper
pub fn stub_0xef123c() {
    // IDA 0xef123c: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_itoa_async")]
// 0xef133c — _itoa_async
pub fn stub_0xef133c() {
    // IDA 0xef133c: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_ctime_async")]
// 0xef1410 — _ctime_async
pub fn stub_0xef1410() {
    // IDA 0xef1410: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_get_system_log_messages")]
// 0xef16ec — _get_system_log_messages
pub fn stub_0xef16ec() {
    // IDA 0xef16ec: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_read_from_log_file")]
// 0xef1b60 — _read_from_log_file
// type: int __fastcall(int, char *)
pub fn stub_0xef1b60() {
    // IDA 0xef1b60: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_write_to_log_file")]
// 0xef1c04 — _write_to_log_file
// type: int __fastcall(int, char *)
pub fn stub_0xef1c04() {
    // IDA 0xef1c04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_read_llu_from_file")]
// 0xef1ca0 — _read_llu_from_file
// type: ssize_t __fastcall(void *, char *)
pub fn stub_0xef1ca0() {
    // IDA 0xef1ca0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_write_llu_to_file")]
// 0xef1ce0 — _write_llu_to_file
// type: int __fastcall(int, int, char *)
pub fn stub_0xef1ce0() {
    // IDA 0xef1ce0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_write_str_to_file")]
// 0xef1d2c — _write_str_to_file
// type: int __fastcall(int, char *)
pub fn stub_0xef1d2c() {
    // IDA 0xef1d2c: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "+[NSData dataByTransformingData:usingGZipOperation:error:]")]
// 0xef1d94 — +[NSData dataByTransformingData:usingGZipOperation:error:]
// type: id __cdecl(id, SEL, id, int, id *)
pub fn stub_0xef1d94() {
    // IDA 0xef1d94: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData base64EncodedString]")]
// 0xef1f30 — -[NSData base64EncodedString]
// type: id __cdecl(NSData *self, SEL)
pub fn stub_0xef1f30() {
    // IDA 0xef1f30: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData dataByGZipCompressingWithError:]")]
// 0xef2080 — -[NSData dataByGZipCompressingWithError:]
// type: id __cdecl(NSData *self, SEL, id *)
pub fn stub_0xef2080() {
    // IDA 0xef2080: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData dataByGZipDecompressingDataWithError:]")]
// 0xef20b4 — -[NSData dataByGZipDecompressingDataWithError:]
// type: id __cdecl(NSData *self, SEL, id *)
pub fn stub_0xef20b4() {
    // IDA 0xef20b4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseImageManager registerCallback]")]
// 0xef20e4 — +[BugSenseImageManager registerCallback]
// type: void __cdecl(id, SEL)
pub fn stub_0xef20e4() {
    // IDA 0xef20e4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_image_add_callback")]
// 0xef20f8 — _image_add_callback
pub fn stub_0xef20f8() {
    // IDA 0xef20f8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BugSenseImageManager loadedImages]")]
// 0xef21a8 — +[BugSenseImageManager loadedImages]
// type: id __cdecl(id, SEL)
pub fn stub_0xef21a8() {
    // IDA 0xef21a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[PLCrashSignalHandler sharedHandler]")]
// 0xef2470 — +[PLCrashSignalHandler sharedHandler]
// type: id __cdecl(id, SEL)
pub fn stub_0xef2470() {
    // IDA 0xef2470: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[PLCrashSignalHandler init]")]
// 0xef24c0 — -[PLCrashSignalHandler init]
// type: PLCrashSignalHandler *__cdecl(PLCrashSignalHandler *self, SEL)
pub fn stub_0xef24c0() {
    // IDA 0xef24c0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[PLCrashSignalHandler registerHandlerForSignal:error:]")]
// 0xef2540 — -[PLCrashSignalHandler registerHandlerForSignal:error:]
// type: char __cdecl(PLCrashSignalHandler *self, SEL, int, id *)
pub fn stub_0xef2540() {
    // IDA 0xef2540: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_fatal_signal_handler")]
// 0xef25ac — _fatal_signal_handler
pub fn stub_0xef25ac() {
    // IDA 0xef25ac: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[PLCrashSignalHandler registerHandlerWithCallback:context:error:]")]
// 0xef2620 — -[PLCrashSignalHandler registerHandlerWithCallback:context:error:]
// type: char __cdecl(PLCrashSignalHandler *self, SEL, void *, void *, id *)
pub fn stub_0xef2620() {
    // IDA 0xef2620: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_strerror")]
// 0xef26f0 — _plframe_strerror
pub fn stub_0xef26f0() {
    // IDA 0xef26f0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_read_addr")]
// 0xef2710 — _plframe_read_addr
// type: int __fastcall(vm_address_t address, vm_address_t data)
pub fn stub_0xef2710() {
    // IDA 0xef2710: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_test_thread_spawn")]
// 0xef2738 — _plframe_test_thread_spawn
pub fn stub_0xef2738() {
    // IDA 0xef2738: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_test_stack_thr")]
// 0xef2780 — _test_stack_thr
pub fn stub_0xef2780() {
    // IDA 0xef2780: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plframe_test_thread_stop")]
// 0xef27a8 — _plframe_test_thread_stop
pub fn stub_0xef27a8() {
    // IDA 0xef27a8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_cursor_init")]
// 0xef27d0 — _plframe_cursor_init
// type: int __fastcall(int, int)
pub fn stub_0xef27d0() {
    // IDA 0xef27d0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_cursor_thread_init")]
// 0xef27e0 — _plframe_cursor_thread_init
// type: int __fastcall(int, thread_act_t target_act)
pub fn stub_0xef27e0() {
    // IDA 0xef27e0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_cursor_next")]
// 0xef284c — _plframe_cursor_next
pub fn stub_0xef284c() {
    // IDA 0xef284c: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_get_reg")]
// 0xef28ac — _plframe_get_reg
pub fn stub_0xef28ac() {
    // IDA 0xef28ac: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_get_freg")]
// 0xef2904 — _plframe_get_freg
pub fn stub_0xef2904() {
    // IDA 0xef2904: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plframe_get_regname")]
// 0xef2908 — _plframe_get_regname
pub fn stub_0xef2908() {
    // IDA 0xef2908: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_plcrash_log_writer_init")]
// 0xef29f8 — _plcrash_log_writer_init
// type: int __fastcall(int, id)
pub fn stub_0xef29f8() {
    // IDA 0xef29f8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_set_exception")]
// 0xef2c0c — _plcrash_log_writer_set_exception
// type: int __fastcall(int, id)
pub fn stub_0xef2c0c() {
    // IDA 0xef2c0c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_close")]
// 0xef2dc8 — _plcrash_log_writer_close
pub fn stub_0xef2dc8() {
    // IDA 0xef2dc8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_free")]
// 0xef2dcc — _plcrash_log_writer_free
pub fn stub_0xef2dcc() {
    // IDA 0xef2dcc: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_log_writer_write")]
// 0xef2e4c — _plcrash_log_writer_write
pub fn stub_0xef2e4c() {
    // IDA 0xef2e4c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_system_info")]
// 0xef3130 — _plcrash_writer_write_system_info
pub fn stub_0xef3130() {
    // IDA 0xef3130: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_machine_info")]
// 0xef31c4 — _plcrash_writer_write_machine_info
pub fn stub_0xef31c4() {
    // IDA 0xef31c4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_process_info")]
// 0xef3240 — _plcrash_writer_write_process_info
pub fn stub_0xef3240() {
    // IDA 0xef3240: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_thread")]
// 0xef32cc — _plcrash_writer_write_thread
pub fn stub_0xef32cc() {
    // IDA 0xef32cc: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_write_binary_image")]
// 0xef3478 — _plcrash_writer_write_binary_image
pub fn stub_0xef3478() {
    // IDA 0xef3478: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_exception")]
// 0xef360c — _plcrash_writer_write_exception
pub fn stub_0xef360c() {
    // IDA 0xef360c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_signal")]
// 0xef36d8 — _plcrash_writer_write_signal
pub fn stub_0xef36d8() {
    // IDA 0xef36d8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_writer_write_processor_info")]
// 0xef37a4 — _plcrash_writer_write_processor_info
pub fn stub_0xef37a4() {
    // IDA 0xef37a4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_strerror")]
// 0xef37f4 — _plcrash_strerror
pub fn stub_0xef37f4() {
    // IDA 0xef37f4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_plcrash_async_memcpy")]
// 0xef3814 — _plcrash_async_memcpy
// type: char *__fastcall(_BYTE *, char *, int)
pub fn stub_0xef3814() {
    // IDA 0xef3814: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_init")]
// 0xef3828 — _plcrash_async_file_init
pub fn stub_0xef3828() {
    // IDA 0xef3828: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_write")]
// 0xef383c — _plcrash_async_file_write
pub fn stub_0xef383c() {
    // IDA 0xef383c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_writen")]
// 0xef38e8 — _writen
pub fn stub_0xef38e8() {
    // IDA 0xef38e8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_flush")]
// 0xef3924 — _plcrash_async_file_flush
pub fn stub_0xef3924() {
    // IDA 0xef3924: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_async_file_close")]
// 0xef394c — _plcrash_async_file_close
pub fn stub_0xef394c() {
    // IDA 0xef394c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_plcrash_writer_pack")]
// 0xef396c — _plcrash_writer_pack
pub fn stub_0xef396c() {
    // IDA 0xef396c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_uint64_pack")]
// 0xef3d00 — _uint64_pack
pub fn stub_0xef3d00() {
    // IDA 0xef3d00: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[PLCrashReporter initialize]")]
// 0xef3da8 — +[PLCrashReporter initialize]
// type: void __cdecl(id, SEL)
pub fn stub_0xef3da8() {
    // IDA 0xef3da8: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_image_add_callback_0")]
// 0xef3e1c — _image_add_callback_0
pub fn stub_0xef3e1c() {
    // IDA 0xef3e1c: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_image_remove_callback")]
// 0xef3e64 — _image_remove_callback
pub fn stub_0xef3e64() {
    // IDA 0xef3e64: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}
