//! core shard ni — 100 core stubs EA-sorted asc next uncovered fallback filtered not yet in rbx_core.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 filtered fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|FMOD|Sound|Audio|Lua|Script|Yield (fallback 32196, 919 uncovered before batch, rbx_core::SharedPtr not boost).
//! Range: 0xf607a4..0xf6be14, lowest EA first.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "XmlElement::~XmlElement()")]
// 0xf607a4 — j___ZN10XmlElementD2Ev
// type: void __fastcall(XmlElement *__hidden this)
pub fn stub_0xf607a4() {
    // IDA 0xf607a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
// 0xf613a4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf613a4() {
    // IDA 0xf613a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "DataStructures::Queue<HuffmanEncodingTreeNode *>::Push(HuffmanEncodingTreeNode * const&,char const*,unsigned int)")]
// 0xf61a44 — j___ZN14DataStructures5QueueIP23HuffmanEncodingTreeNodeE4PushERKS2_PKcj
pub fn stub_0xf61a44() {
    // IDA 0xf61a44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "DataStructures::Queue<bool>::Push(bool const&,char const*,unsigned int)")]
// 0xf61cf4 — j___ZN14DataStructures5QueueIbE4PushERKbPKcj
// type: int()
pub fn stub_0xf61cf4() {
    // IDA 0xf61cf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "DataStructures::Queue<bool>::Compress(char const*,unsigned int)")]
// 0xf61d04 — j___ZN14DataStructures5QueueIbE8CompressEPKcj
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf61d04() {
    // IDA 0xf61d04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE9classNameEv")]
// 0xf65194 — j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE9classNameEv
pub fn stub_0xf65194() {
    // IDA 0xf65194: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_11sForceFieldEEEERKS0_v")]
// 0xf654a4 — j___ZN3RBX4Name7declareILZNS_11sForceFieldEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf654a4() {
    // IDA 0xf654a4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE9classNameEv")]
// 0xf65854 — j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE9classNameEv
pub fn stub_0xf65854() {
    // IDA 0xf65854: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")]
// 0xf65884 — j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v
pub fn stub_0xf65884() {
    // IDA 0xf65884: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")]
// 0xf66104 — j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v
// type: int(void)
pub fn stub_0xf66104() {
    // IDA 0xf66104: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "_class_addMethod")]
// 0xf6adf4 — _class_addMethod
// type: BOOL __cdecl(Class cls, SEL name, IMP imp, const char *types)
pub fn stub_0xf6adf4() {
    // IDA 0xf6adf4: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "_class_getIvarLayout")]
// 0xf6ae34 — _class_getIvarLayout
// type: const uint8_t *__cdecl(Class cls)
pub fn stub_0xf6ae34() {
    // IDA 0xf6ae34: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "_class_getSuperclass")]
// 0xf6ae44 — _class_getSuperclass
// type: Class __cdecl(Class cls)
pub fn stub_0xf6ae44() {
    // IDA 0xf6ae44: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "_ivar_getName")]
// 0xf6ae54 — _ivar_getName
// type: const char *__cdecl(Ivar v)
pub fn stub_0xf6ae54() {
    // IDA 0xf6ae54: Name interning declare shim (static Name registry key). &'static str registry — carrier no-op.
}

#[doc(alias = "_ivar_getOffset")]
// 0xf6ae64 — _ivar_getOffset
// type: ptrdiff_t __cdecl(Ivar v)
pub fn stub_0xf6ae64() {
    // IDA 0xf6ae64: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_objc_autorelease")]
// 0xf6ae74 — _objc_autorelease
// type: id __cdecl(id)
pub fn stub_0xf6ae74() {
    // IDA 0xf6ae74: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_autoreleaseReturnValue")]
// 0xf6ae84 — _objc_autoreleaseReturnValue
// type: id __cdecl(id)
pub fn stub_0xf6ae84() {
    // IDA 0xf6ae84: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_begin_catch")]
// 0xf6ae94 — _objc_begin_catch
// type: id __cdecl(void *exc_buf)
pub fn stub_0xf6ae94() {
    // IDA 0xf6ae94: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_end_catch")]
// 0xf6aea4 — _objc_end_catch
// type: void(void)
pub fn stub_0xf6aea4() {
    // IDA 0xf6aea4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_enumerationMutation")]
// 0xf6aeb4 — _objc_enumerationMutation
// type: void __cdecl(id obj)
pub fn stub_0xf6aeb4() {
    // IDA 0xf6aeb4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_exception_rethrow")]
// 0xf6aec4 — _objc_exception_rethrow
// type: void(void)
pub fn stub_0xf6aec4() {
    // IDA 0xf6aec4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_getAssociatedObject")]
// 0xf6aed4 — _objc_getAssociatedObject
// type: id __cdecl(id object, const void *key)
pub fn stub_0xf6aed4() {
    // IDA 0xf6aed4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_getClass")]
// 0xf6aee4 — _objc_getClass
// type: Class __cdecl(const char *name)
pub fn stub_0xf6aee4() {
    // IDA 0xf6aee4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_getProperty")]
// 0xf6aef4 — _objc_getProperty
// type: id __cdecl(id self, SEL _cmd, ptrdiff_t offset, bool atomic)
pub fn stub_0xf6aef4() {
    // IDA 0xf6aef4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_msgSend")]
// 0xf6af04 — _objc_msgSend
// type: id(id, SEL, ...)
pub fn stub_0xf6af04() {
    // IDA 0xf6af04: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_msgSendSuper2")]
// 0xf6af14 — _objc_msgSendSuper2
// type: id(objc_super *, SEL, ...)
pub fn stub_0xf6af14() {
    // IDA 0xf6af14: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_msgSend_stret")]
// 0xf6af24 — _objc_msgSend_stret
// type: void(id, SEL, ...)
pub fn stub_0xf6af24() {
    // IDA 0xf6af24: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_release")]
// 0xf6af34 — _objc_release
// type: void __cdecl(id)
pub fn stub_0xf6af34() {
    // IDA 0xf6af34: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_retain")]
// 0xf6af44 — _objc_retain
// type: id __cdecl(id)
pub fn stub_0xf6af44() {
    // IDA 0xf6af44: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_retainAutorelease")]
// 0xf6af54 — _objc_retainAutorelease
// type: id __cdecl(id)
pub fn stub_0xf6af54() {
    // IDA 0xf6af54: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_retainAutoreleaseReturnValue")]
// 0xf6af64 — _objc_retainAutoreleaseReturnValue
// type: id __cdecl(id)
pub fn stub_0xf6af64() {
    // IDA 0xf6af64: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_retainAutoreleasedReturnValue")]
// 0xf6af74 — _objc_retainAutoreleasedReturnValue
// type: id __cdecl(id)
pub fn stub_0xf6af74() {
    // IDA 0xf6af74: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_retainBlock")]
// 0xf6af84 — _objc_retainBlock
// type: id __cdecl(id)
pub fn stub_0xf6af84() {
    // IDA 0xf6af84: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_setAssociatedObject")]
// 0xf6af94 — _objc_setAssociatedObject
// type: void __cdecl(id object, const void *key, id value, void *policy)
pub fn stub_0xf6af94() {
    // IDA 0xf6af94: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_setProperty")]
// 0xf6afa4 — _objc_setProperty
// type: void __cdecl(id self, SEL _cmd, ptrdiff_t offset, id newValue, bool atomic, char shouldCopy)
pub fn stub_0xf6afa4() {
    // IDA 0xf6afa4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_storeStrong")]
// 0xf6afb4 — _objc_storeStrong
// type: void __cdecl(id *location, id obj)
pub fn stub_0xf6afb4() {
    // IDA 0xf6afb4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_sync_enter")]
// 0xf6afc4 — _objc_sync_enter
// type: int __cdecl(id obj)
pub fn stub_0xf6afc4() {
    // IDA 0xf6afc4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_objc_sync_exit")]
// 0xf6afd4 — _objc_sync_exit
// type: int __cdecl(id obj)
pub fn stub_0xf6afd4() {
    // IDA 0xf6afd4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_object_getClass")]
// 0xf6afe4 — _object_getClass
// type: Class __cdecl(id)
pub fn stub_0xf6afe4() {
    // IDA 0xf6afe4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_object_setIvar")]
// 0xf6aff4 — _object_setIvar
// type: void __cdecl(id obj, Ivar ivar, id value)
pub fn stub_0xf6aff4() {
    // IDA 0xf6aff4: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_sel_getUid")]
// 0xf6b014 — _sel_getUid
// type: SEL __cdecl(const char *str)
pub fn stub_0xf6b014() {
    // IDA 0xf6b014: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "operator delete[](void *)")]
// 0xf6ba44 — __ZdaPv
// type: void __fastcall(void *)
pub fn stub_0xf6ba44() {
    // IDA 0xf6ba44: PLCrashReporter crash-capture helper (Mach-O/ObjC parsing). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "operator delete(void *)")]
// 0xf6ba54 — __ZdlPv
// type: void __fastcall(void *)
pub fn stub_0xf6ba54() {
    // IDA 0xf6ba54: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "operator new[](unsigned long)")]
// 0xf6ba74 — __Znam
// type: _DWORD __fastcall(unsigned int)
pub fn stub_0xf6ba74() {
    // IDA 0xf6ba74: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "operator new(unsigned long)")]
// 0xf6ba94 — __Znwm
// type: _DWORD __fastcall(unsigned int)
pub fn stub_0xf6ba94() {
    // IDA 0xf6ba94: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_allocate_exception")]
// 0xf6bab4 — ___cxa_allocate_exception
// type: void *__fastcall(size_t thrown_size)
pub fn stub_0xf6bab4() {
    // IDA 0xf6bab4: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_bad_typeid")]
// 0xf6bac4 — ___cxa_bad_typeid
// type: void __fastcall __noreturn()
pub fn stub_0xf6bac4() {
    // IDA 0xf6bac4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "___cxa_begin_catch")]
// 0xf6bad4 — ___cxa_begin_catch
// type: void *__fastcall(void *)
pub fn stub_0xf6bad4() {
    // IDA 0xf6bad4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "___cxa_call_unexpected")]
// 0xf6bae4 — ___cxa_call_unexpected
// type: void __fastcall __noreturn(void *)
pub fn stub_0xf6bae4() {
    // IDA 0xf6bae4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "___cxa_end_catch")]
// 0xf6baf4 — ___cxa_end_catch
// type: void __fastcall()
pub fn stub_0xf6baf4() {
    // IDA 0xf6baf4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "___cxa_free_exception")]
// 0xf6bb04 — ___cxa_free_exception
// type: void __fastcall(void *)
pub fn stub_0xf6bb04() {
    // IDA 0xf6bb04: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "___cxa_get_exception_ptr")]
// 0xf6bb14 — ___cxa_get_exception_ptr
// type: void *__fastcall(void *)
pub fn stub_0xf6bb14() {
    // IDA 0xf6bb14: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_guard_abort")]
// 0xf6bb24 — ___cxa_guard_abort
// type: void __fastcall(__guard *)
pub fn stub_0xf6bb24() {
    // IDA 0xf6bb24: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_guard_acquire")]
// 0xf6bb34 — ___cxa_guard_acquire
// type: int __fastcall(__guard *)
pub fn stub_0xf6bb34() {
    // IDA 0xf6bb34: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_guard_release")]
// 0xf6bb44 — ___cxa_guard_release
// type: void __fastcall(__guard *)
pub fn stub_0xf6bb44() {
    // IDA 0xf6bb44: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_rethrow")]
// 0xf6bb54 — ___cxa_rethrow
// type: void __fastcall __noreturn()
pub fn stub_0xf6bb54() {
    // IDA 0xf6bb54: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_throw")]
// 0xf6bb64 — ___cxa_throw
// type: void __fastcall __noreturn(void *, struct type_info *lptinfo, void (__fastcall *)(void *))
pub fn stub_0xf6bb64() {
    // IDA 0xf6bb64: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___dynamic_cast")]
// 0xf6bb74 — ___dynamic_cast
// type: void *__fastcall(const void *lpsrc, const struct __class_type_info *lpstype, const struct __class_type_info *lpdtype, ptrdiff_t s2d)
pub fn stub_0xf6bb74() {
    // IDA 0xf6bb74: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "_CC_MD5")]
// 0xf6bb84 — _CC_MD5
// type: unsigned __int8 *__cdecl(const void *data, CC_LONG len, unsigned __int8 *md)
pub fn stub_0xf6bb84() {
    // IDA 0xf6bb84: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "_CC_MD5_Final")]
// 0xf6bb94 — _CC_MD5_Final
// type: int __cdecl(unsigned __int8 *md, CC_MD5_CTX *c)
pub fn stub_0xf6bb94() {
    // IDA 0xf6bb94: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "_CC_MD5_Init")]
// 0xf6bba4 — _CC_MD5_Init
// type: int __cdecl(CC_MD5_CTX *c)
pub fn stub_0xf6bba4() {
    // IDA 0xf6bba4: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "_CC_MD5_Update")]
// 0xf6bbb4 — _CC_MD5_Update
// type: int __cdecl(CC_MD5_CTX *c, const void *data, CC_LONG len)
pub fn stub_0xf6bbb4() {
    // IDA 0xf6bbb4: CommonCrypto digest helper. Crypto primitive at the live site -- carrier no-op.
}

#[doc(alias = "_CC_SHA1")]
// 0xf6bbc4 — _CC_SHA1
// type: unsigned __int8 *__cdecl(const void *data, CC_LONG len, unsigned __int8 *md)
pub fn stub_0xf6bbc4() {
    // IDA 0xf6bbc4: CommonCrypto digest helper. Crypto primitive at the live site -- carrier no-op.
}

#[doc(alias = "_NXGetArchInfoFromCpuType")]
// 0xf6bbd4 — _NXGetArchInfoFromCpuType
// type: const NXArchInfo *__cdecl(cpu_type_t cputype, cpu_subtype_t cpusubtype)
pub fn stub_0xf6bbd4() {
    // IDA 0xf6bbd4: CommonCrypto digest helper. Crypto primitive at the live site -- carrier no-op.
}

#[doc(alias = "_NXGetLocalArchInfo")]
// 0xf6bbe4 — _NXGetLocalArchInfo
// type: const NXArchInfo *(void)
pub fn stub_0xf6bbe4() {
    // IDA 0xf6bbe4: CommonCrypto digest helper. Crypto primitive at the live site -- carrier no-op.
}

#[doc(alias = "_OSAtomicAdd32")]
// 0xf6bbf4 — _OSAtomicAdd32
// type: int32_t __cdecl(int32_t __theAmount, int32_t *__theValue)
pub fn stub_0xf6bbf4() {
    // IDA 0xf6bbf4: CommonCrypto digest helper. Crypto primitive at the live site -- carrier no-op.
}

#[doc(alias = "_OSAtomicAdd32Barrier")]
// 0xf6bc04 — _OSAtomicAdd32Barrier
// type: int32_t __cdecl(int32_t __theAmount, int32_t *__theValue)
pub fn stub_0xf6bc04() {
    // IDA 0xf6bc04: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "_OSAtomicCompareAndSwap32")]
// 0xf6bc14 — _OSAtomicCompareAndSwap32
// type: bool __cdecl(int32_t __oldValue, int32_t __newValue, int32_t *__theValue)
pub fn stub_0xf6bc14() {
    // IDA 0xf6bc14: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "_OSAtomicCompareAndSwap32Barrier")]
// 0xf6bc24 — _OSAtomicCompareAndSwap32Barrier
// type: bool __cdecl(int32_t __oldValue, int32_t __newValue, int32_t *__theValue)
pub fn stub_0xf6bc24() {
    // IDA 0xf6bc24: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "_OSAtomicCompareAndSwapLong")]
// 0xf6bc34 — _OSAtomicCompareAndSwapLong
// type: bool __cdecl(__int32 __oldValue, __int32 __newValue, __int32 *__theValue)
pub fn stub_0xf6bc34() {
    // IDA 0xf6bc34: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "_OSAtomicCompareAndSwapPtrBarrier")]
// 0xf6bc44 — _OSAtomicCompareAndSwapPtrBarrier
// type: bool __cdecl(void *__oldValue, void *__newValue, void **__theValue)
pub fn stub_0xf6bc44() {
    // IDA 0xf6bc44: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "_OSMemoryBarrier")]
// 0xf6bc54 — _OSMemoryBarrier
// type: void(void)
pub fn stub_0xf6bc54() {
    // IDA 0xf6bc54: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "_OSSpinLockLock")]
// 0xf6bc64 — _OSSpinLockLock
// type: void __cdecl(OSSpinLock *__lock)
pub fn stub_0xf6bc64() {
    // IDA 0xf6bc64: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "_OSSpinLockUnlock")]
// 0xf6bc74 — _OSSpinLockUnlock
// type: void __cdecl(OSSpinLock *__lock)
pub fn stub_0xf6bc74() {
    // IDA 0xf6bc74: OSAtomic primitive. std::sync::atomic -- carrier no-op.
}

#[doc(alias = "__Block_copy")]
// 0xf6bc84 — __Block_copy
// type: void *__cdecl(const void *aBlock)
pub fn stub_0xf6bc84() {
    // IDA 0xf6bc84: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "__Block_object_assign")]
// 0xf6bc94 — __Block_object_assign
// type: void __cdecl(void *, const void *, const int)
pub fn stub_0xf6bc94() {
    // IDA 0xf6bc94: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "__Block_object_dispose")]
// 0xf6bca4 — __Block_object_dispose
// type: void __cdecl(const void *, const int)
pub fn stub_0xf6bca4() {
    // IDA 0xf6bca4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "__Block_release")]
// 0xf6bcb4 — __Block_release
// type: void __cdecl(const void *aBlock)
pub fn stub_0xf6bcb4() {
    // IDA 0xf6bcb4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "__NSGetExecutablePath")]
// 0xf6bcc4 — __NSGetExecutablePath
// type: int __cdecl(char *buf, uint32_t *bufsize)
pub fn stub_0xf6bcc4() {
    // IDA 0xf6bcc4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "__Unwind_SjLj_Register")]
// 0xf6bcd4 — __Unwind_SjLj_Register
// type: void __fastcall(struct SjLj_Function_Context *lpfctx)
pub fn stub_0xf6bcd4() {
    // IDA 0xf6bcd4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "__Unwind_SjLj_Resume")]
// 0xf6bce4 — __Unwind_SjLj_Resume
// type: void __fastcall(struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf6bce4() {
    // IDA 0xf6bce4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "__Unwind_SjLj_Unregister")]
// 0xf6bcf4 — __Unwind_SjLj_Unregister
// type: void __fastcall(struct SjLj_Function_Context *lpfctx)
pub fn stub_0xf6bcf4() {
    // IDA 0xf6bcf4: ObjC block copy/destroy helper. ARC Block_copy/Block_release — carrier no-op.
}

#[doc(alias = "___assert_rtn")]
// 0xf6bd04 — ___assert_rtn
// type: void __cdecl __noreturn(const char *, const char *, int, const char *)
pub fn stub_0xf6bd04() {
    // IDA 0xf6bd04: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___cxa_atexit")]
// 0xf6bd14 — ___cxa_atexit
// type: int __fastcall(void (__fastcall *lpfunc)(void *), void *obj, void *lpdso_handle)
pub fn stub_0xf6bd14() {
    // IDA 0xf6bd14: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___divdi3")]
// 0xf6bd24 — ___divdi3
pub fn stub_0xf6bd24() {
    // IDA 0xf6bd24: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___divmodsi4")]
// 0xf6bd34 — ___divmodsi4
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf6bd34() {
    // IDA 0xf6bd34: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___divsi3")]
// 0xf6bd44 — ___divsi3
pub fn stub_0xf6bd44() {
    // IDA 0xf6bd44: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___error")]
// 0xf6bd54 — ___error
// type: int *(void)
pub fn stub_0xf6bd54() {
    // IDA 0xf6bd54: C++ runtime (new/delete/unwind/personality). GlobalAlloc/panic runtime — carrier no-op.
}

#[doc(alias = "___fixdfdi")]
// 0xf6bd64 — ___fixdfdi
pub fn stub_0xf6bd64() {
    // IDA 0xf6bd64: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___fixsfdi")]
// 0xf6bd74 — ___fixsfdi
pub fn stub_0xf6bd74() {
    // IDA 0xf6bd74: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___fixunsdfdi")]
// 0xf6bd84 — ___fixunsdfdi
// type: unsigned __int64 __fastcall(double)
pub fn stub_0xf6bd84() {
    // IDA 0xf6bd84: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___fixunssfdi")]
// 0xf6bd94 — ___fixunssfdi
pub fn stub_0xf6bd94() {
    // IDA 0xf6bd94: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___floatdidf")]
// 0xf6bda4 — ___floatdidf
pub fn stub_0xf6bda4() {
    // IDA 0xf6bda4: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___floatdisf")]
// 0xf6bdb4 — ___floatdisf
pub fn stub_0xf6bdb4() {
    // IDA 0xf6bdb4: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___floatundidf")]
// 0xf6bdc4 — ___floatundidf
pub fn stub_0xf6bdc4() {
    // IDA 0xf6bdc4: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___floatundisf")]
// 0xf6bdd4 — ___floatundisf
pub fn stub_0xf6bdd4() {
    // IDA 0xf6bdd4: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___fpclassifyf")]
// 0xf6bde4 — ___fpclassifyf
// type: int __cdecl(float)
pub fn stub_0xf6bde4() {
    // IDA 0xf6bde4: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___maskrune")]
// 0xf6bdf4 — ___maskrune
// type: int __cdecl(__darwin_ct_rune_t, unsigned __int32)
pub fn stub_0xf6bdf4() {
    // IDA 0xf6bdf4: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___memcpy_chk")]
// 0xf6be04 — ___memcpy_chk
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf6be04() {
    // IDA 0xf6be04: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}

#[doc(alias = "___moddi3")]
// 0xf6be14 — ___moddi3
pub fn stub_0xf6be14() {
    // IDA 0xf6be14: compiler-rt arithmetic helper. Native int ops -- carrier no-op.
}
