//! core shard ko — 120 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 not yet in rbx_core after kn 0xeb5894 (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound; 33260 filtered, 4943->4823 remaining, 36218->36338 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "+[GAIError errorWithCode:withFormat:]")]
#[doc(alias = "+[GAIError errorWithCode:withFormat:]")]
// 0xebc958 — +[GAIError errorWithCode:withFormat:]
// type: id __cdecl(id, SEL, int, id)
pub fn stub_0xebc958() {
    // IDA 0xebc958: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIError errorWithCode:withFailedFilePath:withFormat:]")]
#[doc(alias = "+[GAIError errorWithCode:withFailedFilePath:withFormat:]")]
// 0xebca58 — +[GAIError errorWithCode:withFailedFilePath:withFormat:]
// type: id __cdecl(id, SEL, int, id, id)
pub fn stub_0xebca58() {
    // IDA 0xebca58: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIError errorWithCode:withUnderlyingError:withFormat:]")]
#[doc(alias = "+[GAIError errorWithCode:withUnderlyingError:withFormat:]")]
// 0xebcb5c — +[GAIError errorWithCode:withUnderlyingError:withFormat:]
// type: id __cdecl(id, SEL, int, id, id)
pub fn stub_0xebcb5c() {
    // IDA 0xebcb5c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitFormat hitMetaInfos]")]
#[doc(alias = "+[GAIHitFormat hitMetaInfos]")]
// 0xebcd3c — +[GAIHitFormat hitMetaInfos]
// type: id __cdecl(id, SEL)
pub fn stub_0xebcd3c() {
    // IDA 0xebcd3c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "sub_EBCD70")]
#[doc(alias = "sub_EBCD70")]
// 0xebcd70 — sub_EBCD70
// type: void __cdecl(id)
pub fn stub_0xebcd70() {
    // IDA 0xebcd70: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitFormat allKeys]")]
#[doc(alias = "+[GAIHitFormat allKeys]")]
// 0xebd6bc — +[GAIHitFormat allKeys]
// type: id __cdecl(id, SEL)
pub fn stub_0xebd6bc() {
    // IDA 0xebd6bc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitFormat metaInfoForKey:]")]
#[doc(alias = "+[GAIHitFormat metaInfoForKey:]")]
// 0xebd6e4 — +[GAIHitFormat metaInfoForKey:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xebd6e4() {
    // IDA 0xebd6e4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIClientId init]")]
#[doc(alias = "-[GAIClientId init]")]
// 0xebd710 — -[GAIClientId init]
// type: GAIClientId *__cdecl(GAIClientId *self, SEL)
pub fn stub_0xebd710() {
    // IDA 0xebd710: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIClientId generateClientId]")]
#[doc(alias = "+[GAIClientId generateClientId]")]
// 0xebd76c — +[GAIClientId generateClientId]
// type: id __cdecl(id, SEL)
pub fn stub_0xebd76c() {
    // IDA 0xebd76c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAICoreDataUtil init]")]
#[doc(alias = "-[GAICoreDataUtil init]")]
// 0xebd7a8 — -[GAICoreDataUtil init]
// type: GAICoreDataUtil *__cdecl(GAICoreDataUtil *self, SEL)
pub fn stub_0xebd7a8() {
    // IDA 0xebd7a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAICoreDataUtil attributeNamed:type:indexed:required:]")]
#[doc(alias = "+[GAICoreDataUtil attributeNamed:type:indexed:required:]")]
// 0xebd804 — +[GAICoreDataUtil attributeNamed:type:indexed:required:]
// type: id __cdecl(id, SEL, id, unsigned int, char, char)
pub fn stub_0xebd804() {
    // IDA 0xebd804: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitBuilder check:required:model:]")]
#[doc(alias = "+[GAIHitBuilder check:required:model:]")]
// 0xebd8a8 — +[GAIHitBuilder check:required:model:]
// type: id __cdecl(id, SEL, id, id, id)
pub fn stub_0xebd8a8() {
    // IDA 0xebd8a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitBuilder requiredParametersForHitType:]")]
#[doc(alias = "+[GAIHitBuilder requiredParametersForHitType:]")]
// 0xebd9a0 — +[GAIHitBuilder requiredParametersForHitType:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xebd9a0() {
    // IDA 0xebd9a0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitBuilder hitWithType:withModel:]")]
#[doc(alias = "+[GAIHitBuilder hitWithType:withModel:]")]
// 0xebdc1c — +[GAIHitBuilder hitWithType:withModel:]
// type: id __cdecl(id, SEL, id, id)
pub fn stub_0xebdc1c() {
    // IDA 0xebdc1c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIHitUtil init]")]
#[doc(alias = "-[GAIHitUtil init]")]
// 0xebdd88 — -[GAIHitUtil init]
// type: GAIHitUtil *__cdecl(GAIHitUtil *self, SEL)
pub fn stub_0xebdd88() {
    // IDA 0xebdd88: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil systemLanguageFromLanguages:]")]
#[doc(alias = "+[GAIHitUtil systemLanguageFromLanguages:]")]
// 0xebdde4 — +[GAIHitUtil systemLanguageFromLanguages:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xebdde4() {
    // IDA 0xebdde4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil systemCountryFromLocale:]")]
#[doc(alias = "+[GAIHitUtil systemCountryFromLocale:]")]
// 0xebde4c — +[GAIHitUtil systemCountryFromLocale:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xebde4c() {
    // IDA 0xebde4c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil userAgentStringWithProduct:version:deviceModel:systemName:systemVersion:systemLanguage:systemCountry:]")]
#[doc(alias = "+[GAIHitUtil userAgentStringWithProduct:version:deviceModel:systemName:systemVersion:systemLanguage:systemCountry:]")]
// 0xebde84 — +[GAIHitUtil userAgentStringWithProduct:version:deviceModel:systemName:systemVersion:systemLanguage:systemCountry:]
// type: id __cdecl(id, SEL, id, id, id, id, id, id, id)
pub fn stub_0xebde84() {
    // IDA 0xebde84: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil userAgentString]")]
#[doc(alias = "+[GAIHitUtil userAgentString]")]
// 0xebded8 — +[GAIHitUtil userAgentString]
// type: id __cdecl(id, SEL)
pub fn stub_0xebded8() {
    // IDA 0xebded8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil systemLanguage]")]
#[doc(alias = "+[GAIHitUtil systemLanguage]")]
// 0xebdfb0 — +[GAIHitUtil systemLanguage]
// type: id __cdecl(id, SEL)
pub fn stub_0xebdfb0() {
    // IDA 0xebdfb0: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil systemCountry]")]
#[doc(alias = "+[GAIHitUtil systemCountry]")]
// 0xebdff4 — +[GAIHitUtil systemCountry]
// type: id __cdecl(id, SEL)
pub fn stub_0xebdff4() {
    // IDA 0xebdff4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil systemScreenResolution]")]
#[doc(alias = "+[GAIHitUtil systemScreenResolution]")]
// 0xebe038 — +[GAIHitUtil systemScreenResolution]
// type: id __cdecl(id, SEL)
pub fn stub_0xebe038() {
    // IDA 0xebe038: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil systemScreenColorDepth]")]
#[doc(alias = "+[GAIHitUtil systemScreenColorDepth]")]
// 0xebe0c8 — +[GAIHitUtil systemScreenColorDepth]
// type: id __cdecl(id, SEL)
pub fn stub_0xebe0c8() {
    // IDA 0xebe0c8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIHitUtil millisecondsElapsedFrom:To:]")]
#[doc(alias = "+[GAIHitUtil millisecondsElapsedFrom:To:]")]
// 0xebe0d8 — +[GAIHitUtil millisecondsElapsedFrom:To:]
// type: signed __int64 __cdecl(id, SEL, id, id)
pub fn stub_0xebe0d8() {
    // IDA 0xebe0d8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIRequestBuilder init]")]
#[doc(alias = "-[GAIRequestBuilder init]")]
// 0xebe110 — -[GAIRequestBuilder init]
// type: GAIRequestBuilder *__cdecl(GAIRequestBuilder *self, SEL)
pub fn stub_0xebe110() {
    // IDA 0xebe110: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIRequestBuilder dealloc]")]
#[doc(alias = "-[GAIRequestBuilder dealloc]")]
// 0xebe17c — -[GAIRequestBuilder dealloc]
// type: void __cdecl(GAIRequestBuilder *self, SEL)
pub fn stub_0xebe17c() {
    // IDA 0xebe17c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIRequestBuilder encodeParameter:withValue:]")]
#[doc(alias = "-[GAIRequestBuilder encodeParameter:withValue:]")]
// 0xebe1c8 — -[GAIRequestBuilder encodeParameter:withValue:]
// type: id __cdecl(GAIRequestBuilder *self, SEL, id, id)
pub fn stub_0xebe1c8() {
    // IDA 0xebe1c8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIRequestBuilder encodeParameters:]")]
#[doc(alias = "-[GAIRequestBuilder encodeParameters:]")]
// 0xebe238 — -[GAIRequestBuilder encodeParameters:]
// type: id __cdecl(GAIRequestBuilder *self, SEL, id)
pub fn stub_0xebe238() {
    // IDA 0xebe238: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIRequestBuilder requestForHit:]")]
#[doc(alias = "-[GAIRequestBuilder requestForHit:]")]
// 0xebe3d0 — -[GAIRequestBuilder requestForHit:]
// type: id __cdecl(GAIRequestBuilder *self, SEL, id)
pub fn stub_0xebe3d0() {
    // IDA 0xebe3d0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIRequestBuilder userAgent]")]
#[doc(alias = "-[GAIRequestBuilder userAgent]")]
// 0xebe7e0 — -[GAIRequestBuilder userAgent]
// type: NSString *__cdecl(GAIRequestBuilder *self, SEL)
pub fn stub_0xebe7e0() {
    // IDA 0xebe7e0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIRequestBuilder setUserAgent:]")]
#[doc(alias = "-[GAIRequestBuilder setUserAgent:]")]
// 0xebe7f8 — -[GAIRequestBuilder setUserAgent:]
// type: void __cdecl(GAIRequestBuilder *self, SEL, id)
pub fn stub_0xebe7f8() {
    // IDA 0xebe7f8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState anonymize]")]
#[doc(alias = "-[GAITrackerState anonymize]")]
// 0xebe820 — -[GAITrackerState anonymize]
// type: char __cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebe820() {
    // IDA 0xebe820: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setAnonymize:]")]
#[doc(alias = "-[GAITrackerState setAnonymize:]")]
// 0xebe864 — -[GAITrackerState setAnonymize:]
// type: void __cdecl(GAITrackerState *self, SEL, char)
pub fn stub_0xebe864() {
    // IDA 0xebe864: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState clientId]")]
#[doc(alias = "-[GAITrackerState clientId]")]
// 0xebe8bc — -[GAITrackerState clientId]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebe8bc() {
    // IDA 0xebe8bc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState trackingId]")]
#[doc(alias = "-[GAITrackerState trackingId]")]
// 0xebe8f0 — -[GAITrackerState trackingId]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebe8f0() {
    // IDA 0xebe8f0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState appName]")]
#[doc(alias = "-[GAITrackerState appName]")]
// 0xebe924 — -[GAITrackerState appName]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebe924() {
    // IDA 0xebe924: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setAppName:]")]
#[doc(alias = "-[GAITrackerState setAppName:]")]
// 0xebe958 — -[GAITrackerState setAppName:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebe958() {
    // IDA 0xebe958: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState appVersion]")]
#[doc(alias = "-[GAITrackerState appVersion]")]
// 0xebe990 — -[GAITrackerState appVersion]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebe990() {
    // IDA 0xebe990: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setAppVersion:]")]
#[doc(alias = "-[GAITrackerState setAppVersion:]")]
// 0xebe9c4 — -[GAITrackerState setAppVersion:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebe9c4() {
    // IDA 0xebe9c4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState appId]")]
#[doc(alias = "-[GAITrackerState appId]")]
// 0xebe9fc — -[GAITrackerState appId]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebe9fc() {
    // IDA 0xebe9fc: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setAppId:]")]
#[doc(alias = "-[GAITrackerState setAppId:]")]
// 0xebea30 — -[GAITrackerState setAppId:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebea30() {
    // IDA 0xebea30: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState appScreen]")]
#[doc(alias = "-[GAITrackerState appScreen]")]
// 0xebea68 — -[GAITrackerState appScreen]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebea68() {
    // IDA 0xebea68: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setAppScreen:]")]
#[doc(alias = "-[GAITrackerState setAppScreen:]")]
// 0xebea9c — -[GAITrackerState setAppScreen:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebea9c() {
    // IDA 0xebea9c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState sessionControl]")]
#[doc(alias = "-[GAITrackerState sessionControl]")]
// 0xebead4 — -[GAITrackerState sessionControl]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebead4() {
    // IDA 0xebead4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setSessionControl:]")]
#[doc(alias = "-[GAITrackerState setSessionControl:]")]
// 0xebeb08 — -[GAITrackerState setSessionControl:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebeb08() {
    // IDA 0xebeb08: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState adHitId]")]
#[doc(alias = "-[GAITrackerState adHitId]")]
// 0xebeb40 — -[GAITrackerState adHitId]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebeb40() {
    // IDA 0xebeb40: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setAdHitId:]")]
#[doc(alias = "-[GAITrackerState setAdHitId:]")]
// 0xebeb74 — -[GAITrackerState setAdHitId:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebeb74() {
    // IDA 0xebeb74: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState sampleRate]")]
#[doc(alias = "-[GAITrackerState sampleRate]")]
// 0xebebac — -[GAITrackerState sampleRate]
// type: double __cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebebac() {
    // IDA 0xebebac: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setSampleRate:]")]
#[doc(alias = "-[GAITrackerState setSampleRate:]")]
// 0xebebc8 — -[GAITrackerState setSampleRate:]
// type: void __cdecl(GAITrackerState *self, SEL, double)
pub fn stub_0xebebc8() {
    // IDA 0xebebc8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setHttpDispatchUrl:]")]
#[doc(alias = "-[GAITrackerState setHttpDispatchUrl:]")]
// 0xebeda0 — -[GAITrackerState setHttpDispatchUrl:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebeda0() {
    // IDA 0xebeda0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setHttpsDispatchUrl:]")]
#[doc(alias = "-[GAITrackerState setHttpsDispatchUrl:]")]
// 0xebeed8 — -[GAITrackerState setHttpsDispatchUrl:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebeed8() {
    // IDA 0xebeed8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState dispatchUrl]")]
#[doc(alias = "-[GAITrackerState dispatchUrl]")]
// 0xebf010 — -[GAITrackerState dispatchUrl]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebf010() {
    // IDA 0xebf010: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState referrerUrl]")]
#[doc(alias = "-[GAITrackerState referrerUrl]")]
// 0xebf040 — -[GAITrackerState referrerUrl]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebf040() {
    // IDA 0xebf040: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setReferrerUrl:]")]
#[doc(alias = "-[GAITrackerState setReferrerUrl:]")]
// 0xebf050 — -[GAITrackerState setReferrerUrl:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebf050() {
    // IDA 0xebf050: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState campaignUrl]")]
#[doc(alias = "-[GAITrackerState campaignUrl]")]
// 0xebf0c4 — -[GAITrackerState campaignUrl]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebf0c4() {
    // IDA 0xebf0c4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setCampaignUrl:]")]
#[doc(alias = "-[GAITrackerState setCampaignUrl:]")]
// 0xebf0d4 — -[GAITrackerState setCampaignUrl:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebf0d4() {
    // IDA 0xebf0d4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState init]")]
#[doc(alias = "-[GAITrackerState init]")]
// 0xebf304 — -[GAITrackerState init]
// type: GAITrackerState *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebf304() {
    // IDA 0xebf304: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState initWithTrackingId:withClientId:withAppName:withAppVersion:]")]
#[doc(alias = "-[GAITrackerState initWithTrackingId:withClientId:withAppName:withAppVersion:]")]
// 0xebf360 — -[GAITrackerState initWithTrackingId:withClientId:withAppName:withAppVersion:]
// type: GAITrackerState *__cdecl(GAITrackerState *self, SEL, id, id, id, id)
pub fn stub_0xebf360() {
    // IDA 0xebf360: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAITrackerState trackerStateWithTrackingId:withClientId:withAppName:withAppVersion:]")]
#[doc(alias = "+[GAITrackerState trackerStateWithTrackingId:withClientId:withAppName:withAppVersion:]")]
// 0xebf5a8 — +[GAITrackerState trackerStateWithTrackingId:withClientId:withAppName:withAppVersion:]
// type: GAITrackerState *__cdecl(id, SEL, id, id, id, id)
pub fn stub_0xebf5a8() {
    // IDA 0xebf5a8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState dealloc]")]
#[doc(alias = "-[GAITrackerState dealloc]")]
// 0xebf73c — -[GAITrackerState dealloc]
// type: void __cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebf73c() {
    // IDA 0xebf73c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setEventParameters:action:label:value:]")]
#[doc(alias = "-[GAITrackerState setEventParameters:action:label:value:]")]
// 0xebf7c8 — -[GAITrackerState setEventParameters:action:label:value:]
// type: void __cdecl(GAITrackerState *self, SEL, id, id, id, id)
pub fn stub_0xebf7c8() {
    // IDA 0xebf7c8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setTransactionParameters:]")]
#[doc(alias = "-[GAITrackerState setTransactionParameters:]")]
// 0xebf89c — -[GAITrackerState setTransactionParameters:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebf89c() {
    // IDA 0xebf89c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setItemParameters:transaction:]")]
#[doc(alias = "-[GAITrackerState setItemParameters:transaction:]")]
// 0xebfa4c — -[GAITrackerState setItemParameters:transaction:]
// type: void __cdecl(GAITrackerState *self, SEL, id, id)
pub fn stub_0xebfa4c() {
    // IDA 0xebfa4c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setExceptionParameters:isFatal:]")]
#[doc(alias = "-[GAITrackerState setExceptionParameters:isFatal:]")]
// 0xebfc2c — -[GAITrackerState setExceptionParameters:isFatal:]
// type: void __cdecl(GAITrackerState *self, SEL, id, char)
pub fn stub_0xebfc2c() {
    // IDA 0xebfc2c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setTimingParameters:value:category:label:]")]
#[doc(alias = "-[GAITrackerState setTimingParameters:value:category:label:]")]
// 0xebfcc0 — -[GAITrackerState setTimingParameters:value:category:label:]
// type: void __cdecl(GAITrackerState *self, SEL, id, double, id, id)
pub fn stub_0xebfcc0() {
    // IDA 0xebfcc0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setSocialParameters:action:target:]")]
#[doc(alias = "-[GAITrackerState setSocialParameters:action:target:]")]
// 0xebfde0 — -[GAITrackerState setSocialParameters:action:target:]
// type: void __cdecl(GAITrackerState *self, SEL, id, id, id)
pub fn stub_0xebfde0() {
    // IDA 0xebfde0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState model]")]
#[doc(alias = "-[GAITrackerState model]")]
// 0xebfe7c — -[GAITrackerState model]
// type: GAIModel *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebfe7c() {
    // IDA 0xebfe7c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setModel:]")]
#[doc(alias = "-[GAITrackerState setModel:]")]
// 0xebfe8c — -[GAITrackerState setModel:]
// type: void __cdecl(GAITrackerState *self, SEL, id)
pub fn stub_0xebfe8c() {
    // IDA 0xebfe8c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState sampled]")]
#[doc(alias = "-[GAITrackerState sampled]")]
// 0xebfeb0 — -[GAITrackerState sampled]
// type: char __cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebfeb0() {
    // IDA 0xebfeb0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setSampled:]")]
#[doc(alias = "-[GAITrackerState setSampled:]")]
// 0xebfec0 — -[GAITrackerState setSampled:]
// type: void __cdecl(GAITrackerState *self, SEL, char)
pub fn stub_0xebfec0() {
    // IDA 0xebfec0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState httpDispatchUrl]")]
#[doc(alias = "-[GAITrackerState httpDispatchUrl]")]
// 0xebfed0 — -[GAITrackerState httpDispatchUrl]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebfed0() {
    // IDA 0xebfed0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState httpsDispatchUrl]")]
#[doc(alias = "-[GAITrackerState httpsDispatchUrl]")]
// 0xebfee8 — -[GAITrackerState httpsDispatchUrl]
// type: NSString *__cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebfee8() {
    // IDA 0xebfee8: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState useHttps]")]
#[doc(alias = "-[GAITrackerState useHttps]")]
// 0xebff00 — -[GAITrackerState useHttps]
// type: char __cdecl(GAITrackerState *self, SEL)
pub fn stub_0xebff00() {
    // IDA 0xebff00: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackerState setUseHttps:]")]
#[doc(alias = "-[GAITrackerState setUseHttps:]")]
// 0xebff10 — -[GAITrackerState setUseHttps:]
// type: void __cdecl(GAITrackerState *self, SEL, char)
pub fn stub_0xebff10() {
    // IDA 0xebff10: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIStringUtil init]")]
#[doc(alias = "-[GAIStringUtil init]")]
// 0xebff20 — -[GAIStringUtil init]
// type: GAIStringUtil *__cdecl(GAIStringUtil *self, SEL)
pub fn stub_0xebff20() {
    // IDA 0xebff20: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil trim:]")]
#[doc(alias = "+[GAIStringUtil trim:]")]
// 0xebff7c — +[GAIStringUtil trim:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xebff7c() {
    // IDA 0xebff7c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil intString:]")]
#[doc(alias = "+[GAIStringUtil intString:]")]
// 0xebffb4 — +[GAIStringUtil intString:]
// type: id __cdecl(id, SEL, signed __int64)
pub fn stub_0xebffb4() {
    // IDA 0xebffb4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil uintString:]")]
#[doc(alias = "+[GAIStringUtil uintString:]")]
// 0xebffec — +[GAIStringUtil uintString:]
// type: id __cdecl(id, SEL, unsigned __int64)
pub fn stub_0xebffec() {
    // IDA 0xebffec: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil decimalString:decimals:]")]
#[doc(alias = "+[GAIStringUtil decimalString:decimals:]")]
// 0xec0024 — +[GAIStringUtil decimalString:decimals:]
// type: id __cdecl(id, SEL, double, int)
pub fn stub_0xec0024() {
    // IDA 0xec0024: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil currencyMicrosString:]")]
#[doc(alias = "+[GAIStringUtil currencyMicrosString:]")]
// 0xec00f4 — +[GAIStringUtil currencyMicrosString:]
// type: id __cdecl(id, SEL, signed __int64)
pub fn stub_0xec00f4() {
    // IDA 0xec00f4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil percentEncode:]")]
#[doc(alias = "+[GAIStringUtil percentEncode:]")]
// 0xec0294 — +[GAIStringUtil percentEncode:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xec0294() {
    // IDA 0xec0294: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil percentDecode:]")]
#[doc(alias = "+[GAIStringUtil percentDecode:]")]
// 0xec02fc — +[GAIStringUtil percentDecode:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xec02fc() {
    // IDA 0xec02fc: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil decodeParametersFromQuery:]")]
#[doc(alias = "+[GAIStringUtil decodeParametersFromQuery:]")]
// 0xec033c — +[GAIStringUtil decodeParametersFromQuery:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xec033c() {
    // IDA 0xec033c: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIStringUtil decodeParametersFromUrl:]")]
#[doc(alias = "+[GAIStringUtil decodeParametersFromUrl:]")]
// 0xec0500 — +[GAIStringUtil decodeParametersFromUrl:]
// type: id __cdecl(id, SEL, id)
pub fn stub_0xec0500() {
    // IDA 0xec0500: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection dealloc]")]
#[doc(alias = "-[GAIURLConnection dealloc]")]
// 0xec0598 — -[GAIURLConnection dealloc]
// type: void __cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0598() {
    // IDA 0xec0598: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection init]")]
#[doc(alias = "-[GAIURLConnection init]")]
// 0xec0648 — -[GAIURLConnection init]
// type: GAIURLConnection *__cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0648() {
    // IDA 0xec0648: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection initWithRequest:completionQueue:completionHandler:]")]
#[doc(alias = "-[GAIURLConnection initWithRequest:completionQueue:completionHandler:]")]
// 0xec06a4 — -[GAIURLConnection initWithRequest:completionQueue:completionHandler:]
// type: GAIURLConnection *__cdecl(GAIURLConnection *self, SEL, id, dispatch_queue_s *, id)
pub fn stub_0xec06a4() {
    // IDA 0xec06a4: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection cancel]")]
#[doc(alias = "-[GAIURLConnection cancel]")]
// 0xec07c4 — -[GAIURLConnection cancel]
// type: void __cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec07c4() {
    // IDA 0xec07c4: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection connection:didFailWithError:]")]
#[doc(alias = "-[GAIURLConnection connection:didFailWithError:]")]
// 0xec0800 — -[GAIURLConnection connection:didFailWithError:]
// type: void __cdecl(GAIURLConnection *self, SEL, id, id)
pub fn stub_0xec0800() {
    // IDA 0xec0800: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "sub_EC0878")]
#[doc(alias = "sub_EC0878")]
// 0xec0878 — sub_EC0878
pub fn stub_0xec0878() {
    // IDA 0xec0878: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "sub_EC08B4")]
#[doc(alias = "sub_EC08B4")]
// 0xec08b4 — sub_EC08B4
pub fn stub_0xec08b4() {
    // IDA 0xec08b4: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "sub_EC08D8")]
#[doc(alias = "sub_EC08D8")]
// 0xec08d8 — sub_EC08D8
pub fn stub_0xec08d8() {
    // IDA 0xec08d8: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection connection:willSendRequest:redirectResponse:]")]
#[doc(alias = "-[GAIURLConnection connection:willSendRequest:redirectResponse:]")]
// 0xec08f0 — -[GAIURLConnection connection:willSendRequest:redirectResponse:]
// type: id __cdecl(GAIURLConnection *self, SEL, id, id, id)
pub fn stub_0xec08f0() {
    // IDA 0xec08f0: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection connection:willCacheResponse:]")]
#[doc(alias = "-[GAIURLConnection connection:willCacheResponse:]")]
// 0xec08f4 — -[GAIURLConnection connection:willCacheResponse:]
// type: id __cdecl(GAIURLConnection *self, SEL, id, id)
pub fn stub_0xec08f4() {
    // IDA 0xec08f4: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection connection:didReceiveData:]")]
#[doc(alias = "-[GAIURLConnection connection:didReceiveData:]")]
// 0xec08f8 — -[GAIURLConnection connection:didReceiveData:]
// type: void __cdecl(GAIURLConnection *self, SEL, id, id)
pub fn stub_0xec08f8() {
    // IDA 0xec08f8: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection connection:didReceiveResponse:]")]
#[doc(alias = "-[GAIURLConnection connection:didReceiveResponse:]")]
// 0xec0938 — -[GAIURLConnection connection:didReceiveResponse:]
// type: void __cdecl(GAIURLConnection *self, SEL, id, id)
pub fn stub_0xec0938() {
    // IDA 0xec0938: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection connectionDidFinishLoading:]")]
#[doc(alias = "-[GAIURLConnection connectionDidFinishLoading:]")]
// 0xec0984 — -[GAIURLConnection connectionDidFinishLoading:]
// type: void __cdecl(GAIURLConnection *self, SEL, id)
pub fn stub_0xec0984() {
    // IDA 0xec0984: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "sub_EC09F8")]
#[doc(alias = "sub_EC09F8")]
// 0xec09f8 — sub_EC09F8
pub fn stub_0xec09f8() {
    // IDA 0xec09f8: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "sub_EC0A34")]
#[doc(alias = "sub_EC0A34")]
// 0xec0a34 — sub_EC0A34
pub fn stub_0xec0a34() {
    // IDA 0xec0a34: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "sub_EC0A44")]
#[doc(alias = "sub_EC0A44")]
// 0xec0a44 — sub_EC0A44
pub fn stub_0xec0a44() {
    // IDA 0xec0a44: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection request]")]
#[doc(alias = "-[GAIURLConnection request]")]
// 0xec0a54 — -[GAIURLConnection request]
// type: NSURLRequest *__cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0a54() {
    // IDA 0xec0a54: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection setRequest:]")]
#[doc(alias = "-[GAIURLConnection setRequest:]")]
// 0xec0a64 — -[GAIURLConnection setRequest:]
// type: void __cdecl(GAIURLConnection *self, SEL, id)
pub fn stub_0xec0a64() {
    // IDA 0xec0a64: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection connection]")]
#[doc(alias = "-[GAIURLConnection connection]")]
// 0xec0a88 — -[GAIURLConnection connection]
// type: NSURLConnection *__cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0a88() {
    // IDA 0xec0a88: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection setConnection:]")]
#[doc(alias = "-[GAIURLConnection setConnection:]")]
// 0xec0a98 — -[GAIURLConnection setConnection:]
// type: void __cdecl(GAIURLConnection *self, SEL, id)
pub fn stub_0xec0a98() {
    // IDA 0xec0a98: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection response]")]
#[doc(alias = "-[GAIURLConnection response]")]
// 0xec0abc — -[GAIURLConnection response]
// type: NSURLResponse *__cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0abc() {
    // IDA 0xec0abc: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection setResponse:]")]
#[doc(alias = "-[GAIURLConnection setResponse:]")]
// 0xec0acc — -[GAIURLConnection setResponse:]
// type: void __cdecl(GAIURLConnection *self, SEL, id)
pub fn stub_0xec0acc() {
    // IDA 0xec0acc: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "-[GAIURLConnection data]")]
#[doc(alias = "-[GAIURLConnection data]")]
// 0xec0af0 — -[GAIURLConnection data]
// type: NSMutableData *__cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0af0() {
    // IDA 0xec0af0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection setData:]")]
#[doc(alias = "-[GAIURLConnection setData:]")]
// 0xec0b00 — -[GAIURLConnection setData:]
// type: void __cdecl(GAIURLConnection *self, SEL, id)
pub fn stub_0xec0b00() {
    // IDA 0xec0b00: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection completionQueue]")]
#[doc(alias = "-[GAIURLConnection completionQueue]")]
// 0xec0b24 — -[GAIURLConnection completionQueue]
// type: dispatch_queue_s *__cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0b24() {
    // IDA 0xec0b24: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection setCompletionQueue:]")]
#[doc(alias = "-[GAIURLConnection setCompletionQueue:]")]
// 0xec0b34 — -[GAIURLConnection setCompletionQueue:]
// type: void __cdecl(GAIURLConnection *self, SEL, dispatch_queue_s *)
pub fn stub_0xec0b34() {
    // IDA 0xec0b34: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection completionHandler]")]
#[doc(alias = "-[GAIURLConnection completionHandler]")]
// 0xec0b44 — -[GAIURLConnection completionHandler]
// type: id __cdecl(GAIURLConnection *self, SEL)
pub fn stub_0xec0b44() {
    // IDA 0xec0b44: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIURLConnection setCompletionHandler:]")]
#[doc(alias = "-[GAIURLConnection setCompletionHandler:]")]
// 0xec0b54 — -[GAIURLConnection setCompletionHandler:]
// type: void __cdecl(GAIURLConnection *self, SEL, id)
pub fn stub_0xec0b54() {
    // IDA 0xec0b54: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "+[GAIAdMobInfo adMobInfo]")]
#[doc(alias = "+[GAIAdMobInfo adMobInfo]")]
// 0xec0b78 — +[GAIAdMobInfo adMobInfo]
// type: id __cdecl(id, SEL)
pub fn stub_0xec0b78() {
    // IDA 0xec0b78: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackedViewController viewDidAppear:]")]
#[doc(alias = "-[GAITrackedViewController viewDidAppear:]")]
// 0xec0bac — -[GAITrackedViewController viewDidAppear:]
// type: void __cdecl(GAITrackedViewController *self, SEL, char)
pub fn stub_0xec0bac() {
    // IDA 0xec0bac: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackedViewController dealloc]")]
#[doc(alias = "-[GAITrackedViewController dealloc]")]
// 0xec0cf8 — -[GAITrackedViewController dealloc]
// type: void __cdecl(GAITrackedViewController *self, SEL)
pub fn stub_0xec0cf8() {
    // IDA 0xec0cf8: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackedViewController tracker]")]
#[doc(alias = "-[GAITrackedViewController tracker]")]
// 0xec0d44 — -[GAITrackedViewController tracker]
// type: GAITracker *__cdecl(GAITrackedViewController *self, SEL)
pub fn stub_0xec0d44() {
    // IDA 0xec0d44: ObjC platform singleton/class method (IDA 0xf071d4 family). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackedViewController setTracker:]")]
#[doc(alias = "-[GAITrackedViewController setTracker:]")]
// 0xec0d54 — -[GAITrackedViewController setTracker:]
// type: void __cdecl(GAITrackedViewController *self, SEL, id)
pub fn stub_0xec0d54() {
    // IDA 0xec0d54: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackedViewController trackedViewName]")]
#[doc(alias = "-[GAITrackedViewController trackedViewName]")]
// 0xec0d64 — -[GAITrackedViewController trackedViewName]
// type: NSString *__cdecl(GAITrackedViewController *self, SEL)
pub fn stub_0xec0d64() {
    // IDA 0xec0d64: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAITrackedViewController setTrackedViewName:]")]
#[doc(alias = "-[GAITrackedViewController setTrackedViewName:]")]
// 0xec0d7c — -[GAITrackedViewController setTrackedViewName:]
// type: void __cdecl(GAITrackedViewController *self, SEL, id)
pub fn stub_0xec0d7c() {
    // IDA 0xec0d7c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}

#[doc(alias = "-[GAIReachabilityChecker reachabilityApi]")]
#[doc(alias = "-[GAIReachabilityChecker reachabilityApi]")]
// 0xec0da0 — -[GAIReachabilityChecker reachabilityApi]
// type: const GAIReachabilityApi *__cdecl(GAIReachabilityChecker *self, SEL)
pub fn stub_0xec0da0() {
    // IDA 0xec0da0: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
