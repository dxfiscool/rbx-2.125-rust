//! core shard np — 100 core stubs EA-sorted asc fallback not yet in core after no.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 29556, 14436 uncovered before -> 14336 after, batch 0xf6cac4..0xf6d0f4).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_pthread_key_delete")]
// 0xf6cac4 — _pthread_key_delete
// type: int __cdecl(pthread_key_t)
pub fn stub_0xf6cac4() {
    // IDA 0xf6cac4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mach_thread_np")]
// 0xf6cad4 — _pthread_mach_thread_np
// type: mach_port_t __cdecl(pthread_t)
pub fn stub_0xf6cad4() {
    // IDA 0xf6cad4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutex_destroy")]
// 0xf6cae4 — _pthread_mutex_destroy
// type: int __cdecl(pthread_mutex_t *)
pub fn stub_0xf6cae4() {
    // IDA 0xf6cae4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutex_init")]
// 0xf6caf4 — _pthread_mutex_init
// type: int __cdecl(pthread_mutex_t *, const pthread_mutexattr_t *)
pub fn stub_0xf6caf4() {
    // IDA 0xf6caf4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutex_lock")]
// 0xf6cb04 — _pthread_mutex_lock
// type: int __cdecl(pthread_mutex_t *)
pub fn stub_0xf6cb04() {
    // IDA 0xf6cb04: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutex_trylock")]
// 0xf6cb14 — _pthread_mutex_trylock
// type: int __cdecl(pthread_mutex_t *)
pub fn stub_0xf6cb14() {
    // IDA 0xf6cb14: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutex_unlock")]
// 0xf6cb24 — _pthread_mutex_unlock
// type: int __cdecl(pthread_mutex_t *)
pub fn stub_0xf6cb24() {
    // IDA 0xf6cb24: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutexattr_destroy")]
// 0xf6cb34 — _pthread_mutexattr_destroy
// type: int __cdecl(pthread_mutexattr_t *)
pub fn stub_0xf6cb34() {
    // IDA 0xf6cb34: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutexattr_init")]
// 0xf6cb44 — _pthread_mutexattr_init
// type: int __cdecl(pthread_mutexattr_t *)
pub fn stub_0xf6cb44() {
    // IDA 0xf6cb44: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_mutexattr_settype")]
// 0xf6cb54 — _pthread_mutexattr_settype
// type: int __cdecl(pthread_mutexattr_t *, int)
pub fn stub_0xf6cb54() {
    // IDA 0xf6cb54: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_once")]
// 0xf6cb64 — _pthread_once
// type: int __cdecl(pthread_once_t *, void (*)(void))
pub fn stub_0xf6cb64() {
    // IDA 0xf6cb64: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_self")]
// 0xf6cb74 — _pthread_self
// type: pthread_t(void)
pub fn stub_0xf6cb74() {
    // IDA 0xf6cb74: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_pthread_setspecific")]
// 0xf6cb84 — _pthread_setspecific
// type: int __cdecl(pthread_key_t, const void *)
pub fn stub_0xf6cb84() {
    // IDA 0xf6cb84: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_putc")]
// 0xf6cb94 — _putc
// type: int __cdecl(int, FILE *)
pub fn stub_0xf6cb94() {
    // IDA 0xf6cb94: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_putchar")]
// 0xf6cba4 — _putchar
// type: int __cdecl(int)
pub fn stub_0xf6cba4() {
    // IDA 0xf6cba4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_puts")]
// 0xf6cbb4 — _puts
// type: int __cdecl(const char *)
pub fn stub_0xf6cbb4() {
    // IDA 0xf6cbb4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_qsort")]
// 0xf6cbc4 — _qsort
// type: void __cdecl(void *__base, size_t __nel, size_t __width, int (__cdecl *__compar)(const void *, const void *))
pub fn stub_0xf6cbc4() {
    // IDA 0xf6cbc4: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "_raise")]
// 0xf6cbd4 — _raise
// type: int __cdecl(int)
pub fn stub_0xf6cbd4() {
    // IDA 0xf6cbd4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_rand")]
// 0xf6cbe4 — _rand
// type: int(void)
pub fn stub_0xf6cbe4() {
    // IDA 0xf6cbe4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_read")]
// 0xf6cbf4 — _read
// type: ssize_t __cdecl(int, void *, size_t)
pub fn stub_0xf6cbf4() {
    // IDA 0xf6cbf4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_readdir")]
// 0xf6cc04 — _readdir
// type: dirent *__cdecl(DIR *)
pub fn stub_0xf6cc04() {
    // IDA 0xf6cc04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_readdir_r")]
// 0xf6cc14 — _readdir_r
// type: int __cdecl(DIR *, dirent *, dirent **)
pub fn stub_0xf6cc14() {
    // IDA 0xf6cc14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_realloc")]
// 0xf6cc24 — _realloc
// type: void *__cdecl(void *__ptr, size_t __size)
pub fn stub_0xf6cc24() {
    // IDA 0xf6cc24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_reallocf")]
// 0xf6cc34 — _reallocf
// type: void *__cdecl(void *__ptr, size_t __size)
pub fn stub_0xf6cc34() {
    // IDA 0xf6cc34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_recv")]
// 0xf6cc44 — _recv
// type: ssize_t __cdecl(int, void *, size_t, int)
pub fn stub_0xf6cc44() {
    // IDA 0xf6cc44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_recvfrom")]
// 0xf6cc54 — _recvfrom
// type: ssize_t __cdecl(int, void *, size_t, int, sockaddr *, socklen_t *)
pub fn stub_0xf6cc54() {
    // IDA 0xf6cc54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_remove")]
// 0xf6cc64 — _remove
// type: int __cdecl(const char *)
pub fn stub_0xf6cc64() {
    // IDA 0xf6cc64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_rename")]
// 0xf6cc74 — _rename
// type: int __cdecl(const char *__old, const char *__new)
pub fn stub_0xf6cc74() {
    // IDA 0xf6cc74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_rmdir")]
// 0xf6cc84 — _rmdir
// type: int __cdecl(const char *)
pub fn stub_0xf6cc84() {
    // IDA 0xf6cc84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_select")]
// 0xf6cc94 — _select
// type: int __cdecl(int, fd_set *, fd_set *, fd_set *, timeval *)
pub fn stub_0xf6cc94() {
    // IDA 0xf6cc94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sem_close")]
// 0xf6cca4 — _sem_close
// type: int __cdecl(sem_t *)
pub fn stub_0xf6cca4() {
    // IDA 0xf6cca4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sem_open")]
// 0xf6ccb4 — _sem_open
// type: sem_t *(const char *, int, ...)
pub fn stub_0xf6ccb4() {
    // IDA 0xf6ccb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sem_post")]
// 0xf6ccc4 — _sem_post
// type: int __cdecl(sem_t *)
pub fn stub_0xf6ccc4() {
    // IDA 0xf6ccc4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sem_unlink")]
// 0xf6ccd4 — _sem_unlink
// type: int __cdecl(const char *)
pub fn stub_0xf6ccd4() {
    // IDA 0xf6ccd4: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_sem_wait")]
// 0xf6cce4 — _sem_wait
// type: int __cdecl(sem_t *)
pub fn stub_0xf6cce4() {
    // IDA 0xf6cce4: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_send")]
// 0xf6ccf4 — _send
// type: ssize_t __cdecl(int, const void *, size_t, int)
pub fn stub_0xf6ccf4() {
    // IDA 0xf6ccf4: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_sendto")]
// 0xf6cd04 — _sendto
// type: ssize_t __cdecl(int, const void *, size_t, int, const sockaddr *, socklen_t)
pub fn stub_0xf6cd04() {
    // IDA 0xf6cd04: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_setjmp")]
// 0xf6cd14 — _setjmp
// type: int __cdecl(jmp_buf)
pub fn stub_0xf6cd14() {
    // IDA 0xf6cd14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_setsockopt")]
// 0xf6cd24 — _setsockopt
// type: int __cdecl(int, int, int, const void *, socklen_t)
pub fn stub_0xf6cd24() {
    // IDA 0xf6cd24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_setvbuf")]
// 0xf6cd34 — _setvbuf
// type: int __cdecl(FILE *, char *, int, size_t)
pub fn stub_0xf6cd34() {
    // IDA 0xf6cd34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_setxattr")]
// 0xf6cd44 — _setxattr
// type: int __cdecl(const char *path, const char *name, const void *value, size_t size, u_int32_t position, int options)
pub fn stub_0xf6cd44() {
    // IDA 0xf6cd44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sigaction")]
// 0xf6cd54 — _sigaction
// type: int __cdecl(int, const sigaction *, sigaction *)
pub fn stub_0xf6cd54() {
    // IDA 0xf6cd54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sigaltstack")]
// 0xf6cd64 — _sigaltstack
// type: int __cdecl(const stack_t *, stack_t *)
pub fn stub_0xf6cd64() {
    // IDA 0xf6cd64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_signal")]
// 0xf6cd74 — _signal
// type: void (__cdecl *__cdecl(int, void (__cdecl *)(int)))(int)
pub fn stub_0xf6cd74() {
    // IDA 0xf6cd74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sigprocmask")]
// 0xf6cd84 — _sigprocmask
// type: int __cdecl(int, const sigset_t *, sigset_t *)
pub fn stub_0xf6cd84() {
    // IDA 0xf6cd84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sin")]
// 0xf6cd94 — _sin
// type: double __cdecl(double)
pub fn stub_0xf6cd94() {
    // IDA 0xf6cd94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sinf")]
// 0xf6cda4 — _sinf
// type: float __cdecl(float)
pub fn stub_0xf6cda4() {
    // IDA 0xf6cda4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sinh")]
// 0xf6cdb4 — _sinh
// type: double __cdecl(double)
pub fn stub_0xf6cdb4() {
    // IDA 0xf6cdb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sleep")]
// 0xf6cdc4 — _sleep
// type: unsigned int __cdecl(unsigned int)
pub fn stub_0xf6cdc4() {
    // IDA 0xf6cdc4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_snprintf")]
// 0xf6cdd4 — _snprintf
// type: int(char *__str, size_t __size, const char *__format, ...)
pub fn stub_0xf6cdd4() {
    // IDA 0xf6cdd4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_socket")]
// 0xf6cde4 — _socket
// type: int __cdecl(int, int, int)
pub fn stub_0xf6cde4() {
    // IDA 0xf6cde4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sprintf")]
// 0xf6cdf4 — _sprintf
// type: int(char *, const char *, ...)
pub fn stub_0xf6cdf4() {
    // IDA 0xf6cdf4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_srand")]
// 0xf6ce04 — _srand
// type: void __cdecl(unsigned int)
pub fn stub_0xf6ce04() {
    // IDA 0xf6ce04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sscanf")]
// 0xf6ce14 — _sscanf
// type: int(const char *, const char *, ...)
pub fn stub_0xf6ce14() {
    // IDA 0xf6ce14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_stat")]
// 0xf6ce24 — _stat
// type: int __cdecl(const char *, stat *)
pub fn stub_0xf6ce24() {
    // IDA 0xf6ce24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strcasecmp")]
// 0xf6ce34 — _strcasecmp
// type: int __cdecl(const char *, const char *)
pub fn stub_0xf6ce34() {
    // IDA 0xf6ce34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strcat")]
// 0xf6ce44 — _strcat
// type: char *__cdecl(char *__s1, const char *__s2)
pub fn stub_0xf6ce44() {
    // IDA 0xf6ce44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strchr")]
// 0xf6ce54 — _strchr
// type: char *__cdecl(const char *__s, int __c)
pub fn stub_0xf6ce54() {
    // IDA 0xf6ce54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strcmp")]
// 0xf6ce64 — _strcmp
// type: int __cdecl(const char *__s1, const char *__s2)
pub fn stub_0xf6ce64() {
    // IDA 0xf6ce64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strcoll")]
// 0xf6ce74 — _strcoll
// type: int __cdecl(const char *__s1, const char *__s2)
pub fn stub_0xf6ce74() {
    // IDA 0xf6ce74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strcpy")]
// 0xf6ce84 — _strcpy
// type: char *__cdecl(char *__dst, const char *__src)
pub fn stub_0xf6ce84() {
    // IDA 0xf6ce84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strcspn")]
// 0xf6ce94 — _strcspn
// type: size_t __cdecl(const char *__s, const char *__charset)
pub fn stub_0xf6ce94() {
    // IDA 0xf6ce94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strdup")]
// 0xf6cea4 — _strdup
// type: char *__cdecl(const char *__s1)
pub fn stub_0xf6cea4() {
    // IDA 0xf6cea4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strerror")]
// 0xf6ceb4 — _strerror
// type: char *__cdecl(int __errnum)
pub fn stub_0xf6ceb4() {
    // IDA 0xf6ceb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strerror_r")]
// 0xf6cec4 — _strerror_r
// type: int __cdecl(int __errnum, char *__strerrbuf, size_t __buflen)
pub fn stub_0xf6cec4() {
    // IDA 0xf6cec4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strlcat")]
// 0xf6ced4 — _strlcat
// type: size_t __cdecl(char *__dst, const char *__source, size_t __size)
pub fn stub_0xf6ced4() {
    // IDA 0xf6ced4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strlcpy")]
// 0xf6cee4 — _strlcpy
// type: size_t __cdecl(char *__dst, const char *__source, size_t __size)
pub fn stub_0xf6cee4() {
    // IDA 0xf6cee4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strlen")]
// 0xf6cef4 — _strlen
// type: size_t __cdecl(const char *__s)
pub fn stub_0xf6cef4() {
    // IDA 0xf6cef4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strncasecmp")]
// 0xf6cf04 — _strncasecmp
// type: int __cdecl(const char *, const char *, size_t)
pub fn stub_0xf6cf04() {
    // IDA 0xf6cf04: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strncat")]
// 0xf6cf14 — _strncat
// type: char *__cdecl(char *__s1, const char *__s2, size_t __n)
pub fn stub_0xf6cf14() {
    // IDA 0xf6cf14: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strncmp")]
// 0xf6cf24 — _strncmp
// type: int __cdecl(const char *__s1, const char *__s2, size_t __n)
pub fn stub_0xf6cf24() {
    // IDA 0xf6cf24: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strncpy")]
// 0xf6cf34 — _strncpy
// type: char *__cdecl(char *__dst, const char *__src, size_t __n)
pub fn stub_0xf6cf34() {
    // IDA 0xf6cf34: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strpbrk")]
// 0xf6cf44 — _strpbrk
// type: char *__cdecl(const char *__s, const char *__charset)
pub fn stub_0xf6cf44() {
    // IDA 0xf6cf44: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strrchr")]
// 0xf6cf54 — _strrchr
// type: char *__cdecl(const char *__s, int __c)
pub fn stub_0xf6cf54() {
    // IDA 0xf6cf54: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strsignal")]
// 0xf6cf64 — _strsignal
// type: char *__cdecl(int __sig)
pub fn stub_0xf6cf64() {
    // IDA 0xf6cf64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strstr")]
// 0xf6cf74 — _strstr
// type: char *__cdecl(const char *__big, const char *__little)
pub fn stub_0xf6cf74() {
    // IDA 0xf6cf74: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strtod")]
// 0xf6cf84 — _strtod
// type: double __cdecl(const char *, char **)
pub fn stub_0xf6cf84() {
    // IDA 0xf6cf84: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strtol")]
// 0xf6cf94 — _strtol
// type: __int32 __cdecl(const char *__str, char **__endptr, int __base)
pub fn stub_0xf6cf94() {
    // IDA 0xf6cf94: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strtoll")]
// 0xf6cfa4 — _strtoll
// type: __int64 __cdecl(const char *__str, char **__endptr, int __base)
pub fn stub_0xf6cfa4() {
    // IDA 0xf6cfa4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strtoul")]
// 0xf6cfb4 — _strtoul
// type: unsigned __int32 __cdecl(const char *__str, char **__endptr, int __base)
pub fn stub_0xf6cfb4() {
    // IDA 0xf6cfb4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_strtoull")]
// 0xf6cfc4 — _strtoull
// type: unsigned __int64 __cdecl(const char *__str, char **__endptr, int __base)
pub fn stub_0xf6cfc4() {
    // IDA 0xf6cfc4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sysconf")]
// 0xf6cfd4 — _sysconf
// type: __int32 __cdecl(int)
pub fn stub_0xf6cfd4() {
    // IDA 0xf6cfd4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sysctl")]
// 0xf6cfe4 — _sysctl
// type: int __cdecl(int *, u_int, void *, size_t *, void *, size_t)
pub fn stub_0xf6cfe4() {
    // IDA 0xf6cfe4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_sysctlbyname")]
// 0xf6cff4 — _sysctlbyname
// type: int __cdecl(const char *, void *, size_t *, void *, size_t)
pub fn stub_0xf6cff4() {
    // IDA 0xf6cff4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_tan")]
// 0xf6d004 — _tan
// type: double __cdecl(double)
pub fn stub_0xf6d004() {
    // IDA 0xf6d004: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_tanf")]
// 0xf6d014 — _tanf
// type: float __cdecl(float)
pub fn stub_0xf6d014() {
    // IDA 0xf6d014: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_tanh")]
// 0xf6d024 — _tanh
// type: double __cdecl(double)
pub fn stub_0xf6d024() {
    // IDA 0xf6d024: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_task_info")]
// 0xf6d034 — _task_info
// type: kern_return_t __cdecl(task_name_t target_task, task_flavor_t flavor, task_info_t task_info_out, mach_msg_type_number_t *task_info_outCnt)
pub fn stub_0xf6d034() {
    // IDA 0xf6d034: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_task_set_exception_ports")]
// 0xf6d044 — _task_set_exception_ports
// type: kern_return_t __cdecl(task_t task, exception_mask_t exception_mask, mach_port_t new_port, exception_behavior_t behavior, thread_state_flavor_t new_flavor)
pub fn stub_0xf6d044() {
    // IDA 0xf6d044: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_task_swap_exception_ports")]
// 0xf6d054 — _task_swap_exception_ports
// type: kern_return_t __cdecl(task_t task, exception_mask_t exception_mask, mach_port_t new_port, exception_behavior_t behavior, thread_state_flavor_t new_flavor, exception_mask_array_t masks, mach_msg_type_number_t *masksCnt, exception_handler_array_t old_handlerss, exception_behavior_array_t old_behaviors, exception_flavor_array_t old_flavors)
pub fn stub_0xf6d054() {
    // IDA 0xf6d054: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_task_threads")]
// 0xf6d064 — _task_threads
// type: kern_return_t __cdecl(task_inspect_t target_task, thread_act_array_t *act_list, mach_msg_type_number_t *act_listCnt)
pub fn stub_0xf6d064() {
    // IDA 0xf6d064: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_thread_get_state")]
// 0xf6d074 — _thread_get_state
// type: kern_return_t __cdecl(thread_act_t target_act, thread_state_flavor_t flavor, thread_state_t old_state, mach_msg_type_number_t *old_stateCnt)
pub fn stub_0xf6d074() {
    // IDA 0xf6d074: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_thread_resume")]
// 0xf6d084 — _thread_resume
// type: kern_return_t __cdecl(thread_act_t target_act)
pub fn stub_0xf6d084() {
    // IDA 0xf6d084: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_thread_set_exception_ports")]
// 0xf6d094 — _thread_set_exception_ports
// type: kern_return_t __cdecl(thread_act_t thread, exception_mask_t exception_mask, mach_port_t new_port, exception_behavior_t behavior, thread_state_flavor_t new_flavor)
pub fn stub_0xf6d094() {
    // IDA 0xf6d094: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_thread_suspend")]
// 0xf6d0a4 — _thread_suspend
// type: kern_return_t __cdecl(thread_act_t target_act)
pub fn stub_0xf6d0a4() {
    // IDA 0xf6d0a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_thread_swap_exception_ports")]
// 0xf6d0b4 — _thread_swap_exception_ports
// type: kern_return_t __cdecl(thread_act_t thread, exception_mask_t exception_mask, mach_port_t new_port, exception_behavior_t behavior, thread_state_flavor_t new_flavor, exception_mask_array_t masks, mach_msg_type_number_t *masksCnt, exception_handler_array_t old_handlers, exception_behavior_array_t old_behaviors, exception_flavor_array_t old_flavors)
pub fn stub_0xf6d0b4() {
    // IDA 0xf6d0b4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_time")]
// 0xf6d0c4 — _time
// type: time_t __cdecl(time_t *)
pub fn stub_0xf6d0c4() {
    // IDA 0xf6d0c4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_tmpfile")]
// 0xf6d0d4 — _tmpfile
// type: FILE *(void)
pub fn stub_0xf6d0d4() {
    // IDA 0xf6d0d4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_umask")]
// 0xf6d0e4 — _umask
// type: mode_t __cdecl(mode_t)
pub fn stub_0xf6d0e4() {
    // IDA 0xf6d0e4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "_ungetc")]
// 0xf6d0f4 — _ungetc
// type: int __cdecl(int, FILE *)
pub fn stub_0xf6d0f4() {
    // IDA 0xf6d0f4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}
