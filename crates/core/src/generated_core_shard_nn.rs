//! core shard nn — 100 core stubs EA-sorted asc fallback not yet in rbx_core.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; fallback 32196, 819 uncovered before -> 719 after, batch 0xf6be24..0xf6c474).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "___modsi3")]
// 0xf6be24 — ___modsi3
pub fn stub_0xf6be24() -> ! { todo!("0xf6be24 ___modsi3") }

#[doc(alias = "___snprintf_chk")]
// 0xf6be34 — ___snprintf_chk
// type: int(char *, size_t, int, size_t, const char *, ...)
pub fn stub_0xf6be34() -> ! { todo!("0xf6be34 ___snprintf_chk") }

#[doc(alias = "___sprintf_chk")]
// 0xf6be44 — ___sprintf_chk
// type: int(char *, int, size_t, const char *, ...)
pub fn stub_0xf6be44() -> ! { todo!("0xf6be44 ___sprintf_chk") }

#[doc(alias = "___stack_chk_fail")]
// 0xf6be54 — ___stack_chk_fail
pub fn stub_0xf6be54() -> ! { todo!("0xf6be54 ___stack_chk_fail") }

#[doc(alias = "___strcat_chk")]
// 0xf6be64 — ___strcat_chk
pub fn stub_0xf6be64() -> ! { todo!("0xf6be64 ___strcat_chk") }

#[doc(alias = "___strcpy_chk")]
// 0xf6be74 — ___strcpy_chk
pub fn stub_0xf6be74() -> ! { todo!("0xf6be74 ___strcpy_chk") }

#[doc(alias = "___strncpy_chk")]
// 0xf6be84 — ___strncpy_chk
pub fn stub_0xf6be84() -> ! { todo!("0xf6be84 ___strncpy_chk") }

#[doc(alias = "___tolower")]
// 0xf6be94 — ___tolower
// type: __darwin_ct_rune_t __cdecl(__darwin_ct_rune_t)
pub fn stub_0xf6be94() -> ! { todo!("0xf6be94 ___tolower") }

#[doc(alias = "___toupper")]
// 0xf6bea4 — ___toupper
// type: __darwin_ct_rune_t __cdecl(__darwin_ct_rune_t)
pub fn stub_0xf6bea4() -> ! { todo!("0xf6bea4 ___toupper") }

#[doc(alias = "___udivdi3")]
// 0xf6beb4 — ___udivdi3
pub fn stub_0xf6beb4() -> ! { todo!("0xf6beb4 ___udivdi3") }

#[doc(alias = "___udivsi3")]
// 0xf6bec4 — ___udivsi3
// type: int __fastcall(unsigned int, unsigned int)
pub fn stub_0xf6bec4() -> ! { todo!("0xf6bec4 ___udivsi3") }

#[doc(alias = "___umoddi3")]
// 0xf6bed4 — ___umoddi3
pub fn stub_0xf6bed4() -> ! { todo!("0xf6bed4 ___umoddi3") }

#[doc(alias = "___umodsi3")]
// 0xf6bee4 — ___umodsi3
pub fn stub_0xf6bee4() -> ! { todo!("0xf6bee4 ___umodsi3") }

#[doc(alias = "___vsnprintf_chk")]
// 0xf6bef4 — ___vsnprintf_chk
// type: int __cdecl(char *, size_t, int, size_t, const char *, va_list)
pub fn stub_0xf6bef4() -> ! { todo!("0xf6bef4 ___vsnprintf_chk") }

#[doc(alias = "__dyld_get_image_header")]
// 0xf6bf04 — __dyld_get_image_header
// type: const mach_header *__cdecl(uint32_t image_index)
pub fn stub_0xf6bf04() -> ! { todo!("0xf6bf04 __dyld_get_image_header") }

#[doc(alias = "__dyld_get_image_name")]
// 0xf6bf14 — __dyld_get_image_name
// type: const char *__cdecl(uint32_t image_index)
pub fn stub_0xf6bf14() -> ! { todo!("0xf6bf14 __dyld_get_image_name") }

#[doc(alias = "__dyld_get_image_vmaddr_slide")]
// 0xf6bf24 — __dyld_get_image_vmaddr_slide
// type: intptr_t __cdecl(uint32_t image_index)
pub fn stub_0xf6bf24() -> ! { todo!("0xf6bf24 __dyld_get_image_vmaddr_slide") }

#[doc(alias = "__dyld_image_count")]
// 0xf6bf34 — __dyld_image_count
// type: uint32_t(void)
pub fn stub_0xf6bf34() -> ! { todo!("0xf6bf34 __dyld_image_count") }

#[doc(alias = "__dyld_register_func_for_add_image")]
// 0xf6bf44 — __dyld_register_func_for_add_image
// type: void __cdecl(void (__cdecl *func)(const mach_header *, intptr_t))
pub fn stub_0xf6bf44() -> ! { todo!("0xf6bf44 __dyld_register_func_for_add_image") }

#[doc(alias = "__dyld_register_func_for_remove_image")]
// 0xf6bf54 — __dyld_register_func_for_remove_image
// type: void __cdecl(void (__cdecl *func)(const mach_header *, intptr_t))
pub fn stub_0xf6bf54() -> ! { todo!("0xf6bf54 __dyld_register_func_for_remove_image") }

#[doc(alias = "_abort")]
// 0xf6bf64 — _abort
// type: void __noreturn(void)
pub fn stub_0xf6bf64() -> ! { todo!("0xf6bf64 _abort") }

#[doc(alias = "_accept")]
// 0xf6bf74 — _accept
// type: int __cdecl(int, sockaddr *, socklen_t *)
pub fn stub_0xf6bf74() -> ! { todo!("0xf6bf74 _accept") }

#[doc(alias = "_acos")]
// 0xf6bf84 — _acos
// type: double __cdecl(double)
pub fn stub_0xf6bf84() -> ! { todo!("0xf6bf84 _acos") }

#[doc(alias = "_acosf")]
// 0xf6bf94 — _acosf
// type: float __cdecl(float)
pub fn stub_0xf6bf94() -> ! { todo!("0xf6bf94 _acosf") }

#[doc(alias = "_arc4random")]
// 0xf6bfa4 — _arc4random
// type: uint32_t(void)
pub fn stub_0xf6bfa4() -> ! { todo!("0xf6bfa4 _arc4random") }

#[doc(alias = "_asin")]
// 0xf6bfb4 — _asin
// type: double __cdecl(double)
pub fn stub_0xf6bfb4() -> ! { todo!("0xf6bfb4 _asin") }

#[doc(alias = "_asinf")]
// 0xf6bfc4 — _asinf
// type: float __cdecl(float)
pub fn stub_0xf6bfc4() -> ! { todo!("0xf6bfc4 _asinf") }

#[doc(alias = "_asl_get")]
// 0xf6bfd4 — _asl_get
// type: const char *__cdecl(asl_object_t msg, const char *key)
pub fn stub_0xf6bfd4() -> ! { todo!("0xf6bfd4 _asl_get") }

#[doc(alias = "_asl_key")]
// 0xf6bfe4 — _asl_key
// type: const char *__cdecl(asl_object_t msg, uint32_t n)
pub fn stub_0xf6bfe4() -> ! { todo!("0xf6bfe4 _asl_key") }

#[doc(alias = "_asl_log")]
// 0xf6bff4 — _asl_log
// type: int(asl_object_t client, asl_object_t msg, int level, const char *format, ...)
pub fn stub_0xf6bff4() -> ! { todo!("0xf6bff4 _asl_log") }

#[doc(alias = "_asl_new")]
// 0xf6c004 — _asl_new
// type: asl_object_t __cdecl(uint32_t type)
pub fn stub_0xf6c004() -> ! { todo!("0xf6c004 _asl_new") }

#[doc(alias = "_asl_open")]
// 0xf6c014 — _asl_open
// type: asl_object_t __cdecl(const char *ident, const char *facility, uint32_t opts)
pub fn stub_0xf6c014() -> ! { todo!("0xf6c014 _asl_open") }

#[doc(alias = "_asl_search")]
// 0xf6c024 — _asl_search
// type: asl_object_t __cdecl(asl_object_t obj, asl_object_t query)
pub fn stub_0xf6c024() -> ! { todo!("0xf6c024 _asl_search") }

#[doc(alias = "_asl_set_query")]
// 0xf6c034 — _asl_set_query
// type: int __cdecl(asl_object_t msg, const char *key, const char *value, uint32_t op)
pub fn stub_0xf6c034() -> ! { todo!("0xf6c034 _asl_set_query") }

#[doc(alias = "_aslresponse_free")]
// 0xf6c044 — _aslresponse_free
// type: void __cdecl(asl_object_t obj)
pub fn stub_0xf6c044() -> ! { todo!("0xf6c044 _aslresponse_free") }

#[doc(alias = "_aslresponse_next")]
// 0xf6c054 — _aslresponse_next
// type: asl_object_t __cdecl(asl_object_t obj)
pub fn stub_0xf6c054() -> ! { todo!("0xf6c054 _aslresponse_next") }

#[doc(alias = "_atan")]
// 0xf6c064 — _atan
// type: double __cdecl(double)
pub fn stub_0xf6c064() -> ! { todo!("0xf6c064 _atan") }

#[doc(alias = "_atan2")]
// 0xf6c074 — _atan2
// type: double __cdecl(double, double)
pub fn stub_0xf6c074() -> ! { todo!("0xf6c074 _atan2") }

#[doc(alias = "_atof")]
// 0xf6c084 — _atof
// type: double __cdecl(const char *)
pub fn stub_0xf6c084() -> ! { todo!("0xf6c084 _atof") }

#[doc(alias = "_atoi")]
// 0xf6c094 — _atoi
// type: int __cdecl(const char *)
pub fn stub_0xf6c094() -> ! { todo!("0xf6c094 _atoi") }

#[doc(alias = "_atol")]
// 0xf6c0a4 — _atol
// type: __int32 __cdecl(const char *)
pub fn stub_0xf6c0a4() -> ! { todo!("0xf6c0a4 _atol") }

#[doc(alias = "_bind")]
// 0xf6c0b4 — _bind
// type: int __cdecl(int, const sockaddr *, socklen_t)
pub fn stub_0xf6c0b4() -> ! { todo!("0xf6c0b4 _bind") }

#[doc(alias = "_bsearch")]
// 0xf6c0c4 — _bsearch
// type: void *__cdecl(const void *__key, const void *__base, size_t __nel, size_t __width, int (__cdecl *__compar)(const void *, const void *))
pub fn stub_0xf6c0c4() -> ! { todo!("0xf6c0c4 _bsearch") }

#[doc(alias = "_cabsf")]
// 0xf6c0d4 — _cabsf
// type: float __cdecl(__complex_float)
pub fn stub_0xf6c0d4() -> ! { todo!("0xf6c0d4 _cabsf") }

#[doc(alias = "_calloc")]
// 0xf6c0e4 — _calloc
// type: void *__cdecl(size_t __count, size_t __size)
pub fn stub_0xf6c0e4() -> ! { todo!("0xf6c0e4 _calloc") }

#[doc(alias = "_ceil")]
// 0xf6c0f4 — _ceil
// type: double __cdecl(double)
pub fn stub_0xf6c0f4() -> ! { todo!("0xf6c0f4 _ceil") }

#[doc(alias = "_ceilf")]
// 0xf6c104 — _ceilf
// type: float __cdecl(float)
pub fn stub_0xf6c104() -> ! { todo!("0xf6c104 _ceilf") }

#[doc(alias = "_chmod")]
// 0xf6c114 — _chmod
// type: int __cdecl(const char *, mode_t)
pub fn stub_0xf6c114() -> ! { todo!("0xf6c114 _chmod") }

#[doc(alias = "_clearerr")]
// 0xf6c124 — _clearerr
// type: void __cdecl(FILE *)
pub fn stub_0xf6c124() -> ! { todo!("0xf6c124 _clearerr") }

#[doc(alias = "_clock")]
// 0xf6c134 — _clock
// type: clock_t(void)
pub fn stub_0xf6c134() -> ! { todo!("0xf6c134 _clock") }

#[doc(alias = "_close")]
// 0xf6c144 — _close
// type: int __cdecl(int)
pub fn stub_0xf6c144() -> ! { todo!("0xf6c144 _close") }

#[doc(alias = "_closedir")]
// 0xf6c154 — _closedir
// type: int __cdecl(DIR *)
pub fn stub_0xf6c154() -> ! { todo!("0xf6c154 _closedir") }

#[doc(alias = "_connect")]
// 0xf6c164 — _connect
// type: int __cdecl(int, const sockaddr *, socklen_t)
pub fn stub_0xf6c164() -> ! { todo!("0xf6c164 _connect") }

#[doc(alias = "_cos")]
// 0xf6c174 — _cos
// type: double __cdecl(double)
pub fn stub_0xf6c174() -> ! { todo!("0xf6c174 _cos") }

#[doc(alias = "_cosf")]
// 0xf6c184 — _cosf
// type: float __cdecl(float)
pub fn stub_0xf6c184() -> ! { todo!("0xf6c184 _cosf") }

#[doc(alias = "_cosh")]
// 0xf6c194 — _cosh
// type: double __cdecl(double)
pub fn stub_0xf6c194() -> ! { todo!("0xf6c194 _cosh") }

#[doc(alias = "_ctime")]
// 0xf6c1a4 — _ctime
// type: char *__cdecl(const time_t *)
pub fn stub_0xf6c1a4() -> ! { todo!("0xf6c1a4 _ctime") }

#[doc(alias = "_dispatch_after")]
// 0xf6c1b4 — _dispatch_after
// type: void __cdecl(dispatch_time_t when, dispatch_queue_t queue, dispatch_block_t block)
pub fn stub_0xf6c1b4() -> ! { todo!("0xf6c1b4 _dispatch_after") }

#[doc(alias = "_dispatch_async")]
// 0xf6c1c4 — _dispatch_async
// type: void __cdecl(dispatch_queue_t queue, dispatch_block_t block)
pub fn stub_0xf6c1c4() -> ! { todo!("0xf6c1c4 _dispatch_async") }

#[doc(alias = "_dispatch_get_current_queue")]
// 0xf6c1d4 — _dispatch_get_current_queue
// type: dispatch_queue_t(void)
pub fn stub_0xf6c1d4() -> ! { todo!("0xf6c1d4 _dispatch_get_current_queue") }

#[doc(alias = "_dispatch_get_global_queue")]
// 0xf6c1e4 — _dispatch_get_global_queue
// type: dispatch_queue_global_t __cdecl(__int32 identifier, unsigned __int32 flags)
pub fn stub_0xf6c1e4() -> ! { todo!("0xf6c1e4 _dispatch_get_global_queue") }

#[doc(alias = "_dispatch_once")]
// 0xf6c1f4 — _dispatch_once
// type: void __cdecl(dispatch_once_t *predicate, dispatch_block_t block)
pub fn stub_0xf6c1f4() -> ! { todo!("0xf6c1f4 _dispatch_once") }

#[doc(alias = "_dispatch_queue_create")]
// 0xf6c204 — _dispatch_queue_create
// type: dispatch_queue_t __cdecl(const char *label, dispatch_queue_attr_t attr)
pub fn stub_0xf6c204() -> ! { todo!("0xf6c204 _dispatch_queue_create") }

#[doc(alias = "_dispatch_release")]
// 0xf6c214 — _dispatch_release
// type: void __cdecl(dispatch_object_t object)
pub fn stub_0xf6c214() -> ! { todo!("0xf6c214 _dispatch_release") }

#[doc(alias = "_dispatch_resume")]
// 0xf6c224 — _dispatch_resume
// type: void __cdecl(dispatch_object_t object)
pub fn stub_0xf6c224() -> ! { todo!("0xf6c224 _dispatch_resume") }

#[doc(alias = "_dispatch_retain")]
// 0xf6c234 — _dispatch_retain
// type: void __cdecl(dispatch_object_t object)
pub fn stub_0xf6c234() -> ! { todo!("0xf6c234 _dispatch_retain") }

#[doc(alias = "_dispatch_source_cancel")]
// 0xf6c244 — _dispatch_source_cancel
// type: void __cdecl(dispatch_source_t source)
pub fn stub_0xf6c244() -> ! { todo!("0xf6c244 _dispatch_source_cancel") }

#[doc(alias = "_dispatch_source_create")]
// 0xf6c254 — _dispatch_source_create
// type: dispatch_source_t __cdecl(dispatch_source_type_t type, uintptr_t handle, unsigned __int32 mask, dispatch_queue_t queue)
pub fn stub_0xf6c254() -> ! { todo!("0xf6c254 _dispatch_source_create") }

#[doc(alias = "_dispatch_source_set_event_handler")]
// 0xf6c264 — _dispatch_source_set_event_handler
// type: void __cdecl(dispatch_source_t source, dispatch_block_t handler)
pub fn stub_0xf6c264() -> ! { todo!("0xf6c264 _dispatch_source_set_event_handler") }

#[doc(alias = "_dispatch_source_set_timer")]
// 0xf6c274 — _dispatch_source_set_timer
// type: void __cdecl(dispatch_source_t source, dispatch_time_t start, uint64_t interval, uint64_t leeway)
pub fn stub_0xf6c274() -> ! { todo!("0xf6c274 _dispatch_source_set_timer") }

#[doc(alias = "_dispatch_sync")]
// 0xf6c284 — _dispatch_sync
// type: void __cdecl(dispatch_queue_t queue, dispatch_block_t block)
pub fn stub_0xf6c284() -> ! { todo!("0xf6c284 _dispatch_sync") }

#[doc(alias = "_dispatch_time")]
// 0xf6c294 — _dispatch_time
// type: dispatch_time_t __cdecl(dispatch_time_t when, int64_t delta)
pub fn stub_0xf6c294() -> ! { todo!("0xf6c294 _dispatch_time") }

#[doc(alias = "_div")]
// 0xf6c2a4 — _div
// type: div_t *__cdecl(div_t *__return_ptr __struct_ptr retstr, int, int)
pub fn stub_0xf6c2a4() -> ! { todo!("0xf6c2a4 _div") }

#[doc(alias = "_dladdr")]
// 0xf6c2b4 — _dladdr
// type: int __cdecl(const void *, Dl_info *)
pub fn stub_0xf6c2b4() -> ! { todo!("0xf6c2b4 _dladdr") }

#[doc(alias = "_dlopen")]
// 0xf6c2c4 — _dlopen
// type: void *__cdecl(const char *__path, int __mode)
pub fn stub_0xf6c2c4() -> ! { todo!("0xf6c2c4 _dlopen") }

#[doc(alias = "_dlsym")]
// 0xf6c2d4 — _dlsym
// type: void *__cdecl(void *__handle, const char *__symbol)
pub fn stub_0xf6c2d4() -> ! { todo!("0xf6c2d4 _dlsym") }

#[doc(alias = "_exit")]
// 0xf6c2e4 — _exit
// type: void __cdecl __noreturn(int)
pub fn stub_0xf6c2e4() -> ! { todo!("0xf6c2e4 _exit") }

#[doc(alias = "_exp")]
// 0xf6c2f4 — _exp
// type: double __cdecl(double)
pub fn stub_0xf6c2f4() -> ! { todo!("0xf6c2f4 _exp") }

#[doc(alias = "_exp2")]
// 0xf6c304 — _exp2
// type: double __cdecl(double)
pub fn stub_0xf6c304() -> ! { todo!("0xf6c304 _exp2") }

#[doc(alias = "_expf")]
// 0xf6c314 — _expf
// type: float __cdecl(float)
pub fn stub_0xf6c314() -> ! { todo!("0xf6c314 _expf") }

#[doc(alias = "_fclose")]
// 0xf6c324 — _fclose
// type: int __cdecl(FILE *)
pub fn stub_0xf6c324() -> ! { todo!("0xf6c324 _fclose") }

#[doc(alias = "_fcntl")]
// 0xf6c334 — _fcntl
// type: int(int, int, ...)
pub fn stub_0xf6c334() -> ! { todo!("0xf6c334 _fcntl") }

#[doc(alias = "_fdopen")]
// 0xf6c344 — _fdopen
// type: FILE *__cdecl(int, const char *)
pub fn stub_0xf6c344() -> ! { todo!("0xf6c344 _fdopen") }

#[doc(alias = "_feof")]
// 0xf6c354 — _feof
// type: int __cdecl(FILE *)
pub fn stub_0xf6c354() -> ! { todo!("0xf6c354 _feof") }

#[doc(alias = "_ferror")]
// 0xf6c364 — _ferror
// type: int __cdecl(FILE *)
pub fn stub_0xf6c364() -> ! { todo!("0xf6c364 _ferror") }

#[doc(alias = "_fflush")]
// 0xf6c374 — _fflush
// type: int __cdecl(FILE *)
pub fn stub_0xf6c374() -> ! { todo!("0xf6c374 _fflush") }

#[doc(alias = "_floor")]
// 0xf6c384 — _floor
// type: double __cdecl(double)
pub fn stub_0xf6c384() -> ! { todo!("0xf6c384 _floor") }

#[doc(alias = "_floorf")]
// 0xf6c394 — _floorf
// type: float __cdecl(float)
pub fn stub_0xf6c394() -> ! { todo!("0xf6c394 _floorf") }

#[doc(alias = "_fnmatch")]
// 0xf6c3c4 — _fnmatch
// type: int __cdecl(const char *, const char *, int)
pub fn stub_0xf6c3c4() -> ! { todo!("0xf6c3c4 _fnmatch") }

#[doc(alias = "_fopen")]
// 0xf6c3d4 — _fopen
// type: FILE *__cdecl(const char *__filename, const char *__mode)
pub fn stub_0xf6c3d4() -> ! { todo!("0xf6c3d4 _fopen") }

#[doc(alias = "_fprintf")]
// 0xf6c3e4 — _fprintf
// type: int(FILE *, const char *, ...)
pub fn stub_0xf6c3e4() -> ! { todo!("0xf6c3e4 _fprintf") }

#[doc(alias = "_fputc")]
// 0xf6c3f4 — _fputc
// type: int __cdecl(int, FILE *)
pub fn stub_0xf6c3f4() -> ! { todo!("0xf6c3f4 _fputc") }

#[doc(alias = "_fputs")]
// 0xf6c404 — _fputs
// type: int __cdecl(const char *, FILE *)
pub fn stub_0xf6c404() -> ! { todo!("0xf6c404 _fputs") }

#[doc(alias = "_fread")]
// 0xf6c414 — _fread
// type: size_t __cdecl(void *__ptr, size_t __size, size_t __nitems, FILE *__stream)
pub fn stub_0xf6c414() -> ! { todo!("0xf6c414 _fread") }

#[doc(alias = "_free")]
// 0xf6c424 — _free
// type: void __cdecl(void *)
pub fn stub_0xf6c424() -> ! { todo!("0xf6c424 _free") }

#[doc(alias = "_freeaddrinfo")]
// 0xf6c434 — _freeaddrinfo
// type: void __cdecl(addrinfo *)
pub fn stub_0xf6c434() -> ! { todo!("0xf6c434 _freeaddrinfo") }

#[doc(alias = "_freeifaddrs")]
// 0xf6c444 — _freeifaddrs
// type: void __cdecl(ifaddrs *)
pub fn stub_0xf6c444() -> ! { todo!("0xf6c444 _freeifaddrs") }

#[doc(alias = "_freopen")]
// 0xf6c454 — _freopen
// type: FILE *__cdecl(const char *, const char *, FILE *)
pub fn stub_0xf6c454() -> ! { todo!("0xf6c454 _freopen") }

#[doc(alias = "_frexp")]
// 0xf6c464 — _frexp
// type: double __cdecl(double, int *)
pub fn stub_0xf6c464() -> ! { todo!("0xf6c464 _frexp") }

#[doc(alias = "_fseek")]
// 0xf6c474 — _fseek
// type: int __cdecl(FILE *, __int32, int)
pub fn stub_0xf6c474() -> ! { todo!("0xf6c474 _fseek") }
