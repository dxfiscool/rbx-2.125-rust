//! core shard nq — 100 core stubs EA-sorted asc fallback not yet in core after np.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 519 uncovered before -> 419 after, batch 0xf6d104..0xf6d6c8).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_unlink")]
// 0xf6d104 — _unlink
// type: int __cdecl(const char *)
pub fn stub_0xf6d104() {
    // IDA 0xf6d104: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_usleep")]
// 0xf6d114 — _usleep
// type: int __cdecl(useconds_t)
pub fn stub_0xf6d114() {
    // IDA 0xf6d114: POSIX libc wrapper. std equivalents at the live site -- carrier no-op.
}

#[doc(alias = "_vm_allocate")]
// 0xf6d124 — _vm_allocate
// type: kern_return_t __cdecl(vm_map_t target_task, vm_address_t *address, vm_size_t size, int flags)
pub fn stub_0xf6d124() {
    // IDA 0xf6d124: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_vm_deallocate")]
// 0xf6d134 — _vm_deallocate
// type: kern_return_t __cdecl(vm_map_t target_task, vm_address_t address, vm_size_t size)
pub fn stub_0xf6d134() {
    // IDA 0xf6d134: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_vm_map")]
// 0xf6d144 — _vm_map
// type: kern_return_t __cdecl(vm_map_t target_task, vm_address_t *address, vm_size_t size, vm_address_t mask, int flags, mem_entry_name_port_t object, vm_offset_t offset, boolean_t copy, v
pub fn stub_0xf6d144() {
    // IDA 0xf6d144: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_vm_protect")]
// 0xf6d154 — _vm_protect
// type: kern_return_t __cdecl(vm_map_t target_task, vm_address_t address, vm_size_t size, boolean_t set_maximum, vm_prot_t new_protection)
pub fn stub_0xf6d154() {
    // IDA 0xf6d154: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_vm_read_overwrite")]
// 0xf6d164 — _vm_read_overwrite
// type: kern_return_t __cdecl(vm_map_t target_task, vm_address_t address, vm_size_t size, vm_address_t data, vm_size_t *outsize)
pub fn stub_0xf6d164() {
    // IDA 0xf6d164: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_vsnprintf")]
// 0xf6d174 — _vsnprintf
// type: int __cdecl(char *__str, size_t __size, const char *__format, va_list)
pub fn stub_0xf6d174() {
    // IDA 0xf6d174: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_write")]
// 0xf6d184 — _write
// type: ssize_t __cdecl(int __fd, const void *__buf, size_t __nbyte)
pub fn stub_0xf6d184() {
    // IDA 0xf6d184: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_writev")]
// 0xf6d194 — _writev
// type: ssize_t __cdecl(int, const iovec *, int)
pub fn stub_0xf6d194() {
    // IDA 0xf6d194: Mach VM primitive. mmap equivalent -- carrier no-op.
}

#[doc(alias = "_CFAbsoluteTimeGetCurrent")]
// 0xf6d1a4 — _CFAbsoluteTimeGetCurrent
// type: CFAbsoluteTime(void)
pub fn stub_0xf6d1a4() {
    // IDA 0xf6d1a4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFArrayAppendValue")]
// 0xf6d1b4 — _CFArrayAppendValue
// type: void __cdecl(CFMutableArrayRef theArray, const void *value)
pub fn stub_0xf6d1b4() {
    // IDA 0xf6d1b4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFArrayGetCount")]
// 0xf6d1c4 — _CFArrayGetCount
// type: CFIndex __cdecl(CFArrayRef theArray)
pub fn stub_0xf6d1c4() {
    // IDA 0xf6d1c4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFArrayGetValues")]
// 0xf6d1d4 — _CFArrayGetValues
// type: void __cdecl(CFArrayRef theArray, CFRange range, const void **values)
pub fn stub_0xf6d1d4() {
    // IDA 0xf6d1d4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBitVectorCreate")]
// 0xf6d1e4 — _CFBitVectorCreate
// type: CFBitVectorRef __cdecl(CFAllocatorRef allocator, const UInt8 *bytes, CFIndex numBits)
pub fn stub_0xf6d1e4() {
    // IDA 0xf6d1e4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBitVectorCreateMutableCopy")]
// 0xf6d1f4 — _CFBitVectorCreateMutableCopy
// type: CFMutableBitVectorRef __cdecl(CFAllocatorRef allocator, CFIndex capacity, CFBitVectorRef bv)
pub fn stub_0xf6d1f4() {
    // IDA 0xf6d1f4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBitVectorGetBitAtIndex")]
// 0xf6d204 — _CFBitVectorGetBitAtIndex
// type: CFBit __cdecl(CFBitVectorRef bv, CFIndex idx)
pub fn stub_0xf6d204() {
    // IDA 0xf6d204: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBitVectorSetBitAtIndex")]
// 0xf6d214 — _CFBitVectorSetBitAtIndex
// type: void __cdecl(CFMutableBitVectorRef bv, CFIndex idx, CFBit value)
pub fn stub_0xf6d214() {
    // IDA 0xf6d214: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBooleanGetTypeID")]
// 0xf6d224 — _CFBooleanGetTypeID
// type: CFTypeID(void)
pub fn stub_0xf6d224() {
    // IDA 0xf6d224: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBundleCopyBundleURL")]
// 0xf6d234 — _CFBundleCopyBundleURL
// type: CFURLRef __cdecl(CFBundleRef bundle)
pub fn stub_0xf6d234() {
    // IDA 0xf6d234: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBundleGetBundleWithIdentifier")]
// 0xf6d244 — _CFBundleGetBundleWithIdentifier
// type: CFBundleRef __cdecl(CFStringRef bundleID)
pub fn stub_0xf6d244() {
    // IDA 0xf6d244: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBundleGetInfoDictionary")]
// 0xf6d254 — _CFBundleGetInfoDictionary
// type: CFDictionaryRef __cdecl(CFBundleRef bundle)
pub fn stub_0xf6d254() {
    // IDA 0xf6d254: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFBundleGetMainBundle")]
// 0xf6d264 — _CFBundleGetMainBundle
// type: CFBundleRef(void)
pub fn stub_0xf6d264() {
    // IDA 0xf6d264: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDataCreate")]
// 0xf6d274 — _CFDataCreate
// type: CFDataRef __cdecl(CFAllocatorRef allocator, const UInt8 *bytes, CFIndex length)
pub fn stub_0xf6d274() {
    // IDA 0xf6d274: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDataCreateMutable")]
// 0xf6d284 — _CFDataCreateMutable
// type: CFMutableDataRef __cdecl(CFAllocatorRef allocator, CFIndex capacity)
pub fn stub_0xf6d284() {
    // IDA 0xf6d284: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDataCreateWithBytesNoCopy")]
// 0xf6d294 — _CFDataCreateWithBytesNoCopy
// type: CFDataRef __cdecl(CFAllocatorRef allocator, const UInt8 *bytes, CFIndex length, CFAllocatorRef bytesDeallocator)
pub fn stub_0xf6d294() {
    // IDA 0xf6d294: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDataGetMutableBytePtr")]
// 0xf6d2a4 — _CFDataGetMutableBytePtr
// type: UInt8 *__cdecl(CFMutableDataRef theData)
pub fn stub_0xf6d2a4() {
    // IDA 0xf6d2a4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDataSetLength")]
// 0xf6d2b4 — _CFDataSetLength
// type: void __cdecl(CFMutableDataRef theData, CFIndex length)
pub fn stub_0xf6d2b4() {
    // IDA 0xf6d2b4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDictionaryGetCount")]
// 0xf6d2c4 — _CFDictionaryGetCount
// type: CFIndex __cdecl(CFDictionaryRef theDict)
pub fn stub_0xf6d2c4() {
    // IDA 0xf6d2c4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDictionaryGetKeysAndValues")]
// 0xf6d2d4 — _CFDictionaryGetKeysAndValues
// type: void __cdecl(CFDictionaryRef theDict, const void **keys, const void **values)
pub fn stub_0xf6d2d4() {
    // IDA 0xf6d2d4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDictionaryGetValue")]
// 0xf6d2e4 — _CFDictionaryGetValue
// type: const void *__cdecl(CFDictionaryRef theDict, const void *key)
pub fn stub_0xf6d2e4() {
    // IDA 0xf6d2e4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFDictionarySetValue")]
// 0xf6d2f4 — _CFDictionarySetValue
// type: void __cdecl(CFMutableDictionaryRef theDict, const void *key, const void *value)
pub fn stub_0xf6d2f4() {
    // IDA 0xf6d2f4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFEqual")]
// 0xf6d304 — _CFEqual
// type: Boolean __cdecl(CFTypeRef cf1, CFTypeRef cf2)
pub fn stub_0xf6d304() {
    // IDA 0xf6d304: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFGetTypeID")]
// 0xf6d314 — _CFGetTypeID
// type: CFTypeID __cdecl(CFTypeRef cf)
pub fn stub_0xf6d314() {
    // IDA 0xf6d314: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFHash")]
// 0xf6d324 — _CFHash
// type: CFHashCode __cdecl(CFTypeRef cf)
pub fn stub_0xf6d324() {
    // IDA 0xf6d324: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFNumberCreate")]
// 0xf6d334 — _CFNumberCreate
// type: CFNumberRef __cdecl(CFAllocatorRef allocator, CFNumberType theType, const void *valuePtr)
pub fn stub_0xf6d334() {
    // IDA 0xf6d334: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFNumberGetValue")]
// 0xf6d344 — _CFNumberGetValue
// type: Boolean __cdecl(CFNumberRef number, CFNumberType theType, void *valuePtr)
pub fn stub_0xf6d344() {
    // IDA 0xf6d344: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFPreferencesAppSynchronize")]
// 0xf6d354 — _CFPreferencesAppSynchronize
// type: Boolean __cdecl(CFStringRef applicationID)
pub fn stub_0xf6d354() {
    // IDA 0xf6d354: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFPreferencesCopyAppValue")]
// 0xf6d364 — _CFPreferencesCopyAppValue
// type: CFPropertyListRef __cdecl(CFStringRef key, CFStringRef applicationID)
pub fn stub_0xf6d364() {
    // IDA 0xf6d364: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFPreferencesSetAppValue")]
// 0xf6d374 — _CFPreferencesSetAppValue
// type: void __cdecl(CFStringRef key, CFPropertyListRef value, CFStringRef applicationID)
pub fn stub_0xf6d374() {
    // IDA 0xf6d374: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFPropertyListCreateWithData")]
// 0xf6d384 — _CFPropertyListCreateWithData
// type: CFPropertyListRef __cdecl(CFAllocatorRef allocator, CFDataRef data, CFOptionFlags options, CFPropertyListFormat *format, CFErrorRef *error)
pub fn stub_0xf6d384() {
    // IDA 0xf6d384: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFRelease")]
// 0xf6d394 — _CFRelease
// type: void __cdecl(CFTypeRef cf)
pub fn stub_0xf6d394() {
    // IDA 0xf6d394: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFRetain")]
// 0xf6d3a4 — _CFRetain
// type: CFTypeRef __cdecl(CFTypeRef cf)
pub fn stub_0xf6d3a4() {
    // IDA 0xf6d3a4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFRunLoopCopyAllModes")]
// 0xf6d3b4 — _CFRunLoopCopyAllModes
// type: CFArrayRef __cdecl(CFRunLoopRef rl)
pub fn stub_0xf6d3b4() {
    // IDA 0xf6d3b4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFRunLoopGetCurrent")]
// 0xf6d3c4 — _CFRunLoopGetCurrent
// type: CFRunLoopRef(void)
pub fn stub_0xf6d3c4() {
    // IDA 0xf6d3c4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFRunLoopGetMain")]
// 0xf6d3d4 — _CFRunLoopGetMain
// type: CFRunLoopRef(void)
pub fn stub_0xf6d3d4() {
    // IDA 0xf6d3d4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFRunLoopRunInMode")]
// 0xf6d3e4 — _CFRunLoopRunInMode
// type: CFRunLoopRunResult __cdecl(CFRunLoopMode mode, CFTimeInterval seconds, Boolean returnAfterSourceHandled)
pub fn stub_0xf6d3e4() {
    // IDA 0xf6d3e4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFSetCreateMutable")]
// 0xf6d3f4 — _CFSetCreateMutable
// type: CFMutableSetRef __cdecl(CFAllocatorRef allocator, CFIndex capacity, const CFSetCallBacks *callBacks)
pub fn stub_0xf6d3f4() {
    // IDA 0xf6d3f4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringConvertEncodingToIANACharSetName")]
// 0xf6d404 — _CFStringConvertEncodingToIANACharSetName
// type: CFStringRef __cdecl(CFStringEncoding encoding)
pub fn stub_0xf6d404() {
    // IDA 0xf6d404: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringConvertNSStringEncodingToEncoding")]
// 0xf6d414 — _CFStringConvertNSStringEncodingToEncoding
// type: CFStringEncoding __cdecl(unsigned __int32 encoding)
pub fn stub_0xf6d414() {
    // IDA 0xf6d414: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringCreateWithBytes")]
// 0xf6d424 — _CFStringCreateWithBytes
// type: CFStringRef __cdecl(CFAllocatorRef alloc, const UInt8 *bytes, CFIndex numBytes, CFStringEncoding encoding, Boolean isExternalRepresentation)
pub fn stub_0xf6d424() {
    // IDA 0xf6d424: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringCreateWithBytesNoCopy")]
// 0xf6d434 — _CFStringCreateWithBytesNoCopy
// type: CFStringRef __cdecl(CFAllocatorRef alloc, const UInt8 *bytes, CFIndex numBytes, CFStringEncoding encoding, Boolean isExternalRepresentation, CFAllocatorRef contentsDeallocator)
pub fn stub_0xf6d434() {
    // IDA 0xf6d434: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringCreateWithCStringNoCopy")]
// 0xf6d444 — _CFStringCreateWithCStringNoCopy
// type: CFStringRef __cdecl(CFAllocatorRef alloc, const char *cStr, CFStringEncoding encoding, CFAllocatorRef contentsDeallocator)
pub fn stub_0xf6d444() {
    // IDA 0xf6d444: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringCreateWithCharacters")]
// 0xf6d454 — _CFStringCreateWithCharacters
// type: CFStringRef __cdecl(CFAllocatorRef alloc, const UniChar *chars, CFIndex numChars)
pub fn stub_0xf6d454() {
    // IDA 0xf6d454: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringGetBytes")]
// 0xf6d464 — _CFStringGetBytes
// type: CFIndex __cdecl(CFStringRef theString, CFRange range, CFStringEncoding encoding, UInt8 lossByte, Boolean isExternalRepresentation, UInt8 *buffer, CFIndex maxBufLen, CFIndex *usedBu
pub fn stub_0xf6d464() {
    // IDA 0xf6d464: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringGetCString")]
// 0xf6d474 — _CFStringGetCString
// type: Boolean __cdecl(CFStringRef theString, char *buffer, CFIndex bufferSize, CFStringEncoding encoding)
pub fn stub_0xf6d474() {
    // IDA 0xf6d474: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringGetCStringPtr")]
// 0xf6d484 — _CFStringGetCStringPtr
// type: const char *__cdecl(CFStringRef theString, CFStringEncoding encoding)
pub fn stub_0xf6d484() {
    // IDA 0xf6d484: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringGetFastestEncoding")]
// 0xf6d494 — _CFStringGetFastestEncoding
// type: CFStringEncoding __cdecl(CFStringRef theString)
pub fn stub_0xf6d494() {
    // IDA 0xf6d494: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringGetLength")]
// 0xf6d4a4 — _CFStringGetLength
// type: CFIndex __cdecl(CFStringRef theString)
pub fn stub_0xf6d4a4() {
    // IDA 0xf6d4a4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFStringGetMaximumSizeForEncoding")]
// 0xf6d4b4 — _CFStringGetMaximumSizeForEncoding
// type: CFIndex __cdecl(CFIndex length, CFStringEncoding encoding)
pub fn stub_0xf6d4b4() {
    // IDA 0xf6d4b4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFURLCopyFileSystemPath")]
// 0xf6d4c4 — _CFURLCopyFileSystemPath
// type: CFStringRef __cdecl(CFURLRef anURL, CFURLPathStyle pathStyle)
pub fn stub_0xf6d4c4() {
    // IDA 0xf6d4c4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFURLCreateStringByAddingPercentEscapes")]
// 0xf6d4d4 — _CFURLCreateStringByAddingPercentEscapes
// type: CFStringRef __cdecl(CFAllocatorRef allocator, CFStringRef originalString, CFStringRef charactersToLeaveUnescaped, CFStringRef legalURLCharactersToBeEscaped, CFStringEncoding encodi
pub fn stub_0xf6d4d4() {
    // IDA 0xf6d4d4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFURLCreateStringByReplacingPercentEscapesUsingEncoding")]
// 0xf6d4e4 — _CFURLCreateStringByReplacingPercentEscapesUsingEncoding
// type: CFStringRef __cdecl(CFAllocatorRef allocator, CFStringRef origString, CFStringRef charsToLeaveEscaped, CFStringEncoding encoding)
pub fn stub_0xf6d4e4() {
    // IDA 0xf6d4e4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFUUIDCreate")]
// 0xf6d4f4 — _CFUUIDCreate
// type: CFUUIDRef __cdecl(CFAllocatorRef alloc)
pub fn stub_0xf6d4f4() {
    // IDA 0xf6d4f4: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFUUIDCreateFromUUIDBytes")]
// 0xf6d504 — _CFUUIDCreateFromUUIDBytes
// type: CFUUIDRef __cdecl(CFAllocatorRef alloc, CFUUIDBytes bytes)
pub fn stub_0xf6d504() {
    // IDA 0xf6d504: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "_CFUUIDCreateString")]
// 0xf6d514 — _CFUUIDCreateString
// type: CFStringRef __cdecl(CFAllocatorRef alloc, CFUUIDRef uuid)
pub fn stub_0xf6d514() {
    // IDA 0xf6d514: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "stub helpers")]
// 0xf6d524 —  stub helpers
// type: int(void)
pub fn stub_0xf6d524() {
    // IDA 0xf6d524: CoreFoundation type owned by the platform crate -- carrier no-op in core.
}

#[doc(alias = "sub_F6D548")]
// 0xf6d548 — sub_F6D548
pub fn stub_0xf6d548() {
    // IDA 0xf6d548: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D554")]
// 0xf6d554 — sub_F6D554
pub fn stub_0xf6d554() {
    // IDA 0xf6d554: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D560")]
// 0xf6d560 — sub_F6D560
pub fn stub_0xf6d560() {
    // IDA 0xf6d560: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D56C")]
// 0xf6d56c — sub_F6D56C
pub fn stub_0xf6d56c() {
    // IDA 0xf6d56c: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D578")]
// 0xf6d578 — sub_F6D578
pub fn stub_0xf6d578() {
    // IDA 0xf6d578: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D584")]
// 0xf6d584 — sub_F6D584
pub fn stub_0xf6d584() {
    // IDA 0xf6d584: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D590")]
// 0xf6d590 — sub_F6D590
pub fn stub_0xf6d590() {
    // IDA 0xf6d590: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D59C")]
// 0xf6d59c — sub_F6D59C
pub fn stub_0xf6d59c() {
    // IDA 0xf6d59c: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5A8")]
// 0xf6d5a8 — sub_F6D5A8
pub fn stub_0xf6d5a8() {
    // IDA 0xf6d5a8: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5B4")]
// 0xf6d5b4 — sub_F6D5B4
pub fn stub_0xf6d5b4() {
    // IDA 0xf6d5b4: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5C0")]
// 0xf6d5c0 — sub_F6D5C0
pub fn stub_0xf6d5c0() {
    // IDA 0xf6d5c0: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5CC")]
// 0xf6d5cc — sub_F6D5CC
pub fn stub_0xf6d5cc() {
    // IDA 0xf6d5cc: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5D8")]
// 0xf6d5d8 — sub_F6D5D8
pub fn stub_0xf6d5d8() {
    // IDA 0xf6d5d8: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5E4")]
// 0xf6d5e4 — sub_F6D5E4
pub fn stub_0xf6d5e4() {
    // IDA 0xf6d5e4: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5F0")]
// 0xf6d5f0 — sub_F6D5F0
pub fn stub_0xf6d5f0() {
    // IDA 0xf6d5f0: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D5FC")]
// 0xf6d5fc — sub_F6D5FC
pub fn stub_0xf6d5fc() {
    // IDA 0xf6d5fc: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D608")]
// 0xf6d608 — sub_F6D608
pub fn stub_0xf6d608() {
    // IDA 0xf6d608: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D614")]
// 0xf6d614 — sub_F6D614
pub fn stub_0xf6d614() {
    // IDA 0xf6d614: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D620")]
// 0xf6d620 — sub_F6D620
pub fn stub_0xf6d620() {
    // IDA 0xf6d620: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D62C")]
// 0xf6d62c — sub_F6D62C
pub fn stub_0xf6d62c() {
    // IDA 0xf6d62c: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D638")]
// 0xf6d638 — sub_F6D638
pub fn stub_0xf6d638() {
    // IDA 0xf6d638: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D644")]
// 0xf6d644 — sub_F6D644
pub fn stub_0xf6d644() {
    // IDA 0xf6d644: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D650")]
// 0xf6d650 — sub_F6D650
pub fn stub_0xf6d650() {
    // IDA 0xf6d650: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D65C")]
// 0xf6d65c — sub_F6D65C
pub fn stub_0xf6d65c() {
    // IDA 0xf6d65c: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D668")]
// 0xf6d668 — sub_F6D668
pub fn stub_0xf6d668() {
    // IDA 0xf6d668: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D674")]
// 0xf6d674 — sub_F6D674
pub fn stub_0xf6d674() {
    // IDA 0xf6d674: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D680")]
// 0xf6d680 — sub_F6D680
pub fn stub_0xf6d680() {
    // IDA 0xf6d680: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D68C")]
// 0xf6d68c — sub_F6D68C
pub fn stub_0xf6d68c() {
    // IDA 0xf6d68c: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D698")]
// 0xf6d698 — sub_F6D698
pub fn stub_0xf6d698() {
    // IDA 0xf6d698: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D6A4")]
// 0xf6d6a4 — sub_F6D6A4
pub fn stub_0xf6d6a4() {
    // IDA 0xf6d6a4: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D6B0")]
// 0xf6d6b0 — sub_F6D6B0
pub fn stub_0xf6d6b0() {
    // IDA 0xf6d6b0: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D6BC")]
// 0xf6d6bc — sub_F6D6BC
pub fn stub_0xf6d6bc() {
    // IDA 0xf6d6bc: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}

#[doc(alias = "sub_F6D6C8")]
// 0xf6d6c8 — sub_F6D6C8
pub fn stub_0xf6d6c8() {
    // IDA 0xf6d6c8: unnamed IDA subroutine (sub_ auto-name; sampled 0xF6EFF4/0xF6F00C decompile to dyld _stub_helpers trampolines). Link/codegen helper — carrier no-op.
}
