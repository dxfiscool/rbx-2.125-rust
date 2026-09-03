//! core shard kq — 120 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 smallest not yet in rbx_core after kp 0xebc8ac (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33260 filtered, 28557 distinct, 4703 remaining before -> 4583 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[GAIReachabilityChecker setReachabilityApi:]")]
#[doc(alias = "-[GAIReachabilityChecker setReachabilityApi:]")]
// 0xec0db0 — -[GAIReachabilityChecker setReachabilityApi:]
// type: void __cdecl(GAIReachabilityChecker *self, SEL, const GAIReachabilityApi *)
pub fn stub_0xec0db0() {
    // IDA 0xec0db0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker isActive]")]
#[doc(alias = "-[GAIReachabilityChecker isActive]")]
// 0xec0e30 — -[GAIReachabilityChecker isActive]
// type: char __cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec0e30() {
    // IDA 0xec0e30: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker setDelegate:]")]
#[doc(alias = "-[GAIReachabilityChecker setDelegate:]")]
// 0xec0e48 — -[GAIReachabilityChecker setDelegate:]
// type: void __cdecl(GAIReachabilityChecker *self, SEL, id)
pub fn stub_0xec0e48() {
    // IDA 0xec0e48: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker init]")]
#[doc(alias = "-[GAIReachabilityChecker init]")]
// 0xec0ee4 — -[GAIReachabilityChecker init]
// type: GAIReachabilityChecker *__cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec0ee4() {
    // IDA 0xec0ee4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker initWithDelegate:withHost:]")]
#[doc(alias = "-[GAIReachabilityChecker initWithDelegate:withHost:]")]
// 0xec0f40 — -[GAIReachabilityChecker initWithDelegate:withHost:]
// type: GAIReachabilityChecker *__cdecl(GAIReachabilityChecker *self, SEL, id, id)
pub fn stub_0xec0f40() {
    // IDA 0xec0f40: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker dealloc]")]
#[doc(alias = "-[GAIReachabilityChecker dealloc]")]
// 0xec106c — -[GAIReachabilityChecker dealloc]
// type: void __cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec106c() {
    // IDA 0xec106c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker start]")]
#[doc(alias = "-[GAIReachabilityChecker start]")]
// 0xec10cc — -[GAIReachabilityChecker start]
// type: char __cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec10cc() {
    // IDA 0xec10cc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC11A8")]
#[doc(alias = "sub_EC11A8")]
// 0xec11a8 — sub_EC11A8
// type: int __fastcall(int, int, id)
pub fn stub_0xec11a8() {
    // IDA 0xec11a8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker stop]")]
#[doc(alias = "-[GAIReachabilityChecker stop]")]
// 0xec11c4 — -[GAIReachabilityChecker stop]
// type: void __cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec11c4() {
    // IDA 0xec11c4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker statusForFlags:]")]
#[doc(alias = "-[GAIReachabilityChecker statusForFlags:]")]
// 0xec1234 — -[GAIReachabilityChecker statusForFlags:]
// type: int __cdecl(GAIReachabilityChecker *self, SEL, unsigned int)
pub fn stub_0xec1234() {
    // IDA 0xec1234: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker reachabilityFlagsChanged:]")]
#[doc(alias = "-[GAIReachabilityChecker reachabilityFlagsChanged:]")]
// 0xec1260 — -[GAIReachabilityChecker reachabilityFlagsChanged:]
// type: void __cdecl(GAIReachabilityChecker *self, SEL, unsigned int)
pub fn stub_0xec1260() {
    // IDA 0xec1260: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker reachability]")]
#[doc(alias = "-[GAIReachabilityChecker reachability]")]
// 0xec1324 — -[GAIReachabilityChecker reachability]
// type: __SCNetworkReachability *__cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec1324() {
    // IDA 0xec1324: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker setReachability:]")]
#[doc(alias = "-[GAIReachabilityChecker setReachability:]")]
// 0xec1334 — -[GAIReachabilityChecker setReachability:]
// type: void __cdecl(GAIReachabilityChecker *self, SEL, __SCNetworkReachability *)
pub fn stub_0xec1334() {
    // IDA 0xec1334: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker reachabilityStatus]")]
#[doc(alias = "-[GAIReachabilityChecker reachabilityStatus]")]
// 0xec1344 — -[GAIReachabilityChecker reachabilityStatus]
// type: int __cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec1344() {
    // IDA 0xec1344: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker setReachabilityStatus:]")]
#[doc(alias = "-[GAIReachabilityChecker setReachabilityStatus:]")]
// 0xec1354 — -[GAIReachabilityChecker setReachabilityStatus:]
// type: void __cdecl(GAIReachabilityChecker *self, SEL, int)
pub fn stub_0xec1354() {
    // IDA 0xec1354: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker host]")]
#[doc(alias = "-[GAIReachabilityChecker host]")]
// 0xec1364 — -[GAIReachabilityChecker host]
// type: NSString *__cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec1364() {
    // IDA 0xec1364: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker setHost:]")]
#[doc(alias = "-[GAIReachabilityChecker setHost:]")]
// 0xec137c — -[GAIReachabilityChecker setHost:]
// type: void __cdecl(GAIReachabilityChecker *self, SEL, id)
pub fn stub_0xec137c() {
    // IDA 0xec137c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker delegate]")]
#[doc(alias = "-[GAIReachabilityChecker delegate]")]
// 0xec13a0 — -[GAIReachabilityChecker delegate]
// type: GAIReachabilityDelegate *__cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec13a0() {
    // IDA 0xec13a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_GAIInstallUncaughtExceptionHandler")]
#[doc(alias = "_GAIInstallUncaughtExceptionHandler")]
// 0xec13b0 — _GAIInstallUncaughtExceptionHandler
// type: int(void)
pub fn stub_0xec13b0() {
    // IDA 0xec13b0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_GAIUncaughtExceptionHandler")]
#[doc(alias = "_GAIUncaughtExceptionHandler")]
// 0xec13e8 — _GAIUncaughtExceptionHandler
pub fn stub_0xec13e8() {
    // IDA 0xec13e8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_GAIUninstallUncaughtExceptionHandler")]
#[doc(alias = "_GAIUninstallUncaughtExceptionHandler")]
// 0xec1620 — _GAIUninstallUncaughtExceptionHandler
// type: int(void)
pub fn stub_0xec1620() {
    // IDA 0xec1620: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReadOnlySetter setValue:forKey:withModel:isTemporary:]")]
#[doc(alias = "-[GAIReadOnlySetter setValue:forKey:withModel:isTemporary:]")]
// 0xec164c — -[GAIReadOnlySetter setValue:forKey:withModel:isTemporary:]
// type: char __cdecl(GAIReadOnlySetter *self, SEL, id, id, id, char)
pub fn stub_0xec164c() {
    // IDA 0xec164c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaModel init]")]
#[doc(alias = "-[GAIMetaModel init]")]
// 0xec16b4 — -[GAIMetaModel init]
// type: GAIMetaModel *__cdecl(GAIMetaModel *self, SEL)
pub fn stub_0xec16b4() {
    // IDA 0xec16b4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaModel dealloc]")]
#[doc(alias = "-[GAIMetaModel dealloc]")]
// 0xec1718 — -[GAIMetaModel dealloc]
// type: void __cdecl(GAIMetaModel *self, SEL)
pub fn stub_0xec1718() {
    // IDA 0xec1718: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaModel allKeys]")]
#[doc(alias = "-[GAIMetaModel allKeys]")]
// 0xec1764 — -[GAIMetaModel allKeys]
// type: id __cdecl(GAIMetaModel *self, SEL)
pub fn stub_0xec1764() {
    // IDA 0xec1764: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaModel allMetaInfos]")]
#[doc(alias = "-[GAIMetaModel allMetaInfos]")]
// 0xec1788 — -[GAIMetaModel allMetaInfos]
// type: id __cdecl(GAIMetaModel *self, SEL)
pub fn stub_0xec1788() {
    // IDA 0xec1788: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaModel metaInfoForKey:]")]
#[doc(alias = "-[GAIMetaModel metaInfoForKey:]")]
// 0xec17ac — -[GAIMetaModel metaInfoForKey:]
// type: id __cdecl(GAIMetaModel *self, SEL, id)
pub fn stub_0xec17ac() {
    // IDA 0xec17ac: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaModel addField:forKey:]")]
#[doc(alias = "-[GAIMetaModel addField:forKey:]")]
// 0xec17d0 — -[GAIMetaModel addField:forKey:]
// type: char __cdecl(GAIMetaModel *self, SEL, id, id)
pub fn stub_0xec17d0() {
    // IDA 0xec17d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo initWithUrlParam:isTemporary:assumedValue:getter:setter:]")]
#[doc(alias = "-[GAIMetaInfo initWithUrlParam:isTemporary:assumedValue:getter:setter:]")]
// 0xec1824 — -[GAIMetaInfo initWithUrlParam:isTemporary:assumedValue:getter:setter:]
// type: GAIMetaInfo *__cdecl(GAIMetaInfo *self, SEL, id, char, id, id, id)
pub fn stub_0xec1824() {
    // IDA 0xec1824: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo initWithUrlParam:isTemporary:assumedValue:getter:]")]
#[doc(alias = "-[GAIMetaInfo initWithUrlParam:isTemporary:assumedValue:getter:]")]
// 0xec1974 — -[GAIMetaInfo initWithUrlParam:isTemporary:assumedValue:getter:]
// type: GAIMetaInfo *__cdecl(GAIMetaInfo *self, SEL, id, char, id, id)
pub fn stub_0xec1974() {
    // IDA 0xec1974: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo dealloc]")]
#[doc(alias = "-[GAIMetaInfo dealloc]")]
// 0xec1adc — -[GAIMetaInfo dealloc]
// type: void __cdecl(GAIMetaInfo *self, SEL)
pub fn stub_0xec1adc() {
    // IDA 0xec1adc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo urlParameter]")]
#[doc(alias = "-[GAIMetaInfo urlParameter]")]
// 0xec1b68 — -[GAIMetaInfo urlParameter]
// type: NSString *__cdecl(GAIMetaInfo *self, SEL)
pub fn stub_0xec1b68() {
    // IDA 0xec1b68: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo isTemporary]")]
#[doc(alias = "-[GAIMetaInfo isTemporary]")]
// 0xec1b78 — -[GAIMetaInfo isTemporary]
// type: char __cdecl(GAIMetaInfo *self, SEL)
pub fn stub_0xec1b78() {
    // IDA 0xec1b78: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo assumedValue]")]
#[doc(alias = "-[GAIMetaInfo assumedValue]")]
// 0xec1b88 — -[GAIMetaInfo assumedValue]
// type: NSString *__cdecl(GAIMetaInfo *self, SEL)
pub fn stub_0xec1b88() {
    // IDA 0xec1b88: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo getter]")]
#[doc(alias = "-[GAIMetaInfo getter]")]
// 0xec1b98 — -[GAIMetaInfo getter]
// type: GAIGetter *__cdecl(GAIMetaInfo *self, SEL)
pub fn stub_0xec1b98() {
    // IDA 0xec1b98: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIMetaInfo setter]")]
#[doc(alias = "-[GAIMetaInfo setter]")]
// 0xec1ba8 — -[GAIMetaInfo setter]
// type: GAISetter *__cdecl(GAIMetaInfo *self, SEL)
pub fn stub_0xec1ba8() {
    // IDA 0xec1ba8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel init]")]
#[doc(alias = "-[GAIModel init]")]
// 0xec1bb8 — -[GAIModel init]
// type: GAIModel *__cdecl(GAIModel *self, SEL)
pub fn stub_0xec1bb8() {
    // IDA 0xec1bb8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel initWithMetaModel:]")]
#[doc(alias = "-[GAIModel initWithMetaModel:]")]
// 0xec1c14 — -[GAIModel initWithMetaModel:]
// type: GAIModel *__cdecl(GAIModel *self, SEL, id)
pub fn stub_0xec1c14() {
    // IDA 0xec1c14: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel dealloc]")]
#[doc(alias = "-[GAIModel dealloc]")]
// 0xec1cc4 — -[GAIModel dealloc]
// type: void __cdecl(GAIModel *self, SEL)
pub fn stub_0xec1cc4() {
    // IDA 0xec1cc4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel internalSet:forKey:isTemporary:isRaw:]")]
#[doc(alias = "-[GAIModel internalSet:forKey:isTemporary:isRaw:]")]
// 0xec1d3c — -[GAIModel internalSet:forKey:isTemporary:isRaw:]
// type: char __cdecl(GAIModel *self, SEL, id, id, char, char)
pub fn stub_0xec1d3c() {
    // IDA 0xec1d3c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel setValue:forKey:isTemporary:]")]
#[doc(alias = "-[GAIModel setValue:forKey:isTemporary:]")]
// 0xec2144 — -[GAIModel setValue:forKey:isTemporary:]
// type: char __cdecl(GAIModel *self, SEL, id, id, char)
pub fn stub_0xec2144() {
    // IDA 0xec2144: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel setValue:forKey:]")]
#[doc(alias = "-[GAIModel setValue:forKey:]")]
// 0xec2168 — -[GAIModel setValue:forKey:]
// type: char __cdecl(GAIModel *self, SEL, id, id)
pub fn stub_0xec2168() {
    // IDA 0xec2168: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel rawSetValue:forKey:isTemporary:]")]
#[doc(alias = "-[GAIModel rawSetValue:forKey:isTemporary:]")]
// 0xec2190 — -[GAIModel rawSetValue:forKey:isTemporary:]
// type: char __cdecl(GAIModel *self, SEL, id, id, char)
pub fn stub_0xec2190() {
    // IDA 0xec2190: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel setFromDictionary:isTemporary:]")]
#[doc(alias = "-[GAIModel setFromDictionary:isTemporary:]")]
// 0xec21b4 — -[GAIModel setFromDictionary:isTemporary:]
// type: char __cdecl(GAIModel *self, SEL, id, char)
pub fn stub_0xec21b4() {
    // IDA 0xec21b4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC22C8")]
#[doc(alias = "sub_EC22C8")]
// 0xec22c8 — sub_EC22C8
pub fn stub_0xec22c8() {
    // IDA 0xec22c8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC2300")]
#[doc(alias = "sub_EC2300")]
// 0xec2300 — sub_EC2300
pub fn stub_0xec2300() {
    // IDA 0xec2300: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC2324")]
#[doc(alias = "sub_EC2324")]
// 0xec2324 — sub_EC2324
pub fn stub_0xec2324() {
    // IDA 0xec2324: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel get:]")]
#[doc(alias = "-[GAIModel get:]")]
// 0xec233c — -[GAIModel get:]
// type: id __cdecl(GAIModel *self, SEL, id)
pub fn stub_0xec233c() {
    // IDA 0xec233c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel clearTemporaryValues]")]
#[doc(alias = "-[GAIModel clearTemporaryValues]")]
// 0xec2584 — -[GAIModel clearTemporaryValues]
// type: void __cdecl(GAIModel *self, SEL)
pub fn stub_0xec2584() {
    // IDA 0xec2584: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel paramsWithValues]")]
#[doc(alias = "-[GAIModel paramsWithValues]")]
// 0xec25a8 — -[GAIModel paramsWithValues]
// type: id __cdecl(GAIModel *self, SEL)
pub fn stub_0xec25a8() {
    // IDA 0xec25a8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel metaModel]")]
#[doc(alias = "-[GAIModel metaModel]")]
// 0xec2814 — -[GAIModel metaModel]
// type: GAIMetaModel *__cdecl(GAIModel *self, SEL)
pub fn stub_0xec2814() {
    // IDA 0xec2814: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel values]")]
#[doc(alias = "-[GAIModel values]")]
// 0xec2824 — -[GAIModel values]
// type: NSMutableDictionary *__cdecl(GAIModel *self, SEL)
pub fn stub_0xec2824() {
    // IDA 0xec2824: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel setValues:]")]
#[doc(alias = "-[GAIModel setValues:]")]
// 0xec2834 — -[GAIModel setValues:]
// type: void __cdecl(GAIModel *self, SEL, id)
pub fn stub_0xec2834() {
    // IDA 0xec2834: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel temporaryValues]")]
#[doc(alias = "-[GAIModel temporaryValues]")]
// 0xec2858 — -[GAIModel temporaryValues]
// type: NSMutableDictionary *__cdecl(GAIModel *self, SEL)
pub fn stub_0xec2858() {
    // IDA 0xec2858: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIModel setTemporaryValues:]")]
#[doc(alias = "-[GAIModel setTemporaryValues:]")]
// 0xec2868 — -[GAIModel setTemporaryValues:]
// type: void __cdecl(GAIModel *self, SEL, id)
pub fn stub_0xec2868() {
    // IDA 0xec2868: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIExceptionParser init]")]
#[doc(alias = "-[GAIExceptionParser init]")]
// 0xec288c — -[GAIExceptionParser init]
// type: GAIExceptionParser *__cdecl(GAIExceptionParser *self, SEL)
pub fn stub_0xec288c() {
    // IDA 0xec288c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIExceptionParser symbolFromStackFrame:]")]
#[doc(alias = "+[GAIExceptionParser symbolFromStackFrame:]")]
// 0xec28e8 — +[GAIExceptionParser symbolFromStackFrame:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xec28e8() {
    // IDA 0xec28e8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIHitMetaModel init]")]
#[doc(alias = "-[GAIHitMetaModel init]")]
// 0xec2b84 — -[GAIHitMetaModel init]
// type: GAIHitMetaModel *__cdecl(GAIHitMetaModel *self, SEL)
pub fn stub_0xec2b84() {
    // IDA 0xec2b84: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitMetaModel allocWithZone:]")]
#[doc(alias = "+[GAIHitMetaModel allocWithZone:]")]
// 0xec2d84 — +[GAIHitMetaModel allocWithZone:]
// type: id __cdecl(id, SEL, _NSZone *)
pub fn stub_0xec2d84() {
    // IDA 0xec2d84: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIHitMetaModel retain]")]
#[doc(alias = "-[GAIHitMetaModel retain]")]
// 0xec2e6c — -[GAIHitMetaModel retain]
// type: GAIHitMetaModel *__cdecl(GAIHitMetaModel *self, SEL)
pub fn stub_0xec2e6c() {
    // IDA 0xec2e6c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIHitMetaModel retainCount]")]
#[doc(alias = "-[GAIHitMetaModel retainCount]")]
// 0xec2e70 — -[GAIHitMetaModel retainCount]
// type: unsigned int __cdecl(GAIHitMetaModel *self, SEL)
pub fn stub_0xec2e70() {
    // IDA 0xec2e70: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIHitMetaModel release]")]
#[doc(alias = "-[GAIHitMetaModel release]")]
// 0xec2e78 — -[GAIHitMetaModel release]
// type: void __cdecl(GAIHitMetaModel *self, SEL)
pub fn stub_0xec2e78() {
    // IDA 0xec2e78: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIHitMetaModel autorelease]")]
#[doc(alias = "-[GAIHitMetaModel autorelease]")]
// 0xec2e7c — -[GAIHitMetaModel autorelease]
// type: GAIHitMetaModel *__cdecl(GAIHitMetaModel *self, SEL)
pub fn stub_0xec2e7c() {
    // IDA 0xec2e7c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIHitMetaModel copyWithZone:]")]
#[doc(alias = "-[GAIHitMetaModel copyWithZone:]")]
// 0xec2e80 — -[GAIHitMetaModel copyWithZone:]
// type: id __cdecl(GAIHitMetaModel *self, SEL, _NSZone *)
pub fn stub_0xec2e80() {
    // IDA 0xec2e80: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher optOut]")]
#[doc(alias = "-[GAIDispatcher optOut]")]
// 0xec2e88 — -[GAIDispatcher optOut]
// type: char __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec2e88() {
    // IDA 0xec2e88: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher saveUpdatedOptOut]")]
#[doc(alias = "-[GAIDispatcher saveUpdatedOptOut]")]
// 0xec2eac — -[GAIDispatcher saveUpdatedOptOut]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec2eac() {
    // IDA 0xec2eac: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setOptOut:]")]
#[doc(alias = "-[GAIDispatcher setOptOut:]")]
// 0xec31ac — -[GAIDispatcher setOptOut:]
// type: void __cdecl(GAIDispatcher *self, SEL, char)
pub fn stub_0xec31ac() {
    // IDA 0xec31ac: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC3220")]
#[doc(alias = "sub_EC3220")]
// 0xec3220 — sub_EC3220
pub fn stub_0xec3220() {
    // IDA 0xec3220: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC3238")]
#[doc(alias = "sub_EC3238")]
// 0xec3238 — sub_EC3238
pub fn stub_0xec3238() {
    // IDA 0xec3238: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC3248")]
#[doc(alias = "sub_EC3248")]
// 0xec3248 — sub_EC3248
pub fn stub_0xec3248() {
    // IDA 0xec3248: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher dispatchInterval]")]
#[doc(alias = "-[GAIDispatcher dispatchInterval]")]
// 0xec3258 — -[GAIDispatcher dispatchInterval]
// type: double __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec3258() {
    // IDA 0xec3258: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setDispatchInterval:]")]
#[doc(alias = "-[GAIDispatcher setDispatchInterval:]")]
// 0xec3288 — -[GAIDispatcher setDispatchInterval:]
// type: void __cdecl(GAIDispatcher *self, SEL, double)
pub fn stub_0xec3288() {
    // IDA 0xec3288: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher timerFired:]")]
#[doc(alias = "-[GAIDispatcher timerFired:]")]
// 0xec33d4 — -[GAIDispatcher timerFired:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec33d4() {
    // IDA 0xec33d4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher dispatch]")]
#[doc(alias = "-[GAIDispatcher dispatch]")]
// 0xec350c — -[GAIDispatcher dispatch]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec350c() {
    // IDA 0xec350c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC3560")]
#[doc(alias = "sub_EC3560")]
// 0xec3560 — sub_EC3560
pub fn stub_0xec3560() {
    // IDA 0xec3560: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC357C")]
#[doc(alias = "sub_EC357C")]
// 0xec357c — sub_EC357C
pub fn stub_0xec357c() {
    // IDA 0xec357c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC358C")]
#[doc(alias = "sub_EC358C")]
// 0xec358c — sub_EC358C
pub fn stub_0xec358c() {
    // IDA 0xec358c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher internalCreateTimer]")]
#[doc(alias = "-[GAIDispatcher internalCreateTimer]")]
// 0xec359c — -[GAIDispatcher internalCreateTimer]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec359c() {
    // IDA 0xec359c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher createTimer]")]
#[doc(alias = "-[GAIDispatcher createTimer]")]
// 0xec37b4 — -[GAIDispatcher createTimer]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec37b4() {
    // IDA 0xec37b4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC3908")]
#[doc(alias = "sub_EC3908")]
// 0xec3908 — sub_EC3908
pub fn stub_0xec3908() {
    // IDA 0xec3908: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC3920")]
#[doc(alias = "sub_EC3920")]
// 0xec3920 — sub_EC3920
pub fn stub_0xec3920() {
    // IDA 0xec3920: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC3930")]
#[doc(alias = "sub_EC3930")]
// 0xec3930 — sub_EC3930
pub fn stub_0xec3930() {
    // IDA 0xec3930: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher cancelTimer]")]
#[doc(alias = "-[GAIDispatcher cancelTimer]")]
// 0xec3940 — -[GAIDispatcher cancelTimer]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec3940() {
    // IDA 0xec3940: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher cancelDispatch]")]
#[doc(alias = "-[GAIDispatcher cancelDispatch]")]
// 0xec3bb8 — -[GAIDispatcher cancelDispatch]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec3bb8() {
    // IDA 0xec3bb8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher init]")]
#[doc(alias = "-[GAIDispatcher init]")]
// 0xec3d10 — -[GAIDispatcher init]
// type: GAIDispatcher *__cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec3d10() {
    // IDA 0xec3d10: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher initWithDataStore:withClientId:withOptOut:withNumHits:withDispatchInterval:]")]
#[doc(alias = "-[GAIDispatcher initWithDataStore:withClientId:withOptOut:withNumHits:withDispatchInterval:]")]
// 0xec3d6c — -[GAIDispatcher initWithDataStore:withClientId:withOptOut:withNumHits:withDispatchInterval:]
// type: GAIDispatcher *__cdecl(GAIDispatcher *self, SEL, id, id, char, int, double)
pub fn stub_0xec3d6c() {
    // IDA 0xec3d6c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher dealloc]")]
#[doc(alias = "-[GAIDispatcher dealloc]")]
// 0xec3ee8 — -[GAIDispatcher dealloc]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec3ee8() {
    // IDA 0xec3ee8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher shouldRetryWithError:withHttpStatus:]")]
#[doc(alias = "-[GAIDispatcher shouldRetryWithError:withHttpStatus:]")]
// 0xec3f98 — -[GAIDispatcher shouldRetryWithError:withHttpStatus:]
// type: char __cdecl(GAIDispatcher *self, SEL, id, int)
pub fn stub_0xec3f98() {
    // IDA 0xec3f98: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher reachability:statusChanged:]")]
#[doc(alias = "-[GAIDispatcher reachability:statusChanged:]")]
// 0xec4004 — -[GAIDispatcher reachability:statusChanged:]
// type: void __cdecl(GAIDispatcher *self, SEL, id, int)
pub fn stub_0xec4004() {
    // IDA 0xec4004: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher connectivityLost:]")]
#[doc(alias = "-[GAIDispatcher connectivityLost:]")]
// 0xec40bc — -[GAIDispatcher connectivityLost:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec40bc() {
    // IDA 0xec40bc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher connectivityEstablished]")]
#[doc(alias = "-[GAIDispatcher connectivityEstablished]")]
// 0xec424c — -[GAIDispatcher connectivityEstablished]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec424c() {
    // IDA 0xec424c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher dispatchComplete:withStartTime:withRetryNumber:withResponse:withData:withError:]")]
#[doc(alias = "-[GAIDispatcher dispatchComplete:withStartTime:withRetryNumber:withResponse:withData:withError:]")]
// 0xec4420 — -[GAIDispatcher dispatchComplete:withStartTime:withRetryNumber:withResponse:withData:withError:]
// type: void __cdecl(GAIDispatcher *self, SEL, id, double, int, id, id, id)
pub fn stub_0xec4420() {
    // IDA 0xec4420: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher nextPendingHit]")]
#[doc(alias = "-[GAIDispatcher nextPendingHit]")]
// 0xec4bf8 — -[GAIDispatcher nextPendingHit]
// type: id __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec4bf8() {
    // IDA 0xec4bf8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher initiateDispatch:retryNumber:]")]
#[doc(alias = "-[GAIDispatcher initiateDispatch:retryNumber:]")]
// 0xec4e78 — -[GAIDispatcher initiateDispatch:retryNumber:]
// type: void __cdecl(GAIDispatcher *self, SEL, id, int)
pub fn stub_0xec4e78() {
    // IDA 0xec4e78: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC51DC")]
#[doc(alias = "sub_EC51DC")]
// 0xec51dc — sub_EC51DC
pub fn stub_0xec51dc() {
    // IDA 0xec51dc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC5220")]
#[doc(alias = "sub_EC5220")]
// 0xec5220 — sub_EC5220
pub fn stub_0xec5220() {
    // IDA 0xec5220: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC5244")]
#[doc(alias = "sub_EC5244")]
// 0xec5244 — sub_EC5244
pub fn stub_0xec5244() {
    // IDA 0xec5244: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher persistAndDispatch:url:timestamp:]")]
#[doc(alias = "-[GAIDispatcher persistAndDispatch:url:timestamp:]")]
// 0xec525c — -[GAIDispatcher persistAndDispatch:url:timestamp:]
// type: void __cdecl(GAIDispatcher *self, SEL, id, id, double)
pub fn stub_0xec525c() {
    // IDA 0xec525c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIDispatcher dispatcherWithDatastorePath:withDispatchInterval:withError:]")]
#[doc(alias = "+[GAIDispatcher dispatcherWithDatastorePath:withDispatchInterval:withError:]")]
// 0xec55e4 — +[GAIDispatcher dispatcherWithDatastorePath:withDispatchInterval:withError:]
// type: GAIDispatcher *__cdecl(id, SEL, id, double, id *)
pub fn stub_0xec55e4() {
    // IDA 0xec55e4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher queueDispatch:url:timestamp:]")]
#[doc(alias = "-[GAIDispatcher queueDispatch:url:timestamp:]")]
// 0xec5940 — -[GAIDispatcher queueDispatch:url:timestamp:]
// type: void __cdecl(GAIDispatcher *self, SEL, id, id, double)
pub fn stub_0xec5940() {
    // IDA 0xec5940: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC59CC")]
#[doc(alias = "sub_EC59CC")]
// 0xec59cc — sub_EC59CC
pub fn stub_0xec59cc() {
    // IDA 0xec59cc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC59F8")]
#[doc(alias = "sub_EC59F8")]
// 0xec59f8 — sub_EC59F8
pub fn stub_0xec59f8() {
    // IDA 0xec59f8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC5A28")]
#[doc(alias = "sub_EC5A28")]
// 0xec5a28 — sub_EC5A28
pub fn stub_0xec5a28() {
    // IDA 0xec5a28: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher queueDispatch:url:]")]
#[doc(alias = "-[GAIDispatcher queueDispatch:url:]")]
// 0xec5a48 — -[GAIDispatcher queueDispatch:url:]
// type: void __cdecl(GAIDispatcher *self, SEL, id, id)
pub fn stub_0xec5a48() {
    // IDA 0xec5a48: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC5B1C")]
#[doc(alias = "sub_EC5B1C")]
// 0xec5b1c — sub_EC5B1C
pub fn stub_0xec5b1c() {
    // IDA 0xec5b1c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC5B48")]
#[doc(alias = "sub_EC5B48")]
// 0xec5b48 — sub_EC5B48
pub fn stub_0xec5b48() {
    // IDA 0xec5b48: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EC5B78")]
#[doc(alias = "sub_EC5B78")]
// 0xec5b78 — sub_EC5B78
pub fn stub_0xec5b78() {
    // IDA 0xec5b78: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher wait_and_release]")]
#[doc(alias = "-[GAIDispatcher wait_and_release]")]
// 0xec5b98 — -[GAIDispatcher wait_and_release]
// type: void __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5b98() {
    // IDA 0xec5b98: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher clientId]")]
#[doc(alias = "-[GAIDispatcher clientId]")]
// 0xec5c60 — -[GAIDispatcher clientId]
// type: NSString *__cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5c60() {
    // IDA 0xec5c60: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setClientId:]")]
#[doc(alias = "-[GAIDispatcher setClientId:]")]
// 0xec5c78 — -[GAIDispatcher setClientId:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec5c78() {
    // IDA 0xec5c78: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher dataStore]")]
#[doc(alias = "-[GAIDispatcher dataStore]")]
// 0xec5c9c — -[GAIDispatcher dataStore]
// type: GAIDataStore *__cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5c9c() {
    // IDA 0xec5c9c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setDataStore:]")]
#[doc(alias = "-[GAIDispatcher setDataStore:]")]
// 0xec5cac — -[GAIDispatcher setDataStore:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec5cac() {
    // IDA 0xec5cac: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher requestBuilder]")]
#[doc(alias = "-[GAIDispatcher requestBuilder]")]
// 0xec5cd0 — -[GAIDispatcher requestBuilder]
// type: GAIRequestBuilder *__cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5cd0() {
    // IDA 0xec5cd0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setRequestBuilder:]")]
#[doc(alias = "-[GAIDispatcher setRequestBuilder:]")]
// 0xec5ce0 — -[GAIDispatcher setRequestBuilder:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec5ce0() {
    // IDA 0xec5ce0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher pendingDispatch]")]
#[doc(alias = "-[GAIDispatcher pendingDispatch]")]
// 0xec5d04 — -[GAIDispatcher pendingDispatch]
// type: GAIURLConnection *__cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5d04() {
    // IDA 0xec5d04: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setPendingDispatch:]")]
#[doc(alias = "-[GAIDispatcher setPendingDispatch:]")]
// 0xec5d14 — -[GAIDispatcher setPendingDispatch:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec5d14() {
    // IDA 0xec5d14: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher reachability]")]
#[doc(alias = "-[GAIDispatcher reachability]")]
// 0xec5d38 — -[GAIDispatcher reachability]
// type: GAIReachabilityChecker *__cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5d38() {
    // IDA 0xec5d38: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setReachability:]")]
#[doc(alias = "-[GAIDispatcher setReachability:]")]
// 0xec5d48 — -[GAIDispatcher setReachability:]
// type: void __cdecl(GAIDispatcher *self, SEL, id)
pub fn stub_0xec5d48() {
    // IDA 0xec5d48: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher cacheBuster]")]
#[doc(alias = "-[GAIDispatcher cacheBuster]")]
// 0xec5d6c — -[GAIDispatcher cacheBuster]
// type: unsigned __int64 __cdecl(GAIDispatcher *self, SEL)
pub fn stub_0xec5d6c() {
    // IDA 0xec5d6c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIDispatcher setCacheBuster:]")]
#[doc(alias = "-[GAIDispatcher setCacheBuster:]")]
// 0xec5d84 — -[GAIDispatcher setCacheBuster:]
// type: void __cdecl(GAIDispatcher *self, SEL, unsigned __int64)
pub fn stub_0xec5d84() {
    // IDA 0xec5d84: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
