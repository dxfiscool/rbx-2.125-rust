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
pub fn stub_0xf6c484() -> ! { todo!("0xf6c484 _fseeko") }

#[doc(alias = "_fstat")]
// 0xf6c494 — _fstat
// type: int __cdecl(int, stat *)
pub fn stub_0xf6c494() -> ! { todo!("0xf6c494 _fstat") }

#[doc(alias = "_ftell")]
// 0xf6c4a4 — _ftell
// type: __int32 __cdecl(FILE *)
pub fn stub_0xf6c4a4() -> ! { todo!("0xf6c4a4 _ftell") }

#[doc(alias = "_fwrite")]
// 0xf6c4b4 — _fwrite
// type: size_t __cdecl(const void *__ptr, size_t __size, size_t __nitems, FILE *__stream)
pub fn stub_0xf6c4b4() -> ! { todo!("0xf6c4b4 _fwrite") }

#[doc(alias = "_getaddrinfo")]
// 0xf6c4c4 — _getaddrinfo
// type: int __cdecl(const char *, const char *, const addrinfo *, addrinfo **)
pub fn stub_0xf6c4c4() -> ! { todo!("0xf6c4c4 _getaddrinfo") }

#[doc(alias = "_getc")]
// 0xf6c4d4 — _getc
// type: int __cdecl(FILE *)
pub fn stub_0xf6c4d4() -> ! { todo!("0xf6c4d4 _getc") }

#[doc(alias = "_getchar")]
// 0xf6c4e4 — _getchar
// type: int(void)
pub fn stub_0xf6c4e4() -> ! { todo!("0xf6c4e4 _getchar") }

#[doc(alias = "_getcwd")]
// 0xf6c4f4 — _getcwd
// type: char *__cdecl(char *, size_t)
pub fn stub_0xf6c4f4() -> ! { todo!("0xf6c4f4 _getcwd") }

#[doc(alias = "_getenv")]
// 0xf6c504 — _getenv
// type: char *__cdecl(const char *)
pub fn stub_0xf6c504() -> ! { todo!("0xf6c504 _getenv") }

#[doc(alias = "_getgid")]
// 0xf6c514 — _getgid
// type: gid_t(void)
pub fn stub_0xf6c514() -> ! { todo!("0xf6c514 _getgid") }

#[doc(alias = "_gethostbyname")]
// 0xf6c524 — _gethostbyname
// type: hostent *__cdecl(const char *)
pub fn stub_0xf6c524() -> ! { todo!("0xf6c524 _gethostbyname") }

#[doc(alias = "_getifaddrs")]
// 0xf6c534 — _getifaddrs
// type: int __cdecl(ifaddrs **)
pub fn stub_0xf6c534() -> ! { todo!("0xf6c534 _getifaddrs") }

#[doc(alias = "_getnameinfo")]
// 0xf6c544 — _getnameinfo
// type: int __cdecl(const sockaddr *, socklen_t, char *, socklen_t, char *, socklen_t, int)
pub fn stub_0xf6c544() -> ! { todo!("0xf6c544 _getnameinfo") }

#[doc(alias = "_getpagesize")]
// 0xf6c554 — _getpagesize
// type: int(void)
pub fn stub_0xf6c554() -> ! { todo!("0xf6c554 _getpagesize") }

#[doc(alias = "_getpid")]
// 0xf6c564 — _getpid
// type: pid_t(void)
pub fn stub_0xf6c564() -> ! { todo!("0xf6c564 _getpid") }

#[doc(alias = "_getppid")]
// 0xf6c574 — _getppid
// type: pid_t(void)
pub fn stub_0xf6c574() -> ! { todo!("0xf6c574 _getppid") }

#[doc(alias = "_getprogname")]
// 0xf6c584 — _getprogname
// type: const char *(void)
pub fn stub_0xf6c584() -> ! { todo!("0xf6c584 _getprogname") }

#[doc(alias = "_getpwuid")]
// 0xf6c594 — _getpwuid
// type: passwd *__cdecl(uid_t)
pub fn stub_0xf6c594() -> ! { todo!("0xf6c594 _getpwuid") }

#[doc(alias = "_getrusage")]
// 0xf6c5a4 — _getrusage
// type: int __cdecl(int, rusage *)
pub fn stub_0xf6c5a4() -> ! { todo!("0xf6c5a4 _getrusage") }

#[doc(alias = "_getsockname")]
// 0xf6c5b4 — _getsockname
// type: int __cdecl(int, sockaddr *, socklen_t *)
pub fn stub_0xf6c5b4() -> ! { todo!("0xf6c5b4 _getsockname") }

#[doc(alias = "_getsockopt")]
// 0xf6c5c4 — _getsockopt
// type: int __cdecl(int, int, int, void *, socklen_t *)
pub fn stub_0xf6c5c4() -> ! { todo!("0xf6c5c4 _getsockopt") }

#[doc(alias = "_gettimeofday")]
// 0xf6c5d4 — _gettimeofday
// type: int __cdecl(timeval *, void *)
pub fn stub_0xf6c5d4() -> ! { todo!("0xf6c5d4 _gettimeofday") }

#[doc(alias = "_getuid")]
// 0xf6c5e4 — _getuid
// type: uid_t(void)
pub fn stub_0xf6c5e4() -> ! { todo!("0xf6c5e4 _getuid") }

#[doc(alias = "_gmtime_r")]
// 0xf6c5f4 — _gmtime_r
// type: tm *__cdecl(const time_t *, tm *)
pub fn stub_0xf6c5f4() -> ! { todo!("0xf6c5f4 _gmtime_r") }

#[doc(alias = "_host_page_size")]
// 0xf6c604 — _host_page_size
// type: kern_return_t __cdecl(host_t, vm_size_t *)
pub fn stub_0xf6c604() -> ! { todo!("0xf6c604 _host_page_size") }

#[doc(alias = "_host_statistics")]
// 0xf6c614 — _host_statistics
// type: kern_return_t __cdecl(host_t host_priv, host_flavor_t flavor, host_info_t host_info_out, mach_msg_type_number_t *host_info_outCnt)
pub fn stub_0xf6c614() -> ! { todo!("0xf6c614 _host_statistics") }

#[doc(alias = "_inet_addr")]
// 0xf6c624 — _inet_addr
// type: in_addr_t __cdecl(const char *)
pub fn stub_0xf6c624() -> ! { todo!("0xf6c624 _inet_addr") }

#[doc(alias = "_inet_aton")]
// 0xf6c634 — _inet_aton
// type: int __cdecl(const char *, in_addr *)
pub fn stub_0xf6c634() -> ! { todo!("0xf6c634 _inet_aton") }

#[doc(alias = "_inet_ntoa")]
// 0xf6c644 — _inet_ntoa
// type: char *__cdecl(in_addr)
pub fn stub_0xf6c644() -> ! { todo!("0xf6c644 _inet_ntoa") }

#[doc(alias = "_kill")]
// 0xf6c654 — _kill
// type: int __cdecl(pid_t, int)
pub fn stub_0xf6c654() -> ! { todo!("0xf6c654 _kill") }

#[doc(alias = "_ldexp")]
// 0xf6c664 — _ldexp
// type: double __cdecl(double, int)
pub fn stub_0xf6c664() -> ! { todo!("0xf6c664 _ldexp") }

#[doc(alias = "_listen")]
// 0xf6c674 — _listen
// type: int __cdecl(int, int)
pub fn stub_0xf6c674() -> ! { todo!("0xf6c674 _listen") }

#[doc(alias = "_localeconv")]
// 0xf6c684 — _localeconv
// type: lconv *(void)
pub fn stub_0xf6c684() -> ! { todo!("0xf6c684 _localeconv") }

#[doc(alias = "_localtime")]
// 0xf6c694 — _localtime
// type: tm *__cdecl(const time_t *)
pub fn stub_0xf6c694() -> ! { todo!("0xf6c694 _localtime") }

#[doc(alias = "_localtime_r")]
// 0xf6c6a4 — _localtime_r
// type: tm *__cdecl(const time_t *, tm *)
pub fn stub_0xf6c6a4() -> ! { todo!("0xf6c6a4 _localtime_r") }

#[doc(alias = "_log")]
// 0xf6c6b4 — _log
// type: double __cdecl(double)
pub fn stub_0xf6c6b4() -> ! { todo!("0xf6c6b4 _log") }

#[doc(alias = "_log10")]
// 0xf6c6c4 — _log10
// type: double __cdecl(double)
pub fn stub_0xf6c6c4() -> ! { todo!("0xf6c6c4 _log10") }

#[doc(alias = "_log10f")]
// 0xf6c6d4 — _log10f
// type: float __cdecl(float)
pub fn stub_0xf6c6d4() -> ! { todo!("0xf6c6d4 _log10f") }

#[doc(alias = "_logf")]
// 0xf6c6e4 — _logf
// type: float __cdecl(float)
pub fn stub_0xf6c6e4() -> ! { todo!("0xf6c6e4 _logf") }

#[doc(alias = "_longjmp")]
// 0xf6c6f4 — _longjmp
// type: void __cdecl __noreturn(jmp_buf, int)
pub fn stub_0xf6c6f4() -> ! { todo!("0xf6c6f4 _longjmp") }

#[doc(alias = "_lrint")]
// 0xf6c704 — _lrint
// type: __int32 __cdecl(double)
pub fn stub_0xf6c704() -> ! { todo!("0xf6c704 _lrint") }

#[doc(alias = "_lrintf")]
// 0xf6c714 — _lrintf
// type: __int32 __cdecl(float)
pub fn stub_0xf6c714() -> ! { todo!("0xf6c714 _lrintf") }

#[doc(alias = "_lround")]
// 0xf6c724 — _lround
// type: __int32 __cdecl(double)
pub fn stub_0xf6c724() -> ! { todo!("0xf6c724 _lround") }

#[doc(alias = "_lseek")]
// 0xf6c734 — _lseek
// type: off_t __cdecl(int, off_t, int)
pub fn stub_0xf6c734() -> ! { todo!("0xf6c734 _lseek") }

#[doc(alias = "_lstat")]
// 0xf6c744 — _lstat
// type: int __cdecl(const char *, stat *)
pub fn stub_0xf6c744() -> ! { todo!("0xf6c744 _lstat") }

#[doc(alias = "_mach_absolute_time")]
// 0xf6c754 — _mach_absolute_time
// type: uint64_t(void)
pub fn stub_0xf6c754() -> ! { todo!("0xf6c754 _mach_absolute_time") }

#[doc(alias = "_mach_host_self")]
// 0xf6c764 — _mach_host_self
// type: mach_port_t(void)
pub fn stub_0xf6c764() -> ! { todo!("0xf6c764 _mach_host_self") }

#[doc(alias = "_mach_make_memory_entry_64")]
// 0xf6c774 — _mach_make_memory_entry_64
// type: kern_return_t __cdecl(vm_map_t target_task, memory_object_size_t *size, memory_object_offset_t offset, vm_prot_t permission, mach_port_t *object_handle, mem_entry_name_port_t parent_entry)
pub fn stub_0xf6c774() -> ! { todo!("0xf6c774 _mach_make_memory_entry_64") }

#[doc(alias = "_mach_msg")]
// 0xf6c784 — _mach_msg
// type: mach_msg_return_t __cdecl(mach_msg_header_t *msg, mach_msg_option_t option, mach_msg_size_t send_size, mach_msg_size_t rcv_size, mach_port_name_t rcv_name, mach_msg_timeout_t timeout, mach_port_name_t notify)
pub fn stub_0xf6c784() -> ! { todo!("0xf6c784 _mach_msg") }

#[doc(alias = "_mach_port_allocate")]
// 0xf6c794 — _mach_port_allocate
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_right_t right, mach_port_name_t *name)
pub fn stub_0xf6c794() -> ! { todo!("0xf6c794 _mach_port_allocate") }

#[doc(alias = "_mach_port_deallocate")]
// 0xf6c7a4 — _mach_port_deallocate
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name)
pub fn stub_0xf6c7a4() -> ! { todo!("0xf6c7a4 _mach_port_deallocate") }

#[doc(alias = "_mach_port_insert_right")]
// 0xf6c7b4 — _mach_port_insert_right
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name, mach_port_t poly, mach_msg_type_name_t polyPoly)
pub fn stub_0xf6c7b4() -> ! { todo!("0xf6c7b4 _mach_port_insert_right") }

#[doc(alias = "_mach_port_mod_refs")]
// 0xf6c7c4 — _mach_port_mod_refs
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name, mach_port_right_t right, mach_port_delta_t delta)
pub fn stub_0xf6c7c4() -> ! { todo!("0xf6c7c4 _mach_port_mod_refs") }

#[doc(alias = "_mach_thread_self")]
// 0xf6c7d4 — _mach_thread_self
// type: mach_port_t(void)
pub fn stub_0xf6c7d4() -> ! { todo!("0xf6c7d4 _mach_thread_self") }

#[doc(alias = "_mach_timebase_info")]
// 0xf6c7e4 — _mach_timebase_info
// type: kern_return_t __cdecl(mach_timebase_info_t info)
pub fn stub_0xf6c7e4() -> ! { todo!("0xf6c7e4 _mach_timebase_info") }

#[doc(alias = "_malloc")]
// 0xf6c7f4 — _malloc
// type: void *__cdecl(size_t __size)
pub fn stub_0xf6c7f4() -> ! { todo!("0xf6c7f4 _malloc") }

#[doc(alias = "_memchr")]
// 0xf6c804 — _memchr
// type: void *__cdecl(const void *__s, int __c, size_t __n)
pub fn stub_0xf6c804() -> ! { todo!("0xf6c804 _memchr") }

#[doc(alias = "_memcmp")]
// 0xf6c814 — _memcmp
// type: int __cdecl(const void *__s1, const void *__s2, size_t __n)
pub fn stub_0xf6c814() -> ! { todo!("0xf6c814 _memcmp") }

#[doc(alias = "_memcpy")]
// 0xf6c824 — _memcpy
// type: void *__cdecl(void *__dst, const void *__src, size_t __n)
pub fn stub_0xf6c824() -> ! { todo!("0xf6c824 _memcpy") }

#[doc(alias = "_memmove")]
// 0xf6c834 — _memmove
// type: void *__cdecl(void *__dst, const void *__src, size_t __len)
pub fn stub_0xf6c834() -> ! { todo!("0xf6c834 _memmove") }

#[doc(alias = "_memset")]
// 0xf6c844 — _memset
// type: void *__cdecl(void *__b, int __c, size_t __len)
pub fn stub_0xf6c844() -> ! { todo!("0xf6c844 _memset") }

#[doc(alias = "_memset_pattern16")]
// 0xf6c854 — _memset_pattern16
// type: void __cdecl(void *__b, const void *__pattern16, size_t __len)
pub fn stub_0xf6c854() -> ! { todo!("0xf6c854 _memset_pattern16") }

#[doc(alias = "_mkdir")]
// 0xf6c864 — _mkdir
// type: int __cdecl(const char *, mode_t)
pub fn stub_0xf6c864() -> ! { todo!("0xf6c864 _mkdir") }

#[doc(alias = "_mkstemp")]
// 0xf6c874 — _mkstemp
// type: int __cdecl(char *)
pub fn stub_0xf6c874() -> ! { todo!("0xf6c874 _mkstemp") }

#[doc(alias = "_mktime")]
// 0xf6c884 — _mktime
// type: time_t __cdecl(tm *)
pub fn stub_0xf6c884() -> ! { todo!("0xf6c884 _mktime") }

#[doc(alias = "_mmap")]
// 0xf6c894 — _mmap
// type: void *__cdecl(void *, size_t, int, int, int, off_t)
pub fn stub_0xf6c894() -> ! { todo!("0xf6c894 _mmap") }

#[doc(alias = "_modf")]
// 0xf6c8a4 — _modf
// type: double __cdecl(double, double *)
pub fn stub_0xf6c8a4() -> ! { todo!("0xf6c8a4 _modf") }

#[doc(alias = "_modff")]
// 0xf6c8b4 — _modff
// type: float __cdecl(float, float *)
pub fn stub_0xf6c8b4() -> ! { todo!("0xf6c8b4 _modff") }

#[doc(alias = "_munmap")]
// 0xf6c8c4 — _munmap
// type: int __cdecl(void *, size_t)
pub fn stub_0xf6c8c4() -> ! { todo!("0xf6c8c4 _munmap") }

#[doc(alias = "_nanosleep")]
// 0xf6c8d4 — _nanosleep
// type: int __cdecl(const timespec *__rqtp, timespec *__rmtp)
pub fn stub_0xf6c8d4() -> ! { todo!("0xf6c8d4 _nanosleep") }

#[doc(alias = "_open")]
// 0xf6c8e4 — _open
// type: int(const char *, int, ...)
pub fn stub_0xf6c8e4() -> ! { todo!("0xf6c8e4 _open") }

#[doc(alias = "_opendir")]
// 0xf6c8f4 — _opendir
// type: DIR *__cdecl(const char *)
pub fn stub_0xf6c8f4() -> ! { todo!("0xf6c8f4 _opendir") }

#[doc(alias = "_perror")]
// 0xf6c904 — _perror
// type: void __cdecl(const char *)
pub fn stub_0xf6c904() -> ! { todo!("0xf6c904 _perror") }

#[doc(alias = "_pow")]
// 0xf6c914 — _pow
// type: double __cdecl(double, double)
pub fn stub_0xf6c914() -> ! { todo!("0xf6c914 _pow") }

#[doc(alias = "_powf")]
// 0xf6c924 — _powf
// type: float __cdecl(float, float)
pub fn stub_0xf6c924() -> ! { todo!("0xf6c924 _powf") }

#[doc(alias = "_printf")]
// 0xf6c934 — _printf
// type: int(const char *, ...)
pub fn stub_0xf6c934() -> ! { todo!("0xf6c934 _printf") }

#[doc(alias = "_pthread_attr_destroy")]
// 0xf6c944 — _pthread_attr_destroy
// type: int __cdecl(pthread_attr_t *)
pub fn stub_0xf6c944() -> ! { todo!("0xf6c944 _pthread_attr_destroy") }

#[doc(alias = "_pthread_attr_init")]
// 0xf6c954 — _pthread_attr_init
// type: int __cdecl(pthread_attr_t *)
pub fn stub_0xf6c954() -> ! { todo!("0xf6c954 _pthread_attr_init") }

#[doc(alias = "_pthread_attr_setdetachstate")]
// 0xf6c964 — _pthread_attr_setdetachstate
// type: int __cdecl(pthread_attr_t *, int)
pub fn stub_0xf6c964() -> ! { todo!("0xf6c964 _pthread_attr_setdetachstate") }

#[doc(alias = "_pthread_attr_setschedparam")]
// 0xf6c974 — _pthread_attr_setschedparam
// type: int __cdecl(pthread_attr_t *, const sched_param *)
pub fn stub_0xf6c974() -> ! { todo!("0xf6c974 _pthread_attr_setschedparam") }

#[doc(alias = "_pthread_attr_setschedpolicy")]
// 0xf6c984 — _pthread_attr_setschedpolicy
// type: int __cdecl(pthread_attr_t *, int)
pub fn stub_0xf6c984() -> ! { todo!("0xf6c984 _pthread_attr_setschedpolicy") }

#[doc(alias = "_pthread_attr_setstacksize")]
// 0xf6c994 — _pthread_attr_setstacksize
// type: int __cdecl(pthread_attr_t *, size_t)
pub fn stub_0xf6c994() -> ! { todo!("0xf6c994 _pthread_attr_setstacksize") }

#[doc(alias = "_pthread_cond_broadcast")]
// 0xf6c9a4 — _pthread_cond_broadcast
// type: int __cdecl(pthread_cond_t *)
pub fn stub_0xf6c9a4() -> ! { todo!("0xf6c9a4 _pthread_cond_broadcast") }

#[doc(alias = "_pthread_cond_destroy")]
// 0xf6c9b4 — _pthread_cond_destroy
// type: int __cdecl(pthread_cond_t *)
pub fn stub_0xf6c9b4() -> ! { todo!("0xf6c9b4 _pthread_cond_destroy") }

#[doc(alias = "_pthread_cond_init")]
// 0xf6c9c4 — _pthread_cond_init
// type: int __cdecl(pthread_cond_t *, const pthread_condattr_t *)
pub fn stub_0xf6c9c4() -> ! { todo!("0xf6c9c4 _pthread_cond_init") }

#[doc(alias = "_pthread_cond_signal")]
// 0xf6c9d4 — _pthread_cond_signal
// type: int __cdecl(pthread_cond_t *)
pub fn stub_0xf6c9d4() -> ! { todo!("0xf6c9d4 _pthread_cond_signal") }

#[doc(alias = "_pthread_cond_timedwait")]
// 0xf6c9e4 — _pthread_cond_timedwait
// type: int __cdecl(pthread_cond_t *, pthread_mutex_t *, const timespec *)
pub fn stub_0xf6c9e4() -> ! { todo!("0xf6c9e4 _pthread_cond_timedwait") }

#[doc(alias = "_pthread_cond_wait")]
// 0xf6c9f4 — _pthread_cond_wait
// type: int __cdecl(pthread_cond_t *, pthread_mutex_t *)
pub fn stub_0xf6c9f4() -> ! { todo!("0xf6c9f4 _pthread_cond_wait") }

#[doc(alias = "_pthread_condattr_destroy")]
// 0xf6ca04 — _pthread_condattr_destroy
// type: int __cdecl(pthread_condattr_t *)
pub fn stub_0xf6ca04() -> ! { todo!("0xf6ca04 _pthread_condattr_destroy") }

#[doc(alias = "_pthread_condattr_init")]
// 0xf6ca14 — _pthread_condattr_init
// type: int __cdecl(pthread_condattr_t *)
pub fn stub_0xf6ca14() -> ! { todo!("0xf6ca14 _pthread_condattr_init") }

#[doc(alias = "_pthread_create")]
// 0xf6ca24 — _pthread_create
// type: int __cdecl(pthread_t *, const pthread_attr_t *, void *(__cdecl *)(void *), void *)
pub fn stub_0xf6ca24() -> ! { todo!("0xf6ca24 _pthread_create") }

#[doc(alias = "_pthread_detach")]
// 0xf6ca34 — _pthread_detach
// type: int __cdecl(pthread_t)
pub fn stub_0xf6ca34() -> ! { todo!("0xf6ca34 _pthread_detach") }

#[doc(alias = "_pthread_equal")]
// 0xf6ca44 — _pthread_equal
// type: int __cdecl(pthread_t, pthread_t)
pub fn stub_0xf6ca44() -> ! { todo!("0xf6ca44 _pthread_equal") }

#[doc(alias = "_pthread_from_mach_thread_np")]
// 0xf6ca54 — _pthread_from_mach_thread_np
// type: pthread_t __cdecl(mach_port_t)
pub fn stub_0xf6ca54() -> ! { todo!("0xf6ca54 _pthread_from_mach_thread_np") }

#[doc(alias = "_pthread_get_stackaddr_np")]
// 0xf6ca64 — _pthread_get_stackaddr_np
// type: void *__cdecl(pthread_t)
pub fn stub_0xf6ca64() -> ! { todo!("0xf6ca64 _pthread_get_stackaddr_np") }

#[doc(alias = "_pthread_get_stacksize_np")]
// 0xf6ca74 — _pthread_get_stacksize_np
// type: size_t __cdecl(pthread_t)
pub fn stub_0xf6ca74() -> ! { todo!("0xf6ca74 _pthread_get_stacksize_np") }

#[doc(alias = "_pthread_getname_np")]
// 0xf6ca84 — _pthread_getname_np
// type: int __cdecl(pthread_t, char *, size_t)
pub fn stub_0xf6ca84() -> ! { todo!("0xf6ca84 _pthread_getname_np") }

#[doc(alias = "_pthread_getspecific")]
// 0xf6ca94 — _pthread_getspecific
// type: void *__cdecl(pthread_key_t)
pub fn stub_0xf6ca94() -> ! { todo!("0xf6ca94 _pthread_getspecific") }

#[doc(alias = "_pthread_join")]
// 0xf6caa4 — _pthread_join
// type: int __cdecl(pthread_t, void **)
pub fn stub_0xf6caa4() -> ! { todo!("0xf6caa4 _pthread_join") }

#[doc(alias = "_pthread_key_create")]
// 0xf6cab4 — _pthread_key_create
// type: int __cdecl(pthread_key_t *, void (__cdecl *)(void *))
pub fn stub_0xf6cab4() -> ! { todo!("0xf6cab4 _pthread_key_create") }
