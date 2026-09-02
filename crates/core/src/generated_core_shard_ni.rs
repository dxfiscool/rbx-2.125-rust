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
pub fn stub_0xf607a4() -> ! {
    todo!("0xf607a4 j___ZN10XmlElementD2Ev")
}

#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
// 0xf613a4 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xf613a4() -> ! {
    todo!("0xf613a4 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "DataStructures::Queue<HuffmanEncodingTreeNode *>::Push(HuffmanEncodingTreeNode * const&,char const*,unsigned int)")]
// 0xf61a44 — j___ZN14DataStructures5QueueIP23HuffmanEncodingTreeNodeE4PushERKS2_PKcj
pub fn stub_0xf61a44() -> ! {
    todo!("0xf61a44 j___ZN14DataStructures5QueueIP23HuffmanEncodingTreeNodeE4PushERKS2_PKcj")
}

#[doc(alias = "DataStructures::Queue<bool>::Push(bool const&,char const*,unsigned int)")]
// 0xf61cf4 — j___ZN14DataStructures5QueueIbE4PushERKbPKcj
// type: int()
pub fn stub_0xf61cf4() -> ! {
    todo!("0xf61cf4 j___ZN14DataStructures5QueueIbE4PushERKbPKcj")
}

#[doc(alias = "DataStructures::Queue<bool>::Compress(char const*,unsigned int)")]
// 0xf61d04 — j___ZN14DataStructures5QueueIbE8CompressEPKcj
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf61d04() -> ! {
    todo!("0xf61d04 j___ZN14DataStructures5QueueIbE8CompressEPKcj")
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE9classNameEv")]
// 0xf65194 — j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE9classNameEv
pub fn stub_0xf65194() -> ! {
    todo!("0xf65194 j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_20sMeshContentProviderEEE9classNameEv")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_11sForceFieldEEEERKS0_v")]
// 0xf654a4 — j___ZN3RBX4Name7declareILZNS_11sForceFieldEEEERKS0_v
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0xf654a4() -> ! {
    todo!("0xf654a4 j___ZN3RBX4Name7declareILZNS_11sForceFieldEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE9classNameEv")]
// 0xf65854 — j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE9classNameEv
pub fn stub_0xf65854() -> ! {
    todo!("0xf65854 j___ZN3RBX17NonFactoryProductINS_24CacheableContentProviderELZNS_23sTextureContentProviderEEE9classNameEv")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")]
// 0xf65884 — j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v
pub fn stub_0xf65884() -> ! {
    todo!("0xf65884 j___ZN3RBX4Name7declareILZNS_13sCylinderMeshEEEERKS0_v")
}

#[doc(alias = "j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")]
// 0xf66104 — j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v
// type: int(void)
pub fn stub_0xf66104() -> ! {
    todo!("0xf66104 j___ZN3RBX4Name7declareILZNS_10sBlockMeshEEEERKS0_v")
}

#[doc(alias = "_class_addMethod")]
// 0xf6adf4 — _class_addMethod
// type: BOOL __cdecl(Class cls, SEL name, IMP imp, const char *types)
pub fn stub_0xf6adf4() -> ! {
    todo!("0xf6adf4 _class_addMethod")
}

#[doc(alias = "_class_getIvarLayout")]
// 0xf6ae34 — _class_getIvarLayout
// type: const uint8_t *__cdecl(Class cls)
pub fn stub_0xf6ae34() -> ! {
    todo!("0xf6ae34 _class_getIvarLayout")
}

#[doc(alias = "_class_getSuperclass")]
// 0xf6ae44 — _class_getSuperclass
// type: Class __cdecl(Class cls)
pub fn stub_0xf6ae44() -> ! {
    todo!("0xf6ae44 _class_getSuperclass")
}

#[doc(alias = "_ivar_getName")]
// 0xf6ae54 — _ivar_getName
// type: const char *__cdecl(Ivar v)
pub fn stub_0xf6ae54() -> ! {
    todo!("0xf6ae54 _ivar_getName")
}

#[doc(alias = "_ivar_getOffset")]
// 0xf6ae64 — _ivar_getOffset
// type: ptrdiff_t __cdecl(Ivar v)
pub fn stub_0xf6ae64() -> ! {
    todo!("0xf6ae64 _ivar_getOffset")
}

#[doc(alias = "_objc_autorelease")]
// 0xf6ae74 — _objc_autorelease
// type: id __cdecl(id)
pub fn stub_0xf6ae74() -> ! {
    todo!("0xf6ae74 _objc_autorelease")
}

#[doc(alias = "_objc_autoreleaseReturnValue")]
// 0xf6ae84 — _objc_autoreleaseReturnValue
// type: id __cdecl(id)
pub fn stub_0xf6ae84() -> ! {
    todo!("0xf6ae84 _objc_autoreleaseReturnValue")
}

#[doc(alias = "_objc_begin_catch")]
// 0xf6ae94 — _objc_begin_catch
// type: id __cdecl(void *exc_buf)
pub fn stub_0xf6ae94() -> ! {
    todo!("0xf6ae94 _objc_begin_catch")
}

#[doc(alias = "_objc_end_catch")]
// 0xf6aea4 — _objc_end_catch
// type: void(void)
pub fn stub_0xf6aea4() -> ! {
    todo!("0xf6aea4 _objc_end_catch")
}

#[doc(alias = "_objc_enumerationMutation")]
// 0xf6aeb4 — _objc_enumerationMutation
// type: void __cdecl(id obj)
pub fn stub_0xf6aeb4() -> ! {
    todo!("0xf6aeb4 _objc_enumerationMutation")
}

#[doc(alias = "_objc_exception_rethrow")]
// 0xf6aec4 — _objc_exception_rethrow
// type: void(void)
pub fn stub_0xf6aec4() -> ! {
    todo!("0xf6aec4 _objc_exception_rethrow")
}

#[doc(alias = "_objc_getAssociatedObject")]
// 0xf6aed4 — _objc_getAssociatedObject
// type: id __cdecl(id object, const void *key)
pub fn stub_0xf6aed4() -> ! {
    todo!("0xf6aed4 _objc_getAssociatedObject")
}

#[doc(alias = "_objc_getClass")]
// 0xf6aee4 — _objc_getClass
// type: Class __cdecl(const char *name)
pub fn stub_0xf6aee4() -> ! {
    todo!("0xf6aee4 _objc_getClass")
}

#[doc(alias = "_objc_getProperty")]
// 0xf6aef4 — _objc_getProperty
// type: id __cdecl(id self, SEL _cmd, ptrdiff_t offset, bool atomic)
pub fn stub_0xf6aef4() -> ! {
    todo!("0xf6aef4 _objc_getProperty")
}

#[doc(alias = "_objc_msgSend")]
// 0xf6af04 — _objc_msgSend
// type: id(id, SEL, ...)
pub fn stub_0xf6af04() -> ! {
    todo!("0xf6af04 _objc_msgSend")
}

#[doc(alias = "_objc_msgSendSuper2")]
// 0xf6af14 — _objc_msgSendSuper2
// type: id(objc_super *, SEL, ...)
pub fn stub_0xf6af14() -> ! {
    todo!("0xf6af14 _objc_msgSendSuper2")
}

#[doc(alias = "_objc_msgSend_stret")]
// 0xf6af24 — _objc_msgSend_stret
// type: void(id, SEL, ...)
pub fn stub_0xf6af24() -> ! {
    todo!("0xf6af24 _objc_msgSend_stret")
}

#[doc(alias = "_objc_release")]
// 0xf6af34 — _objc_release
// type: void __cdecl(id)
pub fn stub_0xf6af34() -> ! {
    todo!("0xf6af34 _objc_release")
}

#[doc(alias = "_objc_retain")]
// 0xf6af44 — _objc_retain
// type: id __cdecl(id)
pub fn stub_0xf6af44() -> ! {
    todo!("0xf6af44 _objc_retain")
}

#[doc(alias = "_objc_retainAutorelease")]
// 0xf6af54 — _objc_retainAutorelease
// type: id __cdecl(id)
pub fn stub_0xf6af54() -> ! {
    todo!("0xf6af54 _objc_retainAutorelease")
}

#[doc(alias = "_objc_retainAutoreleaseReturnValue")]
// 0xf6af64 — _objc_retainAutoreleaseReturnValue
// type: id __cdecl(id)
pub fn stub_0xf6af64() -> ! {
    todo!("0xf6af64 _objc_retainAutoreleaseReturnValue")
}

#[doc(alias = "_objc_retainAutoreleasedReturnValue")]
// 0xf6af74 — _objc_retainAutoreleasedReturnValue
// type: id __cdecl(id)
pub fn stub_0xf6af74() -> ! {
    todo!("0xf6af74 _objc_retainAutoreleasedReturnValue")
}

#[doc(alias = "_objc_retainBlock")]
// 0xf6af84 — _objc_retainBlock
// type: id __cdecl(id)
pub fn stub_0xf6af84() -> ! {
    todo!("0xf6af84 _objc_retainBlock")
}

#[doc(alias = "_objc_setAssociatedObject")]
// 0xf6af94 — _objc_setAssociatedObject
// type: void __cdecl(id object, const void *key, id value, void *policy)
pub fn stub_0xf6af94() -> ! {
    todo!("0xf6af94 _objc_setAssociatedObject")
}

#[doc(alias = "_objc_setProperty")]
// 0xf6afa4 — _objc_setProperty
// type: void __cdecl(id self, SEL _cmd, ptrdiff_t offset, id newValue, bool atomic, char shouldCopy)
pub fn stub_0xf6afa4() -> ! {
    todo!("0xf6afa4 _objc_setProperty")
}

#[doc(alias = "_objc_storeStrong")]
// 0xf6afb4 — _objc_storeStrong
// type: void __cdecl(id *location, id obj)
pub fn stub_0xf6afb4() -> ! {
    todo!("0xf6afb4 _objc_storeStrong")
}

#[doc(alias = "_objc_sync_enter")]
// 0xf6afc4 — _objc_sync_enter
// type: int __cdecl(id obj)
pub fn stub_0xf6afc4() -> ! {
    todo!("0xf6afc4 _objc_sync_enter")
}

#[doc(alias = "_objc_sync_exit")]
// 0xf6afd4 — _objc_sync_exit
// type: int __cdecl(id obj)
pub fn stub_0xf6afd4() -> ! {
    todo!("0xf6afd4 _objc_sync_exit")
}

#[doc(alias = "_object_getClass")]
// 0xf6afe4 — _object_getClass
// type: Class __cdecl(id)
pub fn stub_0xf6afe4() -> ! {
    todo!("0xf6afe4 _object_getClass")
}

#[doc(alias = "_object_setIvar")]
// 0xf6aff4 — _object_setIvar
// type: void __cdecl(id obj, Ivar ivar, id value)
pub fn stub_0xf6aff4() -> ! {
    todo!("0xf6aff4 _object_setIvar")
}

#[doc(alias = "_sel_getUid")]
// 0xf6b014 — _sel_getUid
// type: SEL __cdecl(const char *str)
pub fn stub_0xf6b014() -> ! {
    todo!("0xf6b014 _sel_getUid")
}

#[doc(alias = "operator delete[](void *)")]
// 0xf6ba44 — __ZdaPv
// type: void __fastcall(void *)
pub fn stub_0xf6ba44() -> ! {
    todo!("0xf6ba44 __ZdaPv")
}

#[doc(alias = "operator delete(void *)")]
// 0xf6ba54 — __ZdlPv
// type: void __fastcall(void *)
pub fn stub_0xf6ba54() -> ! {
    todo!("0xf6ba54 __ZdlPv")
}

#[doc(alias = "operator new[](unsigned long)")]
// 0xf6ba74 — __Znam
// type: _DWORD __fastcall(unsigned int)
pub fn stub_0xf6ba74() -> ! {
    todo!("0xf6ba74 __Znam")
}

#[doc(alias = "operator new(unsigned long)")]
// 0xf6ba94 — __Znwm
// type: _DWORD __fastcall(unsigned int)
pub fn stub_0xf6ba94() -> ! {
    todo!("0xf6ba94 __Znwm")
}

#[doc(alias = "___cxa_allocate_exception")]
// 0xf6bab4 — ___cxa_allocate_exception
// type: void *__fastcall(size_t thrown_size)
pub fn stub_0xf6bab4() -> ! {
    todo!("0xf6bab4 ___cxa_allocate_exception")
}

#[doc(alias = "___cxa_bad_typeid")]
// 0xf6bac4 — ___cxa_bad_typeid
// type: void __fastcall __noreturn()
pub fn stub_0xf6bac4() -> ! {
    todo!("0xf6bac4 ___cxa_bad_typeid")
}

#[doc(alias = "___cxa_begin_catch")]
// 0xf6bad4 — ___cxa_begin_catch
// type: void *__fastcall(void *)
pub fn stub_0xf6bad4() -> ! {
    todo!("0xf6bad4 ___cxa_begin_catch")
}

#[doc(alias = "___cxa_call_unexpected")]
// 0xf6bae4 — ___cxa_call_unexpected
// type: void __fastcall __noreturn(void *)
pub fn stub_0xf6bae4() -> ! {
    todo!("0xf6bae4 ___cxa_call_unexpected")
}

#[doc(alias = "___cxa_end_catch")]
// 0xf6baf4 — ___cxa_end_catch
// type: void __fastcall()
pub fn stub_0xf6baf4() -> ! {
    todo!("0xf6baf4 ___cxa_end_catch")
}

#[doc(alias = "___cxa_free_exception")]
// 0xf6bb04 — ___cxa_free_exception
// type: void __fastcall(void *)
pub fn stub_0xf6bb04() -> ! {
    todo!("0xf6bb04 ___cxa_free_exception")
}

#[doc(alias = "___cxa_get_exception_ptr")]
// 0xf6bb14 — ___cxa_get_exception_ptr
// type: void *__fastcall(void *)
pub fn stub_0xf6bb14() -> ! {
    todo!("0xf6bb14 ___cxa_get_exception_ptr")
}

#[doc(alias = "___cxa_guard_abort")]
// 0xf6bb24 — ___cxa_guard_abort
// type: void __fastcall(__guard *)
pub fn stub_0xf6bb24() -> ! {
    todo!("0xf6bb24 ___cxa_guard_abort")
}

#[doc(alias = "___cxa_guard_acquire")]
// 0xf6bb34 — ___cxa_guard_acquire
// type: int __fastcall(__guard *)
pub fn stub_0xf6bb34() -> ! {
    todo!("0xf6bb34 ___cxa_guard_acquire")
}

#[doc(alias = "___cxa_guard_release")]
// 0xf6bb44 — ___cxa_guard_release
// type: void __fastcall(__guard *)
pub fn stub_0xf6bb44() -> ! {
    todo!("0xf6bb44 ___cxa_guard_release")
}

#[doc(alias = "___cxa_rethrow")]
// 0xf6bb54 — ___cxa_rethrow
// type: void __fastcall __noreturn()
pub fn stub_0xf6bb54() -> ! {
    todo!("0xf6bb54 ___cxa_rethrow")
}

#[doc(alias = "___cxa_throw")]
// 0xf6bb64 — ___cxa_throw
// type: void __fastcall __noreturn(void *, struct type_info *lptinfo, void (__fastcall *)(void *))
pub fn stub_0xf6bb64() -> ! {
    todo!("0xf6bb64 ___cxa_throw")
}

#[doc(alias = "___dynamic_cast")]
// 0xf6bb74 — ___dynamic_cast
// type: void *__fastcall(const void *lpsrc, const struct __class_type_info *lpstype, const struct __class_type_info *lpdtype, ptrdiff_t s2d)
pub fn stub_0xf6bb74() -> ! {
    todo!("0xf6bb74 ___dynamic_cast")
}

#[doc(alias = "_CC_MD5")]
// 0xf6bb84 — _CC_MD5
// type: unsigned __int8 *__cdecl(const void *data, CC_LONG len, unsigned __int8 *md)
pub fn stub_0xf6bb84() -> ! {
    todo!("0xf6bb84 _CC_MD5")
}

#[doc(alias = "_CC_MD5_Final")]
// 0xf6bb94 — _CC_MD5_Final
// type: int __cdecl(unsigned __int8 *md, CC_MD5_CTX *c)
pub fn stub_0xf6bb94() -> ! {
    todo!("0xf6bb94 _CC_MD5_Final")
}

#[doc(alias = "_CC_MD5_Init")]
// 0xf6bba4 — _CC_MD5_Init
// type: int __cdecl(CC_MD5_CTX *c)
pub fn stub_0xf6bba4() -> ! {
    todo!("0xf6bba4 _CC_MD5_Init")
}

#[doc(alias = "_CC_MD5_Update")]
// 0xf6bbb4 — _CC_MD5_Update
// type: int __cdecl(CC_MD5_CTX *c, const void *data, CC_LONG len)
pub fn stub_0xf6bbb4() -> ! {
    todo!("0xf6bbb4 _CC_MD5_Update")
}

#[doc(alias = "_CC_SHA1")]
// 0xf6bbc4 — _CC_SHA1
// type: unsigned __int8 *__cdecl(const void *data, CC_LONG len, unsigned __int8 *md)
pub fn stub_0xf6bbc4() -> ! {
    todo!("0xf6bbc4 _CC_SHA1")
}

#[doc(alias = "_NXGetArchInfoFromCpuType")]
// 0xf6bbd4 — _NXGetArchInfoFromCpuType
// type: const NXArchInfo *__cdecl(cpu_type_t cputype, cpu_subtype_t cpusubtype)
pub fn stub_0xf6bbd4() -> ! {
    todo!("0xf6bbd4 _NXGetArchInfoFromCpuType")
}

#[doc(alias = "_NXGetLocalArchInfo")]
// 0xf6bbe4 — _NXGetLocalArchInfo
// type: const NXArchInfo *(void)
pub fn stub_0xf6bbe4() -> ! {
    todo!("0xf6bbe4 _NXGetLocalArchInfo")
}

#[doc(alias = "_OSAtomicAdd32")]
// 0xf6bbf4 — _OSAtomicAdd32
// type: int32_t __cdecl(int32_t __theAmount, int32_t *__theValue)
pub fn stub_0xf6bbf4() -> ! {
    todo!("0xf6bbf4 _OSAtomicAdd32")
}

#[doc(alias = "_OSAtomicAdd32Barrier")]
// 0xf6bc04 — _OSAtomicAdd32Barrier
// type: int32_t __cdecl(int32_t __theAmount, int32_t *__theValue)
pub fn stub_0xf6bc04() -> ! {
    todo!("0xf6bc04 _OSAtomicAdd32Barrier")
}

#[doc(alias = "_OSAtomicCompareAndSwap32")]
// 0xf6bc14 — _OSAtomicCompareAndSwap32
// type: bool __cdecl(int32_t __oldValue, int32_t __newValue, int32_t *__theValue)
pub fn stub_0xf6bc14() -> ! {
    todo!("0xf6bc14 _OSAtomicCompareAndSwap32")
}

#[doc(alias = "_OSAtomicCompareAndSwap32Barrier")]
// 0xf6bc24 — _OSAtomicCompareAndSwap32Barrier
// type: bool __cdecl(int32_t __oldValue, int32_t __newValue, int32_t *__theValue)
pub fn stub_0xf6bc24() -> ! {
    todo!("0xf6bc24 _OSAtomicCompareAndSwap32Barrier")
}

#[doc(alias = "_OSAtomicCompareAndSwapLong")]
// 0xf6bc34 — _OSAtomicCompareAndSwapLong
// type: bool __cdecl(__int32 __oldValue, __int32 __newValue, __int32 *__theValue)
pub fn stub_0xf6bc34() -> ! {
    todo!("0xf6bc34 _OSAtomicCompareAndSwapLong")
}

#[doc(alias = "_OSAtomicCompareAndSwapPtrBarrier")]
// 0xf6bc44 — _OSAtomicCompareAndSwapPtrBarrier
// type: bool __cdecl(void *__oldValue, void *__newValue, void **__theValue)
pub fn stub_0xf6bc44() -> ! {
    todo!("0xf6bc44 _OSAtomicCompareAndSwapPtrBarrier")
}

#[doc(alias = "_OSMemoryBarrier")]
// 0xf6bc54 — _OSMemoryBarrier
// type: void(void)
pub fn stub_0xf6bc54() -> ! {
    todo!("0xf6bc54 _OSMemoryBarrier")
}

#[doc(alias = "_OSSpinLockLock")]
// 0xf6bc64 — _OSSpinLockLock
// type: void __cdecl(OSSpinLock *__lock)
pub fn stub_0xf6bc64() -> ! {
    todo!("0xf6bc64 _OSSpinLockLock")
}

#[doc(alias = "_OSSpinLockUnlock")]
// 0xf6bc74 — _OSSpinLockUnlock
// type: void __cdecl(OSSpinLock *__lock)
pub fn stub_0xf6bc74() -> ! {
    todo!("0xf6bc74 _OSSpinLockUnlock")
}

#[doc(alias = "__Block_copy")]
// 0xf6bc84 — __Block_copy
// type: void *__cdecl(const void *aBlock)
pub fn stub_0xf6bc84() -> ! {
    todo!("0xf6bc84 __Block_copy")
}

#[doc(alias = "__Block_object_assign")]
// 0xf6bc94 — __Block_object_assign
// type: void __cdecl(void *, const void *, const int)
pub fn stub_0xf6bc94() -> ! {
    todo!("0xf6bc94 __Block_object_assign")
}

#[doc(alias = "__Block_object_dispose")]
// 0xf6bca4 — __Block_object_dispose
// type: void __cdecl(const void *, const int)
pub fn stub_0xf6bca4() -> ! {
    todo!("0xf6bca4 __Block_object_dispose")
}

#[doc(alias = "__Block_release")]
// 0xf6bcb4 — __Block_release
// type: void __cdecl(const void *aBlock)
pub fn stub_0xf6bcb4() -> ! {
    todo!("0xf6bcb4 __Block_release")
}

#[doc(alias = "__NSGetExecutablePath")]
// 0xf6bcc4 — __NSGetExecutablePath
// type: int __cdecl(char *buf, uint32_t *bufsize)
pub fn stub_0xf6bcc4() -> ! {
    todo!("0xf6bcc4 __NSGetExecutablePath")
}

#[doc(alias = "__Unwind_SjLj_Register")]
// 0xf6bcd4 — __Unwind_SjLj_Register
// type: void __fastcall(struct SjLj_Function_Context *lpfctx)
pub fn stub_0xf6bcd4() -> ! {
    todo!("0xf6bcd4 __Unwind_SjLj_Register")
}

#[doc(alias = "__Unwind_SjLj_Resume")]
// 0xf6bce4 — __Unwind_SjLj_Resume
// type: void __fastcall(struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xf6bce4() -> ! {
    todo!("0xf6bce4 __Unwind_SjLj_Resume")
}

#[doc(alias = "__Unwind_SjLj_Unregister")]
// 0xf6bcf4 — __Unwind_SjLj_Unregister
// type: void __fastcall(struct SjLj_Function_Context *lpfctx)
pub fn stub_0xf6bcf4() -> ! {
    todo!("0xf6bcf4 __Unwind_SjLj_Unregister")
}

#[doc(alias = "___assert_rtn")]
// 0xf6bd04 — ___assert_rtn
// type: void __cdecl __noreturn(const char *, const char *, int, const char *)
pub fn stub_0xf6bd04() -> ! {
    todo!("0xf6bd04 ___assert_rtn")
}

#[doc(alias = "___cxa_atexit")]
// 0xf6bd14 — ___cxa_atexit
// type: int __fastcall(void (__fastcall *lpfunc)(void *), void *obj, void *lpdso_handle)
pub fn stub_0xf6bd14() -> ! {
    todo!("0xf6bd14 ___cxa_atexit")
}

#[doc(alias = "___divdi3")]
// 0xf6bd24 — ___divdi3
pub fn stub_0xf6bd24() -> ! {
    todo!("0xf6bd24 ___divdi3")
}

#[doc(alias = "___divmodsi4")]
// 0xf6bd34 — ___divmodsi4
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0xf6bd34() -> ! {
    todo!("0xf6bd34 ___divmodsi4")
}

#[doc(alias = "___divsi3")]
// 0xf6bd44 — ___divsi3
pub fn stub_0xf6bd44() -> ! {
    todo!("0xf6bd44 ___divsi3")
}

#[doc(alias = "___error")]
// 0xf6bd54 — ___error
// type: int *(void)
pub fn stub_0xf6bd54() -> ! {
    todo!("0xf6bd54 ___error")
}

#[doc(alias = "___fixdfdi")]
// 0xf6bd64 — ___fixdfdi
pub fn stub_0xf6bd64() -> ! {
    todo!("0xf6bd64 ___fixdfdi")
}

#[doc(alias = "___fixsfdi")]
// 0xf6bd74 — ___fixsfdi
pub fn stub_0xf6bd74() -> ! {
    todo!("0xf6bd74 ___fixsfdi")
}

#[doc(alias = "___fixunsdfdi")]
// 0xf6bd84 — ___fixunsdfdi
// type: unsigned __int64 __fastcall(double)
pub fn stub_0xf6bd84() -> ! {
    todo!("0xf6bd84 ___fixunsdfdi")
}

#[doc(alias = "___fixunssfdi")]
// 0xf6bd94 — ___fixunssfdi
pub fn stub_0xf6bd94() -> ! {
    todo!("0xf6bd94 ___fixunssfdi")
}

#[doc(alias = "___floatdidf")]
// 0xf6bda4 — ___floatdidf
pub fn stub_0xf6bda4() -> ! {
    todo!("0xf6bda4 ___floatdidf")
}

#[doc(alias = "___floatdisf")]
// 0xf6bdb4 — ___floatdisf
pub fn stub_0xf6bdb4() -> ! {
    todo!("0xf6bdb4 ___floatdisf")
}

#[doc(alias = "___floatundidf")]
// 0xf6bdc4 — ___floatundidf
pub fn stub_0xf6bdc4() -> ! {
    todo!("0xf6bdc4 ___floatundidf")
}

#[doc(alias = "___floatundisf")]
// 0xf6bdd4 — ___floatundisf
pub fn stub_0xf6bdd4() -> ! {
    todo!("0xf6bdd4 ___floatundisf")
}

#[doc(alias = "___fpclassifyf")]
// 0xf6bde4 — ___fpclassifyf
// type: int __cdecl(float)
pub fn stub_0xf6bde4() -> ! {
    todo!("0xf6bde4 ___fpclassifyf")
}

#[doc(alias = "___maskrune")]
// 0xf6bdf4 — ___maskrune
// type: int __cdecl(__darwin_ct_rune_t, unsigned __int32)
pub fn stub_0xf6bdf4() -> ! {
    todo!("0xf6bdf4 ___maskrune")
}

#[doc(alias = "___memcpy_chk")]
// 0xf6be04 — ___memcpy_chk
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xf6be04() -> ! {
    todo!("0xf6be04 ___memcpy_chk")
}

#[doc(alias = "___moddi3")]
// 0xf6be14 — ___moddi3
pub fn stub_0xf6be14() -> ! {
    todo!("0xf6be14 ___moddi3")
}
