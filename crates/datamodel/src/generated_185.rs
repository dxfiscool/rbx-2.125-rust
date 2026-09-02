// Auto-generated skeletons for rbx-datamodel -- from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted — filtered complete (10215/10215), global gap filler high-EA
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xf6c614..0xf6cc44 | total filtered 10215, remaining 0 after batch; local 17798->17898 distinct, 67747->67647 not in datamodel (0 global missing)
// Shard: 185 EA-sorted asc next 100 high-EA global gap filler after 0xf6c604 not yet in datamodel (filtered exhausted, 67747 missing before -> 67647 after)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xf6c614 — _host_statistics
// type: kern_return_t __cdecl(host_t host_priv, host_flavor_t flavor, host_info_t host_info_out, mach_msg_type_number_t *host_info_outCnt)
#[doc(alias = "_host_statistics")]
pub fn stub_f6c614() -> ! {
    todo!("0xf6c614 _host_statistics")
}

// 0xf6c624 — _inet_addr
// type: in_addr_t __cdecl(const char *)
#[doc(alias = "_inet_addr")]
pub fn stub_f6c624() -> ! {
    todo!("0xf6c624 _inet_addr")
}

// 0xf6c634 — _inet_aton
// type: int __cdecl(const char *, in_addr *)
#[doc(alias = "_inet_aton")]
pub fn stub_f6c634() -> ! {
    todo!("0xf6c634 _inet_aton")
}

// 0xf6c644 — _inet_ntoa
// type: char *__cdecl(in_addr)
#[doc(alias = "_inet_ntoa")]
pub fn stub_f6c644() -> ! {
    todo!("0xf6c644 _inet_ntoa")
}

// 0xf6c654 — _kill
// type: int __cdecl(pid_t, int)
#[doc(alias = "_kill")]
pub fn stub_f6c654() -> ! {
    todo!("0xf6c654 _kill")
}

// 0xf6c664 — _ldexp
// type: double __cdecl(double, int)
#[doc(alias = "_ldexp")]
pub fn stub_f6c664() -> ! {
    todo!("0xf6c664 _ldexp")
}

// 0xf6c674 — _listen
// type: int __cdecl(int, int)
#[doc(alias = "_listen")]
pub fn stub_f6c674() -> ! {
    todo!("0xf6c674 _listen")
}

// 0xf6c684 — _localeconv
// type: lconv *(void)
#[doc(alias = "_localeconv")]
pub fn stub_f6c684() -> ! {
    todo!("0xf6c684 _localeconv")
}

// 0xf6c694 — _localtime
// type: tm *__cdecl(const time_t *)
#[doc(alias = "_localtime")]
pub fn stub_f6c694() -> ! {
    todo!("0xf6c694 _localtime")
}

// 0xf6c6a4 — _localtime_r
// type: tm *__cdecl(const time_t *, tm *)
#[doc(alias = "_localtime_r")]
pub fn stub_f6c6a4() -> ! {
    todo!("0xf6c6a4 _localtime_r")
}

// 0xf6c6b4 — _log
// type: double __cdecl(double)
#[doc(alias = "_log")]
pub fn stub_f6c6b4() -> ! {
    todo!("0xf6c6b4 _log")
}

// 0xf6c6c4 — _log10
// type: double __cdecl(double)
#[doc(alias = "_log10")]
pub fn stub_f6c6c4() -> ! {
    todo!("0xf6c6c4 _log10")
}

// 0xf6c6d4 — _log10f
// type: float __cdecl(float)
#[doc(alias = "_log10f")]
pub fn stub_f6c6d4() -> ! {
    todo!("0xf6c6d4 _log10f")
}

// 0xf6c6e4 — _logf
// type: float __cdecl(float)
#[doc(alias = "_logf")]
pub fn stub_f6c6e4() -> ! {
    todo!("0xf6c6e4 _logf")
}

// 0xf6c6f4 — _longjmp
// type: void __cdecl __noreturn(jmp_buf, int)
#[doc(alias = "_longjmp")]
pub fn stub_f6c6f4() -> ! {
    todo!("0xf6c6f4 _longjmp")
}

// 0xf6c704 — _lrint
// type: __int32 __cdecl(double)
#[doc(alias = "_lrint")]
pub fn stub_f6c704() -> ! {
    todo!("0xf6c704 _lrint")
}

// 0xf6c714 — _lrintf
// type: __int32 __cdecl(float)
#[doc(alias = "_lrintf")]
pub fn stub_f6c714() -> ! {
    todo!("0xf6c714 _lrintf")
}

// 0xf6c724 — _lround
// type: __int32 __cdecl(double)
#[doc(alias = "_lround")]
pub fn stub_f6c724() -> ! {
    todo!("0xf6c724 _lround")
}

// 0xf6c734 — _lseek
// type: off_t __cdecl(int, off_t, int)
#[doc(alias = "_lseek")]
pub fn stub_f6c734() -> ! {
    todo!("0xf6c734 _lseek")
}

// 0xf6c744 — _lstat
// type: int __cdecl(const char *, stat *)
#[doc(alias = "_lstat")]
pub fn stub_f6c744() -> ! {
    todo!("0xf6c744 _lstat")
}

// 0xf6c754 — _mach_absolute_time
// type: uint64_t(void)
#[doc(alias = "_mach_absolute_time")]
pub fn stub_f6c754() -> ! {
    todo!("0xf6c754 _mach_absolute_time")
}

// 0xf6c764 — _mach_host_self
// type: mach_port_t(void)
#[doc(alias = "_mach_host_self")]
pub fn stub_f6c764() -> ! {
    todo!("0xf6c764 _mach_host_self")
}

// 0xf6c774 — _mach_make_memory_entry_64
// type: kern_return_t __cdecl(vm_map_t target_task, memory_object_size_t *size, memory_object_offset_t offset, vm_prot_t permission, mach_port_t *object_handle, mem_entry_name_port_t parent_entry)
#[doc(alias = "_mach_make_memory_entry_64")]
pub fn stub_f6c774() -> ! {
    todo!("0xf6c774 _mach_make_memory_entry_64")
}

// 0xf6c784 — _mach_msg
// type: mach_msg_return_t __cdecl(mach_msg_header_t *msg, mach_msg_option_t option, mach_msg_size_t send_size, mach_msg_size_t rcv_size, mach_port_name_t rcv_name, mach_msg_timeout_t timeout, mach_port_name_t notify)
#[doc(alias = "_mach_msg")]
pub fn stub_f6c784() -> ! {
    todo!("0xf6c784 _mach_msg")
}

// 0xf6c794 — _mach_port_allocate
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_right_t right, mach_port_name_t *name)
#[doc(alias = "_mach_port_allocate")]
pub fn stub_f6c794() -> ! {
    todo!("0xf6c794 _mach_port_allocate")
}

// 0xf6c7a4 — _mach_port_deallocate
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name)
#[doc(alias = "_mach_port_deallocate")]
pub fn stub_f6c7a4() -> ! {
    todo!("0xf6c7a4 _mach_port_deallocate")
}

// 0xf6c7b4 — _mach_port_insert_right
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name, mach_port_t poly, mach_msg_type_name_t polyPoly)
#[doc(alias = "_mach_port_insert_right")]
pub fn stub_f6c7b4() -> ! {
    todo!("0xf6c7b4 _mach_port_insert_right")
}

// 0xf6c7c4 — _mach_port_mod_refs
// type: kern_return_t __cdecl(ipc_space_t task, mach_port_name_t name, mach_port_right_t right, mach_port_delta_t delta)
#[doc(alias = "_mach_port_mod_refs")]
pub fn stub_f6c7c4() -> ! {
    todo!("0xf6c7c4 _mach_port_mod_refs")
}

// 0xf6c7d4 — _mach_thread_self
// type: mach_port_t(void)
#[doc(alias = "_mach_thread_self")]
pub fn stub_f6c7d4() -> ! {
    todo!("0xf6c7d4 _mach_thread_self")
}

// 0xf6c7e4 — _mach_timebase_info
// type: kern_return_t __cdecl(mach_timebase_info_t info)
#[doc(alias = "_mach_timebase_info")]
pub fn stub_f6c7e4() -> ! {
    todo!("0xf6c7e4 _mach_timebase_info")
}

// 0xf6c7f4 — _malloc
// type: void *__cdecl(size_t __size)
#[doc(alias = "_malloc")]
pub fn stub_f6c7f4() -> ! {
    todo!("0xf6c7f4 _malloc")
}

// 0xf6c804 — _memchr
// type: void *__cdecl(const void *__s, int __c, size_t __n)
#[doc(alias = "_memchr")]
pub fn stub_f6c804() -> ! {
    todo!("0xf6c804 _memchr")
}

// 0xf6c814 — _memcmp
// type: int __cdecl(const void *__s1, const void *__s2, size_t __n)
#[doc(alias = "_memcmp")]
pub fn stub_f6c814() -> ! {
    todo!("0xf6c814 _memcmp")
}

// 0xf6c824 — _memcpy
// type: void *__cdecl(void *__dst, const void *__src, size_t __n)
#[doc(alias = "_memcpy")]
pub fn stub_f6c824() -> ! {
    todo!("0xf6c824 _memcpy")
}

// 0xf6c834 — _memmove
// type: void *__cdecl(void *__dst, const void *__src, size_t __len)
#[doc(alias = "_memmove")]
pub fn stub_f6c834() -> ! {
    todo!("0xf6c834 _memmove")
}

// 0xf6c844 — _memset
// type: void *__cdecl(void *__b, int __c, size_t __len)
#[doc(alias = "_memset")]
pub fn stub_f6c844() -> ! {
    todo!("0xf6c844 _memset")
}

// 0xf6c854 — _memset_pattern16
// type: void __cdecl(void *__b, const void *__pattern16, size_t __len)
#[doc(alias = "_memset_pattern16")]
pub fn stub_f6c854() -> ! {
    todo!("0xf6c854 _memset_pattern16")
}

// 0xf6c864 — _mkdir
// type: int __cdecl(const char *, mode_t)
#[doc(alias = "_mkdir")]
pub fn stub_f6c864() -> ! {
    todo!("0xf6c864 _mkdir")
}

// 0xf6c874 — _mkstemp
// type: int __cdecl(char *)
#[doc(alias = "_mkstemp")]
pub fn stub_f6c874() -> ! {
    todo!("0xf6c874 _mkstemp")
}

// 0xf6c884 — _mktime
// type: time_t __cdecl(tm *)
#[doc(alias = "_mktime")]
pub fn stub_f6c884() -> ! {
    todo!("0xf6c884 _mktime")
}

// 0xf6c894 — _mmap
// type: void *__cdecl(void *, size_t, int, int, int, off_t)
#[doc(alias = "_mmap")]
pub fn stub_f6c894() -> ! {
    todo!("0xf6c894 _mmap")
}

// 0xf6c8a4 — _modf
// type: double __cdecl(double, double *)
#[doc(alias = "_modf")]
pub fn stub_f6c8a4() -> ! {
    todo!("0xf6c8a4 _modf")
}

// 0xf6c8b4 — _modff
// type: float __cdecl(float, float *)
#[doc(alias = "_modff")]
pub fn stub_f6c8b4() -> ! {
    todo!("0xf6c8b4 _modff")
}

// 0xf6c8c4 — _munmap
// type: int __cdecl(void *, size_t)
#[doc(alias = "_munmap")]
pub fn stub_f6c8c4() -> ! {
    todo!("0xf6c8c4 _munmap")
}

// 0xf6c8d4 — _nanosleep
// type: int __cdecl(const timespec *__rqtp, timespec *__rmtp)
#[doc(alias = "_nanosleep")]
pub fn stub_f6c8d4() -> ! {
    todo!("0xf6c8d4 _nanosleep")
}

// 0xf6c8e4 — _open
// type: int(const char *, int, ...)
#[doc(alias = "_open")]
pub fn stub_f6c8e4() -> ! {
    todo!("0xf6c8e4 _open")
}

// 0xf6c8f4 — _opendir
// type: DIR *__cdecl(const char *)
#[doc(alias = "_opendir")]
pub fn stub_f6c8f4() -> ! {
    todo!("0xf6c8f4 _opendir")
}

// 0xf6c904 — _perror
// type: void __cdecl(const char *)
#[doc(alias = "_perror")]
pub fn stub_f6c904() -> ! {
    todo!("0xf6c904 _perror")
}

// 0xf6c914 — _pow
// type: double __cdecl(double, double)
#[doc(alias = "_pow")]
pub fn stub_f6c914() -> ! {
    todo!("0xf6c914 _pow")
}

// 0xf6c924 — _powf
// type: float __cdecl(float, float)
#[doc(alias = "_powf")]
pub fn stub_f6c924() -> ! {
    todo!("0xf6c924 _powf")
}

// 0xf6c934 — _printf
// type: int(const char *, ...)
#[doc(alias = "_printf")]
pub fn stub_f6c934() -> ! {
    todo!("0xf6c934 _printf")
}

// 0xf6c944 — _pthread_attr_destroy
// type: int __cdecl(pthread_attr_t *)
#[doc(alias = "_pthread_attr_destroy")]
pub fn stub_f6c944() -> ! {
    todo!("0xf6c944 _pthread_attr_destroy")
}

// 0xf6c954 — _pthread_attr_init
// type: int __cdecl(pthread_attr_t *)
#[doc(alias = "_pthread_attr_init")]
pub fn stub_f6c954() -> ! {
    todo!("0xf6c954 _pthread_attr_init")
}

// 0xf6c964 — _pthread_attr_setdetachstate
// type: int __cdecl(pthread_attr_t *, int)
#[doc(alias = "_pthread_attr_setdetachstate")]
pub fn stub_f6c964() -> ! {
    todo!("0xf6c964 _pthread_attr_setdetachstate")
}

// 0xf6c974 — _pthread_attr_setschedparam
// type: int __cdecl(pthread_attr_t *, const sched_param *)
#[doc(alias = "_pthread_attr_setschedparam")]
pub fn stub_f6c974() -> ! {
    todo!("0xf6c974 _pthread_attr_setschedparam")
}

// 0xf6c984 — _pthread_attr_setschedpolicy
// type: int __cdecl(pthread_attr_t *, int)
#[doc(alias = "_pthread_attr_setschedpolicy")]
pub fn stub_f6c984() -> ! {
    todo!("0xf6c984 _pthread_attr_setschedpolicy")
}

// 0xf6c994 — _pthread_attr_setstacksize
// type: int __cdecl(pthread_attr_t *, size_t)
#[doc(alias = "_pthread_attr_setstacksize")]
pub fn stub_f6c994() -> ! {
    todo!("0xf6c994 _pthread_attr_setstacksize")
}

// 0xf6c9a4 — _pthread_cond_broadcast
// type: int __cdecl(pthread_cond_t *)
#[doc(alias = "_pthread_cond_broadcast")]
pub fn stub_f6c9a4() -> ! {
    todo!("0xf6c9a4 _pthread_cond_broadcast")
}

// 0xf6c9b4 — _pthread_cond_destroy
// type: int __cdecl(pthread_cond_t *)
#[doc(alias = "_pthread_cond_destroy")]
pub fn stub_f6c9b4() -> ! {
    todo!("0xf6c9b4 _pthread_cond_destroy")
}

// 0xf6c9c4 — _pthread_cond_init
// type: int __cdecl(pthread_cond_t *, const pthread_condattr_t *)
#[doc(alias = "_pthread_cond_init")]
pub fn stub_f6c9c4() -> ! {
    todo!("0xf6c9c4 _pthread_cond_init")
}

// 0xf6c9d4 — _pthread_cond_signal
// type: int __cdecl(pthread_cond_t *)
#[doc(alias = "_pthread_cond_signal")]
pub fn stub_f6c9d4() -> ! {
    todo!("0xf6c9d4 _pthread_cond_signal")
}

// 0xf6c9e4 — _pthread_cond_timedwait
// type: int __cdecl(pthread_cond_t *, pthread_mutex_t *, const timespec *)
#[doc(alias = "_pthread_cond_timedwait")]
pub fn stub_f6c9e4() -> ! {
    todo!("0xf6c9e4 _pthread_cond_timedwait")
}

// 0xf6c9f4 — _pthread_cond_wait
// type: int __cdecl(pthread_cond_t *, pthread_mutex_t *)
#[doc(alias = "_pthread_cond_wait")]
pub fn stub_f6c9f4() -> ! {
    todo!("0xf6c9f4 _pthread_cond_wait")
}

// 0xf6ca04 — _pthread_condattr_destroy
// type: int __cdecl(pthread_condattr_t *)
#[doc(alias = "_pthread_condattr_destroy")]
pub fn stub_f6ca04() -> ! {
    todo!("0xf6ca04 _pthread_condattr_destroy")
}

// 0xf6ca14 — _pthread_condattr_init
// type: int __cdecl(pthread_condattr_t *)
#[doc(alias = "_pthread_condattr_init")]
pub fn stub_f6ca14() -> ! {
    todo!("0xf6ca14 _pthread_condattr_init")
}

// 0xf6ca24 — _pthread_create
// type: int __cdecl(pthread_t *, const pthread_attr_t *, void *(__cdecl *)(void *), void *)
#[doc(alias = "_pthread_create")]
pub fn stub_f6ca24() -> ! {
    todo!("0xf6ca24 _pthread_create")
}

// 0xf6ca34 — _pthread_detach
// type: int __cdecl(pthread_t)
#[doc(alias = "_pthread_detach")]
pub fn stub_f6ca34() -> ! {
    todo!("0xf6ca34 _pthread_detach")
}

// 0xf6ca44 — _pthread_equal
// type: int __cdecl(pthread_t, pthread_t)
#[doc(alias = "_pthread_equal")]
pub fn stub_f6ca44() -> ! {
    todo!("0xf6ca44 _pthread_equal")
}

// 0xf6ca54 — _pthread_from_mach_thread_np
// type: pthread_t __cdecl(mach_port_t)
#[doc(alias = "_pthread_from_mach_thread_np")]
pub fn stub_f6ca54() -> ! {
    todo!("0xf6ca54 _pthread_from_mach_thread_np")
}

// 0xf6ca64 — _pthread_get_stackaddr_np
// type: void *__cdecl(pthread_t)
#[doc(alias = "_pthread_get_stackaddr_np")]
pub fn stub_f6ca64() -> ! {
    todo!("0xf6ca64 _pthread_get_stackaddr_np")
}

// 0xf6ca74 — _pthread_get_stacksize_np
// type: size_t __cdecl(pthread_t)
#[doc(alias = "_pthread_get_stacksize_np")]
pub fn stub_f6ca74() -> ! {
    todo!("0xf6ca74 _pthread_get_stacksize_np")
}

// 0xf6ca84 — _pthread_getname_np
// type: int __cdecl(pthread_t, char *, size_t)
#[doc(alias = "_pthread_getname_np")]
pub fn stub_f6ca84() -> ! {
    todo!("0xf6ca84 _pthread_getname_np")
}

// 0xf6ca94 — _pthread_getspecific
// type: void *__cdecl(pthread_key_t)
#[doc(alias = "_pthread_getspecific")]
pub fn stub_f6ca94() -> ! {
    todo!("0xf6ca94 _pthread_getspecific")
}

// 0xf6caa4 — _pthread_join
// type: int __cdecl(pthread_t, void **)
#[doc(alias = "_pthread_join")]
pub fn stub_f6caa4() -> ! {
    todo!("0xf6caa4 _pthread_join")
}

// 0xf6cab4 — _pthread_key_create
// type: int __cdecl(pthread_key_t *, void (__cdecl *)(void *))
#[doc(alias = "_pthread_key_create")]
pub fn stub_f6cab4() -> ! {
    todo!("0xf6cab4 _pthread_key_create")
}

// 0xf6cac4 — _pthread_key_delete
// type: int __cdecl(pthread_key_t)
#[doc(alias = "_pthread_key_delete")]
pub fn stub_f6cac4() -> ! {
    todo!("0xf6cac4 _pthread_key_delete")
}

// 0xf6cad4 — _pthread_mach_thread_np
// type: mach_port_t __cdecl(pthread_t)
#[doc(alias = "_pthread_mach_thread_np")]
pub fn stub_f6cad4() -> ! {
    todo!("0xf6cad4 _pthread_mach_thread_np")
}

// 0xf6cae4 — _pthread_mutex_destroy
// type: int __cdecl(pthread_mutex_t *)
#[doc(alias = "_pthread_mutex_destroy")]
pub fn stub_f6cae4() -> ! {
    todo!("0xf6cae4 _pthread_mutex_destroy")
}

// 0xf6caf4 — _pthread_mutex_init
// type: int __cdecl(pthread_mutex_t *, const pthread_mutexattr_t *)
#[doc(alias = "_pthread_mutex_init")]
pub fn stub_f6caf4() -> ! {
    todo!("0xf6caf4 _pthread_mutex_init")
}

// 0xf6cb04 — _pthread_mutex_lock
// type: int __cdecl(pthread_mutex_t *)
#[doc(alias = "_pthread_mutex_lock")]
pub fn stub_f6cb04() -> ! {
    todo!("0xf6cb04 _pthread_mutex_lock")
}

// 0xf6cb14 — _pthread_mutex_trylock
// type: int __cdecl(pthread_mutex_t *)
#[doc(alias = "_pthread_mutex_trylock")]
pub fn stub_f6cb14() -> ! {
    todo!("0xf6cb14 _pthread_mutex_trylock")
}

// 0xf6cb24 — _pthread_mutex_unlock
// type: int __cdecl(pthread_mutex_t *)
#[doc(alias = "_pthread_mutex_unlock")]
pub fn stub_f6cb24() -> ! {
    todo!("0xf6cb24 _pthread_mutex_unlock")
}

// 0xf6cb34 — _pthread_mutexattr_destroy
// type: int __cdecl(pthread_mutexattr_t *)
#[doc(alias = "_pthread_mutexattr_destroy")]
pub fn stub_f6cb34() -> ! {
    todo!("0xf6cb34 _pthread_mutexattr_destroy")
}

// 0xf6cb44 — _pthread_mutexattr_init
// type: int __cdecl(pthread_mutexattr_t *)
#[doc(alias = "_pthread_mutexattr_init")]
pub fn stub_f6cb44() -> ! {
    todo!("0xf6cb44 _pthread_mutexattr_init")
}

// 0xf6cb54 — _pthread_mutexattr_settype
// type: int __cdecl(pthread_mutexattr_t *, int)
#[doc(alias = "_pthread_mutexattr_settype")]
pub fn stub_f6cb54() -> ! {
    todo!("0xf6cb54 _pthread_mutexattr_settype")
}

// 0xf6cb64 — _pthread_once
// type: int __cdecl(pthread_once_t *, void (*)(void))
#[doc(alias = "_pthread_once")]
pub fn stub_f6cb64() -> ! {
    todo!("0xf6cb64 _pthread_once")
}

// 0xf6cb74 — _pthread_self
// type: pthread_t(void)
#[doc(alias = "_pthread_self")]
pub fn stub_f6cb74() -> ! {
    todo!("0xf6cb74 _pthread_self")
}

// 0xf6cb84 — _pthread_setspecific
// type: int __cdecl(pthread_key_t, const void *)
#[doc(alias = "_pthread_setspecific")]
pub fn stub_f6cb84() -> ! {
    todo!("0xf6cb84 _pthread_setspecific")
}

// 0xf6cb94 — _putc
// type: int __cdecl(int, FILE *)
#[doc(alias = "_putc")]
pub fn stub_f6cb94() -> ! {
    todo!("0xf6cb94 _putc")
}

// 0xf6cba4 — _putchar
// type: int __cdecl(int)
#[doc(alias = "_putchar")]
pub fn stub_f6cba4() -> ! {
    todo!("0xf6cba4 _putchar")
}

// 0xf6cbb4 — _puts
// type: int __cdecl(const char *)
#[doc(alias = "_puts")]
pub fn stub_f6cbb4() -> ! {
    todo!("0xf6cbb4 _puts")
}

// 0xf6cbc4 — _qsort
// type: void __cdecl(void *__base, size_t __nel, size_t __width, int (__cdecl *__compar)(const void *, const void *))
#[doc(alias = "_qsort")]
pub fn stub_f6cbc4() -> ! {
    todo!("0xf6cbc4 _qsort")
}

// 0xf6cbd4 — _raise
// type: int __cdecl(int)
#[doc(alias = "_raise")]
pub fn stub_f6cbd4() -> ! {
    todo!("0xf6cbd4 _raise")
}

// 0xf6cbe4 — _rand
// type: int(void)
#[doc(alias = "_rand")]
pub fn stub_f6cbe4() -> ! {
    todo!("0xf6cbe4 _rand")
}

// 0xf6cbf4 — _read
// type: ssize_t __cdecl(int, void *, size_t)
#[doc(alias = "_read")]
pub fn stub_f6cbf4() -> ! {
    todo!("0xf6cbf4 _read")
}

// 0xf6cc04 — _readdir
// type: dirent *__cdecl(DIR *)
#[doc(alias = "_readdir")]
pub fn stub_f6cc04() -> ! {
    todo!("0xf6cc04 _readdir")
}

// 0xf6cc14 — _readdir_r
// type: int __cdecl(DIR *, dirent *, dirent **)
#[doc(alias = "_readdir_r")]
pub fn stub_f6cc14() -> ! {
    todo!("0xf6cc14 _readdir_r")
}

// 0xf6cc24 — _realloc
// type: void *__cdecl(void *__ptr, size_t __size)
#[doc(alias = "_realloc")]
pub fn stub_f6cc24() -> ! {
    todo!("0xf6cc24 _realloc")
}

// 0xf6cc34 — _reallocf
// type: void *__cdecl(void *__ptr, size_t __size)
#[doc(alias = "_reallocf")]
pub fn stub_f6cc34() -> ! {
    todo!("0xf6cc34 _reallocf")
}

// 0xf6cc44 — _recv
// type: ssize_t __cdecl(int, void *, size_t, int)
#[doc(alias = "_recv")]
pub fn stub_f6cc44() -> ! {
    todo!("0xf6cc44 _recv")
}
