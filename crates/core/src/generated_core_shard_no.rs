//! core shard no — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 32196, 719 uncovered before -> 619 after, batch 0xf6c484..0xf6cab4).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_fseeko")]
// 0xf6c484 — _fseeko
// type: int __cdecl(FILE *__stream, off_t __offset, int __whence)
pub fn stub_0xf6c484() {
    // IDA 0xf6c484: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_fstat")]
// 0xf6c494 — _fstat
// type: int __cdecl(int, stat *)
pub fn stub_0xf6c494() {
    // IDA 0xf6c494: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_ftell")]
// 0xf6c4a4 — _ftell
// type: __int32 __cdecl(FILE *)
pub fn stub_0xf6c4a4() {
    // IDA 0xf6c4a4: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_fwrite")]
// 0xf6c4b4 — _fwrite
// type: size_t __cdecl(const void *__ptr, size_t __size, size_t __nitems, FILE *__stream)
pub fn stub_0xf6c4b4() {
    // IDA 0xf6c4b4: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_getaddrinfo")]
// 0xf6c4c4 — _getaddrinfo
// type: int __cdecl(const char *, const char *, const addrinfo *, addrinfo **)
pub fn stub_0xf6c4c4() {
    // IDA 0xf6c4c4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getc")]
// 0xf6c4d4 — _getc
// type: int __cdecl(FILE *)
pub fn stub_0xf6c4d4() {
    // IDA 0xf6c4d4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getchar")]
// 0xf6c4e4 — _getchar
// type: int(void)
pub fn stub_0xf6c4e4() {
    // IDA 0xf6c4e4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getcwd")]
// 0xf6c4f4 — _getcwd
// type: char *__cdecl(char *, size_t)
pub fn stub_0xf6c4f4() {
    // IDA 0xf6c4f4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getenv")]
// 0xf6c504 — _getenv
// type: char *__cdecl(const char *)
pub fn stub_0xf6c504() {
    // IDA 0xf6c504: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getgid")]
// 0xf6c514 — _getgid
// type: gid_t(void)
pub fn stub_0xf6c514() {
    // IDA 0xf6c514: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_gethostbyname")]
// 0xf6c524 — _gethostbyname
// type: hostent *__cdecl(const char *)
pub fn stub_0xf6c524() {
    // IDA 0xf6c524: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getifaddrs")]
// 0xf6c534 — _getifaddrs
// type: int __cdecl(ifaddrs **)
pub fn stub_0xf6c534() {
    // IDA 0xf6c534: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getnameinfo")]
// 0xf6c544 — _getnameinfo
// type: int __cdecl(const sockaddr *, socklen_t, char *, socklen_t, char *, socklen_t, int)
pub fn stub_0xf6c544() {
    // IDA 0xf6c544: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getpagesize")]
// 0xf6c554 — _getpagesize
// type: int(void)
pub fn stub_0xf6c554() {
    // IDA 0xf6c554: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getpid")]
// 0xf6c564 — _getpid
// type: pid_t(void)
pub fn stub_0xf6c564() {
    // IDA 0xf6c564: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getppid")]
// 0xf6c574 — _getppid
// type: pid_t(void)
pub fn stub_0xf6c574() {
    // IDA 0xf6c574: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getprogname")]
// 0xf6c584 — _getprogname
// type: const char *(void)
pub fn stub_0xf6c584() {
    // IDA 0xf6c584: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getpwuid")]
// 0xf6c594 — _getpwuid
// type: passwd *__cdecl(uid_t)
pub fn stub_0xf6c594() {
    // IDA 0xf6c594: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getrusage")]
// 0xf6c5a4 — _getrusage
// type: int __cdecl(int, rusage *)
pub fn stub_0xf6c5a4() {
    // IDA 0xf6c5a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getsockname")]
// 0xf6c5b4 — _getsockname
// type: int __cdecl(int, sockaddr *, socklen_t *)
pub fn stub_0xf6c5b4() {
    // IDA 0xf6c5b4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getsockopt")]
// 0xf6c5c4 — _getsockopt
// type: int __cdecl(int, int, int, void *, socklen_t *)
pub fn stub_0xf6c5c4() {
    // IDA 0xf6c5c4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_gettimeofday")]
// 0xf6c5d4 — _gettimeofday
// type: int __cdecl(timeval *, void *)
pub fn stub_0xf6c5d4() {
    // IDA 0xf6c5d4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_getuid")]
// 0xf6c5e4 — _getuid
// type: uid_t(void)
pub fn stub_0xf6c5e4() {
    // IDA 0xf6c5e4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_gmtime_r")]
// 0xf6c5f4 — _gmtime_r
// type: tm *__cdecl(const time_t *, tm *)
pub fn stub_0xf6c5f4() {
    // IDA 0xf6c5f4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_host_page_size")]
// 0xf6c604 — _host_page_size
// type: kern_return_t __cdecl(host_t, vm_size_t *)
pub fn stub_0xf6c604() {
    // IDA 0xf6c604: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_host_statistics")]
// 0xf6c614 — _host_statistics
// type: kern_return_t __cdecl(host_t host_priv, host_flavor_t flavor, host_info_t host_info_out, mach_msg_type_number_t *host_info_outCnt)
pub fn stub_0xf6c614() {
    // IDA 0xf6c614: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_inet_addr")]
// 0xf6c624 — _inet_addr
// type: in_addr_t __cdecl(const char *)
pub fn stub_0xf6c624() {
    // IDA 0xf6c624: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_inet_aton")]
// 0xf6c634 — _inet_aton
// type: int __cdecl(const char *, in_addr *)
pub fn stub_0xf6c634() {
    // IDA 0xf6c634: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_inet_ntoa")]
// 0xf6c644 — _inet_ntoa
// type: char *__cdecl(in_addr)
pub fn stub_0xf6c644() {
    // IDA 0xf6c644: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_kill")]
// 0xf6c654 — _kill
// type: int __cdecl(pid_t, int)
pub fn stub_0xf6c654() {
    // IDA 0xf6c654: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_ldexp")]
// 0xf6c664 — _ldexp
// type: double __cdecl(double, int)
pub fn stub_0xf6c664() {
    // IDA 0xf6c664: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_listen")]
// 0xf6c674 — _listen
// type: int __cdecl(int, int)
pub fn stub_0xf6c674() {
    // IDA 0xf6c674: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_localeconv")]
// 0xf6c684 — _localeconv
// type: lconv *(void)
pub fn stub_0xf6c684() {
    // IDA 0xf6c684: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_localtime")]
// 0xf6c694 — _localtime
// type: tm *__cdecl(const time_t *)
pub fn stub_0xf6c694() {
    // IDA 0xf6c694: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_localtime_r")]
// 0xf6c6a4 — _localtime_r
// type: tm *__cdecl(const time_t *, tm *)
pub fn stub_0xf6c6a4() {
    // IDA 0xf6c6a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_log")]
// 0xf6c6b4 — _log
// type: double __cdecl(double)
pub fn stub_0xf6c6b4() {
    // IDA 0xf6c6b4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_log10")]
// 0xf6c6c4 — _log10
// type: double __cdecl(double)
pub fn stub_0xf6c6c4() {
    // IDA 0xf6c6c4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_log10f")]
// 0xf6c6d4 — _log10f
// type: float __cdecl(float)
pub fn stub_0xf6c6d4() {
    // IDA 0xf6c6d4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_logf")]
// 0xf6c6e4 — _logf
// type: float __cdecl(float)
pub fn stub_0xf6c6e4() {
    // IDA 0xf6c6e4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_longjmp")]
// 0xf6c6f4 — _longjmp
// type: void __cdecl __noreturn(jmp_buf, int)
pub fn stub_0xf6c6f4() {
    // IDA 0xf6c6f4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_lrint")]
// 0xf6c704 — _lrint
// type: __int32 __cdecl(double)
pub fn stub_0xf6c704() {
    // IDA 0xf6c704: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_lrintf")]
// 0xf6c714 — _lrintf
// type: __int32 __cdecl(float)
pub fn stub_0xf6c714() {
    // IDA 0xf6c714: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_lround")]
// 0xf6c724 — _lround
// type: __int32 __cdecl(double)
pub fn stub_0xf6c724() {
    // IDA 0xf6c724: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_lseek")]
// 0xf6c734 — _lseek
// type: off_t __cdecl(int, off_t, int)
pub fn stub_0xf6c734() {
    // IDA 0xf6c734: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_lstat")]
// 0xf6c744 — _lstat
// type: int __cdecl(const char *, stat *)
pub fn stub_0xf6c744() {
    // IDA 0xf6c744: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_absolute_time")]
// 0xf6c754 — _mach_absolute_time
// type: uint64_t(void)
pub fn stub_0xf6c754() {
    // IDA 0xf6c754: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_host_self")]
// 0xf6c764 — _mach_host_self
// type: mach_port_t(void)
pub fn stub_0xf6c764() {
    // IDA 0xf6c764: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_make_memory_entry_64")]
// 0xf6c774 — _mach_make_memory_entry_64
// type: kern_return_t __cdecl(vm_map_t target_task, memory_object_size_t *size, memory_object_offset_t offset, vm_prot_t permission, mach_port_t *object_handle, mem_entry_name_port_t parent_entry)
pub fn stub_0xf6c774() {
    // IDA 0xf6c774: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_msg")]
// 0xf6c784 — _mach_msg
// type: mach_msg_return_t __cdecl(mach_msg_header_t *msg, mach_msg_option_t option, mach_msg_size_t send_size, mach_msg_size_t rcv_size, mach_port_name_t rcv_name, mach_msg_timeout_t timeout, mach_port_name_t notify)
pub fn stub_0xf6c784() {
    // IDA 0xf6c784: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_port_allocate")]
// 0xf6c794 — _mach_port_allocate
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_right_t right, mach_port_name_t *name)
pub fn stub_0xf6c794() {
    // IDA 0xf6c794: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_port_deallocate")]
// 0xf6c7a4 — _mach_port_deallocate
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name)
pub fn stub_0xf6c7a4() {
    // IDA 0xf6c7a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_port_insert_right")]
// 0xf6c7b4 — _mach_port_insert_right
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name, mach_port_t poly, mach_msg_type_name_t polyPoly)
pub fn stub_0xf6c7b4() {
    // IDA 0xf6c7b4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_port_mod_refs")]
// 0xf6c7c4 — _mach_port_mod_refs
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name, mach_port_right_t right, mach_port_delta_t delta)
pub fn stub_0xf6c7c4() {
    // IDA 0xf6c7c4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_thread_self")]
// 0xf6c7d4 — _mach_thread_self
// type: mach_port_t(void)
pub fn stub_0xf6c7d4() {
    // IDA 0xf6c7d4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mach_timebase_info")]
// 0xf6c7e4 — _mach_timebase_info
// type: kern_return_t __cdecl(mach_timebase_info_t info)
pub fn stub_0xf6c7e4() {
    // IDA 0xf6c7e4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_malloc")]
// 0xf6c7f4 — _malloc
// type: void *__cdecl(size_t __size)
pub fn stub_0xf6c7f4() {
    // IDA 0xf6c7f4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_memchr")]
// 0xf6c804 — _memchr
// type: void *__cdecl(const void *__s, int __c, size_t __n)
pub fn stub_0xf6c804() {
    // IDA 0xf6c804: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_memcmp")]
// 0xf6c814 — _memcmp
// type: int __cdecl(const void *__s1, const void *__s2, size_t __n)
pub fn stub_0xf6c814() {
    // IDA 0xf6c814: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_memcpy")]
// 0xf6c824 — _memcpy
// type: void *__cdecl(void *__dst, const void *__src, size_t __n)
pub fn stub_0xf6c824() {
    // IDA 0xf6c824: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_memmove")]
// 0xf6c834 — _memmove
// type: void *__cdecl(void *__dst, const void *__src, size_t __len)
pub fn stub_0xf6c834() {
    // IDA 0xf6c834: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_memset")]
// 0xf6c844 — _memset
// type: void *__cdecl(void *__b, int __c, size_t __len)
pub fn stub_0xf6c844() {
    // IDA 0xf6c844: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_memset_pattern16")]
// 0xf6c854 — _memset_pattern16
// type: void __cdecl(void *__b, const void *__pattern16, size_t __len)
pub fn stub_0xf6c854() {
    // IDA 0xf6c854: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mkdir")]
// 0xf6c864 — _mkdir
// type: int __cdecl(const char *, mode_t)
pub fn stub_0xf6c864() {
    // IDA 0xf6c864: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mkstemp")]
// 0xf6c874 — _mkstemp
// type: int __cdecl(char *)
pub fn stub_0xf6c874() {
    // IDA 0xf6c874: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mktime")]
// 0xf6c884 — _mktime
// type: time_t __cdecl(tm *)
pub fn stub_0xf6c884() {
    // IDA 0xf6c884: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_mmap")]
// 0xf6c894 — _mmap
// type: void *__cdecl(void *, size_t, int, int, int, off_t)
pub fn stub_0xf6c894() {
    // IDA 0xf6c894: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_modf")]
// 0xf6c8a4 — _modf
// type: double __cdecl(double, double *)
pub fn stub_0xf6c8a4() {
    // IDA 0xf6c8a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_modff")]
// 0xf6c8b4 — _modff
// type: float __cdecl(float, float *)
pub fn stub_0xf6c8b4() {
    // IDA 0xf6c8b4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_munmap")]
// 0xf6c8c4 — _munmap
// type: int __cdecl(void *, size_t)
pub fn stub_0xf6c8c4() {
    // IDA 0xf6c8c4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_nanosleep")]
// 0xf6c8d4 — _nanosleep
// type: int __cdecl(const timespec *__rqtp, timespec *__rmtp)
pub fn stub_0xf6c8d4() {
    // IDA 0xf6c8d4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_open")]
// 0xf6c8e4 — _open
// type: int(const char *, int, ...)
pub fn stub_0xf6c8e4() {
    // IDA 0xf6c8e4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_opendir")]
// 0xf6c8f4 — _opendir
// type: DIR *__cdecl(const char *)
pub fn stub_0xf6c8f4() {
    // IDA 0xf6c8f4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_perror")]
// 0xf6c904 — _perror
// type: void __cdecl(const char *)
pub fn stub_0xf6c904() {
    // IDA 0xf6c904: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_pow")]
// 0xf6c914 — _pow
// type: double __cdecl(double, double)
pub fn stub_0xf6c914() {
    // IDA 0xf6c914: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_powf")]
// 0xf6c924 — _powf
// type: float __cdecl(float, float)
pub fn stub_0xf6c924() {
    // IDA 0xf6c924: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_printf")]
// 0xf6c934 — _printf
// type: int(const char *, ...)
pub fn stub_0xf6c934() {
    // IDA 0xf6c934: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_pthread_attr_destroy")]
// 0xf6c944 — _pthread_attr_destroy
// type: int __cdecl(pthread_attr_t *)
pub fn stub_0xf6c944() {
    // IDA 0xf6c944: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_attr_init")]
// 0xf6c954 — _pthread_attr_init
// type: int __cdecl(pthread_attr_t *)
pub fn stub_0xf6c954() {
    // IDA 0xf6c954: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_attr_setdetachstate")]
// 0xf6c964 — _pthread_attr_setdetachstate
// type: int __cdecl(pthread_attr_t *, int)
pub fn stub_0xf6c964() {
    // IDA 0xf6c964: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_attr_setschedparam")]
// 0xf6c974 — _pthread_attr_setschedparam
// type: int __cdecl(pthread_attr_t *, const sched_param *)
pub fn stub_0xf6c974() {
    // IDA 0xf6c974: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_attr_setschedpolicy")]
// 0xf6c984 — _pthread_attr_setschedpolicy
// type: int __cdecl(pthread_attr_t *, int)
pub fn stub_0xf6c984() {
    // IDA 0xf6c984: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_attr_setstacksize")]
// 0xf6c994 — _pthread_attr_setstacksize
// type: int __cdecl(pthread_attr_t *, size_t)
pub fn stub_0xf6c994() {
    // IDA 0xf6c994: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_cond_broadcast")]
// 0xf6c9a4 — _pthread_cond_broadcast
// type: int __cdecl(pthread_cond_t *)
pub fn stub_0xf6c9a4() {
    // IDA 0xf6c9a4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_cond_destroy")]
// 0xf6c9b4 — _pthread_cond_destroy
// type: int __cdecl(pthread_cond_t *)
pub fn stub_0xf6c9b4() {
    // IDA 0xf6c9b4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_cond_init")]
// 0xf6c9c4 — _pthread_cond_init
// type: int __cdecl(pthread_cond_t *, const pthread_condattr_t *)
pub fn stub_0xf6c9c4() {
    // IDA 0xf6c9c4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_cond_signal")]
// 0xf6c9d4 — _pthread_cond_signal
// type: int __cdecl(pthread_cond_t *)
pub fn stub_0xf6c9d4() {
    // IDA 0xf6c9d4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_cond_timedwait")]
// 0xf6c9e4 — _pthread_cond_timedwait
// type: int __cdecl(pthread_cond_t *, pthread_mutex_t *, const timespec *)
pub fn stub_0xf6c9e4() {
    // IDA 0xf6c9e4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_cond_wait")]
// 0xf6c9f4 — _pthread_cond_wait
// type: int __cdecl(pthread_cond_t *, pthread_mutex_t *)
pub fn stub_0xf6c9f4() {
    // IDA 0xf6c9f4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_condattr_destroy")]
// 0xf6ca04 — _pthread_condattr_destroy
// type: int __cdecl(pthread_condattr_t *)
pub fn stub_0xf6ca04() {
    // IDA 0xf6ca04: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_condattr_init")]
// 0xf6ca14 — _pthread_condattr_init
// type: int __cdecl(pthread_condattr_t *)
pub fn stub_0xf6ca14() {
    // IDA 0xf6ca14: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_create")]
// 0xf6ca24 — _pthread_create
// type: int __cdecl(pthread_t *, const pthread_attr_t *, void *(__cdecl *)(void *), void *)
pub fn stub_0xf6ca24() {
    // IDA 0xf6ca24: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_detach")]
// 0xf6ca34 — _pthread_detach
// type: int __cdecl(pthread_t)
pub fn stub_0xf6ca34() {
    // IDA 0xf6ca34: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_equal")]
// 0xf6ca44 — _pthread_equal
// type: int __cdecl(pthread_t, pthread_t)
pub fn stub_0xf6ca44() {
    // IDA 0xf6ca44: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_from_mach_thread_np")]
// 0xf6ca54 — _pthread_from_mach_thread_np
// type: pthread_t __cdecl(mach_port_t)
pub fn stub_0xf6ca54() {
    // IDA 0xf6ca54: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_get_stackaddr_np")]
// 0xf6ca64 — _pthread_get_stackaddr_np
// type: void *__cdecl(pthread_t)
pub fn stub_0xf6ca64() {
    // IDA 0xf6ca64: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_get_stacksize_np")]
// 0xf6ca74 — _pthread_get_stacksize_np
// type: size_t __cdecl(pthread_t)
pub fn stub_0xf6ca74() {
    // IDA 0xf6ca74: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_getname_np")]
// 0xf6ca84 — _pthread_getname_np
// type: int __cdecl(pthread_t, char *, size_t)
pub fn stub_0xf6ca84() {
    // IDA 0xf6ca84: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_getspecific")]
// 0xf6ca94 — _pthread_getspecific
// type: void *__cdecl(pthread_key_t)
pub fn stub_0xf6ca94() {
    // IDA 0xf6ca94: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_join")]
// 0xf6caa4 — _pthread_join
// type: int __cdecl(pthread_t, void **)
pub fn stub_0xf6caa4() {
    // IDA 0xf6caa4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_key_create")]
// 0xf6cab4 — _pthread_key_create
// type: int __cdecl(pthread_key_t *, void (__cdecl *)(void *))
pub fn stub_0xf6cab4() {
    // IDA 0xf6cab4: threading primitive. std::thread/parking_lot — carrier no-op.
}
