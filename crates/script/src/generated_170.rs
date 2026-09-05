// Auto-generated skeletons for rbx-script — Lua|Script|CodeGen|Yield batch
// Filter: Lua|Script|CodeGen|Yield (4818 filtered, 0 remaining) -> global gap filler EA-sorted asc next 100 not yet in script crate
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x419f4..0x45764 EA-sorted asc next 100 global not yet in script crate (script 13921 -> 14021 distinct, global 85545->85545 distinct)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicBool, Ordering};
use crate::generated_110::BoostMutex;
use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included, Unbounded};

/// `RBX::FunctionMarshaller` observable state (IDA 0x4352c..0x43b98): the
/// ctor wires the thread (folds into the id); window binding, wakefulness,
/// and the queued functor ids are observed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MarshallerState {
    pub thread_id: u32,
    pub window: Option<u32>,
    pub queue: Vec<u32>,
}

/// Marshalled app-event outcome (IDA 0x43930): the functor runs (0x4398a),
/// is cleared and deleted (0x43990..0x4399e), and the event is set
/// (0x439b8..0x439bc). Payload management folds into the host.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MarshallerEvent {
    pub calls: u32,
    pub signaled: bool,
}

/// `FunctionMarshaller::StaticData` one-shot latch (IDA 0x441a8).
static MARSHALLER_STATIC_DATA: LazyLock<u32> = LazyLock::new(|| 1);

// ---- Batch model: UserInfo ivars + RobloxGoogleAnalytics init/page-view path ----
// IDA ground truth per stub below (decompile + disasm via IDA MCP).
// Unmodeled throughout: ObjC runtime messaging/retain accounting (the Mutex
// store is the retained slot; objc_setProperty atomic=0/copy=0 needs no more),
// GAI/GAITracker SDK internals (dispatch interval, tracking id, sample rate
// and sent views are the observable state), the main-queue async hop (the
// dispatched flag stands in for the queued block), the iOS settings-service
// fetch (caller passes the tracking id / sample rate it returned), and the
// C++ static-destruction registrations (__cxa_atexit has no host here).

/// ObjC `id` with no host runtime: an opaque handle, `None` is `nil`.
/// Matches the `ObjCId` convention used for the iOS lifecycle bridge.
pub type ObjCId = usize;

/// was: `UserInfo` ObjC class — ten retained ivars at +8..+44.
/// Each getter below is one `LDR [R0, ivar]` (IDA 0x419f4..0x41bd8); each
/// setter is `objc_setProperty(..., atomic=0, copy=0)` at the matching offset.
#[derive(Debug, Default)]
pub struct UserInfo {
    user_info_dict: Mutex<Option<ObjCId>>,
    userinfo: Mutex<Option<ObjCId>>,
    rbx_bal: Mutex<Option<ObjCId>>,
    tik_bal: Mutex<Option<ObjCId>>,
    user_thumb_nail_url: Mutex<Option<ObjCId>>,
    bc_member: Mutex<Option<ObjCId>>,
    encoded_password: Mutex<Option<ObjCId>>,
    encoded_username: Mutex<Option<ObjCId>>,
    username: Mutex<Option<ObjCId>>,
    password: Mutex<Option<ObjCId>>,
}

/// was: `RobloxGoogleAnalytics` class state (`_initializeDone` at 0x41cd0 plus
/// the `GAI` default tracker configured by the init block).
#[derive(Debug, Default)]
pub struct GoogleAnalyticsState {
    /// Set by `+[RobloxGoogleAnalytics initialize]` (IDA 0x41cc4): the init
    /// block was queued on the main queue shim.
    pub init_dispatched: AtomicBool,
    /// `_initializeDone`; set by the init block (IDA 0x41e5a), read by
    /// `initialize` (IDA 0x41cce) and `setPageViewTracking:` (IDA 0x41f8c).
    pub initialize_done: AtomicBool,
    /// `-[GAI setDispatchInterval:]` value; always 10.0s (IDA 0x41dcc).
    pub dispatch_interval_secs: Mutex<f64>,
    /// `-[GAI trackerWithTrackingId:]` value from the settings service.
    pub tracking_id: Mutex<Option<String>>,
    /// `-[GAITracker setSampleRate:]` value (IDA 0x41e4a).
    pub sample_rate: Mutex<f64>,
    /// `-[GAITracker sendView:]` log (IDA 0x41fd6).
    pub sent_page_views: Mutex<Vec<String>>,
    /// `performSelector:withObject:afterDelay:0` retry queue used while not
    /// yet initialized (IDA 0x42012..0x4203a).
    pub deferred_page_views: Mutex<Vec<String>>,
    /// Sent event trackings (IDA 0x4216c).
    pub sent_events: Mutex<Vec<AnalyticsEvent>>,
    /// Deferred event dicts while not initialized (IDA 0x42204..).
    pub deferred_events: Mutex<Vec<AnalyticsEvent>>,
    /// Custom variable pairs (IDA 0x422fe).
    pub custom_vars: Mutex<Vec<(ObjCId, ObjCId)>>,
    /// Deferred custom-variable dicts while not initialized (IDA 0x42348..).
    pub deferred_custom: Mutex<Vec<(ObjCId, ObjCId)>>,
    /// `debug_*` counters in NSUserDefaults (IDA 0x423cc..0x4255c).
    pub debug_counters: Mutex<HashMap<String, i64>>,
}

/// One GA event tracking (IDA 0x42078..0x420bc keys, sent at 0x4216c).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AnalyticsEvent {
    pub category: ObjCId,
    pub action: ObjCId,
    pub label: ObjCId,
    pub value: i32,
}

/// `RobloxWebUtility` instance state (IDA 0x427c0..0x4286e): queue handles,
/// the cached settings service, and the last-request timestamp. ObjC and
/// dispatch objects fold into latches.
#[derive(Debug, Default)]
pub struct RobloxWebUtility {
    pub log_queue: Mutex<bool>,
    pub settings_queue: Mutex<bool>,
    pub cached_settings: Mutex<bool>,
    pub last_request_set: Mutex<bool>,
}

static WEB_UTILITY: LazyLock<RobloxWebUtility> = LazyLock::new(RobloxWebUtility::default);

/// Process-wide web-utility backing the class-method stubs below.
pub fn web_utility() -> &'static RobloxWebUtility {
    &WEB_UTILITY
}

/// `iOSSettingsService` construction latch (IDA 0x43180: tree zeroes at
/// 0x431aa..0x431be, strings at 0x431de..0x43208, `_thisPtr` at 0x4320e,
/// and `Init` at 0x43236 fold into the host).
#[derive(Debug, Default)]
pub struct SettingsServiceState {
    pub inited: bool,
}

/// `__GLOBAL__I_a_12` one-shot latch (IDA 0x42580).
static GLOBAL_A12_INIT: LazyLock<u32> = LazyLock::new(|| 1);

/// Button-tag URL answer (IDA 0x42dec): tags 10-16 map base paths with
/// tablet variants (0x42e60..0x430ae); other tags answer nil; the record
/// flag fires page-view tracking (0x430b4..0x430d4); the printf (0x430d8..
/// 0x430fc) folds into the host.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ButtonUrl {
    pub url: Option<String>,
    pub page: Option<&'static str>,
}

/// `getUrlForButtonTag:recordPageView:query:` (IDA 0x42dec).
pub fn button_url(base: &str, search: &str, tag: i32, tablet: bool, query: &str) -> ButtonUrl {
    let page;
    let url = match tag {
        10 => {
            page = "Games";
            Some(format!("{base}games/list"))
        }
        11 => {
            page = "Catalog";
            Some(format!("{base}{}", if tablet { "Catalog/" } else { "catalog/" }))
        }
        12 => {
            page = "Inventory";
            Some(format!("{base}{}", if tablet { "My/Character.aspx" } else { "inventory" }))
        }
        13 => {
            page = "BuildersClub";
            Some(format!("{base}mobile-app-upgrades/"))
        }
        14 => {
            page = "Profile";
            Some(format!("{base}{}", if tablet { "User.aspx" } else { "" }))
        }
        15 => {
            page = "Messages";
            Some(format!("{base}{}", if tablet { "My/Messages.aspx#Inbox" } else { "inbox" }))
        }
        16 => {
            page = "Search";
            Some(format!("{base}{search}{query}"))
        }
        _ => return ButtonUrl { url: None, page: None },
    };
    ButtonUrl { url, page: Some(page) }
}

static GOOGLE_ANALYTICS: LazyLock<GoogleAnalyticsState> = LazyLock::new(GoogleAnalyticsState::default);

/// Process-wide analytics state backing the class-method stubs below.
pub fn google_analytics() -> &'static GoogleAnalyticsState {
    &GOOGLE_ANALYTICS
}

/// was: `__GLOBAL__I_a_11` TU statics — two `boost::system::generic_category`
/// slots, one `system_category` slot, one `std::ios_base::Init`, and the
/// `bad_alloc` / `bad_exception` static-exception guards (IDA 0x41bfc).
/// The `__cxa_atexit` destructor registrations have no host and are not kept.
#[derive(Debug, Default)]
pub struct CxxModuleStatics {
    pub generic_category_a: Mutex<bool>,
    pub generic_category_b: Mutex<bool>,
    pub system_category: Mutex<bool>,
    pub ios_base_init: Mutex<bool>,
    pub bad_alloc_init: Mutex<bool>,
    pub bad_exception_init: Mutex<bool>,
}

// 0x419f4 — -[UserInfo userInfoDict]
// type: NSDictionary *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userInfoDict]")]
// IDA 0x419f4: `return self->userInfoDict` (ivar load, no retain).
pub fn stub_0x419f4(info: &UserInfo) -> Option<ObjCId> {
    *info.user_info_dict.lock()
}

// 0x41a04 — -[UserInfo setUserInfoDict:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUserInfoDict:]")]
// IDA 0x41a04: `objc_setProperty(self, sel, 8, value, atomic=0, copy=0)`.
pub fn stub_0x41a04(info: &UserInfo, value: Option<ObjCId>) {
    *info.user_info_dict.lock() = value;
}

// 0x41a28 — -[UserInfo userinfo]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userinfo]")]
// IDA 0x41a28: `return self->userinfo` (ivar load, no retain).
pub fn stub_0x41a28(info: &UserInfo) -> Option<ObjCId> {
    *info.userinfo.lock()
}

// 0x41a38 — -[UserInfo setUserinfo:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUserinfo:]")]
// IDA 0x41a38: `objc_setProperty(self, sel, 12, value, atomic=0, copy=0)`.
pub fn stub_0x41a38(info: &UserInfo, value: Option<ObjCId>) {
    *info.userinfo.lock() = value;
}

// 0x41a5c — -[UserInfo rbxBal]
// type: NSNumber *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo rbxBal]")]
// IDA 0x41a5c: `return self->rbxBal` (NSNumber* ivar load, no retain).
pub fn stub_0x41a5c(info: &UserInfo) -> Option<ObjCId> {
    *info.rbx_bal.lock()
}

// 0x41a6c — -[UserInfo setRbxBal:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setRbxBal:]")]
// IDA 0x41a6c: `objc_setProperty(self, sel, 16, value, atomic=0, copy=0)`.
pub fn stub_0x41a6c(info: &UserInfo, value: Option<ObjCId>) {
    *info.rbx_bal.lock() = value;
}

// 0x41a90 — -[UserInfo tikBal]
// type: NSNumber *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo tikBal]")]
// IDA 0x41a90: `return self->tikBal` (NSNumber* ivar load, no retain).
pub fn stub_0x41a90(info: &UserInfo) -> Option<ObjCId> {
    *info.tik_bal.lock()
}

// 0x41aa0 — -[UserInfo setTikBal:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setTikBal:]")]
// IDA 0x41aa0: `objc_setProperty(self, sel, 20, value, atomic=0, copy=0)`.
pub fn stub_0x41aa0(info: &UserInfo, value: Option<ObjCId>) {
    *info.tik_bal.lock() = value;
}

// 0x41ac4 — -[UserInfo userThumbNailUrl]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo userThumbNailUrl]")]
// IDA 0x41ac4: `return self->userThumbNailUrl` (ivar load, no retain).
pub fn stub_0x41ac4(info: &UserInfo) -> Option<ObjCId> {
    *info.user_thumb_nail_url.lock()
}

// 0x41ad4 — -[UserInfo setUserThumbNailUrl:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUserThumbNailUrl:]")]
// IDA 0x41ad4: `objc_setProperty(self, sel, 24, value, atomic=0, copy=0)`.
pub fn stub_0x41ad4(info: &UserInfo, value: Option<ObjCId>) {
    *info.user_thumb_nail_url.lock() = value;
}

// 0x41af8 — -[UserInfo bcMember]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo bcMember]")]
// IDA 0x41af8: `return self->bcMember` (ivar load, no retain).
pub fn stub_0x41af8(info: &UserInfo) -> Option<ObjCId> {
    *info.bc_member.lock()
}

// 0x41b08 — -[UserInfo setBcMember:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setBcMember:]")]
// IDA 0x41b08: `objc_setProperty(self, sel, 28, value, atomic=0, copy=0)`.
pub fn stub_0x41b08(info: &UserInfo, value: Option<ObjCId>) {
    *info.bc_member.lock() = value;
}

// 0x41b2c — -[UserInfo encodedPassword]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo encodedPassword]")]
// IDA 0x41b2c: `return self->encodedPassword` (ivar load, no retain).
pub fn stub_0x41b2c(info: &UserInfo) -> Option<ObjCId> {
    *info.encoded_password.lock()
}

// 0x41b3c — -[UserInfo setEncodedPassword:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setEncodedPassword:]")]
// IDA 0x41b3c: `objc_setProperty(self, sel, 32, value, atomic=0, copy=0)`.
pub fn stub_0x41b3c(info: &UserInfo, value: Option<ObjCId>) {
    *info.encoded_password.lock() = value;
}

// 0x41b60 — -[UserInfo encodedUsername]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo encodedUsername]")]
// IDA 0x41b60: `return self->encodedUsername` (ivar load, no retain).
pub fn stub_0x41b60(info: &UserInfo) -> Option<ObjCId> {
    *info.encoded_username.lock()
}

// 0x41b70 — -[UserInfo setEncodedUsername:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setEncodedUsername:]")]
// IDA 0x41b70: `objc_setProperty(self, sel, 36, value, atomic=0, copy=0)`.
pub fn stub_0x41b70(info: &UserInfo, value: Option<ObjCId>) {
    *info.encoded_username.lock() = value;
}

// 0x41b94 — -[UserInfo username]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo username]")]
// IDA 0x41b94: `return self->_username` (ivar load, no retain).
pub fn stub_0x41b94(info: &UserInfo) -> Option<ObjCId> {
    *info.username.lock()
}

// 0x41ba4 — -[UserInfo setUsername:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setUsername:]")]
// IDA 0x41ba4: `objc_setProperty(self, sel, 40, value, atomic=0, copy=0)`.
pub fn stub_0x41ba4(info: &UserInfo, value: Option<ObjCId>) {
    *info.username.lock() = value;
}

// 0x41bc8 — -[UserInfo password]
// type: NSString *__cdecl(UserInfo *self, SEL)
#[doc(alias = "-[UserInfo password]")]
// IDA 0x41bc8: `return self->_password` (ivar load, no retain).
pub fn stub_0x41bc8(info: &UserInfo) -> Option<ObjCId> {
    *info.password.lock()
}

// 0x41bd8 — -[UserInfo setPassword:]
// type: void __cdecl(UserInfo *self, SEL, id)
#[doc(alias = "-[UserInfo setPassword:]")]
// IDA 0x41bd8: `objc_setProperty(self, sel, 44, value, atomic=0, copy=0)`.
pub fn stub_0x41bd8(info: &UserInfo, value: Option<ObjCId>) {
    *info.password.lock() = value;
}

// 0x41bfc — __GLOBAL__I_a_11
#[doc(alias = "global constructor keyed to_a_11")]
// IDA 0x41bfc: installs `generic_category` twice, `system_category` once,
// runs `std::ios_base::Init::Init`, and one-time inits the `bad_alloc` /
// `bad_exception` static exception objects behind byte guards (0x41c50,
// 0x41c8e). The `__cxa_atexit` registrations are unmodeled.
pub fn stub_0x41bfc(statics: &CxxModuleStatics) {
    *statics.generic_category_a.lock() = true;
    *statics.generic_category_b.lock() = true;
    *statics.system_category.lock() = true;
    *statics.ios_base_init.lock() = true;
    if !*statics.bad_alloc_init.lock() {
        *statics.bad_alloc_init.lock() = true;
    }
    if !*statics.bad_exception_init.lock() {
        *statics.bad_exception_init.lock() = true;
    }
}

// 0x41cc4 — +[RobloxGoogleAnalytics initialize]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[RobloxGoogleAnalytics initialize]")]
// IDA 0x41cc4: `if (!initializeDone) dispatch_async(main_q, block)`; the
// early `BXNE LR` (0x41cd2) makes repeat calls after init a no-op.
// Returns whether the init block was newly queued.
pub fn stub_0x41cc4(state: &GoogleAnalyticsState) -> bool {
    if state.initialize_done.load(Ordering::SeqCst) {
        return false;
    }
    state.init_dispatched.store(true, Ordering::SeqCst);
    true
}

// 0x41cf0 — ___35+[RobloxGoogleAnalytics initialize]_block_invoke
// type: void __cdecl(id)
#[doc(alias = "___35+[RobloxGoogleAnalytics initialize]_block_invoke")]
// IDA 0x41cf0: reads the settings-service tracking id (`var19`) and sample
// (`var20`); when the id string is non-empty (0x41d68) installs the GAI
// tracker — dispatch interval 10.0s, `trackerWithTrackingId:`, sample rate —
// and sets `initializeDone = 1` (0x41e5a). Empty id: string dtor only.
// Returns whether the tracker was installed.
pub fn stub_0x41cf0(state: &GoogleAnalyticsState, tracking_id: &str, sample_rate: f64) -> bool {
    if tracking_id.is_empty() {
        return false;
    }
    *state.dispatch_interval_secs.lock() = 10.0;
    *state.tracking_id.lock() = Some(tracking_id.to_owned());
    *state.sample_rate.lock() = sample_rate;
    state.initialize_done.store(true, Ordering::SeqCst);
    true
}

// 0x41f28 — +[RobloxGoogleAnalytics release]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[RobloxGoogleAnalytics release]")]
// IDA 0x41f28: empty body — `+release` on the class object is a no-op.
pub fn stub_0x41f28() {}

// 0x41f2c — +[RobloxGoogleAnalytics callBackPageTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics callBackPageTracking:]")]
// IDA 0x41f2c: `url = [params objectForKey:@"url"]` (0x41f56), then
// `+[RobloxGoogleAnalytics setPageViewTracking:url]` (0x41f6e). The dict
// lookup is the caller's; the forward is modeled here.
pub fn stub_0x41f2c(state: &GoogleAnalyticsState, url: &str) {
    stub_0x41f74(state, url);
}

// 0x41f74 — +[RobloxGoogleAnalytics setPageViewTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics setPageViewTracking:]")]
// IDA 0x41f74: if initialized, `[[GAI sharedInstance] defaultTracker]
// sendView:url` (0x41faa..0x41fd6); else re-queues via
// `performSelector:callBackPageTracking: withObject:{url:} afterDelay:0`.
pub fn stub_0x41f74(state: &GoogleAnalyticsState, url: &str) {
    if state.initialize_done.load(Ordering::SeqCst) {
        state.sent_page_views.lock().push(url.to_owned());
    } else {
        state.deferred_page_views.lock().push(url.to_owned());
    }
}

// 0x4203c — +[RobloxGoogleAnalytics callBackEventTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics callBackEventTracking:]")]
pub fn stub_0x4203c(state: &GoogleAnalyticsState, category: ObjCId, action: ObjCId, label: ObjCId, value: i32) {
    // IDA 0x4203c: unpacks category/action/label/value (0x42078..0x420bc)
    // and forwards to `setEventTracking:` (0x420d8).
    stub_0x420e4(state, category, action, label, value);
}

// 0x420e4 — +[RobloxGoogleAnalytics setEventTracking:withAction:withLabel:withValue:]
// type: void __cdecl(id, SEL, id, id, id, int)
#[doc(alias = "+[RobloxGoogleAnalytics setEventTracking:withAction:withLabel:withValue:]")]
pub fn stub_0x420e4(state: &GoogleAnalyticsState, category: ObjCId, action: ObjCId, label: ObjCId, value: i32) {
    // IDA 0x420e4: when initialized sends the event (0x42106..0x4216c),
    // else defers the dict for later (0x421b0..0x42204). Dict plumbing
    // folds into the event record.
    let event = AnalyticsEvent { category, action, label, value };
    if state.initialize_done.load(Ordering::SeqCst) {
        state.sent_events.lock().push(event);
    } else {
        state.deferred_events.lock().push(event);
    }
}

// 0x42230 — +[RobloxGoogleAnalytics callbackCustomVariableTracking:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics callbackCustomVariableTracking:]")]
pub fn stub_0x42230(state: &GoogleAnalyticsState, label: ObjCId, value: ObjCId) {
    // IDA 0x42230: unpacks label/value (0x4226a..0x42276) and forwards to
    // `setCustomVariable:` (0x42294).
    stub_0x42298(state, label, value);
}

// 0x42298 — +[RobloxGoogleAnalytics setCustomVariableWithLabel:withValue:]
// type: void __cdecl(id, SEL, id, id)
#[doc(alias = "+[RobloxGoogleAnalytics setCustomVariableWithLabel:withValue:]")]
pub fn stub_0x42298(state: &GoogleAnalyticsState, label: ObjCId, value: ObjCId) {
    // IDA 0x42298: when initialized sets the pair (0x422b6..0x422fe), else
    // defers the dict (0x42348..0x42366).
    if state.initialize_done.load(Ordering::SeqCst) {
        state.custom_vars.lock().push((label, value));
    } else {
        state.deferred_custom.lock().push((label, value));
    }
}

// 0x42374 — +[RobloxGoogleAnalytics debugCountersPrint]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[RobloxGoogleAnalytics debugCountersPrint]")]
pub fn stub_0x42374(state: &GoogleAnalyticsState) -> Vec<(String, i64)> {
    // IDA 0x42374: synchronizes defaults (0x423aa) and reads the seven
    // `debug_*` keys (0x423cc..0x42344) for the NSLog dump (0x42450..).
    // The store folds into the host map; the snapshot is observed.
    let mut out: Vec<(String, i64)> = state.debug_counters.lock().clone().into_iter().collect();
    out.sort();
    out
}

// 0x424cc — +[RobloxGoogleAnalytics debugCounterIncrement:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[RobloxGoogleAnalytics debugCounterIncrement:]")]
pub fn stub_0x424cc(state: &GoogleAnalyticsState, name: &str) -> i64 {
    // IDA 0x424cc: synchronizes (0x42506), reads `debug_{name}` (0x42532..
    // 0x4254c), writes back +1 (0x4255c..0x42564), and logs (0x42576).
    // Defaults storage folds into the host map.
    let key = format!("debug_{name}");
    let mut counters = state.debug_counters.lock();
    let next = counters.get(&key).copied().unwrap_or(0) + 1;
    counters.insert(key, next);
    next
}

// 0x42580 — __GLOBAL__I_a_12
#[doc(alias = "global constructor keyed to_a_12")]
pub fn stub_0x42580() -> u32 {
    // IDA 0x42580: `__GLOBAL__I_a_12` — see `GLOBAL_A12_INIT`.
    *GLOBAL_A12_INIT
}

// 0x42718 — +[RobloxWebUtility sharedInstance]
// type: id __cdecl(id, SEL)
#[doc(alias = "+[RobloxWebUtility sharedInstance]")]
pub fn stub_0x42718() -> &'static RobloxWebUtility {
    // IDA 0x42718: `sharedInstance` once-inits (0x42760..0x4276e) and
    // answers the singleton (0x42766).
    web_utility()
}

// 0x42774 — ___34+[RobloxWebUtility sharedInstance]_block_invoke
#[doc(alias = "___34+[RobloxWebUtility sharedInstance]_block_invoke")]
pub fn stub_0x42774() {
    // IDA 0x42774: the once block allocs and inits (0x42786..0x427a4);
    // covered by `sharedInstance` — no-op.
}

// 0x427a8 — ___copy_helper_block__7
#[doc(alias = "___copy_helper_block__7")]
pub fn stub_0x427a8() {
    // IDA 0x427a8: block copy helper retains captures; `Clone` glue
    // covers it — no-op.
}

// 0x427b4 — ___destroy_helper_block__7
#[doc(alias = "___destroy_helper_block__7")]
pub fn stub_0x427b4() {
    // IDA 0x427b4: block destroy helper releases captures; drop glue
    // covers it — no-op.
}

// 0x427c0 — -[RobloxWebUtility init]
// type: RobloxWebUtility *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility init]")]
pub fn stub_0x427c0(util: &RobloxWebUtility) {
    // IDA 0x427c0: `init` chains to super (0x427e8) then stamps the epoch
    // timestamp (0x42810..0x4282a) and creates both queues (0x42858..
    // 0x4286e).
    *util.last_request_set.lock() = true;
    *util.log_queue.lock() = true;
    *util.settings_queue.lock() = true;
}

// 0x42880 — -[RobloxWebUtility dealloc]
// type: void __cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility dealloc]")]
pub fn stub_0x42880(util: &RobloxWebUtility) {
    // IDA 0x42880: `dealloc` releases the timestamp (0x42894..0x428a4),
    // both queues (0x428b6..0x428c8), and the cached service (0x428d8..
    // 0x428e4); drop glue covers the peers.
    *util.last_request_set.lock() = false;
    *util.log_queue.lock() = false;
    *util.settings_queue.lock() = false;
    *util.cached_settings.lock() = false;
}

// 0x4290c — -[RobloxWebUtility getiOSLogQueue]
// type: dispatch_queue_s *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSLogQueue]")]
pub fn stub_0x4290c(util: &RobloxWebUtility) -> bool {
    // IDA 0x4290c: answers the log queue (0x4291a).
    *util.log_queue.lock()
}

// 0x4291c — -[RobloxWebUtility getiOSSettingsQueue]
// type: dispatch_queue_s *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSSettingsQueue]")]
pub fn stub_0x4291c(util: &RobloxWebUtility) -> bool {
    // IDA 0x4291c: answers the settings queue (0x4292a).
    *util.settings_queue.lock()
}

// 0x4292c — -[RobloxWebUtility setCachediOSSettings:]
// type: void __cdecl(RobloxWebUtility *self, SEL, iOSSettingsService *)
#[doc(alias = "-[RobloxWebUtility setCachediOSSettings:]")]
pub fn stub_0x4292c(util: &RobloxWebUtility, has: bool) {
    // IDA 0x4292c: stores the cached service (0x42938).
    *util.cached_settings.lock() = has;
}

// 0x4293c — -[RobloxWebUtility getCachediOSSettings]
// type: iOSSettingsService *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getCachediOSSettings]")]
pub fn stub_0x4293c(util: &RobloxWebUtility) -> bool {
    // IDA 0x4293c: answers the cached service (0x4294a).
    *util.cached_settings.lock()
}

// 0x4294c — -[RobloxWebUtility getLastSettingsRequestTime]
// type: id __cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getLastSettingsRequestTime]")]
pub fn stub_0x4294c(util: &RobloxWebUtility) -> bool {
    // IDA 0x4294c: answers the last-request timestamp (0x4295a).
    *util.last_request_set.lock()
}

// 0x4295c — -[RobloxWebUtility getiOSSettingsServiceFromWeb]
// type: iOSSettingsService *__cdecl(RobloxWebUtility *self, SEL)
#[doc(alias = "-[RobloxWebUtility getiOSSettingsServiceFromWeb]")]
pub fn stub_0x4295c(util: &RobloxWebUtility) {
    // IDA 0x4295c: news the service (0x4298a), runs the ctor (0x429b4),
    // fetches client settings (0x429d6), caches (0x429ee), and stamps the
    // time (0x42a32..0x42a4a).
    *util.cached_settings.lock() = true;
    *util.last_request_set.lock() = true;
}

// 0x42a98 — +[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]
// type: iOSSettingsService *__cdecl(id, SEL, char)
#[doc(alias = "+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]")]
pub fn stub_0x42a98(util: &RobloxWebUtility, force: bool) -> bool {
    // IDA 0x42a98: dispatches the refresh decision to the block; a forced
    // read refetches (cf. 0x42bc8), otherwise the cached service stands.
    if force || !*util.cached_settings.lock() {
        stub_0x4295c(util);
    }
    *util.cached_settings.lock()
}

// 0x42bc8 — ___63+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]_block_invoke
// type: iOSSettingsService *__fastcall(int)
#[doc(alias = "___63+[RobloxWebUtility getiOSSettingsServiceWithForcedReadFromWeb:]_block_invoke")]
pub fn stub_0x42bc8(util: &RobloxWebUtility, force: bool, stale: bool) -> bool {
    // IDA 0x42bc8: the block refetches on force or staleness, else keeps
    // the cache; the interval math folds into the `stale` input.
    if force || stale {
        stub_0x4295c(util);
    }
    *util.cached_settings.lock()
}

// 0x42dd8 — ___copy_helper_block_65
#[doc(alias = "___copy_helper_block_65")]
pub fn stub_0x42dd8() {
    // IDA 0x42dd8: block copy helper — no-op.
}

// 0x42de4 — ___destroy_helper_block_66
#[doc(alias = "___destroy_helper_block_66")]
pub fn stub_0x42de4() {
    // IDA 0x42de4: block destroy helper — no-op.
}

// 0x42dec — +[RobloxWebUtility getUrlForButtonTag:recordPageView:query:]
// type: id __cdecl(id, SEL, int, char, id)
#[doc(alias = "+[RobloxWebUtility getUrlForButtonTag:recordPageView:query:]")]
pub fn stub_0x42dec(base: &str, search: &str, tag: i32, tablet: bool, query: &str) -> ButtonUrl {
    // IDA 0x42dec: `getUrlForButtonTag:` — see `button_url` (tablet flag at
    // 0x42e2c; page tracking at 0x430b4..0x430d4 folds into the host).
    button_url(base, search, tag, tablet, query)
}

// 0x43180 — __ZN18iOSSettingsServiceC2Ev
// type: iOSSettingsService *__fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::iOSSettingsService(void)")]
pub fn stub_0x43180(service: &mut SettingsServiceState) {
    // IDA 0x43180: `iOSSettingsService` ctor — see `SettingsServiceState`
    // (tree zeroes at 0x431aa..0x431be, `_thisPtr` at 0x4320e, `Init` at
    // 0x43236).
    service.inited = true;
}

// 0x432b0 — __ZN18iOSSettingsServiceD1Ev
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService()")]
pub fn stub_0x432b0(service: &mut SettingsServiceState) {
    // IDA 0x432b0: D1 dtor tears down; drop glue covers it and the service
    // is marked dead.
    service.inited = false;
}

// 0x432b4 — __ZN18iOSSettingsServiceD0Ev
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService() [0x432b4]")]
pub fn stub_0x432b4(service: &mut SettingsServiceState) {
    // IDA 0x432b4: D0 dtor (teardown plus delete); drop glue covers it and
    // the service is marked dead.
    service.inited = false;
}

// 0x432c8 — __ZN18iOSSettingsServiceD2Ev
// type: void __fastcall(iOSSettingsService *__hidden this)
#[doc(alias = "iOSSettingsService::~iOSSettingsService() [0x432c8]")]
pub fn stub_0x432c8(service: &mut SettingsServiceState) {
    // IDA 0x432c8: D2 dtor tears down; drop glue covers it and the service
    // is marked dead.
    service.inited = false;
}

// 0x43314 — __ZN10SimpleJSOND1Ev
// type: void __fastcall(SimpleJSON *__hidden this)
#[doc(alias = "SimpleJSON::~SimpleJSON()")]
pub fn stub_0x43314() {
    // IDA 0x43314: `SimpleJSON` D1 dtor; drop glue covers it — no-op.
}

// 0x43338 — __ZN10SimpleJSOND0Ev
// type: void __fastcall(SimpleJSON *__hidden this)
#[doc(alias = "SimpleJSON::~SimpleJSON() [0x43338]")]
pub fn stub_0x43338() {
    // IDA 0x43338: `SimpleJSON` D0 dtor (teardown plus delete); drop glue
    // covers it — no-op.
}

// 0x43394 — __GLOBAL__I_a_13
#[doc(alias = "global constructor keyed to_a_13")]
pub fn stub_0x43394() -> u32 {
    // IDA 0x43394: `__GLOBAL__I_a_13` one-shot latch (same static-init
    // shape as `GLOBAL_A12_INIT`).
    *GLOBAL_A12_INIT
}

// 0x4352c — __ZN3RBX18FunctionMarshallerC2Ej
// type: int __fastcall(RBX::FunctionMarshaller *this, int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::FunctionMarshaller(unsigned int)")]
pub fn stub_0x4352c(thread_id: u32) -> MarshallerState {
    // IDA 0x4352c: `FunctionMarshaller` ctor wires the thread and zeroes
    // the queues; construction folds into host ownership.
    MarshallerState { thread_id, window: None, queue: Vec::new() }
}

// 0x43624 — __ZN3RBX18FunctionMarshaller9GetWindowEv
// type: int __fastcall(RBX::FunctionMarshaller *this, int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::GetWindow(void)")]
pub fn stub_0x43624(windows: &BTreeMap<u32, u32>, thread_id: u32) -> Option<u32> {
    // IDA 0x43624: `GetWindow` probes the static thread map under the
    // mutex; the table walk folds into a lookup.
    windows.get(&thread_id).copied()
}

// 0x43804 — __ZN3RBX18FunctionMarshaller13ReleaseWindowEPS0_
// type: void __fastcall(RBX::FunctionMarshaller *this, RBX::FunctionMarshaller *, int, int)
#[doc(alias = "RBX::FunctionMarshaller::ReleaseWindow(RBX::FunctionMarshaller*)")]
pub fn stub_0x43804(windows: &mut BTreeMap<u32, u32>, thread_id: u32) {
    // IDA 0x43804: `ReleaseWindow` unlinks the thread entry under the
    // mutex.
    windows.remove(&thread_id);
}

// 0x43930 — __ZN3RBX18FunctionMarshaller14handleAppEventEPv
// type: void __fastcall(RBX::FunctionMarshaller *this, void *)
#[doc(alias = "RBX::FunctionMarshaller::handleAppEvent(void *)")]
pub fn stub_0x43930(event: &mut MarshallerEvent) {
    // IDA 0x43930: `handleAppEvent` — see `MarshallerEvent`.
    event.calls += 1;
    event.signaled = true;
}

// 0x43a98 — __ZN3RBX18FunctionMarshaller7ExecuteEN5boost8functionIFvvEEEPNS_6CEventE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::FunctionMarshaller::Execute(boost::function<void ()(void)>,RBX::CEvent *)")]
pub fn stub_0x43a98(queue: &mut Vec<u32>, on_thread: bool, func_id: u32) -> bool {
    // IDA 0x43a98: `Execute` runs the functor inline on the marshaller
    // thread (0x43af0..0x43afa) and otherwise packages it for
    // `sendAppEvent` (0x43b0c..0x43b4c, folds into the queue). Answers ran
    // (true) vs queued (false).
    if on_thread {
        true
    } else {
        queue.push(func_id);
        false
    }
}

// 0x43b98 — __ZN3RBX18FunctionMarshaller6SubmitEN5boost8functionIFvvEEE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::FunctionMarshaller::Submit(boost::function<void ()(void)>)")]
pub fn stub_0x43b98(queue: &mut Vec<u32>, func_id: u32) {
    // IDA 0x43b98: `Submit` packages the functor (0x43bbc..0x43c18) and
    // posts it (0x43c24, folds into the queue).
    queue.push(func_id);
}

// 0x43c70 — __ZN3RBX18FunctionMarshaller15ProcessMessagesEv
// type: CFRunLoopRunResult __fastcall(Roblox *this)
#[doc(alias = "RBX::FunctionMarshaller::ProcessMessages(void)")]
pub fn stub_0x43c70(queue: &mut Vec<u32>) -> u32 {
    // IDA 0x43c70: `ProcessMessages` thunk to `processAppEvents`; the pump
    // folds into draining the queue, answering the run count.
    let n = queue.len() as u32;
    queue.clear();
    n
}

// 0x43c74 — __ZN3RBX18FunctionMarshaller10StaticDataD1Ev
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData()")]
pub fn stub_0x43c74() {
    // IDA 0x43c74: `StaticData` D1 dtor; drop glue covers it — no-op.
}

// 0x43c78 — __ZN3RBX18FunctionMarshaller10StaticDataD2Ev
// type: void __fastcall(RBX::FunctionMarshaller::StaticData *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::StaticData::~StaticData() [0x43c78]")]
pub fn stub_0x43c78() {
    // IDA 0x43c78: `StaticData` D2 dtor; drop glue covers it — no-op.
}

// 0x43d14 — __ZNSt3mapIjPN3RBX18FunctionMarshallerESt4lessIjESaISt4pairIKjS2_EEEixERS6_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<unsigned int,RBX::FunctionMarshaller *,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::operator[](unsigned int const&)")]
pub fn stub_0x43d14(map: &mut BTreeMap<u32, u32>, key: u32) -> u32 {
    // IDA 0x43d14 `map::operator[]`: lower-bound probe with insert-default
    // on miss (cf. 0x23a04 in generated_110.rs). The default marshaller id
    // is 0.
    *map.entry(key).or_insert(0)
}

// 0x43d6c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseERS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(unsigned int const&)")]
pub fn stub_0x43d6c(map: &mut BTreeMap<u32, u32>, key: u32) -> u32 {
    // IDA 0x43d6c `_Rb_tree::erase(key)`: answers the removed count
    // (0/1).
    u32::from(map.remove(&key).is_some())
}

// 0x43d94 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE11equal_rangeERS1_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::equal_range(unsigned int const&)")]
pub fn stub_0x43d94(map: &BTreeMap<u32, u32>, key: u32) -> (Option<u32>, Option<u32>) {
    // IDA 0x43d94 `equal_range`: answers the lower bound (first key >=)
    // and upper bound (first key >).
    let lo = map.range((Included(key), Unbounded)).next().map(|(&k, _)| k);
    let hi = map.range((Excluded(key), Unbounded)).next().map(|(&k, _)| k);
    (lo, hi)
}

// 0x43de0 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::erase(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>)")]
pub fn stub_0x43de0(map: &mut BTreeMap<u32, u32>, key: u32) -> bool {
    // IDA 0x43de0 `_Rb_tree::erase(iterator)`: removes the probed entry
    // (iterator folds into the key).
    map.remove(&key).is_some()
}

// 0x43e40 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,RBX::FunctionMarshaller *>> *)")]
pub fn stub_0x43e40(map: &mut BTreeMap<u32, u32>) {
    // IDA 0x43e40 `_M_erase(node)`: recursive erase (same shape as
    // 0x16d84). Host has no tree nodes; granularity collapses to the
    // owning map.
    map.clear();
}

// 0x43e68 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
pub fn stub_0x43e68(map: &mut BTreeMap<u32, u32>, key: u32, value: u32) -> bool {
    // IDA 0x43e68 `_M_insert_unique`: inserts on miss (cf. 0x243b0 in
    // generated_110.rs). Answers inserted (true) vs already present.
    if map.contains_key(&key) {
        false
    } else {
        map.insert(key, value);
        true
    }
}

// 0x43f1c — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
pub fn stub_0x43f1c(map: &mut BTreeMap<u32, u32>, key: u32, value: u32) {
    // IDA 0x43f1c `_M_insert` positional insert; the hint folds into the
    // host.
    map.insert(key, value);
}

// 0x43f74 — __ZNSt8_Rb_treeIjSt4pairIKjPN3RBX18FunctionMarshallerEESt10_Select1stIS5_ESt4lessIjESaIS5_EE16_M_insert_uniqueERKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,RBX::FunctionMarshaller *>,std::_Select1st<std::pair<unsigned int const,RBX::FunctionMarshaller *>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,RBX::FunctionMarshaller *>>>::_M_insert_unique(std::pair<unsigned int const,RBX::FunctionMarshaller *> const&)")]
pub fn stub_0x43f74(map: &mut BTreeMap<u32, u32>, key: u32, value: u32) -> bool {
    // IDA 0x43f74 `_M_insert_unique` with hint — same insert-or-existing
    // shape as 0x43e68.
    stub_0x43e68(map, key, value)
}

// 0x43fdc — __ZN5boost11unique_lockINS_15recursive_mutexEE4lockEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unique_lock<boost::recursive_mutex>::lock(void)")]
pub fn stub_0x43fdc(mutex: &mut BoostMutex) -> i32 {
    // IDA 0x43fdc: `unique_lock::lock` (same mutex-take shape as
    // `SimpleMutex::Lock` at 0xa7a0d4) latches locked and answers success.
    mutex.locked = true;
    0
}

// 0x441a8 — __ZN3RBX18FunctionMarshaller27safe_static_init_staticDataEv
// type: _DWORD __fastcall(RBX::FunctionMarshaller *__hidden this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_init_staticData(void)")]
pub fn stub_0x441a8() -> u32 {
    // IDA 0x441a8: `safe_static_init_staticData` — see
    // `MARSHALLER_STATIC_DATA`.
    *MARSHALLER_STATIC_DATA
}

// 0x441ac — __ZN3RBX18FunctionMarshaller29safe_static_do_get_staticDataEv
// type: void *__fastcall(RBX::FunctionMarshaller *this)
#[doc(alias = "RBX::FunctionMarshaller::safe_static_do_get_staticData(void)")]
pub fn stub_0x441ac() -> ! {
    todo!("0x441ac RBX::FunctionMarshaller::safe_static_do_get_staticData(void)")
}

// 0x442bc — __ZN5boost15recursive_mutexC2Ev
// type: _DWORD __fastcall(boost::recursive_mutex *__hidden this)
#[doc(alias = "boost::recursive_mutex::recursive_mutex(void)")]
pub fn stub_0x442bc() -> ! {
    todo!("0x442bc boost::recursive_mutex::recursive_mutex(void)")
}

// 0x44564 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")]
pub fn stub_0x44564() -> ! {
    todo!("0x44564 std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::~_Deque_base()")
}

// 0x44590 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")]
pub fn stub_0x44590() -> ! {
    todo!("0x44590 std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_initialize_map(unsigned long)")
}

// 0x446e8 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x446e8() -> ! {
    todo!("0x446e8 std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_allocate_map(unsigned long)")
}

// 0x44700 — __ZNSt11_Deque_baseIPN5boost8functionIFvvEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")]
pub fn stub_0x44700() -> ! {
    todo!("0x44700 std::_Deque_base<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::_M_create_nodes(boost::function<void ()(void)> ***,boost::function<void ()(void)> ***)")
}

// 0x447f4 — __ZNSt5dequeIPN5boost8functionIFvvEEESaIS4_EEC2ERKS6_
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")]
pub fn stub_0x447f4() -> ! {
    todo!("0x447f4 std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>>::deque(std::deque<boost::function<void ()(void)> *,std::allocator<boost::function<void ()(void)> *>> const&)")
}

// 0x44888 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt15_Deque_iteratorIPN5boost8functionIFvvEEERKS8_PS9_ES3_IS8_RS8_PS8_EEET0_T_SH_SG_
#[doc(alias = "std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")]
pub fn stub_0x44888() -> ! {
    todo!("0x44888 std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **> std::__copy<false,std::random_access_iterator_tag>::copy<std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>>(std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> * const&,boost::function<void ()(void)> * const*>,std::_Deque_iterator<boost::function<void ()(void)> *,boost::function<void ()(void)> *&,boost::function<void ()(void)> **>)")
}

// 0x44924 — __GLOBAL__I_a_14
#[doc(alias = "global constructor keyed to_a_14")]
pub fn stub_0x44924() -> ! {
    todo!("0x44924 global constructor keyed to_a_14")
}

// 0x44abc — -[CameraControl init:delegate:]
// type: id __cdecl(CameraControl *self, SEL, CGRect, id)
#[doc(alias = "-[CameraControl init:delegate:]")]
pub fn stub_0x44abc() -> ! {
    todo!("0x44abc -[CameraControl init:delegate:]")
}

// 0x44b90 — -[CameraControl dealloc]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl dealloc]")]
pub fn stub_0x44b90() -> ! {
    todo!("0x44b90 -[CameraControl dealloc]")
}

// 0x44bbc — -[CameraControl setupPostMouseEventConnection]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl setupPostMouseEventConnection]")]
pub fn stub_0x44bbc() -> ! {
    todo!("0x44bbc -[CameraControl setupPostMouseEventConnection]")
}

// 0x44cd4 — -[CameraControl postMouseEventProcessed:inputObject:event:]
// type: void __cdecl(CameraControl *self, SEL, bool, void *, UIEvent)
#[doc(alias = "-[CameraControl postMouseEventProcessed:inputObject:event:]")]
pub fn stub_0x44cd4() -> ! {
    todo!("0x44cd4 -[CameraControl postMouseEventProcessed:inputObject:event:]")
}

// 0x44d04 — -[CameraControl doCameraPanTouchBegan]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchBegan]")]
pub fn stub_0x44d04() -> ! {
    todo!("0x44d04 -[CameraControl doCameraPanTouchBegan]")
}

// 0x44dec — -[CameraControl doCameraPanTouchEnded]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchEnded]")]
pub fn stub_0x44dec() -> ! {
    todo!("0x44dec -[CameraControl doCameraPanTouchEnded]")
}

// 0x44e58 — -[CameraControl doCameraPanTouchMove]
// type: void __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl doCameraPanTouchMove]")]
pub fn stub_0x44e58() -> ! {
    todo!("0x44e58 -[CameraControl doCameraPanTouchMove]")
}

// 0x450a0 — -[CameraControl touchesBegan:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesBegan:withEvent:]")]
pub fn stub_0x450a0() -> ! {
    todo!("0x450a0 -[CameraControl touchesBegan:withEvent:]")
}

// 0x45124 — -[CameraControl touchesEnded:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesEnded:withEvent:]")]
pub fn stub_0x45124() -> ! {
    todo!("0x45124 -[CameraControl touchesEnded:withEvent:]")
}

// 0x45234 — -[CameraControl touchesCancelled:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesCancelled:withEvent:]")]
pub fn stub_0x45234() -> ! {
    todo!("0x45234 -[CameraControl touchesCancelled:withEvent:]")
}

// 0x45344 — -[CameraControl touchesMoved:withEvent:]
// type: void __cdecl(CameraControl *self, SEL, id, id)
#[doc(alias = "-[CameraControl touchesMoved:withEvent:]")]
pub fn stub_0x45344() -> ! {
    todo!("0x45344 -[CameraControl touchesMoved:withEvent:]")
}

// 0x45454 — -[CameraControl .cxx_construct]
// type: id __cdecl(CameraControl *self, SEL)
#[doc(alias = "-[CameraControl .cxx_construct]")]
pub fn stub_0x45454() -> ! {
    todo!("0x45454 -[CameraControl .cxx_construct]")
}

// 0x4546c — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")]
pub fn stub_0x4546c() -> ! {
    todo!("0x4546c rbx::signals::connection rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::connect<boost::function<void ()(bool,void *,RBX::UIEvent)>>(boost::function<void ()(bool,void *,RBX::UIEvent)> const&)")
}

// 0x45554 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")]
pub fn stub_0x45554() -> ! {
    todo!("0x45554 rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::insert(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot *)")
}

// 0x45764 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")]
pub fn stub_0x45764() -> ! {
    todo!("0x45764 boost::intrusive_ptr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot>::operator=(rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot*)")
}

#[cfg(test)]
mod analytics_webutil_batch_tests {
    use super::*;

    fn live_state() -> GoogleAnalyticsState {
        let state = GoogleAnalyticsState::default();
        state.initialize_done.store(true, Ordering::SeqCst);
        state
    }
    #[test]
    fn event_tracking_gates_on_init() {
        let live = live_state();
        stub_0x4203c(&live, 1, 2, 3, 42);
        assert_eq!(live.sent_events.lock().len(), 1);
        assert_eq!(
            live.sent_events.lock()[0],
            AnalyticsEvent { category: 1, action: 2, label: 3, value: 42 }
        );
        let cold = GoogleAnalyticsState::default();
        stub_0x420e4(&cold, 1, 2, 3, 4);
        assert!(cold.sent_events.lock().is_empty());
        assert_eq!(cold.deferred_events.lock().len(), 1);
        stub_0x42230(&live, 7, 8);
        assert_eq!(*live.custom_vars.lock(), vec![(7, 8)]);
        stub_0x42298(&cold, 7, 8);
        assert_eq!(*cold.deferred_custom.lock(), vec![(7, 8)]);
    }

    #[test]
    fn debug_counters() {
        let state = GoogleAnalyticsState::default();
        assert_eq!(stub_0x424cc(&state, "inGame"), 1);
        assert_eq!(stub_0x424cc(&state, "inGame"), 2);
        let snapshot = stub_0x42374(&state);
        assert_eq!(snapshot, vec![("debug_inGame".to_owned(), 2)]);
        assert_eq!(stub_0x42580(), 1);
    }

    #[test]
    fn webutil_lifecycle() {
        let util = RobloxWebUtility::default();
        assert!(!stub_0x4290c(&util));
        stub_0x427c0(&util);
        assert!(stub_0x4290c(&util));
        assert!(stub_0x4291c(&util));
        assert!(stub_0x4294c(&util));
        assert!(!stub_0x4293c(&util));
        stub_0x4292c(&util, true);
        stub_0x4295c(&util);
        assert!(stub_0x4293c(&util));
        assert!(stub_0x42a98(&util, false));
        stub_0x42880(&util);
        assert!(!stub_0x4290c(&util));
        assert!(!stub_0x4293c(&util));
        stub_0x4295c(&util);
        stub_0x427a8();
        stub_0x427b4();
        stub_0x42dd8();
        stub_0x42de4();
        assert!(std::ptr::eq(stub_0x42718(), web_utility()));
    }

    #[test]
    fn settings_refresh() {
        let util = RobloxWebUtility::default();
        assert!(stub_0x42a98(&util, false));
        assert!(stub_0x42a98(&util, true));
        let util = RobloxWebUtility::default();
        assert!(!stub_0x42bc8(&util, false, false));
        assert!(stub_0x42bc8(&util, false, true));
        let mut service = SettingsServiceState::default();
        stub_0x43180(&mut service);
        assert!(service.inited);
    }

    #[test]
    fn button_urls() {
        let base = "https://www.roblox.com/";
        assert_eq!(
            stub_0x42dec(base, "s", 10, false, ""),
            ButtonUrl { url: Some(format!("{base}games/list")), page: Some("Games") }
        );
        assert_eq!(
            stub_0x42dec(base, "s", 11, true, ""),
            ButtonUrl { url: Some(format!("{base}Catalog/")), page: Some("Catalog") }
        );
        assert_eq!(
            stub_0x42dec(base, "s", 11, false, ""),
            ButtonUrl { url: Some(format!("{base}catalog/")), page: Some("Catalog") }
        );
        assert_eq!(
            stub_0x42dec(base, "s", 12, true, ""),
            ButtonUrl { url: Some(format!("{base}My/Character.aspx")), page: Some("Inventory") }
        );
        assert_eq!(
            stub_0x42dec(base, "s", 13, false, ""),
            ButtonUrl { url: Some(format!("{base}mobile-app-upgrades/")), page: Some("BuildersClub") }
        );
        assert_eq!(
            stub_0x42dec(base, "s", 14, false, ""),
            ButtonUrl { url: Some(base.to_owned()), page: Some("Profile") }
        );
        assert_eq!(
            stub_0x42dec(base, "s", 15, true, ""),
            ButtonUrl { url: Some(format!("{base}My/Messages.aspx#Inbox")), page: Some("Messages") }
        );
        assert_eq!(
            stub_0x42dec(base, "srch", 16, false, "q=obby"),
            ButtonUrl { url: Some(format!("{base}srchq=obby")), page: Some("Search") }
        );
        assert_eq!(
            stub_0x42dec(base, "s", 99, false, ""),
            ButtonUrl { url: None, page: None }
        );
    }
}

#[cfg(test)]
mod marshaller_map_batch_tests {
    use super::*;

    #[test]
    fn marshaller_lifecycle() {
        let mut marshaller = stub_0x4352c(7);
        assert_eq!(marshaller.thread_id, 7);
        assert_eq!(marshaller.window, None);
        let mut windows = BTreeMap::from([(7u32, 70u32)]);
        assert_eq!(stub_0x43624(&windows, 7), Some(70));
        assert_eq!(stub_0x43624(&windows, 8), None);
        stub_0x43804(&mut windows, 7);
        assert!(windows.is_empty());
        let mut event = MarshallerEvent::default();
        stub_0x43930(&mut event);
        assert_eq!(event, MarshallerEvent { calls: 1, signaled: true });
        let mut service = SettingsServiceState::default();
        stub_0x43180(&mut service);
        stub_0x432b0(&mut service);
        assert!(!service.inited);
        stub_0x43180(&mut service);
        stub_0x432b4(&mut service);
        assert!(!service.inited);
        stub_0x43180(&mut service);
        stub_0x432c8(&mut service);
        assert!(!service.inited);
        stub_0x43314();
        stub_0x43338();
        assert_eq!(stub_0x43394(), 1);
        assert_eq!(stub_0x441a8(), 1);
    }

    #[test]
    fn execute_queues_off_thread() {
        let mut queue = Vec::new();
        assert!(stub_0x43a98(&mut queue, true, 9));
        assert!(queue.is_empty());
        assert!(!stub_0x43a98(&mut queue, false, 9));
        assert_eq!(queue, vec![9]);
        stub_0x43b98(&mut queue, 10);
        assert_eq!(queue, vec![9, 10]);
        assert_eq!(stub_0x43c70(&mut queue), 2);
        assert!(queue.is_empty());
    }

    #[test]
    fn marshaller_maps() {
        let mut map = BTreeMap::new();
        assert_eq!(stub_0x43d14(&mut map, 3), 0);
        assert_eq!(map[&3], 0);
        assert!(stub_0x43e68(&mut map, 4, 40));
        assert!(!stub_0x43e68(&mut map, 4, 41));
        assert_eq!(map[&4], 40);
        assert_eq!(stub_0x43d94(&map, 3), (Some(3), Some(4)));
        assert_eq!(stub_0x43d94(&map, 5), (None, None));
        assert_eq!(stub_0x43d6c(&mut map, 3), 1);
        assert_eq!(stub_0x43d6c(&mut map, 3), 0);
        assert!(stub_0x43f74(&mut map, 5, 50));
        assert!(stub_0x43de0(&mut map, 5));
        assert_eq!(stub_0x43d6c(&mut map, 4), 1);
        assert!(map.is_empty());
        stub_0x43f1c(&mut map, 6, 60);
        assert_eq!(map[&6], 60);
        stub_0x43e40(&mut map);
        assert!(map.is_empty());
        stub_0x43c74();
        stub_0x43c78();
        let mut mutex = BoostMutex::default();
        assert_eq!(stub_0x43fdc(&mut mutex), 0);
        assert!(mutex.locked);
    }
}
