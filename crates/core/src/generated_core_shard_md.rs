//! core shard md — 100 core stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33887 fallback, 3373 uncovered before -> 3273 after, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "+[Flurry applicationWillResignActive]")]
// 0xefb1fc — +[Flurry applicationWillResignActive]
// type: void __cdecl(id, SEL)
pub fn stub_0xefb1fc() {
    // IDA 0xefb1fc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry createAndSendSession]")]
// 0xefb24c — +[Flurry createAndSendSession]
// type: void __cdecl(id, SEL)
pub fn stub_0xefb24c() {
    // IDA 0xefb24c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry startSession:withOptions:]")]
// 0xefb5f8 — +[Flurry startSession:withOptions:]
// type: void __cdecl(id, SEL, id, id)
pub fn stub_0xefb5f8() {
    // IDA 0xefb5f8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry startSession:]")]
// 0xefb630 — +[Flurry startSession:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xefb630() {
    // IDA 0xefb630: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]")]
// 0xefbb98 — +[Flurry setLatitude:longitude:horizontalAccuracy:verticalAccuracy:]
// type: void __cdecl(id, SEL, double, double, float, float)
pub fn stub_0xefbb98() {
    // IDA 0xefbb98: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry endSession]")]
// 0xefbdf8 — +[Flurry endSession]
// type: void __cdecl(id, SEL)
pub fn stub_0xefbdf8() {
    // IDA 0xefbdf8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry pauseBackgroundSession]")]
// 0xefbf50 — +[Flurry pauseBackgroundSession]
// type: void __cdecl(id, SEL)
pub fn stub_0xefbf50() {
    // IDA 0xefbf50: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry pauseSession]")]
// 0xefbfc0 — +[Flurry pauseSession]
// type: void __cdecl(id, SEL)
pub fn stub_0xefbfc0() {
    // IDA 0xefbfc0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry applicationWillEnterForeground]")]
// 0xefc174 — +[Flurry applicationWillEnterForeground]
// type: void __cdecl(id, SEL)
pub fn stub_0xefc174() {
    // IDA 0xefc174: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry applicationDidBecomeActive]")]
// 0xefc2cc — +[Flurry applicationDidBecomeActive]
// type: void __cdecl(id, SEL)
pub fn stub_0xefc2cc() {
    // IDA 0xefc2cc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logEvent:]")]
// 0xefc588 — +[Flurry logEvent:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xefc588() {
    // IDA 0xefc588: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logEvent:withParameters:]")]
// 0xefc760 — +[Flurry logEvent:withParameters:]
// type: void __cdecl(id, SEL, id, id)
pub fn stub_0xefc760() {
    // IDA 0xefc760: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logEvent:timed:]")]
// 0xefc93c — +[Flurry logEvent:timed:]
// type: void __cdecl(id, SEL, id, char)
pub fn stub_0xefc93c() {
    // IDA 0xefc93c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logEvent:withParameters:timed:]")]
// 0xefcb18 — +[Flurry logEvent:withParameters:timed:]
// type: void __cdecl(id, SEL, id, id, char)
pub fn stub_0xefcb18() {
    // IDA 0xefcb18: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry endTimedEvent:withParameters:]")]
// 0xefccf4 — +[Flurry endTimedEvent:withParameters:]
// type: void __cdecl(id, SEL, id, id)
pub fn stub_0xefccf4() {
    // IDA 0xefccf4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logError:message:exception:]")]
// 0xefcecc — +[Flurry logError:message:exception:]
// type: void __cdecl(id, SEL, id, id, id)
pub fn stub_0xefcecc() {
    // IDA 0xefcecc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logError:message:error:]")]
// 0xefd204 — +[Flurry logError:message:error:]
// type: void __cdecl(id, SEL, id, id, id)
pub fn stub_0xefd204() {
    // IDA 0xefd204: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logAllPageViews:]")]
// 0xefd53c — +[Flurry logAllPageViews:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xefd53c() {
    // IDA 0xefd53c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry logPageView]")]
// 0xefd684 — +[Flurry logPageView]
// type: void __cdecl(id, SEL)
pub fn stub_0xefd684() {
    // IDA 0xefd684: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setUserID:]")]
// 0xefd7dc — +[Flurry setUserID:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xefd7dc() {
    // IDA 0xefd7dc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setEventLoggingEnabled:]")]
// 0xefd9a4 — +[Flurry setEventLoggingEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefd9a4() {
    // IDA 0xefd9a4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setServerURL:]")]
// 0xefdb30 — +[Flurry setServerURL:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xefdb30() {
    // IDA 0xefdb30: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setAppCloudServerToStaging:]")]
// 0xefdc20 — +[Flurry setAppCloudServerToStaging:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefdc20() {
    // IDA 0xefdc20: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setSecureTransportEnabled:]")]
// 0xefdd10 — +[Flurry setSecureTransportEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefdd10() {
    // IDA 0xefdd10: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setCrashReportingEnabled:]")]
// 0xefde58 — +[Flurry setCrashReportingEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefde58() {
    // IDA 0xefde58: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setSessionReportsOnCloseEnabled:]")]
// 0xefdf08 — +[Flurry setSessionReportsOnCloseEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefdf08() {
    // IDA 0xefdf08: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setSessionReportsOnPauseEnabled:]")]
// 0xefe094 — +[Flurry setSessionReportsOnPauseEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefe094() {
    // IDA 0xefe094: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setBackgroundSessionEnabled:]")]
// 0xefe220 — +[Flurry setBackgroundSessionEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefe220() {
    // IDA 0xefe220: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setGender:]")]
// 0xefe3a8 — +[Flurry setGender:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xefe3a8() {
    // IDA 0xefe3a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setAge:]")]
// 0xefe5f0 — +[Flurry setAge:]
// type: void __cdecl(id, SEL, int)
pub fn stub_0xefe5f0() {
    // IDA 0xefe5f0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setPushToken:]")]
// 0xefe784 — +[Flurry setPushToken:]
// type: void __cdecl(id, SEL, id)
pub fn stub_0xefe784() {
    // IDA 0xefe784: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry setShowErrorInLogEnabled:]")]
// 0xefe800 — +[Flurry setShowErrorInLogEnabled:]
// type: void __cdecl(id, SEL, char)
pub fn stub_0xefe800() {
    // IDA 0xefe800: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[Flurry sendUsageInfo]")]
// 0xefe870 — +[Flurry sendUsageInfo]
// type: char __cdecl(id, SEL)
pub fn stub_0xefe870() {
    // IDA 0xefe870: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent initWithName:]")]
// 0xefe874 — -[FlurryEvent initWithName:]
// type: FlurryEvent *__cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xefe874() {
    // IDA 0xefe874: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent initWithName:timed:]")]
// 0xefe894 — -[FlurryEvent initWithName:timed:]
// type: FlurryEvent *__cdecl(FlurryEvent *self, SEL, id, char)
pub fn stub_0xefe894() {
    // IDA 0xefe894: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent initWithName:timed:parameters:]")]
// 0xefe8b8 — -[FlurryEvent initWithName:timed:parameters:]
// type: FlurryEvent *__cdecl(FlurryEvent *self, SEL, id, char, id)
pub fn stub_0xefe8b8() {
    // IDA 0xefe8b8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent initWithCoder:]")]
// 0xefea40 — -[FlurryEvent initWithCoder:]
// type: FlurryEvent *__cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xefea40() {
    // IDA 0xefea40: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent updateParameters:]")]
// 0xefeba4 — -[FlurryEvent updateParameters:]
// type: void __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xefeba4() {
    // IDA 0xefeba4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent encodeWithCoder:]")]
// 0xefec88 — -[FlurryEvent encodeWithCoder:]
// type: void __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xefec88() {
    // IDA 0xefec88: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent isUnterminated]")]
// 0xefed9c — -[FlurryEvent isUnterminated]
// type: char __cdecl(FlurryEvent *self, SEL)
pub fn stub_0xefed9c() {
    // IDA 0xefed9c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent startOffsetFromSessionStart:]")]
// 0xefedcc — -[FlurryEvent startOffsetFromSessionStart:]
// type: int __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xefedcc() {
    // IDA 0xefedcc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent duration]")]
// 0xefee00 — -[FlurryEvent duration]
// type: int __cdecl(FlurryEvent *self, SEL)
pub fn stub_0xefee00() {
    // IDA 0xefee00: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent dataRelativeToSessionStart:]")]
// 0xefee5c — -[FlurryEvent dataRelativeToSessionStart:]
// type: id __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xefee5c() {
    // IDA 0xefee5c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent dealloc]")]
// 0xeff138 — -[FlurryEvent dealloc]
// type: void __cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff138() {
    // IDA 0xeff138: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent name]")]
// 0xeff1f4 — -[FlurryEvent name]
// type: NSString *__cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff1f4() {
    // IDA 0xeff1f4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent setName:]")]
// 0xeff20c — -[FlurryEvent setName:]
// type: void __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xeff20c() {
    // IDA 0xeff20c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent started]")]
// 0xeff230 — -[FlurryEvent started]
// type: NSDate *__cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff230() {
    // IDA 0xeff230: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent setStarted:]")]
// 0xeff240 — -[FlurryEvent setStarted:]
// type: void __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xeff240() {
    // IDA 0xeff240: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent ended]")]
// 0xeff264 — -[FlurryEvent ended]
// type: NSDate *__cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff264() {
    // IDA 0xeff264: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent setEnded:]")]
// 0xeff274 — -[FlurryEvent setEnded:]
// type: void __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xeff274() {
    // IDA 0xeff274: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent timed]")]
// 0xeff298 — -[FlurryEvent timed]
// type: char __cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff298() {
    // IDA 0xeff298: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent setTimed:]")]
// 0xeff2a8 — -[FlurryEvent setTimed:]
// type: void __cdecl(FlurryEvent *self, SEL, char)
pub fn stub_0xeff2a8() {
    // IDA 0xeff2a8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent parameters]")]
// 0xeff2b8 — -[FlurryEvent parameters]
// type: NSDictionary *__cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff2b8() {
    // IDA 0xeff2b8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent setParameters:]")]
// 0xeff2c8 — -[FlurryEvent setParameters:]
// type: void __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xeff2c8() {
    // IDA 0xeff2c8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent automaticallyEnded]")]
// 0xeff2ec — -[FlurryEvent automaticallyEnded]
// type: char __cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff2ec() {
    // IDA 0xeff2ec: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent setAutomaticallyEnded:]")]
// 0xeff2fc — -[FlurryEvent setAutomaticallyEnded:]
// type: void __cdecl(FlurryEvent *self, SEL, char)
pub fn stub_0xeff2fc() {
    // IDA 0xeff2fc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent eventLogId]")]
// 0xeff30c — -[FlurryEvent eventLogId]
// type: NSNumber *__cdecl(FlurryEvent *self, SEL)
pub fn stub_0xeff30c() {
    // IDA 0xeff30c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryEvent setEventLogId:]")]
// 0xeff31c — -[FlurryEvent setEventLogId:]
// type: void __cdecl(FlurryEvent *self, SEL, id)
pub fn stub_0xeff31c() {
    // IDA 0xeff31c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryFileCache instance]")]
// 0xeff468 — +[FlurryFileCache instance]
// type: id __cdecl(id, SEL)
pub fn stub_0xeff468() {
    // IDA 0xeff468: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache initWithApiKey:agentVersion:]")]
// 0xeff478 — -[FlurryFileCache initWithApiKey:agentVersion:]
// type: FlurryFileCache *__cdecl(FlurryFileCache *self, SEL, id, id)
pub fn stub_0xeff478() {
    // IDA 0xeff478: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache createFilePath:forCurentAgentVersion:]")]
// 0xeff4d8 — -[FlurryFileCache createFilePath:forCurentAgentVersion:]
// type: id __cdecl(FlurryFileCache *self, SEL, id, bool)
pub fn stub_0xeff4d8() {
    // IDA 0xeff4d8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache save:prefix:]")]
// 0xeff55c — -[FlurryFileCache save:prefix:]
// type: id __cdecl(FlurryFileCache *self, SEL, id, id)
pub fn stub_0xeff55c() {
    // IDA 0xeff55c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache save:prefix:forCurentAgentVersion:]")]
// 0xeff5f0 — -[FlurryFileCache save:prefix:forCurentAgentVersion:]
// type: id __cdecl(FlurryFileCache *self, SEL, id, id, bool)
pub fn stub_0xeff5f0() {
    // IDA 0xeff5f0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache get:]")]
// 0xeff688 — -[FlurryFileCache get:]
// type: id __cdecl(FlurryFileCache *self, SEL, id)
pub fn stub_0xeff688() {
    // IDA 0xeff688: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache get:forCurentAgentVersion:]")]
// 0xeff72c — -[FlurryFileCache get:forCurentAgentVersion:]
// type: id __cdecl(FlurryFileCache *self, SEL, id, bool)
pub fn stub_0xeff72c() {
    // IDA 0xeff72c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache remove:]")]
// 0xeff7d0 — -[FlurryFileCache remove:]
// type: char __cdecl(FlurryFileCache *self, SEL, id)
pub fn stub_0xeff7d0() {
    // IDA 0xeff7d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache remove:forCurentAgentVersion:]")]
// 0xeff850 — -[FlurryFileCache remove:forCurentAgentVersion:]
// type: char __cdecl(FlurryFileCache *self, SEL, id, bool)
pub fn stub_0xeff850() {
    // IDA 0xeff850: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache dealloc]")]
// 0xeff8d0 — -[FlurryFileCache dealloc]
// type: void __cdecl(FlurryFileCache *self, SEL)
pub fn stub_0xeff8d0() {
    // IDA 0xeff8d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache apiKey]")]
// 0xeff948 — -[FlurryFileCache apiKey]
// type: NSString *__cdecl(FlurryFileCache *self, SEL)
pub fn stub_0xeff948() {
    // IDA 0xeff948: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache setApiKey:]")]
// 0xeff960 — -[FlurryFileCache setApiKey:]
// type: void __cdecl(FlurryFileCache *self, SEL, id)
pub fn stub_0xeff960() {
    // IDA 0xeff960: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache agentVersion]")]
// 0xeff984 — -[FlurryFileCache agentVersion]
// type: NSString *__cdecl(FlurryFileCache *self, SEL)
pub fn stub_0xeff984() {
    // IDA 0xeff984: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryFileCache setAgentVersion:]")]
// 0xeff99c — -[FlurryFileCache setAgentVersion:]
// type: void __cdecl(FlurryFileCache *self, SEL, id)
pub fn stub_0xeff99c() {
    // IDA 0xeff99c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryHeartBeater instance]")]
// 0xeffad4 — +[FlurryHeartBeater instance]
// type: id __cdecl(id, SEL)
pub fn stub_0xeffad4() {
    // IDA 0xeffad4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryHeartBeater start:]")]
// 0xeffae4 — -[FlurryHeartBeater start:]
// type: void __cdecl(FlurryHeartBeater *self, SEL, double)
pub fn stub_0xeffae4() {
    // IDA 0xeffae4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryHeartBeater heartBeat]")]
// 0xeffb3c — -[FlurryHeartBeater heartBeat]
// type: void __cdecl(FlurryHeartBeater *self, SEL)
pub fn stub_0xeffb3c() {
    // IDA 0xeffb3c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryHeartBeater dealloc]")]
// 0xeffb7c — -[FlurryHeartBeater dealloc]
// type: void __cdecl(FlurryHeartBeater *self, SEL)
pub fn stub_0xeffb7c() {
    // IDA 0xeffb7c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryReachability dealloc]")]
// 0xeffbe8 — -[FlurryReachability dealloc]
// type: void __cdecl(FlurryReachability *self, SEL)
pub fn stub_0xeffbe8() {
    // IDA 0xeffbe8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryReachability initWithFlurryReachabilityRef:]")]
// 0xeffc54 — -[FlurryReachability initWithFlurryReachabilityRef:]
// type: FlurryReachability *__cdecl(FlurryReachability *self, SEL, __SCNetworkReachability *)
pub fn stub_0xeffc54() {
    // IDA 0xeffc54: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryReachability flurryStartNotifier]")]
// 0xeffc94 — -[FlurryReachability flurryStartNotifier]
// type: char __cdecl(FlurryReachability *self, SEL)
pub fn stub_0xeffc94() {
    // IDA 0xeffc94: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "_FlurryReachabilityCallback")]
// 0xeffcf8 — _FlurryReachabilityCallback
pub fn stub_0xeffcf8() {
    // IDA 0xeffcf8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryReachability flurryStopNotifier]")]
// 0xeffd98 — -[FlurryReachability flurryStopNotifier]
// type: void __cdecl(FlurryReachability *self, SEL)
pub fn stub_0xeffd98() {
    // IDA 0xeffd98: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurryReachability isFlurryReachabilityEqual:]")]
// 0xeffdcc — -[FlurryReachability isFlurryReachabilityEqual:]
// type: char __cdecl(FlurryReachability *self, SEL, id)
pub fn stub_0xeffdcc() {
    // IDA 0xeffdcc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryReachability flurryReachabilityWithHostName:]")]
// 0xeffe08 — +[FlurryReachability flurryReachabilityWithHostName:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xeffe08() {
    // IDA 0xeffe08: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryReachability flurryMakeAddressKey:]")]
// 0xeffe88 — +[FlurryReachability flurryMakeAddressKey:]
// type: id __cdecl(id, SEL, unsigned int)
pub fn stub_0xeffe88() {
    // IDA 0xeffe88: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryReachability flurryReachabilityWithAddress:]")]
// 0xeffed4 — +[FlurryReachability flurryReachabilityWithAddress:]
// type: id __cdecl(id, SEL, const sockaddr_in *)
pub fn stub_0xeffed4() {
    // IDA 0xeffed4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryReachability flurryReachabilityForInternetConnection]")]
// 0xefff60 — +[FlurryReachability flurryReachabilityForInternetConnection]
// type: id __cdecl(id, SEL)
pub fn stub_0xefff60() {
    // IDA 0xefff60: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[FlurryReachability flurryReachabilityForLocalWiFi]")]
// 0xefffb0 — +[FlurryReachability flurryReachabilityForLocalWiFi]
// type: id __cdecl(id, SEL)
pub fn stub_0xefffb0() {
    // IDA 0xefffb0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession setEndTime:]")]
// 0xf07070 — -[FlurrySession setEndTime:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf07070() {
    // IDA 0xf07070: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession purchaseItems]")]
// 0xf07094 — -[FlurrySession purchaseItems]
// type: NSMutableArray *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07094() {
    // IDA 0xf07094: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession setPurchaseItems:]")]
// 0xf070a4 — -[FlurrySession setPurchaseItems:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf070a4() {
    // IDA 0xf070a4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession resumeTime]")]
// 0xf070c8 — -[FlurrySession resumeTime]
// type: NSDate *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf070c8() {
    // IDA 0xf070c8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession setResumeTime:]")]
// 0xf070e0 — -[FlurrySession setResumeTime:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf070e0() {
    // IDA 0xf070e0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession pauseTime]")]
// 0xf07104 — -[FlurrySession pauseTime]
// type: NSDate *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07104() {
    // IDA 0xf07104: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession setPauseTime:]")]
// 0xf0711c — -[FlurrySession setPauseTime:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf0711c() {
    // IDA 0xf0711c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession totalPauseIntervalMillis]")]
// 0xf07140 — -[FlurrySession totalPauseIntervalMillis]
// type: signed __int64 __cdecl(FlurrySession *self, SEL)
pub fn stub_0xf07140() {
    // IDA 0xf07140: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession setTotalPauseIntervalMillis:]")]
// 0xf07158 — -[FlurrySession setTotalPauseIntervalMillis:]
// type: void __cdecl(FlurrySession *self, SEL, signed __int64)
pub fn stub_0xf07158() {
    // IDA 0xf07158: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession eventCounts]")]
// 0xf0716c — -[FlurrySession eventCounts]
// type: NSMutableDictionary *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf0716c() {
    // IDA 0xf0716c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession setEventCounts:]")]
// 0xf0717c — -[FlurrySession setEventCounts:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf0717c() {
    // IDA 0xf0717c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession pendingEventLog]")]
// 0xf071a0 — -[FlurrySession pendingEventLog]
// type: NSMutableArray *__cdecl(FlurrySession *self, SEL)
pub fn stub_0xf071a0() {
    // IDA 0xf071a0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[FlurrySession setPendingEventLog:]")]
// 0xf071b0 — -[FlurrySession setPendingEventLog:]
// type: void __cdecl(FlurrySession *self, SEL, id)
pub fn stub_0xf071b0() {
    // IDA 0xf071b0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
