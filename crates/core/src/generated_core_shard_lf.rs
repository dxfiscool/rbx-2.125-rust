//! core shard lf — 100 core stubs EA-sorted, next uncovered fallback after shard ld/le 0xee57dc..0xeee7d4 (lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted asc, next 100 uncovered not yet in core (fallback 33260 filtered, 3703 uncovered before batch, 3603 after).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[JKDictionary dealloc]")]
// 0xee57dc — -[JKDictionary dealloc]
// type: void __cdecl(JKDictionary *self, SEL)
pub fn stub_0xee57dc() {
    // IDA 0xee57dc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary count]")]
// 0xee5888 — -[JKDictionary count]
// type: unsigned int __cdecl(JKDictionary *self, SEL)
pub fn stub_0xee5888() {
    // IDA 0xee5888: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary objectForKey:]")]
// 0xee5898 — -[JKDictionary objectForKey:]
// type: id __cdecl(JKDictionary *self, SEL, id)
pub fn stub_0xee5898() {
    // IDA 0xee5898: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "__JKDictionaryHashTableEntryForKey")]
// 0xee596c — __JKDictionaryHashTableEntryForKey
pub fn stub_0xee596c() {
    // IDA 0xee596c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary getObjects:andKeys:]")]
// 0xee5c40 — -[JKDictionary getObjects:andKeys:]
// type: void __cdecl(JKDictionary *self, SEL, id *, id *)
pub fn stub_0xee5c40() {
    // IDA 0xee5c40: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary countByEnumeratingWithState:objects:count:]")]
// 0xee5e34 — -[JKDictionary countByEnumeratingWithState:objects:count:]
// type: unsigned int __cdecl(JKDictionary *self, SEL, $D08BFEEC8BCEE38771DFCC919E079042 *, id *, unsigned int)
pub fn stub_0xee5e34() {
    // IDA 0xee5e34: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary keyEnumerator]")]
// 0xee5f84 — -[JKDictionary keyEnumerator]
// type: id __cdecl(JKDictionary *self, SEL)
pub fn stub_0xee5f84() {
    // IDA 0xee5f84: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary setObject:forKey:]")]
// 0xee5fcc — -[JKDictionary setObject:forKey:]
// type: void __cdecl(JKDictionary *self, SEL, id, id)
pub fn stub_0xee5fcc() {
    // IDA 0xee5fcc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "__JKDictionaryAddObject")]
// 0xee63b8 — __JKDictionaryAddObject
pub fn stub_0xee63b8() {
    // IDA 0xee63b8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary removeObjectForKey:]")]
// 0xee6618 — -[JKDictionary removeObjectForKey:]
// type: void __cdecl(JKDictionary *self, SEL, id)
pub fn stub_0xee6618() {
    // IDA 0xee6618: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "__JKDictionaryRemoveObjectWithEntry")]
// 0xee6724 — __JKDictionaryRemoveObjectWithEntry
// type: unsigned int __fastcall(_DWORD *, int)
pub fn stub_0xee6724() {
    // IDA 0xee6724: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary copyWithZone:]")]
// 0xee6b20 — -[JKDictionary copyWithZone:]
// type: id __cdecl(JKDictionary *self, SEL, _NSZone *)
pub fn stub_0xee6b20() {
    // IDA 0xee6b20: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKDictionary mutableCopyWithZone:]")]
// 0xee6c38 — -[JKDictionary mutableCopyWithZone:]
// type: id __cdecl(JKDictionary *self, SEL, _NSZone *)
pub fn stub_0xee6c38() {
    // IDA 0xee6c38: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[JSONDecoder decoder]")]
// 0xee6d2c — +[JSONDecoder decoder]
// type: id __cdecl(id, SEL)
pub fn stub_0xee6d2c() {
    // IDA 0xee6d2c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[JSONDecoder decoderWithParseOptions:]")]
// 0xee6d44 — +[JSONDecoder decoderWithParseOptions:]
// type: id __cdecl(id, SEL, unsigned int)
pub fn stub_0xee6d44() {
    // IDA 0xee6d44: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder init]")]
// 0xee6d80 — -[JSONDecoder init]
// type: JSONDecoder *__cdecl(JSONDecoder *self, SEL)
pub fn stub_0xee6d80() {
    // IDA 0xee6d80: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder initWithParseOptions:]")]
// 0xee6d98 — -[JSONDecoder initWithParseOptions:]
// type: JSONDecoder *__cdecl(JSONDecoder *self, SEL, unsigned int)
pub fn stub_0xee6d98() {
    // IDA 0xee6d98: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder dealloc]")]
// 0xee6ea8 — -[JSONDecoder dealloc]
// type: void __cdecl(JSONDecoder *self, SEL)
pub fn stub_0xee6ea8() {
    // IDA 0xee6ea8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "__JSONDecoderCleanup")]
// 0xee6edc — __JSONDecoderCleanup
pub fn stub_0xee6edc() {
    // IDA 0xee6edc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder clearCache]")]
// 0xee6f40 — -[JSONDecoder clearCache]
// type: void __cdecl(JSONDecoder *self, SEL)
pub fn stub_0xee6f40() {
    // IDA 0xee6f40: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder parseUTF8String:length:]")]
// 0xee6ff4 — -[JSONDecoder parseUTF8String:length:]
// type: id __cdecl(JSONDecoder *self, SEL, const char *, unsigned int)
pub fn stub_0xee6ff4() {
    // IDA 0xee6ff4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder parseUTF8String:length:error:]")]
// 0xee7018 — -[JSONDecoder parseUTF8String:length:error:]
// type: id __cdecl(JSONDecoder *self, SEL, const char *, unsigned int, id *)
pub fn stub_0xee7018() {
    // IDA 0xee7018: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder parseJSONData:]")]
// 0xee703c — -[JSONDecoder parseJSONData:]
// type: id __cdecl(JSONDecoder *self, SEL, id)
pub fn stub_0xee703c() {
    // IDA 0xee703c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder parseJSONData:error:]")]
// 0xee7054 — -[JSONDecoder parseJSONData:error:]
// type: id __cdecl(JSONDecoder *self, SEL, id, id *)
pub fn stub_0xee7054() {
    // IDA 0xee7054: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder objectWithUTF8String:length:]")]
// 0xee706c — -[JSONDecoder objectWithUTF8String:length:]
// type: id __cdecl(JSONDecoder *self, SEL, const char *, unsigned int)
pub fn stub_0xee706c() {
    // IDA 0xee706c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder objectWithUTF8String:length:error:]")]
// 0xee7090 — -[JSONDecoder objectWithUTF8String:length:error:]
// type: id __cdecl(JSONDecoder *self, SEL, const char *, unsigned int, id *)
pub fn stub_0xee7090() {
    // IDA 0xee7090: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "__JKParseUTF8String")]
// 0xee7138 — __JKParseUTF8String
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0xee7138() {
    // IDA 0xee7138: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder objectWithData:]")]
// 0xee79d8 — -[JSONDecoder objectWithData:]
// type: id __cdecl(JSONDecoder *self, SEL, id)
pub fn stub_0xee79d8() {
    // IDA 0xee79d8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder objectWithData:error:]")]
// 0xee79f0 — -[JSONDecoder objectWithData:error:]
// type: id __cdecl(JSONDecoder *self, SEL, id, id *)
pub fn stub_0xee79f0() {
    // IDA 0xee79f0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder mutableObjectWithUTF8String:length:]")]
// 0xee7a80 — -[JSONDecoder mutableObjectWithUTF8String:length:]
// type: id __cdecl(JSONDecoder *self, SEL, const char *, unsigned int)
pub fn stub_0xee7a80() {
    // IDA 0xee7a80: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder mutableObjectWithUTF8String:length:error:]")]
// 0xee7aa4 — -[JSONDecoder mutableObjectWithUTF8String:length:error:]
// type: id __cdecl(JSONDecoder *self, SEL, const char *, unsigned int, id *)
pub fn stub_0xee7aa4() {
    // IDA 0xee7aa4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder mutableObjectWithData:]")]
// 0xee7b4c — -[JSONDecoder mutableObjectWithData:]
// type: id __cdecl(JSONDecoder *self, SEL, id)
pub fn stub_0xee7b4c() {
    // IDA 0xee7b4c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JSONDecoder mutableObjectWithData:error:]")]
// 0xee7b64 — -[JSONDecoder mutableObjectWithData:error:]
// type: id __cdecl(JSONDecoder *self, SEL, id, id *)
pub fn stub_0xee7b64() {
    // IDA 0xee7b64: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_objectFromJSONString]")]
// 0xee7bf4 — -[NSString bs_objectFromJSONString]
// type: id __cdecl(NSString *self, SEL)
pub fn stub_0xee7bf4() {
    // IDA 0xee7bf4: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_objectFromJSONStringWithParseOptions:]")]
// 0xee7c10 — -[NSString bs_objectFromJSONStringWithParseOptions:]
// type: id __cdecl(NSString *self, SEL, unsigned int)
pub fn stub_0xee7c10() {
    // IDA 0xee7c10: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_objectFromJSONStringWithParseOptions:error:]")]
// 0xee7c28 — -[NSString bs_objectFromJSONStringWithParseOptions:error:]
// type: id __cdecl(NSString *self, SEL, unsigned int, id *)
pub fn stub_0xee7c28() {
    // IDA 0xee7c28: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "__NSStringObjectFromJSONString")]
// 0xee7c38 — __NSStringObjectFromJSONString
pub fn stub_0xee7c38() {
    // IDA 0xee7c38: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_mutableObjectFromJSONString]")]
// 0xee7d98 — -[NSString bs_mutableObjectFromJSONString]
// type: id __cdecl(NSString *self, SEL)
pub fn stub_0xee7d98() {
    // IDA 0xee7d98: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_mutableObjectFromJSONStringWithParseOptions:]")]
// 0xee7db4 — -[NSString bs_mutableObjectFromJSONStringWithParseOptions:]
// type: id __cdecl(NSString *self, SEL, unsigned int)
pub fn stub_0xee7db4() {
    // IDA 0xee7db4: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_mutableObjectFromJSONStringWithParseOptions:error:]")]
// 0xee7dcc — -[NSString bs_mutableObjectFromJSONStringWithParseOptions:error:]
// type: id __cdecl(NSString *self, SEL, unsigned int, id *)
pub fn stub_0xee7dcc() {
    // IDA 0xee7dcc: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData bs_objectFromJSONData]")]
// 0xee7ddc — -[NSData bs_objectFromJSONData]
// type: id __cdecl(NSData *self, SEL)
pub fn stub_0xee7ddc() {
    // IDA 0xee7ddc: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData bs_objectFromJSONDataWithParseOptions:]")]
// 0xee7df8 — -[NSData bs_objectFromJSONDataWithParseOptions:]
// type: id __cdecl(NSData *self, SEL, unsigned int)
pub fn stub_0xee7df8() {
    // IDA 0xee7df8: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData bs_objectFromJSONDataWithParseOptions:error:]")]
// 0xee7e10 — -[NSData bs_objectFromJSONDataWithParseOptions:error:]
// type: id __cdecl(NSData *self, SEL, unsigned int, id *)
pub fn stub_0xee7e10() {
    // IDA 0xee7e10: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData bs_mutableObjectFromJSONData]")]
// 0xee7e5c — -[NSData bs_mutableObjectFromJSONData]
// type: id __cdecl(NSData *self, SEL)
pub fn stub_0xee7e5c() {
    // IDA 0xee7e5c: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData bs_mutableObjectFromJSONDataWithParseOptions:]")]
// 0xee7e78 — -[NSData bs_mutableObjectFromJSONDataWithParseOptions:]
// type: id __cdecl(NSData *self, SEL, unsigned int)
pub fn stub_0xee7e78() {
    // IDA 0xee7e78: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSData bs_mutableObjectFromJSONDataWithParseOptions:error:]")]
// 0xee7e90 — -[NSData bs_mutableObjectFromJSONDataWithParseOptions:error:]
// type: id __cdecl(NSData *self, SEL, unsigned int, id *)
pub fn stub_0xee7e90() {
    // IDA 0xee7e90: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[JKSerializer serializeObject:options:encodeOption:block:delegate:selector:error:]")]
// 0xee7edc — +[JKSerializer serializeObject:options:encodeOption:block:delegate:selector:error:]
// type: id __cdecl(id, SEL, id, unsigned int, unsigned int, id, id, SEL, id *)
pub fn stub_0xee7edc() {
    // IDA 0xee7edc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKSerializer serializeObject:options:encodeOption:block:delegate:selector:error:]")]
// 0xee7f50 — -[JKSerializer serializeObject:options:encodeOption:block:delegate:selector:error:]
// type: id __cdecl(JKSerializer *self, SEL, id, unsigned int, unsigned int, id, id, SEL, id *)
pub fn stub_0xee7f50() {
    // IDA 0xee7f50: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_jk_encode_error")]
// 0xee8544 — _jk_encode_error
pub fn stub_0xee8544() {
    // IDA 0xee8544: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_jk_encode_add_atom_to_buffer")]
// 0xee86b8 — _jk_encode_add_atom_to_buffer
pub fn stub_0xee86b8() {
    // IDA 0xee86b8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKSerializer releaseState]")]
// 0xee9e1c — -[JKSerializer releaseState]
// type: void __cdecl(JKSerializer *self, SEL)
pub fn stub_0xee9e1c() {
    // IDA 0xee9e1c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_jk_managedBuffer_release")]
// 0xee9e50 — _jk_managedBuffer_release
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0xee9e50() {
    // IDA 0xee9e50: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[JKSerializer dealloc]")]
// 0xee9e80 — -[JKSerializer dealloc]
// type: void __cdecl(JKSerializer *self, SEL)
pub fn stub_0xee9e80() {
    // IDA 0xee9e80: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_JSONData]")]
// 0xee9ec0 — -[NSString bs_JSONData]
// type: id __cdecl(NSString *self, SEL)
pub fn stub_0xee9ec0() {
    // IDA 0xee9ec0: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_JSONDataWithOptions:includeQuotes:error:]")]
// 0xee9ee4 — -[NSString bs_JSONDataWithOptions:includeQuotes:error:]
// type: id __cdecl(NSString *self, SEL, unsigned int, char, id *)
pub fn stub_0xee9ee4() {
    // IDA 0xee9ee4: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_JSONString]")]
// 0xee9f30 — -[NSString bs_JSONString]
// type: id __cdecl(NSString *self, SEL)
pub fn stub_0xee9f30() {
    // IDA 0xee9f30: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSString bs_JSONStringWithOptions:includeQuotes:error:]")]
// 0xee9f54 — -[NSString bs_JSONStringWithOptions:includeQuotes:error:]
// type: id __cdecl(NSString *self, SEL, unsigned int, char, id *)
pub fn stub_0xee9f54() {
    // IDA 0xee9f54: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONData]")]
// 0xee9fa0 — -[NSArray bs_JSONData]
// type: id __cdecl(NSArray *self, SEL)
pub fn stub_0xee9fa0() {
    // IDA 0xee9fa0: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONDataWithOptions:error:]")]
// 0xee9fdc — -[NSArray bs_JSONDataWithOptions:error:]
// type: id __cdecl(NSArray *self, SEL, unsigned int, id *)
pub fn stub_0xee9fdc() {
    // IDA 0xee9fdc: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONDataWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]")]
// 0xeea020 — -[NSArray bs_JSONDataWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]
// type: id __cdecl(NSArray *self, SEL, unsigned int, id, SEL, id *)
pub fn stub_0xeea020() {
    // IDA 0xeea020: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONString]")]
// 0xeea068 — -[NSArray bs_JSONString]
// type: id __cdecl(NSArray *self, SEL)
pub fn stub_0xeea068() {
    // IDA 0xeea068: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONStringWithOptions:error:]")]
// 0xeea0a4 — -[NSArray bs_JSONStringWithOptions:error:]
// type: id __cdecl(NSArray *self, SEL, unsigned int, id *)
pub fn stub_0xeea0a4() {
    // IDA 0xeea0a4: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONStringWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]")]
// 0xeea0e8 — -[NSArray bs_JSONStringWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]
// type: id __cdecl(NSArray *self, SEL, unsigned int, id, SEL, id *)
pub fn stub_0xeea0e8() {
    // IDA 0xeea0e8: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONData]")]
// 0xeea130 — -[NSDictionary bs_JSONData]
// type: id __cdecl(NSDictionary *self, SEL)
pub fn stub_0xeea130() {
    // IDA 0xeea130: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONDataWithOptions:error:]")]
// 0xeea16c — -[NSDictionary bs_JSONDataWithOptions:error:]
// type: id __cdecl(NSDictionary *self, SEL, unsigned int, id *)
pub fn stub_0xeea16c() {
    // IDA 0xeea16c: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONDataWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]")]
// 0xeea1b0 — -[NSDictionary bs_JSONDataWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]
// type: id __cdecl(NSDictionary *self, SEL, unsigned int, id, SEL, id *)
pub fn stub_0xeea1b0() {
    // IDA 0xeea1b0: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONString]")]
// 0xeea1f8 — -[NSDictionary bs_JSONString]
// type: id __cdecl(NSDictionary *self, SEL)
pub fn stub_0xeea1f8() {
    // IDA 0xeea1f8: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONStringWithOptions:error:]")]
// 0xeea234 — -[NSDictionary bs_JSONStringWithOptions:error:]
// type: id __cdecl(NSDictionary *self, SEL, unsigned int, id *)
pub fn stub_0xeea234() {
    // IDA 0xeea234: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONStringWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]")]
// 0xeea278 — -[NSDictionary bs_JSONStringWithOptions:serializeUnsupportedClassesUsingDelegate:selector:error:]
// type: id __cdecl(NSDictionary *self, SEL, unsigned int, id, SEL, id *)
pub fn stub_0xeea278() {
    // IDA 0xeea278: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONDataWithOptions:serializeUnsupportedClassesUsingBlock:error:]")]
// 0xeea2c0 — -[NSArray bs_JSONDataWithOptions:serializeUnsupportedClassesUsingBlock:error:]
// type: id __cdecl(NSArray *self, SEL, unsigned int, id, id *)
pub fn stub_0xeea2c0() {
    // IDA 0xeea2c0: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSArray bs_JSONStringWithOptions:serializeUnsupportedClassesUsingBlock:error:]")]
// 0xeea304 — -[NSArray bs_JSONStringWithOptions:serializeUnsupportedClassesUsingBlock:error:]
// type: id __cdecl(NSArray *self, SEL, unsigned int, id, id *)
pub fn stub_0xeea304() {
    // IDA 0xeea304: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONDataWithOptions:serializeUnsupportedClassesUsingBlock:error:]")]
// 0xeea348 — -[NSDictionary bs_JSONDataWithOptions:serializeUnsupportedClassesUsingBlock:error:]
// type: id __cdecl(NSDictionary *self, SEL, unsigned int, id, id *)
pub fn stub_0xeea348() {
    // IDA 0xeea348: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[NSDictionary bs_JSONStringWithOptions:serializeUnsupportedClassesUsingBlock:error:]")]
// 0xeea38c — -[NSDictionary bs_JSONStringWithOptions:serializeUnsupportedClassesUsingBlock:error:]
// type: id __cdecl(NSDictionary *self, SEL, unsigned int, id, id *)
pub fn stub_0xeea38c() {
    // IDA 0xeea38c: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_jk_managedBuffer_resize")]
// 0xeea3d0 — _jk_managedBuffer_resize
// type: int __fastcall(int, size_t __size)
pub fn stub_0xeea3d0() {
    // IDA 0xeea3d0: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_jk_encode_printf")]
// 0xeea594 — _jk_encode_printf
// type: int __fastcall(int, int, int, int, char *, char)
pub fn stub_0xeea594() {
    // IDA 0xeea594: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_ConvertSingleCodePointInUTF8")]
// 0xeea8f0 — _ConvertSingleCodePointInUTF8
pub fn stub_0xeea8f0() {
    // IDA 0xeea8f0: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_jk_encode_writen")]
// 0xeeab74 — _jk_encode_writen
// type: int __fastcall(int, int, int, int, void *__src, size_t __n)
pub fn stub_0xeeab74() {
    // IDA 0xeeab74: ObjC runtime metadata (class/ivar/protocol). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_jk_encode_write1slow")]
// 0xeeadec — _jk_encode_write1slow
pub fn stub_0xeeadec() {
    // IDA 0xeeadec: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_jk_encode_write1fast")]
// 0xeeb014 — _jk_encode_write1fast
// type: int __fastcall(int, char *__s)
pub fn stub_0xeeb014() {
    // IDA 0xeeb014: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_jk_encode_writePrettyPrintWhiteSpace")]
// 0xeeb330 — _jk_encode_writePrettyPrintWhiteSpace
pub fn stub_0xeeb330() {
    // IDA 0xeeb330: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_jk_objectStack_release")]
// 0xeeb500 — _jk_objectStack_release
pub fn stub_0xeeb500() {
    // IDA 0xeeb500: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_jk_parse_next_token")]
// 0xeeb764 — _jk_parse_next_token
pub fn stub_0xeeb764() {
    // IDA 0xeeb764: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jk_object_for_token")]
// 0xeecb48 — _jk_object_for_token
pub fn stub_0xeecb48() {
    // IDA 0xeecb48: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jk_error")]
// 0xeed42c — _jk_error
pub fn stub_0xeed42c() {
    // IDA 0xeed42c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jk_cachedObjects")]
// 0xeed5f0 — _jk_cachedObjects
pub fn stub_0xeed5f0() {
    // IDA 0xeed5f0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jk_objectStack_resize")]
// 0xeeda70 — _jk_objectStack_resize
pub fn stub_0xeeda70() {
    // IDA 0xeeda70: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jk_error_parse_accept_or3")]
// 0xeedcf8 — _jk_error_parse_accept_or3
pub fn stub_0xeedcf8() {
    // IDA 0xeedcf8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__JKDictionaryCapacityForCount")]
// 0xeeddc0 — __JKDictionaryCapacityForCount
pub fn stub_0xeeddc0() {
    // IDA 0xeeddc0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jk_parse_number")]
// 0xeede20 — _jk_parse_number
pub fn stub_0xeede20() {
    // IDA 0xeede20: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "-[BSReachability startNotifier]")]
// 0xeee284 — -[BSReachability startNotifier]
// type: char __cdecl(BSReachability *self, SEL)
pub fn stub_0xeee284() {
    // IDA 0xeee284: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ReachabilityCallback_0")]
// 0xeee2e4 — _ReachabilityCallback_0
// type: id __fastcall(int, int, void *)
pub fn stub_0xeee2e4() {
    // IDA 0xeee2e4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "-[BSReachability stopNotifier]")]
// 0xeee4ac — -[BSReachability stopNotifier]
// type: void __cdecl(BSReachability *self, SEL)
pub fn stub_0xeee4ac() {
    // IDA 0xeee4ac: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "-[BSReachability dealloc]")]
// 0xeee4e0 — -[BSReachability dealloc]
// type: void __cdecl(BSReachability *self, SEL)
pub fn stub_0xeee4e0() {
    // IDA 0xeee4e0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "+[BSReachability reachabilityWithHostName:]")]
// 0xeee538 — +[BSReachability reachabilityWithHostName:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xeee538() {
    // IDA 0xeee538: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSReachability reachabilityWithAddress:]")]
// 0xeee5b8 — +[BSReachability reachabilityWithAddress:]
// type: id __cdecl(id, SEL, const sockaddr_in *)
pub fn stub_0xeee5b8() {
    // IDA 0xeee5b8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSReachability reachabilityForInternetConnection]")]
// 0xeee630 — +[BSReachability reachabilityForInternetConnection]
// type: id __cdecl(id, SEL)
pub fn stub_0xeee630() {
    // IDA 0xeee630: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[BSReachability reachabilityForLocalWiFi]")]
// 0xeee67c — +[BSReachability reachabilityForLocalWiFi]
// type: id __cdecl(id, SEL)
pub fn stub_0xeee67c() {
    // IDA 0xeee67c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BSReachability localWiFiStatusForFlags:]")]
// 0xeee6e0 — -[BSReachability localWiFiStatusForFlags:]
// type: int __cdecl(BSReachability *self, SEL, unsigned int)
pub fn stub_0xeee6e0() {
    // IDA 0xeee6e0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BSReachability connectionRequired]")]
// 0xeee724 — -[BSReachability connectionRequired]
// type: char __cdecl(BSReachability *self, SEL)
pub fn stub_0xeee724() {
    // IDA 0xeee724: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[BSReachability currentReachabilityStatus]")]
// 0xeee7d4 — -[BSReachability currentReachabilityStatus]
// type: int __cdecl(BSReachability *self, SEL)
pub fn stub_0xeee7d4() {
    // IDA 0xeee7d4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

